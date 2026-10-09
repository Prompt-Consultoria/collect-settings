//! Build, encode and decode the configuration QR codes used by
//! [ODK Collect](https://docs.getodk.org/collect-import-export/) and its
//! rebrand KoboCollect.
//!
//! A configuration QR code carries a JSON document with three sections:
//!
//! * [`General`] — unprotected settings: server, account, form management,
//!   user interface;
//! * [`Admin`] — protected settings: the admin password and every
//!   access-control switch (`true` = the interviewer may use it);
//! * [`Project`] — the name, icon and colour shown in the project switcher.
//!
//! The JSON is compressed with zlib, encoded as base64 and rendered as a QR
//! code. Any setting the code omits is reset to its default when scanned, so
//! the model keeps every field optional and serialises only what you set.
//!
//! ```
//! use collect_settings::{Settings, General, Admin, Project, Autosend, FormUpdateMode};
//!
//! let settings = Settings {
//!     general: General {
//!         server_url: Some("https://kc.kobotoolbox.org".into()),
//!         username: Some("interviewer01".into()),
//!         password: Some("secret".into()),
//!         autosend: Some(Autosend::WifiAndCellular),
//!         form_update_mode: Some(FormUpdateMode::MatchExactly),
//!         automatic_update: Some(true),
//!         ..General::default()
//!     },
//!     admin: Admin::locked_interviewer("admin-pw"),
//!     project: Some(Project::new("Household survey").icon('H').color("#6b63e6")),
//! };
//!
//! let payload = settings.encode().unwrap();           // base64(zlib(json))
//! let back = Settings::decode(&payload).unwrap();
//! assert_eq!(back.general.username.as_deref(), Some("interviewer01"));
//!
//! # #[cfg(feature = "qr")] {
//! let svg = settings.qr_svg(4).unwrap();              // ready to embed or print
//! assert!(svg.starts_with("<svg"));
//! # }
//! ```
//!
//! The key names follow the current `ProjectKeys` / `ProtectedProjectKeys` of
//! ODK Collect. Keys that were renamed between releases (`save_as` →
//! `save_as_draft`, `mark_as_finalized` → `finalize_in_form_entry`) are
//! emitted under both names so one code works on old and new app versions;
//! Collect silently drops the keys it does not know.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod admin;
mod codec;
mod general;
mod project;
#[cfg(feature = "qr")]
mod qr;

pub use admin::Admin;
pub use codec::{decode, encode, encode_json, Error};
pub use general::{
    Autosend, BasemapSource, ConstraintBehavior, FontSize, FormUpdateMode, General, GuidanceHint,
    ImageSize, Navigation, PeriodicCheck, Theme,
};
pub use project::Project;
#[cfg(feature = "qr")]
pub use qr::{render_svg, QrOptions};
#[cfg(feature = "qr")]
pub use qrcode::{EcLevel, QrCode};

use serde::{Deserialize, Serialize};

/// The complete document carried by a configuration QR code.
///
/// `general` and `admin` are always serialised (Collect requires both keys,
/// even when empty); `project` is written only when present.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    /// Unprotected settings (server, account, forms, interface).
    #[serde(default)]
    pub general: General,
    /// Protected settings (admin password and access control).
    #[serde(default)]
    pub admin: Admin,
    /// Project name, icon and colour.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

impl Settings {
    /// Compact JSON exactly as it goes into the QR code (before compression).
    pub fn to_json(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(self)?)
    }

    /// Pretty JSON, for display and review.
    pub fn to_json_pretty(&self) -> Result<String, Error> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// `base64(zlib(json))` — the string a QR code must contain.
    pub fn encode(&self) -> Result<String, Error> {
        encode(self)
    }

    /// Parse the string read from a configuration QR code.
    pub fn decode(payload: &str) -> Result<Settings, Error> {
        decode(payload)
    }

    /// Human-readable warnings about combinations that usually surprise people
    /// in the field. An empty list does not prove the configuration is right;
    /// it only means none of the known traps is present.
    pub fn warnings(&self) -> Vec<String> {
        let mut w = Vec::new();
        let g = &self.general;
        let a = &self.admin;
        match g.server_url.as_deref() {
            None | Some("") => w.push("general.server_url is empty: the app will keep its current server".into()),
            Some(u) if !u.starts_with("https://") => {
                w.push(format!("server_url `{u}` is not https: Collect refuses Basic auth over plain http"))
            }
            Some(u) if u.contains("kf.") || u.contains("eu.kobotoolbox.org") => w.push(format!(
                "server_url `{u}` looks like the KoboToolbox web interface (KPI); the app must point at the KoboCAT host (kc.… / kc-eu.…)"
            )),
            _ => {}
        }
        if a.admin_pw.as_deref().map_or(true, str::is_empty) && a.locks_anything() {
            w.push("admin section locks features but admin_pw is empty: anyone can open the protected menu".into());
        }
        if a.qr_code_scanner == Some(true) && a.locks_anything() {
            w.push("qr_code_scanner is allowed: any other QR code can undo these locks".into());
        }
        if a.change_server == Some(true) && a.locks_anything() {
            w.push("change_server is allowed: the interviewer can switch account or server".into());
        }
        if g.form_update_mode == Some(FormUpdateMode::MatchExactly)
            && g.automatic_update == Some(false)
        {
            w.push("form_update_mode is match_exactly but automatic_update is false: new versions will wait for a manual refresh".into());
        }
        if g.form_update_mode.is_some()
            && g.form_update_mode != Some(FormUpdateMode::Manual)
            && g.periodic_form_updates_check.is_none()
        {
            w.push("periodic_form_updates_check is unset: the app will use its default interval (every 24 hours)".into());
        }
        if a.get_blank == Some(false)
            && g.form_update_mode
                .map_or(true, |m| m == FormUpdateMode::Manual)
        {
            w.push("get_blank is hidden and form_update_mode is manual: nobody can download forms on the device".into());
        }
        if a.send_finalized == Some(false) && g.autosend.map_or(true, |m| m == Autosend::Off) {
            w.push(
                "send_finalized is hidden and autosend is off: finalized forms can never be sent"
                    .into(),
            );
        }
        if g.delete_send == Some(true) {
            w.push(
                "delete_send is true: the local copy is removed as soon as a form is sent".into(),
            );
        }
        if let Some(p) = &self.project {
            if p.icon.as_ref().is_some_and(|i| i.chars().count() > 1) {
                w.push("project.icon should be a single character".into());
            }
        }
        w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let s = Settings {
            general: General {
                server_url: Some("https://kc.example.org".into()),
                username: Some("u".into()),
                password: Some("p".into()),
                autosend: Some(Autosend::WifiOnly),
                ..Default::default()
            },
            admin: Admin::locked_interviewer("x"),
            project: Some(Project::new("P").icon('P').color("#000000")),
        };
        let enc = s.encode().unwrap();
        assert_eq!(Settings::decode(&enc).unwrap(), s);
    }

    #[test]
    fn empty_sections_are_present() {
        let json = Settings::default().to_json().unwrap();
        assert_eq!(json, r#"{"general":{},"admin":{}}"#);
    }

    #[test]
    fn warns_about_kpi_host() {
        let s = Settings {
            general: General {
                server_url: Some("https://kf.kobotoolbox.org".into()),
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(s.warnings().iter().any(|w| w.contains("KoboCAT")));
    }
}

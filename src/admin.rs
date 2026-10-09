use serde::{Deserialize, Serialize};

macro_rules! admin_struct {
    ($( $(#[$doc:meta])* $field:ident ),+ $(,)?) => {
        /// Protected settings: the admin password and the access-control
        /// switches. Every switch means "the interviewer may use this":
        /// `Some(false)` hides it behind the admin password. An omitted key
        /// resets to `true` (allowed) when the code is scanned.
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Admin {
            /// Password of the *Protected settings* menu. Empty means open.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub admin_pw: Option<String>,
            $(
                $(#[$doc])*
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub $field: Option<bool>,
            )+
            /// Any other key, kept verbatim through decode → encode.
            #[serde(default, flatten, skip_serializing_if = "serde_json::Map::is_empty")]
            pub extra: serde_json::Map<String, serde_json::Value>,
        }

        impl Admin {
            /// Names of every boolean switch, in the order Collect declares them.
            pub const SWITCHES: &'static [&'static str] = &[$(stringify!($field)),+];

            /// Set every switch to the same value, keeping the password.
            pub fn set_all(mut self, allowed: bool) -> Self {
                $( self.$field = Some(allowed); )+
                self
            }

            /// Set a switch by its key name. Returns `false` for unknown names.
            pub fn set(&mut self, key: &str, allowed: bool) -> bool {
                match key {
                    $( stringify!($field) => { self.$field = Some(allowed); true } )+
                    _ => false,
                }
            }

            /// Current value of a switch by key name (`None` = unset).
            pub fn get(&self, key: &str) -> Option<Option<bool>> {
                match key {
                    $( stringify!($field) => Some(self.$field), )+
                    _ => None,
                }
            }

            /// Whether any switch is explicitly `false`.
            pub fn locks_anything(&self) -> bool {
                false $( || self.$field == Some(false) )+
            }
        }
    };
}

admin_struct! {
    // ------------------------------------------------- main menu
    /// *Edit Saved Form* / Drafts on the main menu.
    edit_saved,
    /// *Send Finalized Form* on the main menu.
    send_finalized,
    /// *View Sent Form* on the main menu.
    view_sent,
    /// *Get Blank Form* on the main menu.
    get_blank,
    /// *Delete Saved Form* on the main menu.
    delete_saved,
    /// Reconfigure the app by scanning another QR code.
    qr_code_scanner,
    // ------------------------------------------------- user settings
    /// Server settings (URL, account).
    change_server,
    /// Project name, icon and colour.
    change_project_display,
    /// App theme (older key, still honoured by some versions).
    change_app_theme,
    /// App language.
    change_app_language,
    /// Font size.
    change_font_size,
    /// Navigation style.
    change_navigation,
    /// Map settings.
    maps,
    /// Blank form update mode.
    form_update_mode,
    /// Update check interval.
    periodic_form_updates_check,
    /// Automatic download of updates.
    automatic_update,
    /// Hide old form versions.
    hide_old_form_versions,
    /// Auto send.
    change_autosend,
    /// Delete after send.
    delete_after_send,
    /// Default to finalized (older versions).
    default_to_finalized,
    /// Constraint processing.
    change_constraint_behavior,
    /// High-resolution photos.
    high_resolution,
    /// Image size.
    image_size,
    /// Guidance hints.
    guidance_hint,
    /// External app recording.
    external_app_recording,
    /// Finalized forms sync.
    instance_form_sync,
    /// Form metadata (username, phone, e-mail).
    change_form_metadata,
    /// Usage analytics.
    analytics,
    // ------------------------------------------------- form entry
    /// Moving backwards inside a form. Turning it off suggests also turning off
    /// `edit_saved`, `save_mid`, `jump_to` and validating on swipe.
    moving_backwards,
    /// The *Settings* entry in the overflow menu while filling a form.
    access_settings,
    /// Change form language while filling.
    change_language,
    /// *Go To Prompt* (jump to any question).
    jump_to,
    /// Save mid-form (older key for drafts).
    save_mid,
    /// *Save as draft* (current key).
    save_as_draft,
    /// *Save as* name editing (older key).
    save_as,
    /// *Finalize* button at the end of the form (current key).
    finalize_in_form_entry,
    /// *Mark form as finalized* (older key).
    mark_as_finalized,
    /// Editing a finalized form through other entry points.
    allow_other_ways_of_editing_form,
    /// Bulk finalize from the drafts list.
    bulk_finalize,
}

impl Admin {
    /// Everything allowed, with the given admin password.
    pub fn open(admin_pw: impl Into<String>) -> Self {
        Admin {
            admin_pw: Some(admin_pw.into()),
            ..Default::default()
        }
        .set_all(true)
    }

    /// The interviewer-only device: open the app, pick a form, fill it, done.
    ///
    /// Hidden behind the admin password: every user setting, the server and
    /// account, scanning another QR code, downloading and deleting forms,
    /// editing drafts. Kept visible: sending finalized forms by hand (the
    /// fallback when autosend hits a dead zone) and viewing what was sent.
    /// Inside the form the interviewer can still move backwards and change the
    /// form language; combine with [`Admin::set`] to tighten that.
    pub fn locked_interviewer(admin_pw: impl Into<String>) -> Self {
        let mut a = Admin {
            admin_pw: Some(admin_pw.into()),
            ..Default::default()
        }
        .set_all(false);
        a.send_finalized = Some(true);
        a.view_sent = Some(true);
        a.moving_backwards = Some(true);
        a.change_language = Some(true);
        a.finalize_in_form_entry = Some(true);
        a.mark_as_finalized = Some(true);
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locked_preset_keeps_send_path() {
        let a = Admin::locked_interviewer("x");
        assert_eq!(a.send_finalized, Some(true));
        assert_eq!(a.qr_code_scanner, Some(false));
        assert_eq!(a.access_settings, Some(false));
        assert!(a.locks_anything());
        assert!(!Admin::open("x").locks_anything());
    }

    #[test]
    fn set_by_name() {
        let mut a = Admin::default();
        assert!(a.set("jump_to", false));
        assert!(!a.set("nope", false));
        assert_eq!(a.get("jump_to"), Some(Some(false)));
        assert!(Admin::SWITCHES.contains(&"bulk_finalize"));
    }
}

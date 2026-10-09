use serde::{Deserialize, Serialize};

macro_rules! str_enum {
    ($(#[$doc:meta])* $name:ident { $($(#[$vdoc:meta])* $variant:ident = $value:literal),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum $name {
            $( $(#[$vdoc])* #[serde(rename = $value)] $variant ),+
        }
        impl $name {
            /// Every variant, in declaration order.
            pub const ALL: &'static [$name] = &[$($name::$variant),+];
            /// The string Collect stores for this value.
            pub fn as_str(self) -> &'static str {
                match self { $($name::$variant => $value),+ }
            }
        }
        impl std::str::FromStr for $name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s { $($value => Ok($name::$variant),)+ other => Err(format!("unknown {}: {other}", stringify!($name))) }
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(self.as_str()) }
        }
    };
}

str_enum! {
    /// When finalized forms are sent without the interviewer pressing *Send*.
    Autosend {
        /// Never send automatically.
        Off = "off",
        /// Only over Wi-Fi.
        WifiOnly = "wifi_only",
        /// Only over mobile data.
        CellularOnly = "cellular_only",
        /// Over any connection.
        WifiAndCellular = "wifi_and_cellular",
    }
}

str_enum! {
    /// How blank forms are kept in sync with the server.
    FormUpdateMode {
        /// Only when the interviewer opens *Get Blank Form*.
        Manual = "manual",
        /// Check for new versions of forms already on the device.
        PreviouslyDownloaded = "previously_downloaded",
        /// Mirror the server: download new forms, update changed ones, remove the rest.
        MatchExactly = "match_exactly",
    }
}

str_enum! {
    /// Interval of the background check for form updates.
    PeriodicCheck {
        /// Every 15 minutes.
        Every15Minutes = "every_fifteen_minutes",
        /// Every hour.
        EveryHour = "every_one_hour",
        /// Every six hours.
        Every6Hours = "every_six_hours",
        /// Once a day.
        Every24Hours = "every_24_hours",
    }
}

str_enum! {
    /// How the interviewer moves between questions.
    Navigation {
        /// Horizontal swipes.
        Swipe = "swipe",
        /// Forward/back buttons.
        Buttons = "buttons",
        /// Both.
        SwipeAndButtons = "swipe_buttons",
    }
}

str_enum! {
    /// When constraints are validated.
    ConstraintBehavior {
        /// On every forward swipe.
        OnSwipe = "on_swipe",
        /// Only when finalizing.
        OnFinalize = "on_finalize",
    }
}

str_enum! {
    /// Whether guidance hints are shown.
    GuidanceHint {
        /// Hidden.
        No = "no",
        /// Always shown.
        Yes = "yes",
        /// Shown collapsed.
        YesCollapsed = "yes_collapsed",
    }
}

str_enum! {
    /// Downscaling applied to photos taken in the form.
    ImageSize {
        /// No downscaling.
        Original = "original",
        /// Very small (640 px).
        VerySmall = "very_small",
        /// Small (1024 px).
        Small = "small",
        /// Medium (2048 px).
        Medium = "medium",
        /// Large (3072 px).
        Large = "large",
    }
}

str_enum! {
    /// App theme.
    Theme {
        /// Light.
        Light = "light_theme",
        /// Dark.
        Dark = "dark_theme",
    }
}

str_enum! {
    /// Basemap provider for geo questions.
    BasemapSource {
        /// Google Maps.
        Google = "google",
        /// Mapbox.
        Mapbox = "mapbox",
        /// OpenStreetMap.
        Osm = "osm",
        /// USGS.
        Usgs = "usgs",
        /// Carto.
        Carto = "carto",
    }
}

/// Font size of the form entry screen, in sp. Collect accepts 13, 17, 21, 25 and 29.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "u8", try_from = "u8")]
pub enum FontSize {
    /// 13 sp.
    ExtraSmall,
    /// 17 sp.
    Small,
    /// 21 sp (the app default).
    Medium,
    /// 25 sp.
    Large,
    /// 29 sp.
    ExtraLarge,
}

impl From<FontSize> for u8 {
    fn from(f: FontSize) -> u8 {
        match f {
            FontSize::ExtraSmall => 13,
            FontSize::Small => 17,
            FontSize::Medium => 21,
            FontSize::Large => 25,
            FontSize::ExtraLarge => 29,
        }
    }
}

impl TryFrom<u8> for FontSize {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            13 => FontSize::ExtraSmall,
            17 => FontSize::Small,
            21 => FontSize::Medium,
            25 => FontSize::Large,
            29 => FontSize::ExtraLarge,
            other => {
                return Err(format!(
                    "font_size must be 13, 17, 21, 25 or 29, got {other}"
                ))
            }
        })
    }
}

/// Unprotected settings. Every field is optional: an omitted key resets to
/// the app default when the code is scanned, so set only what you mean.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct General {
    // ---------------------------------------------------------------- server
    /// Server protocol. Always `odk_default` for ODK Central and KoboToolbox.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Base URL of the OpenRosa server. For KoboToolbox this is the KoboCAT
    /// host (`https://kc.kobotoolbox.org`, `https://kc-eu.kobotoolbox.org`),
    /// never the `kf.`/`eu.` web interface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
    /// Account the device logs in with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Password of that account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Legacy custom form list URL (`protocol = other`, removed from recent
    /// app versions). Kept for decoding old codes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formlist_url: Option<String>,
    /// Legacy custom submission URL. See [`General::formlist_url`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submission_url: Option<String>,

    // ------------------------------------------------------------ interface
    /// Light or dark theme.
    #[serde(default, rename = "appTheme", skip_serializing_if = "Option::is_none")]
    pub app_theme: Option<Theme>,
    /// UI language as a BCP 47 code (`pt`, `es`, `en`…).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_language: Option<String>,
    /// Font size of the form entry screen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<FontSize>,
    /// Swipe, buttons or both.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub navigation: Option<Navigation>,

    // ----------------------------------------------------------------- maps
    /// Basemap provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basemap_source: Option<BasemapSource>,
    /// Google map style (1–4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_map_style: Option<u8>,
    /// Mapbox style URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapbox_map_style: Option<String>,
    /// USGS style (`topographic`, `hybrid`, `satellite`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usgs_map_style: Option<String>,
    /// Carto style (`positron`, `dark_matter`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carto_map_style: Option<String>,
    /// Absolute path of an `.mbtiles` reference layer on the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_layer: Option<String>,

    // ------------------------------------------------------ form management
    /// How blank forms are kept in sync.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_update_mode: Option<FormUpdateMode>,
    /// Background check interval (used by `previously_downloaded` and `match_exactly`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periodic_form_updates_check: Option<PeriodicCheck>,
    /// Download updates without asking.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automatic_update: Option<bool>,
    /// Hide superseded versions in the blank-form list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hide_old_form_versions: Option<bool>,
    /// Send finalized forms automatically.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autosend: Option<Autosend>,
    /// Delete the local copy after a successful send (`delete_send` in the app).
    #[serde(
        default,
        rename = "delete_send",
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_send: Option<bool>,
    /// Mark forms as finalized by default on the save screen (older app versions).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_completed: Option<bool>,
    /// Validate on every swipe or only on finalize.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraint_behavior: Option<ConstraintBehavior>,
    /// Take photos at full resolution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_resolution: Option<bool>,
    /// Downscale photos.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSize>,
    /// Record audio with an external app instead of the built-in recorder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_app_recording: Option<bool>,
    /// Guidance hint display.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidance_hint: Option<GuidanceHint>,
    /// Sync the instance list with the file system (`instance_sync`).
    #[serde(
        default,
        rename = "instance_sync",
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_sync: Option<bool>,

    // --------------------------------------------------------------- identity
    /// Send anonymous usage data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analytics: Option<bool>,
    /// Form metadata: username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_username: Option<String>,
    /// Form metadata: phone number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_phonenumber: Option<String>,
    /// Form metadata: e-mail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_email: Option<String>,

    /// Any other key, kept verbatim so unknown or future settings survive a
    /// decode → encode roundtrip.
    #[serde(default, flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl General {
    /// Server, account and the "hands-off" form management the field wants:
    /// the device mirrors the server (`match_exactly`), downloads new versions
    /// on its own every 15 minutes, sends finalized forms over any connection
    /// and keeps the local copy after sending.
    pub fn auto_sync(
        server_url: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        General {
            server_url: Some(server_url.into()),
            username: Some(username.into()),
            password: Some(password.into()),
            form_update_mode: Some(FormUpdateMode::MatchExactly),
            periodic_form_updates_check: Some(PeriodicCheck::Every15Minutes),
            automatic_update: Some(true),
            hide_old_form_versions: Some(true),
            autosend: Some(Autosend::WifiAndCellular),
            delete_send: Some(false),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_strings_match_collect() {
        assert_eq!(
            serde_json::to_string(&Autosend::WifiAndCellular).unwrap(),
            "\"wifi_and_cellular\""
        );
        assert_eq!(serde_json::to_string(&FontSize::Medium).unwrap(), "21");
        assert_eq!(
            serde_json::from_str::<FontSize>("29").unwrap(),
            FontSize::ExtraLarge
        );
        assert!(serde_json::from_str::<FontSize>("20").is_err());
        assert_eq!(
            "match_exactly".parse::<FormUpdateMode>().unwrap(),
            FormUpdateMode::MatchExactly
        );
    }

    #[test]
    fn unknown_keys_survive() {
        let g: General =
            serde_json::from_str(r#"{"server_url":"https://x","future_key":1}"#).unwrap();
        assert_eq!(g.extra["future_key"], 1);
        let json = serde_json::to_string(&g).unwrap();
        assert!(json.contains("future_key"));
    }
}

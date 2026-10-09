use serde::{Deserialize, Serialize};

/// Name, icon and colour of the project as shown in the app's project switcher.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    /// Project name (shown on the main screen).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A single character drawn in the project badge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Badge colour as `#rrggbb`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl Project {
    /// A project with the given name and no icon or colour.
    pub fn new(name: impl Into<String>) -> Self {
        Project {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    /// Set the badge character.
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    /// Set the badge colour (`#rrggbb`).
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

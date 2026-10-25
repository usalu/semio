//! 📋️ Facts of the registry: the serialisable input of the registry build with its project, target, playground and ticket facts. See [`super`].

use super::*;

// #region 🔖️Facts
/// 📦️ What the sources state, before any command is derived: the serialisable input of [`Registry::build`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Facts {
    pub projects: Vec<ProjectFacts>,
    pub playgrounds: Vec<PlaygroundFacts>,
    pub tickets: Vec<TicketFacts>,
    pub problems: Vec<Problem>,
    /// 📇️ Whether the caller supplied a playground catalog publication; without one a `playground:` reference cannot be proved wrong.
    #[serde(default)]
    pub catalog_configured: bool,
    /// 🕸️ Set when no build graph publication could be read: the targets the graph infers are then unknown, not missing.
    #[serde(default)]
    pub graph_unpublished: bool,
}

/// 🏢️ One build project: its root, the file that declares it and its targets.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectFacts {
    pub name: String,
    pub root: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub manifest: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<TargetFacts>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<serde_json::Value>,
}

/// 🎯️ One build target with the facts the registry reads from it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TargetFacts {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuous: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub configurations: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<serde_json::Value>,
}

/// 🛝️ One playground variant of the generated catalog.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaygroundFacts {
    pub variant: String,
    pub plugin: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    pub react: u16,
    pub wgpu: u16,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_react: Vec<u16>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_wgpu: Vec<u16>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<String>,
    /// 🔖️ Other names the variant answers to; a search finds the variant by them.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    /// 🤝️ The hub a collaborative session of the variant joins, declared by its plugin's playground row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hub: Option<String>,
    /// 💾️ The workspace-relative directory the variant keeps its documents in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_dir: Option<String>,
    /// 👥️ The same per user slot, `{N}` standing for the slot number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_data_dir: Option<String>,
    /// 🔒️ Whether the variant can run without reaching any hub.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub local_only: bool,
    /// 👁️ The address a browser opens for the viewer role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer_path: Option<String>,
}

/// 🎫️ One ticket of the index; an open ticket carries its `🎮️commands.json` document.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TicketFacts {
    pub id: String,
    pub open: bool,
    pub title: String,
    pub folder: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commands: Option<serde_json::Value>,
}

/// 📄️ The name of a ticket's command document.
pub const TICKET_COMMANDS: &str = "🎮️commands.json";
// #endregion 🔖️Facts

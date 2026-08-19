//! 🎭️ S Home launcher app — "explore" mode definition (constitutional: ui/Mode).

use semio_framework_plugin::{LocalizedLabel, ModeDefinition};

//#region 🔖️Manifest
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: "explore".into(), label: LocalizedLabel::native("Explore", "Erkunden"), icon_id: "focus".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}
//#endregion 🔖️Manifest

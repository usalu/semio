//! 🎭️ S Studio app — "main" mode definition (constitutional: ui/Mode). The only mode: Workflow +
//! Media VFS + Compiled DAG windows, laid out side by side.

use semio_framework_ui_locale::{LocalizedLabel};
use semio_framework_plugin::{ModeDefinition};

//#region 🔖️Manifest
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: "main".into(), label: LocalizedLabel::native("Space", "Space"), icon_id: "globe".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}
//#endregion 🔖️Manifest

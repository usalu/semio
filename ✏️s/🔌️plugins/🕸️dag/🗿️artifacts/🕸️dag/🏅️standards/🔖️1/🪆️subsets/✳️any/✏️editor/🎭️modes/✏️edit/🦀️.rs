//! ✏️ DAG play app — the `edit` mode: the app's only mode, a two-window authoring layout (node-graph
//! canvas + compiled DSL).

use crate::editor::dag::modes::edit::tools::reorganize;
use crate::editor::dag::modes::edit::windows::{compiled, main};
use semio_framework_plugin::create_default_layout;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::WindowLayout;

pub const DAG_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::dag::create_dag_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: DAG_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: vec![::semio_framework_async::poll::resolve_ready(ToolRef::new(reorganize::TOOL_ID))], layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[main::DAG_PLAY_WINDOW_MAIN.into(), compiled::DAG_PLAY_WINDOW_COMPILED.into()], "row", Some(&[68.0, 32.0]), Some(&["DAG".into(), "DSL".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

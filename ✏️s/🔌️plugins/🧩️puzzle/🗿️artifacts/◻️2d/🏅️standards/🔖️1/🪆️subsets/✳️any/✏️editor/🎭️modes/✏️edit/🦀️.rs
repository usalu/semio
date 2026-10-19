//! 🖥️ the pane-camera framing, board-scene and engagement-HUD builders its three windows share. Each
//! ✏️ Puzzle 2d play app — the `edit` mode: its definition and the triptych layout (overview | detail
//! | selection); the pane builders the three windows share live in the `canvas` child.

use crate::editor::puzzle2d::modes::edit::tools::fill;
use semio_framework_plugin::create_default_layout;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::WindowLayout;
use semio_framework_pack_json::Value;

pub const PUZZLE2D_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle2d::create_puzzle2d_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition {
        id: PUZZLE2D_PLAY_MODE_EDIT.into(),
        label: LocalizedLabel::native("Edit", "Bearbeiten"),
        icon_id: "pencil".into(),
        tools: vec![::semio_framework_async::poll::resolve_ready(ToolRef::new(fill::TOOL_ID))],
        layout_id: None,
        commands: Vec::new(),
    }
}

/// 🪟️ Triptych play layout: a wide interactive overview beside the detail and selection panes.
pub fn layout() -> WindowLayout {
    create_default_layout(&[crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID.into(), crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID.into(), crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID.into()], "row", Some(&[50.0, 25.0, 25.0]), Some(&["Overview".into(), "Detail".into(), "Selection".into()]))
}
//#endregion 🔖️Definition

pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::canvas;

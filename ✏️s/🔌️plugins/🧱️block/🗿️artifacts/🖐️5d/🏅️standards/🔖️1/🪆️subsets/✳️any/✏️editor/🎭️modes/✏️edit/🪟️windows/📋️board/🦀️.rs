//! 📋️ Block 5D play app — the board window: a lightweight 2D-projection summary surface.

use crate::Block5dSnapshot;
use crate::editor::block5d::terminology::Block5dLabels;
use crate::editor::block5d::ui_label;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, PluginAssemblyError, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
// 🚧️ SDK GAP: the block crate has no direct `semio-framework-ui-contract` dependency (unlike puzzle/
// lowpoly), so the contract's node builders are reached through the plugin SDK's own re-export.
use semio_framework_plugin::plugin_app_close_prelude as ui;

//#region 🔖️Constants
pub const BLOCK5D_WINDOW_BOARD: &str = "block5d-board";
pub const BLOCK5D_BODY_BOARD: &str = "block5d.play.board";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::block5d::create_block5d_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: BLOCK5D_WINDOW_BOARD.into(),
        label: LocalizedLabel::native("Board", "Board"),
        body_key: BLOCK5D_BODY_BOARD.into(),
        surface_kind: SurfaceKind::Board2d,
        icon_id: "layout-grid".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn board_error(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("block5d board admission failed at {stage}"))
}

/// 🪧️ One summary line. The id is NOT decoration: two sibling nodes without one project to the same
/// key and the whole body is refused with `duplicate-key` before it reaches a renderer.
fn line(id: &str, value: &str, stage: &'static str) -> UiAssemblyResult<BuiltNode> {
    ui::text(ui_label(value)?).try_id(id).map_err(|_| board_error(stage))?.try_build().map_err(|_| board_error(stage))
}

pub fn render(definition: &Block5dSnapshot, labels: &Block5dLabels) -> UiAssemblyResult<BuiltNode> {
    let summary = line("block5d-play-board.summary", &format!("{}: {}", labels.summary.as_str(), if definition.part_kind.label.is_empty() { "—" } else { &definition.part_kind.label }), "summary")?;
    let grips = line("block5d-play-board.grips", &format!("2d grips: {}", definition.grips.len()), "grips")?;
    ui::column().try_id("block5d-play-board.body").map_err(|_| board_error("id"))?.try_children([summary, grips]).map_err(|_| board_error("children"))?.try_build().map_err(|_| board_error("build"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

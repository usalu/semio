//! 📋️ Block 2D play app — the board window: a lightweight summary surface (block2d's only window
//! kind; the full node-kind editing surface lives in the document/inspection panels).

use crate::Block2dSnapshot;
use crate::editor::block2d::terminology::Block2dLabels;
use crate::editor::block2d::ui_label;
use semio_framework_plugin::plugin_app_close_prelude::{column, text, Buildable, HasBase, HasChildren};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, PluginAssemblyError, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const BLOCK2D_WINDOW_BOARD: &str = "block2d-board";
pub const BLOCK2D_BODY_BOARD: &str = "block2d.play.board";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::block2d::create_block2d_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: BLOCK2D_WINDOW_BOARD.into(),
        label: LocalizedLabel::native("Node Kind", "Knotenart"),
        body_key: BLOCK2D_BODY_BOARD.into(),
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
/// 🪧️ One board summary line. The id is NOT decoration: two sibling nodes without one project to the
/// same key and the whole body is refused with `duplicate-key` before it ever reaches a renderer.
fn line(id: &str, value: String) -> UiAssemblyResult<BuiltNode> {
    text(ui_label(value)?)
        .try_id(id)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "block2d board line id admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "block2d board line admission failed"))
}

pub fn render(definition: &Block2dSnapshot, labels: &Block2dLabels) -> UiAssemblyResult<BuiltNode> {
    let lines = semio_framework_plugin::ui_node_list([
        line("block2d-play-board.summary", format!("{}: {}", labels.summary.as_str(), if definition.node_kind.label.is_empty() { "—" } else { &definition.node_kind.label })),
        line("block2d-play-board.counts", format!("{} {}, {} {}", definition.handle_kinds.len(), labels.handle_kinds.as_str(), definition.handles.len(), labels.handles.as_str())),
    ])?;
    column()
        .try_id("block2d-play-board.body")
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "block2d board id admission failed"))?
        .try_children(lines)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "block2d board children admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "block2d board admission failed"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

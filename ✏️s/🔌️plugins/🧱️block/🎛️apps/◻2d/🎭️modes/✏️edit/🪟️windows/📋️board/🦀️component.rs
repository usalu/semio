//! 📋️ Block 2D play app — the board window: a lightweight summary surface (block2d's only window
//! kind; the full node-kind editing surface lives in the document/inspection panels).

use crate::apps::block2d::terminology::Block2dLabels;
use crate::artifacts::block2d::Block2dSnapshot;
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const BLOCK2D_WINDOW_BOARD: &str = "block2d-board";
pub const BLOCK2D_BODY_BOARD: &str = "block2d.play.board";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::block2d::create_block2d_app`.
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
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(definition: &Block2dSnapshot, labels: &Block2dLabels) -> UiNode {
    ui_stack_vertical(vec![
        ui_text(Label::data(format!("{}: {}", labels.summary.as_str(), if definition.node_kind.label.is_empty() { "—" } else { &definition.node_kind.label }))),
        ui_text(Label::data(format!("{} {}, {} {}", definition.handle_kinds.len(), labels.handle_kinds.as_str(), definition.handles.len(), labels.handles.as_str()))),
    ])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_the_board_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, BLOCK2D_BODY_BOARD);
        assert!(matches!(definition.surface_kind, SurfaceKind::Board2d));
    }
}
//#endregion 🧪️Tests

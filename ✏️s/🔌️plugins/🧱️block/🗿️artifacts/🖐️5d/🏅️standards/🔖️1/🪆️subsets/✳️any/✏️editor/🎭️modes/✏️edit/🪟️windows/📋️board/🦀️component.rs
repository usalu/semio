//! 📋️ Block 5D play app — the board window: a lightweight 2D-projection summary surface.

use crate::editor::block5d::terminology::Block5dLabels;
use crate::artifacts::block5d::Block5dSnapshot;
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const BLOCK5D_WINDOW_BOARD: &str = "block5d-board";
pub const BLOCK5D_BODY_BOARD: &str = "block5d.play.board";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::block5d::create_block5d_app`.
pub async fn definition() -> WindowKindDefinition {
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
pub async fn render(definition: &Block5dSnapshot, labels: &Block5dLabels) -> UiNode {
    ui_stack_vertical(vec![
        ui_text(Label::data(format!("{}: {}", labels.summary.as_str(), if definition.part_kind.label.is_empty() { "—" } else { &definition.part_kind.label }))),
        ui_text(Label::data(format!("2d grips: {}", definition.grips.len()))),
    ])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn definition_declares_the_board_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, BLOCK5D_BODY_BOARD);
        assert!(matches!(definition.surface_kind, SurfaceKind::Board2d));
    }
}
//#endregion 🧪️Tests

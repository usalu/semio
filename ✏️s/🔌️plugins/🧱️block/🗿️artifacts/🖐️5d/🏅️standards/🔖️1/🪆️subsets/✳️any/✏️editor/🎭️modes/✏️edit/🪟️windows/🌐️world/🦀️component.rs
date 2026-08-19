//! 🌐️ Block 5D play app — the world window: a lightweight 3D-projection summary surface.

use crate::editor::block5d::terminology::Block5dLabels;
use crate::artifacts::block5d::Block5dSnapshot;
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const BLOCK5D_WINDOW_WORLD: &str = "block5d-world";
pub const BLOCK5D_BODY_WORLD: &str = "block5d.play.world";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::block5d::create_block5d_app`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: BLOCK5D_WINDOW_WORLD.into(),
        label: LocalizedLabel::native("World", "Welt"),
        body_key: BLOCK5D_BODY_WORLD.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "box".into(),
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
    let mesh_url = definition.representations.first().and_then(|representation| representation.mesh_url.as_deref()).unwrap_or("—");
    ui_stack_vertical(vec![ui_text(Label::data(format!("{}: {}", labels.summary.as_str(), if definition.part_kind.label.is_empty() { "—" } else { &definition.part_kind.label }))), ui_text(Label::data(format!("mesh: {mesh_url}")))])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn definition_declares_the_world_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, BLOCK5D_BODY_WORLD);
        assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
    }
}
//#endregion 🧪️Tests

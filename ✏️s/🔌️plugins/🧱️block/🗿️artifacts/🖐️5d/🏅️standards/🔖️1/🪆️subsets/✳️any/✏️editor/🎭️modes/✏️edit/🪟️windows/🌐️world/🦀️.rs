//! 🌐️ Block 5D play app — the world window: a lightweight 3D-projection summary surface.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::editor::block5d::terminology::Block5dLabels;
use crate::editor::block5d::ui_label;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasChildren};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, PluginAssemblyError, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
// 🚧️ SDK GAP: the block crate has no direct `semio-framework-ui-contract` dependency (unlike puzzle/
// lowpoly), so the contract's node builders are reached through the plugin SDK's own re-export.
use semio_framework_plugin::plugin_app_close_prelude as ui;

//#region 🔖️Constants
pub const BLOCK5D_WINDOW_WORLD: &str = "block5d-world";
pub const BLOCK5D_BODY_WORLD: &str = "block5d.play.world";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::block5d::create_block5d_app`.
pub fn definition() -> WindowKindDefinition {
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
fn world_error(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("block5d world admission failed at {stage}"))
}

fn line(value: &str, stage: &'static str) -> UiAssemblyResult<BuiltNode> {
    ui::text(ui_label(value)?).try_build().map_err(|_| world_error(stage))
}

pub fn render(definition: &Block5dSnapshot, labels: &Block5dLabels) -> UiAssemblyResult<BuiltNode> {
    let mesh_url = definition.representations.first().and_then(|representation| representation.mesh_url.as_deref()).unwrap_or("—");
    let summary = line(&format!("{}: {}", labels.summary.as_str(), if definition.part_kind.label.is_empty() { "—" } else { &definition.part_kind.label }), "summary")?;
    let mesh = line(&format!("mesh: {mesh_url}"), "mesh")?;
    ui::column().try_children([summary, mesh]).map_err(|_| world_error("children"))?.try_build().map_err(|_| world_error("build"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_world_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, BLOCK5D_BODY_WORLD);
        assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
    }
}
//#endregion 🧪️Tests

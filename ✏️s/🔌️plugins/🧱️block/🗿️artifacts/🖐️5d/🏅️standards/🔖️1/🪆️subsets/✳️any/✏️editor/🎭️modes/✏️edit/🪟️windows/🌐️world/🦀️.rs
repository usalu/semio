//! 🌐️ Block 5D play app — the world window: a lightweight 3D-projection summary surface.

use crate::Block5dSnapshot;
use crate::editor::block5d::terminology::Block5dLabels;
use crate::editor::block5d::ui_label;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren};
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

/// 🪧️ One summary line. The id is NOT decoration: two sibling nodes without one project to the same
/// key and the whole body is refused with `duplicate-key` before it reaches a renderer.
fn line(id: &str, value: &str, stage: &'static str) -> UiAssemblyResult<BuiltNode> {
    ui::text(ui_label(value)?).try_id(id).map_err(|_| world_error(stage))?.try_build().map_err(|_| world_error(stage))
}

pub fn render(definition: &Block5dSnapshot, labels: &Block5dLabels) -> UiAssemblyResult<BuiltNode> {
    let mesh_url = definition.representations.first().and_then(|representation| representation.mesh_url.as_deref()).unwrap_or("—");
    let summary = line("block5d-play-world.summary", &format!("{}: {}", labels.summary.as_str(), if definition.part_kind.label.is_empty() { "—" } else { &definition.part_kind.label }), "summary")?;
    let mesh = line("block5d-play-world.mesh", &format!("mesh: {mesh_url}"), "mesh")?;
    ui::column().try_id("block5d-play-world.body").map_err(|_| world_error("id"))?.try_children([summary, mesh]).map_err(|_| world_error("children"))?.try_build().map_err(|_| world_error("build"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

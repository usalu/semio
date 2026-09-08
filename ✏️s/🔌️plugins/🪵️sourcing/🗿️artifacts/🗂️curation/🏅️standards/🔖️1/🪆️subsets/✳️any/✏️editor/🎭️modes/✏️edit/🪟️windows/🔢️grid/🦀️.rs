//! 🔢️ Sourcing curation app — the grid window: every filtered stock object laid out on a 3D grid.

use crate::schema::{filtered_stock, grid_placement, grid_scale, instance_json, kind_mesh_json};
use crate::CurationSnapshot;
use crate::editor::sourcing::config::SourcingCurationConfig;
use semio_framework_plugin::app::WindowKit;
use semio_framework_plugin::{world3d_default_camera, world3d_selection_json, BuiltNode, LocalizedLabel, MeshView, MeshWindowKit, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use std::collections::HashSet;

//#region 🔖️Constants
pub const SOURCING_CURATION_WINDOW_GRID: &str = "sourcing-grid";
pub const SOURCING_CURATION_BODY_GRID: &str = "sourcing.grid";
const SOURCING_CURATION_GRID_CELL: f64 = 2.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SOURCING_CURATION_WINDOW_GRID.into(),
        label: LocalizedLabel::native("Grid", "Raster"),
        body_key: SOURCING_CURATION_BODY_GRID.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "grid-3x3".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        interactions: Vec::new(),
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
pub fn render(document: &CurationSnapshot, cfg: &SourcingCurationConfig) -> UiAssemblyResult<BuiltNode> {
    let filtered = filtered_stock(document, &cfg.filters);
    let mut seen_mesh_ids = HashSet::new();
    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    for (index, kind) in filtered.iter().enumerate() {
        if seen_mesh_ids.insert(kind.id.clone()) {
            meshes.push(kind_mesh_json(kind));
        }
        let (x, z) = grid_placement(filtered.len(), index, SOURCING_CURATION_GRID_CELL);
        let scale = grid_scale(&kind.geometry, SOURCING_CURATION_GRID_CELL * 0.8);
        // 🕹️ The "rows" selection now lives in the framework-owned interaction domain (ticket
        // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — `ArtifactApp::render` carries no
        // `InteractionView`, so this scene payload can no longer embed a live selection.
        instances.push(instance_json(kind, [x, 0.0, z], scale, false));
    }
    MeshWindowKit::render(&MeshView {
        camera_json: world3d_default_camera(),
        meshes_json: dsl::json::to_json_string(&dsl::DslValue::Array(meshes)),
        instances_json: dsl::json::to_json_string(&dsl::DslValue::Array(instances)),
        selection_json: world3d_selection_json("rectangle", &[], None),
    })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

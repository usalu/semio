//! 🔢️ Sourcing curation app — the grid window: every curated object laid out on a 3D grid.

use crate::editor::sourcing::modes::edit::windows::curated::curated_rows;
use crate::schema::{box_parts, grid_placement, grid_scale, kind_instances_json, kind_mesh_json, unit_box_mesh_json};
use crate::CurationSnapshot;
use crate::editor::sourcing::config::SourcingCurationConfig;
use crate::ObjectKind;
use semio_framework_plugin::app::WindowKit;
use semio_framework_plugin::{world3d_default_camera, world3d_selection_json, BuiltNode, LocalizedLabel, MeshView, MeshWindowKit, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

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
fn curated_grid_slots(document: &CurationSnapshot, cfg: &SourcingCurationConfig) -> Vec<ObjectKind> {
    curated_rows(document, cfg)
        .into_iter()
        .flat_map(|(item, kind)| std::iter::repeat_n(kind, item.count as usize))
        .collect()
}

pub fn render(document: &CurationSnapshot, cfg: &SourcingCurationConfig) -> UiAssemblyResult<BuiltNode> {
    let slots = curated_grid_slots(document, cfg);
    let mut meshes = Vec::new();
    if slots.iter().any(|kind| box_parts(&kind.geometry).is_some()) {
        meshes.push(unit_box_mesh_json());
    }
    let mut mesh_ids = std::collections::BTreeSet::new();
    for kind in &slots {
        if box_parts(&kind.geometry).is_none() && mesh_ids.insert(kind.id.clone()) {
            meshes.push(kind_mesh_json(kind));
        }
    }
    let mut instances = Vec::new();
    for (index, kind) in slots.iter().enumerate() {
        let (x, z) = grid_placement(slots.len(), index, SOURCING_CURATION_GRID_CELL);
        let scale = grid_scale(&kind.geometry, SOURCING_CURATION_GRID_CELL * 0.8);
        instances.extend(kind_instances_json(kind, [x, 0.0, z], scale, false));
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

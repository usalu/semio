//! 🔢️ Sourcing curation app — the grid window: every curated object laid out on a 3D grid.

use crate::editor::sourcing::modes::edit::windows::curated::curated_rows;
use crate::editor::sourcing::modes::edit::windows::grid::config::{
    GridWindowConfig, GRID_INSTANCE_DISPLAY_LINE_BEHIND, GRID_INSTANCE_DISPLAY_REPRESENTATIVE, GRID_INSTANCE_DISPLAY_REPRESENTATIVE_WITH_COUNT,
};
use crate::editor::sourcing::terminology::SourcingLabels;
use crate::schema::{append_grid_count_glyph_instances, append_kind_scene_meshes, box_parts, grid_placement, grid_scale, kind_instances_json, unit_box_mesh_json};
use crate::CurationSnapshot;
use crate::editor::sourcing::config::SourcingCurationConfig;
use crate::ObjectKind;
use semio_framework_plugin::app::WindowKit;
use semio_framework_plugin::{world3d_default_camera, world3d_selection_json, BuiltNode, LocalizedLabel, MeshView, MeshWindowKit, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowMeasure, WindowOptions};

//#region 🔖️Constants
pub const SOURCING_CURATION_WINDOW_GRID: &str = "sourcing-grid";
pub const SOURCING_CURATION_BODY_GRID: &str = "sourcing.grid";
const SOURCING_CURATION_GRID_CELL: f64 = 2.0;
const GRID_LINE_BEHIND_SPACING: f64 = 0.35;
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

pub fn window_measures(cfg: &GridWindowConfig, labels: &SourcingLabels) -> Vec<WindowMeasure> {
    vec![crate::editor::sourcing::modes::edit::windows::grid::instance_display::measure(cfg, labels)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
struct CuratedGridRow {
    kind: ObjectKind,
    count: u32,
}

fn curated_grid_rows(document: &CurationSnapshot, cfg: &SourcingCurationConfig) -> Vec<CuratedGridRow> {
    curated_rows(document, cfg).into_iter().map(|(item, kind)| CuratedGridRow { kind, count: item.count }).collect()
}

fn line_behind_offset(copy_index: usize, cell: f64) -> f64 {
    -(copy_index as f64) * cell * GRID_LINE_BEHIND_SPACING
}

fn push_kind_copies(instances: &mut Vec<dsl::DslValue>, kind: &ObjectKind, count: u32, position: [f64; 3], scale: f64, line_behind: bool) {
    let copies = if line_behind { count.max(1) as usize } else { 1 };
    for copy in 0..copies {
        let z = position[2] + if line_behind { line_behind_offset(copy, SOURCING_CURATION_GRID_CELL) } else { 0.0 };
        instances.extend(kind_instances_json(kind, [position[0], position[1], z], scale, false));
    }
}

pub fn render(document: &CurationSnapshot, cfg: &SourcingCurationConfig, window: &GridWindowConfig) -> UiAssemblyResult<BuiltNode> {
    let rows = curated_grid_rows(document, cfg);
    let display = window.instance_display.as_str();
    let mut meshes = Vec::new();
    let needs_unit_box = rows.iter().any(|row| box_parts(&row.kind.geometry).is_some())
        || display == GRID_INSTANCE_DISPLAY_REPRESENTATIVE_WITH_COUNT;
    if needs_unit_box {
        meshes.push(unit_box_mesh_json());
    }
    let mut mesh_ids = std::collections::BTreeSet::new();
    for row in &rows {
        append_kind_scene_meshes(&mut meshes, &mut mesh_ids, &row.kind);
    }
    let mut instances = Vec::new();
    let row_count = rows.len();
    for (index, row) in rows.iter().enumerate() {
        let (x, z) = grid_placement(row_count, index, SOURCING_CURATION_GRID_CELL);
        let scale = grid_scale(&row.kind.geometry, SOURCING_CURATION_GRID_CELL * 0.8);
        let anchor = [x, 0.0, z];
        match display {
            GRID_INSTANCE_DISPLAY_REPRESENTATIVE => push_kind_copies(&mut instances, &row.kind, 1, anchor, scale, false),
            GRID_INSTANCE_DISPLAY_REPRESENTATIVE_WITH_COUNT => {
                push_kind_copies(&mut instances, &row.kind, 1, anchor, scale, false);
                append_grid_count_glyph_instances(&mut instances, &row.kind.id, row.count, anchor, SOURCING_CURATION_GRID_CELL);
            }
            _ => push_kind_copies(&mut instances, &row.kind, row.count, anchor, scale, true),
        }
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

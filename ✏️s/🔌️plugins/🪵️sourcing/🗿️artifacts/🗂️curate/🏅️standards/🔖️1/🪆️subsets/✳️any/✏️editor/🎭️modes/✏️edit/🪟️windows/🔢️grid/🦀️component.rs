//! 🔢️ Sourcing curate app — the grid window: every filtered stock object laid out on a 3D grid.

use crate::editor::sourcing::config::SourcingCurateConfig;
use crate::editor::sourcing::SOURCING_CONTROLLER_ID;
use crate::artifacts::curate::schema::{filtered_stock, grid_placement, grid_scale, instance_json, kind_mesh_json};
use crate::artifacts::curate::CurateSnapshot;
use semio_framework_plugin::{build_world_3d_scene, world3d_default_camera, world3d_scene, world3d_selection_json, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions, WorldSunConfig};
use serde_json::json;
use std::collections::HashSet;

//#region 🔖️Constants
pub const SOURCING_CURATE_WINDOW_GRID: &str = "sourcing-grid";
pub const SOURCING_CURATE_BODY_GRID: &str = "sourcing.grid";
const SOURCING_CURATE_SURFACE_GRID: &str = "sourcing.grid.world";
const SOURCING_CURATE_GRID_CELL: f64 = 2.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SOURCING_CURATE_WINDOW_GRID.into(),
        label: LocalizedLabel::native("Grid", "Raster"),
        body_key: SOURCING_CURATE_BODY_GRID.into(),
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
pub async fn render(document: &CurateSnapshot, cfg: &SourcingCurateConfig) -> UiNode {
    let filtered = filtered_stock(document, &cfg.filters);
    let mut seen_mesh_ids = HashSet::new();
    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    for (index, kind) in filtered.iter().enumerate() {
        if seen_mesh_ids.insert(kind.id.clone()) {
            meshes.push(kind_mesh_json(kind));
        }
        let (x, z) = grid_placement(filtered.len(), index, SOURCING_CURATE_GRID_CELL);
        let scale = grid_scale(&kind.geometry, SOURCING_CURATE_GRID_CELL * 0.8);
        // 🕹️ The "rows" selection now lives in the framework-owned interaction domain (ticket
        // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — `ArtifactApp::render` carries no
        // `InteractionView`, so this scene payload can no longer embed a live selection.
        instances.push(instance_json(kind, [x, 0.0, z], scale, false));
    }
    let mut scene = world3d_scene(world3d_default_camera(), json!(meshes).to_string(), json!(instances).to_string(), world3d_selection_json("rectangle", &[], None), &WorldSunConfig::default());
    scene.fit_json = Some(json!({ "enabled": true, "padding": 0.3 }).to_string());
    build_world_3d_scene(SOURCING_CURATE_SURFACE_GRID, SOURCING_CONTROLLER_ID, scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::sourcing::testkit::{new_app, render as render_body};
    use crate::artifacts::curate::Filters;

    #[semio_framework_async_macros::async_test]
    async fn grid_instance_count_matches_filtered_stock_and_normalizes_scale() {
        let document = crate::artifacts::curate::schema::default_document();
        let cfg = SourcingCurateConfig { filters: Filters { module_ids: vec!["slabs".into()], ..Default::default() }, ..Default::default() };
        let node = render(&document, &cfg);
        let json = serde_json::to_value(&node).unwrap();
        let instances_json = json.pointer("/world3d/instancesJson").and_then(|value| value.as_str()).unwrap();
        let instances: Vec<serde_json::Value> = serde_json::from_str(instances_json).unwrap();
        assert_eq!(instances.len(), filtered_stock(&document, &cfg.filters).len());
        for instance in &instances {
            let scale = instance["scale"][0].as_f64().unwrap();
            assert!(scale > 0.0);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_world3d_surface_and_body_key() {
        let def = definition();
        assert_eq!(def.body_key, SOURCING_CURATE_BODY_GRID);
        assert!(matches!(def.surface_kind, SurfaceKind::World3d));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_via_the_app() {
        let mut app = new_app();
        assert!(render_body(&mut app, SOURCING_CURATE_BODY_GRID).contains("world3d"));
    }
}
//#endregion 🧪️Tests

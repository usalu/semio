//! 🔢️ Sourcing curate app — the grid window: every filtered stock object laid out on a 3D grid.

use crate::artifacts::curate::schema::{filtered_stock, grid_placement, grid_scale, instance_json, kind_mesh_json};
use crate::artifacts::curate::CurateSnapshot;
use crate::editor::sourcing::config::SourcingCurateConfig;
use semio_framework_plugin::app::WindowKit;
use semio_framework_plugin::{world3d_default_camera, world3d_selection_json, BuiltNode, LocalizedLabel, MeshView, MeshWindowKit, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use std::collections::HashSet;

//#region 🔖️Constants
pub const SOURCING_CURATE_WINDOW_GRID: &str = "sourcing-grid";
pub const SOURCING_CURATE_BODY_GRID: &str = "sourcing.grid";
const SOURCING_CURATE_GRID_CELL: f64 = 2.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
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
pub fn render(document: &CurateSnapshot, cfg: &SourcingCurateConfig) -> UiAssemblyResult<BuiltNode> {
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
mod tests {
    use super::*;
    use crate::artifacts::curate::Filters;
    use crate::editor::sourcing::testkit::{new_app, render as render_body};

    /// 🎬️ Asserts through the packed scene, not the node JSON — see the preview window's sibling test
    /// for why `serde_json::to_string(&node)` can no longer carry any id.
    #[semio_framework_async_macros::async_test]
    async fn grid_instance_count_matches_filtered_stock_and_normalizes_scale() {
        let document = crate::artifacts::curate::schema::default_document();
        let cfg = SourcingCurateConfig { filters: Filters { module_ids: vec!["slabs".into()], ..Default::default() }, ..Default::default() };
        let node = render(&document, &cfg).expect("bounded grid");
        let semio_framework_plugin::Component::Surface(props) = node.component else { panic!("grid must build a World3d surface") };
        let scene: semio_framework_ui_scene::World3dScene = semio_framework_ui_scene::decode(&props).expect("decode world3d scene");
        let filtered = filtered_stock(&document, &cfg.filters);
        assert!(!filtered.is_empty(), "the slabs module must contribute stock");
        for kind in &filtered {
            assert!(scene.meshes_json.contains(&kind.id), "{} must contribute a mesh", kind.id);
            assert!(scene.instances_json.contains(&kind.id), "{} must contribute an instance", kind.id);
        }
        assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.instances_json).unwrap().as_array().unwrap().len(), filtered.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_world3d_surface_and_body_key() {
        let def = definition();
        assert_eq!(def.body_key, SOURCING_CURATE_BODY_GRID);
        assert!(matches!(def.surface_kind, SurfaceKind::World3d));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_via_the_app() {
        let mut app = new_app().await;
        assert!(render_body(&mut app, SOURCING_CURATE_BODY_GRID).await.contains("world3d"));
    }
}
//#endregion 🧪️Tests

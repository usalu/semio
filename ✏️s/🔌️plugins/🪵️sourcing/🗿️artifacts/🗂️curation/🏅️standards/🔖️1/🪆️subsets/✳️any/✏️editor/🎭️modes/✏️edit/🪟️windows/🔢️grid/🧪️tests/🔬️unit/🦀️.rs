
use super::*;
use crate::editor::sourcing::SourcingCurationCommand;
use crate::editor::sourcing::commands::curation_add;
use crate::editor::sourcing::unit_tests::context::{dispatch, new_app};
use crate::schema::{box_parts, curation_set, filtered_stock};
use crate::Filters;

/// 🎬️ Asserts through the packed scene, not the node JSON — see the preview window's sibling test
/// for why `serde_json::to_string(&node)` can no longer carry any id.
#[semio_framework_async_macros::async_test]
async fn grid_instance_count_matches_curated_counts_and_normalizes_scale() {
    let mut document = crate::schema::default_document();
    curation_set(&mut document, "slab-clt-160", 2);
    let cfg = SourcingCurationConfig { filters: Filters { module_ids: vec!["slabs".into()], ..Default::default() }, ..Default::default() };
    let node = render(&document, &cfg).expect("bounded grid");
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("assemble world3d scene");
    assert!(filtered_stock(&document, &cfg.filters).len() > 1, "stock still has more slabs than the curated subset");
    let parts: usize = 2 * box_parts(&crate::stock_of(&document).iter().find(|kind| kind.id == "slab-clt-160").expect("stock kind").geometry).map_or(1, |parts| parts.len());
    assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.instances_json).unwrap().as_array().unwrap().len(), parts);
    assert!(scene.meshes_json.contains(crate::schema::SOURCING_UNIT_BOX_MESH_ID), "box-built stock shares the unit box mesh");
    assert!(scene.instances_json.contains("slab-clt-160"), "curated kind must contribute instances");
}

/// 🧺️ A fresh curation renders an empty grid — stock lives in the pool, not the grid.
#[semio_framework_async_macros::async_test]
async fn grid_renders_empty_when_nothing_is_curated() {
    let document = crate::schema::default_document();
    let cfg = SourcingCurationConfig::default();
    let node = render(&document, &cfg).expect("bounded empty grid");
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("assemble world3d scene");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.meshes_json).unwrap().as_array().unwrap().len(), 0);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.instances_json).unwrap().as_array().unwrap().len(), 0);
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_world3d_surface_and_body_key() {
    let def = definition();
    assert_eq!(def.body_key, SOURCING_CURATION_BODY_GRID);
    assert!(matches!(def.surface_kind, SurfaceKind::World3d));
}

#[semio_framework_async_macros::async_test]
async fn renders_via_the_app() {
    let mut app = new_app().await;
    let object_id = "beam-glulam-gl24h";
    dispatch(&mut app, SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: object_id.into() })).await;
    let rendered = semio_framework_plugin::PluginApp::render(&mut *app, SOURCING_CURATION_BODY_GRID, None, &semio_framework_plugin::ViewModel::default()).await.expect("render");
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&rendered.root).expect("assemble world3d scene");
    assert!(scene.meshes_json.contains(crate::schema::SOURCING_UNIT_BOX_MESH_ID));
    assert!(scene.instances_json.contains(object_id));
}

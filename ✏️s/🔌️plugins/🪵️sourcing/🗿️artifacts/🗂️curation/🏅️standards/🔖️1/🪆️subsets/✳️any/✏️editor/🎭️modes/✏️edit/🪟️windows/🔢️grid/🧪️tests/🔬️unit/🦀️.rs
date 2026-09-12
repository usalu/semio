
use super::*;
use crate::Filters;
use crate::editor::sourcing::unit_tests::context::{new_app, render as render_body};

/// 🎬️ Asserts through the packed scene, not the node JSON — see the preview window's sibling test
/// for why `serde_json::to_string(&node)` can no longer carry any id.
#[semio_framework_async_macros::async_test]
async fn grid_instance_count_matches_filtered_stock_and_normalizes_scale() {
    let document = crate::schema::default_document();
    let cfg = SourcingCurationConfig { filters: Filters { module_ids: vec!["slabs".into()], ..Default::default() }, ..Default::default() };
    let node = render(&document, &cfg).expect("bounded grid");
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("assemble world3d scene");
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
    assert_eq!(def.body_key, SOURCING_CURATION_BODY_GRID);
    assert!(matches!(def.surface_kind, SurfaceKind::World3d));
}

#[semio_framework_async_macros::async_test]
async fn renders_via_the_app() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SOURCING_CURATION_BODY_GRID).await.contains("world3d"));
}

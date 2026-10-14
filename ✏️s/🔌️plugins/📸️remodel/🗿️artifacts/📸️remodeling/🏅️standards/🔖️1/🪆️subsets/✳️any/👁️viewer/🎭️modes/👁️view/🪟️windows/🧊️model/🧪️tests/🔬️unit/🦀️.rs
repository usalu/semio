use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_world3d_model_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.surface_kind, SurfaceKind::World3d);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let scene = semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::default_remodeling_scene();
    let _node = render(&scene);
}

#[semio_framework_async_macros::async_test]
async fn world_meshes_json_renders_the_real_placeholder_mesh_when_the_cache_is_warm() {
    let scene = semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::default_remodeling_scene();
    assert!(world_meshes_json(&scene).contains(REMODELING_VIEW_MESH_ID));
}

#[semio_framework_async_macros::async_test]
async fn world_instances_json_is_never_gated_on_a_layer_toggle() {
    assert!(world_instances_json().contains(REMODELING_VIEW_MESH_ID));
}

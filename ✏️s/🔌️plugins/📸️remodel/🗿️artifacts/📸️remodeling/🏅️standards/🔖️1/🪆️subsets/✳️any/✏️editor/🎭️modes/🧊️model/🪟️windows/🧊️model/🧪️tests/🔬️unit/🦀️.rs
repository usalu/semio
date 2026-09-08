
use super::*;
use crate::default_remodeling_scene;
use crate::editor::remodeling::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn default_scene_seeds_the_world3d_mesh_json() {
    let scene = default_remodeling_scene();
    assert!(world_meshes_json(&scene).contains(REMODELING_MESH_ID));
    let config = RemodelingConfig::default();
    assert!(world_instances_json(&config).contains(REMODELING_MESH_ID));
}

#[semio_framework_async_macros::async_test]
async fn hiding_the_mesh_layer_drops_the_instance() {
    let mut config = RemodelingConfig::default();
    config.layers.mesh = false;
    assert_eq!(world_instances_json(&config), "[]");
}

#[semio_framework_async_macros::async_test]
async fn renders_a_world_3d_surface() {
    let mut app = app().await;
    assert!(render_body(&mut app, REMODELING_PLAY_BODY_MAIN).await.contains("world-3d"));
}

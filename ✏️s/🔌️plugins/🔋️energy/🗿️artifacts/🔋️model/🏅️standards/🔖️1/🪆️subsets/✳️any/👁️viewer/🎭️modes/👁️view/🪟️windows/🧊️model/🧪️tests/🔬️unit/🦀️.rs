use super::*;
use semio_framework_plugin::World3dScene;

fn scene_of(node: &BuiltNode) -> World3dScene {
    semio_framework_plugin::artifact_app_laws::built_surface_scene(node).expect("the built node decodes back into a world scene")
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_world3d_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert_eq!(def.surface_kind, SurfaceKind::World3d);
    assert!(def.actions.is_empty(), "a viewer window declares no verbs");
    assert!(def.interactions.is_empty(), "a viewer declares no interaction domain");
}

#[semio_framework_async_macros::async_test]
async fn render_shows_the_same_geometry_as_the_editor_twin_without_a_domain() {
    let model = crate::examples::bestest_600::model();
    let node = render(&model).expect("the world surface assembles");
    let scene = scene_of(&node);
    assert!(scene.domain_id.is_none(), "a read-only viewer never binds a picking domain");
    assert!(scene.domain_granularity_id.is_none());
    for surface in &model.surfaces {
        assert!(scene.instances_json.contains(&format!("\"id\":\"{}\"", surface.id.0)), "surface {} has no instance: {}", surface.id.0, scene.instances_json);
    }
    for window in &model.fenestrations {
        assert!(scene.instances_json.contains(&format!("\"id\":\"{}\"", window.id.0)), "window {} has no instance: {}", window.id.0, scene.instances_json);
    }

    // 🧊️ The two surfaces must publish byte-identical geometry: one builder, two windows.
    let (meshes_json, instances_json) = crate::scene::energy_model_scene_parts(&model, &EnergySceneStyle::default());
    assert_eq!(scene.meshes_json, meshes_json);
    assert_eq!(scene.instances_json, instances_json);
}

#[semio_framework_async_macros::async_test]
async fn an_empty_model_renders_without_faulting() {
    let scene = scene_of(&render(&crate::model::Model::default()).expect("an empty model still assembles a surface"));
    assert_eq!(scene.meshes_json, "[]");
    assert_eq!(scene.instances_json, "[]");
}

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
    // 🎥️ The ONE verb a read-only 3d window owns: the orbit pose the host dispatches on its own.
    assert_eq!(def.actions.len(), 1, "a viewer window declares its camera and nothing else");
    assert_eq!(def.actions[0].id, SET_CAMERA_ACTION_ID);
    assert_eq!(def.actions[0].kind, ActionKind::View, "a camera is never a document mutation");
    assert_eq!(def.actions[0].semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "Migrated is the only UI-dispatchable classification");
    assert!(def.actions[0].args.iter().any(|arg| arg.id == "camera"), "an undeclared `camera` arg is filtered out by effective_action_args before the bridge sees it");
    assert!(def.interactions.is_empty(), "a viewer declares no interaction domain");
}

#[semio_framework_async_macros::async_test]
async fn a_stored_camera_replaces_the_model_derived_one_and_changes_nothing_else() {
    let model = crate::examples::bestest_600::model();
    let bare = scene_of(&render(&model).expect("the default render assembles"));
    let pose = config::EnergyModelViewerCameraPose { position: [12.0, -9.0, 7.5], target: [4.0, 3.0, 1.35], zoom: 1.0 };
    let stored = scene_of(&render_with_camera(&model, Some(&config::EnergyModelViewerWindowConfig { camera: pose })).expect("the stored render assembles"));

    assert_ne!(bare.camera_json, stored.camera_json, "a retained pose really replaces the model-derived camera");
    assert_eq!(stored.camera_json, pose.scene_camera_json());
    // 🎥️ …and NOTHING else moves, so republishing the host's own echo never re-arms the auto fit.
    assert_eq!(bare.meshes_json, stored.meshes_json);
    assert_eq!(bare.instances_json, stored.instances_json);
    assert_eq!(bare.fit_json, stored.fit_json);
    assert!(stored.domain_id.is_none(), "retaining a camera does not make a viewer pickable");
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

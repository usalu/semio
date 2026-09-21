
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_shared_mesh_window_kind() {
    let def = definition();
    assert_eq!(def.id, "framework.window.mesh");
    assert_eq!(def.body_key, "framework.window.mesh");
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_empty_document() {
    let document = crate::standards::v1::subsets::any::schema::empty_block3d_snapshot();
    let node = render(&document).expect("the empty document must still assemble a scene surface");
    assert!(matches!(node.component, semio_framework_plugin::Component::Surface(_)));
}

/// 🎯️ A viewer declares no camera action at all (`Block3dViewCommand::Noop`) and boots on the
/// curated example, so an authored `camera3d` that misses the representation mesh would leave this
/// surface permanently empty with no way for the reader to recover it — it stages the same one-shot
/// fit as the editor's world window (ticket 26/09/19 play-grid visual audit, class 1).
#[semio_framework_async_macros::async_test]
async fn render_stages_a_one_shot_fit_for_the_booted_example() {
    let document = crate::standards::v1::subsets::any::schema::snapshot::text::block3d_boot_snapshot();
    assert!(!document.representations.is_empty(), "the booted example must carry the representation whose mesh this window frames");
    let node = render(&document).expect("the booted example assembles its world surface");
    let scene: World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("the world surface decodes as a 3d scene");
    let fit = scene.fit_json.as_deref().expect("the viewer's world window must publish a fit lane");
    assert!(fit.contains("\"enabled\":true"), "the fit lane must be enabled, else the authored camera is the only framing: {fit}");
    assert!(fit.contains(&format!("\"revision\":{}", crate::block3d_world_fit_revision(&document))), "the lane must carry this document's own identity revision: {fit}");
}

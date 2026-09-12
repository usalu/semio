use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_an_ink_canvas_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.surface_kind, SurfaceKind::InkCanvas);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_read_only_ink_canvas_scene_for_the_empty_document() {
    let document = crate::schema::empty_note_snapshot();
    let node = render(&document).expect("viewer canvas");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("semantic canvas") };
    let scene: InkCanvasScene = semio_framework_ui_scene::decode(props).expect("packed viewer scene");
    assert!(!scene.interactive);
    assert!(scene.active_utility.is_empty());
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire viewer canvas");
}

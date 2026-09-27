use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_canvas2d_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.surface_kind, SurfaceKind::Canvas2d);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = crate::schema::default_drawing_document("empty", None);
    let _node = render(&document);
}

#[test]
fn drawing_viewer_framing_uses_the_artifact_world_bounds() {
    let document = crate::DrawingSnapshot { artboard: Some(crate::DrawingArtboard { width: 200.0,height: 300.0 }),..Default::default() };
    let tree = semio_framework_plugin::built_to_component_tree(render(&document).unwrap());
    let scene: Canvas2dScene = match &tree.root.component {
        semio_framework_plugin::Component::Surface(surface) => semio_framework_ui_scene::decode(surface).unwrap(),
        _ => panic!("viewer must publish a canvas surface"),
    };
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap();
    assert_eq!(scene.framing.unwrap().bounds,[0.0,0.0,200.0,300.0]);
    eprintln!("[DEBUG] Drawing viewer publishes artifact bounds for measured initial fitting");
}

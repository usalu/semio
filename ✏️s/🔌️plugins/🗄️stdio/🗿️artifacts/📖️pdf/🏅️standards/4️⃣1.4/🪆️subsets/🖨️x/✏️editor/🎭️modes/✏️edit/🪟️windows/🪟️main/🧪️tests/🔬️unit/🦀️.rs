use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::demo_pdf17_snapshot;

#[test]
fn definition_declares_the_page_canvas() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.actions.iter().any(|action| action.id == "insert-text"));
    assert!(def.actions.iter().any(|action| action.id == "insert-rectangle"));
    assert!(def.actions.iter().any(|action| action.id == "insert-image"));
}

fn first_surface(node: &semio_framework_ui_contract::BuiltNode) -> Option<&semio_framework_ui_contract::BuiltNode> {
    matches!(node.component, semio_framework_ui_contract::Component::Surface(_)).then_some(node).or_else(|| node.children.iter().find_map(first_surface))
}

#[test]
fn render_paints_the_page_text_on_the_canvas() {
    let document = demo_pdf17_snapshot();
    let node = render(&document).expect("page canvas");
    let surface = first_surface(&node).expect("canvas surface");
    let semio_framework_ui_contract::Component::Surface(props) = &surface.component else { unreachable!() };
    let scene: semio_framework_plugin::Canvas2dScene = semio_framework_plugin::decode_surface_doc(props).expect("canvas scene");
    assert!(scene.layers_json.contains("Semio"), "{}", scene.layers_json);
}

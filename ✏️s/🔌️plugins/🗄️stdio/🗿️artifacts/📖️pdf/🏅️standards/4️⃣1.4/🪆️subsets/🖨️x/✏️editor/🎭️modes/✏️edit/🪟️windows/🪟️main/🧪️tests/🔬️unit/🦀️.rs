use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::demo_pdf17_snapshot;

#[test]
fn definition_declares_a_document_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

fn first_surface(node: &BuiltNode) -> Option<&BuiltNode> {
    matches!(node.component, semio_framework_ui_contract::Component::Surface(_)).then_some(node).or_else(|| node.children.iter().find_map(first_surface))
}

#[test]
fn render_prefills_one_explicit_text_draft_per_page() {
    let document = demo_pdf17_snapshot();
    assert_eq!(document.pages.len(), 1);
    let node = render(&document).expect("bounded PDF document fixture must render");
    assert_eq!(node.children.len(), 1);
    let surface = first_surface(&node).expect("prefilled page draft");
    let semio_framework_ui_contract::Component::Surface(props) = &surface.component else { unreachable!() };
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(props).expect("text scene");
    assert_eq!(scene.buffer, document.pages[0].text());
    let settings: serde_json::Value = serde_json::from_str(scene.settings_json.as_deref().expect("draft settings")).expect("settings json");
    assert_eq!(settings["editAction"], "set-page");
    assert_eq!(settings["editArgument"], "text");
    assert_eq!(settings["commit"], "explicit");
    assert_eq!(settings["editArguments"]["page"], 0);
    assert_eq!(settings["editArguments"]["item"], 0);
}

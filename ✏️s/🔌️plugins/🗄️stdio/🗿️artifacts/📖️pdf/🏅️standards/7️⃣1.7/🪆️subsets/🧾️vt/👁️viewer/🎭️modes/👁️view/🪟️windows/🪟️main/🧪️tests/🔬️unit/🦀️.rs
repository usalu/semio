use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::demo_pdf17_snapshot;

#[test]
fn definition_declares_a_document_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[test]
fn render_lists_one_line_per_page_with_media_box_and_text() {
    let document = demo_pdf17_snapshot();
    assert_eq!(document.pages.len(), 1);
    let node = render(&document).expect("bounded PDF document fixture must render");
    assert_eq!(node.children.len(), 1);
    let semio_framework_ui_contract::Component::Text(text_node) = &node.children[0].component else { panic!("expected Text") };
    assert!(text_node.value.0.contains("MediaBox"));
}

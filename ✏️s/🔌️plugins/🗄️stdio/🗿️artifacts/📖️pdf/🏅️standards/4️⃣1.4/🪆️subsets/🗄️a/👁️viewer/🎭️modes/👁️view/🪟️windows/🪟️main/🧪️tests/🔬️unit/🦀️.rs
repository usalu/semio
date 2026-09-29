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
    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the rendered page window");
    let page: semio_framework_ui_scene::TextEditorScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&projection).expect("the page renders as a read-only text scene with its lanes");
    assert!(page.buffer.contains("MediaBox"));
}

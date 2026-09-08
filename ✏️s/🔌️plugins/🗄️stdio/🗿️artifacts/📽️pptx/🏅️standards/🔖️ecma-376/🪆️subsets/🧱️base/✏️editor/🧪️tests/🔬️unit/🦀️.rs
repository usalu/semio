
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_pptx_editor_builds_a_definition_for_the_editor_role() {
    let def = create_pptx_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, PPTX_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<PptxEditor as ArtifactEditor>::DIALECT, PPTX_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_document_window() {
    let def = create_pptx_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn set_page_writes_the_first_text_bearing_shape_only() {
    let mut snapshot = PptxSnapshot::default();
    snapshot.presentation.slides.push(PptxSlide { shapes: vec![PptxShape::Picture { blip_rel_id: "rId1".into(), position: Default::default() }, PptxShape::TextBox { text_frame: vec![PptxParagraph::text("old")], position: Default::default() }] });
    let mutation = build_set_page_mutation(&snapshot, 0, "new line one\nnew line two").expect("mutation");
    let PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index, shape_index, text_frame }) = &mutation else { panic!("expected SetShapeText") };
    assert_eq!(*slide_index, 0);
    assert_eq!(*shape_index, 1);
    assert_eq!(text_frame.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn set_page_on_a_slide_with_no_text_shape_is_a_documented_no_op() {
    let mut snapshot = PptxSnapshot::default();
    snapshot.presentation.slides.push(PptxSlide { shapes: vec![PptxShape::Picture { blip_rel_id: "rId1".into(), position: Default::default() }] });
    assert!(build_set_page_mutation(&snapshot, 0, "text").is_none());
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = PptxEditorCommand::SetPage { index: 3, text: "a\nmulti line value".into() };
    let printed = <PptxEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <PptxEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}

use super::*;
use crate::schema::snapshot::PptxSlide;
use semio_framework_plugin::app::DocumentWindowKit;

#[semio_framework_async_macros::async_test]
async fn create_pptx_strict_editor_builds_a_definition_for_the_editor_role() {
    let def = create_pptx_strict_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, PPTX_STRICT_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<PptxStrictEditor as ArtifactEditor>::DIALECT, PPTX_STRICT_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_document_window() {
    let def = create_pptx_strict_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn set_page_writes_the_explicit_text_shape_only() {
    let mut snapshot = PptxSnapshot::default();
    snapshot.presentation.slides.push(PptxSlide { shapes: vec![PptxShape::Picture { blip_rel_id: "rId1".into(), position: Default::default() }, PptxShape::TextBox { text_frame: vec![PptxParagraph::text("old")], position: Default::default() }] });
    let mutation = build_set_page_mutation(&snapshot, 0, 1, &DocumentWindowKit::text_revision("old"), "new line one\nnew line two").expect("valid edit").expect("changed edit");
    let PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index, shape_index, text_frame }) = &mutation else { panic!("expected SetShapeText") };
    assert_eq!(*slide_index, 0);
    assert_eq!(*shape_index, 1);
    assert_eq!(text_frame.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn set_page_rejects_a_non_text_shape() {
    let mut snapshot = PptxSnapshot::default();
    snapshot.presentation.slides.push(PptxSlide { shapes: vec![PptxShape::Picture { blip_rel_id: "rId1".into(), position: Default::default() }] });
    assert!(build_set_page_mutation(&snapshot, 0, 0, "", "text").is_err());
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = PptxStrictEditorCommand::SetPage { page: 3, item: 4, revision: "0123456789abcdef".into(), text: "a\nmulti line value".into() };
    let printed = <PptxStrictEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <PptxStrictEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<PptxStrictEditor as ArtifactEditor>::command_from_action("set-page", None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn stale_set_page_revision_is_rejected() {
    let mut snapshot = PptxSnapshot::default();
    snapshot.presentation.slides.push(PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("current")], position: Default::default() }] });
    assert!(build_set_page_mutation(&snapshot, 0, 0, "0000000000000000", "draft").is_err());
}

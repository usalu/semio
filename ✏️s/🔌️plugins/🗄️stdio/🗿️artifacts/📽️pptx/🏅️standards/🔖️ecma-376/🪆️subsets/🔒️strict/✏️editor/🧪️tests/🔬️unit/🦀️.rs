use super::*;
use crate::schema::snapshot::{PptxParagraph, PptxPresentation, PptxShape, PptxSlide};
use semio_framework_plugin::app::DocumentWindowKit;

fn deck(shapes: Vec<PptxShape>) -> PptxSnapshot {
    crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_pptx(PptxPresentation { slides: vec![PptxSlide { shapes }] })
}

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
    assert!(create_pptx_strict_editor().window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn set_page_emits_a_revision_addressed_text_edit() {
    let snapshot = deck(vec![PptxShape::Picture { blip_rel_id: "rId1".into(), position: Default::default() }, PptxShape::TextBox { text_frame: vec![PptxParagraph::text("old")], position: Default::default() }]);
    let mutation = build_set_page_mutation(&snapshot, 0, 1, &DocumentWindowKit::text_revision("old"), "new").expect("valid edit").expect("changed edit");
    let PptxMutation::SetShapeText(set_shape_text::SetShapeText { address, text }) = mutation else { panic!("expected SetShapeText") };
    assert!(!address.shape_id.is_empty());
    assert_eq!(text, "new");
}

#[semio_framework_async_macros::async_test]
async fn set_page_rejects_non_text_and_stale_targets() {
    let picture = deck(vec![PptxShape::Picture { blip_rel_id: "rId1".into(), position: Default::default() }]);
    assert!(build_set_page_mutation(&picture, 0, 0, "", "text").is_err());
    let text = deck(vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("current")], position: Default::default() }]);
    assert!(build_set_page_mutation(&text, 0, 0, "0000000000000000", "draft").is_err());
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = PptxStrictEditorCommand::SetPage { page: 3, item: 4, revision: "0123456789abcdef".into(), text: "a\nmulti line value".into() };
    assert_eq!(<PptxStrictEditorCommand as protocol::OpText>::parse_op(&<PptxStrictEditorCommand as protocol::OpText>::print_op(&command)).expect("parse ok"), command);
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<PptxStrictEditor as ArtifactEditor>::command_from_action("set-page", None).is_err());
}

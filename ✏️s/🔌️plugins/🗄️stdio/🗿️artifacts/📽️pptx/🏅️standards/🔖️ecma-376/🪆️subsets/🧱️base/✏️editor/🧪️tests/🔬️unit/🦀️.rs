use super::*;
use crate::schema::snapshot::PptxSlide;
use semio_framework_plugin::app::DocumentWindowKit;

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
    let command = PptxEditorCommand::SetPage { page: 3, item: 4, revision: "0123456789abcdef".into(), text: "a\nmulti line value".into() };
    let printed = <PptxEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <PptxEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<PptxEditor as ArtifactEditor>::command_from_action("set-page", None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn stale_set_page_revision_is_rejected() {
    let mut snapshot = PptxSnapshot::default();
    snapshot.presentation.slides.push(PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("current")], position: Default::default() }] });
    assert!(build_set_page_mutation(&snapshot, 0, 0, "0000000000000000", "draft").is_err());
}

#[semio_framework_async_macros::async_test]
async fn registered_shape_draft_publishes_once_refuses_stale_and_undoes_redoes() {
    use crate::schema::snapshot::PptxSlide;
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let mut authored = PptxSnapshot::default();
    authored.presentation.slides.push(PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("before")], position: Default::default() }] });
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<PptxEditor>, _>(async { semio_framework_plugin::App { definition: create_pptx_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&authored, STDIO_PPTX_DOCUMENT_SCHEMA) else { panic!("PPTX fixture produces a document load") };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let original = app.snapshot().unwrap().clone();
    assert_eq!(original.presentation, authored.presentation, "the host opens the authored presentation as its canonical package");
    let meta = artifact_app_laws::meta("local");
    let arguments = |revision: String, text: &str| {
        dsl::DslValue::object([("page".into(), dsl::DslValue::float(0.0)), ("item".into(), dsl::DslValue::float(0.0)), ("revision".into(), dsl::DslValue::String(revision)), ("text".into(), dsl::DslValue::String(text.into()))])
    };

    let edit = arguments(DocumentWindowKit::text_revision("before"), "after");
    app.handle_action("set-page", Some(&edit), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    let expected = app.snapshot().unwrap().clone();
    assert_eq!(shape_text(&expected.presentation.slides[0].shapes[0]).as_deref(), Some("after"));

    let no_op = arguments(DocumentWindowKit::text_revision("after"), "after");
    app.handle_action("set-page", Some(&no_op), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), original, "an identical Apply must not add a history entry");
    artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), expected);

    let stale = arguments(DocumentWindowKit::text_revision("before"), "refused draft");
    app.handle_action("set-page", Some(&stale), &meta).await.unwrap();
    assert!(artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.is_err());
    assert_eq!(app.snapshot().unwrap(), expected, "a refused draft must preserve the persisted presentation");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

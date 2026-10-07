use super::*;
use crate::schema::snapshot::{PptxParagraph, PptxPresentation, PptxShape, PptxSlide};
use semio_framework_plugin::app::DocumentWindowKit;

fn deck(shapes: Vec<PptxShape>) -> PptxSnapshot {
    crate::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx(PptxPresentation { slides: vec![PptxSlide { shapes }] })
}

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
    assert!(create_pptx_editor().window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn set_page_addresses_only_the_explicit_text_shape() {
    let snapshot = deck(vec![PptxShape::Picture { blip_rel_id: "rId1".into(), position: Default::default() }, PptxShape::TextBox { text_frame: vec![PptxParagraph::text("old")], position: Default::default() }]);
    let mutation = build_set_page_mutation(&snapshot, 0, 1, &DocumentWindowKit::text_revision("old"), "new line one\nnew line two").expect("valid edit").expect("changed edit");
    let PptxMutation::SetShapeText(set_shape_text::SetShapeText { address, text }) = mutation else { panic!("expected SetShapeText") };
    assert!(!address.shape_id.is_empty());
    assert_eq!(text, "new line one\nnew line two");
}

#[semio_framework_async_macros::async_test]
async fn set_page_rejects_a_non_text_shape() {
    let snapshot = deck(vec![PptxShape::Picture { blip_rel_id: "rId1".into(), position: Default::default() }]);
    assert!(build_set_page_mutation(&snapshot, 0, 0, "", "text").is_err());
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = PptxEditorCommand::SetPage { page: 3, item: 4, revision: "0123456789abcdef".into(), text: "a\nmulti line value".into() };
    assert_eq!(<PptxEditorCommand as protocol::OpText>::parse_op(&<PptxEditorCommand as protocol::OpText>::print_op(&command)).expect("parse ok"), command);
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<PptxEditor as ArtifactEditor>::command_from_action("set-page", None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn stale_set_page_revision_is_rejected() {
    let snapshot = deck(vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("current")], position: Default::default() }]);
    assert!(build_set_page_mutation(&snapshot, 0, 0, "0000000000000000", "draft").is_err());
}

#[semio_framework_async_macros::async_test]
async fn natural_file_route_exports_edited_pptx_xml_and_reopens_through_one_mutation() {
    use quick_xml::{events::Event, Reader};
    use std::io::{Cursor, Read};

    let edited = deck(vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("Natural Open Save")], position: Default::default() }]);
    let bytes = <PptxEditor as ArtifactEditor>::encode_natural_file(&edited).expect("PPTX natural bytes");
    let mut archive = zip::ZipArchive::new(Cursor::new(&bytes)).expect("independent ZIP reader accepts PPTX");
    assert!(archive.by_name("[Content_Types].xml").is_ok());
    let mut slide_xml = String::new();
    archive.by_name("ppt/slides/slide1.xml").expect("PPTX slide part").read_to_string(&mut slide_xml).expect("PPTX slide XML text");
    let mut reader = Reader::from_str(&slide_xml);
    let mut text = Vec::new();
    loop {
        match reader.read_event().expect("independent XML parser accepts PPTX slide") {
            Event::Text(value) => text.push(value.xml10_content().into_owned()),
            Event::Eof => break,
            _ => {}
        }
    }
    assert!(text.iter().any(|value| value == "Natural Open Save"));
    let reopened = <PptxEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("PPTX natural bytes reopen");
    let Some(PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: opened })) = <PptxEditor as ArtifactEditor>::whole_document_operation(reopened) else {
        panic!("natural PPTX opens through one event-sourced snapshot mutation")
    };
    assert_eq!(crate::schema::mutations::xml_address::pptx_slides(&opened).unwrap()[0].shapes[0].text.as_deref(), Some("Natural Open Save"));
}

#[semio_framework_async_macros::async_test]
async fn registered_shape_draft_publishes_once_refuses_stale_and_undoes_redoes() {
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let authored = deck(vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("before")], position: Default::default() }]);
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<PptxEditor>, _>(async { semio_framework_plugin::App { definition: create_pptx_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&authored, STDIO_PPTX_DOCUMENT_SCHEMA) else { panic!("PPTX fixture produces a document load") };
    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let original = app.snapshot().unwrap().clone();
    assert_eq!(original, authored, "the host opens the authored canonical package");
    let meta = artifact_app_laws::meta("local");
    let arguments = |revision: String, text: &str| {
        semio_framework_value::DslValue::object([("page".into(), semio_framework_value::DslValue::float(0.0)), ("item".into(), semio_framework_value::DslValue::float(0.0)), ("revision".into(), semio_framework_value::DslValue::String(revision)), ("text".into(), semio_framework_value::DslValue::String(text.into()))])
    };

    app.handle_action("set-page", Some(&arguments(DocumentWindowKit::text_revision("before"), "after")), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    let expected = app.snapshot().unwrap().clone();
    assert_eq!(crate::schema::mutations::xml_address::pptx_slides(&expected).unwrap()[0].shapes[0].text.as_deref(), Some("after"));

    app.handle_action("set-page", Some(&arguments(DocumentWindowKit::text_revision("after"), "after")), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), original, "an identical Apply must not add a history entry");
    artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), expected);

    app.handle_action("set-page", Some(&arguments(DocumentWindowKit::text_revision("before"), "refused draft")), &meta).await.unwrap();
    assert!(artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.is_err());
    assert_eq!(app.snapshot().unwrap(), expected);
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", PptxEditor, || semio_framework_plugin::App { definition: create_pptx_editor(), examples: Vec::new() }, "../..");

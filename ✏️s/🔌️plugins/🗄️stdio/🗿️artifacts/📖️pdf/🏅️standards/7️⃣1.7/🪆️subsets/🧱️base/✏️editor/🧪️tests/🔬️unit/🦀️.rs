use super::*;

#[semio_framework_async_macros::async_test]
async fn create_pdf17_editor_builds_a_definition_for_the_editor_role() {
    let def = create_pdf17_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, PDF17_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Pdf17Editor as ArtifactEditor>::DIALECT, PDF17_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_pdf17_editor();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<Pdf17Editor as ArtifactEditor>::command_from_action("set-page", None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn explicit_nonzero_page_payload_is_preserved() {
    let args = dsl::DslValue::Object(vec![
        ("page".into(), dsl::DslValue::float(3.0)),
        ("item".into(), dsl::DslValue::float(0.0)),
        ("revision".into(), dsl::DslValue::String("0123456789abcdef".into())),
        ("text".into(), dsl::DslValue::String("replacement".into())),
    ]);
    let command = <Pdf17Editor as ArtifactEditor>::command_from_action("set-page", Some(&args)).expect("typed payload");
    assert!(matches!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf17EditorCommand::SetPage { page: 3, item: 0, revision, text }) if revision == "0123456789abcdef" && text == "replacement"));
}

#[semio_framework_async_macros::async_test]
async fn registered_pdf_draft_publishes_once_refuses_stale_and_undoes_redoes() {
    use crate::schema::snapshot::{PdfOp, PdfPage, PdfTextString};
    use semio_framework_plugin::app::DocumentWindowKit;
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let mut original = PdfSnapshot::default();
    original.pages.push(PdfPage { content: vec![PdfOp::BeginText, PdfOp::ShowText { text: PdfTextString::text("before") }, PdfOp::EndText], ..Default::default() });
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(async { semio_framework_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_PDF_DOCUMENT_SCHEMA) else { panic!("PDF fixture produces a document load") };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let loaded = app.snapshot().unwrap().clone();
    assert_eq!(loaded.pages[0].text(), "before");
    let meta = artifact_app_laws::meta("local");
    let arguments = |revision: String, text: &str| {
        dsl::DslValue::object([("page".into(), dsl::DslValue::float(0.0)), ("item".into(), dsl::DslValue::float(0.0)), ("revision".into(), dsl::DslValue::String(revision)), ("text".into(), dsl::DslValue::String(text.into()))])
    };

    let edit = arguments(DocumentWindowKit::text_revision("before"), "after");
    app.handle_action("set-page", Some(&edit), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    let expected = app.snapshot().unwrap().clone();
    assert_eq!(expected.pages[0].text(), "after");

    let no_op = arguments(DocumentWindowKit::text_revision("after"), "after");
    app.handle_action("set-page", Some(&no_op), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), loaded, "an identical Apply must not add a history entry");
    artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), expected);

    let stale = arguments(DocumentWindowKit::text_revision("before"), "refused draft");
    app.handle_action("set-page", Some(&stale), &meta).await.unwrap();
    assert!(artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.is_err());
    assert_eq!(app.snapshot().unwrap(), expected, "a refused draft must preserve every PDF page operator");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn registered_pdf_page_edit_publishes_and_undoes_redoes() {
    use crate::schema::snapshot::PdfOp;
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let mut original = crate::standards::v1_7::subsets::base::schema::snapshot::demo_pdf17_snapshot();
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(async { semio_framework_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_PDF_DOCUMENT_SCHEMA) else { panic!("PDF fixture produces a document load") };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let loaded = app.snapshot().unwrap().clone();
    let meta = artifact_app_laws::meta("local");
    let insert = dsl::DslValue::object([
        ("page".into(), dsl::DslValue::float(0.0)),
        ("x".into(), dsl::DslValue::float(12.0)),
        ("y".into(), dsl::DslValue::float(24.0)),
        ("width".into(), dsl::DslValue::float(30.0)),
        ("height".into(), dsl::DslValue::float(16.0)),
        ("red".into(), dsl::DslValue::float(0.2)),
        ("green".into(), dsl::DslValue::float(0.4)),
        ("blue".into(), dsl::DslValue::float(0.6)),
    ]);
    app.handle_action("insert-rectangle", Some(&insert), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    let edited = app.snapshot().unwrap().clone();
    assert!(edited.pages[0].content.iter().any(|op| matches!(op, PdfOp::Rectangle { x, y, width, height } if (*x - 12.0).abs() < 0.01 && (*y - 24.0).abs() < 0.01 && (*width - 30.0).abs() < 0.01 && (*height - 16.0).abs() < 0.01)));
    assert!(edited.pages[0].text().contains("Semio"));
    artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), loaded);
    artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), edited);
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

use super::*;
use semio_framework_plugin::app::DocumentWindowKit;

#[semio_framework_async_macros::async_test]
async fn create_docx_editor_builds_a_definition_for_the_editor_role() {
    let def = create_docx_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DOCX_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DocxEditor as ArtifactEditor>::DIALECT, DOCX_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_document_window() {
    let def = create_docx_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn set_page_replaces_paragraph_text_through_an_addressed_revision() {
    let mut snapshot = DocxSnapshot::default();
    snapshot.document.body.push(DocxBlock::paragraph("hello"));
    let mutation = build_set_page_mutation(&snapshot, 0, 0, &DocumentWindowKit::text_revision("hello"), "goodbye").expect("valid edit").expect("changed edit");
    let DocxMutation::SetRunText(set_run_text::SetRunText { path, run_index, text }) = &mutation else { panic!("expected SetRunText") };
    assert_eq!(path.index, 0);
    assert_eq!(*run_index, 0);
    assert_eq!(text, "goodbye");
}

#[semio_framework_async_macros::async_test]
async fn set_page_rejects_a_non_text_target() {
    let mut snapshot = DocxSnapshot::default();
    snapshot.document.body.push(DocxBlock::Table(crate::schema::snapshot::DocxTable::default()));
    assert!(build_set_page_mutation(&snapshot, 0, 0, "", "text").is_err());
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = DocxEditorCommand::SetPage { page: 2, item: 0, revision: "0123456789abcdef".into(), text: "a\nmulti line value".into() };
    let printed = <DocxEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <DocxEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<DocxEditor as ArtifactEditor>::command_from_action("set-page", None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn stale_set_page_revision_is_rejected() {
    let mut snapshot = DocxSnapshot::default();
    snapshot.document.body.push(DocxBlock::paragraph("current"));
    assert!(build_set_page_mutation(&snapshot, 0, 0, "0000000000000000", "draft").is_err());
}

#[test]
fn retained_page_edit_pages_large_unicode_text_and_cancels_without_publication() {
    use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};

    let current = "ä".repeat(40_000);
    let replacement = "🧾".repeat(20_000);
    let mut snapshot = DocxSnapshot::default();
    snapshot.document.body.push(DocxBlock::paragraph(current.clone()));
    let command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage {
        page: 0,
        item: 0,
        revision: DocumentWindowKit::text_revision(&current),
        text: replacement.clone(),
    });
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = semio_framework_plugin::AppOperationContext {
        app_instance_id: 1,
        parent_document_id: "docx-retained-text".into(),
        operation_id: 2,
        generation: 3,
        canonical_base_revision: [4; 32],
    };
    let input = ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };

    let mut work = DocxSetPageWork::default();
    let mut replay_steps = 0usize;
    let mutation = loop {
        match work.step(&input).expect("retained DOCX text step") {
            ArtifactCommandWorkStep::Replay { preview, .. } => {
                replay_steps += 1;
                assert!(std::str::from_utf8(preview).expect("localized preview").contains("de"));
            }
            ArtifactCommandWorkStep::Complete(emit) => break emit.artifact_mutations.into_iter().next().expect("text mutation"),
            ArtifactCommandWorkStep::Progress { .. } | ArtifactCommandWorkStep::CompleteWithEphemeral { .. } => panic!("unexpected retained DOCX step"),
        }
    };
    assert!(replay_steps > 16, "large text must cross multiple retained grants");
    let DocxMutation::SetRunText(set_run_text::SetRunText { text, .. }) = mutation else { panic!("expected run text mutation") };
    assert_eq!(text, replacement);

    let mut cancelled = DocxSetPageWork::default();
    for _ in 0..24 {
        assert!(matches!(cancelled.step(&input).expect("retained pre-cancel step"), ArtifactCommandWorkStep::Replay { .. }));
    }
    cancelled.begin_close();
    while !cancelled.terminal_is_empty() {
        assert!(!matches!(cancelled.close_step(1, DOCX_TEXT_WORK_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Blocked));
    }
    assert_eq!(addressed_run_text(&snapshot, 0, 0).expect("unchanged run").1, current);
}

#[semio_framework_async_macros::async_test]
async fn registered_page_draft_publishes_once_refuses_stale_and_undoes_redoes() {
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let mut original = DocxSnapshot::default();
    original.document.body.push(DocxBlock::paragraph("before"));
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<DocxEditor>, _>(async { semio_framework_plugin::App { definition: create_docx_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_DOCX_DOCUMENT_SCHEMA) else { panic!("DOCX fixture produces a document load") };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let meta = artifact_app_laws::meta("local");
    let arguments = |revision: String, text: &str| {
        dsl::DslValue::object([("page".into(), dsl::DslValue::float(0.0)), ("item".into(), dsl::DslValue::float(0.0)), ("revision".into(), dsl::DslValue::String(revision)), ("text".into(), dsl::DslValue::String(text.into()))])
    };

    let edit = arguments(DocumentWindowKit::text_revision("before"), "after");
    app.handle_action("set-page", Some(&edit), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    let mut expected = original.clone();
    expected.document.body[0] = DocxBlock::paragraph("after");
    assert_eq!(app.snapshot().unwrap(), expected);

    let no_op = arguments(DocumentWindowKit::text_revision("after"), "after");
    app.handle_action("set-page", Some(&no_op), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    assert_eq!(app.snapshot().unwrap(), expected);
    artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), original, "an identical Apply must not add a history entry");
    artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), expected);

    let stale = arguments(DocumentWindowKit::text_revision("before"), "refused draft");
    app.handle_action("set-page", Some(&stale), &meta).await.unwrap();
    assert!(artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.is_err());
    assert_eq!(app.snapshot().unwrap(), expected, "a refused draft must preserve the persisted document");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

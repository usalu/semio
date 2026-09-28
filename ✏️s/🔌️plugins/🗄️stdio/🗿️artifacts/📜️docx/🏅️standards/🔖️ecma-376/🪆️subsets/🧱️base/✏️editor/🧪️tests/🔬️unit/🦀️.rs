use super::*;
use crate::schema::{
    mutations::{docx_top_level_run_at, insert_table_row, insert_xml_node, remove_table_row, remove_xml_node, replace_xml_node, set_paragraph_style, set_run_formatting},
    snapshot::{DocxBlock, DocxDocument},
};
use crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_docx;

fn snapshot_with_blocks(body: Vec<DocxBlock>) -> DocxSnapshot {
    build_minimal_docx(DocxDocument { body, styles: Vec::new() })
}

fn run(snapshot: &DocxSnapshot) -> crate::schema::mutations::DocxEditableRun {
    docx_top_level_run_at(snapshot, 0, 0).expect("canonical first run")
}

fn arguments(address: &DocxXmlAddress, text: &str) -> dsl::DslValue {
    dsl::DslValue::object([("address".into(), dsl::ToValue::to_value(address)), ("text".into(), dsl::DslValue::String(text.into()))])
}

#[semio_framework_async_macros::async_test]
async fn create_docx_editor_builds_a_definition_for_the_editor_role() {
    let def = create_docx_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DOCX_EDITOR_DIALECT.into());
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[test]
fn set_page_replaces_one_revision_bound_canonical_run() {
    let snapshot = snapshot_with_blocks(vec![DocxBlock::paragraph("hello")]);
    let target = run(&snapshot);
    let mutation = build_set_page_mutation(&snapshot, &target.address, "goodbye").expect("valid edit").expect("changed edit");
    let DocxMutation::SetRunText(set_run_text::SetRunText { address, text }) = mutation else { panic!("expected SetRunText") };
    assert_eq!(address, target.address);
    assert_eq!(text, "goodbye");
}

#[test]
fn parser_requires_the_complete_nested_address_and_empty_text_remains_valid() {
    let snapshot = snapshot_with_blocks(vec![DocxBlock::paragraph("hello")]);
    let target = run(&snapshot);
    let command = <DocxEditor as ArtifactEditor>::command_from_action("set-page", Some(&arguments(&target.address, ""))).expect("canonical empty text");
    assert_eq!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage { address: target.address, text: String::new() }));
    assert!(<DocxEditor as ArtifactEditor>::command_from_action("set-page", None).is_err());
}

#[test]
fn stale_address_and_identical_text_preserve_the_snapshot() {
    let snapshot = snapshot_with_blocks(vec![DocxBlock::paragraph("current")]);
    let target = run(&snapshot);
    let mut stale = target.address.clone();
    stale.revision = "stale".into();
    assert!(build_set_page_mutation(&snapshot, &stale, "draft").is_err());
    assert!(build_set_page_mutation(&snapshot, &target.address, "current").expect("valid no-op").is_none());
    assert_eq!(run(&snapshot).text, "current");
}

#[test]
fn command_roundtrips_and_encoded_admission_counts_the_nested_address() {
    let snapshot = snapshot_with_blocks(vec![DocxBlock::paragraph("before")]);
    let command = DocxEditorCommand::SetPage { address: run(&snapshot).address, text: "a\nmulti line value".into() };
    let printed = <DocxEditorCommand as protocol::OpText>::print_op(&command);
    assert_eq!(<DocxEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse"), command);
    let command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(command);
    let encoded = protocol::OpBinary::encode_op(&command).expect("encode set-page");
    assert_eq!(semio_s_artifact_stdio_contract::editing::admit_bounded_native_command(&command, encoded.len()).expect("exact admission"), encoded.len());
    assert!(semio_s_artifact_stdio_contract::editing::admit_bounded_native_command(&command, encoded.len() - 1).is_err());
}

#[test]
fn retained_work_refuses_an_unpaged_large_owner_without_publication() {
    use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork};

    let snapshot = snapshot_with_blocks(vec![DocxBlock::paragraph("before")]);
    let target = run(&snapshot);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📬️preparation/🧫️fixtures/🧵️admission/🔣️.json")).expect("language-neutral admission fixture");
    let command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage { address: target.address, text: "x".repeat(fixture["refusedTextBytes"].as_u64().expect("refused text bytes") as usize) });
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "docx-retained-text".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "authoring-seed-test".into() };
    let input = ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };
    let mut work = DocxSetPageWork::default();
    assert!(work.step(&input).is_err());
    assert_eq!(run(&snapshot).text, "before");
}

#[test]
fn canonical_preparation_route_is_mounted() {
    use semio_s_artifact_stdio_contract::editing::BoundedNativeEditingEditor;
    assert!(DocxEditor::native_edit_preparation_route("stdio-docx-base-snapshot-edit").is_some());
}

#[test]
fn canonical_preparation_recognizes_and_encodes_every_addressed_xml_mutation() {
    use semio_framework_plugin::plugin_app_close_prelude::store::ArtifactCanonicalJson;
    use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📬️preparation/🧫️fixtures/🧵️admission/🔣️.json")).expect("language-neutral admission fixture");
    let address = run(&snapshot_with_blocks(vec![DocxBlock::paragraph("before")])).address;
    let node = XmlNode::Text { text: "payload β".into() };
    let mutations = vec![
        ("setRunText", DocxMutation::SetRunText(set_run_text::SetRunText { address: address.clone(), text: "after".into() })),
        ("replaceXmlNode", DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address: address.clone(), node: node.clone() })),
        ("setRunFormatting", DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address: address.clone(), bold: true, italic: false, underline: true })),
        ("setParagraphStyle", DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address: address.clone(), style_id: Some("Normal".into()) })),
        ("insertTableRow", DocxMutation::InsertTableRow(insert_table_row::InsertTableRow { address: address.clone(), index: 1, cells: vec!["A".into(), "β".into()] })),
        ("removeTableRow", DocxMutation::RemoveTableRow(remove_table_row::RemoveTableRow { address: address.clone(), index: 1 })),
        ("insertXmlNode", DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent: address.clone(), index: 0, node })),
        ("removeXmlNode", DocxMutation::RemoveXmlNode(remove_xml_node::RemoveXmlNode { parent: address, index: 0, expected_name: "{urn:test}node".into(), revision: "revision".into() })),
    ];
    let expected = fixture["recognizedMutations"].as_array().expect("recognized mutation names");
    assert_eq!(mutations.len(), expected.len());
    for ((name, mutation), expected) in mutations.iter().zip(expected) {
        assert_eq!(*name, expected.as_str().expect("recognized mutation name"));
        assert!(preparation::recognizes(mutation));
        preparation::measure_mutation(mutation).expect("addressed mutation fits neutral envelope");
        assert!(ArtifactCanonicalJson::canonical_json_borrowed_root(mutation).expect("borrowed canonical mutation").is_some());
    }
}

#[semio_framework_async_macros::async_test]
async fn registered_canonical_page_edit_publishes_once_and_undoes_redoes() {
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let original = snapshot_with_blocks(vec![DocxBlock::paragraph("before")]);
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<DocxEditor>, _>(async { semio_framework_plugin::App { definition: create_docx_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_DOCX_DOCUMENT_SCHEMA) else { panic!("DOCX fixture produces a document load") };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let opened = app.snapshot().expect("opened DOCX fixture").clone();
    let original_address = run(app.snapshot().unwrap()).address;
    let meta = artifact_app_laws::meta("local");

    app.handle_action("set-page", Some(&arguments(&original_address, "after")), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    assert_eq!(run(app.snapshot().unwrap()).text, "after");

    let current_address = run(app.snapshot().unwrap()).address;
    app.handle_action("set-page", Some(&arguments(&current_address, "after")), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), &opened);
    artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id).await;
    assert_eq!(run(app.snapshot().unwrap()).text, "after");

    app.handle_action("set-page", Some(&arguments(&original_address, "refused")), &meta).await.unwrap();
    assert!(artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.is_err());
    assert_eq!(run(app.snapshot().unwrap()).text, "after");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

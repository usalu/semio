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

fn arguments(address: &DocxXmlAddress, text: &str) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::object([("address".into(), semio_framework_value::ToValue::to_value(address)), ("text".into(), semio_framework_value::DslValue::String(text.into()))])
}

fn formatting_arguments(address: &DocxXmlAddress, bold: bool, italic: bool, underline: bool) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::object([("address".into(), semio_framework_value::ToValue::to_value(address)), ("bold".into(), semio_framework_value::DslValue::Bool(bold)), ("italic".into(), semio_framework_value::DslValue::Bool(italic)), ("underline".into(), semio_framework_value::DslValue::Bool(underline))])
}

#[semio_framework_async_macros::async_test]
async fn create_docx_editor_builds_a_definition_for_the_editor_role() {
    let def = create_docx_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DOCX_EDITOR_DIALECT.into());
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[test]
fn initial_document_matches_the_neutral_empty_paragraph_snapshot_and_renders_one_target() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌱️initial-document/🔣️.json")).unwrap();
    let snapshot = <DocxEditor as ArtifactEditor>::initial_snapshot();
    let targets = crate::schema::mutations::docx_top_level_text_targets(&snapshot, 0).unwrap();
    assert_eq!(targets.len(), fixture["body"].as_array().unwrap().len());
    assert_eq!(targets[0].text, fixture["editableTarget"]["text"].as_str().unwrap());
    assert_eq!(targets[0].kind, crate::schema::mutations::DocxTextTargetKind::EmptyParagraph);
    assert_eq!(main::editable_pages(&snapshot).unwrap().len(), 1);
    assert_eq!(main::render(&snapshot, semio_framework_plugin::UiPublicationRevision(23)).unwrap().children.len(), 1);
}

#[test]
fn natural_file_route_exports_edited_docx_xml_and_reopens_it() {
    use quick_xml::events::Event;
    use std::io::Read;

    let edited = snapshot_with_blocks(vec![DocxBlock::paragraph("Natural Open Save")]);
    let bytes = <DocxEditor as ArtifactEditor>::encode_natural_file(&edited).expect("DOCX natural bytes");
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&bytes)).expect("zip crate accepts exported package");
    let mut document_xml = Vec::new();
    archive.by_name("word/document.xml").expect("DOCX document part").read_to_end(&mut document_xml).expect("document part reads");
    let mut reader = quick_xml::Reader::from_reader(document_xml.as_slice());
    let mut text = String::new();
    loop {
        match reader.read_event().expect("quick-xml accepts document part") {
            Event::Text(value) => text.push_str(&value.xml10_content()),
            Event::Eof => break,
            _ => {}
        }
    }
    assert!(text.contains("Natural Open Save"));
    let reopened = <DocxEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("DOCX natural bytes reopen");
    assert_eq!(run(&reopened).text, "Natural Open Save");
    let Some(DocxMutation::SetSnapshot(set)) = <DocxEditor as ArtifactEditor>::whole_document_operation(reopened) else { panic!("natural DOCX opens through one event-sourced snapshot mutation") };
    assert_eq!(run(&set.snapshot).text, "Natural Open Save");
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
fn formatting_parser_requires_all_schema_fields_and_preserves_the_canonical_address() {
    let snapshot = snapshot_with_blocks(vec![DocxBlock::paragraph("hello")]);
    let target = run(&snapshot);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️run-formatting-action/🔣️.json")).expect("language-neutral formatting fixture");
    let before = &fixture["transition"]["before"];
    let command = <DocxEditor as ArtifactEditor>::command_from_action(
        fixture["action"].as_str().unwrap(),
        Some(&formatting_arguments(&target.address, before["bold"].as_bool().unwrap(), before["italic"].as_bool().unwrap(), before["underline"].as_bool().unwrap())),
    )
    .expect("canonical formatting command");
    assert_eq!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetRunFormatting { address: target.address, bold: false, italic: true, underline: false }));
    assert!(<DocxEditor as ArtifactEditor>::command_from_action("set-run-formatting", None).is_err());
    let incomplete = semio_framework_value::DslValue::object([("address".into(), semio_framework_value::ToValue::to_value(&run(&snapshot).address)), ("bold".into(), semio_framework_value::DslValue::Bool(true))]);
    assert!(<DocxEditor as ArtifactEditor>::command_from_action("set-run-formatting", Some(&incomplete)).is_err());
}

#[test]
fn formatting_mutation_preserves_explicit_off_rejects_stale_addresses_and_elides_identical_direct_values() {
    let snapshot = snapshot_with_blocks(vec![DocxBlock::paragraph("hello")]);
    let target = run(&snapshot);
    let explicit_off = build_set_run_formatting_mutation(&snapshot, &target.address, false, false, false).expect("valid direct override").expect("missing properties are not equivalent to explicit off");
    let mut formatted = snapshot.clone();
    crate::schema::mutations::apply_docx_mutation(&mut formatted, &explicit_off);
    let address = run(&formatted).address;
    assert_eq!(crate::schema::mutations::docx_run_formatting(&formatted, &address).unwrap(), DocxRunFormatting { bold: Some(false), italic: Some(false), underline: Some(false) });
    assert!(build_set_run_formatting_mutation(&formatted, &address, false, false, false).expect("valid no-op").is_none());
    let mut stale = target.address;
    stale.revision = "0000000000000000".into();
    assert!(build_set_run_formatting_mutation(&snapshot, &stale, true, false, false).is_err());
}

#[test]
fn inherited_bold_and_explicit_off_remain_distinct_through_saved_quick_xml_oracle() {
    use crate::schema::snapshot::DocxStyle;
    use quick_xml::{events::Event, reader::Reader};
    use semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text;
    use std::io::Read;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️direct-run-formatting/🔣️.json")).unwrap();
    let mut snapshot = build_minimal_docx(DocxDocument { body: vec![DocxBlock::paragraph("placeholder")], styles: vec![DocxStyle { id: "InheritedBold".into(), name: "Inherited Bold".into(), based_on: None }] });
    snapshot.xml_part_mut("word/document.xml").unwrap().replace_document(xml_document_from_text(fixture["documentXml"].as_str().unwrap()).unwrap()).unwrap();
    snapshot.xml_part_mut("word/styles.xml").unwrap().replace_document(xml_document_from_text(fixture["stylesXml"].as_str().unwrap()).unwrap()).unwrap();
    let target = run(&snapshot);
    assert_eq!(docx_run_formatting(&snapshot, &target.address).unwrap(), DocxRunFormatting { bold: None, italic: None, underline: None });
    let command = &fixture["command"];
    let mutation =
        build_set_run_formatting_mutation(&snapshot, &target.address, command["bold"].as_bool().unwrap(), command["italic"].as_bool().unwrap(), command["underline"].as_bool().unwrap()).unwrap().expect("explicit off overrides inherited formatting");
    crate::schema::mutations::apply_docx_mutation(&mut snapshot, &mutation);
    let address = run(&snapshot).address;
    assert_eq!(docx_run_formatting(&snapshot, &address).unwrap(), DocxRunFormatting { bold: Some(false), italic: Some(false), underline: Some(false) });

    let bytes = crate::engine::encode_docx(&snapshot).unwrap();
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
    let mut read_part = |path: &str| {
        let mut xml = String::new();
        archive.by_name(path).unwrap().read_to_string(&mut xml).unwrap();
        xml
    };
    let document = read_part("word/document.xml");
    let styles = read_part("word/styles.xml");
    let bold_values = |xml: &str| {
        let mut reader = Reader::from_str(xml);
        let mut values = Vec::new();
        loop {
            match reader.read_event().unwrap() {
                Event::Start(event) | Event::Empty(event) if event.local_name().as_ref() == "b" => {
                    values.push(event.attributes().map(Result::unwrap).find(|attribute| attribute.key.local_name().as_ref() == "val").map(|attribute| attribute.normalized_value(quick_xml::XmlVersion::Explicit1_0).unwrap().into_owned()))
                }
                Event::Eof => break,
                _ => {}
            }
        }
        values
    };
    assert_eq!(bold_values(&styles), [None], "third-party XML parser sees inherited bold in the style");
    assert_eq!(bold_values(&document), [Some("0".into())], "third-party XML parser sees the explicit direct off override");
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📬️preparation/🧫️fixtures/🧵️admission/🔣️.json")).expect("language-neutral admission fixture");
    let command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage { address: target.address, text: "x".repeat(fixture["refusedTextBytes"].as_u64().expect("refused text bytes") as usize) });
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "docx-retained-text".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "authoring-seed-test".into() };
    let input = ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };
    let mut work = DocxSetPageWork::default();
    assert!(work
        .step(
            &input,
            &mut semio_framework_job::StepContext::new(
                semio_framework_job::allocate_operation_id(),
                semio_framework_job::Generation(1),
                semio_framework_job::StepBudget::new(256, u64::MAX),
                semio_framework_job::root_cancel_token(),
                || Some(0),
                &mut 0
            )
        )
        .is_err());
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

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📬️preparation/🧫️fixtures/🧵️admission/🔣️.json")).expect("language-neutral admission fixture");
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
async fn retained_opc_and_xml_store_route_cancels_publishes_saves_and_retires_large_owners() {
    use std::io::Read;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📬️preparation/🧫️fixtures/🧬️retained-opc-lifecycle/🔣️.json")).expect("language-neutral retained OPC lifecycle fixture");
    let part_path = fixture["partPath"].as_str().expect("retained OPC part path");
    let content_type = fixture["contentType"].as_str().expect("retained OPC content type");
    let part_bytes = fixture["partBytes"].as_u64().expect("retained OPC part bytes") as usize;
    let fill = fixture["fillByte"].as_u64().expect("retained OPC fill byte") as u8;
    let xml_payload_bytes = fixture["xmlPayloadBytes"].as_u64().expect("retained XML payload bytes") as usize;
    let xml_fill = fixture["xmlFillByte"].as_u64().expect("retained XML fill byte") as u8;
    let before_text = fixture["beforeText"].as_str().expect("retained OPC before text");
    let after_text = fixture["afterText"].as_str().expect("retained OPC after text");
    let maximum_turns = fixture["maximumTurns"].as_u64().expect("retained OPC maximum turns") as usize;
    assert!(part_bytes > store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
    assert!(xml_payload_bytes > store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
    assert!(xml_fill.is_ascii() && !matches!(xml_fill, b'<' | b'>' | b'&' | b'\"' | b'\''));

    let xml_payload = String::from_utf8(vec![xml_fill; xml_payload_bytes]).expect("retained XML ASCII payload");
    let mut imported = snapshot_with_blocks(vec![DocxBlock::paragraph(before_text), DocxBlock::paragraph(xml_payload)]);
    imported.opc.set_part(part_path, content_type, vec![fill; part_bytes]).expect("large retained OPC fixture owner");
    let imported = crate::engine::decode_docx(&crate::engine::encode_docx(&imported).expect("large retained DOCX fixture saves")).expect("large retained DOCX fixture imports");
    assert_eq!(imported.opc.part_bytes(part_path).expect("imported retained part").len(), part_bytes);
    let address = run(&imported).address;
    assert!(imported.xml_part(&address.part_path).expect("imported retained XML main part").document.materialization_owned_bytes().expect("retained XML owned bytes measure") > store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
    let mutation = build_set_page_mutation(&imported, &address, after_text).expect("large retained DOCX mutation validates").expect("large retained DOCX mutation changes text");

    let envelope = store::create_document_envelope(STDIO_DOCX_DOCUMENT_SCHEMA, "docx-retained-opc-lifecycle", imported, None);
    let mut store = store::ArtifactStore::new(envelope).await.expect("retained DOCX Store opens");
    store.install_document_store_owners_exact(<DocxEditor as ArtifactEditor>::build_document_store_owners().expect("DOCX retained Store owners"));
    let factory = <DocxEditor as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().expect("registered DOCX routed preparation factory");
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES };

    let generation = store.generation_now();
    let root = store.snapshot_root();
    let mut cancelled = store
        .begin_apply_batch(semio_framework_job::OperationId(1), generation, store.content_revision_now(), "docx-retained-opc-cancel".into(), vec![mutation.clone()], store::HistoryLane::Document, Some(&factory), None)
        .expect("large retained DOCX cancellation candidate admits");
    let mut progressed = false;
    for _ in 0..maximum_turns {
        match store.advance_apply_batch(&mut cancelled, grant).expect("large retained DOCX cancellation candidate advances") {
            store::ArtifactStoreOneItemAdvance::Progress(_) => {
                progressed = true;
                break;
            }
            store::ArtifactStoreOneItemAdvance::Published(_) => panic!("large retained DOCX candidate published before cancellation"),
            store::ArtifactStoreOneItemAdvance::Blocked => panic!("admitted retained DOCX copy blocked before cancellation"),
            _ => {}
        }
    }
    assert!(progressed, "large retained DOCX cancellation observes copied owner progress");
    assert!(store.cancel_apply_batch(&mut cancelled));
    for _ in 0..maximum_turns {
        if cancelled.close_step(grant).expect("cancelled retained DOCX publication closes") == store::SnapshotRetirementStep::Complete {
            break;
        }
    }
    assert!(cancelled.terminal_is_empty());
    drop(cancelled);
    assert_eq!(store.generation_now(), generation);
    assert!(std::sync::Arc::ptr_eq(&root, &store.snapshot_root()));
    drop(root);

    let mut publication = store
        .begin_apply_batch(
            semio_framework_job::OperationId(2),
            store.generation_now(),
            store.content_revision_now(),
            "docx-retained-opc-publish".into(),
            vec![mutation],
            store::HistoryLane::Document,
            Some(&factory),
            None,
        )
        .expect("large retained DOCX publication admits");
    let mut published = false;
    for _ in 0..maximum_turns {
        match store.advance_apply_batch(&mut publication, grant).expect("large retained DOCX publication advances") {
            store::ArtifactStoreOneItemAdvance::Published(_) => {
                published = true;
                break;
            }
            store::ArtifactStoreOneItemAdvance::Blocked => panic!("admitted retained DOCX publication blocked"),
            _ => {}
        }
    }
    assert!(published, "retained DOCX publication failed to terminate: phase={:?} staged={}/{} checkpoint={:?}", publication.phase(), publication.staged_items(), publication.admitted_items(), publication.progress());
    assert_eq!(run(store.snapshot_ref()).text, after_text);
    assert_eq!(store.snapshot_ref().opc.part_bytes(part_path).expect("published retained part").len(), part_bytes);
    assert!(store.snapshot_ref().xml_part(&address.part_path).expect("published retained XML main part").document.materialization_owned_bytes().expect("published retained XML owned bytes measure") > store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
    assert!(publication.acknowledge());
    for _ in 0..maximum_turns {
        if publication.close_step(grant).expect("published retained DOCX publication closes") == store::SnapshotRetirementStep::Complete {
            break;
        }
    }
    assert!(publication.terminal_is_empty());
    drop(publication);

    let saved = crate::engine::encode_docx(store.snapshot_ref()).expect("published retained DOCX saves");
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&saved)).expect("third-party ZIP oracle opens retained DOCX");
    let mut oracle_bytes = Vec::new();
    archive.by_name(part_path).expect("third-party ZIP oracle finds retained part").read_to_end(&mut oracle_bytes).expect("third-party ZIP oracle reads retained part");
    assert_eq!(oracle_bytes.len(), part_bytes);
    assert!(oracle_bytes.iter().all(|byte| *byte == fill));
    oracle_bytes.clear();
    archive.by_name(&address.part_path).expect("third-party ZIP oracle finds retained XML part").read_to_end(&mut oracle_bytes).expect("third-party ZIP oracle reads retained XML part");
    assert_eq!(oracle_bytes.iter().filter(|byte| **byte == xml_fill).count(), xml_payload_bytes);
    drop(archive);
    let reopened = crate::engine::decode_docx(&saved).expect("published retained DOCX reopens");
    assert_eq!(run(&reopened).text, after_text);
    let reopened_part = reopened.opc.part_bytes(part_path).expect("reopened retained part");
    assert_eq!(reopened_part.len(), part_bytes);
    assert_eq!(reopened_part.iter().next(), Some(fill));
    assert_eq!(reopened_part.iter().last(), Some(fill));
    assert!(reopened.xml_part(&address.part_path).expect("reopened retained XML main part").document.materialization_owned_bytes().expect("reopened retained XML owned bytes measure") > store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);

    let mut reopened_owners = preparation::document_store_owners();
    let mut reopened_retirement = reopened_owners.retire_initial_snapshot_owned(reopened);
    for _ in 0..maximum_turns {
        if reopened_retirement.close_step(1, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).expect("reopened retained DOCX retires") == store::SnapshotRetirementStep::Complete {
            break;
        }
    }
    assert!(reopened_retirement.terminal_is_empty());
    drop(reopened_retirement);
    assert_eq!(reopened_owners.close_uninstalled_owners_step(1).expect("unused reopened Store disposer closes"), store::SnapshotRetirementStep::Complete);
    assert!(reopened_owners.uninstalled_owners_terminal_is_empty());

    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<DocxSnapshot, DocxMutation>::new();
    for _ in 0..maximum_turns {
        if semio_framework_plugin::ArtifactOwnedDisposer::close_step(&mut disposer, &mut store, 1, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).expect("retained DOCX Store closes") == semio_framework_plugin::PluginCloseStep::Complete {
            break;
        }
    }
    assert!(semio_framework_plugin::ArtifactOwnedDisposer::terminal_is_empty(&disposer, &store));
}

#[semio_framework_async_macros::async_test]
async fn registered_canonical_page_edit_publishes_once_and_undoes_redoes() {
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let original = snapshot_with_blocks(vec![DocxBlock::paragraph("before")]);
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<DocxEditor>, _>(async { semio_framework_plugin::App { definition: create_docx_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_DOCX_DOCUMENT_SCHEMA) else { panic!("DOCX fixture produces a document load") };
    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let opened = app.snapshot().expect("opened DOCX fixture").clone();
    let original_address = run(&app.snapshot().unwrap()).address;
    let meta = artifact_app_laws::meta("local");

    app.handle_action("set-page", Some(&arguments(&original_address, "after")), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    assert_eq!(run(&app.snapshot().unwrap()).text, "after");

    let current_address = run(&app.snapshot().unwrap()).address;
    app.handle_action("set-page", Some(&arguments(&current_address, "after")), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), opened);
    artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id).await;
    assert_eq!(run(&app.snapshot().unwrap()).text, "after");

    app.handle_action("set-page", Some(&arguments(&original_address, "refused")), &meta).await.unwrap();
    assert!(artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.is_err());
    assert_eq!(run(&app.snapshot().unwrap()).text, "after");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn registered_run_formatting_action_saves_reopens_and_undoes_redoes() {
    use crate::schema::mutations::docx_run_formatting;
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let original = snapshot_with_blocks(vec![DocxBlock::paragraph("format me")]);
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<DocxEditor>, _>(async { semio_framework_plugin::App { definition: create_docx_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_DOCX_DOCUMENT_SCHEMA) else { panic!("DOCX fixture produces a document load") };
    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let opened = app.snapshot().expect("opened DOCX fixture").clone();
    let original_address = run(&app.snapshot().unwrap()).address;
    let meta = artifact_app_laws::meta("local");

    app.handle_action("set-run-formatting", Some(&formatting_arguments(&original_address, true, true, false)), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    let formatted_address = run(&app.snapshot().unwrap()).address;
    assert_eq!(docx_run_formatting(&app.snapshot().unwrap(), &formatted_address).unwrap(), crate::schema::mutations::DocxRunFormatting { bold: Some(true), italic: Some(true), underline: Some(false) });

    let bytes = crate::engine::encode_docx(&app.snapshot().unwrap()).expect("formatted DOCX saves");
    let reopened = crate::engine::decode_docx(&bytes).expect("formatted DOCX reopens");
    let reopened_address = run(&reopened).address;
    assert_eq!(docx_run_formatting(&reopened, &reopened_address).unwrap(), crate::schema::mutations::DocxRunFormatting { bold: Some(true), italic: Some(true), underline: Some(false) });

    artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), opened);
    artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id).await;
    let redone_address = run(&app.snapshot().unwrap()).address;
    assert_eq!(docx_run_formatting(&app.snapshot().unwrap(), &redone_address).unwrap(), crate::schema::mutations::DocxRunFormatting { bold: Some(true), italic: Some(true), underline: Some(false) });

    app.handle_action("set-run-formatting", Some(&formatting_arguments(&original_address, false, false, false)), &meta).await.unwrap();
    assert!(artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.is_err());
    assert_eq!(docx_run_formatting(&app.snapshot().unwrap(), &redone_address).unwrap(), crate::schema::mutations::DocxRunFormatting { bold: Some(true), italic: Some(true), underline: Some(false) });
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", DocxEditor, || semio_framework_plugin::App { definition: create_docx_editor(), examples: Vec::new() }, "../..");

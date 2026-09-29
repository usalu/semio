use super::*;
use crate::schema::diff::DocxBlockPath;
use crate::schema::mutations::{docx_block_run_address, docx_xml_address};
use crate::schema::snapshot::{DocxBlock, DocxDocument};
use crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_docx;

fn snapshot_with_blocks(body: Vec<DocxBlock>) -> DocxSnapshot {
    build_minimal_docx(DocxDocument { body, styles: Vec::new() })
}

fn action_arguments(address: &DocxXmlAddress, text: &str) -> dsl::DslValue {
    dsl::DslValue::object([
        (
            "address".into(),
            dsl::DslValue::object([
                ("partPath".into(), dsl::DslValue::String(address.part_path.clone())),
                ("nodePath".into(), dsl::DslValue::Array(address.node_path.iter().map(|index| dsl::DslValue::uint(*index as u64)).collect())),
                ("expectedName".into(), dsl::DslValue::String(address.expected_name.clone())),
                ("revision".into(), dsl::DslValue::String(address.revision.clone())),
            ]),
        ),
        ("text".into(), dsl::DslValue::String(text.into())),
    ])
}

#[semio_framework_async_macros::async_test]
async fn create_docx_transitional_editor_builds_a_definition_for_the_editor_role() {
    let def = create_docx_transitional_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DOCX_TRANSITIONAL_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DocxTransitionalEditor as ArtifactEditor>::DIALECT, DOCX_TRANSITIONAL_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_document_window() {
    let def = create_docx_transitional_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn set_page_replaces_paragraph_text_through_an_addressed_revision() {
    let snapshot = snapshot_with_blocks(vec![DocxBlock::paragraph("hello")]);
    let address = docx_block_run_address(&snapshot, &DocxBlockPath { segments: Vec::new(), index: 0 }, 0).expect("canonical address");
    let mutation = build_set_page_mutation(&snapshot, &address, "goodbye").expect("valid edit").expect("changed edit");
    let DocxMutation::SetRunText(crate::schema::mutations::set_run_text::SetRunText { address: actual, text }) = &mutation else { panic!("expected SetRunText") };
    assert_eq!(actual, &address);
    assert_eq!(text, "goodbye");
}

#[semio_framework_async_macros::async_test]
async fn set_page_rejects_a_non_text_target() {
    let snapshot = snapshot_with_blocks(vec![DocxBlock::Table(crate::schema::snapshot::DocxTable::default())]);
    let address = docx_xml_address(&snapshot, "word/document.xml", vec![0, 0]).expect("table address");
    assert!(build_set_page_mutation(&snapshot, &address, "text").is_err());
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = DocxTransitionalEditorCommand::SetPage {
        address: DocxXmlAddress { part_path: "word/document.xml".into(), node_path: vec![0, 2, 1], expected_name: "{urn:test}r".into(), revision: "0123456789abcdef".into() },
        text: "a\nmulti line value".into(),
    };
    let printed = <DocxTransitionalEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <DocxTransitionalEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<DocxTransitionalEditor as ArtifactEditor>::command_from_action("set-page", None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn set_page_action_requires_and_preserves_the_complete_canonical_address() {
    let snapshot = snapshot_with_blocks(vec![DocxBlock::paragraph("current")]);
    let address = docx_block_run_address(&snapshot, &DocxBlockPath { segments: Vec::new(), index: 0 }, 0).expect("canonical address");
    let arguments = action_arguments(&address, "draft");
    let command = <DocxTransitionalEditor as ArtifactEditor>::command_from_action("set-page", Some(&arguments)).expect("canonical command");
    assert_eq!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxTransitionalEditorCommand::SetPage { address, text: "draft".into() }));
}

#[semio_framework_async_macros::async_test]
async fn stale_set_page_revision_is_rejected() {
    let snapshot = snapshot_with_blocks(vec![DocxBlock::paragraph("current")]);
    let mut address = docx_block_run_address(&snapshot, &DocxBlockPath { segments: Vec::new(), index: 0 }, 0).expect("canonical address");
    address.revision = "0000000000000000".into();
    assert!(build_set_page_mutation(&snapshot, &address, "draft").is_err());
}

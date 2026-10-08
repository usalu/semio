use super::*;
use protocol::{DiffBinary,DiffCodec,DiffText, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn canonical_xml_and_opc_diff_text_binary_replay_is_exact() {
    let before = snapshot_a();
    let after = snapshot_b();
    let diff = demo_forward_diff();
    assert!(diff.xml_parts.is_some(), "semantic changes must be expressed against canonical XML parts");
    assert!(diff.opc.is_some(), "binary package changes must be expressed against OPC state");

    for replay in [diff.clone(), DocxDiff::parse_diff(&diff.print_diff()).expect("text replay"), DocxDiff::decode_diff(&diff.encode_diff().expect("binary encode")).expect("binary replay")] {
        assert_eq!(protocol::apply_diff(&replay, &before).expect("diff applies"), after);
        assert_eq!(protocol::apply_diff(&replay.inverse(&before), &after).expect("inverse applies"), before);
    }
}

#[semio_framework_async_macros::async_test]
async fn every_demo_diff_round_trips_without_semantic_shadow_state() {
    for diff in demo_diff_cases() {
        let text = diff.print_diff();
        assert_eq!(DocxDiff::parse_diff(&text).expect("text replay"), diff);
        let binary = diff.encode_diff().expect("binary encode");
        assert_eq!(DocxDiff::decode_diff(&binary).expect("binary replay"), diff);
    }
}

#[test]
fn canonical_fixture_clears_only_the_main_xml_declaration() {
    use protocol::command::DiffAlgebra;
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlQuote,retained::{RetainedXmlDeclaration,RetainedXmlText}};
    use std::io::Read;

    let fixture_text = include_str!("../../🧫️fixtures/🧹️clear-main-declaration/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(fixture_text).expect("third-party JSON parser accepts the neutral fixture");
    let fixture_diff = serde_json::to_string(&fixture["diff"]).expect("fixture diff serializes");
    let diff: DocxDiff = semio_framework_pack_json::from_json_str(&fixture_diff, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("canonical DOCX diff decodes");

    let mut before = DocxSnapshot::default();
    let main_path = fixture["mainPart"].as_str().unwrap();
    before.xml_parts.iter_mut().find(|part| part.path == main_path).unwrap().document.declaration = Some(RetainedXmlDeclaration{version:RetainedXmlText::try_from_str("1.0").unwrap(),encoding:Some(RetainedXmlText::try_from_str("UTF-8").unwrap()),standalone:Some(true),quote:XmlQuote::Double});
    before.validate_authority().expect("authored base is valid");
    let source = before.clone();

    let after = protocol::apply_diff(&diff, &before).expect("fixture diff applies");
    assert_eq!(before, source, "diff replay never rewrites its source");
    let main = after.xml_parts.iter().find(|part| part.path == main_path).unwrap();
    assert!(main.document.declaration.is_none());
    let mut expected = source.clone();
    expected.xml_parts.iter_mut().find(|part| part.path == main_path).unwrap().document.declaration = None;
    assert_eq!(after, expected, "the sparse fixture changes only the canonical XML declaration");

    assert_eq!(protocol::apply_diff(&diff.inverse(&source), &after).expect("inverse applies"), source);
    for replay in [DocxDiff::parse_diff(&diff.print_diff()).expect("text replay"), DocxDiff::decode_diff(&diff.encode_diff().expect("binary encode")).expect("binary replay")] {
        assert_eq!(protocol::apply_diff(&replay, &before).expect("codec replay applies"), after);
    }

    let bytes = crate::engine::encode_docx(&after).expect("declaration-free package publishes");
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes)).expect("third-party ZIP reader opens package");
    let mut xml = String::new();
    archive.by_name(main_path).unwrap().read_to_string(&mut xml).unwrap();
    assert!(!xml.starts_with("<?xml"), "independent package inspection observes the cleared declaration");
}

#[test]
fn archive_comment_only_diff_replay_preserves_exact_text_and_set_part_codecs_keep_the_index() {
    use crate::schema::mutations::{set_part, DocxMutation};
    use protocol::{MutationDiff, OpBinary, OpText};
use semio_framework_value::ToValue;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🎒️zip/📦️opc/🧫️fixtures/💬️archive-comment/🔣️.json")).unwrap();
    let mut before = DocxSnapshot::default();
    before.opc.comment = semio_s_artifact_stdio_zip::opc::retained::RetainedOpcText::try_from_str(fixture["before"].as_str().unwrap()).unwrap();
    let mut after = before.clone();
    after.opc.comment = semio_s_artifact_stdio_zip::opc::retained::RetainedOpcText::try_from_str(fixture["after"].as_str().unwrap()).unwrap();
    let diff = DocxDiff { opc: Some(OpcDiff { comment: Some(fixture["after"].as_str().unwrap().to_string()), ..Default::default() }), ..Default::default() };
    assert!(!diff.is_empty());
    for replay in [DocxDiff::parse_diff(&diff.print_diff()).unwrap(), DocxDiff::decode_diff(&diff.encode_diff().unwrap()).unwrap()] {
        assert_eq!(protocol::apply_diff(&replay, &before).unwrap(), after);
        assert_eq!(protocol::apply_diff(&replay.inverse(&before), &after).unwrap(), before);
    }
    let mut cleared = after.clone();
    cleared.opc.comment = semio_s_artifact_stdio_zip::opc::retained::RetainedOpcText::try_from_str(fixture["cleared"].as_str().unwrap()).unwrap();
    let mut combined = diff;
    combined.absorb(DocxDiff { opc: Some(OpcDiff { comment: Some(fixture["cleared"].as_str().unwrap().to_string()), ..Default::default() }), ..Default::default() });
    assert_eq!(protocol::apply_diff(&combined, &before).unwrap(), cleared);
    let mutation = DocxMutation::SetPart(set_part::SetPart { path: "word/media/x.bin".into(), content_type: "application/octet-stream".into(), payload: set_part::DocxPartContent::Binary { bytes: vec![1, 2] }, index: Some(1), override_index: None });
    assert_eq!(DocxMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    assert_eq!(DocxMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&after.to_value())).unwrap();
    assert_eq!(oracle["opc"]["comment"], fixture["after"]);
}

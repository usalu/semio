use super::*;
use protocol::{DiffCodec, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn canonical_xml_and_opc_diff_text_binary_replay_is_exact() {
    let before = snapshot_a();
    let after = snapshot_b();
    let diff = DocxDiff::between(&before, &after);
    assert!(diff.xml_parts.is_some(), "semantic changes must be expressed against canonical XML parts");
    assert!(diff.opc.is_some(), "binary package changes must be expressed against OPC state");

    for replay in [diff.clone(), DocxDiff::parse_diff(&diff.print_diff()).expect("text replay"), DocxDiff::decode_diff(&diff.encode_diff().expect("binary encode")).expect("binary replay")] {
        assert_eq!(replay.apply(&before).expect("diff applies"), after);
        assert_eq!(replay.inverse(&before).apply(&after).expect("inverse applies"), before);
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
    use semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration;
    use std::io::Read;

    let fixture_text = include_str!("../../🧫️fixtures/🧹️clear-main-declaration/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(fixture_text).expect("third-party JSON parser accepts the neutral fixture");
    let fixture_diff = serde_json::to_string(&fixture["diff"]).expect("fixture diff serializes");
    let diff: DocxDiff = dsl::os_pack::json::from_json_str(&fixture_diff).expect("canonical DOCX diff decodes");

    let mut before = DocxSnapshot::default();
    let main_path = fixture["mainPart"].as_str().unwrap();
    before.xml_parts.iter_mut().find(|part| part.path == main_path).unwrap().document.declaration = Some(XmlDeclaration::new("1.0", Some("UTF-8".into()), Some(true)));
    before.validate_authority().expect("authored base is valid");
    let source = before.clone();

    let after = diff.apply(&before).expect("fixture diff applies");
    assert_eq!(before, source, "diff replay never rewrites its source");
    let main = after.xml_parts.iter().find(|part| part.path == main_path).unwrap();
    assert!(main.document.declaration.is_none());
    let mut expected = source.clone();
    expected.xml_parts.iter_mut().find(|part| part.path == main_path).unwrap().document.declaration = None;
    assert_eq!(after, expected, "the sparse fixture changes only the canonical XML declaration");

    let derived = DocxDiff::between(&source, &after);
    assert_eq!(derived, diff, "the neutral fixture is the exact canonical between-diff");
    assert_eq!(diff.inverse(&source).apply(&after).expect("inverse applies"), source);
    for replay in [DocxDiff::parse_diff(&diff.print_diff()).expect("text replay"), DocxDiff::decode_diff(&diff.encode_diff().expect("binary encode")).expect("binary replay")] {
        assert_eq!(replay.apply(&before).expect("codec replay applies"), after);
    }

    let bytes = crate::engine::encode_docx(&after).expect("declaration-free package publishes");
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes)).expect("third-party ZIP reader opens package");
    let mut xml = String::new();
    archive.by_name(main_path).unwrap().read_to_string(&mut xml).unwrap();
    assert!(!xml.starts_with("<?xml"), "independent package inspection observes the cleared declaration");
}

#[test]
fn archive_comment_only_diff_and_snapshot_replay_preserve_exact_text() {
    use crate::schema::mutations::{set_snapshot, DocxMutation};
    use protocol::{MutationDiff, OpBinary, OpText, ToValue};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🎒️zip/📦️opc/🧫️fixtures/💬️archive-comment/🔣️.json")).unwrap();
    let mut before = DocxSnapshot::default();
    before.opc.comment = fixture["before"].as_str().unwrap().into();
    let mut after = before.clone();
    after.opc.comment = fixture["after"].as_str().unwrap().into();
    let diff = DocxDiff::between(&before, &after);
    assert!(!diff.is_empty());
    for replay in [DocxDiff::parse_diff(&diff.print_diff()).unwrap(), DocxDiff::decode_diff(&diff.encode_diff().unwrap()).unwrap()] {
        assert_eq!(replay.apply(&before).unwrap(), after);
        assert_eq!(replay.inverse(&before).apply(&after).unwrap(), before);
    }
    let mut cleared = after.clone();
    cleared.opc.comment = fixture["cleared"].as_str().unwrap().into();
    let mut combined = diff;
    combined.absorb(DocxDiff::between(&after, &cleared));
    assert_eq!(combined.apply(&before).unwrap(), cleared);
    let mutation = DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: after.clone() });
    assert_eq!(DocxMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    assert_eq!(DocxMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    let oracle: serde_json::Value = serde_json::from_str(&protocol::os_pack::json::to_json_string(&after.to_value())).unwrap();
    assert_eq!(oracle["opc"]["comment"], fixture["after"]);
}

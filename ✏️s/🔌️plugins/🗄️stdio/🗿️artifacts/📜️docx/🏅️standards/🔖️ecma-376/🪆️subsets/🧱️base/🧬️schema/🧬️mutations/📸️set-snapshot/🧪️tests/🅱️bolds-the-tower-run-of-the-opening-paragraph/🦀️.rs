use crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_docx;
use crate::standards::v_ecma_376::subsets::base::schema::diff::DocxDiff;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::{apply_docx_mutation, set_snapshot, DocxMutation};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxSnapshot, DocxStyle};
use protocol::command::DiffAlgebra;
use protocol::{DiffCodec, Mutation, MutationDiff, OpBinary, OpText};

fn before() -> DocxSnapshot {
    build_minimal_docx(DocxDocument {
        body: vec![DocxBlock::Paragraph(DocxParagraph {
            runs: vec![DocxRun { text: "Capsule ".into(), ..Default::default() }, DocxRun { text: "Tower".into(), ..Default::default() }],
            style: Some("Heading1".into()),
            extra_paragraph_properties: Vec::new(),
        })],
        styles: vec![DocxStyle { id: "Heading1".into(), name: "Heading 1".into(), based_on: None }],
    })
}

fn after() -> DocxSnapshot {
    let mut next = before();
    let mut document = next.project_document().unwrap();
    let DocxBlock::Paragraph(paragraph) = &mut document.body[0] else { unreachable!() };
    paragraph.runs[1].bold = true;
    next = build_minimal_docx(document);
    next
}

fn mutation() -> DocxMutation {
    DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: after() })
}

#[semio_framework_async_macros::async_test]
async fn set_snapshot_diff_replay_and_inverse_are_exact() {
    let base = before();
    let mutation = mutation();
    let raised = Mutation::diff(&mutation, &base);
    assert!(raised.diff().xml_parts.is_some());
    assert!(raised.diff().opc.is_none());
    assert_eq!(raised.diff().apply(&base).unwrap(), after());
    assert_eq!(raised.diff().inverse(&base).apply(&after()).unwrap(), base);

    let mut applied = base.clone();
    let outcome = apply_docx_mutation(&mut applied, &mutation);
    assert!(outcome.messages().is_empty());
    assert_eq!(applied, after());
    for inverse in Mutation::inverse(&mutation, &base) {
        apply_docx_mutation(&mut applied, &inverse);
    }
    assert_eq!(applied, base);
}

#[semio_framework_async_macros::async_test]
async fn mutation_and_diff_text_binary_codecs_replay_canonical_state() {
    let mutation = mutation();
    assert_eq!(DocxMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    assert_eq!(DocxMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);

    let diff = DocxDiff::between(&before(), &after());
    assert_eq!(DocxDiff::parse_diff(&diff.print_diff()).unwrap(), diff);
    assert_eq!(DocxDiff::decode_diff(&diff.encode_diff().unwrap()).unwrap(), diff);
}

#[test]
fn neutral_json_fixture_matches_the_canonical_xml_authority() {
    let before_text = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🅱️bolds-the-tower-run-of-the-opening-paragraph/📸️snapshot/⬅️before/🔣️.json");
    let after_text = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🅱️bolds-the-tower-run-of-the-opening-paragraph/📸️snapshot/➡️after/🔣️.json");
    let mutation_text = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🅱️bolds-the-tower-run-of-the-opening-paragraph/🦠️mutation/🔣️.json");
    let diff_text = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🅱️bolds-the-tower-run-of-the-opening-paragraph/🔺️diff/🔣️.json");
    for text in [before_text, after_text, mutation_text, diff_text] {
        serde_json::from_str::<serde_json::Value>(text).expect("third-party JSON parser accepts fixture");
    }

    let fixture_before: DocxSnapshot = dsl::os_pack::json::from_json_str(before_text).expect("before snapshot decodes");
    let fixture_after: DocxSnapshot = dsl::os_pack::json::from_json_str(after_text).expect("after snapshot decodes");
    let fixture_mutation: DocxMutation = dsl::os_pack::json::from_json_str(mutation_text).expect("mutation decodes");
    let fixture_diff: DocxDiff = dsl::os_pack::json::from_json_str(diff_text).expect("diff decodes");

    fixture_before.validate_authority().expect("before authority is valid");
    fixture_after.validate_authority().expect("after authority is valid");
    assert_eq!(fixture_mutation, DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: fixture_after.clone() }));
    assert_eq!(fixture_diff, DocxDiff::between(&fixture_before, &fixture_after));
    assert_eq!(fixture_diff.apply(&fixture_before).expect("fixture diff applies"), fixture_after);
    assert_eq!(fixture_diff.inverse(&fixture_before).apply(&fixture_after).expect("fixture inverse applies"), fixture_before);
}

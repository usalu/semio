use crate::schema::{diff::PptxDiff, mutations::set_snapshot::SetSnapshot, snapshot::{PptxShape, PptxTransform}};
use protocol::{command::{DiffAlgebra, MutationKind}, DiffBinary,DiffCodec,DiffText, MutationDiff};

#[test]
fn set_snapshot_diff_replaces_only_canonical_authority_and_inverts_exactly() {
    let before: crate::PptxSnapshot = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🏷️retitles/📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored canonical before");
    let after: crate::PptxSnapshot = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🏷️retitles/📸️snapshot/➡️after/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored canonical after");
    let expected: PptxDiff = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🏷️retitles/🔺️diff/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored sparse XML diff");
    let literal: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🏷️retitles/🦠️mutation/🔣️.json")).expect("independent JSON parser");
    let mutation: SetSnapshot = semio_framework_pack_json::from_json_str(&serde_json::json!({"snapshot": literal["snapshot"]}).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("actual typed mutation");
    assert_eq!(literal["mutation"], "setSnapshot");
    assert_eq!(mutation.snapshot, after);
    assert_eq!(before.opc, after.opc);
    let outcome = mutation.diff(&before);
    let diff: PptxDiff = outcome.diff().clone();
    assert_eq!(diff, expected);
    assert_eq!(diff.apply(&before).unwrap(), after);
    assert_eq!(diff.inverse(&before).apply(&after).unwrap(), before);
    assert_eq!(PptxDiff::parse_diff(&diff.print_diff()).unwrap(), diff);
    assert_eq!(PptxDiff::decode_diff(&diff.encode_diff().unwrap()).unwrap(), diff);
    for (snapshot, title, title_y) in [(&before, "Nakagin", 0), (&after, "Nakagin Capsule Tower", 457200)] {
        let presentation = snapshot.presentation().expect("derive actual typed XML authority");
        assert_eq!(presentation.slides.len(), 1);
        assert_eq!(presentation.slides[0].shapes.len(), 2);
        let PptxShape::Placeholder {kind, text_frame, position} = &presentation.slides[0].shapes[0] else {panic!("authored title placeholder")};
        assert_eq!(kind, "title");
        assert_eq!(text_frame[0].runs[0].text, title);
        assert!(!text_frame[0].runs[0].bold && !text_frame[0].runs[0].italic);
        assert_eq!(*position, PptxTransform {x: 0, y: title_y, cx: 9144000, cy: 1143000});
        let PptxShape::Picture {blip_rel_id, position} = &presentation.slides[0].shapes[1] else {panic!("authored independent picture")};
        assert_eq!(blip_rel_id, "rId2");
        assert_eq!(*position, PptxTransform {x: 0, y: 1143000, cx: 9144000, cy: 4000000});
        for part in &snapshot.xml_parts {
            let text = snapshot.part_text(&part.path).expect("render actual XML part");
            let mut reader = quick_xml::reader::Reader::from_str(&text);
            loop {if matches!(reader.read_event().expect("QuickXML independently recognizes full authored part"), quick_xml::events::Event::Eof) {break;}}
        }
    }
}

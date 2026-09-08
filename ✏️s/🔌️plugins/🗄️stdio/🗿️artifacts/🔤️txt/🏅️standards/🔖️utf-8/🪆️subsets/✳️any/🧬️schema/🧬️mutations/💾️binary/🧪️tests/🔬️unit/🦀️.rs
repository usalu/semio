
use super::*;
use crate::schema::snapshot::LineEnding;
use protocol::OpBinary;

#[test]
fn generic_framing_tags_payloads_parse_and_walk_all_leaf_frames() {
    let spec = dsl::parse_protocol(COMPONENT_PROTOCOL_SEMIO).expect("parse one-byte tag protocol");
    let frames = [
        (1, TxtMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewlineMutation { value: true })),
        (2, TxtMutation::SetLineEnding(set_line_ending::SetLineEndingMutation { value: LineEnding::CrLf })),
        (3, TxtMutation::InsertLine(insert_line::InsertLineMutation { index: 0, text: "x".into() })),
        (4, TxtMutation::RemoveLine(remove_line::RemoveLineMutation { index: 0 })),
        (5, TxtMutation::SetLine(set_line::SetLineMutation { index: 0, text: "x".into() })),
    ];
    for (tag, mutation) in frames {
        let frame = mutation.encode_op().expect("encode current leaf frame");
        assert_eq!(frame.first(), Some(&tag));
        assert!(frame.len() > 1);
        let trace = dsl::walk_protocol(&spec, &frame).expect("walk current leaf frame");
        assert_eq!(trace.consumed, frame.len());
        assert_eq!(TxtMutation::decode_op(&frame).expect("decode current leaf frame"), mutation);
    }
    for frame in [Vec::new(), vec![255], vec![1], vec![2], vec![3], vec![4], vec![5]] {
        assert!(TxtMutation::decode_op(&frame).is_err(), "{frame:?}");
    }
}

#[test]
fn generic_framing_refuses_missing_or_unknown_tags() {
    for frame in [Vec::new(), vec![255]] {
        assert!(TxtMutation::decode_op(&frame).is_err(), "{frame:?}");
    }
}


//! 🔒️ Byte-level `OpBinary` round-trip guard for the semantic-mutations-overhaul vocabulary
//! (ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`). The pre-overhaul whole-record-upsert / whole-document-replace wire
//! bytes this guard used to freeze no longer exist — that vocabulary is banned outright, not
//! preserved — so this now asserts the NEW operations' `OpText`/`OpBinary` round-trip
//! (`print_op`/`parse_op`, `encode_op`/`decode_op`) instead of pinning byte literals for a wire
//! shape this ticket deliberately changed.
use super::*;
use crate::Puzzle2dNode;
use crate::standards::v1::subsets::any::schema::mutations::{change_manifest_id, connect_handles, create_node, delete_node, disconnect_handles, move_node};
use protocol::OpText;

fn ops() -> Vec<Puzzle2dMutation> {
    let node = Puzzle2dNode {
        id: "n1".into(),
        node_kind: Some("Base".into()),
        shape: Some("circle".into()),
        x: 1.5,
        y: -2.25,
        radius: Some(3.0),
        text: Some("hi".into()),
        icon_kind: Some("base".into()),
        root: Some(true),
        scale: Some(2.0),
        visible: Some(true),
        locked: Some(false),
        ..Default::default()
    };
    vec![
        create_node(node, Some(0)),
        move_node("n1".into(), 4.0, 5.0),
        delete_node("n1".into()),
        connect_handles("e1".into(), "n1:h0".into(), "n2:h0".into(), Some("wire.link".into()), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Some("none".into()), Some("arrow".into())),
        disconnect_handles("e1".into()),
        change_manifest_id(Some("nakagin".into())),
    ]
}

/// ⚖️ Every operation still prints, parses, encodes, and decodes back to an equal value.
#[test]
fn operations_round_trip_text_and_binary() {
    let operations = ops();
    assert!(!operations.is_empty());
    for operation in &operations {
        let line = operation.print_op();
        assert_eq!(&Puzzle2dMutation::parse_op(&line).expect("parse_op"), operation);
        let bytes = encode_op(operation).expect("encode");
        assert_eq!(&decode_op(&bytes).expect("decode"), operation);
    }
}

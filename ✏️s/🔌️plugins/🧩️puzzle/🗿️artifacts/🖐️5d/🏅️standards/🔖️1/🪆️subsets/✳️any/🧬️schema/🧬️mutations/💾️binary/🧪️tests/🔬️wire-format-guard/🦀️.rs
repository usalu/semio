
//! 🔒️ Byte-level `OpBinary` round-trip guard for the semantic-mutations-overhaul vocabulary
//! (ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`). The pre-overhaul whole-record-upsert /
//! whole-document-replace wire bytes this guard used to freeze no longer exist — that
//! vocabulary is banned outright, not preserved — so this now asserts the NEW operations'
//! `OpText`/`OpBinary` round-trip instead of pinning byte literals for a wire shape this ticket
//! deliberately changed.
use super::*;
use crate::Puzzle5dPart;
use crate::standards::v1::subsets::any::schema::mutations::{change_description, change_domain, connect_grips, create_part, delete_part, disconnect_grips};
use protocol::OpText;

fn ops() -> Vec<Puzzle5dMutation> {
    let part = Puzzle5dPart { id: "p1".into(), part_kind: Some("Capsule".into()), ..Default::default() };
    vec![
        create_part(part, Some(0)),
        delete_part("p1".into()),
        connect_grips("f1".into(), "p1:g0".into(), "p2:g0".into(), Some("fk".into()), 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.0, 0.0),
        disconnect_grips("f1".into()),
        change_domain("architecture".into()),
        change_description("a scene".into()),
    ]
}

/// ⚖️ Every operation still prints, parses, encodes, and decodes back to an equal value.
#[test]
fn operations_round_trip_text_and_binary() {
    let operations = ops();
    assert!(!operations.is_empty());
    for operation in &operations {
        let line = operation.print_op();
        assert_eq!(&Puzzle5dMutation::parse_op(&line).expect("parse_op"), operation);
        let bytes = encode_op(operation).expect("encode");
        assert_eq!(&decode_op(&bytes).expect("decode"), operation);
    }
}

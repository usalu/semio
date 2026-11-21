
//! 🔒️ Tagged mutation wire framing and text / binary round trips.
use crate::standards::v1::subsets::any::io::binary::mutations::{decode_op, encode_op};
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart;
use crate::standards::v1::subsets::any::schema::mutations::change_description;
use crate::standards::v1::subsets::any::schema::mutations::change_domain;
use crate::standards::v1::subsets::any::schema::mutations::connect_grips;
use crate::standards::v1::subsets::any::schema::mutations::create_part;
use crate::standards::v1::subsets::any::schema::mutations::delete_part;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_grips;

use protocol::OpText;

fn ops() -> Vec<(u8, Puzzle5dMutation)> {
    let part = Puzzle5dPart { id: "p1".into(), part_kind: Some("Capsule".into()), ..Default::default() };
    vec![
        (0, create_part(part, Some(0))),
        (1, delete_part("p1".into())),
        (18, connect_grips("f1".into(), "p1:g0".into(), "p2:g0".into(), Some("fk".into()), 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.0, 0.0, None)),
        (19, disconnect_grips("f1".into())),
        (23, change_domain("architecture".into())),
        (24, change_description("a scene".into())),
    ]
}

/// ⚖️ Every operation still prints, parses, encodes, and decodes back to an equal value.
#[test]
fn operations_round_trip_text_and_binary() {
    let operations = ops();
    assert!(!operations.is_empty());
    for (tag, operation) in &operations {
        let line = operation.print_op();
        assert_eq!(&Puzzle5dMutation::parse_op(&line).expect("parse_op"), operation);
        let bytes = encode_op(operation).expect("encode");
        assert_eq!(bytes.get(..2), Some([1, *tag].as_slice()));
        assert_eq!(&decode_op(&bytes).expect("decode"), operation);
    }
}

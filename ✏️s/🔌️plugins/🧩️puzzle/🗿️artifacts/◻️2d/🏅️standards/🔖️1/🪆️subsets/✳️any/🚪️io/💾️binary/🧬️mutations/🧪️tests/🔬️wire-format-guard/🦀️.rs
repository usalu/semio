//! 🔒️ Puzzle 2D mutation text round trips, authored Pack tags and a shared byte fixture.
use crate::standards::v1::subsets::any::io::binary::mutations::*;
use crate::standards::v1::subsets::any::schema::mutations::change_manifest_id;
use crate::standards::v1::subsets::any::schema::mutations::connect_handles;
use crate::standards::v1::subsets::any::schema::mutations::create_node;
use crate::standards::v1::subsets::any::schema::mutations::delete_node;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_handles;
use crate::standards::v1::subsets::any::schema::mutations::drag_selection;
use crate::standards::v1::subsets::any::schema::mutations::move_node;
use crate::standards::v1::subsets::any::schema::mutations::rotate_selection;
use crate::standards::v1::subsets::any::schema::mutations::scale_selection;

use crate::standards::v1::subsets::any::schema::records::Puzzle2dNode;
use protocol::OpText;
use semio_framework_dsl_record::DslVariants;
use semio_framework_value::RetainedCloneGrant;
use semio_os_kernel::original_receiving::{ReceivingStock, receive_to_completion};

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
        connect_handles("e1".into(), "n1:h0".into(), "n2:h0".into(), Some("wire.link".into()), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Some("none".into()), Some("arrow".into()), None),
        disconnect_handles("e1".into()),
        change_manifest_id(Some("nakagin".into())),
        drag_selection(vec!["n1".into(), "region-1".into()].into(), 5.0, -2.5),
        rotate_selection(vec!["n1".into()].into(), 20.0, 10.0, std::f64::consts::FRAC_PI_2),
        scale_selection(vec!["n1".into(), "region-1".into()].into(), 20.0, 10.0, 0.5),
    ]
}

/// ⚖️ Every operation still prints, parses, encodes, and decodes back to an equal value.
#[test]
fn operations_round_trip_text_and_binary() {
    let operations = ops();
    assert!(!operations.is_empty());
    let grant = RetainedCloneGrant {
        maximum_items: 1 << 20,
        maximum_copy_bytes: 1 << 24,
        maximum_capacity_bytes: 1 << 24,
        maximum_release_bytes: 1 << 24,
        maximum_depth: 128,
    };
    for operation in &operations {
        let line = operation.print_op();
        assert_eq!(&Puzzle2dMutation::parse_op(&line).expect("parse_op"), operation);
        let bytes = encode_op(operation).expect("encode");
        assert_eq!(&decode_op(&bytes).expect("decode"), operation);
        let received = receive_to_completion::<Puzzle2dMutation>(
            &bytes,
            (0, bytes.len()),
            ReceivingStock { stock: grant, quantum: grant, fuel: 1 << 20 },
        ).expect("original binary receiving");
        assert_eq!(&received, operation);
    }
}

/// 🪪️ Native receiving uses the same tags and concrete record body as the authored Pack wire.
#[test]
fn replacement_wire_matches_declared_tags_and_record_body() {
    let authored = dsl::protocol_record::records(COMPONENT_PROTOCOL_SEMIO).collect::<Vec<_>>();
    let variants = Puzzle2dMutation::variants();
    assert_eq!(authored.len(), variants.len());
    for ((tag, (kind, _)), (authored_kind, authored_tag)) in variants.iter().enumerate().zip(authored) {
        assert_eq!((kind.as_str(), tag as u64), (authored_kind, authored_tag));
    }

    let fixture: serde_json::Value = serde_json::from_str(include_str!("delete-node.json")).unwrap();
    let operation = delete_node(fixture["id"].as_str().unwrap().into());
    let bytes = encode_op(&operation).unwrap();
    assert_eq!(u64::from(bytes[0]), fixture["format"].as_u64().unwrap());
    assert_eq!(u64::from(bytes[1]), fixture["tag"].as_u64().unwrap());
    let hex = bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    assert_eq!(hex, fixture["hex"].as_str().unwrap());
    assert_eq!(decode_op(&bytes).unwrap(), operation);
    let Some(store::ArtifactPreparedOperationSource::Pack { tag, spec, .. }) = prepared_operation_wire_source(&operation) else {
        panic!("replacement comparison must borrow the authored Pack body");
    };
    assert_eq!(tag, fixture["tag"].as_u64().unwrap());
    assert_eq!(spec.keyword, fixture["kind"].as_str());
}

/// 🏷️ The three selection transforms print under their own opcode and sit at their own protocol tags
/// (`drag-selection` 33, `rotate-selection` 34, `scale-selection` 35): the frame is `format u8` then the
/// tag varint, so the second byte names the kind.
#[test]
fn selection_transforms_print_their_opcode_and_carry_their_tag() {
    let cases: [(Puzzle2dMutation, &str, u8); 3] = [
        (drag_selection(vec!["n1".into()].into(), 1.0, 2.0), "drag-selection", 33u8),
        (rotate_selection(vec!["n1".into()].into(), 0.0, 0.0, 1.0), "rotate-selection", 34),
        (scale_selection(vec!["n1".into()].into(), 0.0, 0.0, 2.0), "scale-selection", 35),
    ];
    for (operation, opcode, tag) in cases {
        let line = operation.print_op();
        assert!(line.starts_with(&format!("{opcode} ")), "{line}");
        assert_eq!(encode_op(&operation).expect("encode")[1], tag, "{opcode} must sit at protocol tag {tag}");
    }
}

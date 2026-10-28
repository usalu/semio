use super::*;
use protocol::{DiffBinary,DiffCodec,DiffText};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn entity(id: u64, name: &str, args: Vec<IfcValue>) -> IfcEntity {
    IfcEntity { id, name: name.into(), args, complex: vec![] }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base() -> IfcSnapshot {
    IfcSnapshot {
        schema: "stdio.ifc".into(),
        header: crate::standards::v4::subsets::any::schema::snapshot::IfcHeader { file_description: vec![IfcValue::String("d".into())], file_name: vec![IfcValue::String("n".into())], file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])] },
        entities: vec![
            entity(1, "IFCPROJECT", vec![IfcValue::String("gid".into()), IfcValue::Reference(2), IfcValue::Unset, IfcValue::Derived]),
            IfcEntity {
                id: 2,
                name: "IFCQUANTITYAREA".into(),
                args: vec![IfcValue::Real(10.5), IfcValue::Integer(-3), IfcValue::Enum("EDGE".into())],
                complex: vec![IfcComplexType { name: "IFCPHYSICALSIMPLEQUANTITY".into(), args: vec![IfcValue::Unset] }],
            },
        ],
    }
}

/// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `IfcDiff` grammar — exercises every
/// `IfcValue` variant (incl. `Aggregate`/`TypedValue` recursion), the `entities` collection
/// triple, and the nested per-entity `args` collection triple + `complex` weak-list replace.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let cases = demo_diff_cases();
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = IfcDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = IfcDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}

#[test]
fn absorb_preserves_insert_position_when_later_removal_is_after_it() {
    let original = base();
    let mut first = diff_insert_entity(0, entity(99, "IFCSITE", vec![]));
    let middle = protocol::apply_diff(&first, &original).unwrap();
    let later = diff_remove_entity(2, 2);
    let expected = protocol::apply_diff(&later, &middle).unwrap();
    first.absorb(later);
    assert_eq!(protocol::apply_diff(&first, &original).unwrap(), expected);
}

#[test]
fn absorb_preserves_insert_position_when_earlier_insert_is_annihilated() {
    let original = base();
    let mut first = diff_insert_entity(0, entity(99, "IFCSITE", vec![]));
    first.absorb(diff_insert_entity(2, entity(100, "IFCBUILDING", vec![])));
    let middle = protocol::apply_diff(&first, &original).unwrap();
    let later = diff_remove_entity(99, 0);
    let expected = protocol::apply_diff(&later, &middle).unwrap();
    first.absorb(later);
    assert_eq!(protocol::apply_diff(&first, &original).unwrap(), expected);
}

#[test]
fn removed_identity_must_match_original_position() {
    let original = base();
    assert!(protocol::apply_diff(&diff_remove_entity(2, 0), &original).is_err());
}

#[test]
fn remove_and_reinsert_identity_composes_and_inverts() {
    let original = base();
    let mut combined = diff_remove_entity(1, 0);
    combined.absorb(diff_insert_entity(1, entity(1, "IFCSITE", vec![])));
    let changed = protocol::apply_diff(&combined, &original).unwrap();
    assert_eq!(changed.entities.iter().map(|entity| entity.id).collect::<Vec<_>>(), vec![2, 1]);
    assert_eq!(protocol::apply_diff(&combined.inverse(&original), &changed).unwrap(), original);
}

#[test]
fn diff_binary_rejects_unknown_framing_and_trailing_payload() {
    for bytes in [vec![0, 0], vec![1, 16], vec![1, 0, 0]] {
        assert!(IfcDiff::decode_diff(&bytes).is_err());
    }
}

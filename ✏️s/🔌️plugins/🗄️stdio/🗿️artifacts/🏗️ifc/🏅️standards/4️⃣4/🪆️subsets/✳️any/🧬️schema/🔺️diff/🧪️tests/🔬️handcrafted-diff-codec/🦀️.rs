use super::*;
use protocol::DiffCodec;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn entity(id: u64, name: &str, args: Vec<IfcValue>) -> IfcEntity {
    IfcEntity { id, name: name.into(), args, complex: vec![] }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base() -> IfcSnapshot {
    IfcSnapshot {
        schema: "stdio.ifc".into(),
        header: crate::schema::snapshot::IfcHeader { file_description: vec![IfcValue::String("d".into())], file_name: vec![IfcValue::String("n".into())], file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])] },
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
    let a = base();
    let mut b = base();
    b.header.file_name = vec![IfcValue::String("changed".into())];
    b.entities.remove(0); // remove id 1
    b.entities[0].name = "IFCQUANTITYVOLUME".into(); // modify id 2
    b.entities[0].args = vec![IfcValue::TypedValue { name: "IFCLENGTHMEASURE".into(), items: vec![IfcValue::Real(3000.0)] }];
    b.entities[0].complex = vec![];
    b.entities.push(entity(300, "IFCBUILDINGSTOREY", vec![IfcValue::Aggregate(vec![IfcValue::Integer(1), IfcValue::Integer(2)])]));

    let cases = vec![IfcDiff::default(), IfcDiff::between(&a, &b), IfcDiff::between(&b, &a), IfcDiff::between(&a, &a)];
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

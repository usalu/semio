use super::*;

#[semio_framework_async_macros::async_test]
async fn invalid_instance_order_is_rejected_before_mutation() {
    let base = Ifc2x3Snapshot::default();
    let diff = Ifc2x3Diff { instance_order: Some(vec![1]), ..Default::default() };
    let error = protocol::apply_diff(&diff, &base).expect_err("unknown instance order target must be rejected");
    assert_eq!(error.code, "mutation.apply.invalid-instance-order");
    assert_eq!(error.target, vec!["instanceOrder", "1"]);
    assert_eq!(base, Ifc2x3Snapshot::default());
}
use semio_s_artifact_stdio_contract::part21::Part21Value;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn inst(id: u64, name: &str) -> Part21Instance {
    Part21Instance { id, entities: vec![(name.to_string(), vec![Part21Value::Int(id as i64)])] }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snap(schema: &str, header: Part21Header, instances: Vec<Part21Instance>) -> Ifc2x3Snapshot {
    let document = semio_s_artifact_stdio_contract::part21::Part21Document { header, instances };
    Ifc2x3Snapshot { schema: schema.into(), document, edm_preamble: None }
}

#[semio_framework_async_macros::async_test]
async fn absorb_upsert_then_remove_same_id_cancels_to_removed_only() {
    let mut d1 = Ifc2x3Diff { upserted_instances: vec![inst(5, "IFCSLAB")], ..Default::default() };
    let d2 = Ifc2x3Diff { removed_instances: vec![5], ..Default::default() };
    d1.absorb(d2);
    assert!(d1.upserted_instances.is_empty());
    assert_eq!(d1.removed_instances, vec![5]);
}

#[semio_framework_async_macros::async_test]
async fn absorb_remove_then_upsert_same_id_un_removes() {
    let mut d1 = Ifc2x3Diff { removed_instances: vec![7], ..Default::default() };
    let d2 = Ifc2x3Diff { upserted_instances: vec![inst(7, "IFCBEAM")], ..Default::default() };
    d1.absorb(d2);
    assert!(d1.removed_instances.is_empty());
    assert_eq!(d1.upserted_instances, vec![inst(7, "IFCBEAM")]);
}

#[semio_framework_async_macros::async_test]
async fn absorb_matches_sequential_apply() {
    let base = snap("stdio.ifc.2x3", Part21Header::default(), vec![inst(1, "IFCWALL")]);
    let d1 = Ifc2x3Diff { upserted_instances: vec![inst(2, "IFCDOOR")], ..Default::default() };
    let d2 = Ifc2x3Diff { removed_instances: vec![1], upserted_instances: vec![inst(3, "IFCWINDOW")], ..Default::default() };
    let mut merged = d1.clone();
    merged.absorb(d2.clone());
    let sequential = {
        let mid = protocol::apply_diff(&d1, &base).expect("valid first diff");
        protocol::apply_diff(&d2, &mid).expect("valid second diff")
    };
    assert_eq!(protocol::apply_diff(&merged, &base).expect("valid absorbed diff"), sequential);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diff_level_roundtrip() {
    let base = snap("stdio.ifc.2x3", Part21Header::default(), vec![inst(1, "IFCWALL"), inst(2, "IFCDOOR")]);
    let d = Ifc2x3Diff { removed_instances: vec![2], upserted_instances: vec![inst(1, "IFCWALLSTANDARDCASE"), inst(4, "IFCCOLUMN")], ..Default::default() };
    let next = protocol::apply_diff(&d, &base).expect("valid forward diff");
    let inv = d.inverse(&base);
    assert_eq!(protocol::apply_diff(&inv, &next).expect("valid inverse diff"), base);
}

//#region 🔖️diff_codec_text_binary_roundtrip_law
/// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `Ifc2x3Diff` grammar — exercises every
/// top-level field (`schema`/`header`/`removed`/`upserted`) and every `Part21Value` tag incl.
/// `List`/`Typed` recursion and a real COMPLEX instance (2-entry `entities`).
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    use protocol::{DiffBinary,DiffCodec,DiffText};
    let complex_inst = Part21Instance { id: 9, entities: vec![("IFCQUANTITYAREA".into(), vec![Part21Value::Real(10.5.into()), Part21Value::Int(-3), Part21Value::Enum("EDGE".into())]), ("IFCPHYSICALSIMPLEQUANTITY".into(), vec![Part21Value::Unset])] };
    let cases = vec![
        Ifc2x3Diff::default(),
        Ifc2x3Diff {
            schema: Some("stdio.ifc.2x3.v2".into()),
            header: Some(Part21Header {
                file_description: vec![Part21Value::Str("desc".into())],
                file_name: vec![Part21Value::List(vec![Part21Value::Str("a".into()), Part21Value::Unset])],
                file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
            }),
            removed_instances: vec![1, 2],
            upserted_instances: vec![complex_inst.clone(), Part21Instance { id: 300, entities: vec![("IFCBUILDINGSTOREY".into(), vec![Part21Value::Typed { name: "IFCLENGTHMEASURE".into(), items: vec![Part21Value::Real(3000.0.into())] }])] }],
            edm_preamble: None,
            instance_order: None,
        },
        Ifc2x3Diff { removed_instances: vec![7], ..Default::default() },
        Ifc2x3Diff { upserted_instances: vec![complex_inst], ..Default::default() },
    ];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = Ifc2x3Diff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e:?}"));
        let decoded = Ifc2x3Diff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e:?}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
//#endregion 🔖️diff_codec_text_binary_roundtrip_law

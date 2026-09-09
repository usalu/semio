use super::*;
use crate::schema::diff::PlyElementsDiff;
use crate::schema::mutations::apply_ply_mutation;
use crate::schema::mutations::{add_element, insert_comment, insert_row, remove_element, remove_row, set_format, set_row_property, set_snapshot};
use crate::schema::{demo_ply_snapshot, empty_ply_snapshot};
use crate::{PlyDiff, PlyMutation};
use protocol::command::DiffAlgebra;
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn missing_element_target_is_rejected_before_mutation() {
    let base = PlySnapshot::default();
    let diff = PlyDiff { elements: Some(PlyElementsDiff { removed: vec!["missing".into()], ..Default::default() }), ..Default::default() };
    let error = diff.apply(&base).expect_err("missing element target must be rejected");
    assert_eq!(error.code, "invalid-remove-target");
    assert_eq!(error.target, vec!["elements", "missing"]);
    assert_eq!(base, PlySnapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = empty_ply_snapshot();
    assert_eq!(snapshot.schema, STDIO_PLY_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn codec_round_trip() {
    let snap = empty_ply_snapshot();
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <PlySnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed.schema, snap.schema);
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <PlySnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

//#region 🔖️MeshFixture
/// 🔺 A tetrahedron expressed as real `vertex`/`face` elements: 4 vertices, 4 triangular
/// faces (via a `list uchar int vertex_indices` property) — small enough to hand-check,
/// non-trivial enough (mixed-sign coords, several list-shaped faces) to catch layout bugs.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn tetrahedron() -> PlySnapshot {
    let vertex_props = vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }, PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float }, PlyProperty::Scalar { name: "z".into(), kind: PlyScalarType::Float }];
    let face_props = vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }];
    let vertex_rows: Vec<PlyRow> = [(0.0f32, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)].into_iter().map(|(x, y, z)| PlyRow { values: vec![PlyValue::Float(x), PlyValue::Float(y), PlyValue::Float(z)] }).collect();
    let face_rows: Vec<PlyRow> = [[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]].into_iter().map(|idx: [i32; 3]| PlyRow { values: vec![PlyValue::List(idx.into_iter().map(PlyValue::Int).collect())] }).collect();
    PlySnapshot {
        schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::Ascii,
        comments: Vec::new(),
        elements: vec![PlyElement { name: "vertex".into(), count: 4, properties: vertex_props, rows: vertex_rows }, PlyElement { name: "face".into(), count: 4, properties: face_props, rows: face_rows }],
    }
}
//#endregion 🔖️MeshFixture

#[semio_framework_async_macros::async_test]
async fn ascii_tetrahedron_round_trip() {
    let snap = tetrahedron();
    let bytes = encode_ply_with_format(&snap, PlyFormat::Ascii).expect("encode ascii");
    let decoded = decode_ply(&bytes).expect("decode ascii");
    assert_eq!(decoded.elements, snap.elements);
    assert_eq!(decoded.format, PlyFormat::Ascii);
}

#[semio_framework_async_macros::async_test]
async fn binary_little_endian_tetrahedron_round_trip() {
    let snap = tetrahedron();
    let bytes = encode_ply_with_format(&snap, PlyFormat::BinaryLittleEndian).expect("encode binary LE");
    assert!(bytes.starts_with(b"ply\nformat binary_little_endian 1.0\n"), "header must declare binary_little_endian");
    let decoded = decode_ply(&bytes).expect("decode binary LE");
    assert_eq!(decoded.elements, snap.elements, "binary LE elements must exactly match the original");
}

#[semio_framework_async_macros::async_test]
async fn binary_big_endian_tetrahedron_round_trip() {
    let snap = tetrahedron();
    let bytes = encode_ply_with_format(&snap, PlyFormat::BinaryBigEndian).expect("encode binary BE");
    let decoded = decode_ply(&bytes).expect("decode binary BE");
    assert_eq!(decoded.elements, snap.elements, "binary BE elements must exactly match the original");
}

#[semio_framework_async_macros::async_test]
async fn binary_decode_skips_unmodeled_properties() {
    let header = "ply\nformat binary_little_endian 1.0\nelement vertex 1\nproperty float x\nproperty float y\nproperty float z\nproperty float nx\nproperty float ny\nproperty float nz\nend_header\n";
    let mut bytes = header.as_bytes().to_vec();
    for f in [1.5f32, 2.5, 3.5, 9.0, 9.0, 9.0] {
        bytes.extend_from_slice(&f.to_le_bytes());
    }
    let decoded = decode_ply(&bytes).expect("decode with retained normal properties");
    assert_eq!(decoded.elements.len(), 1);
    let vertex = &decoded.elements[0];
    assert_eq!(vertex.properties.len(), 6, "all 6 declared properties retained, none dropped");
    assert_eq!(vertex.rows[0].values[0], PlyValue::Float(1.5));
    assert_eq!(vertex.rows[0].values[3], PlyValue::Float(9.0), "nx retained, not silently discarded");
}

#[semio_framework_async_macros::async_test]
async fn ascii_decode_rejects_truncated_body() {
    let header = "ply\nformat ascii 1.0\nelement vertex 2\nproperty float x\nproperty float y\nproperty float z\nend_header\n1 2 3\n";
    let err = decode_ply(header.as_bytes()).unwrap_err();
    assert!(err.contains("eof"), "unexpected error: {err}");
}

#[semio_framework_async_macros::async_test]
async fn missing_end_header_is_rejected() {
    let err = decode_ply(b"ply\nformat ascii 1.0\n").unwrap_err();
    assert!(err.contains("end_header"));
}

#[semio_framework_async_macros::async_test]
async fn comments_are_retained_in_order() {
    let text = "ply\nformat ascii 1.0\ncomment first\ncomment second\nelement vertex 0\nproperty float x\nend_header\n";
    let decoded = decode_ply(text.as_bytes()).expect("decode with comments");
    assert_eq!(decoded.comments, vec!["first".to_string(), "second".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn arbitrary_named_element_round_trips() {
    let props = vec![PlyProperty::Scalar { name: "weight".into(), kind: PlyScalarType::Double }, PlyProperty::List { name: "endpoints".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::UShort }];
    let rows = vec![PlyRow { values: vec![PlyValue::Double(2.5), PlyValue::List(vec![PlyValue::UShort(3), PlyValue::UShort(7)])] }];
    let snap = PlySnapshot { schema: STDIO_PLY_DOCUMENT_SCHEMA.into(), format: PlyFormat::Ascii, comments: vec![], elements: vec![PlyElement { name: "edge".into(), count: 1, properties: props, rows }] };
    let bytes = encode_ply(&snap).expect("encode");
    let decoded = decode_ply(&bytes).expect("decode");
    assert_eq!(decoded.elements, snap.elements);
}
//#endregion

//#region 🔖️LawFixtures
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn law_base() -> PlySnapshot {
    tetrahedron()
}
//#endregion

//#region 🔖️MutationDiffLaw
/// 1️⃣ `mutation_diff_law`: ∀ variant, `m.diff(base).diff().apply(base)` matches
/// `apply_ply_mutation`'s in-place result, and the returned diff equals `m.diff(base)`.
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    let base = law_base();
    let variants = vec![
        PlyMutation::SetFormat(set_format::SetFormat { format: PlyFormat::BinaryLittleEndian }),
        PlyMutation::InsertComment(insert_comment::InsertComment { index: 0, comment: "hello".into() }),
        PlyMutation::AddElement(add_element::AddElement {
            index: 0,
            element: PlyElement { name: "material".into(), count: 1, properties: vec![PlyProperty::Scalar { name: "shininess".into(), kind: PlyScalarType::Float }], rows: vec![PlyRow { values: vec![PlyValue::Float(0.5)] }] },
        }),
        PlyMutation::RemoveElement(remove_element::RemoveElement { name: "face".into() }),
        PlyMutation::InsertRow(insert_row::InsertRow { element_name: "vertex".into(), index: 1, row: PlyRow { values: vec![PlyValue::Float(9.0), PlyValue::Float(9.0), PlyValue::Float(9.0)] } }),
        PlyMutation::RemoveRow(remove_row::RemoveRow { element_name: "vertex".into(), index: 0 }),
        PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name: "vertex".into(), row_index: 0, property_name: "x".into(), value: PlyValue::Float(42.0) }),
        PlyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: PlySnapshot::default() }),
    ];
    for m in variants {
        let mut snapshot = base.clone();
        let returned = apply_ply_mutation(&mut snapshot, &m);
        let expected_diff = m.diff(&base);
        assert_eq!(returned, expected_diff, "returned diff must equal m.diff(base) for {m:?}");
        assert_eq!(snapshot, expected_diff.diff().apply(&base).expect("valid mutation diff"), "apply_ply_mutation result must equal diff.diff().apply(base) for {m:?}");
    }
}
//#endregion

//#region 🔖️InverseLaw
/// 2️⃣ `inverse_law`: mutation-level round trip for every variant, plus diff-level
/// `d.diff().inverse(base).apply(&d.diff().apply(base)) == base`.
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let base = law_base();
    let variants = vec![
        PlyMutation::SetFormat(set_format::SetFormat { format: PlyFormat::BinaryBigEndian }),
        PlyMutation::InsertComment(insert_comment::InsertComment { index: 0, comment: "note".into() }),
        PlyMutation::AddElement(add_element::AddElement { index: 2, element: PlyElement { name: "edge".into(), count: 0, properties: vec![], rows: vec![] } }),
        PlyMutation::RemoveElement(remove_element::RemoveElement { name: "face".into() }),
        PlyMutation::InsertRow(insert_row::InsertRow { element_name: "vertex".into(), index: 0, row: PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(1.0), PlyValue::Float(1.0)] } }),
        PlyMutation::RemoveRow(remove_row::RemoveRow { element_name: "vertex".into(), index: 2 }),
        PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name: "vertex".into(), row_index: 1, property_name: "y".into(), value: PlyValue::Float(-1.0) }),
    ];
    for m in variants {
        let mut snapshot = base.clone();
        let d = apply_ply_mutation(&mut snapshot, &m);
        for inv in <PlyMutation as Mutation<PlySnapshot>>::inverse(&m, &base) {
            let mut undone = snapshot.clone();
            apply_ply_mutation(&mut undone, &inv);
            assert_eq!(undone, base, "mutation-level inverse must restore base for {m:?}");
        }
        let d_inv = d.diff().inverse(&base);
        let mutated = d.diff().apply(&base).expect("valid forward diff");
        assert_eq!(d_inv.apply(&mutated).expect("valid inverse diff"), base, "diff-level inverse must restore base for {m:?}");
    }
}
//#endregion

//#region 🔖️AbsorbLaw
/// 3️⃣ `absorb_law`: curated op pairs (Insert+Remove-before, Insert+Insert-same-index,
/// Add+SetField, Modify+Remove per key kind) plus associativity.
#[semio_framework_async_macros::async_test]
async fn absorb_law_insert_then_remove_before() {
    let base = law_base();
    let m1 = PlyMutation::InsertRow(insert_row::InsertRow { element_name: "vertex".into(), index: 2, row: PlyRow { values: vec![PlyValue::Float(9.0), PlyValue::Float(9.0), PlyValue::Float(9.0)] } });
    let mut mid = base.clone();
    let d1 = apply_ply_mutation(&mut mid, &m1);
    let m2 = PlyMutation::RemoveRow(remove_row::RemoveRow { element_name: "vertex".into(), index: 0 });
    let mut after = mid.clone();
    let d2 = apply_ply_mutation(&mut after, &m2);
    let mut merged = d1.diff().clone();
    merged.absorb(d2.diff().clone());
    assert_eq!(merged.apply(&base).expect("valid absorbed diff"), after, "absorb(d1,d2).apply(base) == d2.diff().apply(d1.diff().apply(base))");
    let rows_diff = merged.elements.as_ref().unwrap().modified.iter().find(|m| m.name == "vertex").unwrap().diff.rows.as_ref().unwrap();
    assert_eq!(rows_diff.removed, vec![0], "base index 0 removed");
    assert_eq!(rows_diff.added.len(), 1, "the surviving insert, shifted to final index 1");
    assert_eq!(rows_diff.added[0].index, 1);
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_insert_insert_same_index_both_survive() {
    let base = law_base();
    let m1 = PlyMutation::InsertRow(insert_row::InsertRow { element_name: "vertex".into(), index: 2, row: PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(1.0), PlyValue::Float(1.0)] } });
    let mut mid = base.clone();
    let d1 = apply_ply_mutation(&mut mid, &m1);
    let m2 = PlyMutation::InsertRow(insert_row::InsertRow { element_name: "vertex".into(), index: 2, row: PlyRow { values: vec![PlyValue::Float(2.0), PlyValue::Float(2.0), PlyValue::Float(2.0)] } });
    let mut after = mid.clone();
    let d2 = apply_ply_mutation(&mut after, &m2);
    let mut merged = d1.diff().clone();
    merged.absorb(d2.diff().clone());
    assert_eq!(merged.apply(&base).expect("valid absorbed diff"), after, "both inserts must survive absorb");
    let rows_diff = merged.elements.as_ref().unwrap().modified.iter().find(|m| m.name == "vertex").unwrap().diff.rows.as_ref().unwrap();
    assert_eq!(rows_diff.added.len(), 2, "both inserts survive (fixes the op-slot LWW bug the recipe bans)");
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_add_element_then_set_row_property_patches_into_added() {
    let base = law_base();
    let new_element = PlyElement { name: "material".into(), count: 1, properties: vec![PlyProperty::Scalar { name: "shininess".into(), kind: PlyScalarType::Float }], rows: vec![PlyRow { values: vec![PlyValue::Float(0.1)] }] };
    let m1 = PlyMutation::AddElement(add_element::AddElement { index: 2, element: new_element });
    let mut mid = base.clone();
    let d1 = apply_ply_mutation(&mut mid, &m1);
    let m2 = PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name: "material".into(), row_index: 0, property_name: "shininess".into(), value: PlyValue::Float(0.9) });
    let mut after = mid.clone();
    let d2 = apply_ply_mutation(&mut after, &m2);
    let mut merged = d1.diff().clone();
    merged.absorb(d2.diff().clone());
    assert_eq!(merged.apply(&base).expect("valid absorbed diff"), after);
    let ed = merged.elements.as_ref().unwrap();
    assert!(ed.modified.iter().all(|m| m.name != "material"), "no separate modified entry for the added element");
    let added = ed.added.iter().find(|a| a.element.name == "material").expect("material still in added[]");
    assert_eq!(added.element.rows[0].values[0], PlyValue::Float(0.9), "patched directly into the carried added payload");
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_modify_then_remove_name_keyed() {
    let base = law_base();
    let m1 = PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name: "face".into(), row_index: 0, property_name: "vertex_indices".into(), value: PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2)]) });
    let mut mid = base.clone();
    let d1 = apply_ply_mutation(&mut mid, &m1);
    let m2 = PlyMutation::RemoveElement(remove_element::RemoveElement { name: "face".into() });
    let mut after = mid.clone();
    let d2 = apply_ply_mutation(&mut after, &m2);
    let mut merged = d1.diff().clone();
    merged.absorb(d2.diff().clone());
    assert_eq!(merged.apply(&base).expect("valid absorbed diff"), after);
    let ed = merged.elements.as_ref().unwrap();
    assert!(ed.modified.iter().all(|m| m.name != "face"), "modified-of-removed collapses away");
    assert!(ed.removed.contains(&"face".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_modify_then_remove_index_keyed() {
    let base = law_base();
    let m1 = PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name: "vertex".into(), row_index: 1, property_name: "x".into(), value: PlyValue::Float(5.0) });
    let mut mid = base.clone();
    let d1 = apply_ply_mutation(&mut mid, &m1);
    let m2 = PlyMutation::RemoveRow(remove_row::RemoveRow { element_name: "vertex".into(), index: 1 });
    let mut after = mid.clone();
    let d2 = apply_ply_mutation(&mut after, &m2);
    let mut merged = d1.diff().clone();
    merged.absorb(d2.diff().clone());
    assert_eq!(merged.apply(&base).expect("valid absorbed diff"), after);
    let rows_diff = merged.elements.as_ref().unwrap().modified.iter().find(|m| m.name == "vertex").unwrap().diff.rows.as_ref().unwrap();
    assert!(rows_diff.modified.iter().all(|m| m.index != 1), "modified-of-removed row collapses away");
    assert!(rows_diff.removed.contains(&1));
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_associativity() {
    let base = law_base();
    let m1 = PlyMutation::SetFormat(set_format::SetFormat { format: PlyFormat::BinaryLittleEndian });
    let m2 = PlyMutation::InsertComment(insert_comment::InsertComment { index: 0, comment: "x".into() });
    let m3 = PlyMutation::RemoveElement(remove_element::RemoveElement { name: "face".into() });
    let mut s1 = base.clone();
    let d1 = apply_ply_mutation(&mut s1, &m1);
    let mut s2 = s1.clone();
    let d2 = apply_ply_mutation(&mut s2, &m2);
    let mut s3 = s2.clone();
    let d3 = apply_ply_mutation(&mut s3, &m3);

    let mut left = d1.diff().clone();
    left.absorb(d2.diff().clone());
    left.absorb(d3.diff().clone());

    let mut right_inner = d2.diff().clone();
    right_inner.absorb(d3.diff().clone());
    let mut right = d1.diff().clone();
    right.absorb(right_inner);

    assert_eq!(left.apply(&base).expect("valid left diff"), right.apply(&base).expect("valid right diff"), "associativity: (d1∘d2)∘d3 == d1∘(d2∘d3) applied");
    assert_eq!(left.apply(&base).expect("valid associated diff"), s3, "both associations must equal sequential application");
}
//#endregion

//#region 🔖️BetweenRoundtripLaw
/// 4️⃣ `between_roundtrip_law`: `between(a,b).apply(a) == b` on synthetic fixtures.
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = law_base();
    let mut b = a.clone();
    b.format = PlyFormat::BinaryBigEndian;
    b.comments = vec!["hello".into()];
    b.elements[0].rows[0].values[0] = PlyValue::Float(100.0);
    b.elements.remove(1); // drop "face" entirely
    b.elements.push(PlyElement { name: "edge".into(), count: 0, properties: vec![], rows: vec![] });

    let d = PlyDiff::between(&a, &b);
    assert_eq!(d.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) == b");
    let back = PlyDiff::between(&b, &a);
    assert_eq!(back.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) == a");
}
//#endregion

//#region 🔖️CodecRetentionLaw
/// 5️⃣ `codec_retention_law`: decode→encode is byte-preserving for ascii/binary fixtures.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    for format in [PlyFormat::Ascii, PlyFormat::BinaryLittleEndian, PlyFormat::BinaryBigEndian] {
        let snap = tetrahedron();
        let encoded = encode_ply_with_format(&snap, format).expect("encode");
        let decoded = decode_ply(&encoded).expect("decode");
        let re_encoded = encode_ply_with_format(&decoded, format).expect("re-encode");
        assert_eq!(encoded, re_encoded, "decode→encode must be byte-preserving for {format:?}");
    }
}
//#endregion

//#region 🔖️FieldSweep
/// 6️⃣ `field_sweep`: `sweep_a`/`sweep_b` differ in EVERY mutable field.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> PlySnapshot {
    PlySnapshot {
        schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::Ascii,
        comments: vec!["a".into()],
        elements: vec![
            PlyElement {
                name: "vertex".into(),
                count: 2,
                properties: vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }, PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float }],
                rows: vec![PlyRow { values: vec![PlyValue::Float(0.0), PlyValue::Float(0.0)] }, PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(1.0)] }],
            },
            PlyElement {
                name: "face".into(),
                count: 1,
                properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }],
                rows: vec![PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2)])] }],
            },
        ],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> PlySnapshot {
    PlySnapshot {
        schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::BinaryLittleEndian,
        comments: vec!["a".into(), "b".into()],
        elements: vec![
            PlyElement {
                name: "vertex".into(),
                count: 1,
                properties: vec![PlyProperty::Scalar { name: "nx".into(), kind: PlyScalarType::Double }, PlyProperty::Scalar { name: "ny".into(), kind: PlyScalarType::Double }],
                rows: vec![PlyRow { values: vec![PlyValue::Double(9.0), PlyValue::Double(9.0)] }],
            },
            PlyElement { name: "edge".into(), count: 1, properties: vec![PlyProperty::Scalar { name: "weight".into(), kind: PlyScalarType::Double }], rows: vec![PlyRow { values: vec![PlyValue::Double(3.5)] }] },
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn field_sweep_covers_every_mutable_field() {
    let a = sweep_a();
    let b = sweep_b();

    let ab = PlyDiff::between(&a, &b);
    assert!(ab.format.is_some(), "format field must be exercised");
    assert!(ab.comments.is_some(), "comments field must be exercised");
    let ab_elements = ab.elements.as_ref().expect("elements diff must be present");
    assert!(!ab_elements.removed.is_empty(), "sweep must exercise a removed element (face)");
    assert!(!ab_elements.added.is_empty(), "sweep must exercise an added element (edge)");
    assert!(!ab_elements.modified.is_empty(), "sweep must exercise a modified element (vertex)");
    let vertex_mod = ab_elements.modified.iter().find(|m| m.name == "vertex").expect("vertex modified");
    assert!(vertex_mod.diff.properties.is_some(), "properties weak-replace must be exercised");
    assert!(vertex_mod.diff.rows.is_some(), "rows triple must be exercised (schema-change scope cut path)");
    assert_eq!(ab.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) == b");

    let ba = PlyDiff::between(&b, &a);
    let ba_elements = ba.elements.as_ref().expect("reverse elements diff must be present");
    assert!(!ba_elements.removed.is_empty(), "reverse direction: edge removed");
    assert!(!ba_elements.added.is_empty(), "reverse direction: face added");
    assert_eq!(ba.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) == a");

    assert!(PlyDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    assert!(PlyDiff::between(&b, &b).is_empty(), "between(b,b) must be empty");
}

/// 🧪 Direct row-level triple sweep (not routed through the schema-change scope cut).
#[semio_framework_async_macros::async_test]
async fn field_sweep_row_triple_both_directions() {
    let common_props = vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Int }];
    let a = PlySnapshot {
        schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::Ascii,
        comments: vec![],
        elements: vec![PlyElement { name: "point".into(), count: 2, properties: common_props.clone(), rows: vec![PlyRow { values: vec![PlyValue::Int(1)] }, PlyRow { values: vec![PlyValue::Int(2)] }] }],
    };
    let b = PlySnapshot {
        schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::Ascii,
        comments: vec![],
        elements: vec![PlyElement { name: "point".into(), count: 3, properties: common_props, rows: vec![PlyRow { values: vec![PlyValue::Int(99)] }, PlyRow { values: vec![PlyValue::Int(2)] }, PlyRow { values: vec![PlyValue::Int(3)] }] }],
    };
    let ab = PlyDiff::between(&a, &b);
    let ab_rows = ab.elements.as_ref().unwrap().modified[0].diff.rows.as_ref().expect("rows diff");
    assert!(!ab_rows.modified.is_empty(), "row 0 modified (1 -> 99)");
    assert!(!ab_rows.added.is_empty(), "row 2 added (b longer)");
    assert!(ab_rows.removed.is_empty(), "b is longer, no removed tail in this direction");
    assert_eq!(ab.apply(&a).expect("valid forward diff"), b);

    let ba = PlyDiff::between(&b, &a);
    let ba_rows = ba.elements.as_ref().unwrap().modified[0].diff.rows.as_ref().expect("rows diff");
    assert!(!ba_rows.removed.is_empty(), "a is shorter, removed tail in this direction");
    assert_eq!(ba.apply(&b).expect("valid backward diff"), a);
}
//#endregion

//#region 🔖️ConformanceLaws
/// 🧪️ Per-artifact conformance laws — grammar/protocol parseability, `Recognizer` against
/// real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
/// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip.
mod conformance_laws {
    use super::*;
    use crate::schema::{diff, mutations, snapshot};
    use protocol::{DiffCodec, OpBinary, OpText};

    #[semio_framework_async_macros::async_test]
    async fn committed_facet_files_parse() {
        for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
            let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
            assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
        }
        for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
            dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        let text = store::ArtifactDsl::print_dsl(&demo_ply_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
    }

    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for mutation in mutations::demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for d in diff::demo_diff_cases() {
            let printed = d.print_diff();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let demo = demo_ply_snapshot();
        let packed = store::ArtifactPack::encode_pack(&demo);
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack, ascii) failed @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, inner.len(), "pack walk (ascii) did not consume every byte");

        for format in [PlyFormat::BinaryLittleEndian, PlyFormat::BinaryBigEndian] {
            let raw = encode_ply_with_format(&demo, format).expect("encode binary variant");
            let trace = dsl::walk_protocol(&pack_spec, &raw).unwrap_or_else(|e| panic!("walk_protocol(pack, {format:?}) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, raw.len(), "pack walk ({format:?}) did not consume every byte");
        }

        let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        for mutation in mutations::demo_mutation_cases() {
            let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
            let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
        }

        let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        for d in diff::demo_diff_cases() {
            let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
            let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = demo_ply_snapshot();

        let parsed = <PlySnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_ply_snapshot()");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_ply_snapshot()) drifted from the shipped .dsl.semio fixture");

        let decoded = <PlySnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_ply_snapshot()");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_ply_snapshot()) drifted from the shipped .pack.semio fixture");
    }
}
//#endregion 🔖️ConformanceLaws

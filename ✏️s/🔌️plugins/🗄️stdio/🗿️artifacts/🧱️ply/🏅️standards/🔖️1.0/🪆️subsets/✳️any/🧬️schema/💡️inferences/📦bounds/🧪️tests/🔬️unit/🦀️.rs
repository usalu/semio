use super::*;
use crate::schema::snapshot::{PlyElement, PlyFormat, PlyRow};
use crate::STDIO_PLY_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn vertex_element(rows: Vec<[f64; 3]>) -> PlyElement {
    PlyElement {
        name: "vertex".into(),
        count: rows.len(),
        properties: vec![
            PlyProperty::Scalar { name: "x".into(), kind: crate::schema::snapshot::PlyScalarType::Float },
            PlyProperty::Scalar { name: "y".into(), kind: crate::schema::snapshot::PlyScalarType::Float },
            PlyProperty::Scalar { name: "z".into(), kind: crate::schema::snapshot::PlyScalarType::Float },
        ],
        rows: rows.into_iter().map(|[x, y, z]| PlyRow { values: vec![PlyValue::Double(x), PlyValue::Double(y), PlyValue::Double(z)] }).collect(),
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn face_element(face_count: usize) -> PlyElement {
    PlyElement {
        name: "face".into(),
        count: face_count,
        properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: crate::schema::snapshot::PlyScalarType::UChar, value_kind: crate::schema::snapshot::PlyScalarType::Int }],
        rows: (0..face_count).map(|_| PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2)])] }).collect(),
    }
}

#[semio_framework_async_macros::async_test]
async fn bounds_matches_hand_built_element_extent() {
    let snapshot = PlySnapshot { schema: STDIO_PLY_DOCUMENT_SCHEMA.into(), format: PlyFormat::Ascii, comments: Vec::new(), elements: vec![vertex_element(vec![[-1.0, 0.0, 2.0], [3.0, 5.0, -2.0], [0.0, 1.0, 1.0]]), face_element(2)] };
    let bounds = compute_ply_bounds(&snapshot);
    assert_eq!(bounds.min, [-1.0, 0.0, -2.0]);
    assert_eq!(bounds.max, [3.0, 5.0, 2.0]);
    assert_eq!(bounds.vertex_count, 3);
    assert_eq!(bounds.face_count, 2);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = PlySnapshot { schema: STDIO_PLY_DOCUMENT_SCHEMA.into(), format: PlyFormat::Ascii, comments: Vec::new(), elements: vec![vertex_element(vec![[1.0, 1.0, 1.0]])] };
    assert_eq!(compute_ply_bounds(&snapshot), compute_ply_bounds(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_ply_bounds(&PlySnapshot::default()), PlyBounds::default());
}

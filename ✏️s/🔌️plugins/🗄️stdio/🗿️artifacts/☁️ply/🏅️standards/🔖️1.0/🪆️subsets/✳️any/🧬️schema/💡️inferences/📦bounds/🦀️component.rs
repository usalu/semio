//! 📦 `bounds` — the ply snapshot's vertex-element bounding box plus vertex/face row counts.
//! PLY's real convention (its own `📸️snapshot` doc comment) is that an element literally named
//! `"vertex"` carries `x`/`y`/`z` scalar properties, and one named `"face"` carries face data —
//! nothing in the model hardcodes this, so this fold locates those properties/elements by name
//! generically: the `"vertex"` element's own `properties` list is searched by name for `"x"`/
//! `"y"`/`"z"`, and every row's cell at those indices is converted from whichever numeric
//! [`PlyValue`] variant appears to `f64` via [`ply_value_as_f64`]. `faceCount` is the row count of
//! an element literally named `"face"`, if present — also generic, no assumption it exists. A
//! pure whole-snapshot fold — no `InferredField` needed.

use crate::artifacts::ply::schema::snapshot::{PlyElement, PlyProperty, PlyValue};
use crate::artifacts::ply::PlySnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ Ply vertex-element bounding box plus vertex/face row counts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlyBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub vertex_count: u32,
    pub face_count: u32,
}

/// 🔣️ Converts whichever numeric [`PlyValue`] variant appears to `f64`; `List` cells (e.g. a
/// face's vertex-index list) have no scalar meaning and are honestly `None`.
fn ply_value_as_f64(value: &PlyValue) -> Option<f64> {
    match value {
        PlyValue::Char(v) => Some(*v as f64),
        PlyValue::UChar(v) => Some(*v as f64),
        PlyValue::Short(v) => Some(*v as f64),
        PlyValue::UShort(v) => Some(*v as f64),
        PlyValue::Int(v) => Some(*v as f64),
        PlyValue::UInt(v) => Some(*v as f64),
        PlyValue::Float(v) => Some(*v as f64),
        PlyValue::Double(v) => Some(*v),
        PlyValue::List(_) => None,
    }
}

fn expand(min: &mut [f64; 3], max: &mut [f64; 3], seen: &mut bool, p: [f64; 3]) {
    if !*seen {
        *min = p;
        *max = p;
        *seen = true;
        return;
    }
    for i in 0..3 {
        min[i] = min[i].min(p[i]);
        max[i] = max[i].max(p[i]);
    }
}

fn property_index(properties: &[PlyProperty], name: &str) -> Option<usize> {
    properties.iter().position(|p| p.name() == name)
}

/// 📦️ Computes [`PlyBounds`] over the `"vertex"` element's own `x`/`y`/`z` property columns, plus
/// the `"face"` element's own row count — see module doc comment for the by-name lookup rule.
pub fn compute_ply_bounds(snapshot: &PlySnapshot) -> PlyBounds {
    let mut min = [0.0f64; 3];
    let mut max = [0.0f64; 3];
    let mut seen = false;
    let mut vertex_count = 0u32;
    let mut face_count = 0u32;

    for element in &snapshot.elements {
        if element.name == "vertex" {
            vertex_count += element.rows.len() as u32;
            let coords: Option<(usize, usize, usize)> = property_index(&element.properties, "x")
                .zip(property_index(&element.properties, "y"))
                .zip(property_index(&element.properties, "z"))
                .map(|((ix, iy), iz)| (ix, iy, iz));
            if let Some((ix, iy, iz)) = coords {
                for row in &element.rows {
                    let x = row.values.get(ix).and_then(ply_value_as_f64);
                    let y = row.values.get(iy).and_then(ply_value_as_f64);
                    let z = row.values.get(iz).and_then(ply_value_as_f64);
                    if let (Some(x), Some(y), Some(z)) = (x, y, z) {
                        expand(&mut min, &mut max, &mut seen, [x, y, z]);
                    }
                }
            }
        } else if element.name == "face" {
            face_count += element.rows.len() as u32;
        }
    }

    PlyBounds { min, max, vertex_count, face_count }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::ply::schema::snapshot::{PlyFormat, PlyRow};
    use crate::artifacts::ply::STDIO_PLY_DOCUMENT_SCHEMA;

    fn vertex_element(rows: Vec<[f64; 3]>) -> PlyElement {
        PlyElement {
            name: "vertex".into(),
            count: rows.len(),
            properties: vec![
                PlyProperty::Scalar { name: "x".into(), kind: crate::artifacts::ply::schema::snapshot::PlyScalarType::Float },
                PlyProperty::Scalar { name: "y".into(), kind: crate::artifacts::ply::schema::snapshot::PlyScalarType::Float },
                PlyProperty::Scalar { name: "z".into(), kind: crate::artifacts::ply::schema::snapshot::PlyScalarType::Float },
            ],
            rows: rows.into_iter().map(|[x, y, z]| PlyRow { values: vec![PlyValue::Double(x), PlyValue::Double(y), PlyValue::Double(z)] }).collect(),
        }
    }

    fn face_element(face_count: usize) -> PlyElement {
        PlyElement {
            name: "face".into(),
            count: face_count,
            properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: crate::artifacts::ply::schema::snapshot::PlyScalarType::UChar, value_kind: crate::artifacts::ply::schema::snapshot::PlyScalarType::Int }],
            rows: (0..face_count).map(|_| PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2)])] }).collect(),
        }
    }

    #[test]
    fn bounds_matches_hand_built_element_extent() {
        let snapshot = PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::Ascii,
            comments: Vec::new(),
            elements: vec![
                vertex_element(vec![[-1.0, 0.0, 2.0], [3.0, 5.0, -2.0], [0.0, 1.0, 1.0]]),
                face_element(2),
            ],
        };
        let bounds = compute_ply_bounds(&snapshot);
        assert_eq!(bounds.min, [-1.0, 0.0, -2.0]);
        assert_eq!(bounds.max, [3.0, 5.0, 2.0]);
        assert_eq!(bounds.vertex_count, 3);
        assert_eq!(bounds.face_count, 2);
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::Ascii,
            comments: Vec::new(),
            elements: vec![vertex_element(vec![[1.0, 1.0, 1.0]])],
        };
        assert_eq!(compute_ply_bounds(&snapshot), compute_ply_bounds(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_ply_bounds(&PlySnapshot::default()), PlyBounds::default());
    }
}
//#endregion 🧪️Tests

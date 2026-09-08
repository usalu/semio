//! 📦 `bounds` — the ply snapshot's vertex-element bounding box plus vertex/face row counts.
//! PLY's real convention (its own `📸️snapshot` doc comment) is that an element literally named
//! `"vertex"` carries `x`/`y`/`z` scalar properties, and one named `"face"` carries face data —
//! nothing in the model hardcodes this, so this fold locates those properties/elements by name
//! generically: the `"vertex"` element's own `properties` list is searched by name for `"x"`/
//! `"y"`/`"z"`, and every row's cell at those indices is converted from whichever numeric
//! [`PlyValue`] variant appears to `f64` via [`ply_value_as_f64`]. `faceCount` is the row count of
//! an element literally named `"face"`, if present — also generic, no assumption it exists. A
//! pure whole-snapshot fold — no `InferredField` needed.

use crate::schema::snapshot::{PlyProperty, PlyValue};
use crate::PlySnapshot;

//#region 🔖️Bounds
/// 📦️ Ply vertex-element bounding box plus vertex/face row counts.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PlyBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub vertex_count: u32,
    pub face_count: u32,
}

/// 🔣️ Converts whichever numeric [`PlyValue`] variant appears to `f64`; `List` cells (e.g. a
/// face's vertex-index list) have no scalar meaning and are honestly `None`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn property_index(properties: &[PlyProperty], name: &str) -> Option<usize> {
    properties.iter().position(|p| p.name() == name)
}

/// 📦️ Computes [`PlyBounds`] over the `"vertex"` element's own `x`/`y`/`z` property columns, plus
/// the `"face"` element's own row count — see module doc comment for the by-name lookup rule.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_ply_bounds(snapshot: &PlySnapshot) -> PlyBounds {
    let mut min = [0.0f64; 3];
    let mut max = [0.0f64; 3];
    let mut seen = false;
    let mut vertex_count = 0u32;
    let mut face_count = 0u32;

    for element in &snapshot.elements {
        if element.name == "vertex" {
            vertex_count += element.rows.len() as u32;
            let coords: Option<(usize, usize, usize)> = property_index(&element.properties, "x").zip(property_index(&element.properties, "y")).zip(property_index(&element.properties, "z")).map(|((ix, iy), iz)| (ix, iy, iz));
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

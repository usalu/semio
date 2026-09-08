//! 📦 `bounds` — one named inference: the spatial bounding box over every `v` position record's
//! `x`/`y`/`z` (the optional homogeneous 4th component `w` is deliberately excluded — it is not a
//! spatial extent). `vertexCount`/`faceCount`/`groupCount` are direct tallies of
//! `vertices`/`faces`/`groups` (no fold needed). A pure whole-snapshot scalar (one min/max fold) —
//! no `InferredField` needed.

use crate::schema::snapshot::ObjSnapshot;

//#region 🔖️Bounds
/// 📦️ Obj's vertex-derived spatial bounding box.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub vertex_count: u32,
    pub face_count: u32,
    pub group_count: u32,
}

/// 🩹 Hand-rolled: an empty vertex set has no honest min/max — `[0,0,0]`/`[0,0,0]` matches what
/// `compute` returns for zero vertices (the fold's identity value), keeping the inference-default
/// law correct.
impl Default for ObjBounds {
    fn default() -> Self {
        Self { min: [0.0, 0.0, 0.0], max: [0.0, 0.0, 0.0], vertex_count: 0, face_count: 0, group_count: 0 }
    }
}

/// 📦️ Computes [`ObjBounds`] over every `vertices[].{x,y,z}` (ignoring the optional homogeneous
/// `w`) plus direct `faces`/`groups` tallies.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_obj_bounds(snapshot: &ObjSnapshot) -> ObjBounds {
    let mut min = [0.0f64; 3];
    let mut max = [0.0f64; 3];
    let mut seen = false;

    for vertex in &snapshot.vertices {
        let p = [vertex.x, vertex.y, vertex.z];
        for i in 0..3 {
            if seen {
                min[i] = min[i].min(p[i]);
                max[i] = max[i].max(p[i]);
            } else {
                min[i] = p[i];
                max[i] = p[i];
            }
        }
        seen = true;
    }

    ObjBounds { min, max, vertex_count: snapshot.vertices.len() as u32, face_count: snapshot.faces.len() as u32, group_count: snapshot.groups.len() as u32 }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

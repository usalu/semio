//! 📦 `bounds` — the las snapshot's header-declared bounding box and point count. LAS's own spec
//! (§2.3 Public Header Block) puts the authoritative min/max extent and point-record count
//! directly in the header — this is an honest read of `snapshot.header`'s own fields, never a
//! recompute over `points` (which would silently diverge from what real-world LAS writers
//! declare, several of which legitimately carry a header bbox looser than the actual point
//! extent). A pure whole-snapshot scalar read — no `InferredField` needed.

use crate::LasSnapshot;

//#region 🔖️Bounds
/// 📦️ Las header-declared bounding box and point count.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LasBounds {
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
    pub point_count: u32,
}

/// 📦️ Computes [`LasBounds`] as a direct read of `snapshot.header`'s own declared bounds and
/// point-record count — no fold over `points`, matching LAS's own spec (the header carries the
/// authoritative bbox, not a derived one).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_las_bounds(snapshot: &LasSnapshot) -> LasBounds {
    let header = &snapshot.header;
    LasBounds { min_x: header.min_x, min_y: header.min_y, min_z: header.min_z, max_x: header.max_x, max_y: header.max_y, max_z: header.max_z, point_count: header.number_of_point_records }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//! 📦 `bounds` — the las snapshot's header-declared bounding box and point count. LAS's own spec
//! (§2.3 Public Header Block) puts the authoritative min/max extent and point-record count
//! directly in the header — this is an honest read of `snapshot.header`'s own fields, never a
//! recompute over `points` (which would silently diverge from what real-world LAS writers
//! declare, several of which legitimately carry a header bbox looser than the actual point
//! extent). A pure whole-snapshot scalar read — no `InferredField` needed.

use crate::artifacts::las::LasSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ Las header-declared bounding box and point count.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
pub fn compute_las_bounds(snapshot: &LasSnapshot) -> LasBounds {
    let header = &snapshot.header;
    LasBounds { min_x: header.min_x, min_y: header.min_y, min_z: header.min_z, max_x: header.max_x, max_y: header.max_y, max_z: header.max_z, point_count: header.number_of_point_records }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::las::schema::snapshot::{LasHeader, LasPoint, LasVlr};
    use crate::artifacts::las::STDIO_LAS_DOCUMENT_SCHEMA;

    fn snapshot_with(header: LasHeader, points: Vec<LasPoint>) -> LasSnapshot {
        LasSnapshot { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), header, vlrs: Vec::<LasVlr>::new(), points }
    }

    #[test]
    fn bounds_matches_hand_built_header_extent() {
        let header = LasHeader { min_x: -12.5, min_y: 0.0, min_z: -3.25, max_x: 100.0, max_y: 88.75, max_z: 15.0, number_of_point_records: 3, ..LasHeader::default() };
        let snapshot = snapshot_with(header, vec![LasPoint::default(), LasPoint::default(), LasPoint::default()]);
        let bounds = compute_las_bounds(&snapshot);
        assert_eq!(bounds.min_x, -12.5);
        assert_eq!(bounds.min_y, 0.0);
        assert_eq!(bounds.min_z, -3.25);
        assert_eq!(bounds.max_x, 100.0);
        assert_eq!(bounds.max_y, 88.75);
        assert_eq!(bounds.max_z, 15.0);
        assert_eq!(bounds.point_count, 3);
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = snapshot_with(LasHeader { number_of_point_records: 1, ..LasHeader::default() }, vec![LasPoint::default()]);
        assert_eq!(compute_las_bounds(&snapshot), compute_las_bounds(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_las_bounds(&LasSnapshot::default()), LasBounds::default());
    }
}
//#endregion 🧪️Tests

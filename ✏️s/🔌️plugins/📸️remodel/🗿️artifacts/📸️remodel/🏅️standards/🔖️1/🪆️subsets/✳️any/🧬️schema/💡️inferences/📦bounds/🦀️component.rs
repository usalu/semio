//! 📦 `bounds` — one named inference: the reconstructed mesh's axis-aligned bounding box plus
//! vertex/face counts, read straight off `results.mesh.mesh` (`MeshData`, the only field on this
//! snapshot with real 3D geometry). Whole-snapshot scalar, not per-entity, so this leaf holds a
//! plain pure function rather than an `InferredField` chain — the family root's
//! `impl protocol::Inference<RemodelSnapshot>` calls it directly.

use crate::artifacts::remodel::RemodelSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ Axis-aligned bounding box, `[x, y, z]` corners.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemodelBoundingBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

/// 📦️ Reconstructed-mesh bounds summary.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemodelBounds {
    pub bounding_box: RemodelBoundingBox,
    pub vertex_count: u32,
    pub face_count: u32,
}

/// 📦️ `MeshData::aabb()` returns `[INFINITY; 3]`/`[NEG_INFINITY; 3]` for an empty mesh (no
/// vertices to fold over); normalized here to a zero box so an empty/default snapshot infers a
/// clean, serializable `RemodelBounds::default()` rather than propagating infinities.
pub fn compute_remodel_bounds(snapshot: &RemodelSnapshot) -> RemodelBounds {
    let mesh = &snapshot.results.mesh.mesh;
    if mesh.positions.is_empty() {
        return RemodelBounds { bounding_box: RemodelBoundingBox::default(), vertex_count: 0, face_count: 0 };
    }
    let (min, max) = mesh.aabb();
    RemodelBounds {
        bounding_box: RemodelBoundingBox { min: [min[0] as f64, min[1] as f64, min[2] as f64], max: [max[0] as f64, max[1] as f64, max[2] as f64] },
        vertex_count: mesh.vertex_count() as u32,
        face_count: mesh.triangle_count() as u32,
    }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use semio_framework::MeshData;

    #[test]
    fn empty_mesh_yields_a_zero_bounds() {
        let bounds = compute_remodel_bounds(&RemodelSnapshot::default());
        assert_eq!(bounds, RemodelBounds::default());
    }

    #[test]
    fn a_single_triangle_bounds_and_counts_exactly() {
        let mut snapshot = RemodelSnapshot::default();
        snapshot.results.mesh.mesh = MeshData { positions: vec![-1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.0, 0.0], indices: vec![0, 1, 2], ..MeshData::default() };
        let bounds = compute_remodel_bounds(&snapshot);
        assert_eq!(bounds.bounding_box.min, [-1.0, 0.0, 0.0]);
        assert_eq!(bounds.bounding_box.max, [1.0, 2.0, 0.0]);
        assert_eq!(bounds.vertex_count, 3);
        assert_eq!(bounds.face_count, 1);
    }

    #[test]
    fn bounds_is_deterministic() {
        let mut snapshot = RemodelSnapshot::default();
        snapshot.results.mesh.mesh = MeshData { positions: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], ..MeshData::default() };
        assert_eq!(compute_remodel_bounds(&snapshot), compute_remodel_bounds(&snapshot));
    }
}
//#endregion 🧪️Tests

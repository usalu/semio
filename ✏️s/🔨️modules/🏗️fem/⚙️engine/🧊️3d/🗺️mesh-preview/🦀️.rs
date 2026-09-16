//! 🗺️ FEM 3D module engine — cheap solid mesh preview + nodal-averaged stress (pure FE algorithm,
//! moved out of the artifact tree).

use crate::analyses;
use crate::fem3d_engine::{meshing, Fem3dError};
use crate::Fem3dSnapshot;
use std::collections::HashMap;

/// 🗺️ One meshed solid's preview geometry — the full volume mesh (points/tets) plus its outer boundary
/// triangulation for surface rendering, with the DOCUMENT-WIDE node ids the solver addresses the same
/// points by. The one `meshing::SolidMesh` type, re-exported so a renderer and the solver never disagree.
pub use meshing::SolidMesh;

/// 🗺️ Triangulates+extrudes+tet-splits every `FemSolid` in `doc` through the very `meshing::mesh_solids`
/// call `resolve_geometry` makes (same deterministic `crate::mesh` calls, same merged node ids, so tet
/// indices line up with `"{solid_id}_c{i}"` element ids) and returns just the geometry plus its outer
/// surface — cheap enough for every render, no `crate::model::Element` is built.
pub fn fem3d_mesh_preview(doc: &Fem3dSnapshot) -> Result<Vec<SolidMesh>, Fem3dError> {
    meshing::mesh_solids(doc).map(|(_, meshes)| meshes)
}

/// 🎨️ Nodal-averaged von Mises stress for `case_id`'s solved result, keyed by node id — the
/// document-layer bridge to `crate::analyses::nodal_averaged_scalar`, mirroring `fem_2d`'s
/// `fem2d_nodal_von_mises`, feeding `fem-plugin`'s solid stress contour rendering.
pub fn fem3d_nodal_von_mises(doc: &Fem3dSnapshot, case_id: &str) -> Result<HashMap<String, f64>, Fem3dError> {
    let results = super::fem3d_solve_all(doc)?;
    let result = results.get(case_id).ok_or_else(|| Fem3dError::LoadCaseNotFound(case_id.to_string()))?;
    fem3d_nodal_von_mises_of(doc, result)
}

/// 🎨️ Nodal-averaged von Mises stress of ONE already-solved result — the cache-friendly half of
/// [`fem3d_nodal_von_mises`], for a caller that keeps its solved fields across renders.
pub fn fem3d_nodal_von_mises_of(doc: &Fem3dSnapshot, result: &crate::model::StaticResult) -> Result<HashMap<String, f64>, Fem3dError> {
    let (nodes, elements, _solids, supports) = meshing::resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    Ok(analyses::nodal_averaged_scalar(&model, result, analyses::StressScalar::VonMises))
}

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

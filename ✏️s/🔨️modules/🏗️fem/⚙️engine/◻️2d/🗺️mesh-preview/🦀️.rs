//! 🗺️ FEM 2D module engine — cheap mesh preview + nodal stress bridge (pure FE algorithm, moved out of
//! the artifact tree).

use crate::Fem2dSnapshot;
use crate::fem2d_engine::meshing::build_nodes_and_elements;
use crate::fem2d_engine::Fem2dError;
use crate::model::Support;
use std::collections::HashMap;

/// 🗺️ One meshed region's cheap preview geometry — mesh points plus triangle vertex indices, WITHOUT
/// building any `crate::model::Element`. Used purely for a mesh-edge preview overlay in the model window
/// and to correlate `fem2d_solve_all`'s `Tri3Cst` results (ids `"{region_id}_t{tri_index}"`, see
/// `crate::fem2d_engine::meshing::build_nodes_and_elements`) back to screen-space triangles
/// for contour rendering.
pub struct RegionMesh {
    pub region_id: String,
    pub points: Vec<[f64; 2]>,
    pub tris: Vec<[u32; 3]>,
    /// 🪪️ Per-point node id, SAME coincident-node resolution `build_nodes_and_elements` uses (existing
    /// doc node within `1e-9` reused, else synthesized `"{region_id}_m{point_index}"`) — lets a caller
    /// (the results window's nodal-averaged contour rendering) map `fem2d_nodal_von_mises`'s node-keyed
    /// map straight onto this mesh's triangles.
    pub node_ids: Vec<String>,
}

/// 🗺️ Triangulates every `FemRegion` in `doc` (same `crate::mesh::triangulate` call as
/// `build_nodes_and_elements`, so triangle indices/ids line up deterministically with solved results)
/// and returns just the geometry — cheap enough to call on every render.
pub fn fem2d_mesh_preview(doc: &Fem2dSnapshot) -> Result<Vec<RegionMesh>, Fem2dError> {
    let mut out = Vec::with_capacity(doc.regions.len());
    for region in &doc.regions {
        let domain = crate::mesh::PlanarDomain { outer: region.outline.clone(), holes: region.holes.clone() };
        let opts = crate::mesh::MeshOpts { max_edge: region.mesh_size, min_angle_deg: 20.0 };
        let tri_mesh = crate::mesh::triangulate(&domain, &opts).map_err(|e| Fem2dError::MeshFailed { region_id: region.id.clone(), reason: e.to_string() })?;
        let node_ids = tri_mesh
            .points
            .iter()
            .enumerate()
            .map(|(point_index, p)| match doc.nodes.iter().find(|n| (n.x - p[0]).abs() < 1e-9 && (n.y - p[1]).abs() < 1e-9) {
                Some(n) => n.id.clone(),
                None => format!("{}_m{}", region.id, point_index),
            })
            .collect();
        out.push(RegionMesh { region_id: region.id.clone(), points: tri_mesh.points, tris: tri_mesh.tris, node_ids });
    }
    Ok(out)
}

/// 🎨️ Nodal-averaged von Mises stress for `case_id`'s solved result (via `fem2d_solve_all`, so `case_id`
/// may name either a `FemLoadCase` or a `FemCombination`), keyed by node id — the document-layer bridge
/// to `crate::analyses::nodal_averaged_scalar`, feeding the results window's banded contour
/// rendering.
pub fn fem2d_nodal_von_mises(doc: &Fem2dSnapshot, case_id: &str) -> Result<HashMap<String, f64>, Fem2dError> {
    let (nodes, elements, _regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();
    let model = crate::analyses::AnalysisModel { nodes, elements, supports };
    let results = crate::fem2d_engine::fem2d_solve_all(doc)?;
    let result = results.get(case_id).ok_or_else(|| Fem2dError::LoadCaseNotFound(case_id.to_string()))?;
    Ok(crate::analyses::nodal_averaged_scalar(&model, result, crate::analyses::StressScalar::VonMises))
}

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

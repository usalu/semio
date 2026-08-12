//! 🗺️ FEM 2D module engine — cheap mesh preview + nodal stress bridge (pure FE algorithm, moved out of
//! the artifact tree).

use crate::fem2d_engine::meshing::build_nodes_and_elements;
use crate::fem2d_engine::Fem2dError;
use crate::artifacts::fem2d::Fem2dSnapshot;
use crate::model::Support;
use std::collections::HashMap;

/// 🗺️ One meshed region's cheap preview geometry — mesh points plus triangle vertex indices, WITHOUT
/// building any `crate::model::Element`. Used purely for a mesh-edge preview overlay in the model window
/// and to correlate `fem2d_solve_all`'s `Tri3Cst` results (ids `"{region_id}_t{tri_index}"`, see
/// `crate::artifacts::fem2d::engine::meshing::build_nodes_and_elements`) back to screen-space triangles
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
    let results = crate::artifacts::fem2d::engine::fem2d_solve_all(doc)?;
    let result = results.get(case_id).ok_or_else(|| Fem2dError::LoadCaseNotFound(case_id.to_string()))?;
    Ok(crate::analyses::nodal_averaged_scalar(&model, result, crate::analyses::StressScalar::VonMises))
}

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::fem2d::{FemAnalysisSettings, FemDof, FemLoad, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSupport};

    /// 🟩️ A 4x2m rectangular region (steel, 0.02m thick, 1m mesh) whose 4 corners are pre-placed as
    /// document nodes.
    fn rectangle_region_doc() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![FemNode { id: "c0".into(), x: 0.0, y: 0.0 }, FemNode { id: "c1".into(), x: 4.0, y: 0.0 }, FemNode { id: "c2".into(), x: 4.0, y: 2.0 }, FemNode { id: "c3".into(), x: 0.0, y: 2.0 }],
            elements: vec![],
            regions: vec![FemRegion { id: "r1".into(), name: "slab".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 1.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "c0".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "c1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "self weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    #[test]
    fn fem2d_mesh_preview_returns_region_triangles() {
        let doc = rectangle_region_doc();
        let meshes = fem2d_mesh_preview(&doc).expect("mesh preview succeeds");
        assert_eq!(meshes.len(), 1);
        assert_eq!(meshes[0].region_id, "r1");
        assert!(!meshes[0].tris.is_empty(), "expected at least one triangle");
        assert!(!meshes[0].points.is_empty(), "expected mesh points");
        assert_eq!(meshes[0].node_ids.len(), meshes[0].points.len(), "one node id per mesh point");
        // `rectangle_region_doc`'s 4 outline corners coincide with existing doc nodes `c0..c3`, so those
        // 4 mesh points must resolve to the doc's own node ids, not synthesized `r1_m*` ids.
        for corner_id in ["c0", "c1", "c2", "c3"] {
            assert!(meshes[0].node_ids.contains(&corner_id.to_string()), "expected corner {corner_id} to be reused, got {:?}", meshes[0].node_ids);
        }
    }

    /// 🎨️ `fem2d_nodal_von_mises` returns one value per mesh node, and a uniform area-pressure load
    /// (biaxial-ish but still smoothly varying membrane stress) produces FINITE values at every node —
    /// not a tight numeric benchmark (the region isn't a pure patch-test field), just a wiring check that
    /// the document-bridge correctly plumbs `crate::analyses::nodal_averaged_scalar`.
    #[test]
    fn fem2d_nodal_von_mises_returns_one_value_per_mesh_node() {
        let mut doc = rectangle_region_doc();
        doc.load_cases = vec![FemLoadCase { id: "pressure".into(), name: "pressure".into(), loads: vec![FemLoad::Area { id: "a1".into(), region_id: "r1".into(), pressure: 5000.0 }], self_weight: false }];
        let averaged = fem2d_nodal_von_mises(&doc, "pressure").expect("nodal von mises solves");
        let meshes = fem2d_mesh_preview(&doc).expect("mesh preview succeeds");
        for node_id in &meshes[0].node_ids {
            let v = *averaged.get(node_id).unwrap_or_else(|| panic!("missing averaged value for node {node_id}"));
            assert!(v.is_finite() && v >= 0.0, "node {node_id}: von mises {v} should be finite and non-negative");
        }
    }
}
// #endregion 🧪️Tests

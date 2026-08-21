//! 🗺️ FEM 3D module engine — cheap solid mesh preview + nodal-averaged stress (pure FE algorithm,
//! moved out of the artifact tree).

use crate::analyses;
use crate::artifacts::fem3d::Fem3dSnapshot;
use crate::fem3d_engine::{meshing, Fem3dError};
use std::collections::HashMap;

/// 🗺️ One meshed solid's cheap preview geometry — the full volume mesh (points/tets) plus its outer
/// boundary triangulation (via `crate::mesh::boundary_faces`) for surface rendering, WITHOUT
/// building any `crate::model::Element`. Mirrors `fem_2d::RegionMesh`/`fem2d_mesh_preview`.
pub struct SolidMesh {
    pub solid_id: String,
    pub points: Vec<[f64; 3]>,
    pub tets: Vec<[u32; 4]>,
    pub boundary_tris: Vec<[u32; 3]>,
    pub node_ids: Vec<String>,
}

/// 🗺️ Triangulates+extrudes+tet-splits every `FemSolid` in `doc` (same deterministic `crate::model::mesh`
/// calls as `resolve_geometry`, so tet indices line up with `"{solid_id}_c{i}"` element ids) and returns
/// just the geometry plus its outer surface — cheap enough for every render.
pub async fn fem3d_mesh_preview(doc: &Fem3dSnapshot) -> Result<Vec<SolidMesh>, Fem3dError> {
    let mut out = Vec::with_capacity(doc.solids.len());
    for solid in &doc.solids {
        let domain = crate::mesh::PlanarDomain { outer: solid.outline.clone(), holes: solid.holes.clone() };
        let opts = crate::mesh::MeshOpts { max_edge: solid.mesh_size, min_angle_deg: 20.0 };
        let tri_mesh = crate::mesh::triangulate(&domain, &opts).map_err(|e| Fem3dError::MeshFailed { solid_id: solid.id.clone(), reason: e.to_string() })?;
        let volume_mesh = crate::mesh::extrude_tri_mesh(&tri_mesh, solid.height, solid.layers.max(1));
        let tet_mesh = crate::mesh::split_to_tets(&volume_mesh);
        let points: Vec<[f64; 3]> = tet_mesh.points.iter().map(|p| [p[0], p[1], p[2] + solid.base_z]).collect();
        let node_ids = points
            .iter()
            .enumerate()
            .map(|(point_index, p)| match doc.nodes.iter().find(|n| (n.x - p[0]).abs() < 1e-9 && (n.y - p[1]).abs() < 1e-9 && (n.z - p[2]).abs() < 1e-9) {
                Some(n) => n.id.clone(),
                None => format!("{}_m{}", solid.id, point_index),
            })
            .collect();
        let tets: Vec<[u32; 4]> = tet_mesh
            .cells
            .iter()
            .filter_map(|c| match c {
                crate::mesh::Cell::Tet4(t) => Some(*t),
                _ => None,
            })
            .collect();
        let boundary_mesh = crate::mesh::VolumeMesh { points: points.clone(), cells: tet_mesh.cells };
        let boundary_tris = crate::mesh::boundary_faces(&boundary_mesh);
        out.push(SolidMesh { solid_id: solid.id.clone(), points, tets, boundary_tris, node_ids });
    }
    Ok(out)
}

/// 🎨️ Nodal-averaged von Mises stress for `case_id`'s solved result, keyed by node id — the
/// document-layer bridge to `crate::analyses::nodal_averaged_scalar`, mirroring `fem_2d`'s
/// `fem2d_nodal_von_mises`, feeding `fem-plugin`'s solid stress contour rendering.
pub async fn fem3d_nodal_von_mises(doc: &Fem3dSnapshot, case_id: &str) -> Result<HashMap<String, f64>, Fem3dError> {
    let (nodes, elements, _solids, supports) = meshing::resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let results = super::fem3d_solve_all(doc)?;
    let result = results.get(case_id).ok_or_else(|| Fem3dError::LoadCaseNotFound(case_id.to_string()))?;
    Ok(analyses::nodal_averaged_scalar(&model, result, analyses::StressScalar::VonMises))
}

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::fem3d::{FemAnalysisSettings, FemDof, FemLoadCase, FemMaterial, FemNode, FemSolid, FemSupport};

    async fn solid_slab_doc() -> Fem3dSnapshot {
        Fem3dSnapshot {
            nodes: vec![FemNode { id: "sc0".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "sc1".into(), x: 2.0, y: 0.0, z: 0.0 }, FemNode { id: "sc2".into(), x: 2.0, y: 1.0, z: 0.0 }, FemNode { id: "sc3".into(), x: 0.0, y: 1.0, z: 0.0 }],
            elements: vec![],
            materials: vec![FemMaterial { id: "concrete".into(), name: "Concrete".into(), e: 30e9, g: 12.5e9, nu: 0.2, rho: 2400.0 }],
            sections: vec![],
            solids: vec![FemSolid { id: "sol1".into(), name: "Slab".into(), outline: vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]], holes: vec![], base_z: 0.0, height: 0.5, layers: 1, mesh_size: 1.0, material_id: "concrete".into() }],
            supports: vec![
                FemSupport { id: "s1".into(), node_id: "sc0".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s2".into(), node_id: "sc1".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s3".into(), node_id: "sc2".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s4".into(), node_id: "sc3".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
            ],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "Self Weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn fem3d_mesh_preview_returns_solid_tets_and_boundary() {
        let doc = solid_slab_doc();
        let previews = fem3d_mesh_preview(&doc).expect("mesh preview succeeds");
        assert_eq!(previews.len(), 1);
        assert_eq!(previews[0].solid_id, "sol1");
        assert!(!previews[0].tets.is_empty(), "expected at least one tet");
        assert!(!previews[0].boundary_tris.is_empty(), "expected boundary triangles");
        assert_eq!(previews[0].node_ids.len(), previews[0].points.len());
        for corner_id in ["sc0", "sc1", "sc2", "sc3"] {
            assert!(previews[0].node_ids.contains(&corner_id.to_string()), "expected corner {corner_id} to be reused");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn fem3d_nodal_von_mises_returns_finite_values_for_solid() {
        let mut doc = solid_slab_doc();
        doc.load_cases = vec![FemLoadCase { id: "pressure".into(), name: "Pressure".into(), loads: vec![crate::artifacts::fem3d::FemLoad::Area { id: "a1".into(), solid_id: "sol1".into(), pressure: 8000.0 }], self_weight: false }];
        let averaged = fem3d_nodal_von_mises(&doc, "pressure").expect("nodal von mises solves");
        assert!(!averaged.is_empty());
        for v in averaged.values() {
            assert!(v.is_finite() && *v >= 0.0, "von mises {v} should be finite and non-negative");
        }
    }
}
// #endregion 🧪️Tests


use super::*;
use crate::{FemAnalysisSettings, FemDof, FemLoad, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSupport};

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

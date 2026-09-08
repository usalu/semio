
use super::*;
use crate::artifacts::fem3d::{FemAnalysisSettings, FemDof, FemLoadCase, FemMaterial, FemNode, FemSolid, FemSupport};

fn solid_slab_doc() -> Fem3dSnapshot {
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

#[test]
fn fem3d_mesh_preview_returns_solid_tets_and_boundary() {
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

#[test]
fn fem3d_nodal_von_mises_returns_finite_values_for_solid() {
    let mut doc = solid_slab_doc();
    doc.load_cases = vec![FemLoadCase { id: "pressure".into(), name: "Pressure".into(), loads: vec![crate::artifacts::fem3d::FemLoad::Area { id: "a1".into(), solid_id: "sol1".into(), pressure: 8000.0 }], self_weight: false }];
    let averaged = fem3d_nodal_von_mises(&doc, "pressure").expect("nodal von mises solves");
    assert!(!averaged.is_empty());
    for v in averaged.values() {
        assert!(v.is_finite() && *v >= 0.0, "von mises {v} should be finite and non-negative");
    }
}

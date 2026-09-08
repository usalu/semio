
use super::*;
use crate::{FemAnalysisSettings, FemDof, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};

// #region 🔖️Fixtures
fn simply_supported_beam_doc() -> Fem2dSnapshot {
    Fem2dSnapshot {
        nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 6.0, y: 0.0 }],
        elements: vec![crate::FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
        regions: vec![],
        materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
        supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![FemDof::Ty] }],
        load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
        combinations: vec![],
        analysis: FemAnalysisSettings::default(),
    }
}

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
// #endregion 🔖️Fixtures

#[test]
fn fem2d_modal_returns_requested_mode_count() {
    let doc = rectangle_region_doc();
    let result = fem2d_modal(&doc).expect("modal solves");
    assert_eq!(result.frequencies_hz.len(), doc.analysis.modal_count);
    for w in result.frequencies_hz.windows(2) {
        assert!(w[0] <= w[1], "frequencies should be ascending: {:?}", result.frequencies_hz);
    }
    for &f in &result.frequencies_hz {
        assert!(f.is_finite() && f >= 0.0, "frequency should be finite and non-negative: {f}");
    }
}

#[test]
fn fem2d_modal_mode_values_returns_node_displacements() {
    let doc = simply_supported_beam_doc();
    let (freq, values) = fem2d_modal_mode_values(&doc, 0).expect("modal mode values solves");
    assert!(freq.is_finite() && freq >= 0.0);
    assert!(values.contains_key("n1"));
    assert!(values.contains_key("n2"));
}

#[test]
fn fem2d_buckling_returns_requested_mode_count() {
    let doc = simply_supported_beam_doc();
    let result = fem2d_buckling(&doc, "dead").expect("buckling solves");
    assert_eq!(result.factors.len(), doc.analysis.buckling_count);
    for &f in &result.factors {
        assert!(f.is_finite(), "buckling factor should be finite: {f}");
    }
}

#[test]
fn fem2d_buckling_mode_values_returns_node_displacements() {
    let doc = simply_supported_beam_doc();
    let (factor, values) = fem2d_buckling_mode_values(&doc, "dead", 0).expect("buckling mode values solves");
    assert!(factor.is_finite());
    assert!(values.contains_key("n1"));
    assert!(values.contains_key("n2"));
}

#[test]
fn fem2d_buckling_unknown_case_errors() {
    let doc = simply_supported_beam_doc();
    let err = fem2d_buckling(&doc, "missing").err().expect("expected error");
    assert!(err.to_string().contains("load case not found"), "unexpected error: {err}");
}

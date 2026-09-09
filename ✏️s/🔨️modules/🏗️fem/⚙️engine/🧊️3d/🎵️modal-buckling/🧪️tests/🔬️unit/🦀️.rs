use super::*;
use crate::{FemAnalysisSettings, FemDof, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport};

fn cantilever_fixture() -> Fem3dSnapshot {
    let (e, g, a, iy, iz, j, l, p) = (210e9, 80.77e9, 0.00538, 0.0000369, 0.0000133, 0.00000060, 3.0, 5000.0);
    Fem3dSnapshot {
        nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: l, y: 0.0, z: 0.0 }],
        elements: vec![FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.0 }],
        materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e, g, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "hea200".into(), name: "HEA200".into(), area: a, iy, iz, j }],
        solids: vec![],
        supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() }],
        load_cases: vec![FemLoadCase { id: "point".into(), name: "Point Load".into(), loads: vec![crate::FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -p }], self_weight: false }],
        combinations: vec![],
        analysis: FemAnalysisSettings::default(),
    }
}

#[test]
fn fem3d_modal_returns_requested_mode_count() {
    let doc = cantilever_fixture();
    let result = fem3d_modal(&doc).expect("modal solves");
    assert_eq!(result.frequencies_hz.len(), doc.analysis.modal_count);
    for w in result.frequencies_hz.windows(2) {
        assert!(w[0] <= w[1], "frequencies should be ascending: {:?}", result.frequencies_hz);
    }
    for &f in &result.frequencies_hz {
        assert!(f.is_finite() && f >= 0.0, "frequency should be finite and non-negative: {f}");
    }
}

#[test]
fn fem3d_modal_mode_values_returns_node_displacements() {
    let doc = cantilever_fixture();
    let (freq, values) = fem3d_modal_mode_values(&doc, 0).expect("modal mode values solves");
    assert!(freq.is_finite() && freq >= 0.0);
    assert!(values.contains_key("n1"));
    assert!(values.contains_key("n2"));
}

#[test]
fn fem3d_buckling_returns_requested_mode_count() {
    let doc = cantilever_fixture();
    let result = fem3d_buckling(&doc, "point").expect("buckling solves");
    assert_eq!(result.factors.len(), doc.analysis.buckling_count);
    for &f in &result.factors {
        assert!(f.is_finite(), "buckling factor should be finite: {f}");
    }
}

#[test]
fn fem3d_buckling_mode_values_returns_node_displacements() {
    let doc = cantilever_fixture();
    let (factor, values) = fem3d_buckling_mode_values(&doc, "point", 0).expect("buckling mode values solves");
    assert!(factor.is_finite());
    assert!(values.contains_key("n1"));
    assert!(values.contains_key("n2"));
}

#[test]
fn fem3d_buckling_unknown_case_errors() {
    let doc = cantilever_fixture();
    let err = fem3d_buckling(&doc, "missing").err().expect("expected error");
    assert!(err.to_string().contains("load case not found"), "unexpected error: {err}");
}

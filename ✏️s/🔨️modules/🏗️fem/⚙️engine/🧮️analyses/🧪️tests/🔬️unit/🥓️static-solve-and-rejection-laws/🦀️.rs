/// 🥓️ Static solve and rejection laws: multi-case solve, combination, modal, buckling, nodal averaging and invalid-model refusals.


/// 🧮️ Cross-validates `solve_multi_case`'s sparse RCM-ordered pipeline (single case) against
/// `solve_linear_static`'s already-correct dense pipeline on an equivalent model — same oracle
/// strategy already used elsewhere in this crate.
#[test]
fn solve_multi_case_matches_single_case_dense_solve() {
    let (e, area, iy, l) = (200e9, 0.01, 1e-5, 2.0);
    let (model, cases) = cantilever_analysis_model(e, area, iy, l, 0.0);
    let results = solve_multi_case(&model, &cases, &[], [0.0, 0.0, 0.0]).expect("solves");
    let sparse_result = results.get(&ResultSource::LoadCase("tip_load".into())).expect("case present");

    let dense_model = Model {
        nodes: model.nodes.clone(),
        elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 }.into()],
        supports: model.supports.clone(),
        nodal_loads: cases[0].nodal_loads.clone(),
        member_loads: vec![],
    };
    let dense_result = solve_linear_static(&dense_model).expect("dense solves");

    for sd in &sparse_result.displacements {
        let dd = dense_result.displacements.iter().find(|d| d.node_id == sd.node_id).unwrap();
        for k in 0..6 {
            assert!((sd.values[k] - dd.values[k]).abs() < 1e-8, "displacement mismatch at {} dof {k}: {} vs {}", sd.node_id, sd.values[k], dd.values[k]);
        }
    }
    for sr in &sparse_result.reactions {
        let dr = dense_result.reactions.iter().find(|r| r.node_id == sr.node_id && r.dof == sr.dof).unwrap();
        assert!((sr.value - dr.value).abs() < 1e-8, "reaction mismatch at {} {:?}: {} vs {}", sr.node_id, sr.dof, sr.value, dr.value);
    }
}

/// ➕️ A `Combination` must equal hand-computed superposition of the individually-solved case results.
#[test]
fn combination_equals_manual_superposition() {
    let (e, area, iy, l) = (200e9, 0.01, 1e-5, 2.0);
    let model = AnalysisModel {
        nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
        elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 }.into()],
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
    };
    let case_a = LoadCase { id: "a_case".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: -1000.0 }], member_loads: vec![], self_weight: false };
    let case_b = LoadCase { id: "b_case".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Rz, value: 500.0 }], member_loads: vec![], self_weight: false };
    let combo = Combination { id: "combo".into(), terms: vec![("a_case".into(), 1.35), ("b_case".into(), 1.5)] };

    let results = solve_multi_case(&model, &[case_a, case_b], &[combo], [0.0, 0.0, 0.0]).expect("solves");
    let ra = results.get(&ResultSource::LoadCase("a_case".into())).unwrap().clone();
    let rb = results.get(&ResultSource::LoadCase("b_case".into())).unwrap().clone();
    let combined = results.get(&ResultSource::Combination("combo".into())).unwrap();

    for cd in &combined.displacements {
        let ad = ra.displacements.iter().find(|d| d.node_id == cd.node_id).unwrap();
        let bd = rb.displacements.iter().find(|d| d.node_id == cd.node_id).unwrap();
        for k in 0..6 {
            let expected = 1.35 * ad.values[k] + 1.5 * bd.values[k];
            assert!((cd.values[k] - expected).abs() < 1e-8, "combo displacement mismatch at {} dof {k}", cd.node_id);
        }
    }
    for cr in &combined.reactions {
        let ar = ra.reactions.iter().find(|r| r.node_id == cr.node_id && r.dof == cr.dof).unwrap();
        let br = rb.reactions.iter().find(|r| r.node_id == cr.node_id && r.dof == cr.dof).unwrap();
        let expected = 1.35 * ar.value + 1.5 * br.value;
        assert!((cr.value - expected).abs() < 1e-8, "combo reaction mismatch at {} {:?}", cr.node_id, cr.dof);
    }
}

/// ⚖️ Self-weight-only equilibrium: the sum of vertical reactions must equal `ρAL * g` — a
/// strong, simple physical check independent of the moment distribution.
#[test]
fn self_weight_matches_total_mass_times_gravity() {
    let (e, area, iy, l, density) = (30e9, 0.05, 1e-4, 6.0, 2400.0);
    let model = AnalysisModel {
        nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
        elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density }.into()],
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: "b".into(), fixed: vec![Dof::Ty] }],
    };
    let case = LoadCase { id: "self_weight".into(), nodal_loads: vec![], member_loads: vec![], self_weight: true };
    let results = solve_multi_case(&model, &[case], &[], [0.0, -9.81, 0.0]).expect("solves");
    let result = results.get(&ResultSource::LoadCase("self_weight".into())).unwrap();

    let total_ty_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Ty).map(|r| r.value).sum();
    let expected = density * area * l * 9.81;
    assert!((total_ty_reaction - expected).abs() / expected < 0.01, "reaction sum {total_ty_reaction} vs expected {expected}");
}

/// 🎯️ Cantilever modal frequencies vs the classical closed form `f_i = (β_iL)²/(2πL²)·sqrt(EI/ρA)`
/// with `β_iL` the first three roots of `cos(x)cosh(x)+1=0`. NINE elements are kept (27 free
/// DOFs; `sparse::SUBSPACE_MAXIMUM_ORDER = 40` caps any refinement at 13 anyway) because that
/// discretisation already reaches 2 % with two orders of magnitude to spare: an independent
/// numpy/scipy `eigh` on the identical consistent-mass Hermitian beam system reports
/// 9.924511 / 62.198891 / 174.216924 Hz, i.e. 0.0001 % / 0.005 % / 0.039 % above the closed form
/// (`🔨️w7-kernel-references.py` section 1). The 2 % bound therefore gates the SOLVER
/// (`subspace_iteration` convergence, mass assembly), not the mesh.
#[test]
fn modal_cantilever_matches_analytical_frequencies() {
    let (e, iy, area, density, total_l) = (200e9, 1e-5, 0.01, 7850.0, 3.0);
    let n = 9;
    let dl = total_l / n as f64;
    let nodes: Vec<Node> = (0..=n).map(|i| Node { id: format!("n{i}"), pos: [dl * i as f64, 0.0, 0.0] }).collect();
    let elements: Vec<Elements> = (0..n).map(|i| BeamEb2 { id: format!("e{i}"), start: format!("n{i}"), end: format!("n{}", i + 1), e, area, iy, density }.into()).collect();
    let model = AnalysisModel { nodes, elements, supports: vec![Support { node_id: "n0".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }] };

    let result = modal(&model, 3).expect("modal solves");
    let beta_l = [1.875104068711961_f64, 4.694091132974175, 7.854757438237613];
    let scale = (e * iy / (density * area)).sqrt();
    for i in 0..3 {
        let expected = (beta_l[i] * beta_l[i]) / (2.0 * std::f64::consts::PI * total_l * total_l) * scale;
        let actual = result.frequencies_hz[i];
        assert!((actual - expected).abs() / expected < 0.02, "mode {i}: {actual} Hz vs analytical {expected} Hz");
    }
}

/// 🌀️ Euler column buckling across all four classical end conditions vs `π²EI/(KL)²`. An
/// independent numpy/scipy geometric-stiffness eigenproblem on this identical 7-element
/// discretisation (`🔨️w7-kernel-references.py` section 2) reports 1754694.177 N (K=1.0,
/// 0.006 % above the closed form), 7024460.631 N (K=0.5, 0.087 %), 3590288.885 N (K=0.7,
/// 0.265 % — of which 0.24 % is the K=0.7 rounding of the exact `4.4934²EI/L²` root of
/// `tan(x)=x`) and 438650.625 N (K=2.0, 0.0004 %), so 2 % is a real gate on every case.
#[test]
fn buckling_euler_column_matches_analytical_load() {
    let (e, iy, area, density, total_l) = (200e9, 8e-6, 0.005, 7850.0, 3.0);
    let n = 7usize;
    let dl = total_l / n as f64;
    let p_ref = 1.0;
    let cases: [(&str, &[Dof], &[Dof], f64); 4] = [
        ("pinned-pinned", &[Dof::Tx, Dof::Ty], &[Dof::Ty], 1.0),
        ("fixed-fixed", &[Dof::Tx, Dof::Ty, Dof::Rz], &[Dof::Ty, Dof::Rz], 0.5),
        ("fixed-pinned", &[Dof::Tx, Dof::Ty, Dof::Rz], &[Dof::Ty], 0.7),
        ("fixed-free", &[Dof::Tx, Dof::Ty, Dof::Rz], &[], 2.0),
    ];

    for (name, base, tip, k_factor) in cases {
        let nodes: Vec<Node> = (0..=n).map(|i| Node { id: format!("n{i}"), pos: [dl * i as f64, 0.0, 0.0] }).collect();
        let elements: Vec<Elements> = (0..n).map(|i| BeamEb2 { id: format!("e{i}"), start: format!("n{i}"), end: format!("n{}", i + 1), e, area, iy, density }.into()).collect();
        let mut supports = vec![Support { node_id: "n0".into(), fixed: base.to_vec() }];
        if !tip.is_empty() {
            supports.push(Support { node_id: format!("n{n}"), fixed: tip.to_vec() });
        }
        let model = AnalysisModel { nodes, elements, supports };
        let reference_case = LoadCase { id: "axial_compression".into(), nodal_loads: vec![NodalLoad { node_id: format!("n{n}"), dof: Dof::Tx, value: -p_ref }], member_loads: vec![], self_weight: false };

        let static_results = solve_multi_case(&model, std::slice::from_ref(&reference_case), &[], [0.0, 0.0, 0.0]).expect("reference solves");
        let static_result = static_results.get(&ResultSource::LoadCase("axial_compression".into())).unwrap();
        for d in &static_result.displacements {
            assert!(d.values[Dof::Ty.index()].abs() < 1e-9, "{name}: unexpected transverse displacement at {}: {}", d.node_id, d.values[Dof::Ty.index()]);
        }

        let critical_load = buckling(&model, &reference_case, 1, [0.0; 3]).expect("buckling solves").factors[0] * p_ref;
        let expected = std::f64::consts::PI.powi(2) * e * iy / (k_factor * total_l).powi(2);
        assert!(critical_load > 0.0, "{name}: critical load should be positive, got {critical_load}");
        assert!((critical_load - expected).abs() / expected < 0.02, "{name}: critical load {critical_load} vs analytical {expected}");
    }
}

/// 🔍️ Duplicate-node-id models are rejected the same way `lib.rs::validate` rejects them.
#[test]
fn duplicate_node_id_is_rejected() {
    let model = AnalysisModel { nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "a".into(), pos: [1.0, 0.0, 0.0] }], elements: vec![], supports: vec![] };
    let err = solve_multi_case(&model, &[], &[], [0.0, 0.0, 0.0]).unwrap_err();
    assert_eq!(err, FemError::DuplicateNodeId("a".into()));
}

/// 🔍️ A `Bar2` model works fine through the multi-case pipeline too (not just `BeamEb2`).
#[test]
fn solve_multi_case_supports_bar2_truss() {
    let (e, area, l, p) = (200e9, 0.001, 2.0, 5000.0);
    let model = AnalysisModel {
        nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
        elements: vec![Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, density: 0.0 }.into()],
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: "b".into(), fixed: vec![Dof::Ty] }],
    };
    let case = LoadCase { id: "axial".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Tx, value: p }], member_loads: vec![], self_weight: false };
    let results = solve_multi_case(&model, &[case], &[], [0.0, 0.0, 0.0]).expect("solves");
    let result = results.get(&ResultSource::LoadCase("axial".into())).unwrap();
    let expected = p * l / (e * area);
    let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
    assert!((b.values[Dof::Tx.index()] - expected).abs() / expected < 1e-8);
}

/// 🎨️ Patch test for `nodal_averaged_scalar`: TWO `Tri3Cst` triangles splitting a square along its
/// diagonal, both under the SAME uniform uniaxial strain field (`u=a*x`, `v=-nu*a*y`) — every
/// node's averaged von Mises must equal the exact analytical `E*a` (a constant field averages to
/// itself regardless of how many elements touch a node).
#[test]
fn nodal_averaged_scalar_patch_test_is_exact_under_uniform_stress() {
    use semio_s_fem_elements2d::PlaneKind;
    use semio_s_fem_elements2d::Tri3Cst;
    let (e, nu, t) = (1000.0, 0.25, 1.0);
    let coords = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]];
    let nodes: Vec<Node> = (0..4).map(|i| Node { id: format!("n{i}"), pos: [coords[i][0], coords[i][1], 0.0] }).collect();
    let el1 = Tri3Cst { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };
    let el2 = Tri3Cst { id: "t2".into(), nodes: ["n0".into(), "n2".into(), "n3".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };

    let a = 0.01;
    let u_of = |ids: [usize; 3]| VecD::from_vec(ids.iter().flat_map(|&i| [a * coords[i][0], -nu * a * coords[i][1]]).collect());
    let ctx_of = |ids: [usize; 3]| ElementContext { positions: ids.iter().map(|&i| [coords[i][0], coords[i][1], 0.0]).collect() };

    let r1 = el1.recover(&ctx_of([0, 1, 2]), &u_of([0, 1, 2]), None);
    let r2 = el2.recover(&ctx_of([0, 2, 3]), &u_of([0, 2, 3]), None);

    let model = AnalysisModel { nodes, elements: vec![el1.into(), el2.into()], supports: vec![] };
    let result = StaticResult { displacements: vec![], reactions: vec![], elements: vec![("t1".into(), r1), ("t2".into(), r2)], checks: SolutionChecks { residual_norm: 0.0, reaction_sum: [0.0; 6] } };

    let averaged = nodal_averaged_scalar(&model, &result, StressScalar::VonMises);
    let expected_vm = (e * a).abs();
    for id in ["n0", "n1", "n2", "n3"] {
        let v = *averaged.get(id).unwrap_or_else(|| panic!("node {id} missing from averaged map"));
        assert!((v - expected_vm).abs() / expected_vm < 1e-8, "node {id}: {v} vs {expected_vm}");
    }
}

/// 🎨️ `nodal_averaged_scalar` on two elements sharing exactly one node but reporting DIFFERENT
/// constant von Mises values: the shared node's averaged value must land strictly between the
/// two elements' own values, while each element's exclusive nodes keep that element's exact value.
#[test]
fn nodal_averaged_scalar_shared_node_is_between_neighboring_element_values() {
    use semio_s_fem_elements2d::PlaneKind;
    use semio_s_fem_elements2d::Tri3Cst;
    let (e, nu, t) = (1000.0, 0.25, 1.0);
    let el_a = Tri3Cst { id: "a".into(), nodes: ["shared".into(), "a1".into(), "a2".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };
    let el_b = Tri3Cst { id: "b".into(), nodes: ["shared".into(), "b1".into(), "b2".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };

    let ctx_a = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]] };
    let ctx_b = ElementContext { positions: vec![[0.0, 0.0, 0.0], [-2.0, 0.0, 0.0], [0.0, -2.0, 0.0]] };
    let u_a = VecD::from_vec(vec![0.0, 0.0, 0.04, 0.0, 0.0, 0.0]);
    let u_b = VecD::from_vec(vec![0.0, 0.0, -0.1, 0.0, 0.0, 0.0]);

    let r_a = el_a.recover(&ctx_a, &u_a, None);
    let r_b = el_b.recover(&ctx_b, &u_b, None);
    let (va, vb) = match (&r_a, &r_b) {
        (ElementResult::Plane { gauss: ga }, ElementResult::Plane { gauss: gb }) => (ga[0].von_mises, gb[0].von_mises),
        _ => panic!("expected plane results"),
    };
    assert!(va < vb, "test setup should give distinct, ordered element values, got {va} vs {vb}");

    let nodes = vec![
        Node { id: "shared".into(), pos: [0.0, 0.0, 0.0] },
        Node { id: "a1".into(), pos: [2.0, 0.0, 0.0] },
        Node { id: "a2".into(), pos: [0.0, 2.0, 0.0] },
        Node { id: "b1".into(), pos: [-2.0, 0.0, 0.0] },
        Node { id: "b2".into(), pos: [0.0, -2.0, 0.0] },
    ];
    let model = AnalysisModel { nodes, elements: vec![el_a.into(), el_b.into()], supports: vec![] };
    let result = StaticResult { displacements: vec![], reactions: vec![], elements: vec![("a".into(), r_a), ("b".into(), r_b)], checks: SolutionChecks { residual_norm: 0.0, reaction_sum: [0.0; 6] } };

    let averaged = nodal_averaged_scalar(&model, &result, StressScalar::VonMises);
    let shared = *averaged.get("shared").unwrap();
    assert!(shared > va && shared < vb, "shared node value {shared} should be strictly between {va} and {vb}");
    assert!((*averaged.get("a1").unwrap() - va).abs() < 1e-9);
    assert!((*averaged.get("a2").unwrap() - va).abs() < 1e-9);
    assert!((*averaged.get("b1").unwrap() - vb).abs() < 1e-9);
    assert!((*averaged.get("b2").unwrap() - vb).abs() < 1e-9);
}

/// 🔍️ An empty `AnalysisModel` is rejected the same way `Model`'s top-level `validate` rejects it.
#[test]
fn empty_model_is_rejected() {
    let model = AnalysisModel { nodes: vec![], elements: vec![], supports: vec![] };
    let err = solve_multi_case(&model, &[], &[], [0.0, 0.0, 0.0]).unwrap_err();
    assert_eq!(err, FemError::EmptyModel);
}

/// 🔍️ An element referencing a node id absent from `model.nodes` is rejected.
#[test]
fn dangling_element_node_ref_is_rejected() {
    let model = AnalysisModel { nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }], elements: vec![Bar2 { id: "e1".into(), start: "a".into(), end: "missing".into(), e: 1.0, area: 1.0, density: 0.0 }.into()], supports: vec![] };
    let err = solve_multi_case(&model, &[], &[], [0.0, 0.0, 0.0]).unwrap_err();
    assert_eq!(err, FemError::DanglingNodeRef("missing".into()));
}

/// 🔍️ A support referencing a node id absent from `model.nodes` is rejected.
#[test]
fn dangling_support_node_ref_is_rejected() {
    let model = AnalysisModel { nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }], elements: vec![], supports: vec![Support { node_id: "missing".into(), fixed: vec![Dof::Tx] }] };
    let err = solve_multi_case(&model, &[], &[], [0.0, 0.0, 0.0]).unwrap_err();
    assert_eq!(err, FemError::DanglingNodeRef("missing".into()));
}

/// 🔍️ A `LoadCase` nodal load referencing a node id absent from `model.nodes` is rejected —
/// `validate_case`'s own check, distinct from `validate`'s model-wide checks above.
#[test]
fn dangling_load_case_node_ref_is_rejected() {
    let model = AnalysisModel { nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }], elements: vec![], supports: vec![] };
    let case = LoadCase { id: "bad".into(), nodal_loads: vec![NodalLoad { node_id: "missing".into(), dof: Dof::Tx, value: 1.0 }], member_loads: vec![], self_weight: false };
    let err = solve_multi_case(&model, &[case], &[], [0.0, 0.0, 0.0]).unwrap_err();
    assert_eq!(err, FemError::DanglingNodeRef("missing".into()));
}

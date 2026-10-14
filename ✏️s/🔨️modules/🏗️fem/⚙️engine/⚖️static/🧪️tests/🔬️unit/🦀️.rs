use super::*;

fn two_spring_model() -> Model {
    Model {
        nodes: vec![Node { id: "n1".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "n2".into(), pos: [1.0, 0.0, 0.0] }],
        elements: vec![AxialSpring { id: "e1".into(), a: "n1".into(), b: "n2".into(), k: 1000.0 }.into()],
        supports: vec![Support { node_id: "n1".into(), fixed: vec![Dof::Tx] }],
        nodal_loads: vec![NodalLoad { node_id: "n2".into(), dof: Dof::Tx, value: 10.0 }],
        member_loads: vec![],
    }
}

#[test]
fn solves_single_spring_against_hand_calc() {
    let model = two_spring_model();
    let result = solve_linear_static(&model).expect("solves");
    let n2 = result.displacements.iter().find(|d| d.node_id == "n2").unwrap();
    assert!((n2.values[Dof::Tx.index()] - 0.01).abs() < 1e-9);
    let reaction = result.reactions.iter().find(|r| r.node_id == "n1").unwrap();
    assert!((reaction.value + 10.0).abs() < 1e-9);
    match &result.elements[0].1 {
        ElementResult::Bar { n } => assert!((n - 10.0).abs() < 1e-9),
        _ => panic!("expected bar result"),
    }
}

#[test]
fn equilibrium_checks_are_near_zero() {
    let model = two_spring_model();
    let result = solve_linear_static(&model).expect("solves");
    assert!(result.checks.residual_norm < 1e-9);
    assert!(result.checks.reaction_sum[Dof::Tx.index()].abs() < 1e-9);
}

#[test]
fn empty_model_is_rejected() {
    let model = Model::default();
    assert_eq!(solve_linear_static(&model), Err(FemError::EmptyModel));
}

#[test]
fn dangling_node_ref_is_rejected() {
    let mut model = two_spring_model();
    model.supports.push(Support { node_id: "missing".into(), fixed: vec![Dof::Tx] });
    assert_eq!(solve_linear_static(&model), Err(FemError::DanglingNodeRef("missing".into())));
}

#[test]
fn unconstrained_model_is_singular() {
    let mut model = two_spring_model();
    model.supports.clear();
    assert_eq!(solve_linear_static(&model), Err(FemError::Singular));
}

#[test]
fn load_on_inactive_dof_is_silently_skipped() {
    let mut model = two_spring_model();
    model.nodal_loads.push(NodalLoad { node_id: "n2".into(), dof: Dof::Ty, value: 999.0 });
    let result = solve_linear_static(&model).expect("solves despite inactive-dof load");
    let n2 = result.displacements.iter().find(|d| d.node_id == "n2").unwrap();
    assert!((n2.values[Dof::Tx.index()] - 0.01).abs() < 1e-9);
}

/// 🔍️ Duplicate node ids are rejected the same way `analyses::validate` rejects them.
#[test]
fn duplicate_node_id_is_rejected() {
    let mut model = two_spring_model();
    model.nodes.push(Node { id: "n1".into(), pos: [5.0, 0.0, 0.0] });
    assert_eq!(solve_linear_static(&model), Err(FemError::DuplicateNodeId("n1".into())));
}

/// 🔍️ `Model`'s hand-rolled `Debug` (trait objects aren't `Debug`) must print element ids, not panic.
#[test]
fn model_debug_fmt_prints_element_ids_not_trait_objects() {
    let model = two_spring_model();
    let printed = format!("{model:?}");
    assert!(printed.contains("e1"), "expected element id \"e1\" in {printed}");
    assert!(printed.contains("n1") && printed.contains("n2"), "expected node ids in {printed}");
}

/// 🌬️ A member UDL on an element that doesn't override `equivalent_nodal_loads` (the trait default,
/// `None`) is silently a no-op — same displacement as solving with no member load at all.
#[test]
fn member_udl_on_element_without_udl_support_is_a_no_op() {
    let mut model = two_spring_model();
    model.member_loads.push(("e1".into(), MemberUdl { wx: 123.0, wy: 456.0, wz: 0.0 }));
    let with_udl = solve_linear_static(&model).expect("solves");
    let without_udl = solve_linear_static(&two_spring_model()).expect("solves");
    for (a, b) in with_udl.displacements.iter().zip(without_udl.displacements.iter()) {
        for k in 0..6 {
            assert!((a.values[k] - b.values[k]).abs() < 1e-12, "dof {k}: {} vs {}", a.values[k], b.values[k]);
        }
    }
}

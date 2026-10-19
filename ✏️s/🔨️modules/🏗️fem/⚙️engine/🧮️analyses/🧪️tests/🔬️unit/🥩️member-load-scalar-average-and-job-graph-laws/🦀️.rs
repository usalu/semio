/// 🥩️ Member load, scalar average and job graph laws.


/// 🌬️ `solve_multi_case`'s member-UDL branch (`case_rhs_old`'s `equivalent_nodal_loads` path) must
/// match `solve_linear_static`'s dense pipeline (`model.member_loads`) on an equivalent model.
#[test]
fn solve_multi_case_applies_member_udl_equivalent_loads() {
    let (e, area, iy, l, w) = (200e9, 0.01, 1e-5, 2.0, 500.0);
    let model = AnalysisModel {
        nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
        elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 }.into()],
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
    };
    let case = LoadCase { id: "udl".into(), nodal_loads: vec![], member_loads: vec![("e1".into(), MemberUdl { wx: 0.0, wy: -w, wz: 0.0 })], self_weight: false };
    let results = solve_multi_case(&model, &[case], &[], [0.0, 0.0, 0.0]).expect("solves");
    let sparse_result = results.get(&ResultSource::LoadCase("udl".into())).unwrap();

    let dense_model = Model {
        nodes: model.nodes.clone(),
        elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 }.into()],
        supports: model.supports.clone(),
        nodal_loads: vec![],
        member_loads: vec![("e1".into(), MemberUdl { wx: 0.0, wy: -w, wz: 0.0 })],
    };
    let dense_result = solve_linear_static(&dense_model).expect("dense solves");

    for sd in &sparse_result.displacements {
        let dd = dense_result.displacements.iter().find(|d| d.node_id == sd.node_id).unwrap();
        for k in 0..6 {
            assert!((sd.values[k] - dd.values[k]).abs() < 1e-8, "displacement mismatch at {} dof {k}", sd.node_id);
        }
    }
}

/// 📊️ `element_scalar_average` covers every element-kind/scalar combination it recognizes (`Some`)
/// and every mismatched combination (`None`) — arms `nodal_averaged_scalar`'s own patch tests never
/// happen to exercise (those only touch `Plane`/`VonMises`).
#[test]
fn element_scalar_average_covers_every_variant_and_scalar_combination() {
    let plane = ElementResult::Plane { gauss: vec![PlaneStress { sxx: 1.0, syy: 2.0, sxy: 3.0, von_mises: 4.0 }] };
    assert_eq!(element_scalar_average(&plane, StressScalar::VonMises), Some(4.0));
    assert_eq!(element_scalar_average(&plane, StressScalar::Sxx), Some(1.0));
    assert_eq!(element_scalar_average(&plane, StressScalar::Syy), Some(2.0));
    assert_eq!(element_scalar_average(&plane, StressScalar::Sxy), Some(3.0));
    assert_eq!(element_scalar_average(&plane, StressScalar::Szz), None);
    assert_eq!(element_scalar_average(&plane, StressScalar::VonMisesTop), None);

    let solid = ElementResult::Solid { gauss: vec![SolidStress { sxx: 1.0, syy: 2.0, szz: 3.0, sxy: 4.0, syz: 5.0, sxz: 6.0, von_mises: 7.0 }] };
    assert_eq!(element_scalar_average(&solid, StressScalar::VonMises), Some(7.0));
    assert_eq!(element_scalar_average(&solid, StressScalar::Sxx), Some(1.0));
    assert_eq!(element_scalar_average(&solid, StressScalar::Syy), Some(2.0));
    assert_eq!(element_scalar_average(&solid, StressScalar::Szz), Some(3.0));
    assert_eq!(element_scalar_average(&solid, StressScalar::Sxy), Some(4.0));
    assert_eq!(element_scalar_average(&solid, StressScalar::Syz), Some(5.0));
    assert_eq!(element_scalar_average(&solid, StressScalar::Sxz), Some(6.0));
    assert_eq!(element_scalar_average(&solid, StressScalar::VonMisesTop), None);

    let shell = ElementResult::Shell { gauss: vec![ShellState { nxx: 0.0, nyy: 0.0, nxy: 0.0, mxx: 0.0, myy: 0.0, mxy: 0.0, von_mises_top: 8.0, von_mises_bottom: 9.0 }] };
    assert_eq!(element_scalar_average(&shell, StressScalar::VonMisesTop), Some(8.0));
    assert_eq!(element_scalar_average(&shell, StressScalar::VonMisesBottom), Some(9.0));
    assert_eq!(element_scalar_average(&shell, StressScalar::VonMises), None);

    let bar = ElementResult::Bar { n: 42.0 };
    assert_eq!(element_scalar_average(&bar, StressScalar::VonMises), None);
}

fn graph_operation(id: u64) -> Operation {
    Operation::new(semio_framework_job::OperationId(id), semio_framework_job::RevisionId(4), semio_framework_job::Generation(2), 9)
}

fn graph_plan() -> Vec<FemStagePlan> {
    vec![
        FemStagePlan { stage: FemJobStage::ValidateReferences, units: 1 },
        FemStagePlan { stage: FemJobStage::BuildDofMap, units: 2 },
        FemStagePlan { stage: FemJobStage::OrderEquations, units: 3 },
        FemStagePlan { stage: FemJobStage::Assemble, units: 4 },
        FemStagePlan { stage: FemJobStage::Factor, units: 5 },
        FemStagePlan { stage: FemJobStage::Solve, units: 6 },
        FemStagePlan { stage: FemJobStage::Recover, units: 7 },
        FemStagePlan { stage: FemJobStage::Finalize, units: 8 },
    ]
}

#[test]
fn fem_job_graph_checkpoint_resume_preserves_stage_order() {
    let operation = graph_operation(201);
    let mut graph = FemJobGraph::new(operation, graph_plan(), 2);
    let mut sequence = 0;
    let checkpoint = loop {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(2, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        match seen_of(graph.step(&mut context)) {
            Seen::Checkpoint(pages) => break pages_bytes(&pages),
            outcome => assert!(!matches!(outcome, Seen::Complete(_)), "graph must checkpoint before completing"),
        }
    };
    let mut resumed = FemJobGraph::from_checkpoint(operation, &checkpoint).expect("graph checkpoint restores");
    assert_eq!(resumed.checkpoint_bytes(), checkpoint);
    let mut seen = Vec::new();
    loop {
        if let Some(stage) = resumed.progress().stage {
            if seen.last() != Some(&stage) {
                seen.push(stage);
            }
        }
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(3, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        if matches!(seen_of(resumed.step(&mut context)), Seen::Complete(_)) {
            break;
        }
    }
    close_job(&mut graph);
    close_job(&mut resumed);
    assert_eq!(resumed.progress().completed_units, 36);
    assert_eq!(seen, graph_plan().into_iter().skip(1).map(|plan| plan.stage).collect::<Vec<_>>());
}

#[test]
fn fem_job_graph_rejects_stale_and_cancelled_steps_without_mutation() {
    let operation = graph_operation(202);
    let mut graph = FemJobGraph::new(operation, graph_plan(), 2);
    let before = graph.checkpoint_bytes();
    let mut sequence = 0;
    let mut sequence_receipt = RetainedCloneProgress::default();
    let mut stale = StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(2, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
    assert!(matches!(seen_of(graph.step(&mut stale)), Seen::Fault(_)));
    assert_eq!(graph.checkpoint_bytes(), before);

    let token = semio_framework_job::root_cancel_token();
    token.cancel_now();
    let mut sequence_receipt = RetainedCloneProgress::default();
    drop(stale);
    let mut cancelled = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(2, u64::MAX, TEST_GRANT), token, || Some(0), &mut sequence, &mut sequence_receipt);
    assert_eq!(seen_of(graph.step(&mut cancelled)), Seen::Cancelled);
    assert_eq!(graph.checkpoint_bytes(), before);
}

/// 🎟️ Resource admission refuses excess owners before allocation and never labels a mechanism.
#[test]
fn numerical_owner_page_admission_refuses_before_allocating_and_is_not_singularity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️owner-admission/🔣️.json")).unwrap();
    assert_eq!(fixture["pageBytes"].as_u64().unwrap() as usize, MOUNTED_OWNER_PAGE_BYTES);
    assert_eq!(fixture["itemBytes"].as_u64().unwrap() as usize, size_of::<f64>());
    for case in fixture["cases"].as_array().unwrap() {
        let mut owner = Vec::<f64>::new();
        let accepted = reserve_exact_owner_page(&mut owner, case["requested"].as_u64().unwrap() as usize);
        assert_eq!(accepted, case["accepted"].as_bool().unwrap());
        if !accepted { assert_eq!(owner.capacity(), 0); }
    }
    let model = Arc::new(AnalysisModel { nodes: vec![Node { id: "node".into(), pos: [0.0; 3] }], elements: vec![], supports: vec![] });
    let operation = Operation::new(semio_framework_job::OperationId(1), semio_framework_job::RevisionId(1), semio_framework_job::Generation(1), 1);
    let mut construction = AssemblyJobConstruction::new_owned(model, operation, MOUNTED_OWNER_PAGE_BYTES + 1);
    assert_eq!(construction.step_one(), Err(FemError::ResourceAdmission));
    assert!(!FemError::ResourceAdmission.to_string().contains("singular"));
    let mut complete = false;
    for _ in 0..100 { if construction.close_step(MOUNTED_OWNER_PAGE_BYTES).0 { complete = true; break; } }
    assert!(complete);
    eprintln!("[DEBUG] numerical owner over-capacity admission refused before allocation with resource classification");
}

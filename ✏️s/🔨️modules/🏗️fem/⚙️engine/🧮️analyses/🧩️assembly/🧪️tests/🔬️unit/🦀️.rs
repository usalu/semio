use super::*;
use semio_s_fem_analyses_model::*;
use semio_s_fem_elements2d::{Bar2, BeamEb2};
use semio_s_fem_model::*;
use semio_s_fem_test_support::{pages_bytes, seen_of, test_operation, Seen, TEST_GRANT};

fn cantilever_analysis_model(e: f64, area: f64, iy: f64, l: f64, density: f64) -> (AnalysisModel, Vec<LoadCase>) {
    let model = AnalysisModel {
        nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
        elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density }.into()],
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
    };
    let cases = vec![LoadCase { id: "tip_load".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: -1000.0 }], member_loads: vec![], self_weight: false }];
    (model, cases)
}

fn axial_chain(element_count: usize) -> AnalysisModel {
    AnalysisModel {
        nodes: (0..=element_count).map(|index| Node { id: format!("n{index}"), pos: [index as f64, 0.0, 0.0] }).collect(),
        elements: (0..element_count).map(|index| AxialSpring { id: format!("e{index}"), a: format!("n{index}"), b: format!("n{}", index + 1), k: 10.0 + index as f64 }.into()).collect(),
        supports: vec![Support { node_id: "n0".to_string(), fixed: vec![Dof::Tx] }],
    }
}

/// ⛽️ Assembly control, publication, and work transitions share the same exact grant admission.
#[test]
fn assembly_triplet_pages_control_transitions_preserve_state_until_granted() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/⛽️step-grant/🔣️.json")).expect("assembly step grants");
    for (index, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let operation = test_operation(940 + index as u64);
        let model = Arc::new(cantilever_analysis_model(210e9, 0.02, 8e-6, 3.0, 7_850.0).0);
        let mut construction = AssemblyJobConstruction::new_owned(model, operation, 1);
        while !construction.step_one().expect("step grant construction") {}
        let mut job = construction.take_complete().expect("step grant job");
        job.state.checkpoint_due = case["before"]["checkpointDue"].as_bool().unwrap();
        job.state.preview_due = case["before"]["previewDue"].as_bool().unwrap();
        if case["before"]["complete"].as_bool().unwrap() { job.state.stage = AssemblyJobStage::Complete; }
        let mut sequence = 0;
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(case["fuel"].as_u64().unwrap(), case["deadline"].as_u64().unwrap(), TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        let outcome = seen_of(job.step(&mut context));
        let observed = serde_json::json!({ "checkpointDue": job.state.checkpoint_due, "previewDue": job.state.preview_due, "pendingBuild": job.state.pending_build.is_some(), "complete": job.state.stage == AssemblyJobStage::Complete });
        assert_eq!(observed, case["after"], "fixture {index} preserves or advances exactly the admitted transition");
        assert_eq!(context.fuel_remaining(), case["fuelRemaining"].as_u64().unwrap(), "fixture {index} fuel");
        assert_eq!(match outcome { Seen::Yield => "yield", Seen::Complete(_) => "complete", _ => panic!("unexpected assembly control outcome") }, case["outcome"].as_str().unwrap());
        let mut closed = false;
        for _ in 0..20_000 {
            let (terminal, items, bytes) = job.close_retained_step(MOUNTED_OWNER_PAGE_BYTES);
            assert!(items <= 1 && bytes <= MOUNTED_OWNER_PAGE_BYTES);
            if terminal { closed = true; break; }
        }
        assert!(closed, "step grant fixture releases every exact owner");
    }
}

fn assembly_physical_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️physical-owners/🔣️.json")).expect("assembly physical fixture")
}

fn assembly_physical_empty_build(matrix: Option<Csr>) -> AssemblyCsrBuild {
    AssemblyCsrBuild {
        assembly: None,
        entries: PagedList::default(),
        stage: AssemblyCsrBuildStage::Complete,
        sort_outer: 0,
        sort_inner: 0,
        count_cursor: 0,
        unique_count: 0,
        count_key: None,
        merge_cursor: 0,
        row_cursor: 0,
        row_counts: PagedList::default(),
        indptr: PagedList::default(),
        indices: PagedList::default(),
        values: PagedList::default(),
        last_key: None,
        matrix,
        n: 0,
    }
}

/// 🪶 Clearing an inline assembly record has no physical backing release of its own.
#[test]
fn assembly_triplet_pages_inline_job_close_reports_no_struct_bytes() {
    static EMPTY_MODEL: AnalysisModel = AnalysisModel { nodes: Vec::new(), elements: Vec::new(), supports: Vec::new() };
    let fixture = assembly_physical_fixture();
    let model = Arc::new(cantilever_analysis_model(210e9, 0.02, 8e-6, 3.0, 7_850.0).0);
    let mut construction = AssemblyJobConstruction::new_owned(model, test_operation(956), 1);
    let mut constructed = false;
    for _ in 0..4_096 {
        if construction.step_one().expect("inline assembly setup") { constructed = true; break; }
    }
    assert!(constructed);
    let mut job = construction.take_complete().expect("inline assembly fixture job");
    let mut job_closed = false;
    for _ in 0..20_000 {
        if job.close_retained_step(MOUNTED_OWNER_PAGE_BYTES).0 { job_closed = true; break; }
    }
    assert!(job_closed);
    job.model = AnalysisModelOwner::Borrowed(&EMPTY_MODEL);
    job.state.stage = AssemblyJobStage::Complete;
    let mut build = AssemblyCsrBuild::new(job).unwrap_or_else(|_| panic!("closed inline fixture"));
    let first = build.close_step(0);
    let terminal = build.close_step(0);
    assert_eq!(first, (false, fixture["release"]["maximumOwnersPerStep"].as_u64().unwrap() as usize, fixture["release"]["inlineOwnerBytes"].as_u64().unwrap() as usize));
    assert_eq!(terminal, (true, 0, 0));
}

/// 🧮 Empty capacitated matrix pages remain live until their exact physical grant arrives.
#[test]
fn assembly_triplet_pages_inline_matrix_close_reports_only_actual_backing() {
    let fixture = assembly_physical_fixture();
    for case in fixture["cases"].as_array().unwrap() {
        for grant in fixture["grants"].as_array().unwrap() {
            let count = case["backingCount"].as_u64().unwrap();
            let (mut indptr, mut indices, mut values) = (PagedList::default(), PagedList::default(), PagedList::default());
            if count != 0 {
                while indptr.capacity() < 1 { indptr.reserve_capacity_one(1, MOUNTED_OWNER_PAGE_BYTES).expect("row backing"); }
                while indices.capacity() < 1 { indices.reserve_capacity_one(1, MOUNTED_OWNER_PAGE_BYTES).expect("index backing"); }
                while values.capacity() < 1 { values.reserve_capacity_one(1, MOUNTED_OWNER_PAGE_BYTES).expect("value backing"); }
            }
            let next_bytes = if count == 0 { 0 } else { values.next_release_allocation_bytes().expect("next matrix backing") };
            let matrix = Csr::from_paged_parts(0, indptr, indices, values);
            let before = matrix.physical_backing_bytes();
            let allocated: usize = before.into_iter().sum();
            let bytes = match grant["relativeBytes"].as_str().unwrap() {
                "zero" => 0,
                "one-less" => next_bytes.saturating_sub(1),
                "next-backing" => next_bytes,
                _ => unreachable!(),
            };
            let mut build = assembly_physical_empty_build(Some(matrix));
            let first = build.close_step(bytes);
            let after = build.matrix.as_ref().map_or([0; 3], Csr::physical_backing_bytes);
            let mut released = first.2;
            let mut exact = allocated == after.into_iter().sum::<usize>() + first.2;
            let mut terminal = first.0;
            for _ in 0..100 {
                let before_step = build.matrix.as_ref().map_or(0, |owner| owner.physical_backing_bytes().into_iter().sum::<usize>());
                let step = build.close_step(MOUNTED_OWNER_PAGE_BYTES);
                let after_step = build.matrix.as_ref().map_or(0, |owner| owner.physical_backing_bytes().into_iter().sum::<usize>());
                exact &= step.1 <= 1 && step.2 <= MOUNTED_OWNER_PAGE_BYTES && before_step == after_step + step.2;
                released += step.2;
                if step.0 { terminal = true; break; }
            }
            assert!(terminal);
            assert!(exact, "inline retirement cannot invent backing bytes");
            assert_eq!(released, allocated);
            if count != 0 && grant["releasedBackings"].as_u64().unwrap() == 0 {
                assert_eq!(after, before);
                assert_eq!(first, (false, 0, 0));
            }
        }
    }
}

fn merge_fixture_triplet(value: &serde_json::Value) -> AssemblyTriplet {
    AssemblyTriplet {
        sequence: value["sequence"].as_u64().expect("triplet sequence"),
        row: value["row"].as_u64().and_then(|value| value.try_into().ok()).expect("triplet row"),
        col: value["column"].as_u64().and_then(|value| value.try_into().ok()).expect("triplet column"),
        value: value["value"].as_f64().expect("triplet value"),
    }
}

/// 🔀️ A full paged merge destination retains the exact selected producer and source cursor until
/// capacity becomes available, then publishes and advances each exactly once in both merge lanes.
#[test]
fn assembly_triplet_pages_merge_refusal_retains_candidate_for_retry() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔀️merge-refusal/🔣️.json")).expect("merge refusal fixture");
    for (case_index, case) in fixture["cases"].as_array().expect("merge refusal cases").iter().enumerate() {
        let full = case["lane"].as_str().expect("merge lane") == "full";
        let candidate = merge_fixture_triplet(&case["candidate"]);
        let model = Arc::new(cantilever_analysis_model(210e9, 0.02, 8e-6, 3.0, 7_850.0).0);
        let mut construction = AssemblyJobConstruction::new_owned(model, test_operation(900 + case_index as u64), 1);
        while !construction.step_one().expect("merge refusal construction") {}
        let mut job = construction.take_complete().expect("merge refusal job");
        let partition = &mut job.state.partitions[0];
        let source = if full { &mut partition.full } else { &mut partition.free };
        source.push_reserved(candidate).expect("source candidate fits admitted partition");
        let destination = if full { &mut job.state.merged_full } else { &mut job.state.merged_free };
        let filler = AssemblyTriplet { sequence: 0, row: 0, col: 0, value: 0.0 };
        while destination.has_reserved_slot() {
            destination.push_reserved(filler).expect("destination filler fits advertised slot");
        }
        let destination_len = destination.len();
        assert_eq!(job.advance_partition_merge(full), Ok(Some(false)), "scan selects the fixture candidate");
        assert_eq!(job.state.merge_candidate, Some((0, candidate)));
        assert_eq!(job.advance_partition_merge(full), Err(FemError::Singular), "full destination refuses exact candidate");
        let cursor = if full { job.state.full_merge_cursors[0] } else { job.state.free_merge_cursors[0] };
        assert_eq!(cursor, case["refused"]["sourceCursor"].as_u64().expect("refused cursor") as usize);
        assert_eq!(job.state.merge_candidate, Some((0, candidate)));
        let destination = if full { &mut job.state.merged_full } else { &mut job.state.merged_free };
        assert_eq!(destination.len(), destination_len, "refusal leaves destination unchanged");
        assert_eq!(destination.pop(), Some(filler), "one retired filler restores one exact reserved slot");
        assert_eq!(job.advance_partition_merge(full), Ok(Some(true)), "retry publishes retained candidate");
        let cursor = if full { job.state.full_merge_cursors[0] } else { job.state.free_merge_cursors[0] };
        assert_eq!(cursor, case["retried"]["sourceCursor"].as_u64().expect("retried cursor") as usize);
        assert_eq!(job.state.merge_candidate, None);
        let destination = if full { &job.state.merged_full } else { &job.state.merged_free };
        assert_eq!(destination.get(destination.len() - 1), Some(&candidate), "retry publishes the same fixture producer");
        let mut closed = false;
        for _ in 0..20_000 {
            let (terminal, released_items, released_bytes) = job.close_retained_step(MOUNTED_OWNER_PAGE_BYTES);
            assert!(released_items <= 1);
            assert!(released_bytes <= MOUNTED_OWNER_PAGE_BYTES);
            if terminal {
                closed = true;
                break;
            }
        }
        assert!(closed, "refusal fixture closes under the 4 KiB grant");
    }
}

#[test]
fn mounted_assembly_construction_is_retained_and_preserves_the_exact_model_owner() {
    let operation = test_operation(83);
    let model = Arc::new(cantilever_analysis_model(210e9, 0.02, 8e-6, 3.0, 7_850.0).0);
    let pointer = Arc::as_ptr(&model);
    let mut construction = AssemblyJobConstruction::new_owned(model, operation, 1);
    assert!(!construction.step_one().expect("first reservation opportunity"));
    let mut opportunities = 1;
    while !construction.step_one().expect("retained assembly construction") {
        opportunities += 1;
        assert!(opportunities < 4_096, "fixed construction reaches a finite terminal witness");
    }
    assert!(opportunities > 16, "validation, references, DOFs and partition outputs cannot collapse into one constructor turn");
    let job = construction.take_complete().expect("terminal construction returns one exact job");
    match &job.model {
        AnalysisModelOwner::Owned(owner) => assert_eq!(Arc::as_ptr(owner), pointer),
        AnalysisModelOwner::Borrowed(_) => panic!("mounted construction must preserve the owned model authority"),
        AnalysisModelOwner::Mounted(_) => panic!("dynamic construction cannot substitute mounted fixed authority"),
    }
    assert!(construction.take_complete().is_none(), "completion transfers exactly once");
}

#[test]
fn assembly_triplet_pages_cross_one_physical_page_without_extra_logical_partitions() {
    let nodes = vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [2.0, 0.0, 0.0] }];
    let mut model = MountedAnalysisModel::new();
    while !model.admit_node_one(nodes.len(), MOUNTED_ANALYSIS_BACKING_BYTES).unwrap().complete {}
    for node in nodes { model.push_node(node).unwrap(); }
    while !model.admit_element_one(50, MOUNTED_ANALYSIS_BACKING_BYTES).unwrap().complete {}
    for index in 0..50 {
        let element = BeamEb2 { id: format!("e{index}"), start: "a".into(), end: "b".into(), e: 210e9, area: 0.02, iy: 8e-6, density: 7_850.0 }.into();
        model.push_element(element).unwrap_or_else(|_| panic!("admitted mounted element"));
    }
    while !model.admit_support_one(1, MOUNTED_ANALYSIS_BACKING_BYTES).unwrap().complete {}
    let mut support = MountedAnalysisSupport::new("a".into());
    for dof in [Dof::Tx, Dof::Ty, Dof::Rz] { support.push_fixed(dof).unwrap(); }
    model.push_support(support).unwrap_or_else(|_| panic!("admitted mounted support"));
    let mut construction = AssemblyJobConstruction::new_mounted(model, test_operation(87), 1);
    let mut previous_paged_bytes = 0;
    let mut opportunities = 0;
    loop {
        let complete = construction.step_one().expect("paged assembly construction");
        opportunities += 1;
        let paged_bytes = construction.partitions.iter().map(|partition| partition.full.allocated_bytes() + partition.free.allocated_bytes()).sum::<usize>() + construction.merged_full.allocated_bytes() + construction.merged_free.allocated_bytes();
        assert!(paged_bytes.saturating_sub(previous_paged_bytes) <= MOUNTED_OWNER_PAGE_BYTES);
        previous_paged_bytes = paged_bytes;
        if complete {
            break;
        }
        assert!(opportunities < 16_384);
    }
    let mut job = construction.take_complete().expect("paged construction publishes one job");
    assert_eq!(job.state.partitions.len(), 1);
    let partition = &job.state.partitions[0];
    for owner in [&partition.full, &partition.free, &job.state.merged_full, &job.state.merged_free] {
        assert!(owner.capacity() >= 50 * 6 * 6);
        assert!(owner.allocated_bytes() > MOUNTED_OWNER_PAGE_BYTES);
    }
    for _ in 0..100_000 {
        let (terminal, released_items, released_bytes) = job.close_retained_step(MOUNTED_OWNER_PAGE_BYTES);
        assert!(released_items <= 1);
        assert!(released_bytes <= MOUNTED_OWNER_PAGE_BYTES);
        if terminal {
            return;
        }
    }
    panic!("paged assembly close did not reach its exact terminal witness");
}

/// 📚️ Final compressed row pointers, columns, and values retain separate fixed physical pages
/// while preserving one logical CSR and its exact matrix action.
#[test]
fn assembly_triplet_pages_build_final_csr_without_contiguous_arrays() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🔢️sparse/🧪️tests/🧫️fixtures/📚️paged-csr/🔣️.json")).expect("paged CSR fixture");
    let expected_order = fixture["order"].as_u64().expect("CSR order") as usize;
    let node_count = expected_order / 3;
    assert_eq!(node_count * 3, expected_order);
    let model = Arc::new(AnalysisModel {
        nodes: (0..node_count).map(|index| Node { id: format!("n{index}"), pos: [index as f64, 0.0, 0.0] }).collect(),
        elements: (1..node_count).map(|index| BeamEb2 { id: format!("e{index}"), start: format!("n{}", index - 1), end: format!("n{index}"), e: 210e9, area: 0.02, iy: 8e-6, density: 7_850.0 }.into()).collect(),
        supports: vec![Support { node_id: "n0".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
    });
    let plan = AssemblyPlan::prepare(&model).expect("beam CSR source plan");
    let source_grant = [
        model.nodes.capacity() * size_of::<Node>(),
        model.elements.capacity() * size_of::<Elements>(),
        model.supports.capacity() * size_of::<Support>(),
        plan.dof_map.order.capacity() * size_of::<(String, Dof)>(),
        plan.inv_perm.capacity() * size_of::<usize>(),
        plan.free_new.capacity() * size_of::<usize>(),
        plan.compact_of_new.capacity() * size_of::<Option<usize>>(),
        MOUNTED_OWNER_PAGE_BYTES,
    ].into_iter().max().unwrap();
    let n = plan.ndof;
    assert_eq!(n, expected_order);
    let entry_count = n.checked_mul(3).and_then(|count| count.checked_sub(2)).expect("tridiagonal entry count");
    assert_eq!(entry_count, fixture["entryCount"].as_u64().expect("CSR entry count") as usize);
    let mut merged_full = cold_paged_owner(entry_count).expect("tridiagonal paged triplets");
    let mut sequence = 0;
    for row in 0..n {
        for column in row.saturating_sub(1)..=(row + 1).min(n - 1) {
            let value = if row == column {
                if row == 0 || row + 1 == n {
                    1.0
                } else {
                    2.0
                }
            } else {
                -1.0
            };
            merged_full.push_reserved(AssemblyTriplet { sequence, row: row.try_into().expect("row index"), col: column.try_into().expect("column index"), value }).expect("tridiagonal triplet slot");
            sequence += 1;
        }
    }
    let operation = test_operation(88);
    let model_signature = assembly_model_signature(&model);
    let total_elements = model.elements.len();
    let job: AssemblyJob<'static> = AssemblyJob {
        model: AnalysisModelOwner::Owned(model),
        operation,
        model_signature,
        plan,
        state: AssemblyCheckpoint {
            stage: AssemblyJobStage::Complete,
            total_elements,
            element_cursor: total_elements,
            pending_build: None,
            pending: None,
            partitions: Vec::new(),
            full_merge_cursors: Vec::new(),
            free_merge_cursors: Vec::new(),
            merged_full,
            merged_free: PagedList::default(),
            checkpoint_due: false,
            preview_due: false,
            preview_cursor: 0,
            resume_target: 0,
            merge_scan_partition: 0,
            merge_candidate: None,
        },
        close_lane: 0,
        desk: NumericalOutcomeDesk::default(),
        model_close: AnalysisModelCloseCursor::default(),
    };
    let mut build = AssemblyCsrBuild::new(job).unwrap_or_else(|_| panic!("completed assembly enters CSR build"));
    let mut previous_bytes = 0;
    for _ in 0..100_000 {
        let complete = build.step_one().expect("paged CSR build");
        let current_bytes = build.row_counts.allocated_bytes() + build.indptr.allocated_bytes() + build.indices.allocated_bytes() + build.values.allocated_bytes();
        assert!(current_bytes.saturating_sub(previous_bytes) <= MOUNTED_OWNER_PAGE_BYTES);
        previous_bytes = current_bytes;
        if complete {
            break;
        }
    }
    let mut matrix = build.take_complete().expect("paged CSR completes");
    assert_eq!(matrix.entries_len(), entry_count);
    let [indptr_bytes, index_bytes, value_bytes] = matrix.physical_backing_bytes();
    assert!(indptr_bytes != 0);
    let page_bytes = fixture["physicalPageBytes"].as_u64().expect("CSR physical page") as usize;
    assert_eq!(page_bytes, MOUNTED_OWNER_PAGE_BYTES);
    assert!(index_bytes > page_bytes);
    assert!(value_bytes > page_bytes);
    let rule = &fixture["vectorRule"];
    let multiplier = rule["multiplier"].as_i64().expect("vector multiplier");
    let modulus = rule["modulus"].as_i64().expect("vector modulus");
    let offset = rule["offset"].as_i64().expect("vector offset");
    let divisor = rule["divisor"].as_f64().expect("vector divisor");
    let vector = VecD::from_vec((0..n).map(|index| ((index as i64 * multiplier) % modulus - offset) as f64 / divisor).collect());
    let action = matrix.mul_vec(&vector);
    for sample in fixture["actionSamples"].as_array().expect("CSR action samples") {
        let index = sample["index"].as_u64().expect("action index") as usize;
        let expected = sample["value"].as_f64().expect("action value");
        assert!((action.get(index) - expected).abs() <= 1e-12, "NumPy action sample {index}");
    }
    let action_l1: f64 = action.0.iter().map(|value| value.abs()).sum();
    assert!((action_l1 - fixture["actionL1"].as_f64().expect("action L1")).abs() <= 1e-10);
    let mut build_closed = false;
    for _ in 0..20_000 {
        let grant = if build.assembly.is_some() { source_grant } else { MOUNTED_OWNER_PAGE_BYTES };
        let (terminal, released_items, released_bytes) = build.close_step(grant);
        assert!(released_items <= 1);
        assert!(released_bytes <= grant);
        if terminal {
            build_closed = true;
            break;
        }
    }
    assert!(build_closed, "CSR construction closes under each source owner's exact physical grant");
    let mut closed = false;
    for _ in 0..20_000 {
        let (terminal, released_items, released_bytes) = matrix.close_step(MOUNTED_OWNER_PAGE_BYTES);
        assert!(released_items <= 1);
        assert!(released_bytes <= MOUNTED_OWNER_PAGE_BYTES);
        if terminal {
            closed = true;
            break;
        }
    }
    assert!(closed, "paged CSR closes under the exact physical grant");
}

#[test]
fn mounted_element_build_reserves_and_reclaims_one_exact_owner_per_turn() {
    let operation = test_operation(89);
    let model = Arc::new(cantilever_analysis_model(210e9, 0.02, 8e-6, 3.0, 7_850.0).0);
    let mut construction = AssemblyJobConstruction::new_owned(model, operation, 1);
    while !construction.step_one().expect("mounted construction") {}
    let mut job = construction.take_complete().expect("mounted assembly job");
    let mut sequence = 0;
    fn step_once(job: &mut AssemblyJob<'static>, operation: Operation, sequence: &mut u64) -> Seen {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), sequence, &mut sequence_receipt);
        seen_of(job.step(&mut context))
    }
    assert!(matches!(step_once(&mut job, operation, &mut sequence), Seen::Yield));
    let build = job.state.pending_build.as_ref().expect("first turn retains only the build shell");
    assert_eq!((build.indices_new.capacity(), build.positions.capacity(), build.stiffness.capacity()), (0, 0, 0));
    assert!(matches!(step_once(&mut job, operation, &mut sequence), Seen::Yield));
    let build = job.state.pending_build.as_ref().expect("second turn retains the fixed index page");
    assert!(build.indices_new.capacity() != 0);
    assert_eq!((build.positions.capacity(), build.stiffness.capacity()), (0, 0));
    let mut saw_positions = false;
    let mut saw_stiffness = false;
    let mut saw_reclaim = [false; 3];
    for _ in 0..128 {
        assert!(matches!(step_once(&mut job, operation, &mut sequence), Seen::Yield | Seen::Preview(_) | Seen::Checkpoint(_)));
        if let Some(build) = job.state.pending_build.as_ref() {
            saw_positions |= build.positions.capacity() != 0;
            saw_stiffness |= build.stiffness.capacity() != 0;
        }
        if let Some(pending) = job.state.pending.as_ref() {
            saw_reclaim[0] |= pending.complete && pending.reclaim_lane >= 1 && pending.stiffness.capacity() == 0;
            saw_reclaim[1] |= pending.complete && pending.reclaim_lane >= 2 && pending.indices_new.capacity() == 0;
            saw_reclaim[2] |= pending.complete && pending.reclaim_lane >= 3 && pending.positions.capacity() == 0;
        }
        if saw_reclaim.into_iter().all(|seen| seen) {
            break;
        }
    }
    assert!(saw_positions && saw_stiffness, "positions and stiffness are retained in distinct admitted stages");
    assert!(saw_reclaim.into_iter().all(|seen| seen), "each element backing is returned in its own later worker opportunity");
}

/// ⏱️ The product's interactive-step law over one driven job, not one wall-clock sample: a step above
/// the 8 ms ceiling is an overrun, and only `SUSTAINED_OVERRUN_QUARANTINE_STEPS` consecutive overruns
/// attribute the latency to the step's own work — the job runtime's `StepOverrunLedger` quarantines on
/// exactly that, while a lone overrun is the machine descheduling the thread (a peer build, a GC pause).
#[derive(Default)]
struct StepLatency {
    consecutive: u32,
    longest_run: u32,
    overruns: u32,
    worst: std::time::Duration,
}

impl StepLatency {
    fn admit(&mut self, elapsed: std::time::Duration) {
        self.worst = self.worst.max(elapsed);
        if semio_framework_job::interactive_step_contract_violated(u64::try_from(elapsed.as_micros()).unwrap_or(u64::MAX)) {
            self.consecutive += 1;
            self.overruns += 1;
            self.longest_run = self.longest_run.max(self.consecutive);
        } else {
            self.consecutive = 0;
        }
    }

    fn assert_interactive(&self, what: &str) {
        assert!(
            self.longest_run < semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS,
            "{what}: {} consecutive steps overran the 8 ms ceiling (worst {:?}, {} overruns in total)",
            self.longest_run,
            self.worst,
            self.overruns
        );
    }
}

fn finish_assembly_job<'model>(mut job: AssemblyJob<'model>, operation: Operation, fuel: u64) -> (UnfactoredSystem, Vec<AssemblyPreview>, StepLatency) {
    let mut sequence = 0;
    let mut previews = Vec::new();
    let mut latency = StepLatency::default();
    loop {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(fuel, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        let started = std::time::Instant::now();
        let outcome = seen_of(job.step(&mut context));
        latency.admit(started.elapsed());
        match outcome {
            Seen::Preview(pages) => previews.push(decode_value(&pages_bytes(&pages)).expect("assembly preview decodes")),
            Seen::Complete(_) => break,
            Seen::Checkpoint(_) | Seen::Yield => {}
            Seen::Cancelled | Seen::Fault(_) => panic!("assembly fixture must complete"),
        }
    }
    job.retire_outcome();
    (job.finish().expect("completed assembly yields matrices"), previews, latency)
}

/// 🧮️ Worker-local partition counts cannot alter triplet reduction order or matrix bytes.
#[test]
fn assembly_job_is_exact_across_partition_counts() {
    let model = axial_chain(24);
    let operation = test_operation(301);
    let (single, single_previews, _) = finish_assembly_job(AssemblyJob::new(&model, operation, 1).expect("single partition prepares"), operation, 5);
    let (fleet, fleet_previews, _) = finish_assembly_job(AssemblyJob::new(&model, operation, 7).expect("fleet partitions prepare"), operation, 3);
    assert_eq!(single.k_full_coo.to_dense().data, fleet.k_full_coo.to_dense().data);
    assert_eq!(single.k_ff_coo.to_dense().data, fleet.k_ff_coo.to_dense().data);
    assert_eq!(single_previews.iter().map(|preview| preview.assembled_element_ids.len()).sum::<usize>(), model.elements.len());
    assert_eq!(fleet_previews.iter().map(|preview| preview.assembled_element_ids.len()).sum::<usize>(), model.elements.len());
    assert!(single_previews.iter().all(|preview| preview.assembled_element_ids.len() <= 1), "every preview carries only the ids assembled since the previous one");
}

/// 💾️ A serialized element-boundary checkpoint resumes to the exact same merged matrices.
#[test]
fn assembly_job_checkpoint_resume_is_byte_stable() {
    let model = axial_chain(20);
    let operation = test_operation(302);
    let mut job = AssemblyJob::new(&model, operation, 4).expect("assembly prepares");
    let mut sequence = 0;
    let checkpoint = loop {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(4, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        match seen_of(job.step(&mut context)) {
            Seen::Checkpoint(pages) => break pages_bytes(&pages),
            outcome => assert!(!matches!(outcome, Seen::Complete(_)), "assembly must checkpoint before completing"),
        }
    };
    let resumed = AssemblyJob::from_checkpoint(&model, operation, &checkpoint).expect("assembly checkpoint restores");
    assert_eq!(resumed.checkpoint_bytes(), checkpoint);
    let (original, _, _) = finish_assembly_job(job, operation, 9);
    let (restored, _, _) = finish_assembly_job(resumed, operation, 2);
    assert_eq!(original.k_full_coo.to_dense().data, restored.k_full_coo.to_dense().data);
    assert_eq!(original.k_ff_coo.to_dense().data, restored.k_ff_coo.to_dense().data);
}

/// ⏱️ One-fuel adversarial stepping never sustains an overrun of the global eight-millisecond ceiling.
#[test]
fn assembly_job_one_fuel_steps_stay_below_eight_milliseconds() {
    let model = axial_chain(512);
    let operation = test_operation(303);
    let (_, previews, latency) = finish_assembly_job(AssemblyJob::new(&model, operation, 8).expect("assembly prepares"), operation, 1);
    assert!(!previews.is_empty());
    latency.assert_interactive("one-fuel assembly");
}

/// 🧪️ Mounted fixed-family cell cursors preserve numerical identity and every interruption seam.
#[test]
fn p6h_element_stiffness_microcursor_deadline_stale_cancel_close_and_stage_laws() {
    use semio_s_fem_elements2d::{PlaneKind, Tri3Cst};

    fn exercise(model: AnalysisModel, expected: Vec<f64>, operation: Operation) {
        let mut construction = AssemblyJobConstruction::new_owned(Arc::new(model), operation, 1);
        while !construction.step_one().expect("owned assembly construction") {}
        let mut job = construction.take_complete().expect("owned construction transfers mounted job");
        assert!(matches!(&job.model, AnalysisModelOwner::Owned(_)));
        let required = [
            PendingElementBuildStage::Indices,
            PendingElementBuildStage::PublishIndex,
            PendingElementBuildStage::Positions,
            PendingElementBuildStage::PublishPosition,
            PendingElementBuildStage::ReferenceQuadraturePoint,
            PendingElementBuildStage::ShapeFunctionDerivativeScalar,
            PendingElementBuildStage::JacobianCell,
            PendingElementBuildStage::DeterminantInverseCell,
            PendingElementBuildStage::StrainDisplacementCell,
            PendingElementBuildStage::ConstitutiveCell,
            PendingElementBuildStage::LocalStiffnessMultiplyCell,
            PendingElementBuildStage::BodyTractionLoadCell,
            PendingElementBuildStage::LocalToGlobalTripletCell,
        ];
        let mut seen = HashSet::new();
        let mut sequence = 0;
        let mut maximum_micros = 0;
        for _ in 0..1_024 {
            let before = job.state.pending_build.as_ref().map(|build| (build.stage, build.scalar_cursor, build.lookup_cursor, build.lookup_match, build.indices_new.len(), build.positions.len(), build.stiffness.len()));
            if let Some((stage, ..)) = before.filter(|(stage, ..)| required.contains(stage)) {
                seen.insert(stage);
                let mut sequence_receipt = RetainedCloneProgress::default();
                let mut deadline = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 0, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                assert_eq!(seen_of(job.step(&mut deadline)), Seen::Yield);
                assert_eq!(job.state.pending_build.as_ref().map(|build| (build.stage, build.scalar_cursor, build.lookup_cursor, build.lookup_match, build.indices_new.len(), build.positions.len(), build.stiffness.len())), before);
                drop(deadline);
                let mut stale =
                    StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                assert!(matches!(seen_of(job.step(&mut stale)), Seen::Fault(_)));
                assert_eq!(job.state.pending_build.as_ref().map(|build| (build.stage, build.scalar_cursor, build.lookup_cursor, build.lookup_match, build.indices_new.len(), build.positions.len(), build.stiffness.len())), before);
                let token = semio_framework_job::root_cancel_token();
                token.cancel_now();
                let mut sequence_receipt = RetainedCloneProgress::default();
                drop(stale);
                let mut cancelled = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX, TEST_GRANT), token, || Some(0), &mut sequence, &mut sequence_receipt);
                assert_eq!(seen_of(job.step(&mut cancelled)), Seen::Cancelled);
                assert_eq!(job.state.pending_build.as_ref().map(|build| (build.stage, build.scalar_cursor, build.lookup_cursor, build.lookup_match, build.indices_new.len(), build.positions.len(), build.stiffness.len())), before);
            }
            let started = std::time::Instant::now();
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            assert!(matches!(seen_of(job.step(&mut context)), Seen::Yield | Seen::Preview(_) | Seen::Checkpoint(_)));
            maximum_micros = maximum_micros.max(started.elapsed().as_micros());
            if job.state.pending.is_some() {
                break;
            }
        }
        assert!(required.into_iter().all(|stage| seen.contains(&stage)));
        let actual = &job.state.pending.as_ref().expect("mounted stiffness candidate published").stiffness;
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.iter().zip(expected) {
            assert!((actual - expected).abs() <= expected.abs().max(1.0) * 1e-12, "mounted stiffness mismatch: {actual} vs {expected}");
        }
        assert!(maximum_micros < 8_000, "fixed mounted cell exceeded timing ceiling: {maximum_micros} us");
        let mut close_turns = 0;
        loop {
            close_turns += 1;
            let (terminal, released_items, _) = job.close_retained_step(usize::MAX);
            assert!(released_items <= 1);
            if terminal {
                break;
            }
            assert!(close_turns < 20_000);
        }
        assert!(job.close_lane > 11);
    }

    let bar = Bar2 { id: "bar".into(), start: "a".into(), end: "b".into(), e: 210e9, area: 0.02, density: 0.0 };
    let bar_context = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.0]] };
    let bar_expected = bar.stiffness_global(&bar_context).data;
    exercise(AnalysisModel { nodes: vec![Node { id: "a".into(), pos: bar_context.positions[0] }, Node { id: "b".into(), pos: bar_context.positions[1] }], elements: vec![bar.into()], supports: Vec::new() }, bar_expected, test_operation(304));

    let beam = BeamEb2 { id: "beam".into(), start: "a".into(), end: "b".into(), e: 205e9, area: 0.015, iy: 7e-6, density: 0.0 };
    let beam_context = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 2.0, 0.0]] };
    let beam_expected = beam.stiffness_global(&beam_context).data;
    exercise(AnalysisModel { nodes: vec![Node { id: "a".into(), pos: beam_context.positions[0] }, Node { id: "b".into(), pos: beam_context.positions[1] }], elements: vec![beam.into()], supports: Vec::new() }, beam_expected, test_operation(305));

    let triangle = Tri3Cst { id: "tri".into(), nodes: ["a".into(), "b".into(), "c".into()], e: 70e9, nu: 0.27, thickness: 0.2, kind: PlaneKind::Stress, density: 0.0 };
    let triangle_context = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.25, 1.5, 0.0]] };
    let triangle_expected = triangle.stiffness_global(&triangle_context).data;
    exercise(
        AnalysisModel {
            nodes: vec![Node { id: "a".into(), pos: triangle_context.positions[0] }, Node { id: "b".into(), pos: triangle_context.positions[1] }, Node { id: "c".into(), pos: triangle_context.positions[2] }],
            elements: vec![triangle.into()],
            supports: Vec::new(),
        },
        triangle_expected,
        test_operation(306),
    );
}

/// 🔀️ Owned lookup and partition-min cursors interrupt and replay without hidden scans.
#[test]
fn p6h_owned_assembly_lookup_partition_scan_transfer_interrupt_replay_and_timing() {
    fn model() -> AnalysisModel {
        let nodes = (0..=4).map(|index| Node { id: format!("n{index}"), pos: [index as f64, (index % 2) as f64 * 0.25, 0.0] }).collect::<Vec<_>>();
        let elements = (0..4).map(|index| Bar2 { id: format!("b{index}"), start: format!("n{index}"), end: format!("n{}", index + 1), e: 200e9, area: 0.01, density: 0.0 }.into()).collect();
        AnalysisModel { nodes, elements, supports: Vec::new() }
    }

    fn run(operation: Operation) -> (Vec<f64>, Vec<f64>, u128) {
        let mut construction = AssemblyJobConstruction::new_owned(Arc::new(model()), operation, 4);
        while !construction.step_one().expect("owned assembly construction") {}
        let mut job = construction.take_complete().expect("owned assembly job");
        let mut sequence = 0;
        let mut maximum_micros = 0;
        for _ in 0..100_000 {
            let merge = matches!(job.state.stage, AssemblyJobStage::MergeFull | AssemblyJobStage::MergeFree);
            let lookup =
                job.state.pending_build.as_ref().is_some_and(|build| matches!(build.stage, PendingElementBuildStage::Indices | PendingElementBuildStage::PublishIndex | PendingElementBuildStage::Positions | PendingElementBuildStage::PublishPosition));
            if merge || lookup {
                let before = (
                    job.state.stage,
                    job.state.merge_scan_partition,
                    job.state.merge_candidate,
                    job.state.full_merge_cursors.clone(),
                    job.state.free_merge_cursors.clone(),
                    job.state.pending_build.as_ref().map(|build| (build.stage, build.scalar_cursor, build.lookup_cursor, build.lookup_match)),
                );
                let mut sequence_receipt = RetainedCloneProgress::default();
                let mut deadline = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 0, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                assert_eq!(seen_of(job.step(&mut deadline)), Seen::Yield);
                assert_eq!(
                    (
                        job.state.stage,
                        job.state.merge_scan_partition,
                        job.state.merge_candidate,
                        job.state.full_merge_cursors.clone(),
                        job.state.free_merge_cursors.clone(),
                        job.state.pending_build.as_ref().map(|build| (build.stage, build.scalar_cursor, build.lookup_cursor, build.lookup_match)),
                    ),
                    before
                );
                drop(deadline);
                let mut stale =
                    StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                assert!(matches!(seen_of(job.step(&mut stale)), Seen::Fault(_)));
                let token = semio_framework_job::root_cancel_token();
                token.cancel_now();
                let mut sequence_receipt = RetainedCloneProgress::default();
                drop(stale);
                let mut cancelled = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX, TEST_GRANT), token, || Some(0), &mut sequence, &mut sequence_receipt);
                assert_eq!(seen_of(job.step(&mut cancelled)), Seen::Cancelled);
            }
            let started = std::time::Instant::now();
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            if matches!(seen_of(job.step(&mut context)), Seen::Complete(_)) {
                maximum_micros = maximum_micros.max(started.elapsed().as_micros());
                let system = job.finish().expect("owned assembly completes");
                return (system.k_full_coo.to_dense().data, system.k_ff_coo.to_dense().data, maximum_micros);
            }
            maximum_micros = maximum_micros.max(started.elapsed().as_micros());
        }
        panic!("owned assembly cursor law did not complete")
    }

    let first = run(test_operation(307));
    let second = run(test_operation(307));
    assert_eq!((first.0, first.1), (second.0, second.1));
    assert!(first.2.max(second.2) < 8_000);
}

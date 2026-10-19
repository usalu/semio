/// 🍳️ Assembly page and mounted admission laws: triplet pages, mounted fixture owners, inline closes and merge refusal.

use semio_s_fem_analyses_model::MountedAnalysisModel;
use semio_s_fem_analyses_model::MountedAnalysisSupport;
use semio_s_fem_analyses_model::MountedAnalysisFault;
use semio_framework_value_paged_list::list::PagedList;
use semio_s_fem_analyses_assembly::AssemblyJobConstruction;
use semio_s_fem_analyses_assembly::AssemblyJobStage;
use semio_s_fem_analyses_assembly::AssemblyCsrBuild;
use semio_s_fem_sparse::Csr;
use semio_s_fem_analyses_model::MountedAnalysisAdmission;
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

fn assembly_operation(id: u64) -> Operation {
    Operation::new(semio_framework_job::OperationId(id), semio_framework_job::RevisionId(7), semio_framework_job::Generation(3), 11)
}

/// ⛽️ Assembly control, publication, and work transitions share the same exact grant admission.
#[test]
fn assembly_triplet_pages_control_transitions_preserve_state_until_granted() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⛽️step-grant/🔣️.json")).expect("assembly step grants");
    for (index, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let operation = assembly_operation(940 + index as u64);
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
    serde_json::from_str(include_str!("../../🧫️fixtures/📦️physical-owners/🔣️.json")).expect("assembly physical fixture")
}

/// 🧱 The mounted model moves as small metadata while payload pages require explicit admission.
#[test]
fn assembly_triplet_pages_mounted_model_has_no_inline_payload_arrays() {
    let fixture = assembly_physical_fixture();
    let bytes = size_of::<MountedAnalysisModel>();
    assert!(bytes <= fixture["mountedModel"]["maximumInlineBytes"].as_u64().unwrap() as usize);
}


type MountedAdmissionQuery = fn(&MountedAnalysisModel, usize) -> Result<Option<usize>, MountedAnalysisFault>;
type MountedAdmissionStep = fn(&mut MountedAnalysisModel, usize, usize) -> Result<MountedAnalysisAdmission, MountedAnalysisFault>;

fn mounted_fixture_owners() -> [(&'static str, MountedAdmissionQuery, MountedAdmissionStep); 3] {
    [
        ("node", MountedAnalysisModel::next_node_allocation_bytes, MountedAnalysisModel::admit_node_one),
        ("element", MountedAnalysisModel::next_element_allocation_bytes, MountedAnalysisModel::admit_element_one),
        ("support", MountedAnalysisModel::next_support_allocation_bytes, MountedAnalysisModel::admit_support_one),
    ]
}

fn mounted_fixture_coordinates(model: &MountedAnalysisModel, kind: &str) -> (usize, usize, usize) {
    match kind {
        "node" => (model.nodes.admitted, model.nodes.values.len(), model.nodes.values.capacity()),
        "element" => (model.elements.admitted, model.elements.values.len(), model.elements.values.capacity()),
        "support" => (model.supports.admitted, model.supports.values.len(), model.supports.values.capacity()),
        _ => unreachable!(),
    }
}

fn close_mounted_fixture(model: &mut MountedAnalysisModel) -> (usize, bool) {
    let mut released = 0;
    for _ in 0..4_096 {
        let before = model.physical_backing_bytes().into_iter().sum::<usize>();
        let step = model.close_step(MOUNTED_ANALYSIS_BACKING_BYTES);
        let after = model.physical_backing_bytes().into_iter().sum::<usize>();
        released += step.2;
        assert!(step.1 <= 1 && step.2 <= MOUNTED_ANALYSIS_BACKING_BYTES);
        assert_eq!(before, after + step.2);
        if step.0 { return (released, model.terminal_is_empty()); }
    }
    (released, false)
}

/// 🎟️ Every model backing is separately queried, admitted and physically retired under exact grants.
#[test]
fn assembly_triplet_pages_mounted_model_admits_one_backing_and_preserves_refused_grants() {
    let fixture = assembly_physical_fixture();
    for (kind, query, admit) in mounted_fixture_owners() {
        for target_name in fixture["mountedModel"]["targets"].as_array().unwrap() {
            let maximum = fixture["mountedModel"]["logicalLimits"][kind].as_u64().unwrap() as usize;
            let target = match target_name.as_str().unwrap() { "empty" => 0, "one" => 1, "maximum" => maximum, _ => unreachable!() };
            let mut model = MountedAnalysisModel::new();
            assert_eq!(model.physical_backing_bytes(), [0; 3]);
            let mut allocated = 0;
            let mut allocations = 0;
            let mut complete = false;
            for _ in 0..4_096 {
                let before = model.physical_backing_bytes();
                let coordinates = mounted_fixture_coordinates(&model, kind);
                let demand = query(&model, target).expect("exact model allocation demand");
                if let Some(bytes) = demand {
                    for refused in [0, bytes.saturating_sub(1)] {
                        assert_eq!(admit(&mut model, target, refused), Ok(MountedAnalysisAdmission::default()));
                        assert_eq!(model.physical_backing_bytes(), before);
                        assert_eq!(mounted_fixture_coordinates(&model, kind), coordinates);
                    }
                }
                let step = admit(&mut model, target, demand.unwrap_or(0)).expect("model admission");
                let after = model.physical_backing_bytes();
                assert_eq!(after.into_iter().sum::<usize>(), before.into_iter().sum::<usize>() + step.allocated_bytes);
                assert!(step.allocated_bytes <= MOUNTED_ANALYSIS_BACKING_BYTES);
                if step.allocated_bytes != 0 {
                    assert_eq!(mounted_fixture_coordinates(&model, kind).0, coordinates.0);
                    allocations += 1;
                }
                allocated += step.allocated_bytes;
                if step.complete { complete = true; break; }
            }
            assert!(complete);
            assert_eq!(mounted_fixture_coordinates(&model, kind).0, target);
            assert_eq!(mounted_fixture_coordinates(&model, kind).1, 0);
            for _ in 0..target {
                match kind {
                    "node" => model.push_node(Node { id: String::new(), pos: [0.0; 3] }).unwrap(),
                    "element" => model.push_element(BeamEb2 { id: String::new(), start: String::new(), end: String::new(), e: 1.0, area: 1.0, iy: 1.0, density: 1.0 }.into()).unwrap_or_else(|_| panic!("admitted element")),
                    "support" => model.push_support(MountedAnalysisSupport::new(String::new())).unwrap_or_else(|_| panic!("admitted support")),
                    _ => unreachable!(),
                }
            }
            let before_close = model.physical_backing_bytes();
            for _ in 0..1_024 {
                let step = model.close_step(0);
                assert_eq!(step.2, 0);
                if step.0 || step.1 == 0 { break; }
            }
            assert_eq!(model.physical_backing_bytes(), before_close);
            let (released, terminal) = close_mounted_fixture(&mut model);
            eprintln!("mounted model {kind}/{target_name}: allocations={allocations}, allocated={allocated}, released={released}, terminal={terminal}");
            assert_eq!(allocated, released);
            assert!(terminal);
            if target == maximum && kind != "support" { assert!(allocated > MOUNTED_ANALYSIS_BACKING_BYTES); }
        }
    }
}

/// 🛑 A partially admitted model retains its first fault and every real backing for cancellation.
#[test]
fn assembly_triplet_pages_mounted_model_partial_admission_and_fault_retain_every_backing() {
    let fixture = assembly_physical_fixture();
    for (kind, _, admit) in mounted_fixture_owners() {
        let maximum = fixture["mountedModel"]["logicalLimits"][kind].as_u64().unwrap() as usize;
        for cutoff in 1..=5 {
            let mut model = MountedAnalysisModel::new();
            let mut allocated = 0;
            for _ in 0..cutoff {
                allocated += admit(&mut model, maximum, MOUNTED_ANALYSIS_BACKING_BYTES).expect("partial model admission").allocated_bytes;
            }
            let before = model.physical_backing_bytes();
            let coordinates = mounted_fixture_coordinates(&model, kind);
            let expected = Err(MountedAnalysisFault::Capacity { requested: maximum + 1, maximum });
            assert_eq!(admit(&mut model, maximum + 1, MOUNTED_ANALYSIS_BACKING_BYTES), expected);
            assert_eq!(admit(&mut model, maximum, MOUNTED_ANALYSIS_BACKING_BYTES), expected);
            assert_eq!(model.physical_backing_bytes(), before);
            assert_eq!(mounted_fixture_coordinates(&model, kind), coordinates);
            let (released, terminal) = close_mounted_fixture(&mut model);
            assert_eq!(allocated, released);
            assert!(terminal);
        }
    }
}

fn assembly_physical_empty_build(matrix: Option<Csr>) -> AssemblyCsrBuild {
    AssemblyCsrBuild {
        assembly: None,
        csr: CsrTripletBuild {
        entries: PagedList::default(),
        stage: CsrTripletBuildStage::Complete,
        sort: HeapSortCursor::new(0),
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
        },
    }
}

/// 🪶 Clearing an inline assembly record has no physical backing release of its own.
#[test]
fn assembly_triplet_pages_inline_job_close_reports_no_struct_bytes() {
    static EMPTY_MODEL: AnalysisModel = AnalysisModel { nodes: Vec::new(), elements: Vec::new(), supports: Vec::new() };
    let fixture = assembly_physical_fixture();
    let model = Arc::new(cantilever_analysis_model(210e9, 0.02, 8e-6, 3.0, 7_850.0).0);
    let mut construction = AssemblyJobConstruction::new_owned(model, assembly_operation(956), 1);
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

fn merge_fixture_triplet(value: &serde_json::Value) -> SparseTriplet {
    SparseTriplet {
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔀️merge-refusal/🔣️.json")).expect("merge refusal fixture");
    for (case_index, case) in fixture["cases"].as_array().expect("merge refusal cases").iter().enumerate() {
        let full = case["lane"].as_str().expect("merge lane") == "full";
        let candidate = merge_fixture_triplet(&case["candidate"]);
        let model = Arc::new(cantilever_analysis_model(210e9, 0.02, 8e-6, 3.0, 7_850.0).0);
        let mut construction = AssemblyJobConstruction::new_owned(model, assembly_operation(900 + case_index as u64), 1);
        while !construction.step_one().expect("merge refusal construction") {}
        let mut job = construction.take_complete().expect("merge refusal job");
        let partition = &mut job.state.partitions[0];
        let source = if full { &mut partition.full } else { &mut partition.free };
        source.push_reserved(candidate).expect("source candidate fits admitted partition");
        let destination = if full { &mut job.state.merged_full } else { &mut job.state.merged_free };
        let filler = SparseTriplet { sequence: 0, row: 0, col: 0, value: 0.0 };
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
    let operation = assembly_operation(83);
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
    let mut construction = AssemblyJobConstruction::new_mounted(model, assembly_operation(87), 1);
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

/// 🗂️ The neutral NumPy matrix order crosses DOF, index and compact pages with exact funded admission.
#[test]
fn assembly_order_pages_admit_exact_grants_and_retire_every_original_backing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🔢️sparse/🧪️tests/🧫️fixtures/📚️paged-csr/🔣️.json")).unwrap();
    let order = fixture["order"].as_u64().unwrap() as usize;
    let model = Arc::new(AnalysisModel {
        nodes: (0..order / 3).map(|index| Node { id: format!("n{index}"), pos: [index as f64, 0.0, 0.0] }).collect(),
        elements: (1..order / 3).map(|index| BeamEb2 { id: format!("e{index}"), start: format!("n{}", index - 1), end: format!("n{index}"), e: 210e9, area: 0.02, iy: 8e-6, density: 7_850.0 }.into()).collect(),
        supports: vec![Support { node_id: "n0".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
    });
    let pointer = Arc::as_ptr(&model);
    let mut construction = AssemblyJobConstruction::new_owned(model, assembly_operation(88), 1);
    for _ in 0..1_000_000 {
        if construction.stage == AssemblyConstructionStage::ReserveDofs && construction.plan.dof_map.order.capacity() < order * 2 {
            let before = (construction.stage, construction.plan.dof_map.order.capacity(), construction.plan.dof_map.order.allocated_bytes());
            let copy = construction.plan.dof_map.order.next_capacity_copy_byte_demand(order * 2).unwrap();
            let capacity = construction.plan.dof_map.order.next_capacity_allocation_bytes(order * 2).unwrap().unwrap();
            let depth = construction.plan.dof_map.order.next_capacity_depth_demand(order * 2).unwrap();
            for grant in [
                RetainedCloneGrant { maximum_items: 0, ..TEST_GRANT },
                RetainedCloneGrant { maximum_copy_bytes: copy - 1, ..TEST_GRANT },
                RetainedCloneGrant { maximum_capacity_bytes: capacity - 1, ..TEST_GRANT },
                RetainedCloneGrant { maximum_depth: depth - 1, ..TEST_GRANT },
            ] {
                assert_eq!(construction.step_one_funded(grant).unwrap(), (false, RetainedCloneProgress::default()));
                assert_eq!((construction.stage, construction.plan.dof_map.order.capacity(), construction.plan.dof_map.order.allocated_bytes()), before);
            }
            let (_, receipt) = construction.step_one_funded(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_depth: depth, ..TEST_GRANT }).unwrap();
            assert_eq!(receipt.retained_capacity_bytes, capacity);
            assert_eq!(receipt.copied_bytes, copy);
            assert_eq!(receipt.copied_items, 1);
        } else if construction.step_one_funded(TEST_GRANT).unwrap().0 { break; }
    }
    let mut job = construction.take_complete().expect("paged assembly order terminates");
    assert_eq!(job.plan.ndof, order);
    for bytes in [job.plan.dof_map.order.allocated_bytes(), job.plan.dof_map.slots.allocated_bytes(), job.plan.inv_perm.allocated_bytes(), job.plan.compact_of_new.allocated_bytes()] { assert!(bytes > MOUNTED_OWNER_PAGE_BYTES); }
    for index in 0..order / 3 {
        for (axis, dof) in [Dof::Tx, Dof::Ty, Dof::Rz].into_iter().enumerate() {
            let full = index * 3 + axis;
            assert_eq!(job.visual_equation_indices(&format!("n{index}"), dof), Some((full, full.checked_sub(3))));
        }
    }
    assert!(matches!(&job.model, AnalysisModelOwner::Owned(model) if Arc::as_ptr(model) == pointer));
    let mut preparation_closed = false;
    for _ in 0..100_000 {
        let (terminal, items, bytes) = construction.close_step(MOUNTED_OWNER_PAGE_BYTES);
        assert!(items <= 1 && bytes <= MOUNTED_OWNER_PAGE_BYTES);
        if terminal { preparation_closed = true; break; }
    }
    assert!(preparation_closed && construction.constrained_old.terminal_is_empty());
    assert_eq!(job.close_retained_step(0), (false, 0, 0));
    let mut closed = false;
    for _ in 0..1_000_000 {
        let (terminal, items, bytes) = job.close_retained_step(1 << 20);
        assert!(items <= 1);
        if terminal { closed = true; break; }
        if job.close_lane <= 10 { assert!(bytes <= MOUNTED_OWNER_PAGE_BYTES); }
    }
    assert!(closed, "paged order releases all original backing");
    assert!(job.plan.dof_map.order.terminal_is_empty() && job.plan.dof_map.slots.terminal_is_empty() && job.plan.inv_perm.terminal_is_empty() && job.plan.free_new.terminal_is_empty() && job.plan.compact_of_new.terminal_is_empty());
    eprintln!("[DEBUG] FEM assembly paged order={order}, exact item/copy/capacity/depth refusals preserved source={pointer:p}, closed={closed}");
}

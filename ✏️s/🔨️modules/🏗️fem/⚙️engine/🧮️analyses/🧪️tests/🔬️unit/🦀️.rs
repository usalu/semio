use super::*;
use crate::elements2d::{Bar2, BeamEb2};
use crate::model::{solve_linear_static, AxialSpring, Model};
use crate::numerical_testkit::payload_bytes;

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
fn mounted_element_build_reserves_and_reclaims_one_exact_owner_per_turn() {
    let operation = assembly_operation(89);
    let model = Arc::new(cantilever_analysis_model(210e9, 0.02, 8e-6, 3.0, 7_850.0).0);
    let mut construction = AssemblyJobConstruction::new_owned(model, operation, 1);
    while !construction.step_one().expect("mounted construction") {}
    let mut job = construction.take_complete().expect("mounted assembly job");
    let mut sequence = 0;
    fn step_once(job: &mut AssemblyJob<'static>, operation: Operation, sequence: &mut u64) -> StepOutcome {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), sequence);
        job.step(&mut context)
    }
    assert!(matches!(step_once(&mut job, operation, &mut sequence), StepOutcome::Yield));
    let build = job.state.pending_build.as_ref().expect("first turn retains only the build shell");
    assert_eq!((build.indices_new.capacity(), build.positions.capacity(), build.stiffness.capacity()), (0, 0, 0));
    assert!(matches!(step_once(&mut job, operation, &mut sequence), StepOutcome::Yield));
    let build = job.state.pending_build.as_ref().expect("second turn retains the fixed index page");
    assert!(build.indices_new.capacity() != 0);
    assert_eq!((build.positions.capacity(), build.stiffness.capacity()), (0, 0));
    let mut saw_positions = false;
    let mut saw_stiffness = false;
    let mut saw_reclaim = [false; 3];
    for _ in 0..128 {
        assert!(matches!(step_once(&mut job, operation, &mut sequence), StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_)));
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

#[test]
fn mounted_element_stiffness_observes_before_admit_and_retires_rejected_backing() {
    let mut rejected = Vec::<f64>::new();
    assert!(!reserve_exact_owner_page(&mut rejected, MOUNTED_OWNER_PAGE_BYTES / size_of::<f64>() + 1));
    let observed = rejected.capacity() * size_of::<f64>();
    let pointer = rejected.as_ptr();
    assert!(observed > MOUNTED_OWNER_PAGE_BYTES);
    assert_eq!(close_vec_owner_step(&mut rejected, observed - 1), Err(()));
    assert_eq!(rejected.as_ptr(), pointer, "an interrupted close preserves the exact rejected backing");
    assert_eq!(close_vec_owner_step(&mut rejected, observed), Ok(Some((1, observed))), "the exact rejected allocation retires through the retained close helper");
    assert_eq!(rejected.capacity(), 0);
}

fn finish_assembly_job<'model>(mut job: AssemblyJob<'model>, operation: Operation, fuel: u64) -> (UnfactoredSystem, Vec<AssemblyPreview>, u128) {
    let mut sequence = 0;
    let mut previews = Vec::new();
    let mut max_step_micros = 0;
    loop {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(fuel, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        let started = std::time::Instant::now();
        let outcome = job.step(&mut context);
        max_step_micros = max_step_micros.max(started.elapsed().as_micros());
        match outcome {
            StepOutcome::PreviewReady(bytes) => previews.push(decode_value(&payload_bytes(bytes)).expect("assembly preview decodes")),
            StepOutcome::Complete(_) => break,
            StepOutcome::Yield | StepOutcome::CheckpointReady(_) => {}
            StepOutcome::Cancelled | StepOutcome::Fault(_) => panic!("assembly fixture must complete"),
        }
    }
    (job.finish().expect("completed assembly yields matrices"), previews, max_step_micros)
}

/// 🧮️ Worker-local partition counts cannot alter triplet reduction order or matrix bytes.
#[test]
fn assembly_job_is_exact_across_partition_counts() {
    let model = axial_chain(24);
    let operation = assembly_operation(301);
    let (single, single_previews, _) = finish_assembly_job(AssemblyJob::new(&model, operation, 1).expect("single partition prepares"), operation, 5);
    let (fleet, fleet_previews, _) = finish_assembly_job(AssemblyJob::new(&model, operation, 7).expect("fleet partitions prepare"), operation, 3);
    assert_eq!(single.k_full_coo.to_dense().data, fleet.k_full_coo.to_dense().data);
    assert_eq!(single.k_ff_coo.to_dense().data, fleet.k_ff_coo.to_dense().data);
    assert_eq!(single_previews.last().expect("single publishes marks").assembled_element_ids.len(), model.elements.len());
    assert_eq!(fleet_previews.last().expect("fleet publishes marks").assembled_element_ids.len(), model.elements.len());
}

/// 💾️ A serialized element-boundary checkpoint resumes to the exact same merged matrices.
#[test]
fn assembly_job_checkpoint_resume_is_byte_stable() {
    let model = axial_chain(20);
    let operation = assembly_operation(302);
    let mut job = AssemblyJob::new(&model, operation, 4).expect("assembly prepares");
    let mut sequence = 0;
    let checkpoint = loop {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(4, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
            break payload_bytes(checkpoint.state);
        }
    };
    let resumed = AssemblyJob::from_checkpoint(&model, operation, &checkpoint).expect("assembly checkpoint restores");
    assert_eq!(resumed.checkpoint_bytes(), checkpoint);
    let (original, _, _) = finish_assembly_job(job, operation, 9);
    let (restored, _, _) = finish_assembly_job(resumed, operation, 2);
    assert_eq!(original.k_full_coo.to_dense().data, restored.k_full_coo.to_dense().data);
    assert_eq!(original.k_ff_coo.to_dense().data, restored.k_ff_coo.to_dense().data);
}

/// ⏱️ One-fuel adversarial stepping keeps every callback below the global eight-millisecond ceiling.
#[test]
fn assembly_job_one_fuel_steps_stay_below_eight_milliseconds() {
    let model = axial_chain(512);
    let operation = assembly_operation(303);
    let (_, previews, max_step_micros) = finish_assembly_job(AssemblyJob::new(&model, operation, 8).expect("assembly prepares"), operation, 1);
    assert!(!previews.is_empty());
    assert!(max_step_micros < 8_000, "slowest assembly step was {max_step_micros} us");
}

/// 🧪️ Mounted fixed-family cell cursors preserve numerical identity and every interruption seam.
#[test]
fn p6h_element_stiffness_microcursor_deadline_stale_cancel_close_and_stage_laws() {
    use crate::elements2d::{PlaneKind, Tri3Cst};

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
                let mut deadline = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 0), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
                assert_eq!(job.step(&mut deadline), StepOutcome::Yield);
                assert_eq!(job.state.pending_build.as_ref().map(|build| (build.stage, build.scalar_cursor, build.lookup_cursor, build.lookup_match, build.indices_new.len(), build.positions.len(), build.stiffness.len())), before);
                let mut stale =
                    StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
                assert!(matches!(job.step(&mut stale), StepOutcome::Fault(_)));
                assert_eq!(job.state.pending_build.as_ref().map(|build| (build.stage, build.scalar_cursor, build.lookup_cursor, build.lookup_match, build.indices_new.len(), build.positions.len(), build.stiffness.len())), before);
                let token = semio_framework_job::root_cancel_token();
                semio_framework_async::block_on(token.cancel());
                let mut cancelled = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), token, || Some(0), &mut sequence);
                assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
                assert_eq!(job.state.pending_build.as_ref().map(|build| (build.stage, build.scalar_cursor, build.lookup_cursor, build.lookup_match, build.indices_new.len(), build.positions.len(), build.stiffness.len())), before);
            }
            let started = std::time::Instant::now();
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            assert!(matches!(job.step(&mut context), StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_)));
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
            let (terminal, released_items, _) = job.close_step(usize::MAX);
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
    exercise(AnalysisModel { nodes: vec![Node { id: "a".into(), pos: bar_context.positions[0] }, Node { id: "b".into(), pos: bar_context.positions[1] }], elements: vec![bar.into()], supports: Vec::new() }, bar_expected, assembly_operation(304));

    let beam = BeamEb2 { id: "beam".into(), start: "a".into(), end: "b".into(), e: 205e9, area: 0.015, iy: 7e-6, density: 0.0 };
    let beam_context = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 2.0, 0.0]] };
    let beam_expected = beam.stiffness_global(&beam_context).data;
    exercise(AnalysisModel { nodes: vec![Node { id: "a".into(), pos: beam_context.positions[0] }, Node { id: "b".into(), pos: beam_context.positions[1] }], elements: vec![beam.into()], supports: Vec::new() }, beam_expected, assembly_operation(305));

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
        assembly_operation(306),
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
                let mut deadline = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 0), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
                assert_eq!(job.step(&mut deadline), StepOutcome::Yield);
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
                let mut stale =
                    StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
                assert!(matches!(job.step(&mut stale), StepOutcome::Fault(_)));
                let token = semio_framework_job::root_cancel_token();
                semio_framework_async::block_on(token.cancel());
                let mut cancelled = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), token, || Some(0), &mut sequence);
                assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
            }
            let started = std::time::Instant::now();
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            if matches!(job.step(&mut context), StepOutcome::Complete(_)) {
                maximum_micros = maximum_micros.max(started.elapsed().as_micros());
                let system = job.finish().expect("owned assembly completes");
                return (system.k_full_coo.to_dense().data, system.k_ff_coo.to_dense().data, maximum_micros);
            }
            maximum_micros = maximum_micros.max(started.elapsed().as_micros());
        }
        panic!("owned assembly cursor law did not complete")
    }

    let first = run(assembly_operation(307));
    let second = run(assembly_operation(307));
    assert_eq!((first.0, first.1), (second.0, second.1));
    assert!(first.2.max(second.2) < 8_000);
}

/// 🧮️ Cross-validates `solve_multi_case`'s sparse RCM-ordered pipeline (single case) against
/// `solve_linear_static`'s already-correct dense pipeline on an equivalent model — same oracle
/// strategy already used elsewhere in this crate.
#[test]
fn solve_multi_case_matches_single_case_dense_solve() {
    let (e, area, iy, l) = (200e9, 0.01, 1e-5, 2.0);
    let (model, cases) = cantilever_analysis_model(e, area, iy, l, 0.0);
    let results = solve_multi_case(&model, &cases, &[], [0.0, 0.0, 0.0]).expect("solves");
    let sparse_result = results.get("tip_load").expect("case present");

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
    let ra = results.get("a_case").unwrap().clone();
    let rb = results.get("b_case").unwrap().clone();
    let combined = results.get("combo").unwrap();

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
    let result = results.get("self_weight").unwrap();

    let total_ty_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Ty).map(|r| r.value).sum();
    let expected = density * area * l * 9.81;
    // Reactions balance the applied (downward, negative) self-weight load, so they sum positive.
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

        // Sanity-check the reference static solve first: pure axial compression should give nonzero Tx
        // displacement at the loaded end and ~zero Ty/Rz everywhere (no bending under a concentric load).
        let static_results = solve_multi_case(&model, std::slice::from_ref(&reference_case), &[], [0.0, 0.0, 0.0]).expect("reference solves");
        let static_result = static_results.get("axial_compression").unwrap();
        for d in &static_result.displacements {
            assert!(d.values[Dof::Ty.index()].abs() < 1e-9, "{name}: unexpected transverse displacement at {}: {}", d.node_id, d.values[Dof::Ty.index()]);
        }

        let critical_load = buckling(&model, &reference_case, 1).expect("buckling solves").factors[0] * p_ref;
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
    let result = results.get("axial").unwrap();
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
    use crate::elements2d::{PlaneKind, Tri3Cst};
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
    use crate::elements2d::{PlaneKind, Tri3Cst};
    let (e, nu, t) = (1000.0, 0.25, 1.0);
    let el_a = Tri3Cst { id: "a".into(), nodes: ["shared".into(), "a1".into(), "a2".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };
    let el_b = Tri3Cst { id: "b".into(), nodes: ["shared".into(), "b1".into(), "b2".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };

    let ctx_a = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]] };
    let ctx_b = ElementContext { positions: vec![[0.0, 0.0, 0.0], [-2.0, 0.0, 0.0], [0.0, -2.0, 0.0]] };
    // `u = k*x` uniaxial fields with distinct magnitudes `k_a=0.02`, `k_b=0.05`, both zero at the
    // shared origin node so they stay purely constant-strain (patch-test-exact) on each triangle.
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
    let sparse_result = results.get("udl").unwrap();

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

/// 🌱️ `zero_like` zero-initializes every non-`Beam` `ElementResult` variant (the `Beam` variant is
/// covered by `combination_equals_manual_superposition` above), and `add_scaled_element_result` on
/// a freshly-zeroed accumulator reduces to exactly `factor * term`, field-by-field, per variant.
#[test]
fn zero_like_and_add_scaled_element_result_handle_every_non_beam_variant() {
    let factor = 2.5;

    let bar = ElementResult::Bar { n: 4.0 };
    let zero_bar = zero_like(&bar);
    assert_eq!(zero_bar, ElementResult::Bar { n: 0.0 });
    match add_scaled_element_result(&zero_bar, &bar, factor) {
        ElementResult::Bar { n } => assert!((n - factor * 4.0).abs() < 1e-12),
        other => panic!("expected bar, got {other:?}"),
    }

    let plane = ElementResult::Plane { gauss: vec![PlaneStress { sxx: 1.0, syy: 2.0, sxy: 3.0, von_mises: 4.0 }] };
    let zero_plane = zero_like(&plane);
    match &zero_plane {
        ElementResult::Plane { gauss } => assert_eq!(gauss[0], PlaneStress { sxx: 0.0, syy: 0.0, sxy: 0.0, von_mises: 0.0 }),
        other => panic!("expected plane, got {other:?}"),
    }
    match add_scaled_element_result(&zero_plane, &plane, factor) {
        ElementResult::Plane { gauss } => {
            assert!((gauss[0].sxx - factor * 1.0).abs() < 1e-12);
            assert!((gauss[0].syy - factor * 2.0).abs() < 1e-12);
            assert!((gauss[0].sxy - factor * 3.0).abs() < 1e-12);
        }
        other => panic!("expected plane, got {other:?}"),
    }

    let plate = ElementResult::Plate { gauss: vec![PlateMoments { mx: 1.0, my: 2.0, mxy: 3.0 }] };
    let zero_plate = zero_like(&plate);
    match add_scaled_element_result(&zero_plate, &plate, factor) {
        ElementResult::Plate { gauss } => {
            assert!((gauss[0].mx - factor * 1.0).abs() < 1e-12);
            assert!((gauss[0].my - factor * 2.0).abs() < 1e-12);
            assert!((gauss[0].mxy - factor * 3.0).abs() < 1e-12);
        }
        other => panic!("expected plate, got {other:?}"),
    }

    let solid = ElementResult::Solid { gauss: vec![SolidStress { sxx: 1.0, syy: 2.0, szz: 3.0, sxy: 4.0, syz: 5.0, sxz: 6.0, von_mises: 7.0 }] };
    let zero_solid = zero_like(&solid);
    match add_scaled_element_result(&zero_solid, &solid, factor) {
        ElementResult::Solid { gauss } => {
            assert!((gauss[0].sxx - factor * 1.0).abs() < 1e-12);
            assert!((gauss[0].szz - factor * 3.0).abs() < 1e-12);
            assert!((gauss[0].syz - factor * 5.0).abs() < 1e-12);
        }
        other => panic!("expected solid, got {other:?}"),
    }

    let shell = ElementResult::Shell { gauss: vec![ShellState { nxx: 1.0, nyy: 2.0, nxy: 3.0, mxx: 4.0, myy: 5.0, mxy: 6.0, von_mises_top: 7.0, von_mises_bottom: 8.0 }] };
    let zero_shell = zero_like(&shell);
    match add_scaled_element_result(&zero_shell, &shell, factor) {
        ElementResult::Shell { gauss } => {
            assert!((gauss[0].nxx - factor * 1.0).abs() < 1e-12);
            assert!((gauss[0].mxy - factor * 6.0).abs() < 1e-12);
            assert!((gauss[0].von_mises_bottom - factor * 8.0).abs() < 1e-12);
        }
        other => panic!("expected shell, got {other:?}"),
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
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(2, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        if let StepOutcome::CheckpointReady(checkpoint) = graph.step(&mut context) {
            break payload_bytes(checkpoint.state);
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
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(3, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        if matches!(resumed.step(&mut context), StepOutcome::Complete(_)) {
            break;
        }
    }
    assert_eq!(resumed.progress().completed_units, 36);
    assert_eq!(seen, graph_plan().into_iter().skip(1).map(|plan| plan.stage).collect::<Vec<_>>());
}

#[test]
fn fem_job_graph_rejects_stale_and_cancelled_steps_without_mutation() {
    let operation = graph_operation(202);
    let mut graph = FemJobGraph::new(operation, graph_plan(), 2);
    let before = graph.checkpoint_bytes();
    let mut sequence = 0;
    let mut stale = StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(2, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(graph.step(&mut stale), StepOutcome::Fault(_)));
    assert_eq!(graph.checkpoint_bytes(), before);

    let token = semio_framework_job::root_cancel_token();
    semio_framework_async::block_on(token.cancel());
    let mut cancelled = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(2, u64::MAX), token, || Some(0), &mut sequence);
    assert_eq!(graph.step(&mut cancelled), StepOutcome::Cancelled);
    assert_eq!(graph.checkpoint_bytes(), before);
}

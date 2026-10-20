/// 🧈️ Assembly CSR, element build and assembly job laws: contiguous-free CSR, exact owner builds, partition-exact and resumable jobs, microcursor deadlines.

use semio_s_fem_test_support::Seen;
use semio_s_fem_analyses_assembly::AssemblyJob;
use semio_s_fem_analyses_assembly::AssemblyPreview;

#[test]
fn mounted_element_build_reserves_and_reclaims_one_exact_owner_per_turn() {
    let operation = assembly_operation(89);
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
    let operation = assembly_operation(301);
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
    let operation = assembly_operation(302);
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
    let operation = assembly_operation(303);
    let (_, previews, latency) = finish_assembly_job(AssemblyJob::new(&model, operation, 8).expect("assembly prepares"), operation, 1);
    assert!(!previews.is_empty());
    latency.assert_interactive("one-fuel assembly");
}

/// 🧪️ Mounted fixed-family cell cursors preserve numerical identity and every interruption seam.
#[test]
fn p6h_element_stiffness_microcursor_deadline_stale_cancel_close_and_stage_laws() {
    use semio_s_fem_elements2d::PlaneKind;
    use semio_s_fem_elements2d::Tri3Cst;

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
                let mut stale =
                    StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                assert!(matches!(seen_of(job.step(&mut stale)), Seen::Fault(_)));
                assert_eq!(job.state.pending_build.as_ref().map(|build| (build.stage, build.scalar_cursor, build.lookup_cursor, build.lookup_match, build.indices_new.len(), build.positions.len(), build.stiffness.len())), before);
                let token = semio_framework_job::root_cancel_token();
                semio_framework_async::block_on(token.cancel());
                let mut sequence_receipt = RetainedCloneProgress::default();
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
                let mut stale =
                    StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                assert!(matches!(seen_of(job.step(&mut stale)), Seen::Fault(_)));
                let token = semio_framework_job::root_cancel_token();
                semio_framework_async::block_on(token.cancel());
                let mut sequence_receipt = RetainedCloneProgress::default();
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

    let first = run(assembly_operation(307));
    let second = run(assembly_operation(307));
    assert_eq!((first.0, first.1), (second.0, second.1));
    assert!(first.2.max(second.2) < 8_000);
}

use super::*;
use semio_s_fem_test_support::close_turn;

#[test]
fn mesh_preparation_refuses_logical_fill_before_mutating_pending_input() {
    fn witness(owner: &MeshInputPreparation) -> (String, [usize; 3], [usize; 3]) {
        (
            format!("{owner:?}"),
            [owner.points.as_ptr() as usize, owner.point_indices.as_ptr() as usize, owner.constraints.as_ptr() as usize],
            [owner.points.capacity() * size_of::<[f64; 2]>(), owner.point_indices.capacity() * size_of::<((u64, u64), usize)>(), owner.constraints.capacity() * size_of::<Edge>()],
        )
    }

    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️preparation-owners/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in corpus["fillCases"].as_array().unwrap() {
        let operation = mesh_operation();
        let outer: Vec<[f64; 2]> = serde_json::from_value(row["outer"].clone()).unwrap();
        let mut domain = MountedPlanarDomain::new();
        while !domain.admit_outer_one(outer.len()).unwrap() {}
        for point in outer { domain.push_outer(point).unwrap(); }
        let mut job = MeshJob::new_mounted_bounded(domain, no_refine(), operation, row["maximumPoints"].as_u64().unwrap() as usize, row["maximumTriangles"].as_u64().unwrap() as usize);
        let mut sequence = 0;
        for _ in 0..4 {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            assert_eq!(seen_of(job.step(&mut context)), Seen::Yield);
        }
        let admitted = witness(job.preparation.as_ref().unwrap()).2.iter().sum::<usize>();
        let mut fault = false;
        let mut holds = true;
        for _ in 0..512 {
            let before = witness(job.preparation.as_ref().unwrap());
            for (fuel, deadline, cancelled) in [(0, u64::MAX, false), (1, 0, false), (1, u64::MAX, true)] {
                let token = root_cancel_token();
                if cancelled { token.cancel_now(); }
                let mut sequence_receipt = RetainedCloneProgress::default();
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, deadline, TEST_GRANT), token, || Some(0), &mut sequence, &mut sequence_receipt);
                let outcome = seen_of(job.step(&mut context));
                holds &= outcome == if cancelled { Seen::Cancelled } else { Seen::Yield };
                holds &= witness(job.preparation.as_ref().unwrap()) == before;
            }
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            match seen_of(job.step(&mut context)) {
                Seen::Fault(_) => {
                    fault = true;
                    holds &= witness(job.preparation.as_ref().unwrap()) == before;
                    let stage = job.stage;
                    let failed = witness(job.preparation.as_ref().unwrap());
                    drop(context);
                    let mut sequence_receipt = RetainedCloneProgress::default();
                    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                    match seen_of(job.step(&mut context)) {
                        Seen::Fault(_) => {}
                        _ => holds = false,
                    }
                    holds &= job.stage == stage && witness(job.preparation.as_ref().unwrap()) == failed;
                    break;
                }
                Seen::Yield => {}
                _ => { holds = false; break; }
            }
            if job.stage != MeshJobStage::PrepareInput { break; }
        }
        let owner = job.preparation.as_ref().unwrap();
        let actual = serde_json::json!({
            "points": owner.points.len(), "constraints": owner.constraints.len(),
            "pendingPoint": owner.pending_point.is_some(), "pendingIndex": owner.pending_index.is_some()
        });
        let retained = witness(owner).2.iter().sum::<usize>();
        InteractiveJob::begin_close(&mut job);
        let mut released = 0;
        for _ in 0..1024 {
            let (complete, items, bytes) = observe_mesh_close(&mut job, mesh_release_grant(4096));
            holds &= items <= 1 && bytes <= 4096;
            released += bytes;
            if complete { break; }
        }
        let terminal = InteractiveJob::terminal_is_empty(&job);
        observations.push((row["id"].clone(), actual, row["expected"].clone(), fault, holds, admitted, retained, released, terminal));
    }
    for (id, actual, expected, fault, holds, admitted, retained, released, terminal) in observations {
        assert_eq!(actual, expected, "{id}");
        assert!(fault && holds && terminal, "{id}");
        assert_eq!(retained, admitted, "{id} refuses before reallocation");
        assert_eq!(released, admitted, "{id} conserves physical ownership");
    }
}

#[test]
fn mesh_preparation_owns_each_reservation_and_preserves_lookup_on_handoff() {
    fn witness(job: &MeshJob) -> ([usize; 3], [usize; 3], [usize; 3]) {
        job.preparation.as_ref().map(|owner| (
            [owner.points.capacity(), owner.point_indices.capacity(), owner.constraints.capacity()],
            [owner.points.len(), owner.point_indices.len(), owner.constraints.len()],
            [owner.points.as_ptr() as usize, owner.point_indices.as_ptr() as usize, owner.constraints.as_ptr() as usize],
        )).unwrap_or(([0; 3], [0; 3], [0; 3]))
    }

    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️preparation-owners/🔣️.json")).expect("neutral preparation cases");
    let mut observations = Vec::new();
    for row in corpus["cases"].as_array().expect("cases") {
        let operation = mesh_operation();
        let mut domain = MountedPlanarDomain::new();
        while !domain.admit_outer_one(4).expect("mounted domain credit") {}
        for point in square(1.0) { domain.push_outer(point).expect("admitted point"); }
        let mut job = MeshJob::new_mounted_bounded(domain, no_refine(), operation, row["maximumPoints"].as_u64().unwrap() as usize, row["maximumTriangles"].as_u64().unwrap() as usize);
        let mut sequence = 0;
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        assert_eq!(seen_of(job.step(&mut context)), Seen::Yield);
        let mut reservations = Vec::new();
        let mut fault = false;
        let mut fault_detail = None;
        let mut holds = true;
        drop(context);
        for _ in 0..8 {
            let before = (job.stage, witness(&job));
            for (fuel, deadline, cancel) in [(0, u64::MAX, false), (1, 0, false), (1, u64::MAX, true)] {
                let token = root_cancel_token();
                if cancel { token.cancel_now(); }
                let mut sequence_receipt = RetainedCloneProgress::default();
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, deadline, TEST_GRANT), token, || Some(0), &mut sequence, &mut sequence_receipt);
                let outcome = seen_of(job.step(&mut context));
                holds &= outcome == if cancel { Seen::Cancelled } else { Seen::Yield };
                holds &= (job.stage, witness(&job)) == before;
            }
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            match seen_of(job.step(&mut context)) {
                Seen::Fault(detail) => { fault_detail = Some(detail); fault = true; }
                Seen::Yield => {}
                other => panic!("unexpected preparation outcome: {other:?}"),
            }
            holds &= context.fuel_remaining() == 0;
            reservations.push(witness(&job).0.map(|capacity| capacity != 0));
            if fault || job.stage == MeshJobStage::PrepareInput { break; }
        }
        let admitted = witness(&job);
        if fault {
            let before = (job.stage, witness(&job));
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            let Seen::Fault(repeated) = seen_of(job.step(&mut context)) else { panic!("preparation fault must remain sticky") };
            holds &= Some(repeated) == fault_detail;
            holds &= (job.stage, witness(&job)) == before;
        }
        let admitted_bytes = admitted.0[0] * size_of::<[f64; 2]>() + admitted.0[1] * size_of::<((u64, u64), usize)>() + admitted.0[2] * size_of::<Edge>();
        let close_grant = 4096.max(admitted.0[0] * size_of::<[f64; 2]>()).max(admitted.0[1] * size_of::<((u64, u64), usize)>()).max(admitted.0[2] * size_of::<Edge>());
        let mut preserves_index = false;
        if !fault {
            for _ in 0..4096 {
                let mut sequence_receipt = RetainedCloneProgress::default();
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                assert_eq!(seen_of(job.step(&mut context)), Seen::Yield);
                if job.stage == MeshJobStage::CountInput { break; }
            }
            let retained = witness(&job);
            preserves_index = retained.0[1] == admitted.0[1] && retained.2[1] == admitted.2[1] && retained.1[1] != 0;
            holds &= job.prepared_input.as_ref().is_some_and(|(points, edges)| points.len() == 4 && edges.len() == 4 && points.as_ptr() as usize == admitted.2[0] && edges.as_ptr() as usize == admitted.2[2]);
        }
        InteractiveJob::begin_close(&mut job);
        let before = witness(&job);
        holds &= close_turn(&mut job, RetainedCloneGrant { maximum_items: 0, ..mesh_release_grant(4096) }).expect("mesh close turn") == InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
        holds &= witness(&job) == before;
        let mut released = 0;
        for _ in 0..4096 {
            let (complete, items, bytes) = observe_mesh_close(&mut job, mesh_release_grant(close_grant));
            holds &= items <= 1 && bytes <= 4096;
            released += bytes;
            if complete { break; }
        }
        let actual = serde_json::json!({ "reservations": reservations, "fault": fault, "preservesIndexOnHandoff": preserves_index });
        observations.push((row["id"].clone(), actual, row["expected"].clone(), holds, admitted_bytes, released, InteractiveJob::terminal_is_empty(&job)));
    }
    for (id, actual, expected, holds, admitted, released, terminal) in observations {
        assert_eq!(actual, expected, "{id}");
        assert!(holds && terminal, "{id}");
        assert_eq!(released, admitted, "{id} retires every admitted backing");
    }
}

#[test]
fn mesh_preparation_cancellation_closes_each_partial_owner_under_exact_grants() {
    for cut in 1..=4 {
        let operation = mesh_operation();
        let mut domain = MountedPlanarDomain::new();
        while !domain.admit_outer_one(4).unwrap() {}
        for point in square(1.0) { domain.push_outer(point).unwrap(); }
        let mut job = MeshJob::new_mounted_bounded(domain, no_refine(), operation, 8, 8);
        let mut sequence = 0;
        for _ in 0..cut {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            assert_eq!(seen_of(job.step(&mut context)), Seen::Yield);
        }
        let owner = job.preparation.as_ref().unwrap();
        let allocations = [owner.points.capacity() * size_of::<[f64; 2]>(), owner.point_indices.capacity() * size_of::<((u64, u64), usize)>(), owner.constraints.capacity() * size_of::<Edge>()];
        let admitted = allocations.iter().sum::<usize>();
        let token = root_cancel_token();
        token.cancel_now();
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), token, || Some(0), &mut sequence, &mut sequence_receipt);
        assert_eq!(seen_of(job.step(&mut context)), Seen::Cancelled);
        assert_eq!(context.fuel_remaining(), 1);
        InteractiveJob::begin_close(&mut job);
        assert_eq!(close_turn(&mut job, RetainedCloneGrant { maximum_items: 0, ..mesh_release_grant(4096) }).expect("mesh close turn"), InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        let mut released = 0;
        for _ in 0..128 {
            let (complete, items, bytes) = observe_mesh_close(&mut job, mesh_release_grant(0));
            assert_eq!(bytes, 0);
            if complete || items == 0 { break; }
        }
        if let Some(bytes) = allocations.into_iter().find(|bytes| *bytes != 0) {
            assert_eq!(observe_mesh_close(&mut job, mesh_release_grant(bytes - 1)), (false, 0, 0));
            assert_eq!(observe_mesh_close(&mut job, mesh_release_grant(bytes)), (false, 1, bytes));
            released += bytes;
        }
        for _ in 0..128 {
            let (complete, items, bytes) = observe_mesh_close(&mut job, mesh_release_grant(4096));
            assert!(items <= 1 && bytes <= 4096);
            released += bytes;
            if complete { break; }
        }
        assert!(InteractiveJob::terminal_is_empty(&job));
        assert_eq!(released, admitted);
        eprintln!("mesh preparation cancellation cut={cut}, admitted={admitted}, released={released}");
    }
}

#[test]
fn mesh_edge_authority_uses_completed_faces_and_closes_exact_backing() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🕸️edge-authority/🔣️.json")).expect("neutral edge authority cases");
    for row in corpus["cases"].as_array().expect("cases") {
        let operation = mesh_operation();
        let points = square(1.0);
        let triangles: Vec<[usize; 3]> = serde_json::from_value(row["triangles"].clone()).expect("neutral triangles");
        let count = triangles.len();
        let mut job = MeshJob::new_bounded(PlanarDomain { outer: points.clone(), holes: Vec::new() }, no_refine(), operation, 62, row["maximumTriangles"].as_u64().expect("triangle limit") as usize);
        job.triangulation = Some(OwnedTriangulation {
            points, triangles, input_len: 4, insert_cursor: 4, insertion_order: Vec::new(), insertion: None,
            maximum_triangles: job.maximum_triangles, allocation_fault: false,
            mounted_initialization: MountedTriangulationInitialization { stage: MountedTriangulationStage::Complete, cursor: 0, sort_outer: 4, sort_inner: 4, bounds: [0.0, 1.0, 0.0, 1.0], center: [0.5, 0.5], span: 1.0 },
            finish: TriangulationFinishCursor { stage: TriangulationFinishStage::Complete, read: count, write: count, sort_outer: count, sort_inner: count },
        });
        job.stage = MeshJobStage::ReserveEdgeAuthorities;
        let mut sequence = 0;
        for _ in 0..256 {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            assert!(matches!(seen_of(job.step(&mut context)), Seen::Yield), "{}", row["id"]);
            if job.stage == MeshJobStage::ConstrainBoundary { break; }
        }
        assert_eq!(job.stage, MeshJobStage::ConstrainBoundary);
        assert_eq!(job.indexed_constraint_edges.capacity(), row["expected"]["slots"].as_u64().expect("slots") as usize);
        assert_eq!(job.indexed_constraint_edges.len(), row["expected"]["edges"].as_u64().expect("edges") as usize);
        assert_eq!(job.indexed_constraint_edges.iter().filter(|edge| edge.adjacent[1].is_none()).count(), row["expected"]["boundary"].as_u64().expect("boundary") as usize);
        assert_eq!(job.indexed_constraint_edges.iter().filter(|edge| edge.adjacent[1].is_some()).count(), row["expected"]["interior"].as_u64().expect("interior") as usize);
        while job.indexed_constraint_edges.len() < job.indexed_constraint_edges.capacity() {
            let index = job.indexed_constraint_edges.len() + 10;
            job.indexed_constraint_edges.push(IndexedConstraintEdge { edge: Edge::new(index, index + 1), adjacent: [Some(index), None], active: true, fixed: false });
        }
        let before = (job.indexed_constraint_edges.as_ptr(), job.indexed_constraint_edges.len(), job.indexed_constraint_edges.capacity());
        job.edge_index_candidate = Some((Edge::new(100, 101), 100));
        job.edge_index_lookup_cursor = 0;
        job.edge_index_vacancy = None;
        while job.edge_index_lookup_cursor < job.indexed_constraint_edges.len() {
            assert_eq!(job.advance_edge_index_candidate(), Ok(false));
        }
        assert_eq!(job.advance_edge_index_candidate(), Err(()));
        assert_eq!((job.indexed_constraint_edges.as_ptr(), job.indexed_constraint_edges.len(), job.indexed_constraint_edges.capacity()), before);
        job.close_lane = 9;
        for _ in 0..before.1 {
            assert_eq!(observe_mesh_close(&mut job, mesh_release_grant(0)), (false, 1, 0));
        }
        let bytes = before.2 * size_of::<IndexedConstraintEdge>();
        assert_eq!(observe_mesh_close(&mut job, mesh_release_grant(bytes - 1)), (false, 0, 0));
        assert_eq!(job.indexed_constraint_edges.as_ptr(), before.0);
        assert_eq!(observe_mesh_close(&mut job, mesh_release_grant(bytes)), (false, 1, bytes));
        assert_eq!(job.indexed_constraint_edges.capacity(), 0);
        assert_eq!(job.close_granted(mesh_release_grant(0)).map_err(|error| error.kind), Err(ValueRefusalKind::InvariantViolated));
        job.close_lane = 0;
        let mut closed = false;
        for _ in 0..256 {
            if observe_mesh_close(&mut job, mesh_release_grant(16_384)).0 { closed = true; break; }
        }
        assert!(closed, "all remaining mesh owners retire");
        assert!(InteractiveJob::terminal_is_empty(&job));
    }
}
use semio_framework_job::root_cancel_token;
use semio_framework_job::Generation;
use semio_framework_job::OperationId;
use semio_framework_job::RevisionId;
use semio_framework_job::StepBudget;
use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;

fn mesh_operation() -> Operation {
    Operation::new(OperationId(700), RevisionId(11), Generation(3), 19)
}

const TEST_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

/// 🧭️ What one step lent to its caller, copied out before the borrow is released.
#[derive(Debug, PartialEq, Eq)]
enum Seen {
    Yield,
    Preview(Vec<u8>),
    Checkpoint(Vec<u8>),
    Fault(Vec<u8>),
    Cancelled,
    Complete { state: Vec<u8>, output: Vec<u8> },
}

/// 🚪️ Closes a locally owned job through its own quoted ladder so no lent payload is abandoned.
fn close_mesh_job(job: &mut MeshJob) {
    InteractiveJob::begin_close(job);
    for _ in 0..10_000_000 {
        if InteractiveJob::terminal_is_empty(job) {
            return;
        }
        let demand = job.retirement_demand().expect("a locally owned mesh quotes its close demand");
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let step = job.close_granted(grant);
        assert!(matches!(step, Ok(InteractiveJobCloseStep::Pending { .. } | InteractiveJobCloseStep::Complete { .. })), "a locally owned mesh close has no external owner and no refusal");
    }
    panic!("the mesh close ladder never reached terminal-empty");
}

fn payload_bytes(payload: &RetainedJobPayload) -> Vec<u8> {
    (0..payload.page_count()).flat_map(|page| payload.page(page).expect("retained mesh payload page").iter().copied()).collect()
}

fn seen_of<E: std::fmt::Debug>(result: Result<Option<JobOutcomeBorrow<'_>>, E>) -> Seen {
    use semio_framework_job::JobOutcomeBorrow;
    match result.expect("mesh step admission") {
        None | Some(JobOutcomeBorrow::Yield { .. }) => Seen::Yield,
        Some(JobOutcomeBorrow::PreviewReady { payload, .. }) => Seen::Preview(payload_bytes(payload)),
        Some(JobOutcomeBorrow::CheckpointReady { state, .. }) => Seen::Checkpoint(payload_bytes(state)),
        Some(JobOutcomeBorrow::Fault { detail, .. }) => Seen::Fault(payload_bytes(detail)),
        Some(JobOutcomeBorrow::Cancelled { .. }) => Seen::Cancelled,
        Some(JobOutcomeBorrow::Complete { state, output, .. }) => Seen::Complete { state: state.map(payload_bytes).unwrap_or_default(), output: output.map(payload_bytes).unwrap_or_default() },
    }
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
    worst: Duration,
}

impl StepLatency {
    fn admit(&mut self, elapsed: Duration) {
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

fn drive_mesh_job(mut job: MeshJob) -> (Vec<u8>, usize, StepLatency) {
    fn now() -> Option<u64> {
        Some(0)
    }
    let cancel = root_cancel_token();
    let mut sequence = 0;
    let mut previews = 0;
    let mut latency = StepLatency::default();
    for _ in 0..10_000_000 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(64, 10, TEST_GRANT), cancel.clone(), now, &mut sequence, &mut sequence_receipt);
        let started = Instant::now();
        let outcome = seen_of(job.step(&mut context));
        latency.admit(started.elapsed());
        match outcome {
            Seen::Preview(_) => previews += 1,
            Seen::Complete { output, .. } => {
                close_mesh_job(&mut job);
                return (output, previews, latency);
            }
            Seen::Checkpoint(_) => {}
            Seen::Yield => {}
            other => panic!("mesh job failed: {other:?}"),
        }
    }
    panic!("mesh job did not complete")
}

fn shoelace_area(points: &[[f64; 2]]) -> f64 {
    let mut sum = 0.0;
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        sum += a[0] * b[1] - b[0] * a[1];
    }
    (sum * 0.5).abs()
}

fn tri_area(mesh: &TriMesh2, tri: &[u32; 3]) -> f64 {
    shoelace_area(&[mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]])
}

fn total_area(mesh: &TriMesh2) -> f64 {
    mesh.tris.iter().map(|t| tri_area(mesh, t)).sum()
}

fn no_refine() -> MeshOpts {
    MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }
}

fn square(side: f64) -> Vec<[f64; 2]> {
    vec![[0.0, 0.0], [side, 0.0], [side, side], [0.0, side]]
}

#[test]
fn triangulate_square_area_matches_input() {
    let outer = square(10.0);
    let expected = shoelace_area(&outer);
    let domain = PlanarDomain { outer, holes: vec![] };
    let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
    assert!(!mesh.tris.is_empty());
    assert!((total_area(&mesh) - expected).abs() < 1e-9);
}

#[test]
fn triangulate_respects_hole_area() {
    let outer = square(10.0);
    let hole = vec![[3.0, 3.0], [7.0, 3.0], [7.0, 7.0], [3.0, 7.0]];
    let domain = PlanarDomain { outer, holes: vec![hole.clone()] };
    let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
    let expected = 100.0 - 16.0;
    assert!((total_area(&mesh) - expected).abs() < 1e-6, "area={}", total_area(&mesh));
    for tri in &mesh.tris {
        let p0 = mesh.points[tri[0] as usize];
        let p1 = mesh.points[tri[1] as usize];
        let p2 = mesh.points[tri[2] as usize];
        let centroid = [(p0[0] + p1[0] + p2[0]) / 3.0, (p0[1] + p1[1] + p2[1]) / 3.0];
        assert!(!point_in_polygon(centroid, &hole));
    }
}

#[test]
fn triangulate_honors_constrained_boundary_edges() {
    // L-shape: non-convex outer boundary.
    let outer = vec![[0.0, 0.0], [10.0, 0.0], [10.0, 5.0], [5.0, 5.0], [5.0, 10.0], [0.0, 10.0]];
    let domain = PlanarDomain { outer: outer.clone(), holes: vec![] };
    let mesh = triangulate(&domain, &no_refine()).expect("triangulates");

    let key = |p: [f64; 2]| (p[0].to_bits(), p[1].to_bits());
    let mut edge_set: HashSet<((u64, u64), (u64, u64))> = HashSet::new();
    for tri in &mesh.tris {
        let p = [mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]];
        for i in 0..3 {
            let a = key(p[i]);
            let b = key(p[(i + 1) % 3]);
            let edge = if a <= b { (a, b) } else { (b, a) };
            edge_set.insert(edge);
        }
    }

    for i in 0..outer.len() {
        let a = key(outer[i]);
        let b = key(outer[(i + 1) % outer.len()]);
        let edge = if a <= b { (a, b) } else { (b, a) };
        assert!(edge_set.contains(&edge), "boundary edge {i} missing from triangulation");
    }
}

#[test]
fn triangulate_rejects_degenerate_outer_boundary() {
    let domain = PlanarDomain { outer: vec![[0.0, 0.0], [1.0, 0.0]], holes: vec![] };
    match triangulate(&domain, &no_refine()) {
        Err(MeshError::DegenerateDomain) => {}
        other => panic!("expected DegenerateDomain, got {other:?}"),
    }
}

#[test]
fn triangulate_rejects_degenerate_hole() {
    let domain = PlanarDomain { outer: square(10.0), holes: vec![vec![[3.0, 3.0], [4.0, 4.0]]] };
    match triangulate(&domain, &no_refine()) {
        Err(MeshError::DegenerateDomain) => {}
        other => panic!("expected DegenerateDomain, got {other:?}"),
    }
}

#[test]
fn point_in_polygon_returns_false_for_degenerate_polygon() {
    assert!(!point_in_polygon([0.0, 0.0], &[]));
    assert!(!point_in_polygon([0.0, 0.0], &[[0.0, 0.0], [1.0, 0.0]]));
}

#[test]
fn mesh_job_is_previewing_deterministic_and_step_bounded() {
    let domain = PlanarDomain { outer: square(8.0), holes: vec![square(2.0).into_iter().map(|point| [point[0] + 3.0, point[1] + 3.0]).collect()] };
    let options = MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 };
    let first = drive_mesh_job(MeshJob::new(domain.clone(), options, mesh_operation()));
    let second = drive_mesh_job(MeshJob::new(domain, options, mesh_operation()));
    assert_eq!(first.0, second.0);
    assert!(first.1 > 0);
    first.2.assert_interactive("mesh job");
}

#[test]
fn mesh_job_observes_cancellation_before_mutating() {
    fn now() -> Option<u64> {
        Some(0)
    }
    let operation = mesh_operation();
    let mut job = MeshJob::new(PlanarDomain { outer: square(2.0), holes: vec![] }, MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }, operation);
    let cancel = root_cancel_token();
    cancel.cancel_now();
    let mut sequence = 0;
    let mut sequence_receipt = RetainedCloneProgress::default();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(64, 10, TEST_GRANT), cancel, now, &mut sequence, &mut sequence_receipt);
    assert_eq!(seen_of(job.step(&mut context)), Seen::Cancelled);
    assert_eq!(job.stage, MeshJobStage::Validate);
    assert!(job.triangulation.is_none());
}

#[test]
fn mesh_job_large_boundary_never_runs_to_completion_in_one_step() {
    let boundary = (0..256)
        .map(|index| {
            let angle = index as f64 * std::f64::consts::TAU / 256.0;
            [angle.cos() * 100.0, angle.sin() * 100.0]
        })
        .collect();
    let (_, previews, latency) = drive_mesh_job(MeshJob::new(PlanarDomain { outer: boundary, holes: vec![] }, MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }, mesh_operation()));
    assert!(previews > 0);
    latency.assert_interactive("large-boundary mesh job");
}

#[test]
fn bounded_mesh_plus_one_fault_retains_the_exact_domain_for_cursor_close() {
    let operation = mesh_operation();
    let domain = PlanarDomain { outer: (0..65).map(|index| [index as f64, 0.0]).collect(), holes: Vec::new() };
    let mut job = MeshJob::new_bounded(domain, MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }, operation, 64, 128);
    let mut sequence = 0;
    let mut faulted = false;
    for _ in 0..256 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        if matches!(seen_of(job.step(&mut context)), Seen::Fault(_)) {
            faulted = true;
            break;
        }
    }
    assert!(faulted);
    assert_eq!(job.domain.outer().len(), 65, "fault retains every rejected point owner");
    let (terminal, items, _) = observe_mesh_close(&mut job, mesh_release_grant(4_096));
    assert!(!terminal);
    assert_eq!(items, 1);
    assert_eq!(job.domain.outer().len(), 64, "one close grant releases one point owner");
}

#[test]
fn p6h_constraint_flip_interrupts_after_every_edge_phase_and_updates_only_affected_adjacencies() {
    let operation = mesh_operation();
    let points = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let triangulation = OwnedTriangulation {
        points: points.clone(),
        triangles: vec![[0, 1, 2], [0, 2, 3]],
        input_len: 4,
        insert_cursor: 4,
        insertion_order: vec![0, 1, 2, 3],
        insertion: None,
        maximum_triangles: 4,
        allocation_fault: false,
        mounted_initialization: MountedTriangulationInitialization { stage: MountedTriangulationStage::Complete, cursor: 0, sort_outer: 4, sort_inner: 4, bounds: [0.0, 1.0, 0.0, 1.0], center: [0.5, 0.5], span: 1.0 },
        finish: TriangulationFinishCursor { stage: TriangulationFinishStage::Complete, read: 2, write: 2, sort_outer: 2, sort_inner: 2 },
    };
    let mut job = MeshJob::new_bounded(PlanarDomain { outer: points, holes: Vec::new() }, no_refine(), operation, 4, 4);
    job.triangulation = Some(triangulation);
    job.constraints = vec![Edge::new(1, 3)];
    job.stage = MeshJobStage::ConstrainBoundary;
    job.indexed_constraint_edges.try_reserve_exact(12).expect("fixed edge authority");
    for triangle in 0..2 {
        for local in 0..3 {
            assert!(job.begin_edge_index_candidate(triangle, local));
            while !job.advance_edge_index_candidate().expect("index edge") {}
        }
    }
    let mut seen = HashSet::new();
    let mut sequence = 0;
    let mut maximum_micros = 0;
    for _ in 0..512 {
        let stage = job.constraint_stage;
        seen.insert(stage);
        let before = job.triangulation.as_ref().expect("triangulation retained").triangles.clone();
        let before_cursor = (job.constraint_cursor, job.constraint_stage, job.constraint_search_cursor, job.constraint_apply_cursor, job.constraint_retire_cursor, job.constraint_retire_adjacency_cursor);
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut deadline = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        assert_eq!(seen_of(job.step(&mut deadline)), Seen::Yield);
        assert_eq!((job.constraint_cursor, job.constraint_stage, job.constraint_search_cursor, job.constraint_apply_cursor, job.constraint_retire_cursor, job.constraint_retire_adjacency_cursor), before_cursor);
        assert_eq!(job.triangulation.as_ref().expect("triangulation retained").triangles, before);

        let mut sequence_receipt = RetainedCloneProgress::default();
        drop(deadline);
        let mut stale = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        assert!(matches!(seen_of(job.step(&mut stale)), Seen::Fault(_)));
        assert_eq!((job.constraint_cursor, job.constraint_stage, job.constraint_search_cursor, job.constraint_apply_cursor, job.constraint_retire_cursor, job.constraint_retire_adjacency_cursor), before_cursor);

        let token = root_cancel_token();
        token.cancel_now();
        let mut sequence_receipt = RetainedCloneProgress::default();
        drop(stale);
        let mut cancelled = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), token, || Some(0), &mut sequence, &mut sequence_receipt);
        assert_eq!(seen_of(job.step(&mut cancelled)), Seen::Cancelled);
        assert_eq!((job.constraint_cursor, job.constraint_stage, job.constraint_search_cursor, job.constraint_apply_cursor, job.constraint_retire_cursor, job.constraint_retire_adjacency_cursor), before_cursor);

        let started = Instant::now();
        let mut sequence_receipt = RetainedCloneProgress::default();
        drop(cancelled);
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        assert_eq!(seen_of(job.step(&mut context)), Seen::Yield);
        maximum_micros = maximum_micros.max(started.elapsed().as_micros());
        if job.constraint_cursor == job.constraints.len() {
            break;
        }
    }
    for stage in [
        ConstraintRecoveryStage::ReserveConstraintWorkspace,
        ConstraintRecoveryStage::IndexTriangleEdge,
        ConstraintRecoveryStage::SearchConstraintEdge,
        ConstraintRecoveryStage::ClassifyIntersection,
        ConstraintRecoveryStage::SelectDeterministicFlip,
        ConstraintRecoveryStage::ValidateFlip,
        ConstraintRecoveryStage::ApplyFlip,
        ConstraintRecoveryStage::RetireFormerEdge,
        ConstraintRecoveryStage::PublishConstraintProgress,
        ConstraintRecoveryStage::ConstraintComplete,
    ] {
        assert!(seen.contains(&stage), "missing interrupted phase {stage:?}");
    }
    assert!(job.indexed_constraint_edges.iter().any(|slot| slot.active && slot.edge == Edge::new(1, 3)));
    assert!(!job.indexed_constraint_edges.iter().any(|slot| slot.active && slot.edge == Edge::new(0, 2)));
    assert!(maximum_micros < 8_000, "constraint recovery step exceeded timing ceiling: {maximum_micros} us");

    let mut close_turns = 0;
    loop {
        close_turns += 1;
        let (terminal, released_items, _) = observe_mesh_close(&mut job, mesh_release_grant(usize::MAX));
        assert!(released_items <= 1);
        if terminal {
            break;
        }
        assert!(close_turns < 20_000);
    }
    assert!(InteractiveJob::terminal_is_empty(&job));
}

/// 📦️ Mounted preparation, construction, finish, and payload cursors interrupt and replay exactly.
#[test]
fn p6h_mounted_mesh_preparation_initialization_finish_publication_interrupt_replay_timing_and_close() {
    fn domain() -> PlanarDomain {
        PlanarDomain { outer: square(2.0), holes: vec![square(0.5).into_iter().map(|point| [point[0] + 0.75, point[1] + 0.75]).collect()] }
    }

    fn snapshot(job: &MeshJob) -> impl PartialEq + std::fmt::Debug {
        (
            job.stage,
            job.preparation.as_ref().map(|cursor| {
                (
                    (cursor.polygon, cursor.edge, cursor.segment, cursor.point_lookup_cursor, cursor.grid_row, cursor.grid_column, cursor.grid_polygon),
                    (cursor.grid_edge, cursor.pending_point.is_some(), cursor.pending_index, cursor.grid_candidate.is_some(), cursor.points.len(), cursor.constraints.len()),
                )
            }),
            job.triangulation.as_ref().map(|cursor| {
                (
                    cursor.mounted_initialization.stage,
                    cursor.mounted_initialization.cursor,
                    cursor.mounted_initialization.sort_outer,
                    cursor.mounted_initialization.sort_inner,
                    cursor.finish.stage,
                    cursor.finish.read,
                    cursor.finish.write,
                    cursor.finish.sort_outer,
                    cursor.finish.sort_inner,
                    cursor.insert_cursor,
                    cursor.points.len(),
                    cursor.triangles.len(),
                )
            }),
            job.publication_kind,
            job.publication_cursor.stage,
            job.publication_cursor.point,
            job.publication_cursor.coordinate,
            job.publication_cursor.triangle,
            job.publication_cursor.index,
            job.publication_writer.as_ref().and_then(RetainedJobPayloadWriter::staged_page_len),
            job.mesh.points.len(),
            job.mesh.tris.len(),
        )
    }

    fn run(operation: Operation) -> (Vec<u8>, u128) {
        let mut job = MeshJob::new_bounded(domain(), MeshOpts { max_edge: 2.0, min_angle_deg: 0.0 }, operation, 128, 20);
        let mut sequence = 0;
        let mut maximum_micros = 0;
        let mut initialization_seen = HashSet::new();
        let mut finish_seen = HashSet::new();
        let mut payload_seen = HashSet::new();
        let mut preparation_lookup_seen = false;
        let mut preparation_polygon_seen = false;
        for _ in 0..1_000_000 {
            if let Some(preparation) = job.preparation.as_ref() {
                preparation_lookup_seen |= preparation.point_lookup_cursor > 0;
                preparation_polygon_seen |= preparation.grid_candidate.is_some() && preparation.grid_edge > 0;
            }
            if let Some(triangulation) = job.triangulation.as_ref() {
                initialization_seen.insert(triangulation.mounted_initialization.stage);
                finish_seen.insert(triangulation.finish.stage);
            }
            if job.publication_kind.is_some() {
                payload_seen.insert(job.publication_cursor.stage);
            }
            if matches!(job.stage, MeshJobStage::PrepareInput | MeshJobStage::Initialize | MeshJobStage::InsertBoundary | MeshJobStage::PublishPreview | MeshJobStage::PublishCheckpoint | MeshJobStage::Complete) {
                let before = snapshot(&job);
                let mut sequence_receipt = RetainedCloneProgress::default();
                let mut deadline = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                assert_eq!(seen_of(job.step(&mut deadline)), Seen::Yield);
                assert_eq!(snapshot(&job), before);
                let mut sequence_receipt = RetainedCloneProgress::default();
                drop(deadline);
                let mut stale = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                assert!(matches!(seen_of(job.step(&mut stale)), Seen::Fault(_)));
                assert_eq!(snapshot(&job), before);
                let token = root_cancel_token();
                token.cancel_now();
                let mut sequence_receipt = RetainedCloneProgress::default();
                drop(stale);
                let mut cancelled = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), token, || Some(0), &mut sequence, &mut sequence_receipt);
                assert_eq!(seen_of(job.step(&mut cancelled)), Seen::Cancelled);
                assert_eq!(snapshot(&job), before);
            }
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            let started = Instant::now();
            let outcome = seen_of(job.step(&mut context));
            maximum_micros = maximum_micros.max(started.elapsed().as_micros());
            match outcome {
                Seen::Preview(_) | Seen::Checkpoint(_) => {}
                Seen::Complete { output, .. } => {
                    for stage in [
                        MountedTriangulationStage::BoundsPoint,
                        MountedTriangulationStage::ValidateBounds,
                        MountedTriangulationStage::ReserveInsertionOrder,
                        MountedTriangulationStage::BuildInsertionOrder,
                        MountedTriangulationStage::OrderInsertion,
                        MountedTriangulationStage::ReserveSuperPoints,
                        MountedTriangulationStage::AppendSuperPoint,
                        MountedTriangulationStage::ReserveTriangles,
                        MountedTriangulationStage::SeedTriangle,
                        MountedTriangulationStage::Complete,
                    ] {
                        assert!(initialization_seen.contains(&stage), "missing mounted initialization stage {stage:?}");
                    }
                    for stage in [TriangulationFinishStage::Filter, TriangulationFinishStage::TruncateTriangles, TriangulationFinishStage::OrderTriangles, TriangulationFinishStage::TruncatePoints, TriangulationFinishStage::Complete] {
                        assert!(finish_seen.contains(&stage), "missing finish stage {stage:?}");
                    }
                    for stage in [
                        MeshPayloadStage::Magic,
                        MeshPayloadStage::Tier,
                        MeshPayloadStage::Complete,
                        MeshPayloadStage::Sequence,
                        MeshPayloadStage::Refinement,
                        MeshPayloadStage::PointCount,
                        MeshPayloadStage::TriangleCount,
                        MeshPayloadStage::PointCoordinate,
                        MeshPayloadStage::TriangleIndex,
                        MeshPayloadStage::CommitPage,
                    ] {
                        assert!(payload_seen.contains(&stage), "missing payload stage {stage:?}");
                    }
                    assert!(preparation_lookup_seen && preparation_polygon_seen);
                    close_mesh_job(&mut job);
                    return (output, maximum_micros);
                }
                Seen::Yield => {}
                outcome => panic!("mounted mesh cursor law failed: {outcome:?}"),
            }
        }
        panic!("mounted mesh cursor law did not complete")
    }

    let operation = mesh_operation();
    let first = run(operation);
    let second = run(operation);
    assert_eq!(first.0, second.0);
    assert!(first.1.max(second.1) < 8_000);

    let mut interrupted = MeshJob::new_bounded(domain(), MeshOpts { max_edge: 2.0, min_angle_deg: 0.0 }, operation, 128, 20);
    let mut sequence = 0;
    for _ in 0..1_000_000 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        match seen_of(interrupted.step(&mut context)) {
            Seen::Fault(detail) => panic!("interrupted mesh fixture fault: {detail:?}"),
            _ => {}
        }
        if interrupted.publication_writer.as_ref().and_then(RetainedJobPayloadWriter::staged_page_len).is_some_and(|length| length > 0) {
            break;
        }
    }
    assert!(interrupted.publication_writer.is_some());
    InteractiveJob::begin_close(&mut interrupted);
    assert_eq!(close_turn(&mut interrupted, mesh_release_grant(0)).expect("mesh close turn"), InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
    for _ in 0..100_000 {
        match close_turn(&mut interrupted, mesh_release_grant(usize::MAX)).expect("mounted mesh close has no refusal") {
            InteractiveJobCloseStep::Pending { progress } => assert!(progress.copied_items <= 1),
            InteractiveJobCloseStep::Complete { .. } => break,
            InteractiveJobCloseStep::Blocked => panic!("mounted mesh close cannot block"),
        }
    }
    assert!(InteractiveJob::terminal_is_empty(&interrupted));
}

fn mesh_release_grant(release_bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 256, maximum_capacity_bytes: 0, maximum_release_bytes: release_bytes, maximum_depth: 2 }
}

fn observe_mesh_close(job: &mut MeshJob, grant: RetainedCloneGrant) -> (bool, usize, usize) {
    let step = job.close_granted(grant).expect("a locally owned mesh close has no refusal");
    assert!(step.progress().fits(grant));
    assert!(!matches!(step, InteractiveJobCloseStep::Blocked));
    (matches!(step, InteractiveJobCloseStep::Complete { .. }), step.progress().copied_items, step.progress().released_bytes)
}

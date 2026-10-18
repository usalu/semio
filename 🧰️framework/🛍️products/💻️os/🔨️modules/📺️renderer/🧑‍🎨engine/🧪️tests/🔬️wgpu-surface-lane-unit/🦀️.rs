use super::*;
use semio_framework_job::{allocate_operation_id, default_now_us, CancelToken, Generation, OriginalJobFixtureCaller, RetainedCloneGrant, StepBudget};

/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
const UNBOUNDED_STEP_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };

/// 🪙️ The one stock the initiating fixture caller issues for a whole law.
const LANE_STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: usize::MAX / 4, maximum_copy_bytes: usize::MAX / 4, maximum_capacity_bytes: usize::MAX / 4, maximum_release_bytes: usize::MAX / 4, maximum_depth: 64 };

/// 🪙️ The bounded authority each lane turn borrows from that stock.
const LANE_TURN: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 22, maximum_release_bytes: 1 << 22, maximum_depth: 32 };

fn request(generation: u64, width: u32) -> SurfaceResizeRequest {
    SurfaceResizeRequest { token: SurfaceLaneToken { slot: 0, generation: 1 }, metrics_generation: generation, physical_width: width, physical_height: 720, scale_factor: 2.0 }
}

fn lane_caller() -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(allocate_operation_id(), Generation(1), LANE_STOCK, default_now_us, u64::MAX, u64::MAX)
}

fn close_lane(lane: &mut MountedSurfaceResizeLane, caller: &mut OriginalJobFixtureCaller) {
    for _ in 0..64 {
        if caller.turn(LANE_TURN, |cx| lane.close_step(cx)).expect("the lane close turn is not refused") {
            break;
        }
    }
    assert!(lane.terminal_is_empty());
}

#[test]
fn resize_job_consumes_one_scalar_per_grant() {
    let mut job = SurfaceResizeJob::new(request(1, 1280));
    let operation = allocate_operation_id();
    let cancel = CancelToken::root_now();
    let mut preview_sequence = 0;
    let mut cx_retained = semio_framework_job::RetainedCloneProgress::default();
    let mut cx = StepContext::new(operation, Generation(1), StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel, default_now_us, &mut preview_sequence, &mut cx_retained);
    assert!(matches!(job.step(&mut cx), Ok(Some(JobOutcomeBorrow::Yield { .. }))));
    assert_eq!(job.phase, ResizePhase::LogicalHeight);
    assert_eq!(job.logical_width, 640.0);
    assert_eq!(job.logical_height, 0.0);
}

#[test]
fn resize_job_closes_its_sealed_candidate_with_one_paid_item() {
    let mut job = SurfaceResizeJob::new(request(2, 1280));
    let operation = allocate_operation_id();
    let mut preview_sequence = 0;
    let mut cx_retained = semio_framework_job::RetainedCloneProgress::default();
    let mut cx = StepContext::new(operation, Generation(1), StepBudget::new(u64::MAX, u64::MAX, UNBOUNDED_STEP_GRANT), CancelToken::root_now(), default_now_us, &mut preview_sequence, &mut cx_retained);
    for _ in 0..2 {
        assert!(matches!(job.step(&mut cx), Ok(Some(JobOutcomeBorrow::Yield { .. }))));
    }
    assert!(matches!(job.step(&mut cx), Ok(Some(JobOutcomeBorrow::Complete { .. }))));
    assert!(job.candidate.is_some());
    job.begin_close();
    assert!(matches!(job.close_step(&mut cx), Ok(InteractiveJobCloseStep::Pending { progress }) if progress.copied_items == 1));
    assert!(matches!(job.close_step(&mut cx), Ok(InteractiveJobCloseStep::Complete { .. })));
    assert!(job.terminal_is_empty());
}

#[test]
fn million_resize_samples_retain_only_the_latest_exact_request() {
    let operation = allocate_operation_id();
    let mut lane = match MountedSurfaceResizeLane::try_new(operation) {
        Ok(lane) => lane,
        Err(_) => panic!("fixed resize lane must admit the first test surface"),
    };
    for width in 1..=1_000_000 {
        let result = lane.enqueue(width, 720, 2.0);
        assert!(result.is_ok());
    }
    assert_eq!(lane.metrics_generation, 1_000_000);
    assert!(lane.pending.is_some_and(|pending| pending.metrics_generation == 1_000_000 && pending.physical_width == 1_000_000 && pending.physical_height == 720 && pending.scale_factor == 2.0));
    let mut caller = lane_caller();
    close_lane(&mut lane, &mut caller);
    caller.retire(LANE_STOCK);
}

#[test]
fn zero_size_suspends_and_invalid_scale_returns_exact_producer() {
    let operation = allocate_operation_id();
    let mut lane = match MountedSurfaceResizeLane::try_new(operation) {
        Ok(lane) => lane,
        Err(_) => panic!("fixed resize lane must admit the first test surface"),
    };
    assert!(lane.enqueue(0, 0, 2.0).is_ok());
    let rejected = match lane.enqueue(1280, 720, f32::NAN) {
        Ok(_) => panic!("non-finite scale must be rejected"),
        Err(rejected) => rejected,
    };
    assert_eq!(rejected.physical_width, 1280);
    assert!(rejected.scale_factor.is_nan());
    let mut caller = lane_caller();
    close_lane(&mut lane, &mut caller);
    caller.retire(LANE_STOCK);
}

#[test]
fn a_resize_request_prepares_through_one_admitted_session_and_closes_every_owner() {
    let operation = allocate_operation_id();
    let mut lane = match MountedSurfaceResizeLane::try_new(operation) {
        Ok(lane) => lane,
        Err(_) => panic!("fixed resize lane must admit the first test surface"),
    };
    assert!(lane.enqueue(1280, 720, 2.0).is_ok());
    let mut caller = lane_caller();
    let mut ready = None;
    for _ in 0..512 {
        caller.turn(LANE_TURN, |cx| lane.drive_one(cx)).expect("the lane drive turn is not refused");
        ready = lane.take_ready();
        if ready.is_some() {
            break;
        }
    }
    let prepared = ready.expect("one admitted session prepares the exact latest request");
    assert_eq!(prepared.metrics_generation(), 1);
    assert_eq!((prepared.logical_width(), prepared.logical_height(), prepared.scale_factor()), (640.0, 360.0, 2.0));
    assert!(!prepared.suspended());
    close_lane(&mut lane, &mut caller);
    caller.retire(LANE_STOCK);
}

#[test]
fn interrupted_lane_drop_is_rediscovered_and_incrementally_closed() {
    let operation = allocate_operation_id();
    let mut lane = match MountedSurfaceResizeLane::try_new(operation) {
        Ok(lane) => lane,
        Err(_) => panic!("fixed resize lane must admit the first test surface"),
    };
    assert!(lane.enqueue(1280, 720, 2.0).is_ok());
    let token = lane.token;
    drop(lane);
    let mut caller = lane_caller();
    for _ in 0..16 {
        if caller.turn(LANE_TURN, |cx| MountedSurfaceResizeLane::close_abandoned_step(cx)).expect("the abandoned lane close turn is not refused") {
            break;
        }
    }
    caller.retire(LANE_STOCK);
    let token = match token {
        Some(token) => token,
        None => panic!("admitted lane owns a token"),
    };
    assert!(!SURFACE_LANE_OCCUPIED[token.slot as usize].load(Ordering::Acquire));
}


use super::*;

fn request(generation: u64, width: u32) -> SurfaceResizeRequest {
    SurfaceResizeRequest { token: SurfaceLaneToken { slot: 0, generation: 1 }, metrics_generation: generation, physical_width: width, physical_height: 720, scale_factor: 2.0 }
}

#[test]
fn resize_job_consumes_one_scalar_per_grant() {
    let mut job = SurfaceResizeJob::new(request(1, 1280));
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    let mut cx = StepContext::new(operation, Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, &mut preview_sequence);
    assert_eq!(job.step(&mut cx), StepOutcome::Yield);
    assert_eq!(job.phase, ResizePhase::LogicalHeight);
    assert_eq!(job.logical_width, 640.0);
    assert_eq!(job.logical_height, 0.0);
}

#[test]
fn million_resize_samples_retain_only_the_latest_exact_request() {
    let operation = semio_framework_job::allocate_operation_id();
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
    for _ in 0..8 {
        if lane.close_step() {
            break;
        }
    }
    assert!(lane.terminal_is_empty());
}

#[test]
fn zero_size_suspends_and_invalid_scale_returns_exact_producer() {
    let operation = semio_framework_job::allocate_operation_id();
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
    for _ in 0..8 {
        if lane.close_step() {
            break;
        }
    }
    assert!(lane.terminal_is_empty());
}

#[test]
fn interrupted_lane_drop_is_rediscovered_and_incrementally_closed() {
    let operation = semio_framework_job::allocate_operation_id();
    let mut lane = match MountedSurfaceResizeLane::try_new(operation) {
        Ok(lane) => lane,
        Err(_) => panic!("fixed resize lane must admit the first test surface"),
    };
    assert!(lane.enqueue(1280, 720, 2.0).is_ok());
    let token = lane.token;
    drop(lane);
    for _ in 0..8 {
        if MountedSurfaceResizeLane::close_abandoned_step() {
            break;
        }
    }
    let token = match token {
        Some(token) => token,
        None => panic!("admitted lane owns a token"),
    };
    assert!(!SURFACE_LANE_OCCUPIED[token.slot as usize].load(Ordering::Acquire));
}

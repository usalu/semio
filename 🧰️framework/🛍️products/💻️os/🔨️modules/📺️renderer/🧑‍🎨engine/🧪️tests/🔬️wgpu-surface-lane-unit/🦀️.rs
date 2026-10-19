use super::*;
use semio_framework_job::{OriginalJobFixtureCaller, Generation, RetainedCloneGrant};
fn stock() -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 4096, maximum_copy_bytes: 16 * 1024 * 1024, maximum_capacity_bytes: 16 * 1024 * 1024, maximum_release_bytes: 16 * 1024 * 1024, maximum_depth: 128 } }
fn with_original<R>(run: impl FnOnce(&mut OriginalJobFixtureCaller) -> R) -> R {
    let mut caller = OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(), Generation(1), stock(), || Some(0), u64::MAX, 65536);
    let result = run(&mut caller);
    caller.retire(stock());
    result
}
fn close_lane(lane: &mut MountedSurfaceResizeLane, caller: &mut OriginalJobFixtureCaller) {
    for _ in 0..4096 { if caller.turn(stock(), |cx| lane.close_step(cx)).unwrap() { break; } }
    assert!(lane.terminal_is_empty());
}


fn request(generation: u64, width: u32) -> SurfaceResizeRequest {
    SurfaceResizeRequest { token: SurfaceLaneToken { slot: 0, generation: 1 }, metrics_generation: generation, physical_width: width, physical_height: 720, scale_factor: 2.0 }
}

#[test]
fn resize_job_consumes_one_scalar_per_grant() {
    with_original(|caller| {
        let mut job = SurfaceResizeJob::new(request(1, 1280));
        caller.turn(stock(), |cx| { let deadline = cx.deadline_us(); cx.with_original_work_loan(1, deadline, |same| { assert!(matches!(job.step(same)?, Some(JobOutcomeBorrow::Yield { .. }))); Ok(()) }) }).unwrap();
        assert_eq!(job.phase, ResizePhase::LogicalHeight);
        assert_eq!(job.logical_width, 640.0);
        assert_eq!(job.logical_height, 0.0);
        job.begin_close();
        for _ in 0..8 { if matches!(caller.turn(stock(), |cx| job.close_step(cx)).unwrap(), InteractiveJobCloseStep::Complete { .. }) { break; } }
        assert!(job.terminal_is_empty());
    });
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
    with_original(|caller| close_lane(&mut lane, caller));
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
    with_original(|caller| close_lane(&mut lane, caller));
}

#[test]
fn interrupted_lane_retains_its_original_owner_until_paid_close() {
    with_original(|caller| {
        let mut lane = MountedSurfaceResizeLane::try_new(semio_framework_job::allocate_operation_id()).ok().unwrap();
        assert!(lane.enqueue(1280, 720, 2.0).is_ok());
        let token = lane.token.unwrap();
        let before = caller.cumulative_receipt();
        let denied = RetainedCloneGrant { maximum_items: 0, ..stock() };
        assert!(!caller.turn(denied, |cx| lane.close_step(cx)).unwrap());
        assert_eq!(lane.token, Some(token));
        assert!(lane.pending.is_some());
        assert_eq!(caller.cumulative_receipt(), before);
        assert!(SURFACE_LANE_OCCUPIED[token.slot as usize].load(Ordering::Acquire));
        close_lane(&mut lane, caller);
        assert!(!SURFACE_LANE_OCCUPIED[token.slot as usize].load(Ordering::Acquire));
    });
}

#[test]
fn resize_geometry_matches_the_neutral_sqlite_corpus() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🎯️targets/🧊️wgpu/📐️surface-lane/🧪️tests/🔣️.json")).unwrap();
    with_original(|caller| {
        for row in law["rows"].as_array().unwrap() {
            let mut input = request(1, row["width"].as_u64().unwrap() as u32);
            input.physical_height = row["height"].as_u64().unwrap() as u32;
            input.scale_factor = row["scale"].as_f64().unwrap() as f32;
            let mut job = SurfaceResizeJob::new(input);
            for _ in 0..3 { caller.turn(stock(), |cx| { let deadline = cx.deadline_us(); cx.with_original_work_loan(1, deadline, |same| { let _ = job.step(same)?; Ok(()) }) }).unwrap(); }
            assert_eq!(job.candidate.unwrap().logical_width, row["logicalWidth"].as_f64().unwrap() as f32);
            assert_eq!(job.candidate.unwrap().logical_height, row["logicalHeight"].as_f64().unwrap() as f32);
            job.begin_close();
            for _ in 0..8 { caller.turn(stock(), |cx| { job.close_step(cx)?; Ok(()) }).unwrap(); if job.terminal_is_empty() { break; } }
            assert!(job.terminal_is_empty());
        }
    });
}

#[test]
fn resize_job_closes_its_sealed_candidate_with_one_paid_item() {
    let mut job = SurfaceResizeJob::new(request(2, 1280));
    with_original(|caller| {
    for _ in 0..2 {
        assert!(matches!(caller.turn(stock(), |cx| job.step(cx)), Ok(Some(JobOutcomeBorrow::Yield { .. }))));
    }
    assert!(matches!(caller.turn(stock(), |cx| job.step(cx)), Ok(Some(JobOutcomeBorrow::Complete { .. }))));
    assert!(job.candidate.is_some());
    job.begin_close();
    assert!(matches!(caller.turn(stock(), |cx| job.close_step(cx)), Ok(InteractiveJobCloseStep::Pending { progress }) if progress.copied_items == 1));
    assert!(matches!(caller.turn(stock(), |cx| job.close_step(cx)), Ok(InteractiveJobCloseStep::Complete { .. })));
    assert!(job.terminal_is_empty());
    });
}

#[test]
fn a_resize_request_prepares_through_one_admitted_session_and_closes_every_owner() {
    let operation = semio_framework_job::allocate_operation_id();
    let mut lane = match MountedSurfaceResizeLane::try_new(operation) {
        Ok(lane) => lane,
        Err(_) => panic!("fixed resize lane must admit the first test surface"),
    };
    assert!(lane.enqueue(1280, 720, 2.0).is_ok());
    with_original(|caller| {
    let mut ready = None;
    for _ in 0..512 {
        caller.turn(stock(), |cx| lane.drive_one(cx)).expect("the lane drive turn is not refused");
        ready = lane.take_ready();
        if ready.is_some() {
            break;
        }
    }
    let prepared = ready.expect("one admitted session prepares the exact latest request");
    assert_eq!(prepared.metrics_generation(), 1);
    assert_eq!((prepared.logical_width(), prepared.logical_height(), prepared.scale_factor()), (640.0, 360.0, 2.0));
    assert!(!prepared.suspended());
    close_lane(&mut lane, caller);
    });
}

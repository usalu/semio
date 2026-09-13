
use super::*;

fn inputs(now_ms: f64) -> FrameBuildInputs {
    FrameBuildInputs { wheel_zoom_deadline_ms: 500.0, now_ms }
}

fn compute(inputs: FrameBuildInputs) -> FrameDirectives {
    let params = batch_params(OperationId(1), Generation(1), root_cancel_token());
    let mut session = BatchJobSession::try_new(FrameBuildJob::new(inputs), params).unwrap_or_else(|_| panic!("frame compute session admission"));
    assert!(matches!(session.step(), Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal)));
    // 🎫️ A stepped outcome belongs to the session until it is CHECKED OUT — the same two-call order
    // `ActiveFrameBuild::advance` uses; without it the checked-out job is `None` and the directives
    // this law reads are invisible.
    assert!(session.checkout_outcome(), "frame compute outcome checkout");
    let directives = session.checked_out_job_mut().and_then(FrameBuildJob::take_directives).unwrap_or_else(|| panic!("completed directives"));
    let mut outcome = session.take_outcome().unwrap_or_else(|| panic!("frame compute retained outcome"));
    assert!(matches!(outcome, StepOutcome::Complete(_)));
    while !outcome.terminal_is_empty() {
        let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    session.begin_close();
    while !session.terminal_is_empty() {
        let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    directives
}

#[test]
fn not_yet_expired_deadlines_are_kept() {
    // 🐛 `inputs()` fixes `wheel_zoom_deadline_ms` at 500.0 and the world3d deadline at 1_000.0 —
    // "not yet expired" for BOTH needs `now_ms` before the earlier of the two. Caught by the
    // standalone verify crate's real `cargo test` run (`🧪️frame-job-verify`), not by inspection —
    // see `📓️p3b-frame-building.md` §7 for why this file itself cannot be `cargo test`-ed directly.
    let directives = compute(inputs(100.0));
    assert!(!directives.wheel_zoom_deadline_cleared);
}

#[test]
fn expired_deadline_is_reported() {
    let directives = compute(inputs(1_000.0));
    assert!(directives.wheel_zoom_deadline_cleared);
}

#[test]
fn stale_runtime_frame_generation_is_rejected() {
    assert!(generation_is_fresh(Generation(7), Generation(7)));
    assert!(!generation_is_fresh(Generation(8), Generation(7)));
}

#[test]
fn cancellation_retires_deadline_apply_and_build_phases_to_terminal_empty() {
    let deadline = BatchJobSession::try_new(FrameBuildJob::new(FrameBuildInputs { wheel_zoom_deadline_ms: 0.0, now_ms: 2.0 }), batch_params(OperationId(20), Generation(20), root_cancel_token()))
        .unwrap_or_else(|_| panic!("frame deadline test session admission"));
    let mut phases = [ActiveFramePhase::Deadlines(deadline), ActiveFramePhase::ApplyPending(FrameDirectives { wheel_zoom_deadline_cleared: false })];
    for phase in &mut phases {
        for _ in 0..2_000 {
            if retire_active_phase(phase) {
                break;
            }
        }
        assert!(retire_active_phase(phase));
    }
}

#[test]
fn cancellation_retires_empty_preparation_to_terminal_empty() {
    let build = crate::AppFrameBuild {
        input: ui_wgpu::wgpu::PreparedRenderInput::try_new(1, 1, ui_wgpu::wgpu::DrawList::default(), None, 0.0).unwrap_or_else(|_| panic!("empty fixture preparation admission")),
        engine_packets: crate::FrameEnginePackets::default(),
        generation: Generation(1),
        cursor: ui_wgpu::wgpu::SemioCursor::Default,
        theme_dark: false,
        fullscreen: None,
        cursor_wake: None,
        #[cfg(not(target_arch = "wasm32"))]
        job_progress: None,
    };
    let mut phase = ActiveFramePhase::Prepare(build.into_preparation());
    for _ in 0..100 {
        if retire_active_phase(&mut phase) {
            break;
        }
    }
    assert!(retire_active_phase(&mut phase));
}

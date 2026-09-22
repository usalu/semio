use super::*;

fn frozen_clock() -> Option<u64> {
    Some(0)
}

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
fn retained_frame_owner_turn_advances_once_and_charges_terminal_work() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json")).unwrap_or_else(|_| panic!("frame turn fixture"));
    let turns = fixture["requests"][0]["remainingTurns"].as_u64().unwrap_or_else(|| panic!("frame turn count"));
    let mut calls = 0;
    for turn in 0..turns {
        let mut preview_sequence = 0;
        let mut context = StepContext::new(OperationId(41), Generation(7), semio_framework_job::StepBudget::new(1, u64::MAX), root_cancel_token(), frozen_clock, &mut preview_sequence);
        let step = run_frame_owner_turn(&mut context, || {
            calls += 1;
            if turn + 1 == turns {
                ActiveFrameStep::Complete(None)
            } else {
                ActiveFrameStep::Pending
            }
        });
        assert!(matches!((turn + 1 == turns, step), (false, Some(ActiveFrameStep::Pending)) | (true, Some(ActiveFrameStep::Complete(None)))));
        assert!(context.should_yield(), "one attempted terminal or pending unit consumes one fuel credit");
    }
    assert_eq!(calls, turns, "each retained Worker turn advances exactly once");
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
        input_candidate: None,
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

#[cfg(not(target_arch = "wasm32"))]
fn runtime_with_presented_input_candidate() -> (crate::RuntimeMailbox, crate::shell::PresentedInputCandidateWitness) {
    crate::interpreter::begin_accessibility_visible_documents();
    let mut interaction = crate::AppInteractionState {
        shell: crate::shell::ShellState::new(Vec::new(), "frame-candidate-retirement".to_string()),
        input: ui_wgpu::wgpu::InputState::default(),
        theme: ui_wgpu::wgpu::Theme::default(),
        theme_dark: false,
        last_pointer_x: 0.0,
        last_pointer_y: 0.0,
        pointer_down: false,
        pointer_button: 0,
        pointer_capture: crate::shell::PointerCapture::default(),
        modifiers: ui_wgpu::wgpu::PointerModifiers::default(),
        space_pressed: false,
        wheel_zoom_deadline_ms: 0.0,
        caret_blink_at_ms: 0.0,
        caret_blink_visible: true,
        text_streams: std::array::from_fn(|_| None),
        text_fault: None,
        frame_fault: None,
        text_cancel_pending: false,
        last_sync_pump_ms: 0.0,
    };
    let accepted = interaction.shell.seal_presented_input_candidate(&ui_wgpu::wgpu::Theme::default()).expect("accepted A presented input candidate");
    assert!(interaction.shell.acknowledge_presented_input(&mut interaction.input, accepted), "baseline A is accepted before B is staged");
    let witness = interaction.shell.seal_presented_input_candidate(&ui_wgpu::wgpu::Theme::default()).expect("stale B presented input candidate");
    let runtime = crate::RuntimeMailbox::new(crate::AppRuntime {
        atlas: ui_wgpu::wgpu::FontAtlas::builtin(),
        icons: ui_wgpu::wgpu::IconAtlas::default(),
        icon_rebuild: None,
        icon_raster_scale: 1.0,
        interaction: Some(interaction),
        checkout: Default::default(),
        draw: ui_wgpu::wgpu::DrawList::default(),
        overlay: ui_wgpu::wgpu::DrawList::default(),
        pending_frame_deferred: None,
        frame_actions: crate::FrameActionOwners::default(),
        pending_frame_maintenance_refusal: None,
        plugin_modules_root: Default::default(),
        native_plugin_mtimes: Default::default(),
        native_hot_swap_scan: None,
        native_hot_swap_modified: None,
        native_hot_swap_cursor: 0,
        native_reload_pending: false,
    });
    (runtime, witness)
}

#[cfg(not(target_arch = "wasm32"))]
fn close_active_frame(mut active: ActiveFrameBuild) -> crate::RuntimeMailbox {
    let runtime = active.runtime.clone();
    active.begin_close();
    for _ in 0..262_144 {
        if matches!(InteractiveJob::close_step(&mut active, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
            break;
        }
    }
    assert!(active.terminal_is_empty(), "the abandoned frame owner reaches terminal empty");
    runtime
}

#[cfg(not(target_arch = "wasm32"))]
fn assert_candidate_returned(runtime: &crate::RuntimeMailbox, witness: crate::shell::PresentedInputCandidateWitness) {
    let mut runtime = runtime.try_lock().expect("test runtime lock");
    let interaction = runtime.interaction.as_mut().expect("test interaction owner");
    assert!(!interaction.shell.presented_input_candidate_matches(witness), "the abandoned frame returns its exact sealed input candidate");
    let successor = interaction.shell.seal_presented_input_candidate(&ui_wgpu::wgpu::Theme::default()).expect("successor presented input candidate");
    assert_ne!(successor, witness, "the successor receives a fresh presentation witness");
    assert!(interaction.shell.presented_input_candidate_matches(successor), "the successor owns the one retained candidate slot");
    assert!(interaction.shell.acknowledge_presented_input(&mut interaction.input, successor), "the successor candidate remains publishable after abandonment");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn superseded_frame_build_returns_its_exact_presented_input_candidate() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(91);
    let generation = Generation(91);
    let presentation = crate::RuntimePresentationWitness { scene_revision: 1, input_generation: generation.0 };
    let mut transaction = crate::FrameTransaction::new(FrameDirectives::default(), operation, generation);
    let mut cursor = crate::FrameBuildCursor::new(presentation);
    cursor.input_candidate = Some(witness);
    transaction.build_cursor = Some(cursor);
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, root_cancel_token());
    active.phase = ActiveFramePhase::Build(transaction);
    let runtime = close_active_frame(active);
    assert_candidate_returned(&runtime, witness);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn cancelled_frame_preparation_returns_its_exact_presented_input_candidate() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(92);
    let generation = Generation(92);
    let build = crate::AppFrameBuild {
        input: ui_wgpu::wgpu::PreparedRenderInput::try_new(1, generation.0, ui_wgpu::wgpu::DrawList::default(), None, 0.0).unwrap_or_else(|_| panic!("empty prepared input")),
        input_candidate: Some(witness),
        engine_packets: crate::FrameEnginePackets::default(),
        generation,
        cursor: ui_wgpu::wgpu::SemioCursor::Default,
        theme_dark: false,
        fullscreen: None,
        cursor_wake: None,
        job_progress: None,
    };
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, root_cancel_token());
    active.phase = ActiveFramePhase::Prepare(build.into_preparation());
    let runtime = close_active_frame(active);
    assert_candidate_returned(&runtime, witness);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn stale_completed_frame_returns_its_exact_presented_input_candidate() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(93);
    let generation = Generation(93);
    let frame = crate::AppFramePresentation {
        packet: None,
        input_candidate: Some(witness),
        engine_packets: crate::FrameEnginePackets::default(),
        generation,
        cursor: ui_wgpu::wgpu::SemioCursor::Default,
        theme_dark: false,
        fullscreen: None,
        cursor_wake: None,
        job_progress: None,
    };
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, root_cancel_token());
    active.phase = ActiveFramePhase::Terminal;
    active.completed = Some(frame);
    let runtime = close_active_frame(active);
    assert_candidate_returned(&runtime, witness);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn cancelled_after_chrome_frame_returns_its_exact_presented_input_candidate() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(94);
    let generation = Generation(94);
    let mut transaction = crate::FrameTransaction::new(FrameDirectives::default(), operation, generation);
    transaction.after_chrome = Some(crate::AppFrameAfterChrome {
        resource_input: None,
        input_candidate: Some(witness),
        upload_rejected: None,
        draw_rejected: None,
        engine_packets: None,
        fullscreen: None,
        cursor_wake: None,
        job_progress: None,
        retirement: None,
    });
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, root_cancel_token());
    active.phase = ActiveFramePhase::Build(transaction);
    let runtime = close_active_frame(active);
    assert_candidate_returned(&runtime, witness);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn component_close_transiently_retires_the_creating_frame_and_readmits_the_same_generation() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(95);
    let generation = Generation(95);
    let presentation = crate::RuntimePresentationWitness { scene_revision: 1, input_generation: generation.0 };
    let mut transaction = crate::FrameTransaction::new(FrameDirectives::default(), operation, generation);
    let mut cursor = crate::FrameBuildCursor::new(presentation);
    cursor.input_candidate = Some(witness);
    transaction.build_cursor = Some(cursor);
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, root_cancel_token());
    active.phase = ActiveFramePhase::Build(transaction);

    let wake_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let wake_counter = Arc::clone(&wake_count);
    let mut handle = FrameBuildHandle::new();
    handle.set_completion_waker(Arc::new(move || {
        wake_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }));
    handle.admit_active(active);
    handle.last_submitted_generation = Some(generation);

    assert!(!handle.retire_for_component_surface_close_step(), "one bounded close unit cannot discard the whole retained frame");
    for _ in 0..262_144 {
        if handle.retire_for_component_surface_close_step() {
            break;
        }
    }
    assert!(!handle.has_live_session(), "the exact creating frame reaches terminal before the external owner starts");
    assert!(!handle.closing, "component handoff does not permanently close the reusable frame handle");
    assert!(handle.completion_waker.is_some(), "component handoff preserves the host completion wake authority");
    assert_eq!(handle.last_submitted_generation, None, "native may readmit the same generation after terminal handoff");
    assert_candidate_returned(&runtime, witness);

    let _ = handle.poll_runtime_and_resubmit(runtime, FrameBuildInputs::default(), operation, generation);
    assert!(handle.has_live_session(), "the reusable handle admits a same-generation successor without new host input");
    for _ in 0..262_144 {
        if handle.retire_for_component_surface_close_step() {
            break;
        }
    }
    assert!(!handle.has_live_session());
}

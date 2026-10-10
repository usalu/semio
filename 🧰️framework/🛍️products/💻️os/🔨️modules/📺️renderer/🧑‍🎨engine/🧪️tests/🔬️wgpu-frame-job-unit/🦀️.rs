use super::*;

fn fixture_frame_grant() -> RetainedCloneGrant {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🪙️authority/🖼️frame/🔣️.json")).expect("independent frame root authority");
    let grant = &fixture["grant"];
    RetainedCloneGrant { maximum_items: grant["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: grant["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: grant["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: grant["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: grant["maximumDepth"].as_u64().unwrap() as usize }
}


#[cfg(not(target_arch = "wasm32"))]
#[test]
fn partial_control_deadline_publication_cannot_erase_another_owner_when_a_candidate_is_discarded() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🖱️ui/🖌️render/🧫️fixtures/⏱️deadline/🔣️.json")).unwrap();
    for row in fixture["sources"].as_array().unwrap() {
        runtime.publish_shell_clock_deadline(row["shellDueUs"].as_f64().map(|value| value / 1_000_000.0));
        runtime.publish_retained_control_deadline(row["uiDueUs"].as_f64().map(|value| value / 1_000_000.0));
        assert_eq!(runtime.control_deadline_us(), row["expectedDueUs"].as_u64(), "{}", row["id"]);
        assert!(runtime.try_lock().unwrap().shell.presented_input_candidate_matches(witness));
    }
    let mut scheduler = ui_render::FrameScheduler::new();
    let key = crate::deadlines::RETAINED_CONTROL_CLOCK;
    runtime.publish_shell_clock_deadline(Some(1.4));
    runtime.publish_retained_control_deadline(Some(1.0));
    scheduler.replace_deadline(key, crate::deadlines::retained_control_deadline(runtime.control_deadline_us(), Some(0), 0.0));
    assert!(scheduler.should_render(1.0).is_some());
    runtime.publish_retained_control_deadline(Some(1.6));
    let mut candidate = Some(witness);
    assert!(crate::discard_frame_input_candidate(&runtime, &mut candidate));
    assert!(candidate.is_none());
    assert_eq!(runtime.control_deadline_us(), Some(1_400_000));
    scheduler.replace_deadline(key, crate::deadlines::retained_control_deadline(runtime.control_deadline_us(), Some(1_000_000), 1.0));
    assert!(scheduler.should_render(1.0).is_none());
    assert_eq!(scheduler.next_deadline().map(|value| value.due), Some(1.4));
    assert!(scheduler.should_render(1.4).is_some());
    runtime.publish_shell_clock_deadline(Some(1.8));
    scheduler.replace_deadline(key, crate::deadlines::retained_control_deadline(runtime.control_deadline_us(), Some(1_400_000), 1.4));
    assert!(scheduler.should_render(1.4).is_none());
    assert!((scheduler.next_deadline().unwrap().due - 1.6).abs() < f64::EPSILON * 2.0);
    runtime.publish_shell_clock_deadline(None);
    assert_eq!(runtime.control_deadline_us(), Some(1_600_000));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn retained_clock_publication_preserves_the_live_frame_and_uses_its_completion_wake() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let wakes = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = wakes.clone();
    runtime.set_waker(Arc::new(move || {
        observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }));
    let before = runtime.0.presentation_authority.current();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🖱️ui/🖌️render/🧫️fixtures/⏱️deadline/🔣️.json")).unwrap();
    for row in fixture["bridges"].as_array().unwrap() {
        let seconds = row["dueUs"].as_f64().map(|microseconds| microseconds / 1_000_000.0);
        runtime.publish_retained_control_deadline(seconds);
        assert_eq!(runtime.0.retained_control_deadline_us.load(std::sync::atomic::Ordering::Acquire), row["dueUs"].as_u64().unwrap_or(u64::MAX), "{}", row["id"]);
        assert_eq!(runtime.0.presentation_authority.current(), before);
        assert!(runtime.try_lock().unwrap().shell.presented_input_candidate_matches(witness));
    }
    assert_eq!(wakes.load(std::sync::atomic::Ordering::SeqCst), 0, "deadline publication rides the existing frame completion; generic Wake would supersede this build");
    assert!(runtime.retained_control_deadline(0.0).is_none());
    let mut candidate = Some(witness);
    assert!(crate::discard_frame_input_candidate(&runtime, &mut candidate));
    assert!(candidate.is_none());
}

fn frozen_clock() -> Option<u64> {
    Some(0)
}

fn inputs(now_ms: f64) -> FrameBuildInputs {
    FrameBuildInputs { wheel_zoom_deadline_ms: 500.0, now_ms }
}

/// 🎫️ A stepped outcome belongs to the session until it is CHECKED OUT — the same two-call order
/// `ActiveFrameBuild::advance` uses; without it the checked-out job is `None` and the directives
/// this law reads are invisible.
fn compute(inputs: FrameBuildInputs) -> FrameDirectives {
    let params = batch_params(OperationId(1), Generation(1), root_cancel_token(), fixture_frame_grant());
    let mut session = BatchJobSession::try_new(FrameBuildJob::new(inputs), params).unwrap_or_else(|_| panic!("frame compute session admission"));
    assert!(matches!(session.step(), Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal)));
    assert!(session.checkout_outcome(), "frame compute outcome checkout");
    let directives = session.checked_out_job_mut().and_then(FrameBuildJob::take_directives).unwrap_or_else(|| panic!("completed directives"));
    let mut outcome = session.take_outcome().unwrap_or_else(|| panic!("frame compute retained outcome"));
    assert!(matches!(outcome, StepOutcome::Complete(_)));
    while !outcome.terminal_is_empty() {
        let _ = outcome.close_step(fixture_frame_grant());
    }
    session.begin_close();
    while !session.terminal_is_empty() {
        let _ = session.close_step(fixture_frame_grant());
    }
    directives
}

/// 🐛 `inputs()` fixes `wheel_zoom_deadline_ms` at 500.0 and the world3d deadline at 1_000.0 —
/// "not yet expired" for BOTH needs `now_ms` before the earlier of the two. Caught by the
/// standalone verify crate's real `cargo test` run (`🧪️frame-job-verify`), not by inspection —
/// see `📓️p3b-frame-building.md` §7 for why this file itself cannot be `cargo test`-ed directly.
#[test]
fn not_yet_expired_deadlines_are_kept() {
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
        let mut retained_progress = RetainedCloneProgress::default();
        let mut context = StepContext::new(OperationId(41), Generation(7), semio_framework_job::StepBudget::new(1, u64::MAX, fixture_frame_grant()), root_cancel_token(), frozen_clock, &mut preview_sequence, &mut retained_progress);
        let step = run_frame_owner_turn(&mut context, |_| {
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
    let deadline = BatchJobSession::try_new(FrameBuildJob::new(FrameBuildInputs { wheel_zoom_deadline_ms: 0.0, now_ms: 2.0 }), batch_params(OperationId(20), Generation(20), root_cancel_token(), fixture_frame_grant()))
        .unwrap_or_else(|_| panic!("frame deadline test session admission"));
    let mut phases = [ActiveFramePhase::Deadlines(deadline), ActiveFramePhase::ApplyPending(FrameDirectives { wheel_zoom_deadline_cleared: false })];
    for phase in &mut phases {
        for _ in 0..2_000 {
            if matches!(retire_active_phase(phase, fixture_frame_grant()), InteractiveJobCloseStep::Complete { .. }) {
                break;
            }
        }
        assert!(matches!(retire_active_phase(phase, fixture_frame_grant()), InteractiveJobCloseStep::Complete { .. }));
    }
}

#[test]
fn cancellation_retires_empty_preparation_to_terminal_empty() {
    let build = crate::AppFrameBuild {
        retained: fixture_frame_grant(),
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
        if matches!(retire_active_phase(&mut phase, fixture_frame_grant()), InteractiveJobCloseStep::Complete { .. }) {
            break;
        }
    }
    assert!(matches!(retire_active_phase(&mut phase, fixture_frame_grant()), InteractiveJobCloseStep::Complete { .. }));
}

#[cfg(not(target_arch = "wasm32"))]
fn runtime_with_presented_input_candidate() -> (crate::RuntimeMailbox, crate::shell::PresentedInputCandidateWitness) {
    crate::interpreter::begin_accessibility_visible_documents();
    let mut interaction = crate::AppInteractionState {
        shell: crate::shell::ShellState::new(Vec::new(), "frame-candidate-retirement".to_string(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native),
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
        if matches!(InteractiveJob::close_step(&mut active, fixture_frame_grant()), semio_framework_job::InteractiveJobCloseStep::Complete { .. }) {
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
    let mut transaction = crate::FrameTransaction::new(FrameDirectives::default(), operation, generation, fixture_frame_grant());
    let mut cursor = crate::FrameBuildCursor::new(presentation, fixture_frame_grant());
    cursor.input_candidate = Some(witness);
    transaction.build_cursor = Some(cursor);
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, root_cancel_token(), fixture_frame_grant());
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
        retained: fixture_frame_grant(),
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
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, root_cancel_token(), fixture_frame_grant());
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
        retained: fixture_frame_grant(),
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
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, root_cancel_token(), fixture_frame_grant());
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
    let mut transaction = crate::FrameTransaction::new(FrameDirectives::default(), operation, generation, fixture_frame_grant());
    transaction.after_chrome =
        Some(crate::AppFrameAfterChrome { retained: fixture_frame_grant(), resource_input: None, input_candidate: Some(witness), upload_rejected: None, draw_rejected: None, engine_packets: None, fullscreen: None, cursor_wake: None, job_progress: None, retirement: None });
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, root_cancel_token(), fixture_frame_grant());
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
    let mut transaction = crate::FrameTransaction::new(FrameDirectives::default(), operation, generation, fixture_frame_grant());
    let mut cursor = crate::FrameBuildCursor::new(presentation, fixture_frame_grant());
    cursor.input_candidate = Some(witness);
    transaction.build_cursor = Some(cursor);
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, root_cancel_token(), fixture_frame_grant());
    active.phase = ActiveFramePhase::Build(transaction);

    let wake_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let wake_counter = Arc::clone(&wake_count);
    let mut handle = FrameBuildHandle::new(fixture_frame_grant());
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

#[test]
fn original_frame_directive_inline_retirement_preserves_the_full_independent_grant() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/♻️frame-close/🔣️.json")).expect("frame close corpus");
    let caller = &fixture["caller"];
    let grant = RetainedCloneGrant {
        maximum_items: caller["maximumItems"].as_u64().unwrap() as usize,
        maximum_copy_bytes: caller["maximumCopyBytes"].as_u64().unwrap() as usize,
        maximum_capacity_bytes: caller["maximumCapacityBytes"].as_u64().unwrap() as usize,
        maximum_release_bytes: caller["maximumReleaseBytes"].as_u64().unwrap() as usize,
        maximum_depth: caller["maximumDepth"].as_u64().unwrap() as usize,
    };
    let mut job = FrameBuildJob::new(inputs(1_000.0));
    let mut sequence = 0;
    let mut retained_progress = RetainedCloneProgress::default();
    let mut context = StepContext::new(OperationId(99), Generation(99), semio_framework_job::StepBudget::new(1, u64::MAX, fixture_frame_grant()), root_cancel_token(), frozen_clock, &mut sequence, &mut retained_progress);
    assert!(job.step(&mut context).unwrap().is_none());
    assert_eq!(context.retained_progress(), RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Option<FrameDirectives>>(), ..RetainedCloneProgress::default() });
    job.begin_close();
    let original = job.complete.as_ref().unwrap() as *const FrameDirectives;
    assert_eq!(job.next_close_copy_byte_demand().unwrap(), size_of::<FrameDirectives>());
    assert_eq!(job.next_close_capacity_byte_demand(0).unwrap(), 0);
    assert_eq!(job.next_close_release_byte_demand().unwrap(), 0);
    assert_eq!(job.next_close_depth_demand().unwrap(), 1);
    for short in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }] {
        assert_eq!(job.close_step(short), InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        assert_eq!(job.complete.as_ref().unwrap() as *const FrameDirectives, original);
    }
    assert_eq!(job.close_step(RetainedCloneGrant { maximum_depth: 0, ..grant }), InteractiveJobCloseStep::Refused { kind: semio_framework_value::ValueRefusalKind::DepthLimit, progress: RetainedCloneProgress::default() });
    assert_eq!(job.complete.as_ref().unwrap() as *const FrameDirectives, original);
    let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<FrameDirectives>(), ..RetainedCloneProgress::default() };
    assert_eq!(job.close_step(grant), InteractiveJobCloseStep::Pending { progress: receipt });
    assert!(receipt.fits(grant));
    assert!(job.terminal_is_empty());
    assert_eq!(job.next_close_copy_byte_demand().unwrap(), 0);
    assert_eq!(job.close_step(RetainedCloneGrant::default()), InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
    eprintln!("[DEBUG] original frame directive copy={} physicalRelease={}", receipt.copied_bytes, receipt.released_bytes);
}

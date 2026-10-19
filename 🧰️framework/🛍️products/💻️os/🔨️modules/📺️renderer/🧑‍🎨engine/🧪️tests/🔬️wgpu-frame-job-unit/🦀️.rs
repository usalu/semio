use super::*;
use semio_framework_os_renderer_wgpu_app_runtime::{AppFrameAfterChrome, AppFrameBuild, AppFramePresentation, AppMailbox, AppRuntime, FrameBuildCursor, FrameEnginePackets, discard_frame_input_candidate};
use semio_framework_os_renderer_wgpu_frame_actions::{AppInteractionState, FrameActionOwners};
use semio_framework_os_renderer_wgpu_frame_transaction::FrameTransaction;
use semio_framework_os_renderer_wgpu_host_plumbing::deadlines;
use semio_framework_os_renderer_wgpu_runtime_mailbox::{RuntimeMailbox, RuntimePresentationWitness};
use semio_framework_os_renderer_wgpu_shell_input::{PointerCapture, PresentedInputCandidateWitness};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_view_chrome::ChromeViewCtx;
use semio_framework_os_renderer_wgpu_shell_view_panels::PanelsViewCtx;
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger;
use semio_framework_ui_locale::{Locale, Terminology};
use semio_framework_job::{root_cancel_token, OriginalJobFixtureCaller};
use serde_json::Value;
use std::sync::atomic::Ordering;
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_wgpu_input::{InputState, PointerModifiers};
use semio_framework_ui_wgpu_hosting::cursor::SemioCursor;
use semio_framework_ui_theme::Theme;

fn fixture_frame_grant() -> RetainedCloneGrant {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🪙️authority/🖼️frame/🔣️.json")).expect("independent frame root authority");
    let grant = &fixture["grant"];
    RetainedCloneGrant { maximum_items: grant["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: grant["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: grant["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: grant["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: grant["maximumDepth"].as_u64().unwrap() as usize }
}


/// 🪙️ The one stock the initiating fixture caller issues for a whole law.
const FRAME_STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: usize::MAX / 4, maximum_copy_bytes: usize::MAX / 4, maximum_capacity_bytes: usize::MAX / 4, maximum_release_bytes: usize::MAX / 4, maximum_depth: 64 };

fn frame_caller(operation: OperationId, generation: Generation) -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(operation, generation, FRAME_STOCK, frozen_clock, u64::MAX, u64::MAX)
}

/// ♻️ Drives one phase through the independent frame authority, one paid unit per caller turn, to terminal empty.
fn retire_phase(phase: &mut ActiveFramePhase, caller: &mut OriginalJobFixtureCaller) {
    for _ in 0..262_144 {
        let step = caller
            .turn(fixture_frame_grant(), |cx| {
                let witness = cx.original_cancel_token().clone();
                retire_active_phase(phase, cx, &witness)
            })
            .expect("the original phase retirement turn is not refused");
        if matches!(step, InteractiveJobCloseStep::Complete { .. }) {
            return;
        }
    }
    panic!("the original phase never reached terminal empty");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn partial_control_deadline_publication_cannot_erase_another_owner_when_a_candidate_is_discarded() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🖱️ui/🖌️render/🧪️tests/🧫️fixtures/⏱️deadline/🔣️.json")).unwrap();
    for row in fixture["sources"].as_array().unwrap() {
        runtime.publish_shell_clock_deadline(row["shellDueUs"].as_f64().map(|value| value / 1_000_000.0));
        runtime.publish_retained_control_deadline(row["uiDueUs"].as_f64().map(|value| value / 1_000_000.0));
        assert_eq!(runtime.control_deadline_us(), row["expectedDueUs"].as_u64(), "{}", row["id"]);
        assert!(runtime.try_lock().unwrap().shell.input.presented_input_candidate_matches(witness));
    }
    let mut scheduler = ui_render::FrameScheduler::new();
    let key = semio_framework_os_renderer_wgpu_host_plumbing::deadlines::RETAINED_CONTROL_CLOCK;
    runtime.publish_shell_clock_deadline(Some(1.4));
    runtime.publish_retained_control_deadline(Some(1.0));
    scheduler.replace_deadline(key, semio_framework_os_renderer_wgpu_host_plumbing::deadlines::retained_control_deadline(runtime.control_deadline_us(), Some(0), 0.0));
    assert!(scheduler.should_render(1.0).is_some());
    runtime.publish_retained_control_deadline(Some(1.6));
    let mut candidate = Some(witness);
    assert!(semio_framework_os_renderer_wgpu_app_runtime::discard_frame_input_candidate(&runtime, &mut candidate));
    assert!(candidate.is_none());
    assert_eq!(runtime.control_deadline_us(), Some(1_400_000));
    scheduler.replace_deadline(key, semio_framework_os_renderer_wgpu_host_plumbing::deadlines::retained_control_deadline(runtime.control_deadline_us(), Some(1_000_000), 1.0));
    assert!(scheduler.should_render(1.0).is_none());
    assert_eq!(scheduler.next_deadline().map(|value| value.due), Some(1.4));
    assert!(scheduler.should_render(1.4).is_some());
    runtime.publish_shell_clock_deadline(Some(1.8));
    scheduler.replace_deadline(key, semio_framework_os_renderer_wgpu_host_plumbing::deadlines::retained_control_deadline(runtime.control_deadline_us(), Some(1_400_000), 1.4));
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🖱️ui/🖌️render/🧪️tests/🧫️fixtures/⏱️deadline/🔣️.json")).unwrap();
    for row in fixture["bridges"].as_array().unwrap() {
        let seconds = row["dueUs"].as_f64().map(|microseconds| microseconds / 1_000_000.0);
        runtime.publish_retained_control_deadline(seconds);
        assert_eq!(runtime.0.retained_control_deadline_us.load(std::sync::atomic::Ordering::Acquire), row["dueUs"].as_u64().unwrap_or(u64::MAX), "{}", row["id"]);
        assert_eq!(runtime.0.presentation_authority.current(), before);
        assert!(runtime.try_lock().unwrap().shell.input.presented_input_candidate_matches(witness));
    }
    assert_eq!(wakes.load(std::sync::atomic::Ordering::SeqCst), 0, "deadline publication rides the existing frame completion; generic Wake would supersede this build");
    assert!(runtime.retained_control_deadline(0.0).is_none());
    let mut candidate = Some(witness);
    assert!(semio_framework_os_renderer_wgpu_app_runtime::discard_frame_input_candidate(&runtime, &mut candidate));
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
    let (operation, generation) = (OperationId(1), Generation(1));
    let grant = fixture_frame_grant();
    let mut caller = frame_caller(operation, generation);
    let cancel = caller.admit_alias(grant);
    let mut job = Some(FrameBuildJob::new(inputs));
    let mut params = Some(BatchJobParams { operation, generation, cancel, now_us: frozen_clock, config: BatchDriveConfig { retained: grant, site: "os_renderer_frame_deadlines", stage: InteractiveStage::InteractiveStep, fuel_per_step: 8, step_budget_us: 1_000 } });
    let admitted = caller.turn(grant, |cx| BatchJobSession::try_admit_owned(&mut job, &mut params, cx).map_err(OriginalTurnFailure::from)).expect("the original admission turn is not refused");
    let (mut session, _posted) = admitted.unwrap_or_else(|| panic!("frame compute session admission"));
    let mut polled = false;
    for _ in 0..8 {
        let poll = caller.turn(grant, |cx| session.step_on_original_caller(cx)).expect("the original deadline step is not refused");
        if matches!(poll, semio_framework_job_worker::WorkerJobPoll::Outcome | semio_framework_job_worker::WorkerJobPoll::Terminal) {
            polled = true;
            break;
        }
    }
    assert!(polled, "frame compute reaches its outcome");
    assert!(session.checkout_outcome(), "frame compute outcome checkout");
    assert!(session.take_checked_out_retained_step_receipt().is_some(), "the original step keeps its physical receipt until the caller takes it");
    for _ in 0..8 {
        let step = caller
            .turn(grant, |cx| {
                let step = session.acknowledge_outcome(cx.retained_grant());
                cx.consume_retained(step.progress())?;
                Ok(step)
            })
            .expect("the original acknowledgement turn is not refused");
        if matches!(step, semio_framework_job::RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    let directives = session.checked_out_job_mut().and_then(FrameBuildJob::take_directives).unwrap_or_else(|| panic!("completed directives"));
    session.begin_close();
    let mut phase = ActiveFramePhase::Deadlines(session);
    retire_phase(&mut phase, &mut caller);
    caller.retire(FRAME_STOCK);
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json")).unwrap_or_else(|_| panic!("frame turn fixture"));
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
    let (operation, generation) = (OperationId(20), Generation(20));
    let grant = fixture_frame_grant();
    let mut caller = frame_caller(operation, generation);
    let cancel = caller.admit_alias(grant);
    let mut job = Some(FrameBuildJob::new(FrameBuildInputs { wheel_zoom_deadline_ms: 0.0, now_ms: 2.0 }));
    let mut params = Some(BatchJobParams { operation, generation, cancel, now_us: frozen_clock, config: BatchDriveConfig { retained: grant, site: "os_renderer_frame_deadlines", stage: InteractiveStage::InteractiveStep, fuel_per_step: 8, step_budget_us: 1_000 } });
    let admitted = caller.turn(grant, |cx| BatchJobSession::try_admit_owned(&mut job, &mut params, cx).map_err(OriginalTurnFailure::from)).expect("the original admission turn is not refused");
    let (deadline, _posted) = admitted.unwrap_or_else(|| panic!("frame deadline test session admission"));
    let mut phases = [ActiveFramePhase::Deadlines(deadline), ActiveFramePhase::ApplyPending(FrameDirectives { wheel_zoom_deadline_cleared: false })];
    for phase in &mut phases {
        retire_phase(phase, &mut caller);
        assert!(matches!(phase, ActiveFramePhase::Terminal));
    }
    caller.retire(FRAME_STOCK);
}

#[test]
fn cancellation_retires_empty_preparation_to_terminal_empty() {
    let build = semio_framework_os_renderer_wgpu_app_runtime::AppFrameBuild {
        retained: fixture_frame_grant(),
        input: semio_framework_ui_prepared::PreparedRenderInput::try_new(1, 1, semio_framework_ui_draw_contract::DrawList::default(), None, 0.0).unwrap_or_else(|_| panic!("empty fixture preparation admission")),
        input_candidate: None,
        engine_packets: semio_framework_os_renderer_wgpu_app_runtime::FrameEnginePackets::default(),
        generation: Generation(1),
        cursor: semio_framework_ui_wgpu_hosting::cursor::SemioCursor::Default,
        theme_dark: false,
        fullscreen: None,
        cursor_wake: None,
        #[cfg(not(target_arch = "wasm32"))]
        job_progress: None,
    };
    let mut phase = ActiveFramePhase::Prepare(build.into_preparation());
    let mut caller = frame_caller(OperationId(21), Generation(1));
    retire_phase(&mut phase, &mut caller);
    assert!(matches!(phase, ActiveFramePhase::Terminal));
    caller.retire(FRAME_STOCK);
}

#[cfg(not(target_arch = "wasm32"))]
fn runtime_with_presented_input_candidate() -> (semio_framework_os_renderer_wgpu_app_runtime::AppMailbox, semio_framework_os_renderer_wgpu_shell_input::PresentedInputCandidateWitness) {
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::begin_accessibility_visible_documents();
    let mut interaction = semio_framework_os_renderer_wgpu_frame_actions::AppInteractionState {
        shell: semio_framework_os_renderer_wgpu_shell_state::ShellState::new(Vec::new(), "frame-candidate-retirement".to_string(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native),
        input: semio_framework_ui_wgpu_input::InputState::default(),
        theme: semio_framework_ui_theme::Theme::default(),
        theme_dark: false,
        last_pointer_x: 0.0,
        last_pointer_y: 0.0,
        pointer_down: false,
        pointer_button: 0,
        pointer_capture: semio_framework_os_renderer_wgpu_shell_input::PointerCapture::default(),
        modifiers: semio_framework_ui_wgpu_input::PointerModifiers::default(),
        space_pressed: false,
        wheel_zoom_deadline_ms: 0.0,
        text_streams: std::array::from_fn(|_| None),
        text_fault: None,
        frame_fault: None,
        text_cancel_pending: false,
        last_sync_pump_ms: 0.0,
    };
    let accepted = semio_framework_os_renderer_wgpu_shell_view_chrome::ChromeViewCtx::of(&mut interaction.shell).seal_presented_input_candidate(&semio_framework_ui_theme::Theme::default()).expect("accepted A presented input candidate");
    assert!(semio_framework_os_renderer_wgpu_shell_view_panels::PanelsViewCtx::of(&mut interaction.shell).acknowledge_presented_input(&mut interaction.input, accepted), "baseline A is accepted before B is staged");
    let witness = semio_framework_os_renderer_wgpu_shell_view_chrome::ChromeViewCtx::of(&mut interaction.shell).seal_presented_input_candidate(&semio_framework_ui_theme::Theme::default()).expect("stale B presented input candidate");
    let runtime = semio_framework_os_renderer_wgpu_runtime_mailbox::RuntimeMailbox::new(semio_framework_os_renderer_wgpu_app_runtime::AppRuntime {
        atlas: semio_framework_ui_text::FontAtlas::builtin(),
        icons: semio_framework_ui_draw_contract::IconAtlas::default(),
        icon_rebuild: None,
        icon_raster_scale: 1.0,
        interaction: Some(interaction),
        checkout: Default::default(),
        draw: semio_framework_ui_draw_contract::DrawList::default(),
        overlay: semio_framework_ui_draw_contract::DrawList::default(),
        pending_frame_deferred: None,
        frame_actions: semio_framework_os_renderer_wgpu_frame_actions::FrameActionOwners::default(),
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
fn close_active_frame(mut active: ActiveFrameBuild, mut caller: OriginalJobFixtureCaller) -> semio_framework_os_renderer_wgpu_app_runtime::AppMailbox {
    let runtime = active.runtime.clone();
    for _ in 0..262_144 {
        let step = caller.turn(fixture_frame_grant(), |cx| InteractiveJob::close_step(&mut active, cx)).expect("the original frame close turn is not refused");
        if matches!(step, InteractiveJobCloseStep::Complete { .. }) {
            break;
        }
    }
    assert!(active.terminal_is_empty(), "the abandoned frame owner reaches terminal empty");
    drop(active);
    caller.retire(FRAME_STOCK);
    runtime
}

#[cfg(not(target_arch = "wasm32"))]
fn assert_candidate_returned(runtime: &semio_framework_os_renderer_wgpu_app_runtime::AppMailbox, witness: semio_framework_os_renderer_wgpu_shell_input::PresentedInputCandidateWitness) {
    let mut runtime = runtime.try_lock().expect("test runtime lock");
    let interaction = runtime.interaction.as_mut().expect("test interaction owner");
    assert!(!interaction.shell.input.presented_input_candidate_matches(witness), "the abandoned frame returns its exact sealed input candidate");
    let successor = semio_framework_os_renderer_wgpu_shell_view_chrome::ChromeViewCtx::of(&mut interaction.shell).seal_presented_input_candidate(&semio_framework_ui_theme::Theme::default()).expect("successor presented input candidate");
    assert_ne!(successor, witness, "the successor receives a fresh presentation witness");
    assert!(interaction.shell.input.presented_input_candidate_matches(successor), "the successor owns the one retained candidate slot");
    assert!(semio_framework_os_renderer_wgpu_shell_view_panels::PanelsViewCtx::of(&mut interaction.shell).acknowledge_presented_input(&mut interaction.input, successor), "the successor candidate remains publishable after abandonment");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn superseded_frame_build_returns_its_exact_presented_input_candidate() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(91);
    let generation = Generation(91);
    let presentation = semio_framework_os_renderer_wgpu_runtime_mailbox::RuntimePresentationWitness { scene_revision: 1, input_generation: generation.0 };
    let mut transaction = semio_framework_os_renderer_wgpu_frame_transaction::FrameTransaction::new(FrameDirectives::default(), operation, generation, fixture_frame_grant());
    let mut cursor = semio_framework_os_renderer_wgpu_app_runtime::FrameBuildCursor::new(presentation, fixture_frame_grant());
    cursor.input_candidate = Some(witness);
    transaction.build_cursor = Some(cursor);
    let mut caller = frame_caller(operation, generation);
    let cancel = caller.admit_alias(fixture_frame_grant());
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, cancel, fixture_frame_grant());
    active.phase = ActiveFramePhase::Build(transaction);
    let runtime = close_active_frame(active, caller);
    assert_candidate_returned(&runtime, witness);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn cancelled_frame_preparation_returns_its_exact_presented_input_candidate() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(92);
    let generation = Generation(92);
    let build = semio_framework_os_renderer_wgpu_app_runtime::AppFrameBuild {
        retained: fixture_frame_grant(),
        input: semio_framework_ui_prepared::PreparedRenderInput::try_new(1, generation.0, semio_framework_ui_draw_contract::DrawList::default(), None, 0.0).unwrap_or_else(|_| panic!("empty prepared input")),
        input_candidate: Some(witness),
        engine_packets: semio_framework_os_renderer_wgpu_app_runtime::FrameEnginePackets::default(),
        generation,
        cursor: semio_framework_ui_wgpu_hosting::cursor::SemioCursor::Default,
        theme_dark: false,
        fullscreen: None,
        cursor_wake: None,
        job_progress: None,
    };
    let mut caller = frame_caller(operation, generation);
    let cancel = caller.admit_alias(fixture_frame_grant());
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, cancel, fixture_frame_grant());
    active.phase = ActiveFramePhase::Prepare(build.into_preparation());
    let runtime = close_active_frame(active, caller);
    assert_candidate_returned(&runtime, witness);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn stale_completed_frame_returns_its_exact_presented_input_candidate() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(93);
    let generation = Generation(93);
    let frame = semio_framework_os_renderer_wgpu_app_runtime::AppFramePresentation {
        retained: fixture_frame_grant(),
        packet: None,
        input_candidate: Some(witness),
        engine_packets: semio_framework_os_renderer_wgpu_app_runtime::FrameEnginePackets::default(),
        generation,
        cursor: semio_framework_ui_wgpu_hosting::cursor::SemioCursor::Default,
        theme_dark: false,
        fullscreen: None,
        cursor_wake: None,
        job_progress: None,
    };
    let mut caller = frame_caller(operation, generation);
    let cancel = caller.admit_alias(fixture_frame_grant());
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, cancel, fixture_frame_grant());
    active.phase = ActiveFramePhase::Terminal;
    active.completed = Some(frame);
    let runtime = close_active_frame(active, caller);
    assert_candidate_returned(&runtime, witness);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn cancelled_after_chrome_frame_returns_its_exact_presented_input_candidate() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(94);
    let generation = Generation(94);
    let mut transaction = semio_framework_os_renderer_wgpu_frame_transaction::FrameTransaction::new(FrameDirectives::default(), operation, generation, fixture_frame_grant());
    transaction.after_chrome =
        Some(semio_framework_os_renderer_wgpu_app_runtime::AppFrameAfterChrome { retained: fixture_frame_grant(), resource_input: None, input_candidate: Some(witness), upload_rejected: None, draw_rejected: None, engine_packets: None, fullscreen: None, cursor_wake: None, job_progress: None, retirement: None });
    let mut caller = frame_caller(operation, generation);
    let cancel = caller.admit_alias(fixture_frame_grant());
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, cancel, fixture_frame_grant());
    active.phase = ActiveFramePhase::Build(transaction);
    let runtime = close_active_frame(active, caller);
    assert_candidate_returned(&runtime, witness);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn component_close_transiently_retires_the_creating_frame_and_readmits_the_same_generation() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(95);
    let generation = Generation(95);
    let presentation = semio_framework_os_renderer_wgpu_runtime_mailbox::RuntimePresentationWitness { scene_revision: 1, input_generation: generation.0 };
    let mut transaction = semio_framework_os_renderer_wgpu_frame_transaction::FrameTransaction::new(FrameDirectives::default(), operation, generation, fixture_frame_grant());
    let mut cursor = semio_framework_os_renderer_wgpu_app_runtime::FrameBuildCursor::new(presentation, fixture_frame_grant());
    cursor.input_candidate = Some(witness);
    transaction.build_cursor = Some(cursor);
    let mut caller = frame_caller(operation, generation);
    let cancel = caller.admit_alias(fixture_frame_grant());
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, cancel, fixture_frame_grant());
    active.phase = ActiveFramePhase::Build(transaction);

    let wake_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let wake_counter = Arc::clone(&wake_count);
    let mut handle = FrameBuildHandle::new();
    handle.set_completion_waker(Arc::new(move || {
        wake_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }));
    handle.original_job = Some(active);
    handle.original_presentation_generation = Some(generation);
    let grant = fixture_frame_grant();

    assert!(!caller.turn(grant, |cx| handle.retire_for_component_surface_close_step(cx)).expect("the original retirement turn is not refused"), "one bounded close unit cannot discard the whole retained frame");
    for _ in 0..262_144 {
        if caller.turn(grant, |cx| handle.retire_for_component_surface_close_step(cx)).expect("the original retirement turn is not refused") {
            break;
        }
    }
    assert!(!handle.has_live_session(), "the exact creating frame reaches terminal before the external owner starts");
    assert!(!handle.closing, "component handoff does not permanently close the reusable frame handle");
    assert!(handle.completion_waker.is_some(), "component handoff preserves the host completion wake authority");
    assert!(handle.original_presentation_generation.is_none(), "the same generation may be readmitted after terminal handoff");
    assert_candidate_returned(&runtime, witness);

    for _ in 0..8 {
        let _ = caller.turn(grant, |cx| handle.poll_runtime_on_original_caller(runtime.clone(), FrameBuildInputs::default(), generation, cx)).expect("the original readmission turn is not refused");
        if handle.has_live_session() {
            break;
        }
    }
    assert!(handle.has_live_session(), "the reusable handle admits a same-generation successor without new host input");
    for _ in 0..262_144 {
        if caller.turn(grant, |cx| handle.retire_for_component_surface_close_step(cx)).expect("the original retirement turn is not refused") {
            break;
        }
    }
    assert!(!handle.has_live_session());
    assert!(handle.original_event_terminal_is_empty());
    for _ in 0..262_144 {
        if caller.turn(grant, |cx| handle.close_original_frame_step(cx)).expect("the original handle close turn is not refused") {
            break;
        }
    }
    assert!(handle.terminal_is_empty());
    caller.retire(FRAME_STOCK);
}

#[test]
fn original_frame_directive_inline_retirement_preserves_the_full_independent_grant() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/♻️frame-close/🔣️.json")).expect("frame close corpus");
    let caller = &fixture["caller"];
    let grant = RetainedCloneGrant {
        maximum_items: caller["maximumItems"].as_u64().unwrap() as usize,
        maximum_copy_bytes: caller["maximumCopyBytes"].as_u64().unwrap() as usize,
        maximum_capacity_bytes: caller["maximumCapacityBytes"].as_u64().unwrap() as usize,
        maximum_release_bytes: caller["maximumReleaseBytes"].as_u64().unwrap() as usize,
        maximum_depth: caller["maximumDepth"].as_u64().unwrap() as usize,
    };
    let source=fixture_frame_grant();
    let ((cancel,cancel_birth),cancel_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::CancelToken::admit_root(source).unwrap().unwrap());
    assert_eq!((cancel_birth.retained_capacity_bytes,cancel_birth.released_bytes),(cancel_heap.requested_bytes,cancel_heap.released_bytes));
    let ((mut authority,ledger_birth),ledger_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::JobPayloadAuthority::admit(OperationId(99),Generation(99),source).unwrap().unwrap());
    assert_eq!((ledger_birth.retained_capacity_bytes,ledger_birth.released_bytes),(ledger_heap.requested_bytes,ledger_heap.released_bytes));
    let mut job = FrameBuildJob::new(inputs(1_000.0));
    let mut sequence=0;
    let mut receipt=RetainedCloneProgress::default();
    {
        let mut cx=StepContext::with_payload_authority(OperationId(99),Generation(99),semio_framework_job::StepBudget::new(1,u64::MAX,source),&cancel,frozen_clock,&mut sequence,&mut receipt,&authority).unwrap();
        assert!(job.step(&mut cx).unwrap().is_none());
        assert_eq!(cx.retained_progress(),RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<FrameDirectives>>(),..Default::default()});
    }
    job.begin_close();
    let original=job.complete.as_ref().unwrap() as *const FrameDirectives;
    assert_eq!(job.next_close_copy_byte_demand().unwrap(),size_of::<FrameDirectives>());
    assert_eq!(job.next_close_capacity_byte_demand(0).unwrap(),0);
    assert_eq!(job.next_close_release_byte_demand().unwrap(),0);
    assert_eq!(job.next_close_depth_demand().unwrap(),1);
    for short in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
        let mut recorded=Default::default();
        let mut cx=StepContext::with_payload_authority(OperationId(99),Generation(99),semio_framework_job::StepBudget::new(1,u64::MAX,short),&cancel,frozen_clock,&mut sequence,&mut recorded,&authority).unwrap();
        let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(&mut cx));
        if short.maximum_depth==0{let failure=result.unwrap_err();assert_eq!(failure.primary.kind,semio_framework_value::ValueRefusalKind::DepthLimit);assert_eq!(failure.primary.retained_progress(),Default::default());assert!(failure.receipt_refusal.is_none());}
        else{assert_eq!(result.unwrap(),InteractiveJobCloseStep::Pending{progress:Default::default()});}
        assert_eq!(cx.retained_progress(),Default::default());
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        assert_eq!(job.complete.as_ref().unwrap() as *const FrameDirectives,original);
    }
    let receipt=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<FrameDirectives>(),..Default::default()};
    {
        let mut recorded=Default::default();
        let mut cx=StepContext::with_payload_authority(OperationId(99),Generation(99),semio_framework_job::StepBudget::new(1,u64::MAX,grant),&cancel,frozen_clock,&mut sequence,&mut recorded,&authority).unwrap();
        let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(&mut cx).unwrap());
        assert_eq!(step,InteractiveJobCloseStep::Pending{progress:receipt});
        assert_eq!(cx.retained_progress(),receipt);
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        assert!(job.terminal_is_empty());
        assert_eq!(job.close_step(&mut cx).unwrap(),InteractiveJobCloseStep::Complete{progress:Default::default()});
        assert_eq!(cx.retained_progress(),receipt);
    }
    let (_,drop_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));
    assert_eq!((drop_heap.requested_bytes,drop_heap.released_bytes),(0,0));
    let mut released=0;
    for _ in 0..64{if authority.terminal_is_empty(){break}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||authority.close_step(source).unwrap());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(heap.requested_bytes,heap.released_bytes));released+=heap.released_bytes;}
    assert!(authority.terminal_is_empty());drop(authority);
    let mut cancel=semio_framework_async::CancelTokenRetirement::from_token(cancel);
    for _ in 0..64{if cancel.terminal_is_empty(){break}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cancel.close_step(source).unwrap());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(heap.requested_bytes,heap.released_bytes));released+=heap.released_bytes;}
    assert!(cancel.terminal_is_empty());drop(cancel);
    assert_eq!(released,cancel_heap.requested_bytes+ledger_heap.requested_bytes);
}

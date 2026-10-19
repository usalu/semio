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
use serde_json::Value;
use std::sync::atomic::Ordering;
use ui_wgpu::wgpu::{DrawList, FontAtlas, IconAtlas, InputState, PointerModifiers, SemioCursor, Theme};
use std::sync::Arc;


fn fixture_frame_grant() -> RetainedCloneGrant {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🪙️authority/🖼️frame/🔣️.json")).expect("independent frame root authority");
    let grant = &fixture["grant"];
    RetainedCloneGrant { maximum_items: grant["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: grant["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: grant["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: grant["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: grant["maximumDepth"].as_u64().unwrap() as usize }
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
fn admit_deadline(job:FrameBuildJob,original:&mut crate::fixture_original_turn::FixtureTurnV1)->BatchJobSession<FrameBuildJob>{
    let mut source=original.turn(|cx|semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(job,(cx.operation(),cx.generation())));
    for _ in 0..32{
        if let Some(session)=original.turn(|cx|source.admit_pending_batch(BatchDriveConfig{retained:cx.retained_grant(),site:"original-frame-deadline-law",stage:InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:1000000},cx)).expect("original deadline admission"){
            assert!(source.terminal_is_empty());return session;
        }
    }
    panic!("original deadline admission did not complete")
}

fn retire_fixture_phase(phase:&mut ActiveFramePhase,original:&mut crate::fixture_original_turn::FixtureTurnV1)->InteractiveJobCloseStep{
    original.turn(|cx|{let witness=cx.borrowed_original_cancel_token()?;retire_active_phase(phase,cx,witness)}).expect("same original phase close")
}

fn compute(inputs: FrameBuildInputs) -> FrameDirectives {
    let mut original=crate::fixture_original_turn::FixtureTurnV1::new(fixture_frame_grant());
    let mut session=admit_deadline(FrameBuildJob::new(inputs),&mut original);
    let mut directives=None;
    for _ in 0..32{
        original.turn(|cx|session.step_on_original_caller(cx)).expect("same original deadline step");
        assert!(session.checkout_outcome());
        let(grant,receipt)=session.take_checked_out_retained_step_receipt().expect("exact original deadline receipt");
        assert!(receipt.fits(grant));
        let complete=matches!(session.checked_out_outcome().unwrap(),Some(JobOutcomeView::Complete{..}));
        if complete{
            for _ in 0..4{original.turn(|cx|semio_framework_job::close_original_retained_step(cx,semio_framework_value::RetirementDemand{depth:2,..Default::default()},|grant|Ok(session.acknowledge_outcome(grant)))).unwrap();}
            directives=session.checked_out_job_mut().and_then(FrameBuildJob::take_directives);break;
        }
        session.resume().expect("same original deadline continuation");
    }
    let mut phase=ActiveFramePhase::Deadlines(session);
    for _ in 0..2000{if matches!(retire_fixture_phase(&mut phase,&mut original),InteractiveJobCloseStep::Complete{..}){break}}
    assert!(matches!(phase,ActiveFramePhase::Terminal));
    directives.expect("actual admitted deadline directives")
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
    let mut original=crate::fixture_original_turn::FixtureTurnV1::new(fixture_frame_grant());
    for turn in 0..turns {
        original.turn(|context|{
            let step=run_frame_owner_turn(context,|_|{calls+=1;if turn+1==turns{ActiveFrameStep::Complete(None)}else{ActiveFrameStep::Pending}});
            assert!(matches!((turn+1==turns,step),(false,Some(ActiveFrameStep::Pending))|(true,Some(ActiveFrameStep::Complete(None)))));
            assert!(context.should_yield(),"the same original one-unit loan is exhausted");
        });
    }
    assert_eq!(calls, turns, "each retained Worker turn advances exactly once");
}

#[test]
fn cancellation_retires_deadline_apply_and_build_phases_to_terminal_empty() {
    let mut original=crate::fixture_original_turn::FixtureTurnV1::new(fixture_frame_grant());
    let deadline=admit_deadline(FrameBuildJob::new(FrameBuildInputs{wheel_zoom_deadline_ms:0.0,now_ms:2.0}),&mut original);
    let mut phases=[ActiveFramePhase::Deadlines(deadline),ActiveFramePhase::ApplyPending(FrameDirectives{wheel_zoom_deadline_cleared:false})];
    for phase in &mut phases{
        for _ in 0..2000{if matches!(retire_fixture_phase(phase,&mut original),InteractiveJobCloseStep::Complete{..}){break}}
        assert!(matches!(phase,ActiveFramePhase::Terminal));
    }
}

#[test]
fn cancellation_retires_empty_preparation_to_terminal_empty() {
    let build = semio_framework_os_renderer_wgpu_app_runtime::AppFrameBuild {
        retained: fixture_frame_grant(),
        input: semio_framework_ui_prepared::PreparedRenderInput::try_new(1, 1, semio_framework_ui_draw_contract::DrawList::default(), None, 0.0).unwrap_or_else(|_| panic!("empty fixture preparation admission")),
        input_candidate: None,
        engine_packets: semio_framework_os_renderer_wgpu_app_runtime::FrameEnginePackets::default(),
        generation: Generation(1),
        cursor: ui_wgpu::wgpu::SemioCursor::Default,
        theme_dark: false,
        fullscreen: None,
        cursor_wake: None,
        #[cfg(not(target_arch = "wasm32"))]
        job_progress: None,
    };
    let mut phase = ActiveFramePhase::Prepare(build.into_preparation());
    let mut original=crate::fixture_original_turn::FixtureTurnV1::new(fixture_frame_grant());
    for _ in 0..100 {
        if matches!(retire_fixture_phase(&mut phase,&mut original), InteractiveJobCloseStep::Complete { .. }) {
            break;
        }
    }
    assert!(matches!(retire_fixture_phase(&mut phase,&mut original), InteractiveJobCloseStep::Complete { .. }));
}

#[cfg(not(target_arch = "wasm32"))]
fn runtime_with_presented_input_candidate() -> (semio_framework_os_renderer_wgpu_app_runtime::AppMailbox, semio_framework_os_renderer_wgpu_shell_input::PresentedInputCandidateWitness) {
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::begin_accessibility_visible_documents();
    let mut interaction = semio_framework_os_renderer_wgpu_frame_actions::AppInteractionState {
        shell: semio_framework_os_renderer_wgpu_shell_state::ShellState::new(Vec::new(), "frame-candidate-retirement".to_string(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native),
        input: ui_wgpu::wgpu::InputState::default(),
        theme: semio_framework_ui_theme::Theme::default(),
        theme_dark: false,
        last_pointer_x: 0.0,
        last_pointer_y: 0.0,
        pointer_down: false,
        pointer_button: 0,
        pointer_capture: semio_framework_os_renderer_wgpu_shell_input::PointerCapture::default(),
        modifiers: ui_wgpu::wgpu::PointerModifiers::default(),
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
        native_hot_swap_scan: None,
        native_hot_swap_program: None,
        native_hot_swap_cursor: 0,
        native_reload_pending: false,
    });
    (runtime, witness)
}

#[cfg(not(target_arch = "wasm32"))]
fn close_active_frame(mut active: ActiveFrameBuild,original:&mut crate::fixture_original_turn::FixtureTurnV1) -> semio_framework_os_renderer_wgpu_app_runtime::AppMailbox {
    let runtime = active.runtime.clone();
    active.begin_close();
    for _ in 0..262_144 {
        if matches!(original.turn(|cx|InteractiveJob::close_step(&mut active,cx)).expect("same original active frame close"), semio_framework_job::InteractiveJobCloseStep::Complete { .. }) {
            break;
        }
    }
    assert!(active.terminal_is_empty(), "the abandoned frame owner reaches terminal empty");
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
    let mut original=crate::fixture_original_turn::FixtureTurnV1::with_identity(fixture_frame_grant(),operation,generation);
    let presentation = semio_framework_os_renderer_wgpu_runtime_mailbox::RuntimePresentationWitness { scene_revision: 1, input_generation: generation.0 };
    let mut transaction = semio_framework_os_renderer_wgpu_frame_transaction::FrameTransaction::new(FrameDirectives::default(), operation, generation, fixture_frame_grant());
    let mut cursor = semio_framework_os_renderer_wgpu_app_runtime::FrameBuildCursor::new(presentation, fixture_frame_grant());
    cursor.input_candidate = Some(witness);
    transaction.build_cursor = Some(cursor);
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, original.alias(), fixture_frame_grant());
    active.phase = ActiveFramePhase::Build(transaction);
    let runtime = close_active_frame(active,&mut original);
    assert_candidate_returned(&runtime, witness);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn cancelled_frame_preparation_returns_its_exact_presented_input_candidate() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(92);
    let generation = Generation(92);
    let mut original=crate::fixture_original_turn::FixtureTurnV1::with_identity(fixture_frame_grant(),operation,generation);
    let build = semio_framework_os_renderer_wgpu_app_runtime::AppFrameBuild {
        retained: fixture_frame_grant(),
        input: semio_framework_ui_prepared::PreparedRenderInput::try_new(1, generation.0, semio_framework_ui_draw_contract::DrawList::default(), None, 0.0).unwrap_or_else(|_| panic!("empty prepared input")),
        input_candidate: Some(witness),
        engine_packets: semio_framework_os_renderer_wgpu_app_runtime::FrameEnginePackets::default(),
        generation,
        cursor: ui_wgpu::wgpu::SemioCursor::Default,
        theme_dark: false,
        fullscreen: None,
        cursor_wake: None,
        job_progress: None,
    };
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, original.alias(), fixture_frame_grant());
    active.phase = ActiveFramePhase::Prepare(build.into_preparation());
    let runtime = close_active_frame(active,&mut original);
    assert_candidate_returned(&runtime, witness);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn stale_completed_frame_returns_its_exact_presented_input_candidate() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(93);
    let generation = Generation(93);
    let mut original=crate::fixture_original_turn::FixtureTurnV1::with_identity(fixture_frame_grant(),operation,generation);
    let frame = semio_framework_os_renderer_wgpu_app_runtime::AppFramePresentation {
        retained: fixture_frame_grant(),
        packet: None,
        input_candidate: Some(witness),
        engine_packets: semio_framework_os_renderer_wgpu_app_runtime::FrameEnginePackets::default(),
        generation,
        cursor: ui_wgpu::wgpu::SemioCursor::Default,
        theme_dark: false,
        fullscreen: None,
        cursor_wake: None,
        job_progress: None,
    };
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, original.alias(), fixture_frame_grant());
    active.phase = ActiveFramePhase::Terminal;
    active.completed = Some(frame);
    let runtime = close_active_frame(active,&mut original);
    assert_candidate_returned(&runtime, witness);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn cancelled_after_chrome_frame_returns_its_exact_presented_input_candidate() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(94);
    let generation = Generation(94);
    let mut original=crate::fixture_original_turn::FixtureTurnV1::with_identity(fixture_frame_grant(),operation,generation);
    let mut transaction = semio_framework_os_renderer_wgpu_frame_transaction::FrameTransaction::new(FrameDirectives::default(), operation, generation, fixture_frame_grant());
    transaction.after_chrome =
        Some(semio_framework_os_renderer_wgpu_app_runtime::AppFrameAfterChrome { retained: fixture_frame_grant(), resource_input: None, input_candidate: Some(witness), upload_rejected: None, draw_rejected: None, engine_packets: None, fullscreen: None, cursor_wake: None, job_progress: None, retirement: None });
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, original.alias(), fixture_frame_grant());
    active.phase = ActiveFramePhase::Build(transaction);
    let runtime = close_active_frame(active,&mut original);
    assert_candidate_returned(&runtime, witness);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn component_close_transiently_retires_the_creating_frame_and_readmits_the_same_generation() {
    let (runtime, witness) = runtime_with_presented_input_candidate();
    let operation = OperationId(95);
    let generation = Generation(95);
    let mut original=crate::fixture_original_turn::FixtureTurnV1::with_identity(fixture_frame_grant(),operation,generation);
    let presentation = semio_framework_os_renderer_wgpu_runtime_mailbox::RuntimePresentationWitness { scene_revision: 1, input_generation: generation.0 };
    let mut transaction = semio_framework_os_renderer_wgpu_frame_transaction::FrameTransaction::new(FrameDirectives::default(), operation, generation, fixture_frame_grant());
    let mut cursor = semio_framework_os_renderer_wgpu_app_runtime::FrameBuildCursor::new(presentation, fixture_frame_grant());
    cursor.input_candidate = Some(witness);
    transaction.build_cursor = Some(cursor);
    let mut active = ActiveFrameBuild::new(runtime.clone(), FrameBuildInputs::default(), operation, generation, original.alias(), fixture_frame_grant());
    active.phase = ActiveFramePhase::Build(transaction);

    let wake_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let wake_counter = Arc::clone(&wake_count);
    let mut handle = FrameBuildHandle::new();
    runtime.set_waker(Arc::new(move || {
        wake_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }));
    handle.original_job=Some(active);
    handle.original_presentation_generation=Some(generation);
    let binding=original.turn(|cx|(cx.operation(),cx.generation(),cx.original_clock_binding(),cx.original_recipient_identity()));

    assert!(!original.turn(|cx|handle.retire_for_component_surface_close_step(cx)).expect("same original transient close"), "one bounded close unit cannot discard the whole retained frame");
    for _ in 0..262_144 {
        if original.turn(|cx|handle.retire_for_component_surface_close_step(cx)).expect("same original transient close") {
            break;
        }
    }
    assert!(!handle.has_live_session(), "the exact creating frame reaches terminal before the external owner starts");
    original.turn(|cx|{assert!(!cx.is_cancelled(),"closing a child does not cancel its original supplier");assert_eq!((cx.operation(),cx.generation(),cx.original_clock_binding(),cx.original_recipient_identity()),binding);});
    assert!(!handle.closing, "component handoff does not permanently close the reusable frame handle");
    let before=wake_count.load(std::sync::atomic::Ordering::SeqCst);
    runtime.0.waker.lock().unwrap().as_ref().expect("the actual installed runtime wake survives handoff")();
    assert_eq!(wake_count.load(std::sync::atomic::Ordering::SeqCst),before+1);
    assert_eq!(handle.original_presentation_generation, None, "native may readmit the same generation after terminal handoff");
    assert_candidate_returned(&runtime, witness);

    let _ = original.turn(|cx|handle.poll_runtime_on_original_caller(runtime,FrameBuildInputs::default(),generation,cx)).expect("same original same-generation successor");
    assert!(handle.has_live_session(), "the reusable handle admits a same-generation successor without new host input");
    for _ in 0..262_144 {
        if original.turn(|cx|handle.retire_for_component_surface_close_step(cx)).expect("same original transient close") {
            break;
        }
    }
    assert!(!handle.has_live_session());
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
    let mut caller=crate::fixture_original_turn::FixtureTurnV1::with_identity(fixture_frame_grant(),OperationId(99),Generation(99));
    let mut job=FrameBuildJob::new(inputs(1_000.0));
    caller.turn(|cx|{assert!(job.step(cx).unwrap().is_none());assert_eq!(cx.retained_progress(),RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<FrameDirectives>>(),..Default::default()});});
    job.begin_close();let original=job.complete.as_ref().unwrap()as*const FrameDirectives;
    assert_eq!(job.next_close_copy_byte_demand().unwrap(),size_of::<FrameDirectives>());assert_eq!(job.next_close_capacity_byte_demand(0).unwrap(),0);assert_eq!(job.next_close_release_byte_demand().unwrap(),0);assert_eq!(job.next_close_depth_demand().unwrap(),1);
    for short in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
        let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.limited(short,|cx|job.close_step(cx)));
        if short.maximum_depth==0{let failure=result.unwrap_err();assert_eq!(failure.primary_value().unwrap().kind,semio_framework_value::ValueRefusalKind::DepthLimit);assert_eq!(failure.primary.retained_progress(),Default::default());assert!(failure.receipt_refusal.is_none());}else{assert_eq!(result.unwrap(),InteractiveJobCloseStep::Pending{progress:Default::default()});}
        assert_eq!(caller.receipt(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(job.complete.as_ref().unwrap()as*const _,original);
    }
    let receipt=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<FrameDirectives>(),..Default::default()};
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.limited(grant,|cx|job.close_step(cx)).unwrap());assert_eq!(step,InteractiveJobCloseStep::Pending{progress:receipt});assert_eq!(caller.receipt(),receipt);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(job.terminal_is_empty());
    assert_eq!(caller.limited(grant,|cx|job.close_step(cx)).unwrap(),InteractiveJobCloseStep::Complete{progress:Default::default()});assert_eq!(caller.receipt(),Default::default());
    let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));caller.finish();
    eprintln!("[DEBUG] actual finite original frame directive: per-axis denial and candidate pointer retained, exact scalar copy{}, whole failure/receipt, unchanged zero heap, decoder and original roots paid closed",receipt.copied_bytes);
}

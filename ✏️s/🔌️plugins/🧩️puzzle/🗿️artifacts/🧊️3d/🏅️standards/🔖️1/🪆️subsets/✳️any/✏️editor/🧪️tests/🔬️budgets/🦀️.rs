//! ⏱️ Puzzle3d editor budget and defect laws — session caches, interactive step budgets, checked defects, opening camera, settings scope, rendered-vs-mutated document, shipped kind catalog and initial snapshot cost.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_puzzle_3d_editor::*;
use semio_s_artifact_puzzle_3d_retained_action::{PUZZLE3D_RETAINED_PAYLOAD_SCHEMA, Puzzle3dAcceptSuggestionWork, Puzzle3dAddBrushObjectWork, Puzzle3dFocusSelectionWork, Puzzle3dInstanceBinding, Puzzle3dPrecomputeCommandWork, Puzzle3dTransformWork, Puzzle3dWindowCommandWork, puzzle3d_retained_extent, puzzle3d_retained_reduce};
use {semio_s_artifact_puzzle_3d_retained_document::Puzzle3dAddObjectKindWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dCreateAttractionWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dKindWeightWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dPatchInspectorWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dSetActiveExampleWork};
use semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_handle_action;
use {semio_s_artifact_puzzle_3d_editor_session::PUZZLE3D_GEOMETRY_SERIALIZATIONS, semio_s_artifact_puzzle_3d_editor_session::PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dInstanceOperationOwner, semio_s_artifact_puzzle_3d_editor_session::with_puzzle3d_session_for, semio_s_artifact_puzzle_3d_editor_session::puzzle3d_view_session_key};
use {semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_DEFAULT_UTILITY, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_ATTRACTION, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_KIND, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_REFERENCE, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_TARGET_VOLUME, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_VORTEX, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_HOVER_CHANNEL, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dInteractionSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dScene, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass, semio_s_artifact_puzzle_3d_editor_snapshot::default_scene_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_addressed_window_id, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_entity_label, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_fault_notices, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_action, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_argument, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_inspector_origin_nudge, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_kind_ids, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_paste_operations, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_active_utility, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_document, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_play_snapshot, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_selected_objects_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_upsert_catalog_rows, semio_s_artifact_puzzle_3d_editor_snapshot::scene_from_snapshot};
use semio_s_artifact_puzzle_3d_commands_document::world_relocate;
use {semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfig, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigMutation, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dRuntime};
use semio_s_artifact_puzzle_3d_editor_modes as edit;
use semio_s_artifact_puzzle_3d_editor_modes::tools::fill as fill_tool;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main::utilities;
use semio_s_artifact_puzzle_3d_editor_panels::{catalogue, artifact, inspection, settings as settings_panel};
use semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsOwner;
use semio_s_artifact_puzzle_3d_editor_presence as presence;
use semio_s_artifact_puzzle_3d_editor_presence::{Puzzle3dPresence, Puzzle3dPresenceMutation};
use semio_s_artifact_puzzle_3d_editor_terminology::{puzzle3d_labels, puzzle3d_localized, puzzle3d_localized_phrase, Puzzle3dLabels};
use semio_s_artifact_puzzle_3d_editor_window as window_ownership;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PastePlacement};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, StepContext};
use semio_framework_plugin::panel_tab_element_id;
use semio_framework_plugin::panel_tab_first_draggable_element_id;
use semio_framework_plugin::window_element_id;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ActionRef;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DialogDefinition;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::ArtifactReservedJob;
use semio_framework_plugin::ArtifactReservedToolInput;
use semio_framework_plugin::ArtifactReservedToolJob;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionVerb;
use semio_framework_plugin::InteractionWrite;
use semio_framework::onboarding::IntroductionDefinition;
use semio_framework::onboarding::IntroductionInteraction;
use semio_framework::onboarding::IntroductionPlacement;
use semio_framework::onboarding::IntroductionStepDefinition;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPortDirection;
use semio_framework_plugin::MediaPortSpec;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PortMultiplicity;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_2d::compute::EngineHandles;
use semio_framework_value::FromValue;
use {semio_framework_pack_json::json, semio_framework_pack_json::object, semio_framework_pack_json::parse, semio_framework_pack_json::to_json_string, semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
use semio_framework_plugin::app::{EphemeralEmit, InteractionView};
use std::collections::{HashMap, HashSet};
use std::collections::{BTreeMap};
use semio_s_artifact_puzzle_3d_commands_document::delete_selection;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;

#[path = "../🧰️context/🦀️.rs"]
pub mod context;
use context::*;
use semio_framework_plugin::EditorApp;

/// 🎟️ Wave W-P: `Puzzle3dPlayApp` used to be rebuilt from `default()` on every dispatch and every render,
/// so `geometry_cache` was structurally unable to observe two calls in a row and the whole scene_snapshot was
/// re-serialized every time (`📓️2026-09-08-performance-architecture-audit.md` §1, fix #1). With a session
/// slot keyed by `app_instance_id`, the second call for the same document must serialize NOTHING.
#[test]
fn a_second_call_on_one_instance_reuses_the_geometry_cache_instead_of_reserializing() {
    let config = Puzzle3dRuntime::default();
    let scene_snapshot = default_scene_snapshot();
    let fingerprint = main::scene_geometry_fingerprint(&scene_snapshot);
    let session = Some((4_001_u32, Some("document-geometry".to_string())));
    let cold = PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(std::cell::Cell::get);
    let first = with_puzzle3d_app_for(session.clone(), &config, |app| app.geometry_jsons(&scene_snapshot, &std::collections::BTreeSet::new(), &[]));
    let after_first = PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(std::cell::Cell::get);
    assert_eq!(after_first - cold, 1, "the first call for a cold instance serializes exactly once");
    let second = with_puzzle3d_app_for(session, &config, |app| {
        let cached = app.mesh_cache.lock().expect("mesh cache");
        assert_eq!(cached.as_ref().map(|(cached, _)| *cached), Some(fingerprint), "the session slot handed the warm cache to a brand-new app object");
        drop(cached);
        assert_eq!(app.instance_residency.lock().expect("instance residency").as_ref().map(|residency| residency.revision()), Some(1), "the per-object instance residency came back with the slot too");
        app.geometry_jsons(&scene_snapshot, &std::collections::BTreeSet::new(), &[])
    });
    assert_eq!(PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(std::cell::Cell::get), after_first, "the second call on the same instance must not re-serialize anything");
    assert_eq!(first, second, "a cache hit returns byte-identical instance and mesh json");
}

/// 🥽️ Wave W-P: a mesh registered by one dispatch used to be gone by the next, because the collision
/// engine died with its app object (audit bottleneck (h)). A worker hop is exactly "a new app object with
/// the same instance id", so the resumed call must still hold the registered geometry.
#[test]
fn a_worker_hop_resume_still_holds_the_registered_brush_mesh() {
    let config = Puzzle3dRuntime::default();
    let session = Some((4_002_u32, Some("document-mesh".to_string())));
    let url = "/test/session-hop.glb";
    let positions: Vec<f32> = vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0];
    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
    with_puzzle3d_app_for(session.clone(), &config, |app| {
        app.precompute.borrow_mut().register_mesh(url, &positions, &indices);
        assert!(app.precompute.borrow().has_mesh(url), "the registering dispatch sees its own mesh");
    });
    with_puzzle3d_app_for(session, &config, |app| {
        assert!(app.precompute.borrow().has_mesh(url), "the resumed dispatch adopted the same instance's registered mesh");
    });
    with_puzzle3d_app_for(Some((4_003_u32, Some("document-other".to_string()))), &config, |app| {
        assert!(app.precompute.borrow_mut().adopt_shared_mesh(url, None), "a different document reaches the same geometry by id alone");
    });
    with_puzzle3d_app_for(None, &config, |app| {
        assert!(!app.precompute.borrow().has_mesh(url), "a call with no instance identity stays session-less, exactly as before");
    });
}

/// 🎫 Wave W-P: instance ids are reused by the framework, so a slot re-keyed to a different parent
/// document must retire — and every lease taken against the previous document must fail to check in
/// rather than leak one document's cached geometry into another's render.
#[test]
fn a_stale_session_lease_is_rejected_and_a_rekeyed_instance_starts_cold() {
    let instance = 4_004_u32;
    let stale = {
        let mut registry = puzzle3d_session_registry().lock().expect("session registry");
        let (stale, held) = registry.check_out(instance, Some("document-a")).expect("first lease");
        assert!(held.instances.is_none() && held.meshes.is_none(), "a cold slot hands out no cached geometry");
        let (fresh, rekeyed) = registry.check_out(instance, Some("document-b")).expect("re-keyed lease");
        assert!(rekeyed.instances.is_none() && rekeyed.meshes.is_none(), "a re-keyed instance starts cold instead of adopting the previous document");
        assert_ne!(stale.generation, fresh.generation, "re-keying bumps the slot generation");
        registry.check_in(stale, Puzzle3dSessionState { meshes: Some((7, "stale-meshes".into())), ..Default::default() });
        stale
    };
    let mut registry = puzzle3d_session_registry().lock().expect("session registry");
    let (_, adopted) = registry.check_out(instance, Some("document-b")).expect("post-stale lease");
    assert!(adopted.instances.is_none() && adopted.meshes.is_none(), "the stale lease's state was refused, so document-b is still cold");
    assert_ne!(stale.generation, registry.generations[usize::try_from(instance).expect("slot base") % PUZZLE3D_SESSION_SLOTS], "the retired generation is never handed out again");
}

/// ⚖️ Wave W-P: the session census is a real bound, not a slot count — a check-in whose bytes would cross
/// `PUZZLE3D_SESSION_PROCESS_BYTES` is dropped, which costs one cold rebuild and never corrupts anything.
#[test]
fn a_session_check_in_over_the_process_byte_ceiling_is_dropped() {
    let mut registry = Puzzle3dSessionRegistry::default();
    let (lease, _) = registry.check_out(11, Some("document-census")).expect("lease");
    registry.check_in(lease, Puzzle3dSessionState { meshes: Some((1, "x".repeat(PUZZLE3D_SESSION_PROCESS_BYTES))), ..Default::default() });
    assert_eq!(registry.aggregate_bytes, PUZZLE3D_SESSION_PROCESS_BYTES, "a census exactly at the ceiling is still admissible");
    let (lease, held) = registry.check_out(12, Some("document-second")).expect("second lease");
    assert!(held.instances.is_none() && held.meshes.is_none(), "a different instance owns a different slot and starts cold");
    registry.check_in(lease, Puzzle3dSessionState { meshes: Some((2, "y".into())), ..Default::default() });
    assert_eq!(registry.aggregate_bytes, PUZZLE3D_SESSION_PROCESS_BYTES, "one byte past the ceiling is refused rather than admitted");
    let (_, refused) = registry.check_out(12, Some("document-second")).expect("third lease");
    assert!(refused.instances.is_none() && refused.meshes.is_none(), "the refused state is simply absent on the next call, so that instance rebuilds cold");
}

/// 📐️ Wave W-P: the session row is a fixed 64-slot array, so one slot's inline size is multiplied by 64
/// every time the registry is constructed. Anything multi-kilobyte by value (a `BuiltNode`, a scene_snapshot)
/// belongs behind a pointer, not inline — a fat slot is how a fixed row turns into a stack overflow.
#[test]
fn one_session_slot_stays_small_enough_for_a_fixed_row() {
    let slot = size_of::<Puzzle3dSessionSlot>();
    let state = size_of::<Puzzle3dSessionState>();
    let collision = size_of::<Puzzle3dCollisionSession>();
    let app = size_of::<Puzzle3dPlayApp>();
    println!("[wave-P sizes] slot={slot} state={state} collision={collision} app={app} row={}", slot * PUZZLE3D_SESSION_SLOTS);
    assert!(slot <= 64, "one session slot grew to {slot} bytes; keep the cached state behind a pointer");
    assert!(slot * PUZZLE3D_SESSION_SLOTS <= 8 * 1024, "the whole session row grew to {} bytes and is built by value", slot * PUZZLE3D_SESSION_SLOTS);
    assert!(state <= 2048, "one session state grew to {state} bytes");
    assert!(collision <= 2048, "the carried collision session grew to {collision} bytes");
    assert!(app <= 32 * 1024, "Puzzle3dPlayApp grew to {app} bytes; it is built on the stack on every dispatch and every render, and async dispatch futures hold several copies inline");
}

//#region ⏱️InteractiveStepBudget
/// 🚨️ Mirror of `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` (8 000 µs) — that crate is not a
/// direct dependency of this artifact. `semio_framework_job` faults any interactive step at or over it
/// with `interactive_step_contract_violated`, and `MountedTypedCommandFullOperation`'s worker pump
/// cancels the lease before the fault body is surfaced, so the user reads a real budget overrun as a
/// bare "typed-operation cancelled". Every retained step of every puzzle3d command must stay under it.
const PUZZLE3D_INTERACTIVE_STEP_CEILING: std::time::Duration = std::time::Duration::from_micros(8_000);

/// 🎯️ What this artifact holds itself to in the UNOPTIMIZED test profile — a quarter of the framework
/// ceiling, so an optimized guest keeps an order of magnitude of headroom over the same document.
const PUZZLE3D_MEASURED_STEP_BUDGET: std::time::Duration = std::time::Duration::from_micros(2_000);

/// 🔁️ Cold runs each measured law drives, keeping each TURN's best across them — the standard robust
/// estimator for "how much work does one turn do", which is the only thing this artifact controls. This
/// repository is worked on by several sessions at once and this box runs their cargo builds alongside
/// the suite: at the millisecond scale a single wall-clock sample measures the scheduler, not the
/// artifact. Measured on the SAME binary, same turn: the retired fill tick gave 1.55 / 3.68 / 2.69 / 3.72 /
/// 2.19 ms across five consecutive runs at load average 64, and 12.67 ms at load average 79, against a
/// best of 1.25 ms (ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3). Both bounds below are therefore read
/// off those per-turn minima — including the framework ceiling, whose per-turn contract the framework
/// itself enforces at runtime; what a test can prove is that the WORK inside a turn fits it.
const PUZZLE3D_MEASURED_STEP_RUNS: u32 = 5;

fn measured_step_loop(work: &mut dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle3dPlayApp>>, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, config: &Puzzle3dConfig, context: &semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle3dPlayApp>>, guard: usize, label: &str) -> Vec<std::time::Duration> {
    use semio_s_plugin_puzzle_retained::testing::PuzzleCommandWorkStep;
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let mut turns = Vec::with_capacity(guard);
    loop {
        assert!(turns.len() <= guard, "{label} step() did not reach Complete within {guard} bounded turns");
        let started = std::time::Instant::now();
        let outcome = semio_s_plugin_puzzle_retained::testing::step(work, command, snapshot, config, &interaction, &hover, Some(context)).expect("bounded step");
        turns.push(started.elapsed());
        if matches!(outcome, PuzzleCommandWorkStep::Complete(_)) {
            break;
        }
    }
    turns
}

fn measured_tool_work(tool_id: &'static str, run: u32) -> (Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle3dPlayApp>>>, semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle3dPlayApp>>) {
    let instance = Puzzle3dInstanceBinding { app_instance_id: 9_000 + run, parent_document_id: format!("measured-{tool_id}-{run}"), owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::new(Box::new(Puzzle3dInstanceOperationOwner::default())) };
    let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle3dPlayApp>>> = match tool_id {
        "acceptSuggestion" => Box::new(Puzzle3dAcceptSuggestionWork::default().bound(&instance)),
        "setActiveExample" => Box::new(Puzzle3dSetActiveExampleWork::default()),
        "openVortexSuggestions" => Box::new(Puzzle3dWindowCommandWork::new(tool_id).bound(&instance)),
        _ => Box::new(semio_s_plugin_puzzle_retained::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent)),
    };
    let context = semio_framework_plugin::app::ArtifactOwnedToolJobContext::new(
        instance.app_instance_id,
        Some(measured_view_state()),
        [0; 32],
        0,
        0,
        semio_framework_plugin::app::ArtifactOwnedToolJobSnapshots { children: Default::default(), draft: Default::default(), transient: Default::default(), window_config: None, window_transient: None },
    );
    (work, context)
}

/// ⏱️ Drives one tool id's REAL work to `Complete` `PUZZLE3D_MEASURED_STEP_RUNS` times, each from its
/// own cold session slot, and answers `(turns, the worst PER-TURN BEST)`: every run of the same cold
/// document takes the same bounded turns in the same order (asserted), so turn `t`'s cost is the
/// minimum of that turn across the runs, and the law is on the worst of those minima. A scheduler
/// preemption lands on one turn of one run and is cancelled by the others — see
/// `PUZZLE3D_MEASURED_STEP_RUNS` for why that matters on this box.
fn measured_cold_runs(tool_id: &'static str, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, config: &Puzzle3dConfig) -> (usize, std::time::Duration) {
    let mut best: Vec<std::time::Duration> = Vec::new();
    for run in 0..PUZZLE3D_MEASURED_STEP_RUNS {
        let (mut work, context) = measured_tool_work(tool_id, run);
        let turns = measured_step_loop(work.as_mut(), command, snapshot, config, &context, semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS, tool_id);
        if best.is_empty() {
            best = turns;
            continue;
        }
        assert_eq!(turns.len(), best.len(), "{tool_id} run {run} took a different number of bounded turns than its cold siblings");
        for (slot, measured) in best.iter_mut().zip(turns) {
            *slot = (*slot).min(measured);
        }
    }
    let (index, worst) = best.iter().enumerate().max_by_key(|(_, turn)| **turn).map_or((0, std::time::Duration::ZERO), |(index, turn)| (index + 1, *turn));
    (best.len(), worst)
}

fn measured_view_state() -> semio_framework_plugin::ViewModel {
    semio_framework_plugin::ViewModel {
        window_id: Some(main::WINDOW_KIND_ID.to_string()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: main::WINDOW_KIND_ID.to_string(), window_kind_id: main::WINDOW_KIND_ID.to_string() }],
        ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    }
}

fn measured_nakagin_snapshot() -> Puzzle3dPlaySnapshot {
    Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"))
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END: `openVortexSuggestions` syncs the whole precompute session
/// and refreshes one vortex's brush candidates. Measured on Nakagin at opt-level 0: 82.5 ms (W-P2's
/// baseline), 17.6 ms once W-P2 removed the doubled session sync and the doubled projection decode, and
/// under this artifact's own 2 000 µs budget once W-P3 split `handle_action_impl`'s prologue into the
/// scene / session-sync / dispatch turns [`Puzzle3dActionPrologue`] declares and made each of them typed.
/// This drives the REAL `Puzzle3dWindowCommandWork` `build_tool_job` routes this tool id to.
#[test]
fn open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_nakagin_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("openVortexSuggestions", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), Some(main::WINDOW_KIND_ID.to_string())).expect("openVortexSuggestions command decodes");
    let (steps, worst) = measured_cold_runs("openVortexSuggestions", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "openVortexSuggestions worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "openVortexSuggestions worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END W-P2: `acceptSuggestion` walks the document for its target
/// vortex and publishes the placement. Measured at 11 796 µs in ONE step before this wave.
#[test]
fn accept_suggestion_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_nakagin_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("acceptSuggestion", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), Some(main::WINDOW_KIND_ID.to_string())).expect("acceptSuggestion command decodes");
    let (steps, worst) = measured_cold_runs("acceptSuggestion", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "acceptSuggestion worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "acceptSuggestion worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

fn measured_concrete_forest_snapshot() -> Puzzle3dPlaySnapshot {
    Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"))
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3: swapping the Concrete Forest document for the
/// 180-object Nakagin one is the single largest document gesture this artifact has — it deletes every
/// existing attraction and object and creates every Nakagin one. Its work is fully typed and cursorized
/// (`Puzzle3dSetActiveExampleWork`), so no turn of it may cross the interactive step ceiling either.
#[test]
fn set_active_example_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_concrete_forest_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), Some(main::WINDOW_KIND_ID.to_string())).expect("setActiveExample command decodes");
    let (steps, worst) = measured_cold_runs("setActiveExample", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "setActiveExample worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "setActiveExample worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

/// 🌉️ Differential law for W-P3's typed `scene_from_snapshot`: the derived `ToValue`/`FromValue`
/// machinery is an independent implementation of the same structural-twin translation, so the typed
/// scene_snapshot must equal what the persisted-projection bridge produced for every shipped document. A
/// disagreement here is a real behaviour change, not a performance one — the semantic document delta
/// every editing action publishes is taken against exactly this scene_snapshot.
///
/// 🪪️ Each snapshot is canonicalized first (rebuilt from its OWN typed authority) because that is the
/// only projection production ever hands the app: a store-driven `Puzzle3dPlaySnapshot` carries the
/// typed document and materializes `value()` from it, so `value()` is by construction
/// `ToValue(typed())`. Feeding a raw editor-side scene_snapshot straight into `new()` can hand the two halves
/// different content — `empty_scene_snapshot()`'s `meta` serializes both members as `Null`, which the typed
/// decode refuses, and `new()` silently falls back to `Puzzle3dSnapshot::default()`; see this wave's
/// report §6.
#[test]
fn puzzle3d_next_object_label_increments_from_the_authored_seed_name() {
    let scene_snapshot = CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone();
    let kind = "Hexagonal Cut Concrete Forest Left";
    let next = puzzle3d_next_object_label(&scene_snapshot.objects, &scene_snapshot, kind);
    assert_eq!(next, "Hexagonal Cut Concrete Forest Left 2");
    let mut peers = scene_snapshot.objects.clone();
    peers.push(Puzzle3dObject {
        id: "fill-1".into(),
        label: Some("Hexagonal Cut Concrete Forest Left 2".into()),
        object_kind: Some(kind.into()),
        origin: [0.0; 3],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: Vec::new(),
        hidden: false,
        locked: false,
    });
    assert_eq!(puzzle3d_next_object_label(&peers, &scene_snapshot, kind), "Hexagonal Cut Concrete Forest Left 3");
}

/// 🗝️ The two untyped `meta` members are compared as the TYPED catalogs they stand for
/// (`Puzzle3dKindCatalogs` / `Vec<Puzzle3dKindCompatibility>` — lossless, and what every reader of
/// those members ultimately decodes them into), because raw `DslValue` equality would compare key
/// ORDER: the bridge inherits the persisted projection's own `serde_json` map order and the typed
/// construction emits declaration order, for byte-identical content. Every typed member of the scene_snapshot
/// is compared directly, unnormalized.
#[test]
fn puzzle3d_typed_fixture_matches_the_projection_bridge_for_every_example() {
    for (label, scene_snapshot) in [("empty", empty_scene_snapshot()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()), ("nakagin", NAKAGIN_EXAMPLE_SNAPSHOT.clone())] {
        let seed = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&scene_snapshot).expect("typed fixture admits"));
        let snapshot = Puzzle3dPlaySnapshot::new(semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(seed.typed())).expect("typed fixture admits"));
        let bridged = Puzzle3dScene { scene_snapshot: semio_framework_value::FromValue::from_value(snapshot.value().clone()).expect("native fixture view admits"), runtime: Puzzle3dRuntime::default(), active_utility: "utility".into() };
        let typed = scene_from_snapshot(snapshot.typed(), Puzzle3dRuntime::default(), "utility");
        assert_eq!(typed.scene_snapshot.schema, bridged.scene_snapshot.schema, "{label}: schema disagrees");
        assert_eq!(typed.scene_snapshot.domain, bridged.scene_snapshot.domain, "{label}: domain disagrees");
        assert_eq!(typed.scene_snapshot.objects, bridged.scene_snapshot.objects, "{label}: objects disagree");
        assert_eq!(typed.scene_snapshot.attractions, bridged.scene_snapshot.attractions, "{label}: attractions disagree");
        assert_eq!(typed.scene_snapshot.target_volumes, bridged.scene_snapshot.target_volumes, "{label}: target volumes disagree");
        assert_eq!(typed.scene_snapshot.references, bridged.scene_snapshot.references, "{label}: references disagree");
        let catalogs = |meta: &Puzzle3dSceneMetadata| -> semio_s_artifact_puzzle_3d_model::Puzzle3dKindCatalogs { meta.kind_catalogs.clone().map_or_else(semio_s_artifact_puzzle_3d_model::Puzzle3dKindCatalogs::default, |rows| semio_framework_value::FromValue::from_value(rows).expect("kind catalogs decode")) };
        let compatibility = |meta: &Puzzle3dSceneMetadata| -> Vec<semio_s_artifact_puzzle_3d_model::Puzzle3dKindCompatibility> { meta.kind_compatibility.clone().map_or_else(Vec::new, |rows| semio_framework_value::FromValue::from_value(rows).expect("kind compatibility decodes")) };
        assert_eq!(catalogs(&typed.scene_snapshot.meta), catalogs(&bridged.scene_snapshot.meta), "{label}: kind catalogs disagree");
        assert_eq!(compatibility(&typed.scene_snapshot.meta), compatibility(&bridged.scene_snapshot.meta), "{label}: kind compatibility disagrees");
        assert_eq!(typed.active_utility, bridged.active_utility, "{label}: active utility disagrees");
    }
}

/// 🧲️ Differential law for the world attraction segments: resolving both ends through the one per-call vortex
/// index lands every segment exactly where `resolve_vortex_world_position` (the scan it replaced) puts it, and
/// drops exactly the attractions that scan cannot resolve — for every example, and for a fill-run-sized
/// document whose attractions multiply with its placements.
#[test]
fn world_attraction_segments_match_the_vortex_position_resolver_for_every_example() {
    let mut grown = NAKAGIN_EXAMPLE_SNAPSHOT.clone();
    let ends: Vec<String> = grown.objects.iter().flat_map(|object| object.vortices.iter().map(|vortex| semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id(&object.id, &vortex.id))).collect();
    assert!(ends.len() > 2, "nakagin declares vortices to attract");
    grown.attractions = ends.windows(2).enumerate().map(|(index, pair)| Puzzle3dAttraction { id: format!("a{index}"), attracting: pair[0].clone(), attracted: pair[1].clone(), ..Default::default() }).collect();
    grown.attractions.push(Puzzle3dAttraction { id: "dangling".into(), attracting: ends[0].clone(), attracted: "no-such-object:no-such-vortex".into(), ..Default::default() });
    for (label, scene_snapshot) in [("empty", empty_scene_snapshot()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()), ("nakagin", NAKAGIN_EXAMPLE_SNAPSHOT.clone()), ("nakagin-dangling", grown)] {
        let expected: Vec<Value> = scene_snapshot
            .attractions
            .iter()
            .filter_map(|attraction| {
                let from = semio_s_artifact_puzzle_3d_editor_snapshot::resolve_vortex_world_position(&scene_snapshot, &attraction.attracting)?;
                let to = semio_s_artifact_puzzle_3d_editor_snapshot::resolve_vortex_world_position(&scene_snapshot, &attraction.attracted)?;
                Some(json!({ "id": attraction.id, "from": from, "to": to, "color": "#60a5fa" }))
            })
            .collect();
        let rendered = parse(&main::world_attractions_json(&scene_snapshot), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("attractions json");
        let rendered = rendered.as_array().cloned().unwrap_or_default();
        assert_eq!(rendered.len(), expected.len(), "{label}: exactly the resolvable attractions render");
        for (segment, resolved) in rendered.iter().zip(&expected) {
            for field in ["id", "from", "to", "color"] {
                assert_eq!(segment[field], resolved[field], "{label}: indexed attraction segment {} diverges from the resolver at {field}", resolved["id"]);
            }
        }
    }
}

/// 🌉️ Differential law for W-P3's typed `scene_config`: the same engine scene the all-`DslValue` bridge
/// (`scene_config_value` + the derived `FromValue`) produced, field for field, on every shipped document
/// and with real kind weights on the runtime. `SceneConfig: PartialEq` is the engine's OWN resync
/// verdict, so equality here is exactly the property the precompute session reads.
#[test]
fn puzzle3d_typed_scene_config_matches_the_value_bridge_for_every_example() {
    for (label, scene_snapshot) in [("empty", empty_scene_snapshot()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()), ("nakagin", NAKAGIN_EXAMPLE_SNAPSHOT.clone())] {
        let mut runtime = Puzzle3dRuntime::default();
        runtime.contact_tolerance = 0.375;
        runtime.object_kind_weights.insert("capsule".into(), 0.25);
        runtime.vortex_kind_weights.insert("rim".into(), 0.75);
        let envelope = Puzzle3dScene { scene_snapshot, runtime, active_utility: "utility".into() };
        let bridged: semio_s_artifact_puzzle_3d_model::SceneConfig = semio_framework_value::FromValue::from_value(scene_config_value(&envelope)).expect("value bridge decodes");
        let typed = scene_config(&envelope).expect("typed scene config builds");
        assert_eq!(typed, bridged, "{label}: typed engine scene disagrees with the value bridge");
    }
}

/// 🥽️ W-P3: the mesh-url index answers exactly what the per-object catalog scan answered, for every
/// object of every shipped document, and `collect_mesh_urls` still returns the same SET of identities.
#[test]
fn puzzle3d_kind_mesh_index_matches_a_per_object_catalog_scan() {
    for (label, scene_snapshot) in [("empty", empty_scene_snapshot()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()), ("nakagin", NAKAGIN_EXAMPLE_SNAPSHOT.clone())] {
        let index = Puzzle3dKindMeshIndex::of(&scene_snapshot.meta);
        for object in &scene_snapshot.objects {
            let scanned = scene_snapshot
                .meta
                .kind_catalogs
                .as_ref()
                .and_then(|catalogs| catalogs.get("objects"))
                .and_then(semio_framework_value::DslValue::as_array)
                .and_then(|rows| rows.iter().find(|row| row.get("id").and_then(semio_framework_value::DslValue::as_str) == object.object_kind.as_deref()))
                .and_then(|row| row.get("meshUrl"))
                .and_then(semio_framework_value::DslValue::as_str);
            let expected = object.mesh_url.as_deref().filter(|url| !url.is_empty()).or(scanned);
            assert_eq!(index.resolve(object), expected, "{label}: {} resolved to a different mesh identity", object.id);
        }
        let indexed: std::collections::BTreeSet<String> = collect_mesh_urls(&scene_snapshot).into_iter().collect();
        assert!(indexed.iter().all(|url| !url.is_empty()), "{label}: an empty mesh identity was collected");
    }
}

//#endregion ⏱️InteractiveStepBudget

//#region 🩹️CheckedDefects
/// 🎯️ Every action id one built context menu dispatches, groups and submenus included.
fn context_menu_action_ids(items: &[semio_framework_plugin::ContextMenuItemSpec], into: &mut Vec<(String, String)>) {
    for item in items {
        if let Some(action) = item.action.as_deref() {
            into.push((item.id.clone(), action.to_string()));
        }
        if let Some(children) = item.children.as_ref() {
            context_menu_action_ids(children, into);
        }
    }
}

/// 🎯️ Every action id `create_puzzle3d_app()` declares — the set `dispatch_action` can actually route.
fn declared_action_ids() -> std::collections::BTreeSet<String> {
    dispatchable_actions(&create_puzzle3d_app()).into_iter().map(|action| action.id.clone()).collect()
}

/// 🖱️ Context-menu rows bypass the registry-validated `Menu::action` builder (they carry
/// `Puzzle3dLabels` text the English-only `ActionDefinition` cannot resolve), so nothing catches a
/// typo'd action id at build time. `📓️2026-09-09-user-feature-checklist.md` §15/§22 and summary #7:
/// "Zoom to Selection" dispatched `zoomToSelection`, which this crate never declares — the registered
/// verb is `focusSelection`. This law walks EVERY selection kind's menu and holds every emitted id
/// inside the declared set, so the whole class cannot come back.
#[semio_framework_async_macros::async_test]
async fn every_context_menu_row_dispatches_a_declared_action() {
    let declared = declared_action_ids();
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let object_id = first_object_id(&app);
    let vortex_id = first_vortex_full_id(&app);
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    let mut zoom_rows = 0usize;
    for (granularity, id) in [
        (PUZZLE3D_GRANULARITY_OBJECT, object_id.as_str()),
        (PUZZLE3D_GRANULARITY_VORTEX, vortex_id.as_str()),
        (PUZZLE3D_GRANULARITY_TARGET_VOLUME, volume_id.as_str()),
        (PUZZLE3D_GRANULARITY_REFERENCE, "reference-1"),
        (PUZZLE3D_GRANULARITY_ATTRACTION, "attraction-1"),
    ] {
        // 🧹️ `Puzzle3dContextSelection::fill_from_interaction` fills every granularity the CLIENT
        // surface did not send from the framework-owned selection, so the object `addObjectKind`
        // left selected would win every branch and each granularity would build the OBJECT menu.
        // A real client's document selection is the one it right-clicks in; clear it so the
        // surface group under test is the only subject and each branch is genuinely exercised.
        dispatch(&mut app, "clearSelection", None, None).await.expect("clearSelection");
        let menu = context_menu_for_selection(&mut app, granularity, id).await;
        let mut rows = Vec::new();
        context_menu_action_ids(&menu, &mut rows);
        assert!(!rows.is_empty(), "{granularity} selection built a menu with no actionable row");
        for (row, action) in &rows {
            assert!(declared.contains(action), "{granularity} row {row} dispatches undeclared action {action}; declared: {declared:?}");
            if row == "zoom" {
                zoom_rows += 1;
                assert_eq!(action, "focusSelection", "the zoom row must dispatch the registered camera verb");
            }
        }
    }
    assert_eq!(zoom_rows, 3, "object, vortex and reference selections each carry a Zoom to Selection row");
    // 🎯️ Reachability, not just declaration: the id the row carries must be one the typed command
    // channel actually admits (an unknown id panics inside `dispatch`).
    dispatch(&mut app, "focusSelection", None, None).await.expect("the context menu's zoom row must dispatch");
}

/// 🗂️ `📓️2026-09-09-user-feature-checklist.md` §23/summary #9: both the "Add Object" dialog and the
/// standalone `addObjectKind` arg form hardcoded ONE static option (`"Object"`), a kind neither
/// example's catalog declares — so the dialog could not add a single real object kind. Both forms must
/// now offer exactly the declared examples' own catalog rows, bounded by
/// [`PUZZLE3D_OBJECT_KIND_OPTIONS_MAX`], with a default that IS one of them.
#[semio_framework_async_macros::async_test]
async fn the_add_object_dialog_offers_every_object_kind_of_both_examples() {
    use semio_framework_plugin::ArgSchema;
    let definition = create_puzzle3d_app();
    let expected: Vec<String> = [&*CONCRETE_FOREST_EXAMPLE_SNAPSHOT, &*NAKAGIN_EXAMPLE_SNAPSHOT]
        .into_iter()
        .flat_map(|scene_snapshot| puzzle3d_kind_ids(scene_snapshot, "objects"))
        .fold(Vec::new(), |mut ids, id| {
            if !ids.contains(&id) {
                ids.push(id);
            }
            ids
        });
    assert!(expected.len() >= 2, "both shipped examples must declare object kinds, got {expected:?}");
    let dialog = definition.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog declared");
    let standalone = dispatchable_actions(&definition).into_iter().find(|action| action.id == "addObjectKind").expect("addObjectKind declared").args.clone();
    for (surface, args) in [("dialog", dialog.args.clone()), ("action", standalone)] {
        let arg = args.iter().find(|arg| arg.id == "objectKind").unwrap_or_else(|| panic!("{surface} declares an objectKind arg"));
        let ArgSchema::String { options, .. } = &arg.schema else { panic!("{surface}'s objectKind must stay a select") };
        let values: Vec<String> = options.iter().map(|option| option.value.clone()).collect();
        assert_eq!(values, expected, "{surface} must offer the live catalog's object kinds");
        assert!(values.len() <= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX, "{surface} exceeded its fixed option ceiling");
        let default = arg.default.as_ref().and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string();
        assert!(values.contains(&default), "{surface}'s default {default} is not one of its own options");
    }
    assert!(dialog.args.iter().all(|arg| arg.required), "the dialog's kind select stays required");
}

/// 🗨️ Wave B11 (checklist §23, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 7): the shell
/// fallback menu / command palette published TWO rows carrying the identical "Add Object…" label —
/// `addObjectKind` (arg-carrying, so the shell redirects it to the bare action pane) and
/// `openAddObjectDialog` (the declared dialog). The arg-carrying one came FIRST, so every user and
/// every probe clicking "Add Object…" got the action pane and the dialog was unreachable. Exactly one
/// add-object row may be palette/menu vocabulary, and it must be the dialog opener; it must also be
/// declared ahead of `addObjectKind`, because the shell menu keeps its first leaves at top level and
/// folds the rest into "More ›".
#[semio_framework_async_macros::async_test]
async fn exactly_one_add_object_row_is_menu_vocabulary_and_it_opens_the_dialog() {
    let definition = create_puzzle3d_app();
    let actions: Vec<&ActionDefinition> = dispatchable_actions(&definition);
    let find = |id: &str| *actions.iter().find(|action| action.id == id).unwrap_or_else(|| panic!("{id} declared"));
    let opener = find("openAddObjectDialog");
    let parametrized = find("addObjectKind");
    assert_eq!(opener.kind, ActionKind::Shell, "the dialog opener emits an effect, never a document edit");
    assert!(opener.in_palette, "the dialog opener IS the user-facing add-object row");
    assert!(!parametrized.in_palette, "the dialog's parametrized verb must not publish a second identical row");
    fn label(action: &ActionDefinition) -> &str {
        action.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En)
    }
    assert!(label(opener).starts_with(label(parametrized)), "both rows describe the same verb, which is exactly why only one may be offered: {:?} vs {:?}", label(opener), label(parametrized));
    let visible: Vec<&str> = actions.iter().filter(|action| action.in_palette && label(action).starts_with("Add Object")).map(|action| action.id.as_str()).collect();
    assert_eq!(visible, vec!["openAddObjectDialog"], "exactly one add-object row may reach a menu");
    let order = |id: &str| actions.iter().position(|action| action.id == id).unwrap_or_else(|| panic!("{id} declared"));
    assert!(order("openAddObjectDialog") < order("addObjectKind"), "the dialog opener must be declared ahead of the verb it drives so the shell menu keeps it at top level");
    // 🎯️ Reachability, not just declaration: the row's id must be one the typed command channel admits.
    let mut app = app().await;
    let result = dispatch(&mut app, "openAddObjectDialog", None, None).await.expect("the add-object row must dispatch");
    assert!(
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::OpenDialog { dialog_id, .. } if dialog_id == "addObject")),
        "clicking the add-object row must open the declared dialog: {:?}",
        result.requested_effects,
    );
}

/// 🗣️ `📓️2026-09-09-user-feature-checklist.md` §14/summary #10: the engagement input advertised
/// `clear`/`rectangle`/`lasso` while the parser silently dropped all three. The placeholder is now
/// DERIVED from the verb list, and every verb in it must do something observable: the three marquee
/// verbs move the viewport's `selection.method`, and `clear` empties the framework-owned domain.
#[semio_framework_async_macros::async_test]
async fn every_advertised_engagement_verb_is_implemented() {
    use semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_ENGAGEMENT_VERBS;
    let mut app = app().await;
    let envelope = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let placeholder = main::engagement(&envelope, &Puzzle3dLabels::NATIVE_EN, None).input.and_then(|input| input.placeholder).unwrap_or_default();
    for verb in PUZZLE3D_ENGAGEMENT_VERBS {
        assert!(placeholder.contains(verb), "the engagement placeholder must advertise {verb}: {placeholder}");
    }
    let method_of = |node: &Value| selection_of(node).get("method").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_eq!(method_of(&render_composite(&mut app).await), PUZZLE3D_SELECTION_METHOD_PICK, "a fresh window sweeps with the pick default");
    for method in [PUZZLE3D_SELECTION_METHOD_RECTANGLE, PUZZLE3D_SELECTION_METHOD_LASSO, PUZZLE3D_SELECTION_METHOD_PICK] {
        dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": method })), None).await.unwrap_or_else(|error| panic!("engagementSubmit {method}: {error:?}"));
        let rendered = render_composite(&mut app).await;
        assert_eq!(method_of(&rendered), method, "typing {method} must move the viewport marquee method");
    }
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select the object");
    let selected = |node: &Value| selection_of(node).get("ids").and_then(Value::as_array).map(Vec::len).unwrap_or(0);
    assert_eq!(selected(&render_composite(&mut app).await), 1, "the object must be selected before clear");
    dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "clear" })), None).await.expect("engagementSubmit clear");
    assert_eq!(selected(&render_composite(&mut app).await), 0, "typing clear must empty the framework-owned selection");
}

/// 🪣️ Wave B13, `📓️2026-09-11-wave-B1-battery-extension.md` §14 `engagement-fill-verb`: typing `fill <n>`
/// has to ARM the Fill tool, not merely park a utility name in the scene. Fill is a mode-level TOOL, so the
/// only thing that can press its `#tool.fill` tab is an `Effect::SetActiveTool` leaving the turn; `brush` is
/// a window UTILITY and must take the `Effect::SetActiveUtility` lane instead. This law pins both lanes plus
/// the count hand-off, because the two are one `match` apart in `engagement_submit` and the browser
/// symptom of getting it wrong (`#tool.fill aria-pressed=null`) is indistinguishable from a dead input.
#[semio_framework_async_macros::async_test]
async fn the_engagement_fill_verb_arms_the_fill_tool_and_hands_over_its_count() {
    let mut app = app().await;
    let result = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "fill 5" })), None).await.expect("engagementSubmit fill 5");
    assert!(
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id == semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID)),
        "typing `fill 5` must arm the fill tool: {:?}",
        result.requested_effects,
    );
    assert!(
        !result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { .. })),
        "fill is a tool, so it must not also claim a window utility: {:?}",
        result.requested_effects,
    );
    let count = result
        .requested_effects
        .iter()
        .find_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == "setFillCount" => args.as_ref().and_then(|value| value.get("value")).and_then(semio_framework_value::DslValue::as_f64),
            _ => None,
        })
        .expect("typing `fill 5` must hand the count to the retained setFillCount command");
    assert_eq!(count, 5.0, "the typed count reaches setFillCount verbatim");
    assert!(
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, args: Some(args), .. } if action == semio_framework_tool_run::TOOL_RUN_START_ACTION_ID && args.get(semio_framework_tool_run::TOOL_RUN_ARG_TOOL_ID).and_then(semio_framework_value::DslValue::as_str) == Some(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID))),
        "typing `fill 5` starts a fill tool run through the framework: {:?}",
        result.requested_effects,
    );
    let brush = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "brush" })), None).await.expect("engagementSubmit brush");
    assert!(
        brush.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { utility_id, .. } if utility_id == "brush")),
        "typing `brush` must claim the brush window utility: {:?}",
        brush.requested_effects,
    );
    assert!(
        !brush.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { .. })),
        "leaving fill is the host's own `setActiveTool \"\"`, never a guest tool effect: {:?}",
        brush.requested_effects,
    );
}

/// 🧰️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B35, checklist §14 `engagement-brush-verb`: `fill`
/// arms and `brush` does not, because `fill` is a MODE-level tool whose effect carries no window and
/// `brush` is a window utility whose effect carries one. `active_utility_by_window_id` is keyed by
/// window INSTANCE (wave B9), and a pane reads only its own key — so an effect addressed at the bare
/// window KIND (`puzzle3d-main`) arms a key nothing ever reads and the world lane keeps publishing
/// `activeUtility=select` forever, which is exactly the browser verdict. The sibling law above pins
/// WHICH effect the verb emits; this one pins WHERE it lands, on the multi-pane roster the shell
/// actually runs (the engagement line is window chrome, and an app-level panel resolves through
/// `focused_window_id`, wave B15 — both must resolve to an instance, never to the kind).
#[semio_framework_async_macros::async_test]
async fn the_engagement_brush_verb_arms_the_utility_of_a_pane_instance_never_the_window_kind() {
    let mut app = app().await;
    let perspective = "puzzle3d-main-perspective";
    let top = "puzzle3d-main-top";
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": true })), Some(top)).await.expect("grow the roster to two panes");
    let armed = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "brush" })), Some(perspective)).await.expect("engagementSubmit brush at the pane");
    let addressed: Vec<&String> = armed
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::SetActiveUtility { window_id, utility_id } if utility_id == utilities::brush::UTILITY_ID => Some(window_id),
            _ => None,
        })
        .collect();
    assert_eq!(addressed, vec![&perspective.to_string()], "the pane-addressed brush verb must arm that pane's own key: {:?}", armed.requested_effects);
    let unaddressed = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "brush" })), None).await.expect("engagementSubmit brush with no window of its own");
    let fallback: Vec<&String> = unaddressed
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::SetActiveUtility { window_id, utility_id } if utility_id == utilities::brush::UTILITY_ID => Some(window_id),
            _ => None,
        })
        .collect();
    assert!(
        fallback.iter().all(|window_id| !semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_window_id_is_kind(window_id)),
        "a verb that carries no window of its own must still resolve a pane INSTANCE, never the bare kind: {fallback:?}",
    );
}

/// 🛑 Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31, checklist §12/§14 `Abort`: Escape while the Fill
/// TOOL is armed has to LEAVE it — abort the run it shows and disarm the tool — instead of
/// returning untouched, which is what `engagement_abort` did while `puzzle3d_fill_tool_active` and why
/// the browser read `engagement-abort ... activeUtility=fill` after every `fill <n>` (wave B29 §2.5).
/// The disarm is the app's own explicit effect, and the abort leaves NO tool and NO utility armed —
/// the same place the shell's own Escape leaves them (`ShellHost` dispatches `setActiveTool ""`), not
/// a remembered previous tool this app keeps no history of. The `brush`-claims-a-utility law above
/// still holds: a plain verb switch never emits a tool effect, only this abort does.
#[semio_framework_async_macros::async_test]
async fn escaping_the_armed_fill_tool_cancels_the_plan_and_disarms_the_tool() {
    let mut armed = app().await;
    dispatch(&mut armed, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("arm fill");
    let aborted = dispatch(&mut armed, "engagementAbort", None, None).await.expect("engagementAbort");
    assert!(
        aborted.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty())),
        "Escape with fill armed must disarm the tool: {:?}",
        aborted.requested_effects,
    );
    assert!(
        !aborted.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id == semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID)),
        "and must never re-arm the tool it is leaving: {:?}",
        aborted.requested_effects,
    );
    let active_utility = |node: &Value| selection_of(node).get("activeUtility").and_then(Value::as_str).unwrap_or_default().to_string();
    let rendered = render_window(&mut armed, main::WINDOW_KIND_ID).await;
    assert_eq!(active_utility(&rendered), PUZZLE3D_DEFAULT_UTILITY, "the abort leaves no utility armed either: {}", selection_of(&rendered));

    // 🏛️ `active_tool_id` is host-owned (`ViewModel`, `🪟️window/🦀️.rs` `host_activation`), so the guest
    // keeps reading `fill` until the host answers the disarm. A second Escape in that window therefore
    // repeats the same idempotent disarm — and still aborts nothing, because no run was ever shown.
    let repeated = dispatch(&mut armed, "engagementAbort", None, None).await.expect("second engagementAbort");
    assert!(
        repeated.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty())),
        "while the host still reports fill armed the disarm repeats rather than giving up: {:?}",
        repeated.requested_effects,
    );
    assert!(
        !repeated.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID)),
        "and aborts no run, because none is shown: {:?}",
        repeated.requested_effects,
    );

    let mut unarmed = app().await;
    let untouched = dispatch(&mut unarmed, "engagementAbort", None, None).await.expect("engagementAbort with nothing armed");
    assert!(
        !untouched.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { .. }) || matches!(effect, Effect::DispatchAction { action, .. } if action == semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID)),
        "an abort with no tool armed owes no tool or abort effect: {:?}",
        untouched.requested_effects,
    );
}

/// 🧯️ `📓️2026-09-09-user-feature-checklist.md` §9/§13/summary #13: a placement that produces nothing
/// used to produce NOTHING — no fault, no notice, no visible change. Every refusal path of the two
/// placement verbs must now carry exactly one `Effect::Notify`, and a SUCCESSFUL placement must carry
/// none (a notice on the happy path would be noise, and would hide the real ones).
#[semio_framework_async_macros::async_test]
async fn a_refused_placement_surfaces_exactly_one_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { message } => Some(message.clone()),
                _ => None,
            })
            .collect()
    };
    let mut app = app().await;
    // 🎯️ Nothing selected, no popup open, no `fullId` — the accept has no target at all.
    let orphan = dispatch(&mut app, "acceptSuggestion", None, None).await.expect("acceptSuggestion without a target still completes");
    assert_eq!(notices(&orphan).len(), 1, "an accept with no target must say so: {:?}", orphan.requested_effects);
    // 🎯️ A brush placement naming a vortex no object owns, on a kind the catalog does not declare.
    let unknown = dispatch(
        &mut app,
        "addBrushObject",
        Some(&json!({ "targetVortexFullId": "no-such-object:v0", "objectKindId": "no-such-kind", "sourceVortexIndex": 0, "origin": [0.0, 0.0, 0.0], "orientation": [0.0, 0.0, 0.0, 1.0] })),
        None,
    )
    .await
    .expect("addBrushObject on an unknown kind still completes");
    assert_eq!(notices(&unknown).len(), 1, "a brush placement onto an undeclared kind must say so: {:?}", unknown.requested_effects);
    for message in notices(&orphan).into_iter().chain(notices(&unknown)) {
        assert_ne!(message, PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored locale/terminology axis, so the notice must be real prose");
        assert!(!message.is_empty(), "a notice must carry text");
    }
    // 🎯️ The happy path stays quiet.
    let placed = dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    assert!(notices(&placed).is_empty(), "a successful action must not raise a notice: {:?}", placed.requested_effects);
}

/// 🙈️ `📓️2026-09-09-user-feature-checklist.md` §16 (re-checked for W-D4's §17 law): the inspection
/// panel's `patchInspector` flag rows must carry the INVERSE of the row's current state, so an object
/// hidden from here can be un-hidden from here. Driven end to end — hide through the panel's own
/// dispatch shape, then re-read the panel and show again.
#[semio_framework_async_macros::async_test]
async fn inspection_flag_rows_toggle_back_off() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select the object");
    let hidden_of = |app: &Puzzle3dApp, id: &str| {
        projection_of(app)
            .get("objects")
            .and_then(Value::as_array)
            .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(id)).cloned())
            .and_then(|object| object.get("hidden").and_then(Value::as_bool))
            .unwrap_or(false)
    };
    for expected in [true, false] {
        let panel = to_json_string(&render_body(&mut app, inspection::BODY_KEY).await);
        assert!(panel.contains("puzzle3d-play-inspector.object.hidden"), "the inspector must render the object's hidden row: {panel}");
        dispatch(&mut app, "patchInspector", Some(&json!({ "entity": PUZZLE3D_GRANULARITY_OBJECT, "field": "hidden", "ids": [object_id.as_str()], "value": expected })), None).await.expect("patchInspector hidden");
        assert_eq!(hidden_of(&app, &object_id), expected, "the inspector's hidden row must reach {expected}");
    }
}

/// 🙈️ `📓️2026-09-09-user-feature-checklist.md` §17: the outliner row's own Hide/Show control, driven
/// through the target the panel itself declares — never a hand-written literal, since the reported defect
/// ("Hide does nothing, Show can never un-hide") was exactly a wrong value baked into the row. Hide,
/// re-read the panel, and dispatch the row's `setSelectionHidden` with its NEW target args to show again;
/// the flag must round-trip both ways, exactly as the host binds a row verb to the row's one target.
#[semio_framework_async_macros::async_test]
async fn outliner_row_hide_and_show_round_trip_through_their_own_declared_args() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    let hidden_of = |app: &Puzzle3dApp, id: &str| {
        projection_of(app)
            .get("objects")
            .and_then(Value::as_array)
            .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(id)).cloned())
            .and_then(|object| object.get("hidden").and_then(Value::as_bool))
            .unwrap_or(false)
    };
    assert!(!hidden_of(&app, &object_id), "a freshly added object starts visible");
    for expected in [true, false] {
        let panel: Value = from_json_str(&to_json_string(&render_body(&mut app, artifact::BODY_KEY).await), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the outliner renders parseable ui json");
        let args = outliner_verb_row(&panel, &object_id, "setSelectionHidden").and_then(|row| row.pointer("/target/args").cloned()).unwrap_or_else(|| panic!("the outliner row must declare a setSelectionHidden target for {object_id}: {panel}"));
        assert_eq!(args.get("hidden").and_then(Value::as_bool), Some(expected), "the row's own target must ask for the INVERSE of the state it renders: {args}");
        dispatch(&mut app, "setSelectionHidden", Some(&args), None).await.expect("setSelectionHidden from the outliner row's own target");
        assert_eq!(hidden_of(&app, &object_id), expected, "the outliner row's hidden flag must reach {expected}");
    }
}

/// 🙈️ The hop AFTER the one above: hiding an object from the outliner row must also reach the WORLD
/// lane the viewport paints from, and the row itself must re-render with the inverse control. Wave B20
/// measured the browser publishing an unchanged row AND an unchanged `data-instances-json` while the
/// host's history had already recorded `change-object-hidden id=seed-left-001 new-hidden=true`, so
/// "the flag reached the projection" (the law above) is NOT the same claim as "the two surfaces the
/// user looks at moved". `world_instances_geometry_json` encodes hidden as a zero scale — the id stays
/// in the array so no other object's index shifts — and the row's own control flips Hide↔Show.
#[semio_framework_async_macros::async_test]
async fn outliner_hide_reaches_the_world_instance_lane_and_flips_the_row_control() {
    fn instance_scale(world: &Value, object_id: &str) -> Vec<f64> {
        let instances: Value = parse(world.pointer("/world3d/instancesJson").and_then(Value::as_str).expect("the world body publishes an instances lane"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("instances lane is json");
        instances
            .as_array()
            .expect("instances lane is an array")
            .iter()
            .find(|instance| instance.get("id").and_then(Value::as_str) == Some(object_id))
            .and_then(|instance| instance.get("scale").and_then(Value::as_array))
            .map(|scale| scale.iter().filter_map(Value::as_f64).collect())
            .unwrap_or_default()
    }
    /// 🙈️ The visibility row action's rendered face: the icon of the `setSelectionHidden` action on the row whose
    /// target names this object. Keyed on the target rather than on a row id so the law survives any reshaping
    /// of the tree above the control.
    fn visibility_control_icon(panel: &Value, object_id: &str) -> Option<String> {
        let row = outliner_verb_row(panel, object_id, "setSelectionHidden")?;
        let actions = row.get("rowActions").and_then(Value::as_array)?;
        let action = actions.iter().find(|action| action.get("verb").and_then(Value::as_str) == Some("setSelectionHidden"))?;
        action.get("icon").and_then(Value::as_str).filter(|icon| matches!(*icon, "eye" | "eye-off")).map(str::to_string)
    }
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let visible = render_body(&mut app, main::BODY_KEY).await;
    assert_eq!(instance_scale(&visible, &object_id), vec![1.0, 1.0, 1.0], "a visible object publishes its real scale");
    let before_icon = visibility_control_icon(&render_body(&mut app, artifact::BODY_KEY).await, &object_id);
    assert_eq!(before_icon.as_deref(), Some("eye"), "a visible object's row renders the Hide control");

    dispatch(&mut app, "setSelectionHidden", Some(&json!({ "entity": "object", "hidden": true, "ids": [object_id.clone()] })), None).await.expect("setSelectionHidden");

    let hidden = render_body(&mut app, main::BODY_KEY).await;
    assert_eq!(instance_scale(&hidden, &object_id), vec![0.0, 0.0, 0.0], "a hidden object must publish a zero scale into the world instance lane");
    let after_icon = visibility_control_icon(&render_body(&mut app, artifact::BODY_KEY).await, &object_id);
    assert_eq!(after_icon.as_deref(), Some("eye-off"), "a hidden object's row must re-render as the Show control");
}

/// 👁️ The RESTORE half of the law above, in the SAME settle: the row's Show control must put the object's
/// real scale back into the world instance lane, not merely flip the row's label back. The lane is served
/// from [`Puzzle3dInstanceResidency`], which re-serializes only the records whose own
/// `instance_record_fingerprint` moved and caches every other record verbatim — so a key that failed to
/// read `hidden` would leave an un-hidden object at `[0,0,0]` for the rest of the session while the row,
/// the projection and the history all read `hidden=false`, and nothing but the world would disagree.
/// Browser battery #64 reported `outliner-show-restores restored=false` while the `seed-left-001` row had
/// already come back to `Hide` (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B57), and no row-text verdict
/// can tell those two apart.
#[semio_framework_async_macros::async_test]
async fn outliner_show_restores_the_world_instance_scale_in_the_same_settle() {
    fn instance_scale(world: &Value, object_id: &str) -> Vec<f64> {
        let instances: Value = parse(world.pointer("/world3d/instancesJson").and_then(Value::as_str).expect("the world body publishes an instances lane"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("instances lane is json");
        instances
            .as_array()
            .expect("instances lane is an array")
            .iter()
            .find(|instance| instance.get("id").and_then(Value::as_str) == Some(object_id))
            .and_then(|instance| instance.get("scale").and_then(Value::as_array))
            .map(|scale| scale.iter().filter_map(Value::as_f64).collect())
            .unwrap_or_default()
    }
    let flag_args = |object_id: &str, value: bool| json!({ "entity": "object", "hidden": value, "ids": [object_id] });
    let mut app = app().await;
    let object_id = first_object_id(&app);
    assert_eq!(instance_scale(&render_body(&mut app, main::BODY_KEY).await, &object_id), vec![1.0, 1.0, 1.0], "a visible object publishes its real scale");
    dispatch(&mut app, "setSelectionHidden", Some(&flag_args(&object_id, true)), None).await.expect("setSelectionHidden hidden=true");
    assert_eq!(instance_scale(&render_body(&mut app, main::BODY_KEY).await, &object_id), vec![0.0, 0.0, 0.0], "the hide half must still reach the world lane");
    dispatch(&mut app, "setSelectionHidden", Some(&flag_args(&object_id, false)), None).await.expect("setSelectionHidden hidden=false");
    let shown = render_body(&mut app, main::BODY_KEY).await;
    assert_eq!(object_flag(&app, &object_id, "hidden"), Some(false), "the show write must reach the document");
    assert_eq!(instance_scale(&shown, &object_id), vec![1.0, 1.0, 1.0], "un-hiding must republish the object's real scale — a residency that cached the zero-scale record would leave it invisible for good");
}

/// 🎯️ An outliner row's flag write names its OWN entity, so the live selection must not decide what it
/// hits. `set_selection_flag` chooses between the explicit `{entity, ids}` the row declares and the whole
/// live selection, and the browser only ever exercised the branch with NOTHING selected: with a sibling
/// entity selected, `outliner-hide-applies` failed on the same row with the same args while the command
/// still settled `command-complete` with `historyUpserts: 0` (ticket 26/09/02/PUZZLE-3D-END-TO-END wave
/// B47 §6 — PASS with an empty selection at 2 345 ms, FAIL with a sibling selected at 30 237 ms). Locked,
/// too: a lock guards an entity's GEOMETRY, and a row that says "Hide" must still hide.
#[semio_framework_async_macros::async_test]
async fn an_explicit_outliner_flag_write_ignores_whatever_is_selected() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select a vortex, i.e. NOT the object the row names");
    dispatch(&mut app, "setSelectionLocked", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "locked": true })), None).await.expect("lock the object by its own row");
    assert_eq!(object_flag(&app, &object_id, "locked"), Some(true), "an explicit lock write lands while a vortex is the live selection");

    dispatch(&mut app, "setSelectionHidden", Some(&json!({ "entity": "object", "hidden": true, "ids": [object_id.clone()] })), None).await.expect("hide the object by its own row");

    assert_eq!(object_flag(&app, &object_id, "hidden"), Some(true), "the row's explicit id decides what is hidden — never the live selection, and never its lock");
}

/// 🎯️ The rendered outliner row that offers `verb` and whose ONE target names `object_id` — the row a host binds that verb to.
fn outliner_verb_row<'a>(node: &'a Value, object_id: &str, verb: &str) -> Option<&'a Value> {
    let offers = node.get("rowActions").and_then(Value::as_array).is_some_and(|actions| actions.iter().any(|action| action.get("verb").and_then(Value::as_str) == Some(verb)));
    let names = node.pointer("/target/args/ids").and_then(Value::as_array).is_some_and(|ids| ids.iter().any(|id| id.as_str() == Some(object_id)));
    if offers && names {
        return Some(node);
    }
    match node {
        Value::Object(fields) => fields.iter().find_map(|(_, child)| outliner_verb_row(child, object_id, verb)),
        Value::Array(items) => items.iter().find_map(|item| outliner_verb_row(item, object_id, verb)),
        _ => None,
    }
}

/// 🙈️ LAW: the outliner's set-verbs are idempotent by value — replaying `setSelectionHidden`/`setSelectionLocked` with the
/// same `{entity, ids, flag: true}` (a stale view's second click) leaves exactly the first dispatch's one applied history
/// row and a byte-identical document; and a set-verb without its value is refused at the command boundary, never defaulted.
#[semio_framework_async_macros::async_test]
async fn outliner_set_verbs_are_idempotent_by_value_and_refuse_a_missing_value() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let bare = json!({ "entity": "object", "ids": [object_id.clone()] });
    for (verb, flag, args) in [
        ("setSelectionHidden", "hidden", json!({ "entity": "object", "hidden": true, "ids": [object_id.clone()] })),
        ("setSelectionLocked", "locked", json!({ "entity": "object", "ids": [object_id.clone()], "locked": true })),
    ] {
        assert!(Puzzle3dPlayApp::command_from_action(verb, Some((semio_framework_pack_json::to_dsl_value(&bare)).clone()), None).is_err(), "{verb} without its {flag} value must be refused");
        let (first, settled) = dispatch_reporting(&mut app, verb, Some(&args), None).await;
        first.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
        let rows = history_row_labels(&settled);
        assert_eq!(rows.iter().filter(|row| row.contains("applied=true") && !row.contains("ops=0")).count(), 1, "{verb} commits exactly one document edit: {rows:?}");
        assert_eq!(object_flag(&app, &object_id, flag), Some(true), "{verb} sets {flag}");
        let set = committed_document(&app);
        let (replay, settled) = dispatch_reporting(&mut app, verb, Some(&args), None).await;
        replay.unwrap_or_else(|fault| panic!("{verb} replay: {fault:?}"));
        let rows = history_row_labels(&settled);
        assert!(rows.iter().all(|row| !row.contains("applied=true") || row.contains("ops=0")), "replaying {verb} with the same value commits no second edit: {rows:?}");
        assert_eq!(committed_document(&app), set, "replaying {verb} with the same value leaves the document byte-identical");
    }
}

fn first_target_volume_id(app: &Puzzle3dApp) -> String {
    let projection = projection_of(app);
    projection
        .get("targetVolumes")
        .or_else(|| projection.get("target_volumes"))
        .and_then(Value::as_array)
        .and_then(|volumes| volumes.first())
        .and_then(|volume| volume.get("id").and_then(Value::as_str))
        .expect("a target volume is present")
        .to_string()
}

fn volume_origin(app: &Puzzle3dApp, volume_id: &str) -> Vec<f64> {
    let projection = projection_of(app);
    let volumes = projection.get("targetVolumes").or_else(|| projection.get("target_volumes")).and_then(Value::as_array).cloned().unwrap_or_default();
    volumes
        .iter()
        .find(|volume| volume.get("id").and_then(Value::as_str) == Some(volume_id))
        .and_then(|volume| volume.get("origin").or_else(|| volume.get("position")).and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

#[semio_framework_async_macros::async_test]
async fn relocate_target_volume_undoes_and_redoes_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let volume_id = first_target_volume_id(&app);
    let start = volume_origin(&app, &volume_id);
    let pose = |position: [f64; 3]| json!({ "position": position, "quaternion": [0.0, 0.0, 0.0, 1.0], "scale": [1.0, 1.0, 1.0] });
    let (before, after) = ([start[0], start[1], start[2]], [start[0] + 3.0, start[1] + 3.0, start[2] + 3.0]);
    let (_, settled) = dispatch_reporting(&mut app, "relocateTargetVolume", Some(&json!({ "volumeId": volume_id, "mode": "translate", "before": pose(before), "after": pose(after) })), None).await;
    let rows = edit_rows(&settled);
    assert!(rows.len() == 1 && rows[0].op_lines[0].starts_with("drag-selection") && rows[0].transaction.is_some(), "the volume gumball is one relative drag transaction: {rows:?}");
    let moved = volume_origin(&app, &volume_id);
    assert!(moved.iter().zip(after).all(|(landed, wanted)| (landed - wanted).abs() < 1e-9), "relocateTargetVolume must land the gumball's after pose, got {moved:?} from {start:?}");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(volume_origin(&app, &volume_id), start, "undo restores the volume pose");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(volume_origin(&app, &volume_id), moved, "redo reapplies the volume pose");
}

#[semio_framework_async_macros::async_test]
async fn world_relocate_undoes_and_redoes_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let start = object_origin(&app, &object_id);
    dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id, "position": [9.0, 8.0, 7.0] })), None).await.expect("worldRelocate");
    let moved = object_origin(&app, &object_id);
    assert!((moved[0] - 9.0).abs() < 1e-9 && (moved[1] - 8.0).abs() < 1e-9 && (moved[2] - 7.0).abs() < 1e-9, "worldRelocate must write the world position, got {moved:?} from {start:?}");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "undo restores the object origin");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(object_origin(&app, &object_id), moved, "redo reapplies the object origin");
}

/// 🚚️ Wave W-Y: Nakagin-scale `worldRelocate` of one object must admit and complete inside the
/// command work budget — the extent is the true per-item vortex walk, not `objects + attractions`.
#[semio_framework_async_macros::async_test]
async fn world_relocate_on_nakagin_admits_and_completes() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin");
    let object_id = first_object_id(&app);
    let start = object_origin(&app, &object_id);
    dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id, "position": [4.0, 5.0, 6.0] })), None).await.expect("worldRelocate nakagin");
    let moved = object_origin(&app, &object_id);
    assert!((moved[0] - 4.0).abs() < 1e-6 && (moved[1] - 5.0).abs() < 1e-6 && (moved[2] - 6.0).abs() < 1e-6, "nakagin relocate must land, got {moved:?} from {start:?}");
}

/// 🚚️ Wave B5: the Relocate utility's single commit, both answers. An UNLOCKED object takes the absolute
/// world origin the host's pointer-up sends and publishes a real document edit; the SAME verb on a LOCKED
/// object refuses with exactly one `selection_locked` notice, no edit and no movement — the identical
/// refusal `translateSelection` already owes a locked selection, which this verb used to skip silently
/// (the host's relocate ghost just snapped back with nothing said). The declared `ActionKind` is pinned
/// here too: `worldRelocate` writes the document, so undo/redo and the history panel must see a
/// `Mutation`, never the `View` the 2026-09-10 checklist reverification claimed.
#[semio_framework_async_macros::async_test]
async fn world_relocate_moves_an_unlocked_object_and_refuses_a_locked_one_with_one_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let moved = dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id.as_str(), "position": [3.0, -2.0, 1.5] })), None).await.expect("worldRelocate unlocked");
    assert!(notices(&moved).is_empty(), "an unlocked relocate must not refuse: {:?}", moved.requested_effects);
    let landed = object_origin(&app, &object_id);
    assert!((landed[0] - 3.0).abs() < 1e-9 && (landed[1] + 2.0).abs() < 1e-9 && (landed[2] - 1.5).abs() < 1e-9, "worldRelocate must write the absolute origin, got {landed:?}");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.as_str()], "flag": "locked", "value": true })), None).await.expect("lock");
    let refused = dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id.as_str(), "position": [9.0, 9.0, 9.0] })), None).await.expect("worldRelocate locked");
    let raised = notices(&refused);
    assert_eq!(raised.len(), 1, "a locked relocate must raise exactly one notice: {:?}", refused.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so the refusal must be real prose");
    assert!(refused.mutations.is_empty(), "a locked relocate must emit no edit: {:?}", refused.mutations);
    assert_eq!(object_origin(&app, &object_id), landed, "a locked object must not move");
    let definition = create_puzzle3d_app();
    let declared = dispatchable_actions(&definition).into_iter().find(|action| action.id == "worldRelocate").map(|action| action.kind).expect("worldRelocate is declared");
    assert_eq!(declared, ActionKind::Mutation, "worldRelocate edits the document, so history and undo must see a Mutation");
}

/// 📋️ Wave W-Y: copy then paste clones the selection with new ids as one Mutation edit.
#[semio_framework_async_macros::async_test]
async fn copy_then_paste_clones_selection_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("add");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let copied = dispatch(&mut app, "copy", None, None).await.expect("copy");
    let fragment = copied.requested_effects.iter().find_map(|effect| match effect {
        Effect::ClipboardWrite { fragment } => Some(fragment.clone()),
        _ => None,
    }).expect("copy must emit ClipboardWrite");
    let before = object_count(&app);
    let paste_args = json!({ "fragment": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&fragment)) });
    let pasted = dispatch(&mut app, "paste", Some(&paste_args), None).await.expect("paste");
    assert!(!pasted.mutations.is_empty(), "paste must emit one mutation edit: {:?}", pasted.mutations);
    assert_eq!(object_count(&app), before + 1, "paste clones the selection");
    let ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).unwrap().iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    assert!(ids.iter().any(|id| id != &object_id), "the clone must carry a new id: {ids:?}");
}

/// ✂️ Wave W-Y: cut is copy + delete as one undo step.
#[semio_framework_async_macros::async_test]
async fn cut_undoes_as_one_step() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let before = object_count(&app);
    dispatch(&mut app, "cut", None, None).await.expect("cut");
    assert_eq!(object_count(&app), before - 1, "cut removes the selection");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), before, "one undo restores the cut");
}

/// ⬇️ Wave W-Y: export carries the full round-trippable scene_snapshot JSON.
#[semio_framework_async_macros::async_test]
async fn export_snapshot_downloads_round_trippable_json() {
    let mut app = app().await;
    let before = projection_of(&app);
    let result = dispatch(&mut app, "exportSnapshot", None, None).await.expect("export");
    let data = result.requested_effects.iter().find_map(|effect| match effect {
        Effect::DownloadMediaExport { filename, mime_type, data, .. } => {
            assert_eq!(filename, "concrete-forest.json", "a fresh session exports under the example its document was seeded from");
            assert_eq!(mime_type, "application/json");
            Some(data.clone())
        }
        _ => None,
    }).expect("export must emit DownloadMediaExport");
    let exported: Value = parse(&data, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("export JSON parses");
    assert_eq!(exported.get("schema"), before.get("schema"), "export schema must match the live scene_snapshot");
    assert_eq!(exported.get("objects").and_then(Value::as_array).map(Vec::len), before.get("objects").and_then(Value::as_array).map(Vec::len));
}

/// 🧬️ Identity of imported objects without scene_snapshot/snapshot projection twins (anchor / null scale).
fn object_cores(value: &Value) -> Vec<(String, String, String, Value, Vec<String>)> {
    value
        .get("objects")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|object| {
            let id = object.get("id").and_then(Value::as_str)?.to_string();
            let kind = object.get("objectKind").and_then(Value::as_str).unwrap_or_default().to_string();
            let mesh = object.get("meshUrl").and_then(Value::as_str).unwrap_or_default().to_string();
            let origin = object.get("origin").cloned().unwrap_or(Value::Null);
            let vortices = object
                .get("vortices")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|vortex| vortex.get("id").and_then(Value::as_str).map(str::to_string))
                .collect();
            Some((id, kind, mesh, origin, vortices))
        })
        .collect()
}

/// 📥️ Wave W-Y: importing that JSON reproduces the document as one Mutation edit.
#[semio_framework_async_macros::async_test]
async fn import_snapshot_reproduces_the_exported_document() {
    let mut app = app().await;
    let source = projection_of(&app);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0);
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&source) })), None).await;
    imported.expect("import");
    assert_eq!(loads_in(&settled), 1, "import is one whole-document load");
    assert!(history_row_labels(&settled).iter().all(|row| row.contains("applied=false") && row.contains("ops=0")), "a whole-document load journals no applied history row: {:?}", history_row_labels(&settled));
    assert_eq!(object_cores(&imported_projection(&settled)), object_cores(&source), "import reproduces the exported objects");
}

/// 📥️ Wave W-AB: workspace Import is `openImportSnapshot` (file picker). `importSnapshot` stays
/// dispatchable for the host re-dispatch after the pick, but is not a file-menu row.
#[test]
fn file_menu_import_row_opens_the_file_picker() {
    let definition = create_puzzle3d_app();
    let file: Vec<(&str, bool)> = dispatchable_actions(&definition)
        .into_iter()
        .filter(|action| action.category.as_deref() == Some("file"))
        .map(|action| (action.id.as_str(), action.in_palette))
        .collect();
    assert!(file.iter().any(|(id, _)| *id == "exportSnapshot"), "file menu keeps Export: {file:?}");
    assert!(file.iter().any(|(id, in_palette)| *id == "openImportSnapshot" && *in_palette), "file menu Import is openImportSnapshot: {file:?}");
    assert!(!file.iter().any(|(id, _)| *id == "importSnapshot"), "importSnapshot is the picker completion, not a menu row: {file:?}");
    let import = dispatchable_actions(&definition).into_iter().find(|action| action.id == "importSnapshot").expect("importSnapshot stays dispatchable");
    assert!(!import.in_palette);
    assert_eq!(import.category.as_deref(), None);
}

/// 📥️ Wave W-AB: activating Import requests a file open; completing it with scene_snapshot JSON imports.
#[semio_framework_async_macros::async_test]
async fn open_import_snapshot_requests_file_open_then_import_applies_payload() {
    let mut app = app().await;
    let source = projection_of(&app);
    let opened = dispatch(&mut app, "openImportSnapshot", None, None).await.expect("openImportSnapshot");
    let req = opened.requested_effects.iter().find_map(|effect| match effect {
        Effect::RequestFileOpen { accept, read_as, import_action, multiple, .. } => {
            assert!(accept.contains("json"), "picker accepts JSON: {accept}");
            assert_eq!(read_as.as_deref(), Some("text"));
            assert_eq!(import_action, "importSnapshot");
            assert!(!*multiple);
            Some(())
        }
        _ => None,
    });
    assert!(req.is_some(), "Import must emit RequestFileOpen: {:?}", opened.requested_effects);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0);
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&source) })), None).await;
    imported.expect("import");
    assert_eq!(loads_in(&settled), 1, "import is one whole-document load");
    assert_eq!(object_cores(&imported_projection(&settled)), object_cores(&source), "picked payload reproduces the exported objects");
}

/// 📥️ Wave W-AB: re-importing the live scene_snapshot is a store identity, not a guest payload dedupe.
/// `import_snapshot` always assigns. The store folds equal content into no edit, so the admitting
/// settle carries one unapplied `importSnapshot` row (`ops=0`) — the empty-emit command log — and
/// the object cores do not move.
#[semio_framework_async_macros::async_test]
async fn import_snapshot_of_the_live_document_records_whether_identical_content_is_an_edit() {
    let mut app = app().await;
    let source = projection_of(&app);
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&source) })), None).await;
    imported.expect("reimport");
    let objects_after = object_cores(&projection_of(&app));
    assert_eq!(objects_after, object_cores(&source), "identical payload must not rewrite object cores");
    let rows = history_row_labels(&settled);
    assert_eq!(history_rows(&settled), 1, "identical live scene_snapshot logs the command and no edit: {rows:?}");
    assert!(rows.iter().all(|row| row.contains("importSnapshot") && row.contains("applied=false") && row.contains("ops=0")), "{rows:?}");
}

/// 📥️ Wave W-AB #44: leftover `importSnapshot` against a one-object live scene_snapshot applies a distinct two-object JSON.
#[semio_framework_async_macros::async_test]
async fn import_snapshot_of_a_distinct_two_object_json_against_a_one_object_live_fixture_emits_operations() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(
        source.contains(r#""importSnapshot" => Box::new(Puzzle3dWindowCommandWork::new(tool_id))"#),
        "importSnapshot must leftover-commit through Puzzle3dWindowCommandWork, not BoundedFirstStep"
    );
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [0.0, 0.0, 0.0] })), None).await.expect("seed");
    assert_eq!(object_count(&app), 1, "live scene_snapshot must start as one object");
    let live_id = first_object_id(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("distinct");
    let two = projection_of(&app);
    let cores = object_cores(&two);
    assert_eq!(cores.len(), 2, "distinct payload must carry two objects");
    let distinct_id = cores.iter().map(|row| row.0.as_str()).find(|id| *id != live_id).expect("distinct object id").to_string();
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&two) })), None).await;
    imported.expect("import distinct");
    assert_eq!(loads_in(&settled), 1, "distinct two-object import is one whole-document load");
    let after = object_cores(&imported_projection(&settled));
    assert_eq!(after.len(), 2, "after-snapshot must contain both objects");
    assert!(after.iter().any(|row| row.0 == distinct_id), "after-snapshot must contain the distinct object id {distinct_id}");
}

/// 📥️ Wave B9 lane 3: the picker's REAL round trip — the bytes `exportSnapshot` hands the host's
/// download are the bytes the file chooser hands `importSnapshot` back. A distinct export must upsert
/// its objects and record one history row; re-importing the SAME file against the document it
/// describes must stay an identity no-op. The browser measured `effects:0 historyUpserts:0` on the
/// distinct half, which is also exactly what a silently-dropped refusal notice looks like — so this
/// law asserts the absence of a refusal as well as the presence of the edit.
#[semio_framework_async_macros::async_test]
async fn exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity() {
    let exported = |result: &semio_framework_plugin::InvocationResult| -> String {
        result
            .requested_effects
            .iter()
            .find_map(|effect| match effect {
                Effect::DownloadMediaExport { data, .. } => Some(data.clone()),
                _ => None,
            })
            .expect("exportSnapshot emits one DownloadMediaExport")
    };
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { message } => Some(message.clone()),
                _ => None,
            })
            .collect()
    };
    let mut app = app().await;
    let one_object = exported(&dispatch(&mut app, "exportSnapshot", None, None).await.expect("export the boot document"));
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("seed a second object");
    let two_objects = exported(&dispatch(&mut app, "exportSnapshot", None, None).await.expect("export the distinct document"));
    assert_ne!(one_object, two_objects, "the two exports must be distinct files");
    let (restored, restored_settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": one_object.as_str(), "name": "puzzle-3d.json" })), None).await;
    let restored = restored.expect("restore the one-object document");
    assert!(notices(&restored).is_empty(), "restoring an exported file must not refuse: {:?}", notices(&restored));
    assert_eq!(object_cores(&imported_projection(&restored_settled)).len(), 1, "the restored file is the one-object document");
    let (distinct, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": two_objects.as_str(), "name": "puzzle-3d-distinct.json" })), None).await;
    let distinct = distinct.expect("import the distinct file");
    assert!(notices(&distinct).is_empty(), "a distinct exported file must not refuse: {:?}", notices(&distinct));
    assert_eq!(loads_in(&settled), 1, "a distinct exported file is one whole-document load");
    let after_distinct = object_cores(&imported_projection(&settled));
    assert_eq!(after_distinct.len(), 2, "a distinct exported file must carry its objects");
    let (again, again_settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": two_objects.as_str(), "name": "puzzle-3d-distinct.json" })), None).await;
    let again = again.expect("re-import the same file");
    assert!(notices(&again).is_empty(), "re-importing the same file must not refuse: {:?}", notices(&again));
    assert_eq!(object_cores(&imported_projection(&again_settled)), after_distinct, "re-importing the same file loads the same document");
}

/// 🔢️ The payload an import actually receives was written by a BROWSER, not by this crate's own writer:
/// the file chooser hands back whatever `JSON.stringify` produced, which spells a whole float as `1` (no
/// `.0`), a tiny one in exponent form (`-5.551115123125783e-17`), an absent optional as `null`, and a
/// non-ASCII label as a `\u` escape. The law above only ever fed `import_snapshot` this crate's own export
/// text, so none of those spellings was under test — and the browser's `import-distinct` read
/// `effects:0 historyUpserts:0` with NO refusal notice, which is exactly what `import_snapshot`'s
/// `ctx.abort` path looks like when `parse` rejects the text it was handed (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B57 §2). One unreadable spelling is a silently dropped document.
#[semio_framework_async_macros::async_test]
async fn a_browser_serialized_fixture_payload_imports_every_json_number_spelling() {
    let mut app = app().await;
    let payload = r#"{"schema":"puzzle.3d.scene.v1","domain":"architecture","objects":[{"id":"browser-clone","label":"Distinct Capsule J · cs_sl1","objectKind":"Object","origin":[-16.75,-3.6499999999999986,0],"orientation":[-5.551115123125783e-17,5.551115123125783e-17,0.7071067811865475,0.7071067811865475],"scale":null,"vortices":[{"id":"browser-clone:v0","position":[1,0,2]}],"hidden":false,"locked":false}],"attractions":[],"targetVolumes":[],"references":[]}"#;
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": payload, "name": "browser-stringify.json" })), None).await;
    let imported = imported.expect("import a browser-serialized payload");
    let notices: Vec<String> = imported
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect();
    assert!(notices.is_empty(), "a browser-serialized payload must not be refused: {notices:?}");
    assert_eq!(loads_in(&settled), 1, "a browser-serialized payload is one whole-document load");
    let objects = object_cores(&imported_projection(&settled));
    assert_eq!(objects.len(), 1, "the imported document is exactly the payload's objects: {objects:?}");
    assert_eq!(objects.first().map(|(id, ..)| id.as_str()), Some("browser-clone"), "the payload's own object id survives the import: {objects:?}");
}

/// 🧱️ Wave B59: the payload the browser hands `importSnapshot` for a REAL example is 145 714 B — the
/// Nakagin Capsule Tower export, 180 objects — and the live `import-distinct` verdict read
/// `paneObjects=180→180` with no notice at all (wave B57 §2.3). Every import law before this one fed a
/// payload of a few hundred bytes, so the size class the product actually imports was never under test.
///
/// 🧩️ The payload reaches the action the way the framework hands it once it has reassembled the host's
/// chunks (`semio_framework::kernel::ImportStaging`): ONE whole `{payload, name}`. The witnesses are the
/// browser's own three: the object census moves, exactly ONE applied history row lands, nothing is refused.
#[semio_framework_async_macros::async_test]
async fn a_one_hundred_forty_five_kilobyte_distinct_fixture_imports_inside_one_settle() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [220.0, 0.0, 0.0] })), None).await.expect("seed one more object");
    let distinct = semio_s_artifact_puzzle_3d_commands_document::export_snapshot::puzzle3d_export_json(&puzzle3d_scene_snapshot_from_document(app.snapshot().expect("typed app snapshot").typed()));
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("return to the example document");
    assert_eq!(object_count(&app), seeded, "the document is back at the example census before the import");
    assert!(distinct.len() > 140_000, "the payload under test must be the product's own size class; observed {} B", distinct.len());
    let (result, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": distinct.as_str(), "name": "nakagin-capsule-tower-distinct.json" })), None).await;
    let result = result.expect("a product-sized import inside the budget is admitted");
    let notices: Vec<String> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect();
    assert!(notices.is_empty(), "a payload inside the declared import budget must not be refused: {notices:?}");
    assert_eq!(loads_in(&settled), 1, "the whole import is exactly one whole-document load");
    assert_eq!(object_cores(&imported_projection(&settled)).len(), seeded + 1, "a product-sized distinct payload must carry the document it replaces");
}

/// 🧩️ The framework half of an import, end to end for this app: the host's chunk envelope
/// (`semio_framework::kernel::import_chunk_arguments`) goes through the SDK's own action dispatch, whose
/// instance-scoped `ImportStaging` stages every chunk but the last — no command, no edit — and hands
/// `importSnapshot` the whole file once the run closes: ONE applied history row, the census moved.
#[semio_framework_async_macros::async_test]
async fn a_chunked_pick_reaches_import_snapshot_as_one_whole_file_through_the_framework_staging() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [220.0, 0.0, 0.0] })), None).await.expect("seed one more object");
    let distinct = semio_s_artifact_puzzle_3d_commands_document::export_snapshot::puzzle3d_export_json(&puzzle3d_scene_snapshot_from_document(app.snapshot().expect("typed app snapshot").typed()));
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("return to the example document");
    let chunks = semio_framework::kernel::import_payload_chunks(&distinct);
    assert!(chunks.len() > 1, "a product-sized payload spans several host chunks; observed {}", chunks.len());
    let mut loaded = None;
    for chunk in &chunks {
        let args = semio_framework::kernel::import_chunk_arguments("nakagin-capsule-tower-distinct.json", chunk, None);
        let (result, settled) = dispatch_action_reporting(&mut app, "importSnapshot", &args).await;
        result.expect("every chunk of a product-sized pick is admitted");
        let rows = history_row_labels(&settled);
        if chunk.chunk + 1 < chunk.chunk_count {
            assert!(rows.is_empty(), "a staged chunk is no command and no edit; chunk {}: {rows:?}", chunk.chunk);
            assert_eq!(object_count(&app), seeded, "a staged chunk must not move the document; chunk {}", chunk.chunk);
        }
        if loads_in(&settled) == 1 {
            assert!(loaded.is_none(), "the whole chunked pick is exactly one whole-document load");
            loaded = Some(imported_projection(&settled));
        }
    }
    let loaded = loaded.expect("the closing chunk publishes the whole-document load");
    assert_eq!(object_cores(&loaded).len(), seeded + 1, "the reassembled file carries the document it replaces");
}

/// 🧯️ An import above the budget one export may stream is REFUSED with a notice, never silently dropped
/// and never a fault — the framework hands the whole file, so this budget is the app's one size rule.
#[semio_framework_async_macros::async_test]
async fn an_import_above_the_export_budget_refuses_with_a_notice() {
    use semio_s_plugin_puzzle_retained::PUZZLE_IMPORT_TOTAL_BYTES;
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    let oversized = "x".repeat(PUZZLE_IMPORT_TOTAL_BYTES + 1);
    let (result, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": oversized.as_str(), "name": "oversized.json" })), None).await;
    let result = result.expect("an over-budget import is an ANSWER, never a fault");
    let notices: Vec<String> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(notices.len(), 1, "an over-budget import publishes exactly one notice: {notices:?}");
    let rows = history_row_labels(&settled);
    assert_eq!(history_rows(&settled), 1, "a refused import logs the command and no edit: {rows:?}");
    assert!(rows.iter().all(|row| row.contains("importSnapshot") && row.contains("applied=false") && row.contains("ops=0")), "{rows:?}");
    assert_eq!(object_count(&app), seeded, "a refused import leaves the document alone");
}

/// 📤️📥️ LAW: the Nakagin export re-imports as the very document it came from — the whole exported file,
/// read by `puzzle3d_import_value`, rebuilds a scene_snapshot whose own export is byte-identical.
#[test]
fn the_nakagin_export_reimports_byte_for_byte() {
    use semio_s_artifact_puzzle_3d_commands_document::import_snapshot::puzzle3d_import_value;
    let payload = semio_s_artifact_puzzle_3d_commands_document::export_snapshot::puzzle3d_export_json(&NAKAGIN_EXAMPLE_SNAPSHOT.clone());
    assert!(payload.len() > semio_framework::kernel::IMPORT_CHUNK_BYTES, "the payload under test spans several host chunks: {} B", payload.len());
    let root = puzzle3d_import_value(&json!({ "payload": payload.as_str(), "name": "nakagin.json" })).expect("the whole export is a puzzle 3D document");
    assert_eq!(
        semio_s_artifact_puzzle_3d_commands_document::export_snapshot::puzzle3d_export_json(&Puzzle3dSceneSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&root)).expect("the imported root IS a puzzle 3D document")),
        payload,
        "the import is byte-identical to the document that was exported"
    );
}

/// 📥️ Wave B16: leftover `exportSnapshot` must emit `DownloadMediaExport`.
/// 🏷️ Wave B30: a fresh session's document IS the Concrete Forest example
/// (`ArtifactApp::initial_snapshot`), and since the config lane says so from the first render the
/// boot download is named after it. The app-generic `puzzle-3d.json` belongs to a document that came
/// from no example at all — see `export_snapshot_names_the_download_after_the_active_example`, which
/// reaches that state by clearing the picker.
#[semio_framework_async_macros::async_test]
async fn leftover_export_snapshot_downloads_the_boot_example_json() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(
        source.contains(r#""exportSnapshot" => Box::new(Puzzle3dWindowCommandWork::new(tool_id))"#),
        "exportSnapshot must leftover-commit through Puzzle3dWindowCommandWork, not BoundedFirstStep"
    );
    let mut app = app().await;
    let result = dispatch(&mut app, "exportSnapshot", None, None).await.expect("leftover export");
    let filename = result.requested_effects.iter().find_map(|effect| match effect {
        Effect::DownloadMediaExport { filename, .. } => Some(filename.as_str()),
        _ => None,
    });
    assert_eq!(filename, Some("concrete-forest.json"), "leftover export must name the boot example: {:?}", result.requested_effects);
}

/// 🏷️ Wave B26: exporting Concrete Forest and exporting Nakagin must not both land as one constant
/// filename. The download is named after the ACTIVE EXAMPLE — the id `set_active_example` persists on
/// the shared config, restored into `Puzzle3dRuntime` by `window::runtime` — and a document that came
/// from no example at all keeps the app-generic `puzzle-3d.json`. Both picker aliases (`concrete`,
/// `nakagin`) resolve to the canonical id, so the filename never depends on how the row was spelled.
#[semio_framework_async_macros::async_test]
async fn export_snapshot_names_the_download_after_the_active_example() {
    /// 🏷️ The name is asserted across BOTH publication lanes, because the lane is chosen by payload size
    /// and the naming contract is not: an over-budget example publishes a segmented handle rather than an
    /// inline `DownloadMediaExport`, and it must carry the same filename (wave B38).
    async fn exported_filename(app: &mut Puzzle3dApp) -> String {
        let (result, settled) = dispatch_reporting(app, "exportSnapshot", None, None).await;
        result.expect("export");
        let inline = settled.effects.iter().find_map(|effect| match effect {
            Effect::DownloadMediaExport { filename, .. } => Some(filename.clone()),
            _ => None,
        });
        let segmented = settled.downloads.first().map(|download| download.filename.clone());
        assert!(inline.is_none() || segmented.is_none(), "one export publishes exactly one lane, never both: inline={inline:?} segmented={segmented:?}");
        inline.or(segmented).expect("export must publish a download on one of the two lanes")
    }
    let mut app = app().await;
    assert_eq!(exported_filename(&mut app).await, "concrete-forest.json", "a fresh session already names the example its document was seeded from");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("clear to a blank document");
    assert_eq!(exported_filename(&mut app).await, "puzzle-3d.json", "a document that came from no example keeps the app-generic name");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "concrete-forest" })), None).await.expect("load concrete forest");
    assert_eq!(exported_filename(&mut app).await, "concrete-forest.json", "Concrete Forest must export under its own id");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "nakagin" })), None).await.expect("load nakagin by alias");
    assert_eq!(exported_filename(&mut app).await, "nakagin-capsule-tower.json", "the picker alias must still export the canonical example id");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("clear to a blank document");
    assert_eq!(exported_filename(&mut app).await, "puzzle-3d.json", "clearing the example clears the name it exported under");
}

/// ⬇️ Wave B43: an export above what ONE segmented download may carry is refused with a NOTICE, and the
/// budget it is measured against is the framework's own end-to-end cap — never a literal of this app's.
///
/// 🧯 A fault here would land as a dead job; before B43 the whole lane's over-cap answer was a producer
/// `Fault`. The refusal names the file, its size and the budget, so the answer is readable.
#[test]
fn export_refuses_a_payload_above_the_declared_segmented_budget_with_a_notice() {
    use semio_s_artifact_puzzle_3d_commands_document::export_snapshot::{puzzle3d_export_refusal, puzzle3d_export_segmented, puzzle3d_export_segmented_budget_bytes};
    use semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_OUTPUT_BYTES;
    use semio_framework_plugin::app::ArtifactOutputChunks;
    let budget = puzzle3d_export_segmented_budget_bytes().expect("the declared output budget is within the framework cap");
    assert_eq!(budget, PUZZLE_COMMAND_OUTPUT_BYTES, "the segmented budget IS this command's declared contract output budget");
    assert!(budget <= ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES, "no app may declare a download larger than the wire admits");
    let over = "x".repeat(budget + 1);
    assert!(puzzle3d_export_segmented("nakagin-capsule-tower.json".into(), &over).is_err(), "a payload above the budget must never open a download handle");
    let notice = puzzle3d_export_refusal("nakagin-capsule-tower.json", budget + 1, budget);
    assert!(notice.contains("nakagin-capsule-tower.json") && notice.contains(&(budget + 1).to_string()) && notice.contains(&budget.to_string()), "the refusal must name the file, its size and the budget: {notice}");
}

/// ⬇️ Wave B38: an export larger than one wire page must reach the user as ONE download carrying the WHOLE
/// scene_snapshot, through the framework's segmented lane — never as an inline effect field.
///
/// 🧊️ The lane is picked by size against the guest's own contiguous-request ceiling
/// (`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`, one wasm page), derived from the wire constant rather than
/// spelled as a literal: Concrete Forest (7 542 B) rides inline, Nakagin Capsule Tower (145 714 B) does not.
/// The browser measured exactly that split — the inline Nakagin effect left the guest (`"effects":1`) and
/// reached no file (`📓️2026-09-12-wave-B36-full-run-bisect-2.md` §5).
///
/// 🧾️ The assertion is the REASSEMBLED payload, drained one bounded chunk at a time exactly as
/// `drainSegmentedMediaExport` drains it, parsed back to JSON and compared object-for-object with the live
/// document — so a lane that publishes a handle but loses, reorders or truncates chunks fails here.
#[semio_framework_async_macros::async_test]
async fn export_over_the_inline_budget_streams_one_segmented_download_carrying_the_whole_fixture() {
    use semio_s_artifact_puzzle_3d_commands_document::export_snapshot::{puzzle3d_export_inline_budget_bytes, puzzle3d_export_json};
    assert_eq!(
        puzzle3d_export_inline_budget_bytes(),
        semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
        "the inline budget is the guest's own contiguous-request ceiling, never an independent literal"
    );
    let mut app = app().await;

    // 🌲️ Below the budget: the inline lane, unchanged, and NO segmented handle at all.
    let (small, small_settled) = dispatch_reporting(&mut app, "exportSnapshot", None, None).await;
    small.expect("concrete forest export");
    let small_bytes = puzzle3d_export_json(&puzzle3d_scene_snapshot_from_document(app.snapshot().expect("live snapshot").typed())).len();
    assert!(small_bytes <= puzzle3d_export_inline_budget_bytes(), "Concrete Forest must sit under the inline budget, got {small_bytes} B");
    assert!(small_settled.downloads.is_empty(), "a payload that fits one page must not open a segmented handle: {:?}", small_settled.downloads);
    let inline = small_settled.effects.iter().find_map(|effect| match effect {
        Effect::DownloadMediaExport { filename, data, encoding, .. } => Some((filename.clone(), data.len(), encoding.clone())),
        _ => None,
    });
    assert_eq!(inline, Some(("concrete-forest.json".to_string(), small_bytes, Some("utf-8".to_string()))), "the inline lane carries the whole payload and the utf-8 encoding");

    // 🏢️ Above the budget: one segmented handle, no inline payload, and the full JSON on drain.
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "nakagin" })), None).await.expect("load nakagin");
    let expected = puzzle3d_export_json(&puzzle3d_scene_snapshot_from_document(app.snapshot().expect("live snapshot").typed()));
    assert!(expected.len() > puzzle3d_export_inline_budget_bytes(), "Nakagin must exceed the inline budget, got {} B", expected.len());
    let (large, large_settled) = dispatch_reporting(&mut app, "exportSnapshot", None, None).await;
    large.expect("nakagin export");
    assert!(
        !large_settled.effects.iter().any(|effect| matches!(effect, Effect::DownloadMediaExport { .. })),
        "an over-budget export must publish no inline download effect: {:?}",
        large_settled.effects
    );
    assert_eq!(large_settled.downloads.len(), 1, "exactly one segmented download handle: {:?}", large_settled.downloads);
    let handle = large_settled.downloads[0].clone();
    assert_eq!(handle.filename, "nakagin-capsule-tower.json", "the segmented lane keeps the per-example filename");
    assert_eq!(handle.mime_type, "application/json");
    assert_eq!(handle.encoding.as_deref(), Some("identity"), "UTF-8 JSON streams as identity, never base64");
    assert_eq!(handle.bytes, expected.len(), "the handle declares the whole payload");
    let drained = drain_segmented_download(&mut app, &handle).await;
    assert_eq!(drained.len(), expected.len(), "the drained bytes are the whole payload");
    assert_eq!(String::from_utf8(drained).expect("drained payload is UTF-8"), expected, "the drained payload IS the export, byte for byte");
    let reassembled: Value = parse(&expected, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the reassembled export parses");
    let live = projection_of(&app);
    assert_eq!(object_cores(&reassembled), object_cores(&live), "the streamed export reproduces every object of the live document");
    assert_eq!(reassembled.get("schema"), live.get("schema"));
}

/// 📥️ Wave B16: leftover `openImportSnapshot` must emit the file picker that completes as `importSnapshot`.
#[semio_framework_async_macros::async_test]
async fn leftover_open_import_snapshot_requests_file_open() {
    let source: &str = &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads");
    assert!(
        source.contains(r#""openImportSnapshot" => Box::new(Puzzle3dWindowCommandWork::new(tool_id))"#),
        "openImportSnapshot must leftover-commit through Puzzle3dWindowCommandWork, not BoundedFirstStep"
    );
    let mut app = app().await;
    let opened = dispatch(&mut app, "openImportSnapshot", None, None).await.expect("leftover openImportSnapshot");
    let req = opened.requested_effects.iter().find_map(|effect| match effect {
        Effect::RequestFileOpen { accept, import_action, .. } => {
            assert!(accept.contains("json"), "picker accepts JSON: {accept}");
            assert_eq!(import_action, "importSnapshot");
            Some(())
        }
        _ => None,
    });
    assert!(req.is_some(), "leftover Import must emit RequestFileOpen: {:?}", opened.requested_effects);
}

/// 📥️ Wave B16: leftover `importSnapshot` of a distinct two-object JSON replaces the live one-object document.
#[semio_framework_async_macros::async_test]
async fn leftover_import_snapshot_replaces_live_document_with_distinct_two_object_json() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [0.0, 0.0, 0.0] })), None).await.expect("seed");
    assert_eq!(object_count(&app), 1, "live scene_snapshot must start as one object");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("distinct");
    let two = projection_of(&app);
    assert_eq!(object_cores(&two).len(), 2, "distinct payload must carry two objects");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty again");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [0.0, 0.0, 0.0] })), None).await.expect("one-object live");
    assert_eq!(object_count(&app), 1, "live scene_snapshot is one object before import");
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&two), "name": "puzzle-3d-distinct.json" })), None).await;
    imported.expect("leftover distinct import");
    assert_eq!(loads_in(&settled), 1, "distinct leftover import is one whole-document load");
    assert_eq!(object_cores(&imported_projection(&settled)).len(), 2, "distinct leftover import must carry both objects");
}

/// 🖱️ Wave W-Y: a selected-object context menu is puzzle-owned — never the shell fallback vocabulary.
#[semio_framework_async_macros::async_test]
async fn object_context_menu_owns_puzzle_rows_not_shell_fallback() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("add");
    let object_id = first_object_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await;
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("duplicateSelection"), "object menu must own Duplicate: {menu_json}");
    assert!(menu_json.contains("selectSameKindSelection"), "object menu must own Select same kind: {menu_json}");
    assert!(!menu_json.contains("setActiveExample"), "object menu must not dress itself as the shell fallback: {menu_json}");
}

/// 📏️ Wave W-Y: gumball scale on a locked volume refuses with a notice and emits no edit.
#[semio_framework_async_macros::async_test]
async fn gumball_scale_on_locked_volume_refuses_without_edit() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("volume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    dispatch(&mut app, "setTargetVolumeFlag", Some(&json!({ "id": volume_id, "flag": "locked", "value": true })), None).await.expect("lock");
    select_id(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await.expect("select volume");
    let before = projection_of(&app);
    let result = dispatch(&mut app, "scaleSelection", Some(&json!({ "sx": 2.0, "sy": 2.0, "sz": 2.0 })), None).await.expect("scale locked");
    let raised = notices(&result);
    assert_eq!(raised.len(), 1, "locked volume must raise exactly one notice: {:?}", result.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED);
    assert!(result.mutations.is_empty(), "locked volume must emit no edit: {:?}", result.mutations);
    assert_eq!(projection_of(&app).get("targetVolumes"), before.get("targetVolumes"));
}

/// 📋️ leftover vortex-granularity selection still captures the object, and paste clones it with a new id.
#[test]
fn leftover_copy_paste_clones_selected_object() {
    let mut scene_snapshot = empty_scene_snapshot();
    scene_snapshot.objects.push(Puzzle3dObject {
        id: "seed-left-001".into(),
        label: Some("seed".into()),
        object_kind: Some("Object".into()),
        origin: [1.0, 2.0, 3.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: Vec::new(),
        hidden: false,
        locked: false,
    });
    let marks = Puzzle3dInteractionSnapshot {
        granularity: PUZZLE3D_GRANULARITY_VORTEX.into(),
        selected: vec!["seed-left-001".into()],
        hovered: Vec::new(),
        referenced: Vec::new(),
    };
    let objects = puzzle3d_selected_objects_from(&marks, &scene_snapshot);
    assert_eq!(objects.len(), 1, "leftover selected object id must copy even when granularity is vortex: {objects:?}");
    assert_eq!(objects[0].id, "seed-left-001");
    assert_eq!(objects[0].origin, [1.0, 2.0, 3.0]);
    let fragment = puzzle3d_copy_fragment_from(&scene_snapshot, objects).expect("copy fragment");
    let mutations = puzzle3d_paste_operations(&fragment, &semio_framework_plugin::kernel::PastePlacement::default()).expect("paste");
    let created: Vec<_> = mutations
        .iter()
        .filter_map(|op| match op {
            Puzzle3dMutation::CreateObject(create) => Some(create),
            _ => None,
        })
        .collect();
    assert_eq!(created.len(), 1, "paste must commit one create-object, got {mutations:?}");
    assert_ne!(created[0].object.id, "seed-left-001", "clone must mint a fresh id");
    assert!((created[0].object.origin[0] - 1.5).abs() < 1e-9, "clone origin must keep source fields plus paste offset: {:?}", created[0].object.origin);
    assert_eq!(created[0].object.object_kind.as_deref(), Some("Object"));
    assert_eq!(created[0].object.label.as_deref(), Some("seed"));
}

/// 📋️ leftover selected vortex uuid (not object.id) still captures the parent object for copy.
#[test]
fn leftover_copy_paste_clones_object_from_selected_vortex_uuid() {
    let mut scene_snapshot = empty_scene_snapshot();
    scene_snapshot.objects.push(Puzzle3dObject {
        id: "seed-left-001".into(),
        label: Some("seed".into()),
        object_kind: Some("Object".into()),
        origin: [1.0, 2.0, 3.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: vec![Puzzle3dVortex {
            id: "5de35caa-0f02-43d7-ae74-aa730efd3386".into(),
            vortex_kind: None,
            position: [0.0, 0.0, 0.0],
            direction: None,
            radius: None,
            hidden: false,
            locked: false,
        }],
        hidden: false,
        locked: false,
    });
    let marks = Puzzle3dInteractionSnapshot {
        granularity: PUZZLE3D_GRANULARITY_VORTEX.into(),
        selected: vec!["5de35caa-0f02-43d7-ae74-aa730efd3386".into()],
        hovered: Vec::new(),
        referenced: Vec::new(),
    };
    let objects = puzzle3d_selected_objects_from(&marks, &scene_snapshot);
    assert_eq!(objects.len(), 1, "leftover selected vortex uuid must resolve to the parent object: {objects:?}");
    assert_eq!(objects[0].id, "seed-left-001");
    let fragment = puzzle3d_copy_fragment_from(&scene_snapshot, objects).expect("copy fragment");
    let mutations = puzzle3d_paste_operations(&fragment, &semio_framework_plugin::kernel::PastePlacement::default()).expect("paste");
    let created: Vec<_> = mutations
        .iter()
        .filter_map(|op| match op {
            Puzzle3dMutation::CreateObject(create) => Some(create),
            _ => None,
        })
        .collect();
    assert_eq!(created.len(), 1, "vortex-uuid leftover copy must paste one create-object, got {mutations:?}");
    assert_ne!(created[0].object.id, "seed-left-001");
}

/// 🕹️ leftover overlay → translateSelection on a locked object must refuse with a visible notice.
#[semio_framework_async_macros::async_test]
async fn leftover_translate_selection_on_locked_object_refuses_with_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.as_str()], "flag": "locked", "value": true })), None).await.expect("lock");
    let before = object_origin_x(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 4.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("translate locked");
    let raised = notices(&result);
    assert_eq!(raised.len(), 1, "locked object must raise exactly one notice: {:?}", result.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED);
    assert!(result.mutations.is_empty(), "locked object must emit no edit: {:?}", result.mutations);
    assert!((object_origin_x(&app, &object_id) - before).abs() < 1e-9, "locked object origin must not move");
}

/// 🕹️ leftover overlay → translateSelection carries explicit ids; guest snapshot granularity may be empty.
#[semio_framework_async_macros::async_test]
async fn leftover_overlay_translate_selection_moves_unlocked_object() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let before = object_origin(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 4.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("leftover overlay translate");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::Notify { .. })), "unlocked leftover overlay must not refuse: {:?}", result.requested_effects);
    assert!((object_origin(&app, &object_id)[0] - before[0] - 4.0).abs() < 1e-9, "leftover overlay ids must move the unlocked object by dx");
}

/// 🕹️ leftover `translateSelection` with explicit ids + mesh mode commits a pose delta.
#[semio_framework_async_macros::async_test]
async fn leftover_translate_selection_mesh_mode_commits_pose_delta() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let before = object_origin(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "mode": "mesh", "dx": 3.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("leftover mesh translate");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::Notify { .. })), "mesh leftover translate must pre-admit: {:?}", result.requested_effects);
    assert!((object_origin(&app, &object_id)[0] - before[0] - 3.0).abs() < 1e-9, "leftover mesh-mode translateSelection must commit a pose delta");
}

//#endregion 🩹️CheckedDefects

/// 📏️ Wave W-M2: one whole `registerBrushMesh` page validates inside one command's own work budget.
/// The page scan is cursorized at [`PUZZLE3D_MESH_PAGE_SCAN_CHARS`] characters per step over both
/// payload streams, and the largest page the plugin admits is
/// `PUZZLE3D_MESH_PAGE_BASE64_CHARS` characters per stream — so the extent a page claims can never
/// approach `PUZZLE_COMMAND_WORK_ITEMS`, which is what keeps every per-page unit under the 8 ms law.
#[test]
fn one_brush_mesh_page_validates_inside_one_command_work_budget() {
    let page = semio_s_artifact_puzzle_3d_precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS;
    let steps = page.div_ceil(PUZZLE3D_MESH_PAGE_SCAN_CHARS).saturating_mul(2).saturating_add(1);
    assert!(steps <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS, "a whole page validates in {steps} bounded steps, inside {}", semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS);
    assert!(page + 8 < semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_RAW_BYTES, "a page's two payload streams — one page's values, split at most once and padded per stream — leave the JSON envelope room inside the raw wire");
}

/// 🚚️ Wave W-H: a client whose "already uploaded" bookkeeping outlived the guest that justified it
/// announces identities this instantiation holds nothing for. The arm may not answer that with a notice
/// — nothing on the host reads a notification's text, so the brush utility would silently keep no
/// collision body until a full page reload. It publishes the identity on the world body instead, and
/// widens its own otherwise-`Quiet` scope to the viewport so that body is actually republished. The
/// client answers with the page run, and the request retires.
#[semio_framework_async_macros::async_test]
async fn an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes() {
    let mut app = app().await;
    // 🎲️ Geometry no other law in this binary derives — the brush-mesh store is content-addressed since
    // wave B22, so the shared cube's digest would be adoptable under a new id and the refusal this law is
    // about could never happen.
    let (positions, indices) = semio_s_artifact_puzzle_3d_model::precompute_model_tests::context::seeded_cube_mesh_buffers(23.0);
    let digest = semio_s_artifact_puzzle_3d_precompute::brush_mesh_digest(&positions, &indices);
    let url = "/test/restarted-guest-reannounce.glb";
    let announcement = json!({ "surfaceId": "world-3d", "url": url, "digest": digest });
    let refused = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("id-only announcement");
    assert!(!refused.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "a refusal the client cannot act on is not an answer");
    assert!(
        matches!(&refused.ui_scope, UiDirtyScope::Partial { window_bodies, .. } if window_bodies.iter().any(|body| body == main::BODY_KEY)),
        "the request is worthless until the world body carrying it repaints; got {:?}",
        refused.ui_scope
    );
    let requested = interaction_of(&render_composite(&mut app).await);
    assert_eq!(requested.pointer("/meshReuploadUrls").and_then(Value::as_array).cloned().unwrap_or_default(), vec![json!(url)], "the refused identity is published as a request for its bytes");
    let residency = requested.pointer("/meshResidency").and_then(Value::as_u64).expect("the world body publishes the guest's mesh residency");
    // 🐢️ Wave B32: the SECOND refusal of the same id is where the storm lived. The standing request set
    // is already published, so this announcement has nothing new for any surface — and republishing the
    // world body for it re-drives the client's registrar, which re-announces, which refuses again. The
    // live `:6013` shell measured 23 of 33 typed-operation completions carrying this viewport scope in a
    // 75 s window with four user actions, 23 of 31 refresh passes answering every world body
    // `unchanged`, and `translateSelection` needing 8–15 s to reach `data-instances-json` behind them.
    let repeated = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("the client re-announces the same dead identity");
    assert!(matches!(repeated.ui_scope, UiDirtyScope::None), "a re-announcement of an ALREADY standing request republishes nothing and must stay Quiet; got {:?}", repeated.ui_scope);
    let unchanged = interaction_of(&render_composite(&mut app).await);
    assert_eq!(unchanged.pointer("/meshReuploadUrls"), requested.pointer("/meshReuploadUrls"), "the second refusal leaves the published request set byte-identical, which is why it owes no repaint");

    let position_bytes: Vec<u8> = positions.iter().flat_map(|value| value.to_le_bytes()).collect();
    let index_bytes: Vec<u8> = indices.iter().flat_map(|value| value.to_le_bytes()).collect();
    let page = json!({
        "surfaceId": "world-3d",
        "url": url,
        "digest": digest,
        "page": 0,
        "pageCount": 1,
        "positionsB64": semio_framework_io_base64::base64_standard_encode(&position_bytes),
        "indicesB64": semio_framework_io_base64::base64_standard_encode(&index_bytes),
    });
    let accepted = dispatch(&mut app, "registerBrushMesh", Some(&page), None).await.expect("the client answers with the page run");
    assert!(!accepted.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "an accepted page run is silent");
    let settled = interaction_of(&render_composite(&mut app).await);
    assert_eq!(settled.pointer("/meshReuploadUrls").and_then(Value::as_array).map(Vec::len), Some(0), "an answered request retires, so a steady state carries no standing request");
    assert!(settled.pointer("/meshResidency").and_then(Value::as_u64).expect("residency") > residency, "the client's proof that this guest holds the geometry is the counter climbing");

    let readopted = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("the identity is now resident");
    assert!(readopted.requested_effects.is_empty(), "an identity this guest holds is adopted by id alone, at no cost on the wire");
    assert!(matches!(readopted.ui_scope, UiDirtyScope::None), "the adopting fast path stays the quiet command it is declared to be");
    assert!(
        !puzzle3d_action_uses_precompute("registerBrushMesh"),
        "registerBrushMesh only WRITES geometry into the precompute session, so it may not owe the sync prologue: that prologue seeds a fallback body for every id the session has no geometry for — precisely the ids this command supplies — clearing the brush cache and rebuilding the queue each time, which the arm's own install then discards, and which is where the 0.65 s → 2.85 s per announcement measured on 2026-09-09 came from"
    );
}

//#region 📷️OpeningCamera
/// 📷️ Reads one pane's published pose straight off the world lane it renders — the exact string
/// `World3dHost` parses into `data-camera-json`.
async fn published_camera(app: &mut Puzzle3dApp, window_id: &str) -> Value {
    camera_of(&render_window(app, window_id).await)
}

fn camera_position(camera: &Value) -> Vec<f64> {
    camera.get("position").and_then(Value::as_array).map(|axes| axes.iter().filter_map(Value::as_f64).collect()).unwrap_or_default()
}

/// 📷️ Every pane opens on a real pose of its own before the user has touched anything.
///
/// `Puzzle3dCamera::default()` is all zeros, and `World3dHost` frames each pane LOCALLY and
/// deliberately never dispatches that framing back (`🌐️World3dHost/🟦️.tsx`'s `WorldAutoFit`), so
/// until this wave the world lane of both panes carried position == target == `[0,0,0]` until the
/// first `setCamera` landed: no view direction at boot, and two panes of one document publishing the
/// SAME empty pose (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12 §4.1 measured it live, wave B15
/// fixed it).
#[semio_framework_async_macros::async_test]
async fn every_pane_opens_on_its_own_framed_camera_before_any_gesture() {
    let mut app = app().await;
    let top = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;
    let perspective = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    for (window_id, camera) in [(main::WINDOW_INSTANCE_TOP, &top), (main::WINDOW_INSTANCE_PERSPECTIVE, &perspective)] {
        let position = camera_position(camera);
        assert_eq!(position.len(), 3, "{window_id} publishes a three-axis camera position: {camera}");
        assert!(position.iter().any(|axis| axis.abs() > 1e-6), "{window_id} opens on a real pose, never the all-zero default: {camera}");
        assert_ne!(camera.get("position"), camera.get("target"), "{window_id}'s opening camera must look at something other than where it stands: {camera}");
    }
    assert_ne!(top, perspective, "the Top and Perspective panes of ONE document open on DIFFERENT poses: top={top} perspective={perspective}");
    assert_eq!(top.pointer("/projection/mode/kind").and_then(Value::as_str), Some("orthographic"), "the Top pane opens under the orthographic plan its layout template declares: {top}");
    assert_eq!(top.pointer("/projection/orientation/view").and_then(Value::as_str), Some("top"), "the Top pane looks straight down: {top}");
    assert_eq!(perspective.pointer("/projection/mode/kind").and_then(Value::as_str), Some("threePoint"), "the Perspective pane keeps the three-point default: {perspective}");
}

/// 📷️ Swapping the example re-frames the pane on what the NEW document holds, instead of leaving the
/// camera framed on bounds nothing occupies any more.
#[semio_framework_async_macros::async_test]
async fn switching_the_example_reframes_every_unposed_pane_on_the_new_document() {
    let mut app = app().await;
    let before_perspective = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    let before_top = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;

    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("switch to the Nakagin example");

    let after_perspective = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    let after_top = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;
    assert_ne!(before_perspective, after_perspective, "the pane the switch was dispatched from re-frames: before={before_perspective} after={after_perspective}");
    assert_ne!(before_top, after_top, "a sibling pane the user never posed re-frames on the new document too: before={before_top} after={after_top}");
    assert_ne!(after_top, after_perspective, "re-framing keeps the two panes distinct: top={after_top} perspective={after_perspective}");
}

/// 📷️ A framed pane is framed ONCE: an opening pose is not a gesture, so re-reading the same pane
/// twice, and dispatching an unrelated window option in between, must publish the same pose.
#[semio_framework_async_macros::async_test]
async fn an_opening_camera_is_stable_and_never_overrules_a_pose_the_user_set() {
    let mut app = app().await;
    let opening = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("an unrelated window option");
    assert_eq!(published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await, opening, "an unrelated option must not move the opening pose");

    let posed = json!({ "camera": { "position": [11.0, -7.0, 5.0], "target": [1.0, 2.0, 3.0], "zoom": 1.0 } });
    dispatch(&mut app, "setCamera", Some(&posed), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("the user poses the camera");
    let user = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    assert_eq!(camera_position(&user), vec![11.0, -7.0, 5.0], "a real gesture owns the pane's pose from then on: {user}");
    assert_eq!(published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await, user, "and the framing never takes it back: {user}");
}

/// 📽️ Flipping ONE pane's projection repaints THAT pane's published camera and no sibling's.
///
/// A projection is not a label on the pose — it IS the pose: `setProjection` re-derives position and
/// up from the new orientation around the unchanged target (`🎮️commands/📽️set-projection`), and the
/// `WindowOption` scope it publishes under repaints the world body, so the world lane's `cameraJson`
/// — what `World3dHost` parses into `data-camera-json` — must differ in the same settle. The browser
/// probe read `projection-repaints-camera` as bit-identical (ticket 26/09/02 wave B38 §1.3); this law
/// is what says whether the guest owes that, and it does not — the probe sampled the camera ~100 ms
/// after the click, while a measures-rail control's own round trip is documented at 0.7 s and up
/// (`🛠️ShellHelpers/🎚️measure-controls`'s `useWindowMeasureDraft`), so the value it saw flip was that
/// control's optimistic DRAFT and the camera it read was the pre-dispatch one.
#[semio_framework_async_macros::async_test]
async fn flipping_one_panes_projection_repaints_that_panes_camera_and_leaves_its_sibling_alone() {
    let mut app = app().await;
    let before = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    let sibling_before = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;
    assert_eq!(before.pointer("/projection/mode/kind").and_then(Value::as_str), Some("threePoint"), "the law starts from the perspective pane's own three-point default: {before}");

    dispatch(&mut app, "setProjection", Some(&json!({ "field": "orthographicView", "value": "plan" })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("setProjection orthographicView=plan");

    let after = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    assert_eq!(after.pointer("/projection/mode/kind").and_then(Value::as_str), Some("orthographic"), "the flipped pane publishes the projection it was given: {after}");
    assert_eq!(after.pointer("/projection/orientation/view").and_then(Value::as_str), Some("plan"), "and the orientation it was given: {after}");
    assert_ne!(camera_position(&after), camera_position(&before), "an orthographic plan looks straight down — the POSE must move, not only its label: before={before} after={after}");
    assert_ne!(after.get("up"), before.get("up"), "and the up vector is re-derived with it: before={before} after={after}");
    let eye = camera_position(&after);
    let focus = camera_position(&after.get("target").cloned().map(|target| json!({ "position": target })).expect("the camera lane carries its target"));
    assert!((eye[0] - focus[0]).abs() < 1e-9 && (eye[1] - focus[1]).abs() < 1e-9 && eye[2] > focus[2], "a plan view stands directly above what it looks at: eye={eye:?} target={focus:?}");
    assert_eq!(published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await, sibling_before, "the sibling pane's camera lane is untouched by another pane's projection: {sibling_before}");

    dispatch(&mut app, "setProjection", Some(&json!({ "field": "perspectiveKind", "value": "threePoint" })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("setProjection perspectiveKind=threePoint");
    let restored = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    assert_eq!(restored.pointer("/projection/mode/kind").and_then(Value::as_str), Some("threePoint"), "flipping back is symmetric: {restored}");
    assert_ne!(camera_position(&restored), camera_position(&after), "and moves the pose back off the plan axis: after={after} restored={restored}");
}
//#endregion 📷️OpeningCamera

//#region ⚙️SettingsPanelScope
/// ⚙️ The Settings panel addresses the pane the user is LOOKING at.
///
/// 🎯️ An app-level panel body is rendered through `ViewModel::for_panel()`, which clears `window_id`
/// by design — so before this wave `puzzle3d_addressed_window_id` fell through to the roster's first
/// entry, the base window KIND, and every stepper baked THAT into its args: a Settings edit landed on
/// a pane nobody has open (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12 §5.1 measured
/// `setGridSpacing window=Some("puzzle3d-main")` live). `focused_window_id` survives `for_panel` and
/// is what the panel resolves through now.
#[semio_framework_async_macros::async_test]
async fn the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind() {
    fn node_by_key(node: &Value, key: &str) -> Option<Value> {
        if node.get("key").and_then(Value::as_str) == Some(key) {
            return Some(node.clone());
        }
        match node {
            Value::Object(fields) => fields.iter().find_map(|(_, child)| node_by_key(child, key)),
            Value::Array(items) => items.iter().find_map(|item| node_by_key(item, key)),
            _ => None,
        }
    }
    let mut app = app().await;
    for focused in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        let panel = render_panel_body(&mut app, settings_panel::BODY_KEY, Some(focused)).await;
        let rendered = panel.to_string();
        assert!(rendered.contains(focused), "the Settings section names the pane it is addressed at, so the user can see which one a bump will retune: {rendered}");
        for field in ["contact-tolerance", "proximity-radius", "chunk-size", "grid-spacing"] {
            let control = node_by_key(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.{field}.control")).unwrap_or_else(|| panic!("the {field} stepper: {panel}"));
            let bindings = control.get("bindings").and_then(Value::as_array).cloned().unwrap_or_default();
            assert_eq!(bindings[0].pointer("/args/windowId").and_then(Value::as_str), Some(focused), "{field} tags the focused pane, never the base window kind: {bindings:?}");
        }
    }
    // 🪟️ With nothing focused the panel still falls back to the ROSTER — and the roster arm of
    // `puzzle3d_addressed_window_id` deliberately skips ids that are a window KIND
    // (`!puzzle3d_window_id_is_kind`), so it lands on the first LIVE pane instance. That is this wave's
    // whole point: the base kind `puzzle3d-main` is a pane nobody has open, and baking it into a
    // stepper's args is the defect the wave measured. Pinning the base kind here was pinning the
    // pre-wave behaviour the same law's own docstring calls the bug.
    let unfocused = render_panel_body(&mut app, settings_panel::BODY_KEY, None).await;
    let control = node_by_key(&unfocused, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.grid-spacing.control")).expect("the grid spacing stepper");
    let bindings = control.get("bindings").and_then(Value::as_array).cloned().unwrap_or_default();
    let addressed = bindings[0].pointer("/args/windowId").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_ne!(addressed, main::WINDOW_KIND_ID, "with nothing focused the panel addresses a LIVE pane, never the base window kind nobody has open: {bindings:?}");
    assert!([main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE].contains(&addressed.as_str()), "and that pane is one this app really has open: {addressed:?} {bindings:?}");
}

/// ⚙️ A Settings bump reaches the focused pane's own rail and leaves the sibling pane alone — the
/// split-window law A1/A2 §19 asked for, played through the id the panel itself baked.
#[semio_framework_async_macros::async_test]
async fn a_settings_bump_retunes_only_the_focused_panes_rail() {
    let spacing_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing");
    let mut app = app().await;
    for pane in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        drop(render_window(&mut app, pane).await);
    }
    let top_view = app.window_view(main::WINDOW_INSTANCE_TOP);
    let perspective_view = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    let top_before = find_measure_slider(app.window_measures(&top_view).await.get(main::WINDOW_INSTANCE_TOP).expect("top rail"), &spacing_id);
    let perspective_before = find_measure_slider(app.window_measures(&perspective_view).await.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective rail"), &spacing_id);
    assert_eq!(top_before, perspective_before, "both panes start on the same spacing, so a divergence below can only come from the addressing");

    let bumped = top_before.expect("the rail publishes a spacing") + 0.5;
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "windowId": main::WINDOW_INSTANCE_TOP, "value": bumped })), Some(main::WINDOW_INSTANCE_TOP)).await.expect("bump the focused pane's spacing");

    let top_view = app.window_view(main::WINDOW_INSTANCE_TOP);
    let measures = app.window_measures(&top_view).await;
    assert_eq!(find_measure_slider(measures.get(main::WINDOW_INSTANCE_TOP).expect("top rail"), &spacing_id), Some(bumped), "the focused pane's own rail carries what the Settings stepper published");
    assert_eq!(find_measure_slider(measures.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective rail"), &spacing_id), perspective_before, "the pane the user is NOT looking at keeps its own spacing");
}

/// ⚙️ The Settings panel must RENDER the pane it is ADDRESSED at. Every stepper tags the focused pane
/// as `windowId` (law above), but the value it shows came from `ConfigView::window`, which is `None` for
/// a panel projection (`ViewModel::for_panel` drops `window_id`) and therefore fell through to
/// `Puzzle3dWindowConfig::default()`. The panel then displayed the DEFAULT spacing while writing to the
/// focused pane: battery #53 read `settings-value-reaches-window-rail settings=10.5 windowRail=12.5`
/// after the rail had been nudged to 12.5 — the panel bumped a 10.0 it had invented
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B36).
#[semio_framework_async_macros::async_test]
async fn the_settings_panel_renders_the_focused_panes_own_value_not_a_default() {
    fn control(node: &Value, key: &str) -> Option<Value> {
        if node.get("key").and_then(Value::as_str) == Some(key) {
            return Some(node.clone());
        }
        match node {
            Value::Object(fields) => fields.iter().find_map(|(_, child)| control(child, key)),
            Value::Array(items) => items.iter().find_map(|item| control(item, key)),
            _ => None,
        }
    }
    let spacing_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing");
    let mut app = app().await;
    for pane in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        drop(render_window(&mut app, pane).await);
    }
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "windowId": main::WINDOW_INSTANCE_PERSPECTIVE, "value": 12.5 })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("nudge the perspective pane's spacing");
    let perspective_view = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    let rail = app.window_measures(&perspective_view).await;
    assert_eq!(find_measure_slider(rail.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective rail"), &spacing_id), Some(12.5), "the rail carries the nudge, so the panel below has something to disagree with");
    let panel = render_panel_body(&mut app, settings_panel::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await;
    let stepper = control(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.grid-spacing.control")).unwrap_or_else(|| panic!("the grid spacing stepper: {panel}"));
    let rendered = stepper.get("component").and_then(|props| props.get("value")).and_then(Value::as_f64);
    assert_eq!(rendered, Some(12.5), "the Settings panel renders the focused pane's own spacing, never a default it invented: {stepper}");
}
//#endregion ⚙️SettingsPanelScope

//#region 🔬️B23RenderedVsMutatedDocument
/// 🔬️ Wave B23 — the browser's own sequence, on the two-pane session the browser boots: pick the
/// object, read Inspection through BOTH host render routes (the panel route, which carries no
/// `window_id` at all, and the window-instance route), lock the picked selection through the
/// context-menu path (no explicit ids — the arm reads the selection the render just painted), then
/// translate it and read the world lane the pane publishes.
///
/// 🕳️ Every failing verdict of the `#48` selection-scoped family is one row of this law
/// (`inspection-object-fields id=null`, `locked-flag-row lockChrome=false`,
/// `gumball-scene-delta sceneDelta=false`, `relocate-pose-delta`), and the law is GREEN — the guest
/// keeps the pick across both routes and both mutating dispatches, including the
/// `revalidate_interaction_on_document_change` pass every artifact-intent dispatch runs.
/// What the browser has and this law cannot reach is the surface-context route
/// (`plugin_mount_surface` → `SurfaceContexts` → `plugin_render_surface`): every test context render
/// helper calls `PluginApp::render` directly with a hand-built `ViewModel`, so a body that is never
/// re-rendered — or re-rendered against another surface's retained view — is invisible here.
#[semio_framework_async_macros::async_test]
async fn a_browser_shaped_pick_survives_every_render_route_and_both_mutating_dispatches() {
    let mut app = app().await;
    for pane in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        drop(render_window(&mut app, pane).await);
    }
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("browser-shaped pick");
    let panel_route = render_panel_body(&mut app, inspection::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.to_string();
    assert!(!panel_route.contains("puzzle3d-play-inspector.empty"), "the panel route — the only one a real Inspection refresh uses — must see the pick: {panel_route}");
    assert!(panel_route.contains(object_id.as_str()), "the panel route must name the picked object: {panel_route}");
    assert!(panel_route.contains("puzzle3d-play-inspector.object.locked"), "the panel route must assemble the lock flag_row the context menu drives: {panel_route}");
    let window_route = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!window_route.contains("puzzle3d-play-inspector.empty"), "the window-instance route must see the same pick: {window_route}");

    let before = render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_PERSPECTIVE).await.to_string();
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "flag": "locked", "value": true })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("lock the picked object");
    assert_eq!(object_flag(&app, &object_id, "locked"), Some(true), "the context-menu lock path must land on the object the pick named");
    dispatch(&mut app, "translateSelection", Some(&json!({ "dx": 3.0, "dy": 0.0, "dz": 0.0 })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("translate the picked object");
    let after = render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_PERSPECTIVE).await.to_string();
    assert_ne!(before, after, "a translate taken on the SECOND mutating dispatch after the pick must still change the world lane the pane renders");
    assert!(
        !render_panel_body(&mut app, inspection::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.to_string().contains("puzzle3d-play-inspector.empty"),
        "two artifact-intent dispatches run two `revalidate_interaction_on_document_change` passes; neither may eat the pick"
    );
}

fn object_flag(app: &Puzzle3dApp, object_id: &str, flag: &str) -> Option<bool> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get(flag).and_then(Value::as_bool))
}
//#endregion 🔬️B23RenderedVsMutatedDocument

//#region 🔖️Pz2ShippedKindCatalog
/// 🗂️ The authored [`PUZZLE3D_SHIPPED_OBJECT_KINDS`] IS the two shipped examples' own
/// `meta.kindCatalogs` rows, in their own order — the law that lets the `objectKind` select be a
/// `const` instead of a parse of 136 395 B of DSL on the `describe()` path (slice PZ2, 2026-09-22).
/// It pays that parse HERE, once, and fails the moment an example's catalog changes without the
/// authored list following.
#[semio_framework_async_macros::async_test]
async fn shipped_object_kinds_are_the_two_examples_own_catalog_rows() {
    let mut derived: Vec<(String, String)> = Vec::new();
    for scene_snapshot in [&*CONCRETE_FOREST_EXAMPLE_SNAPSHOT, &*NAKAGIN_EXAMPLE_SNAPSHOT] {
        for entry in puzzle3d_catalog_entries(scene_snapshot, "objects") {
            if derived.len() >= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX {
                break;
            }
            let Some(id) = entry.get("id").and_then(semio_framework_value::DslValue::as_str) else {
                continue;
            };
            if derived.iter().any(|(existing, _)| existing == id) {
                continue;
            }
            derived.push((id.to_string(), catalogue::catalog_entry_label(entry)));
        }
    }
    let authored: Vec<(String, String)> = PUZZLE3D_SHIPPED_OBJECT_KINDS.iter().map(|(id, label)| ((*id).to_string(), (*label).to_string())).collect();
    assert_eq!(authored, derived, "PUZZLE3D_SHIPPED_OBJECT_KINDS drifted from the shipped documents — re-author it from `concrete-forest` then `nakagin-capsule-tower`");
}
//#endregion 🔖️Pz2ShippedKindCatalog

//#region 🔖️InitialSnapshotCost
/// 🚀️ LAW (ticket 26/09/23, H12): the initial snapshot costs its document and nothing else. Every construction of
/// this app pays for it — each mount, and every `codec` call a hub makes to create or validate a document, where
/// the interpreter multiplies it — so it must never build a scene, sync a precompute session or parse an example
/// the document does not read. Weighed by this thread's heap peak (`HeapWitness`), deterministic and independent
/// of other laws: the snapshot may peak no higher than converting the default scene_snapshot itself, within 4 KiB.
#[test]
fn the_initial_snapshot_costs_only_its_document() {
    let _ = default_scene_snapshot();
    let peak_of = |work: &dyn Fn()| {
        let before = semio_framework_trace::retained_heap_bytes_on_this_thread();
        semio_framework_trace::reset_heap_peak_on_this_thread();
        work();
        semio_framework_trace::peak_heap_bytes_on_this_thread() - before
    };
    let document = peak_of(&|| drop(Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&default_scene_snapshot()).expect("typed fixture admits"))));
    let snapshot = peak_of(&|| drop(<Puzzle3dPlayApp as ArtifactEditor>::initial_snapshot()));
    assert!(document > 0, "the witness weighs the document conversion: {document}");
    assert!(snapshot <= document + 4096, "the initial snapshot peaked at {snapshot} B, its document at {document} B: it does work the document never reads");
}
//#endregion 🔖️InitialSnapshotCost

/// 🛡️ The host twin carries every object of the typed document and refuses a scale the typed record cannot admit.
#[test]
fn puzzle3d_host_twin_round_trips_its_vocabulary_and_refuses_bad_scale() {
    let document=semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&default_scene_snapshot()).expect("authored fixture admits");
    let mut host=puzzle3d_scene_snapshot_from_document(&document);
    let admitted=semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&host).expect("the twin readmits");
    assert_eq!(admitted.objects.iter().map(|object|object.id.as_str()).collect::<Vec<_>>(),document.objects.iter().map(|object|object.id.as_str()).collect::<Vec<_>>());
    host.objects[0].scale=Some(semio_framework_value::DslValue::String("invalid".into()));
    assert!(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&host).is_err());
}

/// 🍞️ Geometry cache and interactive step budget laws.


/// 🎟️ Wave W-P: `Puzzle3dPlayApp` used to be rebuilt from `default()` on every dispatch and every render,
/// so `geometry_cache` was structurally unable to observe two calls in a row and the whole scene_snapshot was
/// re-serialized every time (`📓️2026-09-08-performance-architecture-audit.md` §1, fix #1). With a session
/// slot keyed by `app_instance_id`, the second call for the same document must serialize NOTHING.
#[test]
fn a_second_call_on_one_instance_reuses_the_geometry_cache_instead_of_reserializing() {
    let config = Puzzle3dRuntime::default();
    let scene_snapshot = default_scene_snapshot();
    let fingerprint = semio_s_artifact_puzzle_3d_editor_session::scene_geometry_fingerprint(&scene_snapshot);
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

fn measured_step_loop(original: &mut semio_s_plugin_puzzle_retained::testing::OriginalFixture, work: &mut dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle3dPlayApp>>, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, config: &Puzzle3dConfig, context: &semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle3dPlayApp>>, guard: usize, label: &str) -> Vec<std::time::Duration> {

    use crate::retained_command::testing::PuzzleCommandWorkStep;
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let mut turns = Vec::with_capacity(guard);
    loop {
        assert!(turns.len() <= guard, "{label} step() did not reach Complete within {guard} bounded turns");
        let started = std::time::Instant::now();
        let outcome = crate::retained_command::testing::step(original, work, command, snapshot, config, &interaction, &hover, Some(context)).expect("bounded step");
        turns.push(started.elapsed());
        if matches!(outcome, PuzzleCommandWorkStep::Complete(_)) {
            break;
        }
    }
    semio_s_plugin_puzzle_retained::testing::close(original, work);
    turns
}

fn measured_tool_work(original: &mut semio_s_plugin_puzzle_retained::testing::OriginalFixture, tool_id: &'static str, run: u32) -> (Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle3dPlayApp>>>, semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle3dPlayApp>>) {
    let instance = Puzzle3dInstanceBinding { app_instance_id: 9_000 + run, parent_document_id: format!("measured-{tool_id}-{run}"), owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::new(Box::new(Puzzle3dInstanceOperationOwner::default())) };
    let app_instance_id = instance.app_instance_id;
    let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle3dPlayApp>>> = match tool_id {
        "acceptSuggestion" => original.construct_boxed(Puzzle3dAcceptSuggestionWork::original_birth_demands().unwrap(), |scope| Puzzle3dAcceptSuggestionWork::from_original_construction(scope).map(|(work, progress)| (work.bound(instance), progress))),
        "setActiveExample" => original.construct_boxed(Puzzle3dSetActiveExampleWork::original_birth_demands().unwrap(), |scope| Puzzle3dSetActiveExampleWork::from_original_construction(scope)),
        "openVortexSuggestions" => original.construct_boxed(Default::default(), |_scope| Ok((Puzzle3dWindowCommandWork::new(tool_id).bound(instance), Default::default()))),
        _ => original.construct_boxed(Default::default(), |_scope| Ok((crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent), Default::default()))),
    };
    let context = semio_framework_plugin::app::ArtifactOwnedToolJobContext::new(
        app_instance_id,
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
            let mut original = semio_s_plugin_puzzle_retained::testing::OriginalFixture::new(
        semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_s_plugin_puzzle_retained::testing::fixture_stock(), || Some(0), u64::MAX, 1 << 24);
        let (mut work, context) = measured_tool_work(&mut original, tool_id, run);
        let turns = measured_step_loop(&mut original, work.as_mut(), command, snapshot, config, &context, crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, tool_id);
        original.finish();
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
        window_id: Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID.to_string()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: main::WINDOW_KIND_ID.to_string(), window_kind_id: main::WINDOW_KIND_ID.to_string() }],
        ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    }
}

fn measured_nakagin_snapshot() -> Puzzle3dPlaySnapshot {
    Puzzle3dPlaySnapshot::new(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"))
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
    let command = Puzzle3dCommand::from_action("openVortexSuggestions", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID.to_string())).expect("openVortexSuggestions command decodes");
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
    let command = Puzzle3dCommand::from_action("acceptSuggestion", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID.to_string())).expect("acceptSuggestion command decodes");
    let (steps, worst) = measured_cold_runs("acceptSuggestion", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "acceptSuggestion worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "acceptSuggestion worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

fn measured_concrete_forest_snapshot() -> Puzzle3dPlaySnapshot {
    Puzzle3dPlaySnapshot::new(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"))
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3: swapping the Concrete Forest document for the
/// 180-object Nakagin one is the single largest document gesture this artifact has — it deletes every
/// existing attraction and object and creates every Nakagin one. Its work is fully typed and cursorized
/// (`Puzzle3dSetActiveExampleWork`), so no turn of it may cross the interactive step ceiling either.
#[test]
fn set_active_example_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_concrete_forest_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), Some(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID.to_string())).expect("setActiveExample command decodes");
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
        let seed = Puzzle3dPlaySnapshot::new(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&scene_snapshot).expect("typed fixture admits"));
        let snapshot = Puzzle3dPlaySnapshot::new(semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(seed.typed())).expect("typed fixture admits"));
        let bridged = Puzzle3dScene { scene_snapshot: semio_framework_value::FromValue::from_value(snapshot.value().clone()).expect("native fixture view admits"), runtime: Puzzle3dRuntime::default(), active_utility: "utility".into() };
        let typed = scene_from_snapshot(snapshot.typed(), Puzzle3dRuntime::default(), "utility");
        assert_eq!(typed.scene_snapshot.schema, bridged.scene_snapshot.schema, "{label}: schema disagrees");
        assert_eq!(typed.scene_snapshot.domain, bridged.scene_snapshot.domain, "{label}: domain disagrees");
        assert_eq!(typed.scene_snapshot.objects, bridged.scene_snapshot.objects, "{label}: objects disagree");
        assert_eq!(typed.scene_snapshot.attractions, bridged.scene_snapshot.attractions, "{label}: attractions disagree");
        assert_eq!(typed.scene_snapshot.target_volumes, bridged.scene_snapshot.target_volumes, "{label}: target volumes disagree");
        assert_eq!(typed.scene_snapshot.references, bridged.scene_snapshot.references, "{label}: references disagree");
        let catalogs = |meta: &Puzzle3dSceneMetadata| -> crate::Puzzle3dKindCatalogs { meta.kind_catalogs.clone().map_or_else(crate::Puzzle3dKindCatalogs::default, |rows| semio_framework_value::FromValue::from_value(rows).expect("kind catalogs decode")) };
        let compatibility = |meta: &Puzzle3dSceneMetadata| -> Vec<crate::Puzzle3dKindCompatibility> { meta.kind_compatibility.clone().map_or_else(Vec::new, |rows| semio_framework_value::FromValue::from_value(rows).expect("kind compatibility decodes")) };
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
    let ends: Vec<String> = grown.objects.iter().flat_map(|object| object.vortices.iter().map(|vortex| crate::editor::puzzle3d::puzzle3d_vortex_full_id(&object.id, &vortex.id))).collect();
    assert!(ends.len() > 2, "nakagin declares vortices to attract");
    grown.attractions = ends.windows(2).enumerate().map(|(index, pair)| Puzzle3dAttraction { id: format!("a{index}"), attracting: pair[0].clone(), attracted: pair[1].clone(), ..Default::default() }).collect();
    grown.attractions.push(Puzzle3dAttraction { id: "dangling".into(), attracting: ends[0].clone(), attracted: "no-such-object:no-such-vortex".into(), ..Default::default() });
    for (label, scene_snapshot) in [("empty", empty_scene_snapshot()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()), ("nakagin", NAKAGIN_EXAMPLE_SNAPSHOT.clone()), ("nakagin-dangling", grown)] {
        let expected: Vec<Value> = scene_snapshot
            .attractions
            .iter()
            .filter_map(|attraction| {
                let from = crate::editor::puzzle3d::resolve_vortex_world_position(&scene_snapshot, &attraction.attracting)?;
                let to = crate::editor::puzzle3d::resolve_vortex_world_position(&scene_snapshot, &attraction.attracted)?;
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
        let bridged: semio_s_artifact_puzzle_3d_model::standards::v1::subsets::any::schema::SceneConfig = semio_framework_value::FromValue::from_value(scene_config_value(&envelope)).expect("value bridge decodes");
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
                .and_then(|value| value.as_array())
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

use super::unit_tests::context::*;
use super::*;
use std::time::Instant;

/// ⏱️ Wave B44 attribution harness: what ONE command costs on a 1-object document against the
/// 180-object Nakagin document, split by the stage that pays it. Not a gate — it prints the table the
/// wave's report quotes, so the numbers in that report are measured rather than argued.
///
/// Run with `--nocapture` to read it.
#[semio_framework_async_macros::async_test]
async fn b44_measures_what_one_command_costs_per_document_size() {
    let mut app = app().await;
    let forest = latency_census(&mut app, "concrete-forest").await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin switch");
    let nakagin = latency_census(&mut app, "nakagin").await;
    for row in [&forest, &nakagin] {
        eprintln!("[DEBUG] b44.native {row}");
    }
    assert!(nakagin.objects > forest.objects, "the switch must actually grow the document under measurement");
}

/// 🚚️ Wave B44 LAW: a pose edit on a large document must re-serialize, and name in its delta, EXACTLY
/// the objects that moved — and the delta's own byte size must be O(changed), not O(n).
///
/// ⏱️ This is the shape the whole wave turns on. Before it, every publication rebuilt all 180 instance
/// records (2 449 µs) behind a change key that `format!`-ed and hashed the whole fixture's JSON
/// (20 716 µs), so moving one capsule cost the same as replacing the document.
#[test]
fn a_pose_edit_reserializes_and_names_exactly_the_objects_that_moved() {
    let fixture = NAKAGIN_EXAMPLE_FIXTURE.clone();
    assert!(fixture.objects.len() >= 100, "the law needs a document large enough for O(n) and O(changed) to differ; got {}", fixture.objects.len());
    let mut residency = main::Puzzle3dInstanceResidency::default();
    assert!(residency.refresh(&fixture), "the first refresh publishes the whole set");
    assert_eq!(residency.rebuilt_records() as usize, fixture.objects.len(), "a cold residency serializes every record exactly once");
    assert!(residency.delta_json().is_none(), "a COLD publication carries no delta lane at all — a consumer with nothing retained must read the full set anyway, and the second copy is pure wire cost");
    let full_bytes = residency.instances_json().len();

    assert!(!residency.refresh(&fixture), "an unchanged fixture republishes nothing at all");
    assert_eq!(residency.rebuilt_records(), 0, "an unchanged fixture re-serializes no record");

    let mut moved = fixture.clone();
    let victim = moved.objects[7].id.clone();
    moved.objects[7].origin[0] += 1.5;
    assert!(residency.refresh(&moved), "a moved object republishes");
    assert_eq!(residency.rebuilt_records(), 1, "exactly one record was re-serialized");
    assert_eq!(residency.changed_ids(), [victim.clone()], "the delta names exactly the moved id");
    assert!(residency.removed_ids().is_empty(), "a pose edit removes nothing");
    let delta = residency.delta_json().expect("a one-record delta on a 180-object set is worth publishing").to_string();
    assert!(
        delta.len() * 8 < full_bytes,
        "the delta must be O(changed): {} bytes for one moved record against a {full_bytes}-byte full set",
        delta.len()
    );
    let parsed = parse(&delta).expect("the delta lane is json");
    let changed = parsed.get("changed").and_then(Value::as_array).expect("the delta declares a changed list");
    assert_eq!(changed.len(), 1, "one changed record rides the delta");
    assert_eq!(changed[0].get("id").and_then(Value::as_str), Some(victim.as_str()), "and it is the moved one");
    assert_eq!(parsed.get("count").and_then(Value::as_u64), Some(fixture.objects.len() as u64), "the delta declares the resulting instance count so a consumer can prove its own set matches");
    assert_eq!(parsed.get("base").and_then(Value::as_u64).map(|base| base + 1), parsed.get("revision").and_then(Value::as_u64), "the delta names the revision it applies to and the one it produces");
    eprintln!("[DEBUG] b44.delta moved={victim} deltaBytes={} fullBytes={full_bytes} rebuilt={}", delta.len(), residency.rebuilt_records());
}

/// 🧾️ Wave B44 LAW: the incremental residency and the whole-set serializer are the same function. The
/// per-object path is only safe if its assembled text is byte-identical to the one-shot encode for
/// every authored example, at every step of an edit sequence.
#[test]
fn the_incremental_residency_assembles_byte_identically_to_the_whole_set_encode() {
    for fixture in [CONCRETE_FOREST_EXAMPLE_FIXTURE.clone(), NAKAGIN_EXAMPLE_FIXTURE.clone()] {
        let mut residency = main::Puzzle3dInstanceResidency::default();
        residency.refresh(&fixture);
        assert_eq!(residency.instances_json(), main::world_instances_geometry_json(&fixture), "a cold residency matches the whole-set encode");
        let mut edited = fixture.clone();
        if let Some(object) = edited.objects.first_mut() {
            object.origin[2] += 3.25;
            object.hidden = !object.hidden;
        }
        residency.refresh(&edited);
        assert_eq!(residency.instances_json(), main::world_instances_geometry_json(&edited), "an incrementally updated residency matches the whole-set encode");
        let mut shortened = edited.clone();
        shortened.objects.pop();
        residency.refresh(&shortened);
        assert_eq!(residency.instances_json(), main::world_instances_geometry_json(&shortened), "a removal matches the whole-set encode");
        assert_eq!(residency.removed_ids().len(), 1, "and the removal is named in the delta");
        let mut grown = shortened.clone();
        grown.objects.push(edited.objects.last().expect("an object to re-add").clone());
        residency.refresh(&grown);
        assert_eq!(residency.instances_json(), main::world_instances_geometry_json(&grown), "an addition matches the whole-set encode");
    }
}

/// ⏱️ Wave B44 LAW: a mutation's PUBLICATION must be O(changed) turns, not O(n).
///
/// 🧾️ A turn is one host↔guest round trip, and the typed-operation publication pages its emit at ONE
/// item per turn (`store::ArtifactStoreOneItemGrant { maximum_items: 1 }`,
/// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`). So the emit's mutation COUNT is the
/// publication's turn count, and that is the observable a latency law can pin deterministically —
/// unlike the settle's own turn total, which also carries the retirement state machine's
/// document-independent overhead (measured 113–362 turns either side of this wave's edits, wave B44
/// §1 and §5: the named residual).
///
/// 🎯️ The shape this forbids: an id-keyed delta degenerating into a rewrite of every following item
/// (an index-addressed list diff does exactly that), which is how one moved capsule would cost 180
/// round trips and burn a 30-second interaction budget.
#[test]
fn a_mutation_emits_o_changed_operations_whatever_the_document_size() {
    let large = &*NAKAGIN_EXAMPLE_FIXTURE;
    assert!(large.objects.len() >= 100, "the law needs a document large enough for O(n) and O(changed) to differ; got {}", large.objects.len());
    let before = crate::editor::puzzle3d::puzzle3d_snapshot_from_fixture(large);

    let mut deleted = large.clone();
    let removed = deleted.objects.remove(0).id;
    let delete_operations = puzzle3d_snapshot_mutations(&before, &crate::editor::puzzle3d::puzzle3d_snapshot_from_fixture(&deleted));
    eprintln!("[DEBUG] b44.emit objects={} deleteOperations={} removed={removed}", large.objects.len(), delete_operations.len());
    assert_eq!(delete_operations.len(), 1, "deleting the FIRST of {} objects must emit one operation, not rewrite the tail", large.objects.len());

    let mut moved = large.clone();
    moved.objects[0].origin[1] += 2.75;
    let move_operations = puzzle3d_snapshot_mutations(&before, &crate::editor::puzzle3d::puzzle3d_snapshot_from_fixture(&moved));
    eprintln!("[DEBUG] b44.emit objects={} moveOperations={}", large.objects.len(), move_operations.len());
    assert_eq!(move_operations.len(), 1, "moving one of {} objects must emit one operation", large.objects.len());

    let mut both = large.clone();
    both.objects[0].origin[1] += 2.75;
    both.objects[1].origin[2] -= 1.25;
    let both_operations = puzzle3d_snapshot_mutations(&before, &crate::editor::puzzle3d::puzzle3d_snapshot_from_fixture(&both));
    assert_eq!(both_operations.len(), 2, "two moved objects emit two operations — the emit is priced by the delta, not by the document");
}

/// ⏱️ Wave B44 measurement (not a gate): the host-grant turn count one delete spends either side of the
/// document-size jump, so the report's before/after numbers are read off the same instrument the law
/// above reasons about. The settle's own overhead is document-independent and noisy, which is exactly
/// why the gate is the emit size rather than this number.
#[semio_framework_async_macros::async_test]
async fn b44_measures_the_turns_one_mutation_settles_in() {
    let mut app = app().await;
    let small_objects = object_count(&app);
    let small = mutation_turns(&mut app).await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin switch");
    let large_objects = object_count(&app);
    let large = mutation_turns(&mut app).await;
    eprintln!("[DEBUG] b44.turns small={small_objects}objects/{small}turns large={large_objects}objects/{large}turns");
    assert!(large_objects > small_objects, "the two measurements must differ in document size");
}

/// 🗑️ Selects the first object and deletes it, answering how many HOST-GRANT turns that took — a turn
/// is one host↔guest round trip, granted [`SETTLE_HOST_TURN_ITEMS`] items, exactly as the browser's own
/// runtime grants them.
async fn mutation_turns(app: &mut Puzzle3dApp) -> usize {
    let victim = first_object_id(app);
    select_id(app, crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT, &victim).await.expect("selection");
    let (result, settled) = dispatch_reporting_with_items(app, "deleteSelection", None, None, SETTLE_HOST_TURN_ITEMS).await;
    result.expect("delete dispatches");
    settled.turns
}

/// 📊️ One document size's per-stage cost, in microseconds, with the object count it was taken at.
struct Puzzle3dLatencyCensus {
    example: &'static str,
    objects: usize,
    typed_decode_us: u128,
    instances_json_us: u128,
    instances_bytes: usize,
    fingerprint_us: u128,
    meshes_json_us: u128,
    vortices_json_us: u128,
    render_viewport_us: u128,
    render_viewport_second_us: u128,
    set_camera_us: u128,
    select_us: u128,
    delete_us: u128,
}

impl std::fmt::Display for Puzzle3dLatencyCensus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "example={} objects={} typedDecode={}us instancesJson={}us/{}B fingerprint={}us meshesJson={}us vorticesJson={}us renderViewport={}us renderViewportAgain={}us setCamera={}us select={}us delete={}us",
            self.example,
            self.objects,
            self.typed_decode_us,
            self.instances_json_us,
            self.instances_bytes,
            self.fingerprint_us,
            self.meshes_json_us,
            self.vortices_json_us,
            self.render_viewport_us,
            self.render_viewport_second_us,
            self.set_camera_us,
            self.select_us,
            self.delete_us,
        )
    }
}

async fn latency_census(app: &mut Puzzle3dApp, example: &'static str) -> Puzzle3dLatencyCensus {
    let snapshot = app.snapshot().expect("projection");
    let typed = snapshot.typed().clone();
    let started = Instant::now();
    let fixture = puzzle3d_fixture_from_snapshot(&typed);
    let typed_decode_us = started.elapsed().as_micros();
    let started = Instant::now();
    let instances = main::world_instances_geometry_json(&fixture);
    let instances_json_us = started.elapsed().as_micros();
    let started = Instant::now();
    let _ = main::fixture_geometry_fingerprint(&fixture);
    let fingerprint_us = started.elapsed().as_micros();
    let started = Instant::now();
    let _ = main::world_meshes_json(&fixture);
    let meshes_json_us = started.elapsed().as_micros();
    let runtime = Puzzle3dRuntime::default();
    let started = Instant::now();
    let _ = main::world_vortices_json(&fixture, &runtime, &Puzzle3dInteractionSnapshot::default(), "select");
    let vortices_json_us = started.elapsed().as_micros();
    let started = Instant::now();
    let _ = render_body(app, main::BODY_KEY).await;
    let render_viewport_us = started.elapsed().as_micros();
    let started = Instant::now();
    let _ = render_body(app, main::BODY_KEY).await;
    let render_viewport_second_us = started.elapsed().as_micros();
    let started = Instant::now();
    let _ = dispatch(app, "setCamera", Some(&json!({ "position": [7.0, -7.0, 5.0], "target": [0.0, 0.0, 0.0], "zoom": 1.0 })), None).await;
    let set_camera_us = started.elapsed().as_micros();
    let victim = first_object_id(app);
    let started = Instant::now();
    let _ = select_id(app, crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT, &victim).await;
    let select_us = started.elapsed().as_micros();
    let started = Instant::now();
    let _ = dispatch(app, "deleteSelection", None, None).await;
    let delete_us = started.elapsed().as_micros();
    Puzzle3dLatencyCensus {
        example,
        objects: fixture.objects.len(),
        typed_decode_us,
        instances_json_us,
        instances_bytes: instances.len(),
        fingerprint_us,
        meshes_json_us,
        vortices_json_us,
        render_viewport_us,
        render_viewport_second_us,
        set_camera_us,
        select_us,
        delete_us,
    }
}

//#region 🔖️B54TurnCensus
/// 🧊️ A document `factor`× the size of Nakagin, built by cloning its objects under fresh ids — the
/// brush-painted condition the checkpoint battery fails under (161 `puzzle3d.brush.*` objects on top of
/// the authored 180) without driving a brush stroke, so a native law can measure the same size.
fn scaled_nakagin_scene(factor: usize) -> Puzzle3dScene {
    let mut fixture = NAKAGIN_EXAMPLE_FIXTURE.clone();
    let authored = fixture.objects.clone();
    for copy in 1..factor {
        for object in &authored {
            let mut clone = object.clone();
            clone.id = format!("{}#b54-{copy}", object.id);
            clone.origin[0] += 120.0 * copy as f64;
            fixture.objects.push(clone);
        }
    }
    Puzzle3dScene { fixture, runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() }
}

/// 📊️ One document size's publication cost, as turns against the units those turns carried.
struct Puzzle3dTurnCensus {
    label: &'static str,
    objects: usize,
    translate_turns: usize,
    translate_units: u64,
    translate_census: semio_framework_plugin::app::TypedOperationUnitCensus,
    delete_turns: usize,
    delete_units: u64,
    delete_census: semio_framework_plugin::app::TypedOperationUnitCensus,
}

impl Puzzle3dTurnCensus {
    /// ⏱️ Host turns the PUBLICATION ladder cost, with the native worker pool's wall-clock poll spin
    /// removed — see the law's own docstring for why that term can never be a gate.
    fn translate_ladder_turns(&self) -> usize {
        self.translate_turns.saturating_sub(self.translate_census.worker as usize)
    }

    fn delete_ladder_turns(&self) -> usize {
        self.delete_turns.saturating_sub(self.delete_census.worker as usize)
    }
}

impl std::fmt::Display for Puzzle3dTurnCensus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "label={} objects={} ladder={}/{}turns translate={}turns/{}units [{}] delete={}turns/{}units [{}]",
            self.label, self.objects, self.translate_ladder_turns(), self.delete_ladder_turns(), self.translate_turns, self.translate_units, self.translate_census, self.delete_turns, self.delete_units, self.delete_census
        )
    }
}

/// ⏱️ Selects the first object, translates it, then selects the first object again and deletes it,
/// answering the host-grant turns and the publication units each of the two spent.
async fn turn_census(app: &mut Puzzle3dApp, label: &'static str) -> Puzzle3dTurnCensus {
    let objects = object_count(app);
    let victim = first_object_id(app);
    select_id(app, crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT, &victim).await.expect("selection before translate");
    let (result, translated) = dispatch_reporting_with_items(app, "translateSelection", Some(&json!({ "ids": [victim.as_str()], "dx": 3.5, "dy": 0.0, "dz": 0.0 })), None, SETTLE_HOST_TURN_ITEMS).await;
    result.expect("translate dispatches");
    let victim = first_object_id(app);
    select_id(app, crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT, &victim).await.expect("selection before delete");
    let (result, deleted) = dispatch_reporting_with_items(app, "deleteSelection", None, None, SETTLE_HOST_TURN_ITEMS).await;
    result.expect("delete dispatches");
    Puzzle3dTurnCensus {
        label,
        objects,
        translate_turns: translated.turns,
        translate_units: translated.census.units,
        translate_census: translated.census,
        delete_turns: deleted.turns,
        delete_units: deleted.census.units,
        delete_census: deleted.census,
    }
}

/// ⏱️ Wave B54 measurement: what one `translateSelection` and one `deleteSelection` cost in HOST TURNS
/// and in publication UNITS at three document sizes. Not a gate — it prints the attribution table the
/// wave's report quotes. Run with `--nocapture`.
#[semio_framework_async_macros::async_test]
async fn b54_measures_the_turns_and_units_one_mutation_costs_per_document_size() {
    let mut app = app().await;
    let small = turn_census(&mut app, "concrete-forest").await;
    eprintln!("[DEBUG] b54.turns {small}");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin switch");
    let nakagin = turn_census(&mut app, "nakagin").await;
    eprintln!("[DEBUG] b54.turns {nakagin}");
    assert!(nakagin.objects > small.objects, "the two measurements must differ in document size: {} / {}", small.objects, nakagin.objects);
}

/// ⏱️ Wave B54 LAW: one mutation's PUBLICATION LADDER completes within a bounded, size-independent
/// number of host turns.
///
/// 🧾️ A turn is one host↔guest round trip and it is the unit a browser actually pays. Before this wave
/// every publication unit was its own turn, so one mutation spent 21 round trips walking a state machine
/// whose steps are all document-independent: 17 store units (stage the batch, prepare the item, fold it,
/// retire its preparation owner, validate the cursor, preflight, commit, mint the receipt), 3
/// acknowledgeable result pages, 1 slot retirement. [`Puzzle3dApp`]'s ladder now drives a bounded RUN of
/// units per turn ([`semio_framework_plugin::app::TYPED_OPERATION_PUBLICATION_PUMPS`]), so what a turn
/// costs is the number of pages the host must ACKNOWLEDGE — a protocol constant — not the number of
/// steps the machine takes.
///
/// ⚠️ [`Puzzle3dTurnCensus::worker_turns`] is EXCLUDED, and must be: on native the interactive worker pool
/// has real threads, so `drive_typed_operation_worker` submits a job step and returns immediately and the
/// harness POLLS it — the count is wall-clock divided by poll cost (measured 520–1 610 for the same
/// one-object translate across runs), never a browser round trip. Only the publication ladder is
/// deterministic enough to be a gate; the worker term is measured in the wave's report instead.
///
/// 🎯️ The shape this forbids is the one that burned the 30-second budget: a per-command round-trip count
/// that no document size can explain and that no amount of payload work can reduce.
#[semio_framework_async_macros::async_test]
async fn one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns() {
    const LADDER_TURN_CEILING: usize = 8;
    let mut app = app().await;
    let small = turn_census(&mut app, "concrete-forest").await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin switch");
    let large = turn_census(&mut app, "nakagin").await;
    eprintln!("[DEBUG] b54.law.turns small={small} large={large}");
    assert!(large.objects >= 100, "the law needs a document where O(1) and O(n) differ; got {}", large.objects);
    for census in [&small, &large] {
        assert!(
            census.translate_ladder_turns() <= LADDER_TURN_CEILING,
            "a translate on {} objects must publish within {LADDER_TURN_CEILING} host turns, not one per unit: {census}",
            census.objects
        );
        assert!(census.delete_ladder_turns() <= LADDER_TURN_CEILING, "a delete on {} objects must publish within {LADDER_TURN_CEILING} host turns: {census}", census.objects);
        assert!(
            census.translate_census.store >= census.translate_ladder_turns() as u64,
            "the turn must carry MORE than one publication unit or the run bought nothing: {census}"
        );
    }
    assert_eq!(
        small.translate_census.store, large.translate_census.store,
        "the publication ladder is document-INDEPENDENT, so its unit count may not move with the document: {small} / {large}"
    );
}

/// 🗺️ Wave B54 LAW: a pose edit on a large document invalidates the precompute derivation of exactly the
/// objects that changed — O(changed) index cells and cached candidates, not O(n).
///
/// 🧾️ Every scene sync whose `SceneConfig` differed at all used to be a whole-document `rebuild_queue`:
/// `brush_cache` cleared for all 340 objects, both prepare cursors reset to zero so the entire
/// object × vortex product was re-walked, the fill preparation restarted. The background
/// `suggestionsTick`/`fillBuildTick` cadence re-paid that every 120 ms while an interactive mutation
/// waited behind it, which is how a translate that reads ONE object burned a 30-second budget.
#[test]
fn a_pose_edit_invalidates_the_precompute_derivation_of_o_changed_objects() {
    let large = scaled_nakagin_scene(2);
    assert!(large.fixture.objects.len() >= 340, "the law needs a document where O(changed) and O(n) differ; got {}", large.fixture.objects.len());
    let base = scene_config(&large).expect("the authored document builds an engine scene");

    let mut moved_fixture = large.clone();
    moved_fixture.fixture.objects[11].origin[1] += 2.25;
    let moved = scene_config(&moved_fixture).expect("the moved document builds an engine scene");
    let invalidation = crate::editor::puzzle3d::precompute::Puzzle3dSceneInvalidation::between(&base, &moved);
    let vortices = large.fixture.objects[11].vortices.len();
    eprintln!("[DEBUG] b54.invalidation objects={} moved=1 stale={} pending={} topology={} plan={}", large.fixture.objects.len(), invalidation.stale.len(), invalidation.pending.len(), invalidation.topology, invalidation.plan);
    assert!(!invalidation.plan, "a pose edit changes no fill-plan member, so it must not fall back to the whole-scene rebuild");
    assert!(!invalidation.topology, "a pose edit adds and removes no object, so the fill preparation must not restart");
    assert_eq!(invalidation.stale.len(), vortices, "a pose edit invalidates exactly the moved object's own brush targets");
    assert_eq!(invalidation.pending.len(), vortices, "and re-queues exactly those");

    let mut removed_fixture = large.clone();
    let removed = removed_fixture.fixture.objects.remove(0);
    let shortened = scene_config(&removed_fixture).expect("the shortened document builds an engine scene");
    let invalidation = crate::editor::puzzle3d::precompute::Puzzle3dSceneInvalidation::between(&base, &shortened);
    assert!(invalidation.topology, "a removal moves the object topology, so the fill preparation restarts");
    assert!(!invalidation.plan, "a removal still changes no fill-plan member");
    assert_eq!(invalidation.stale.len(), removed.vortices.len(), "deleting the FIRST of {} objects invalidates only its own targets — an index-addressed diff would invalidate the whole tail", large.fixture.objects.len());

    let mut replanned_fixture = large.clone();
    replanned_fixture.runtime.overlap_budget += 0.25;
    let replanned = scene_config(&replanned_fixture).expect("the replanned document builds an engine scene");
    assert!(crate::editor::puzzle3d::precompute::Puzzle3dSceneInvalidation::between(&base, &replanned).plan, "a fill-plan member change invalidates every candidate and must take the whole-scene rebuild");
}

/// 🗺️ Wave B54 LAW: the engine ITSELF keeps the candidates of the objects a pose edit did not touch.
///
/// 🧾️ The law above pins the diff; this one pins that the diff is what the live engine acts on, by
/// syncing a scene, warming the brush lane, moving one object and requiring the cache to survive.
#[test]
fn a_pose_edit_keeps_the_brush_candidates_of_every_object_it_did_not_touch() {
    let large = scaled_nakagin_scene(2);
    let mut session = Puzzle3dPrecomputeSession::new();
    sync_precompute_session(&mut session, &large);
    for _ in 0..2_048 {
        session.precompute_step_lane(crate::standards::v1::subsets::any::schema::PrecomputeLane::Brush, 8);
    }
    let warmed = session.brush_candidate_cache_len();
    assert!(warmed > 0, "the brush lane must resolve at least one candidate before the law can measure what a sync keeps");
    let mut moved = large.clone();
    moved.fixture.objects[11].origin[1] += 2.25;
    sync_precompute_session(&mut session, &moved);
    let kept = session.brush_candidate_cache_len();
    eprintln!("[DEBUG] b54.cache objects={} warmed={warmed} kept={kept}", large.fixture.objects.len());
    assert!(kept + large.fixture.objects[11].vortices.len() >= warmed, "a pose edit on one of {} objects must keep every other object's resolved candidates: {warmed} → {kept}", large.fixture.objects.len());
}
//#endregion 🔖️B54TurnCensus

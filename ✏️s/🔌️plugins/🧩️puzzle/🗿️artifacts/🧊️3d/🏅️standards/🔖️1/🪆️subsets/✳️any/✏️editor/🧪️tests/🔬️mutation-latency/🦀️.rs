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

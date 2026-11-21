use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_DEFAULT_UTILITY;
use semio_s_artifact_draw_drawing_editor::{DRAWING_GESTURE_RAW_BYTES, DRAWING_GESTURE_RETAINED_BYTES, DrawingCommand, DrawingGestureOperationOwner, DrawingInstanceOperationOwner, DrawingRetainedCommandDecoder};
use semio_s_artifact_draw_drawing_editor_tool as canvas_pointer_down;
use semio_framework_job::{Generation, InteractiveJobCloseStep, OperationId, OriginalJobFixtureCaller};
use semio_framework_job_registry::FixedOperationOwner as _;
use semio_s_artifact_draw_drawing_scene::identity::SceneIdentity;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};

fn grant(items: usize, bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: bytes, maximum_capacity_bytes: bytes, maximum_release_bytes: bytes, maximum_depth: 32 }
}

fn original_caller() -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(OperationId(1), Generation(1), grant(1 << 20, 1 << 30), || Some(0), u64::MAX, 1 << 20)
}

fn released(items: usize, bytes: usize) -> InteractiveJobCloseStep {
    InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: items, released_bytes: bytes, ..Default::default() } }
}

fn decode_retained(expected: &'static str, wire: &[u8]) -> bool {
    let mut decoder = DrawingRetainedCommandDecoder::new(expected);
    let (head, tail) = wire.split_at(wire.len() / 2);
    decoder.feed_page(head) && decoder.feed_page(tail) && decoder.finish()
}

fn wire(command: &DrawingCommand) -> Vec<u8> {
    ::protocol::OpBinary::encode_op(command).expect("gesture command encodes")
}

fn key(operation: u64, generation: u64) -> semio_framework_job_registry::FixedOperationKey {
    semio_framework_job_registry::FixedOperationKey::new(semio_framework_job::OperationId(operation), semio_framework_job::Generation(generation))
}

fn drain(registry: &mut semio_framework_job_registry::FixedOperationRegistry<DrawingGestureOperationOwner, 64>) {
    let mut caller = original_caller();
    for _ in 0..64 {
        registry.begin_close_step();
    }
    for _ in 0..256 {
        if registry.is_empty() {
            break;
        }
        caller.turn(grant(1, DRAWING_GESTURE_RETAINED_BYTES), |cx| registry.close_step(cx)).expect("original fixture close turn");
    }
    assert!(registry.is_empty());
    caller.retire_exact();
}

#[test]
fn drawing_gesture_maximum_plus_one_returns_the_exact_owner() {
    let mut registry = semio_framework_job_registry::FixedOperationRegistry::<DrawingGestureOperationOwner, 64>::new(64 * DRAWING_GESTURE_RETAINED_BYTES);
    for operation in 0..64 {
        if registry.admit(key(operation, 0), DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY, "")).is_err() {
            panic!("every distinct fixed slot must admit through the declared maximum");
        }
    }
    let rejected = match registry.admit(key(64, 0), DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY, "")) {
        Ok(()) => panic!("maximum plus one must return its exact owner"),
        Err(rejected) => rejected,
    };
    assert_eq!(rejected.key, key(64, 0));
    assert!(rejected.owner.session.is_some());
    let mut owner = rejected.owner;
    owner.cancel();
    owner.begin_close();
    let mut caller = original_caller();
    assert_eq!(caller.turn(grant(1, DRAWING_GESTURE_RETAINED_BYTES), |cx| owner.close_step(cx)).unwrap(), released(1, DRAWING_GESTURE_RETAINED_BYTES));
    assert!(owner.terminal_is_empty());
    caller.retire_exact();
    drain(&mut registry);
}

#[test]
fn drawing_gesture_stale_generation_and_aba_are_exact() {
    let mut caller = original_caller();
    let mut registry = semio_framework_job_registry::FixedOperationRegistry::<DrawingGestureOperationOwner, 64>::new(DRAWING_GESTURE_RETAINED_BYTES);
    assert!(registry.admit(key(7, 1), DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY, "")).is_ok());
    for _ in 0..64 {
        if registry.cancel_stale_step(semio_framework_job::OperationId(7), semio_framework_job::Generation(2)) {
            break;
        }
    }
    for _ in 0..128 {
        if registry.is_empty() {
            break;
        }
        let _ = caller.turn(grant(1, DRAWING_GESTURE_RETAINED_BYTES), |cx| registry.close_step(cx)).expect("original fixture close turn");
    }
    assert!(registry.is_empty());
    assert!(registry.admit(key(7, 2), DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY, "")).is_ok(), "the new generation owns the retired slot");
    registry.cancel(key(7, 2));
    for _ in 0..128 {
        if registry.is_empty() {
            break;
        }
        let _ = caller.turn(grant(1, DRAWING_GESTURE_RETAINED_BYTES), |cx| registry.close_step(cx)).expect("original fixture close turn");
    }
    assert!(registry.is_empty());
    caller.retire_exact();
}

#[test]
fn drawing_gesture_interrupted_and_repeated_close_is_terminal_empty() {
    let mut caller = original_caller();
    let mut owner = DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY, "");
    owner.cancel();
    owner.begin_close();
    assert_eq!(caller.turn(grant(0, DRAWING_GESTURE_RETAINED_BYTES), |cx| owner.close_step(cx)).expect("original fixture close turn"), InteractiveJobCloseStep::Blocked);
    assert_eq!(caller.turn(RetainedCloneGrant { maximum_release_bytes: 0, ..grant(1, DRAWING_GESTURE_RETAINED_BYTES) }, |cx| owner.close_step(cx)).expect("original fixture close turn"), InteractiveJobCloseStep::Blocked);
    assert_eq!(caller.turn(grant(1, DRAWING_GESTURE_RETAINED_BYTES), |cx| owner.close_step(cx)).expect("original fixture close turn"), released(1, DRAWING_GESTURE_RETAINED_BYTES));
    assert_eq!(caller.turn(grant(1, DRAWING_GESTURE_RETAINED_BYTES), |cx| owner.close_step(cx)).expect("original fixture close turn"), InteractiveJobCloseStep::Complete { progress: Default::default() });
    assert!(owner.terminal_is_empty());
    caller.retire_exact();
}

/// 🧾️ The framework's close and maintenance pumps grant one `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES`
/// page per step; the owner must close under that grant — session dropped on the first page, the
/// declared budget handed back page by page, never a page over the grant.
#[test]
fn drawing_gesture_owner_closes_under_the_framework_page_grant() {
    let mut caller = original_caller();
    let page = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    let mut owner = DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY, "");
    owner.begin_close();
    assert_eq!(caller.turn(grant(1, page), |cx| owner.close_step(cx)).expect("original fixture close turn"), released(1, page));
    assert!(owner.session.is_none(), "the session is dropped on the first granted page");
    let mut released = page;
    let mut steps = 1;
    loop {
        match caller.turn(grant(1, page), |cx| owner.close_step(cx)).expect("original fixture close turn") {
            InteractiveJobCloseStep::Pending { progress } => {
                assert_eq!(progress.copied_items, 0);
                assert!(progress.released_bytes > 0 && progress.released_bytes <= page);
                released += progress.released_bytes;
                steps += 1;
                assert!(steps <= DRAWING_GESTURE_RETAINED_BYTES / page + 1, "close terminates within the declared budget");
            }
            InteractiveJobCloseStep::Complete { .. } => break,
            other => panic!("unexpected close step {other:?}"),
        }
    }
    assert_eq!(released, DRAWING_GESTURE_RETAINED_BYTES, "the whole declared budget is handed back");
    assert!(owner.terminal_is_empty());
    caller.retire_exact();
}

#[test]
fn drawing_retained_decoder_is_incremental_exact_and_fail_closed() {
    let pointer_move = wire(&DrawingCommand::CanvasPointerMove(semio_s_artifact_draw_drawing_editor_tool::canvas_pointer_move::CanvasPointerMove { shift: false, alt: false,  x: 1.0, y: 2.0, width: 10.0, height: 10.0, samples: Vec::new() }));
    let pointer_up = wire(&DrawingCommand::CanvasPointerUp(semio_s_artifact_draw_drawing_editor_tool::canvas_pointer_up::CanvasPointerUp { alt: false,  x: 1.0, y: 2.0, width: 10.0, height: 10.0, shift: false, ctrl: false, meta: false, cancelled: false }));
    assert!(decode_retained("canvasPointerMove", &pointer_move));
    assert!(decode_retained("canvasEscape", &wire(&DrawingCommand::CanvasEscape(semio_s_artifact_draw_drawing_editor_tool::canvas_escape::CanvasEscape {}))));
    assert!(!decode_retained("canvasPointerMove", &pointer_up), "a variant swap rejects");
    assert!(!decode_retained("canvasPointerMove", &pointer_move[..pointer_move.len() - 1]), "truncation rejects");
    assert!(!decode_retained("canvasPointerMove", &vec![0; DRAWING_GESTURE_RAW_BYTES + 1]), "an oversized owner rejects");
    assert!(!decode_retained("canvasPointerMove", br#"["canvasPointerMove",{"x":1,"y":2}]"#), "the retired JSON text shape is no longer a wire");
}

#[test]
fn drawing_preview_rejects_a_stale_revision_and_cancels_the_owner() {
    let mut caller = original_caller();
    let mut owner = DrawingInstanceOperationOwner::new();
    let operation = key(9, 4);
    assert!(owner.operations.admit(operation, DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY, "")).is_ok());
    let source=SceneIdentity{instance:1,base:11,generation:4,revision:[1;32]};
    owner.active = Some((operation, source));
    assert!(owner.preview_projection(SceneIdentity{revision:[2;32],..source}, "selectDirect").is_none());
    assert!(owner.active.is_none());
    for _ in 0..128 {
        if owner.operations.is_empty() {
            break;
        }
        caller.turn(grant(1, DRAWING_GESTURE_RETAINED_BYTES), |cx| owner.operations.close_step(cx)).expect("original registry close turn");
    }
    assert!(owner.operations.is_empty());
    caller.retire_exact();
}

#[test]
fn mounted_vector_gesture_preview_requires_all_captured_full_width_source_lanes(){
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/🎬️scene/🪪️identity/🔣️.json")).unwrap();
 let decode=|value:&serde_json::Value|SceneIdentity{instance:u32::try_from(value["instance"].as_u64().unwrap()).unwrap(),base:semio_s_artifact_draw_drawing_scene::identity::parse_scene_identity_u64(value["base"].as_str().unwrap()).unwrap(),generation:semio_s_artifact_draw_drawing_scene::identity::parse_scene_identity_u64(value["generation"].as_str().unwrap()).unwrap(),revision:value["revision"].as_array().unwrap().iter().map(|byte|u8::try_from(byte.as_u64().unwrap()).unwrap()).collect::<Vec<_>>().try_into().unwrap()};
 for row in cases.as_array().unwrap(){let captured=decode(&row["captured"]);let live=decode(&row["live"]);let mut owner=DrawingInstanceOperationOwner::new();let operation=key(901,captured.generation);let mut gesture=DrawingGestureOperationOwner::new("selectDirect","");let document=semio_s_artifact_draw_drawing_model::default_drawing_document("empty",None);
  gesture.session.as_mut().unwrap().point_query=Some(canvas_pointer_down::DrawingPointQuery::new("canvasPointerDown",canvas_pointer_down::TracePointerJob::new_query(&document,[0.0,0.0],0.0,false),false,"replace".into(),false));assert!(owner.operations.admit(operation,gesture).is_ok());owner.active=Some((operation,captured));let expected=row["matches"].as_bool().unwrap();assert_eq!(owner.preview_projection(live,"selectDirect").is_some(),expected,"{}",row["name"]);assert_eq!(owner.active.is_some(),expected);owner.operations.cancel(operation);owner.active=None;drain(&mut owner.operations);
 }
}

/// 🚪️ Owners admitted on BOTH cursor parities close through the instance owner's own close
/// protocol under the framework page grant — the registry's shared cursor must not strand a slot.
#[test]
fn drawing_instance_owner_close_reaches_every_slot_parity() {
    use semio_plugin::ArtifactInstanceOperationOwner as _;
    let mut caller = original_caller();
    let mut owner = DrawingInstanceOperationOwner::new();
    for operation in 0..3u64 {
        assert!(owner.operations.admit(key(operation, 0), DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY, "")).is_ok());
    }
    let page = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    for _ in 0..4_096 {
        if owner.terminal_is_empty() {
            break;
        }
        let turn = grant(1, page);
        let step = caller.turn(turn, |cx| Ok(owner.close_step(cx, turn))).unwrap().expect("instance owner close step");
        if let semio_plugin::PluginLifecycleStep::Progress(progress) = step {
            assert!(progress.copied_items <= 1 && progress.released_bytes <= page, "one page per step: {step:?}");
        }
    }
    assert!(owner.terminal_is_empty(), "three owners across both cursor parities close under the page grant");
    assert!(owner.operations.is_empty());
    caller.retire_exact();
}

/// 🎰️ The framework mints every tool operation id into the first vacant residue class of its own
/// 64-slot table, so consecutive settled gestures arrive as 256, 320, 384, … — all `≡ 0 (mod 64)`,
/// all addressing ONE direct-mapped slot here. A gesture whose predecessor is cancelled but not yet
/// retired must retire it and admit, never answer `saturated`.
#[test]
fn drawing_gesture_admission_retires_the_retiring_owner_in_its_residue_class() {
    let mut caller = original_caller();
    let mut owner = DrawingInstanceOperationOwner::new();
    let first = key(256, 2);
    assert!(owner.operations.admit(first, DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY, "")).is_ok());
    owner.operations.cancel(first);
    let second = key(320, 3);
    assert!(!owner.operations.can_admit(second, DRAWING_GESTURE_RETAINED_BYTES), "the retiring predecessor still holds the residue class");
    for _ in 0..128 {
        if owner.operations.can_admit(second, DRAWING_GESTURE_RETAINED_BYTES) {
            break;
        }
        caller.turn(grant(1, DRAWING_GESTURE_RETAINED_BYTES), |cx| owner.operations.close_step(cx)).expect("original registry close turn");
    }
    assert!(owner.operations.can_admit(second, DRAWING_GESTURE_RETAINED_BYTES), "one bounded sweep retires the predecessor");
    assert!(owner.operations.admit(second, DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY, "")).is_ok());
    owner.operations.cancel(second);
    drain(&mut owner.operations);
    caller.retire_exact();
}

/// 🔁️ A cancelled owner retires within a handful of maintenance steps under the page grant, not one
/// page per 64-slot cursor lap.
#[test]
fn drawing_gesture_maintenance_retires_a_cancelled_owner_promptly() {
    use semio_plugin::ArtifactInstanceOperationOwner as _;
    let mut caller = original_caller();
    let mut owner = DrawingInstanceOperationOwner::new();
    let live = key(448, 3);
    assert!(owner.operations.admit(live, DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY, "")).is_ok());
    owner.operations.cancel(live);
    let page = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    let mut steps = 0;
    while !owner.operations.is_empty() {
        let turn = grant(1, page);
        caller.turn(turn, |cx| Ok(owner.maintenance_step(cx, turn))).unwrap().expect("gesture maintenance step");
        steps += 1;
        assert!(steps <= DRAWING_GESTURE_RETAINED_BYTES / page + 2, "the owner retires page by page without idle laps: {steps} steps");
    }
    caller.retire_exact();
}

#[test]
fn drawing_empty_instance_owner_records_its_paid_close_transition() {
    use semio_plugin::ArtifactInstanceOperationOwner as _;
    let rows: Vec<serde_json::Value> = serde_json::from_str(fixture_text!("🧫️fixtures/🚪️empty-close/🔣️.json")).unwrap();
    let mut caller = original_caller();
    let mut owner = DrawingInstanceOperationOwner::new();
    for row in rows {
        let copy = row["copyGrant"].as_u64().unwrap() as usize;
        let demand = owner.retirement_demands(copy).unwrap();
        let turn = grant(1, copy);
        let step = caller.turn(turn, |cx| cx.with_original_retirement(|cx| owner.close_step(cx, turn).map_err(Into::into))).unwrap();
        let progress = step.progress().unwrap_or_default();
        assert_eq!(serde_json::json!({
            "demandCopyBytes": demand.copy_bytes,
            "copiedItems": progress.copied_items,
            "copiedBytes": progress.copied_bytes,
            "terminal": owner.terminal_is_empty(),
        }), row["expected"]);
    }
    caller.retire_exact();
}

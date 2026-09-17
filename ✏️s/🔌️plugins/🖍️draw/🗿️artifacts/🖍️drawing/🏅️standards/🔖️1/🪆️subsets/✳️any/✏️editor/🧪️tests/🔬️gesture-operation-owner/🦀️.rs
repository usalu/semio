use super::*;
use semio_framework_job::InteractiveJob as _;

#[test]
fn drawing_completion_rejection_retires_child_before_decoder_without_redispatch() {
    let mut emit: Emit<DrawingMutation, NoConfigMutation, NoDraftMutation> = Emit::default();
    emit.child_emits.push(semio_framework_plugin::app::ChildEmit::of::<DrawingSnapshot, DrawingMutation>("member", "drawing-child", &[]));
    let rejected = semio_framework_plugin::app::ArtifactToolCompletionRejection::<semio_framework_plugin::EditorApp<DrawingPlayApp>> {
        emit: Ok(emit),
        ephemeral: semio_framework_plugin::EphemeralEmit::default(),
        fault: Fault::new(FaultOrigin::Framework, FaultCode::new("test.completion-rejected"), "injected completion rejection"),
    };
    let mut job = DrawingGestureOperationJob {
        payload: None,
        pending_completion_rejection: Some(rejected),
        raw_input: None,
        raw_page_cursor: 0,
        raw_byte_cursor: 0,
        decoder: Some(DrawingRetainedCommandDecoder::new("canvasEscape")),
        raw_validated: true,
        completed: false,
        closing: false,
    };
    job.begin_close();
    // 🧾️ A zero-item grant releases nothing (`ChildEmit::close_one` short-circuits on `maximum_items == 0`).
    assert_eq!(job.close_step(0, 1), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert!(job.pending_completion_rejection.is_some());
    assert!(job.decoder.is_some());
    for _ in 0..128 {
        if job.pending_completion_rejection.is_none() {
            break;
        }
        let step = job.close_step(1, 4);
        if let semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1 && released_bytes <= 4);
        }
    }
    assert!(job.pending_completion_rejection.is_none());
    assert!(job.decoder.is_some(), "normal decoder owner stays retained until the rejected output is terminal");
    assert_eq!(job.close_step(1, 4), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 });
    assert_eq!(job.close_step(1, 4), semio_framework_job::InteractiveJobCloseStep::Complete);
    assert!(job.terminal_is_empty());
}

fn decode_retained(expected: &'static str, wire: &[u8]) -> bool {
    let mut decoder = DrawingRetainedCommandDecoder::new(expected);
    // 📄️ Fed as two pages so the page seam is exercised like the host's paged wire owner.
    let (head, tail) = wire.split_at(wire.len() / 2);
    decoder.feed_page(head) && decoder.feed_page(tail) && decoder.finish()
}

fn wire(command: &DrawingCommand) -> Vec<u8> {
    ::protocol::OpBinary::encode_op(command).expect("gesture command encodes")
}

fn key(operation: u64, generation: u64) -> semio_framework_job::FixedOperationKey {
    semio_framework_job::FixedOperationKey::new(semio_framework_job::OperationId(operation), semio_framework_job::Generation(generation))
}

fn drain(registry: &mut semio_framework_job::FixedOperationRegistry<DrawingGestureOperationOwner, 64>) {
    for operation in 0..64 {
        registry.cancel(key(operation, 0));
    }
    for _ in 0..256 {
        if registry.is_empty() {
            return;
        }
        let _ = registry.close_step(1, DRAWING_GESTURE_RETAINED_BYTES);
    }
    assert!(registry.is_empty());
}

#[test]
fn drawing_gesture_maximum_plus_one_returns_the_exact_owner() {
    let mut registry = semio_framework_job::FixedOperationRegistry::<DrawingGestureOperationOwner, 64>::new(64 * DRAWING_GESTURE_RETAINED_BYTES);
    for operation in 0..64 {
        if registry.admit(key(operation, 0), DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY)).is_err() {
            panic!("every distinct fixed slot must admit through the declared maximum");
        }
    }
    let rejected = match registry.admit(key(64, 0), DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY)) {
        Ok(()) => panic!("maximum plus one must return its exact owner"),
        Err(rejected) => rejected,
    };
    assert_eq!(rejected.key, key(64, 0));
    assert!(rejected.owner.session.is_some());
    let mut owner = rejected.owner;
    owner.cancel();
    owner.begin_close();
    assert_eq!(owner.close_step(1, DRAWING_GESTURE_RETAINED_BYTES), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: DRAWING_GESTURE_RETAINED_BYTES });
    assert!(owner.terminal_is_empty());
    drain(&mut registry);
}

#[test]
fn drawing_gesture_stale_generation_and_aba_are_exact() {
    let mut registry = semio_framework_job::FixedOperationRegistry::<DrawingGestureOperationOwner, 64>::new(DRAWING_GESTURE_RETAINED_BYTES);
    assert!(registry.admit(key(7, 1), DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY)).is_ok());
    for _ in 0..64 {
        if registry.cancel_stale_step(semio_framework_job::OperationId(7), semio_framework_job::Generation(2)) {
            break;
        }
    }
    for _ in 0..128 {
        if registry.is_empty() {
            break;
        }
        let _ = registry.close_step(1, DRAWING_GESTURE_RETAINED_BYTES);
    }
    assert!(registry.is_empty());
    assert!(registry.admit(key(7, 2), DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY)).is_ok(), "the new generation owns the retired slot");
    registry.cancel(key(7, 2));
    for _ in 0..128 {
        if registry.is_empty() {
            break;
        }
        let _ = registry.close_step(1, DRAWING_GESTURE_RETAINED_BYTES);
    }
    assert!(registry.is_empty());
}

#[test]
fn drawing_gesture_interrupted_and_repeated_close_is_terminal_empty() {
    let mut owner = DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY);
    owner.cancel();
    owner.begin_close();
    assert_eq!(owner.close_step(0, DRAWING_GESTURE_RETAINED_BYTES), semio_framework_job::InteractiveJobCloseStep::Blocked);
    assert_eq!(owner.close_step(1, 0), semio_framework_job::InteractiveJobCloseStep::Blocked);
    assert_eq!(owner.close_step(1, DRAWING_GESTURE_RETAINED_BYTES), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: DRAWING_GESTURE_RETAINED_BYTES });
    assert_eq!(owner.close_step(1, DRAWING_GESTURE_RETAINED_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete);
    assert!(owner.terminal_is_empty());
}

/// 🧾️ The framework's close and maintenance pumps grant one `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES`
/// page per step; the owner must close under that grant — session dropped on the first page, the
/// declared budget handed back page by page, never a page over the grant.
#[test]
fn drawing_gesture_owner_closes_under_the_framework_page_grant() {
    let page = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    let mut owner = DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY);
    owner.begin_close();
    assert_eq!(owner.close_step(1, page), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: page });
    assert!(owner.session.is_none(), "the session is dropped on the first granted page");
    let mut released = page;
    let mut steps = 1;
    loop {
        match owner.close_step(1, page) {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                assert_eq!(released_items, 0);
                assert!(released_bytes > 0 && released_bytes <= page);
                released += released_bytes;
                steps += 1;
                assert!(steps <= DRAWING_GESTURE_RETAINED_BYTES / page + 1, "close terminates within the declared budget");
            }
            semio_framework_job::InteractiveJobCloseStep::Complete => break,
            other => panic!("unexpected close step {other:?}"),
        }
    }
    assert_eq!(released, DRAWING_GESTURE_RETAINED_BYTES, "the whole declared budget is handed back");
    assert!(owner.terminal_is_empty());
}

#[test]
fn drawing_retained_decoder_is_incremental_exact_and_fail_closed() {
    let pointer_move = wire(&DrawingCommand::CanvasPointerMove(crate::editor::drawing::commands::canvas_pointer_move::CanvasPointerMove { x: 1.0, y: 2.0, width: 10.0, height: 10.0, samples: Vec::new() }));
    let pointer_up = wire(&DrawingCommand::CanvasPointerUp(crate::editor::drawing::commands::canvas_pointer_up::CanvasPointerUp { x: 1.0, y: 2.0, width: 10.0, height: 10.0, shift: false, ctrl: false, meta: false, cancelled: false }));
    assert!(decode_retained("canvasPointerMove", &pointer_move));
    assert!(decode_retained("canvasEscape", &wire(&DrawingCommand::CanvasEscape(crate::editor::drawing::commands::canvas_escape::CanvasEscape {}))));
    assert!(!decode_retained("canvasPointerMove", &pointer_up), "a variant swap rejects");
    assert!(!decode_retained("canvasPointerMove", &pointer_move[..pointer_move.len() - 1]), "truncation rejects");
    assert!(!decode_retained("canvasPointerMove", &vec![0; DRAWING_GESTURE_RAW_BYTES + 1]), "an oversized owner rejects");
    assert!(!decode_retained("canvasPointerMove", br#"["canvasPointerMove",{"x":1,"y":2}]"#), "the retired JSON text shape is no longer a wire");
}

#[test]
fn drawing_preview_rejects_a_stale_revision_and_cancels_the_owner() {
    let mut owner = DrawingInstanceOperationOwner::new();
    let operation = key(9, 4);
    assert!(owner.operations.admit(operation, DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY)).is_ok());
    owner.active = Some((operation, [1; 32]));
    assert!(owner.preview_projection([2; 32], "selectDirect").is_none());
    assert!(owner.active.is_none());
    for _ in 0..128 {
        if owner.operations.is_empty() {
            break;
        }
        let _ = owner.operations.close_step(1, DRAWING_GESTURE_RETAINED_BYTES);
    }
    assert!(owner.operations.is_empty());
}

/// 🚪️ Owners admitted on BOTH cursor parities close through the instance owner's own close
/// protocol under the framework page grant — the registry's shared cursor must not strand a slot.
#[test]
fn drawing_instance_owner_close_reaches_every_slot_parity() {
    use semio_framework_plugin::ArtifactInstanceOperationOwner as _;
    let mut owner = DrawingInstanceOperationOwner::new();
    for operation in 0..3u64 {
        assert!(owner.operations.admit(key(operation, 0), DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY)).is_ok());
    }
    let page = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    for _ in 0..4_096 {
        if owner.terminal_is_empty() {
            break;
        }
        let step = owner.close_step(1, page).expect("instance owner close step");
        if let semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1 && released_bytes <= page, "one page per step: {step:?}");
        }
    }
    assert!(owner.terminal_is_empty(), "three owners across both cursor parities close under the page grant");
    assert!(owner.operations.is_empty());
}

/// 🎰️ The framework mints every tool operation id into the first vacant residue class of its own
/// 64-slot table, so consecutive settled gestures arrive as 256, 320, 384, … — all `≡ 0 (mod 64)`,
/// all addressing ONE direct-mapped slot here. A gesture whose predecessor is cancelled but not yet
/// retired must retire it and admit, never answer `saturated`.
#[test]
fn drawing_gesture_admission_retires_the_retiring_owner_in_its_residue_class() {
    let mut owner = DrawingInstanceOperationOwner::new();
    let first = key(256, 2);
    assert!(owner.operations.admit(first, DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY)).is_ok());
    owner.operations.cancel(first);
    let second = key(320, 3);
    assert!(!owner.operations.can_admit(second, DRAWING_GESTURE_RETAINED_BYTES), "the retiring predecessor still holds the residue class");
    for _ in 0..DRAWING_GESTURE_OPERATION_SLOTS * 2 {
        if owner.operations.can_admit(second, DRAWING_GESTURE_RETAINED_BYTES) {
            break;
        }
        let _ = owner.operations.close_step(1, DRAWING_GESTURE_RETAINED_BYTES);
    }
    assert!(owner.operations.can_admit(second, DRAWING_GESTURE_RETAINED_BYTES), "one bounded sweep retires the predecessor");
    assert!(owner.operations.admit(second, DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY)).is_ok());
    owner.operations.cancel(second);
    drain(&mut owner.operations);
}

/// 🔁️ A cancelled owner retires within a handful of maintenance steps under the page grant, not one
/// page per 64-slot cursor lap.
#[test]
fn drawing_gesture_maintenance_retires_a_cancelled_owner_promptly() {
    use semio_framework_plugin::ArtifactInstanceOperationOwner as _;
    let mut owner = DrawingInstanceOperationOwner::new();
    let live = key(448, 3);
    assert!(owner.operations.admit(live, DrawingGestureOperationOwner::new(DRAWING_DEFAULT_UTILITY)).is_ok());
    owner.operations.cancel(live);
    let page = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    let mut steps = 0;
    while !owner.operations.is_empty() {
        owner.maintenance_step(1, page).expect("gesture maintenance step");
        steps += 1;
        assert!(steps <= DRAWING_GESTURE_RETAINED_BYTES / page + 2, "the owner retires page by page without idle laps: {steps} steps");
    }
}

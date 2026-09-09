use super::*;
use semio_framework_job::InteractiveJob as _;

#[test]
fn drawing_completion_rejection_retires_child_before_decoder_without_redispatch() {
    let mut emit: Emit<DrawingMutation, DrawingConfigMutation, NoDraftMutation> = Emit::default();
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
    assert_eq!(job.close_step(0, 1), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 1 });
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
    for byte in wire {
        decoder.feed(*byte);
    }
    decoder.finish()
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
    assert_eq!(owner.close_step(1, DRAWING_GESTURE_RETAINED_BYTES - 1), semio_framework_job::InteractiveJobCloseStep::Blocked);
    assert_eq!(owner.close_step(1, DRAWING_GESTURE_RETAINED_BYTES), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: DRAWING_GESTURE_RETAINED_BYTES });
    assert_eq!(owner.close_step(1, DRAWING_GESTURE_RETAINED_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete);
    assert!(owner.terminal_is_empty());
}

#[test]
fn drawing_retained_decoder_is_incremental_exact_and_fail_closed() {
    assert!(decode_retained("canvasPointerMove", br#"["canvasPointerMove",{"x":1,"y":2}]"#));
    assert!(decode_retained("canvasEscape", br#"["canvasEscape",null]"#));
    assert!(!decode_retained("canvasPointerMove", br#"["canvasPointerUp",{"x":1}]"#));
    assert!(!decode_retained("canvasPointerMove", br#"["canvasPointerMove",{"x":1}"#));
    assert!(!decode_retained("canvasPointerMove", br#"["canvasPointerMove",{"x":1}]x"#));
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

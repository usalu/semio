use super::*;

#[semio_framework_async_macros::async_test]
async fn resolve_before_first_poll_leaves_the_future_immediately_ready() {
    let registry = RequestRegistry::new();
    let future = registry.request(|req| Effect::CancelJob { job: req.0 });
    assert_eq!(registry.drain().len(), 1, "request() must queue exactly one effect");
    registry.resolve(RequestId(1), Ok(b"ok".to_vec()));
    let mut future = Box::pin(future);
    let waker = futures_test_waker();
    let mut cx = Context::from_waker(waker);
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(Ok(bytes)) => assert_eq!(bytes, b"ok"),
        Poll::Ready(Err(fault)) => panic!("expected Ok, got a fault: {fault:?}"),
        Poll::Pending => panic!("a request resolved before its first poll must be immediately ready"),
    }
}

#[semio_framework_async_macros::async_test]
async fn append_chunk_reassembles_a_multi_chunk_body_to_the_exact_original_bytes() {
    let registry = RequestRegistry::new();
    let future = registry.request(|req| Effect::CancelJob { job: req.0 });
    let original: Vec<u8> = (0u8..=255).chain(0u8..=255).chain(0u8..100).collect(); // 710 bytes, non-trivial and non-uniform
    for (index, window) in original.chunks(97).enumerate() {
        let done = (index + 1) * 97 >= original.len();
        registry.append_chunk(RequestId(1), window, done, 1024 * 1024);
    }
    let mut future = Box::pin(future);
    let waker = futures_test_waker();
    let mut cx = Context::from_waker(waker);
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(Ok(bytes)) => assert_eq!(bytes, original, "every chunk (not just the final one) must survive reassembly"),
        Poll::Ready(Err(fault)) => panic!("expected Ok, got a fault: {fault:?}"),
        Poll::Pending => panic!("the request must be Ready once the final (done) chunk arrived"),
    }
}

#[semio_framework_async_macros::async_test]
async fn append_chunk_over_cap_faults_instead_of_silently_truncating() {
    let registry = RequestRegistry::new();
    let future = registry.request(|req| Effect::CancelJob { job: req.0 });
    registry.append_chunk(RequestId(1), &[0u8; 40], false, 64);
    registry.append_chunk(RequestId(1), &[0u8; 40], false, 64); // 80 > 64 cap — must fault here, not wait for `done`
    let mut future = Box::pin(future);
    let waker = futures_test_waker();
    let mut cx = Context::from_waker(waker);
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(Err(fault)) => assert_eq!(fault.code.0, "plugin.request-registry.body-too-large"),
        Poll::Ready(Ok(bytes)) => panic!("must fault over cap, not silently truncate — got {} bytes", bytes.len()),
        Poll::Pending => panic!("the over-cap chunk must resolve the request immediately, not wait for `done`"),
    }
}

#[semio_framework_async_macros::async_test]
async fn append_chunk_on_an_unknown_or_already_resolved_id_is_a_harmless_no_op() {
    let registry = RequestRegistry::new();
    registry.append_chunk(RequestId(999), &[1, 2, 3], false, 1024); // never requested
    let future = registry.request(|req| Effect::CancelJob { job: req.0 });
    registry.resolve(RequestId(1), Ok(b"already done".to_vec()));
    registry.append_chunk(RequestId(1), &[9, 9, 9], true, 1024); // arrives after resolve
    let mut future = Box::pin(future);
    let waker = futures_test_waker();
    let mut cx = Context::from_waker(waker);
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(Ok(bytes)) => assert_eq!(bytes, b"already done", "a late chunk must not clobber an already-resolved result"),
        other => panic!("expected the original resolution to stand, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn pending_ids_reports_only_unresolved_requests() {
    let registry = RequestRegistry::new();
    let _first = registry.request(|req| Effect::CancelJob { job: req.0 });
    let _second = registry.request(|req| Effect::CancelJob { job: req.0 });
    assert_eq!(registry.pending_ids().len(), 2);
    registry.resolve(RequestId(1), Ok(Vec::new()));
    assert_eq!(registry.pending_ids(), vec![RequestId(2)]);
}

#[semio_framework_async_macros::async_test]
async fn cancel_instance_removes_only_that_instances_pending_requests() {
    let registry = RequestRegistry::new();
    let scoped_to_7 = registry.for_instance(7);
    let scoped_to_9 = registry.for_instance(9);
    let _seven_a = scoped_to_7.request(|req| Effect::CancelJob { job: req.0 });
    let _seven_b = scoped_to_7.request(|req| Effect::CancelJob { job: req.0 });
    let nine = scoped_to_9.request(|req| Effect::CancelJob { job: req.0 });

    assert_eq!(registry.pending_ids().len(), 3, "all three requests share the one underlying queue");
    let before = registry.pending_ids().len();
    let mut cursor = registry.begin_cancel_instance(7);
    while registry.cancel_instance_step(&mut cursor) != RequestCloseStep::Complete {}
    let removed = before - registry.pending_ids().len();
    assert_eq!(removed, 2, "cancel_instance must report exactly the count it removed");
    assert_eq!(registry.pending_ids(), vec![RequestId(3)], "only instance 9's request must survive");

    // 🚫️ A cancelled instance's future observes neither Ready nor a wake — it simply never
    // resolves. The slot is gone outright (no leaked entry to poll against later).
    let mut nine = Box::pin(nine);
    let waker = futures_test_waker();
    let mut cx = Context::from_waker(waker);
    assert!(matches!(nine.as_mut().poll(&mut cx), Poll::Pending), "the surviving instance's request is unaffected");
}

#[semio_framework_async_macros::async_test]
async fn cancel_instance_on_an_instance_with_no_pending_requests_is_a_harmless_no_op() {
    let registry = RequestRegistry::new();
    let mut cursor = registry.begin_cancel_instance(42);
    while registry.cancel_instance_step(&mut cursor) != RequestCloseStep::Complete {}
    assert!(registry.pending_ids().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn for_instance_shares_the_same_id_counter_as_the_registry_it_was_derived_from() {
    let registry = RequestRegistry::new();
    let scoped = registry.for_instance(3);
    let _first = registry.request(|req| Effect::CancelJob { job: req.0 }); // id 1, instance 0
    let _second = scoped.request(|req| Effect::CancelJob { job: req.0 }); // id 2, instance 3
    let mut cursor = registry.begin_cancel_instance(3);
    while registry.cancel_instance_step(&mut cursor) != RequestCloseStep::Complete {}
    assert_eq!(registry.pending_ids(), vec![RequestId(1)]);
}

/// 🔁️ A continuation draws from the SAME id counter as a parked-future request and queues its
/// effect on the SAME outbound queue — that shared counter is what makes a minted `req` unable to
/// collide with a future's, which a hand-written `RequestId(105)` never guaranteed.
#[semio_framework_async_macros::async_test]
async fn request_continuation_queues_one_effect_and_shares_the_request_id_counter() {
    let registry = RequestRegistry::new();
    let _parked = registry.request(|req| Effect::CancelJob { job: req.0 }); // id 1
    let minted = registry.request_continuation("flowEvalResolve".to_string(), "{}".to_string(), |req| Effect::CancelJob { job: req.0 }).expect("continuation admission");
    assert_eq!(minted, RequestId(2), "a continuation must not restart or share the parked-future counter");
    assert_eq!(registry.drain().len(), 2, "both the parked request and the continuation queue exactly one effect each");
    assert_eq!(registry.pending_ids(), vec![RequestId(1)], "a continuation parks no future, so it is not a pending checkpoint id");
}

/// 🔁️ `take_continuation` answers exactly once, and only for a continuation id — a parked-future id
/// stays with `resolve`, which is what keeps the two delivery shapes from stealing each other's work.
#[semio_framework_async_macros::async_test]
async fn take_continuation_answers_once_and_never_claims_a_parked_future() {
    let registry = RequestRegistry::new();
    let scoped = registry.for_instance(5);
    let parked = registry.request(|req| Effect::CancelJob { job: req.0 }); // id 1, instance 0
    let minted = scoped.request_continuation("flowTessellateResolve".to_string(), r#"{"nodeHash":8}"#.to_string(), |req| Effect::CancelJob { job: req.0 }).expect("continuation admission");
    assert!(registry.take_continuation(RequestId(1)).is_none(), "a parked-future id is never a continuation");
    let taken = registry.take_continuation(minted).expect("the minted id owns a continuation");
    assert_eq!(taken.instance, 5, "the continuation remembers the instance whose handle minted it");
    assert_eq!(taken.response_action, "flowTessellateResolve");
    assert_eq!(taken.request_json, r#"{"nodeHash":8}"#);
    assert!(registry.take_continuation(minted).is_none(), "a continuation is consumed by its first taker");
    drop(parked);
}

/// 🔁️ `resolve` on a continuation id is inert — the continuation survives it intact, so a stray
/// completion route can never silently consume a redispatch that has not been dispatched yet.
#[semio_framework_async_macros::async_test]
async fn resolve_does_not_consume_or_corrupt_a_continuation() {
    let registry = RequestRegistry::new();
    let minted = registry.request_continuation("flowEvalResolve".to_string(), "{}".to_string(), |req| Effect::CancelJob { job: req.0 }).expect("continuation admission");
    registry.resolve(minted, Ok(b"ignored".to_vec()));
    assert_eq!(registry.take_continuation(minted).map(|taken| taken.response_action), Some("flowEvalResolve".to_string()));
}

/// 🔁️ `Event::InstanceClose` cancellation sweeps continuations exactly like parked requests: a
/// closed instance can never be redispatched into.
#[semio_framework_async_macros::async_test]
async fn cancel_instance_sweeps_a_continuation_of_that_instance_only() {
    let registry = RequestRegistry::new();
    let doomed = registry.for_instance(9).request_continuation("flowEvalResolve".to_string(), "{}".to_string(), |req| Effect::CancelJob { job: req.0 }).expect("continuation admission");
    let survivor = registry.for_instance(4).request_continuation("flowTessellateResolve".to_string(), "{}".to_string(), |req| Effect::CancelJob { job: req.0 }).expect("continuation admission");
    let mut cursor = registry.begin_cancel_instance(9);
    while registry.cancel_instance_step(&mut cursor) != RequestCloseStep::Complete {}
    assert!(registry.take_continuation(doomed).is_none(), "the closed instance's continuation must be gone");
    assert!(registry.take_continuation(survivor).is_some(), "another instance's continuation must be untouched");
}

// 🚫️async: E4 fn-pointer slot — Waker::noop() replaces the hand-rolled RawWakerVTable outright;
// no vtable fn-pointer slot survives to tag.
fn futures_test_waker() -> &'static Waker {
    Waker::noop()
}

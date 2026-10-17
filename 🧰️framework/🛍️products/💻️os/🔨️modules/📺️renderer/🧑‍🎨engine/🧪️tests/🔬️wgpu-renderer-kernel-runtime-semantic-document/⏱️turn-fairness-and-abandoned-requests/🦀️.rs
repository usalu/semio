/// ⏱️ Turn fairness and abandoned requests: the destroy and close fixtures, the turn slice and gap contracts and the abandonment of undelivered requests.

fn destroy_request(instance: u32) -> KernelRequest {
    KernelRequest::DestroyApp {
        owner: Arc::new(KernelCloseSubmission {
            instance,
            realm: false,
            generation: u64::from(instance) + 1,
            queue: Arc::new(KernelRequestQueue::default()),
            pool: crate::renderer_worker_pool(),
            registry: std::sync::Weak::new(),
            phase: std::sync::atomic::AtomicU8::new(KERNEL_CLOSE_QUEUED),
        }),
    }
}

fn close_submission(registry: &Arc<KernelCloseSubmissionRegistry>, instance: u32, generation: u64) -> Arc<KernelCloseSubmission> {
    Arc::new(KernelCloseSubmission {
        instance,
        realm: false,
        generation,
        queue: Arc::new(KernelRequestQueue::default()),
        pool: crate::renderer_worker_pool(),
        registry: Arc::downgrade(registry),
        phase: std::sync::atomic::AtomicU8::new(KERNEL_CLOSE_UNADMITTED),
    })
}

fn turn_fairness() -> serde_json::Value {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️kernel-pool-future/🔣️.json")).expect("neutral kernel fairness contract");
    fixture["turnFairness"].clone()
}

/// ⚖️ A slice is the contract's fuel and wall, and one slice gap serves the contract's number of requests.
#[test]
fn the_turn_slice_and_its_gap_are_the_fairness_contracts() {
    let fairness = turn_fairness();
    assert_eq!(TURN_BUDGET.fuel, fairness["sliceFuel"].as_u64().expect("slice fuel"));
    assert_eq!(u64::from(TURN_BUDGET.deadline_ms), fairness["sliceWallMs"].as_u64().expect("slice wall"));
    assert_eq!(TURN_REQUESTS_BETWEEN_SLICES as u64, fairness["requestsBetweenSlices"].as_u64().expect("requests between slices"));
}

/// ⚖️ Between two slices of a turn mid-flight on `busy`, the head request runs only when it belongs to another
/// instance and owns its outcome; a close waits for a whole-request boundary; a realm close at the head
/// cancels the turn; FIFO and command credits are untouched by a head that may not run.
#[test]
fn a_slice_gap_serves_only_another_instances_head_request_and_a_realm_close_cancels() {
    for gap in turn_fairness()["gaps"].as_array().expect("gaps") {
        let queue = KernelRequestQueue::default();
        let busy = gap["busy"].as_u64().expect("busy") as u32;
        let instance = gap["headInstance"].as_u64().expect("head instance") as u32;
        let head = match gap["head"].as_str() {
            Some("exchangeCommands") => Some(command_request(instance, 7, 1)),
            Some("destroyApp") => Some(destroy_request(instance)),
            Some("closeRealm") => Some(KernelRequest::CloseRealm { owner: close_submission(&Arc::new(KernelCloseSubmissionRegistry::new()), instance, 3) }),
            _ => None,
        };
        if let Some(head) = head {
            queue.try_push(head, Arc::new(ResponseSlot::default()), None).unwrap_or_else(|_| panic!("fairness gap admission"));
        }
        let cancels = queue.head_is(|request| matches!(request, KernelRequest::CloseRealm { .. }));
        let served = queue.try_next_if(|request| kernel_request_interleavable(request, busy));
        assert_eq!((served.is_some(), cancels), (gap["served"].as_bool().unwrap(), gap["cancels"].as_bool().unwrap()), "{}", gap["id"]);
        if served.is_none() && gap["head"].is_string() {
            assert!(queue.try_next().is_some(), "{}: a head that may not run stays the head", gap["id"]);
        }
    }
}

/// 🛑️ A request is abandoned — its turn cancellable — exactly when its requester dropped the future after the
/// queue admitted it and before its outcome arrived.
#[test]
fn a_kernel_request_is_abandoned_only_when_dropped_undelivered_after_admission() {
    for case in turn_fairness()["abandonment"].as_array().expect("abandonment") {
        let queue = Arc::new(KernelRequestQueue::default());
        let slot = Arc::new(ResponseSlot::default());
        let mut future = KernelFuture { slot: slot.clone(), request: Some(destroy_request(9)), queue: queue.clone(), finished: false };
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        if case["admitted"].as_bool().unwrap() {
            assert!(Pin::new(&mut future).poll(&mut context).is_pending(), "{}", case["id"]);
            assert!(queue.try_next().is_some(), "{}: admitted", case["id"]);
        }
        if case["delivered"].as_bool().unwrap() {
            slot.deliver(KernelOutcome::Created(Ok(9)));
            assert!(Pin::new(&mut future).poll(&mut context).is_ready(), "{}", case["id"]);
        }
        drop(future);
        assert_eq!(slot.abandoned(), case["abandoned"].as_bool().unwrap(), "{}", case["id"]);
    }
}

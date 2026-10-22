/// 🧵️ Slot tables, pool future and delivery wake: the heap first slot tables on a bounded stack, the pool future's owner lock and the response delivery wake.

/// 🧱️ The `boxed_fixed_slots` law for this module's fixed slot tables, against the one committed
/// budget every implementation of it reads (`the committed fixed-slot fixture`).
///
/// Asserts the measured shape of each table (capacity, one slot's bytes, the owner's own bytes)
/// against that record, that each owner is smaller than the table it owns — the structural proof the
/// slots are heap-first rather than an inline `[T; N]` field — and then constructs them on a thread
/// holding only the fixture's `boundedThreadStackBytes`. `Builder::stack_size` overrides
/// `RUST_MIN_STACK`, so the repo runner's 128 MiB floor cannot hide a re-inflated frame here.
#[test]
fn kernel_runtime_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json")).expect("🧱️ the committed fixed-slot-table budget parses");
    let declared: Vec<semio_framework_async_slots::FixedSlotTableBudget> = fixture["tables"]
        .as_array()
        .expect("🧱️ the budget lists its tables")
        .iter()
        .filter(|table| table["guard"] == "renderer::kernel_runtime")
        .map(|table| semio_framework_async_slots::FixedSlotTableBudget::new(table["owner"].as_str().expect("owner"), table["capacity"].as_u64().expect("capacity") as usize, table["elementSizeBytes"].as_u64().expect("element bytes") as usize, table["ownerSizeBytes"].as_u64().expect("owner bytes") as usize))
        .collect();
    let measured = vec![
        semio_framework_async_slots::FixedSlotTableBudget::new("kernel_runtime::RetainedSurfaceRegistry", RETAINED_SURFACE_CAPACITY, size_of::<Option<RetainedSurfaceSlot>>(), size_of::<RetainedSurfaceRegistry>()),
        semio_framework_async_slots::FixedSlotTableBudget::new("kernel_runtime::CommandDocumentRetirementRegistry", COMMAND_DOCUMENT_RETIREMENT_CAPACITY, size_of::<Option<CommandDocumentRetirementState>>(), size_of::<CommandDocumentRetirementRegistry>()),
    ];
    semio_framework_async_slots::assert_fixed_slot_tables(
        "renderer::kernel_runtime",
        fixture["boundedThreadStackBytes"].as_u64().expect("bounded stack budget") as usize,
        fixture["conversionThresholdBytes"].as_u64().expect("conversion threshold") as usize,
        &declared,
        &measured,
        || {
            drop(RetainedSurfaceRegistry::new());
            drop(CommandDocumentRetirementRegistry::new());
        },
    );
}

#[test]
fn kernel_pool_future_releases_its_owner_lock_before_polling_and_resumes_every_wake() {
    fn trace_future(pending_wakes: usize, output: std::sync::mpsc::SyncSender<Vec<&'static str>>) -> impl Future<Output = ()> + Send {
        let mut trace = Vec::with_capacity(pending_wakes + 1);
        std::future::poll_fn(move |context| {
            if trace.len() < pending_wakes {
                trace.push("pending-wake");
                context.waker().wake_by_ref();
                Poll::Pending
            } else {
                trace.push("ready");
                output.send(std::mem::take(&mut trace)).expect("trace receiver remains live");
                Poll::Ready(())
            }
        })
    }

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️kernel-pool-future/🔣️.json")).expect("neutral kernel future trace");
    let runtime = tokio::runtime::Builder::new_current_thread().build().expect("third-party future oracle");
    for case in fixture["cases"].as_array().expect("trace cases") {
        let pending = case["pendingWakes"].as_u64().expect("pending wake count") as usize;
        let expected = case["expectedPolls"].as_array().expect("expected trace").iter().map(|value| value.as_str().expect("poll outcome")).collect::<Vec<_>>();
        let (oracle_output, oracle_input) = std::sync::mpsc::sync_channel(1);
        runtime.block_on(trace_future(pending, oracle_output));
        let oracle = oracle_input.recv().expect("oracle trace");
        assert_eq!(oracle, expected, "neutral trace {}", case["id"]);
        let (output, input) = std::sync::mpsc::sync_channel(1);
        let task = KernelPoolFuture::spawn(crate::renderer_worker_pool(), semio_framework_async::Lane::Interactive, trace_future(pending, output));
        let actual = input.recv_timeout(std::time::Duration::from_secs(2)).expect("a pending future must release its lock and resume its admitted wake");
        assert_eq!(actual, oracle, "kernel trace {}", case["id"]);
        drop(task);
    }
}

#[test]
fn kernel_response_delivery_never_parks_without_a_wake() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{RawWaker, RawWakerVTable};

    struct DeliveryWake {
        on_clone: Mutex<Option<Box<dyn FnOnce() + Send>>>,
        wakes: AtomicUsize,
    }

    unsafe fn clone_wake(pointer: *const ()) -> RawWaker {
        let owner = std::mem::ManuallyDrop::new(unsafe { Arc::from_raw(pointer.cast::<DeliveryWake>()) });
        let deliver = owner.on_clone.lock().unwrap().take();
        if let Some(deliver) = deliver {
            deliver();
        }
        RawWaker::new(Arc::into_raw(Arc::clone(&owner)).cast(), &VTABLE)
    }

    unsafe fn wake(pointer: *const ()) {
        let owner = unsafe { Arc::from_raw(pointer.cast::<DeliveryWake>()) };
        owner.wakes.fetch_add(1, Ordering::SeqCst);
    }

    unsafe fn wake_ref(pointer: *const ()) {
        let owner = std::mem::ManuallyDrop::new(unsafe { Arc::from_raw(pointer.cast::<DeliveryWake>()) });
        owner.wakes.fetch_add(1, Ordering::SeqCst);
    }

    unsafe fn drop_wake(pointer: *const ()) {
        drop(unsafe { Arc::from_raw(pointer.cast::<DeliveryWake>()) });
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_wake, wake, wake_ref, drop_wake);

    fn observe(future: impl Future<Output = u32>, delivery: Box<dyn FnOnce() + Send>, boundary: &str) -> (u32, usize) {
        let state = Arc::new(DeliveryWake { on_clone: Mutex::new(None), wakes: AtomicUsize::new(0) });
        let mut deliver = Some(delivery);
        match boundary {
            "before-poll" => deliver.take().unwrap()(),
            "register-waker" => *state.on_clone.lock().unwrap() = deliver.take(),
            "after-pending" => {}
            _ => panic!("unknown neutral delivery boundary"),
        }
        let waker = unsafe { Waker::from_raw(RawWaker::new(Arc::into_raw(state.clone()).cast(), &VTABLE)) };
        let mut context = Context::from_waker(&waker);
        let mut future = std::pin::pin!(future);
        let first = future.as_mut().poll(&mut context);
        if boundary == "after-pending" {
            assert!(first.is_pending(), "delivery has not occurred");
            deliver.take().unwrap()();
        }
        let actual = match first {
            Poll::Ready(value) => value,
            Poll::Pending => {
                assert!(state.wakes.load(Ordering::SeqCst) > 0, "{boundary}: a delivered response must wake its pending consumer");
                let Poll::Ready(value) = future.as_mut().poll(&mut context) else { panic!("{boundary}: delivered response must be ready") };
                value
            }
        };
        (actual, state.wakes.load(Ordering::SeqCst))
    }

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️kernel-pool-future/🔣️.json")).expect("neutral kernel response traces");
    for case in fixture["responses"].as_array().expect("response cases") {
        let boundary = case["delivery"].as_str().unwrap();
        let expected = case["value"].as_u64().unwrap() as u32;
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let oracle = observe(async move { receiver.await.expect("third-party response") }, Box::new(move || sender.send(expected).expect("oracle receiver remains live")), boundary);
        assert_eq!(oracle.0, expected);
        let slot = Arc::new(ResponseSlot::default());
        let producer = slot.clone();
        let future = KernelFuture { slot, request: None, queue: Arc::new(KernelRequestQueue::default()), finished: false };
        let actual = observe(
            async move {
                match future.await {
                    KernelOutcome::Created(Ok(value)) => value,
                    _ => panic!("neutral response outcome"),
                }
            },
            Box::new(move || producer.deliver(KernelOutcome::Created(Ok(expected)))),
            boundary,
        );
        assert_eq!(actual.0, oracle.0, "delivery {boundary}");
    }
}

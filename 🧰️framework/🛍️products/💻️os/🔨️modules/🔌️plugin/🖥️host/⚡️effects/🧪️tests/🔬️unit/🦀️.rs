use super::*;
use semio_framework_async::testkit::ManualRuntime;
use std::sync::atomic::AtomicUsize;

async fn services<R: HostAsyncRuntime + 'static>(runtime: Arc<R>) -> AsyncServices<R> {
    AsyncServices {
        runtime: runtime.clone(),
        http: Arc::new(HttpPool::new(Arc::new(semio_framework_os_services::UnwiredHttpTransport), Arc::new(ComputePool::new(4).await), 1_000_000, 8).await),
        storage: Arc::new(StorageScheduler::new(runtime.clone(), runtime.open_scope(ScopeOwner::Service("test-storage"), None).await, 4, 1_000_000).await),
        timers: Arc::new(TimerWheel::new(16).await),
        events: Arc::new(EventRouter::new()),
        compute: Arc::new(ComputePool::new(4).await),
        storage_backend: Arc::new(RecordingStorageBackend::default()),
    }
}

#[derive(Default)]
struct RecordingStorageBackend {
    writes: Mutex<Vec<(String, Vec<u8>)>>,
}
impl StorageBackend for RecordingStorageBackend {
    fn read(&self, key: &str) -> Result<Vec<u8>, std::io::Error> {
        Ok(format!("read:{key}").into_bytes())
    }
    fn write(&self, key: &str, bytes: &[u8]) -> Result<(), std::io::Error> {
        self.writes.lock().unwrap().push((key.to_string(), bytes.to_vec()));
        Ok(())
    }
    fn delete(&self, _key: &str) -> Result<(), std::io::Error> {
        Ok(())
    }
}

struct RecordingRouterHandler(Arc<AtomicUsize>);

struct RecordingRouterJob {
    calls: Option<Arc<AtomicUsize>>,
    yielded: bool,
    closing: bool,
}

impl InteractiveJob for RecordingRouterJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if cx.should_yield() {
            return StepOutcome::Yield;
        }
        cx.consume_fuel(1);
        if !self.yielded {
            self.yielded = true;
            return StepOutcome::Yield;
        }
        self.calls.as_ref().expect("recording router calls").fetch_add(1, Ordering::SeqCst);
        let output = cx.payload_from_bytes(JobPayloadStream::CommitOutput, b"ok").unwrap_or_else(|_| RetainedJobPayload::empty(JobPayloadStream::CommitOutput));
        StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.begin_close();
        if self.calls.is_some() {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.calls = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.calls.is_none()
    }
}

impl RouterEffectHandler for RecordingRouterHandler {
    fn create_job(&self, _effect: RouterEffect) -> Box<dyn InteractiveJob + Send> {
        Box::new(RecordingRouterJob { calls: Some(self.0.clone()), yielded: false, closing: false })
    }
}

async fn executor<R: HostAsyncRuntime + 'static>(runtime: Arc<R>) -> (AsyncEffectExecutor<RecordingEnvelopeInjector, R>, RecordingEnvelopeInjector, ActorScopeRegistry) {
    let actors = ActorScopeRegistry::new();
    let events = Arc::new(EventRouter::new());
    let injector = RecordingEnvelopeInjector::new().await;
    let sink = Arc::new(EnvelopeCompletionSink::new(actors.clone(), events.clone(), Arc::new(injector.clone())).await);
    let backbone = Arc::new(BackboneRegistry::new(events.clone(), Arc::new(AllowAllCapabilities)).await);
    let mut svc = services(runtime).await;
    svc.events = events;
    let executor = AsyncEffectExecutor::new(svc, actors.clone(), CapabilityRevocationRegistry::new(), sink, backbone, Arc::new(UnwiredRouterEffectHandler), Arc::new(NullMetricsRecorder)).await;
    (executor, injector, actors)
}

async fn activate<R: HostAsyncRuntime>(executor: &AsyncEffectExecutor<RecordingEnvelopeInjector, R>, actors: &ActorScopeRegistry, runtime: &R, actor: u64, generation: u16) -> ScopeHandle {
    let package_scope = runtime.open_scope(ScopeOwner::Package("pkg".to_string()), None).await;
    let scope = actors.activate(runtime, actor, generation, &package_scope).await;
    let _ = executor;
    scope
}

//#region 🔑️CapabilityRevocationTests
/// 🔑️ Bench budget 8: a revoked capability cancels ONLY the operations holding it — the actor
/// survives, and the revoked operation's own completion carries a `capability-revoked` error
/// while a SIBLING operation (different capability) completes normally.
#[semio_framework_async_macros::async_test]
async fn revoked_capability_cancels_only_its_own_operations_and_actor_survives() {
    let runtime = ManualRuntime::new(0).await;
    let runtime_dyn: Arc<ManualRuntime> = Arc::new(runtime.clone());
    let (mut executor, injector, actors) = executor(runtime_dyn.clone()).await;
    let scope = activate(&executor, &actors, runtime_dyn.as_ref(), 1, 0).await;
    // 🐛️ `executor()`'s default `UnwiredRouterEffectHandler` always returns `Err` — this test
    // needs the NON-revoked operation to actually succeed so the two completions are
    // distinguishable by more than "which one happened to fail", so it swaps in a handler that
    // always succeeds.
    struct AlwaysOkRouterHandler;
    impl RouterEffectHandler for AlwaysOkRouterHandler {
        fn create_job(&self, _effect: RouterEffect) -> Box<dyn InteractiveJob + Send> {
            Box::new(CompleteRouterEffectJob { output: Some(b"ok".to_vec()), writer: None, cursor: 0, closing: false })
        }
    }
    executor.router_handler = Arc::new(AlwaysOkRouterHandler);

    let revoked_cap = CapabilityTokenId(1);
    let kept_cap = CapabilityTokenId(2);

    let dispatch_revoked = EffectDispatchContext { actor: 1, package: PackageId("pkg".to_string()), lane: 0, capability: Some(revoked_cap) };
    let dispatch_kept = EffectDispatchContext { actor: 1, package: PackageId("pkg".to_string()), lane: 0, capability: Some(kept_cap) };

    executor.execute(&dispatch_revoked, &[Effect::BlobLoad { req: RequestId(1), hash: "h1".to_string() }]).await;
    executor.execute(&dispatch_kept, &[Effect::BlobLoad { req: RequestId(2), hash: "h2".to_string() }]).await;

    executor.revoke_capability(revoked_cap).await;
    runtime.drive().await;

    let recorded = injector.recorded().await;
    assert_eq!(recorded.len(), 2, "both operations must complete — one with an error, one normally");
    let find_result = |req: u64| -> RequestOutcome {
        let envelope = recorded.iter().find(|e| matches!(&e.payload, Payload::Event { bytes } if matches!(serde_json::from_slice::<Event>(bytes), Ok(Event::Completed { req: r, .. }) if r.0 == req))).expect("completion for req must exist");
        match &envelope.payload {
            Payload::Event { bytes } => match serde_json::from_slice::<Event>(bytes).unwrap() {
                Event::Completed { result, .. } => result,
                other => panic!("expected Completed, got {other:?}"),
            },
            _ => panic!("expected Payload::Event"),
        }
    };
    assert!(matches!(find_result(1), RequestOutcome::Err(_)), "the revoked operation must complete with an error");
    assert!(matches!(find_result(2), RequestOutcome::Ok(_)), "the sibling operation (different capability) must complete normally");
    assert!(scope.cancel.is_live().await, "the actor's own scope token must be untouched by a capability revocation");
}
//#endregion 🔑️CapabilityRevocationTests

//#region 💾️QuotaTests
/// 💾️ A quota denial (storage byte budget exceeded) must produce a typed completion, never a
/// panic.
#[semio_framework_async_macros::async_test]
async fn storage_quota_denial_produces_a_typed_completion_not_a_panic() {
    let runtime = ManualRuntime::new(0).await;
    let runtime_dyn: Arc<ManualRuntime> = Arc::new(runtime.clone());
    let mut svc = services(runtime_dyn.clone()).await;
    svc.storage = Arc::new(StorageScheduler::new(runtime_dyn.clone(), runtime_dyn.open_scope(ScopeOwner::Service("tiny-storage"), None).await, 4, 4).await);
    let actors = ActorScopeRegistry::new();
    let events = Arc::new(EventRouter::new());
    svc.events = events.clone();
    let injector = RecordingEnvelopeInjector::new().await;
    let sink = Arc::new(EnvelopeCompletionSink::new(actors.clone(), events.clone(), Arc::new(injector.clone())).await);
    let backbone = Arc::new(BackboneRegistry::new(events, Arc::new(AllowAllCapabilities)).await);
    let executor = AsyncEffectExecutor::new(svc, actors.clone(), CapabilityRevocationRegistry::new(), sink, backbone, Arc::new(UnwiredRouterEffectHandler), Arc::new(NullMetricsRecorder)).await;

    let package_scope = runtime_dyn.open_scope(ScopeOwner::Package("pkg".to_string()), None).await;
    actors.activate(runtime_dyn.as_ref(), 1, 0, &package_scope).await;

    let dispatch = EffectDispatchContext { actor: 1, package: PackageId("pkg".to_string()), lane: 0, capability: None };
    let big_bytes = vec![0u8; 1_000];
    executor.execute(&dispatch, &[Effect::StorageWrite { req: RequestId(9), key: "k".to_string(), bytes: big_bytes }]).await;
    runtime.drive().await;

    let recorded = injector.recorded().await;
    assert_eq!(recorded.len(), 1);
    match &recorded[0].payload {
        Payload::Event { bytes } => match serde_json::from_slice::<Event>(bytes).unwrap() {
            Event::Completed { result: RequestOutcome::Err(_), .. } => {}
            other => panic!("expected a typed Err completion, got {other:?}"),
        },
        _ => panic!("expected Payload::Event"),
    }
}
//#endregion 💾️QuotaTests

//#region 🪪️GenerationGatingTests
#[semio_framework_async_macros::async_test]
async fn stale_generation_completion_is_dropped_current_generation_is_delivered() {
    let runtime = ManualRuntime::new(0).await;
    let actors = ActorScopeRegistry::new();
    let events = Arc::new(EventRouter::new());
    let injector = RecordingEnvelopeInjector::new().await;
    let sink = EnvelopeCompletionSink::new(actors.clone(), events, Arc::new(injector.clone())).await;
    let package_scope = runtime.open_scope(ScopeOwner::Service("pkg"), None).await;
    actors.activate(&runtime, 42, 5, &package_scope).await;

    sink.complete(42, 3, encode_event(&Event::Timer { id: 1 }).await, 0);
    assert!(injector.recorded().await.is_empty(), "a completion addressed to a stale generation must be dropped");

    sink.complete(42, 5, encode_event(&Event::Timer { id: 2 }).await, 0);
    let recorded = injector.recorded().await;
    assert_eq!(recorded.len(), 1, "a completion addressed to the CURRENT generation must be delivered");
    match &recorded[0].payload {
        Payload::Event { bytes } => assert_eq!(serde_json::from_slice::<Event>(bytes).unwrap(), Event::Timer { id: 2 }),
        _ => panic!("expected Payload::Event"),
    }
}
//#endregion 🪪️GenerationGatingTests

//#region ⏸️ParkBufferTests
#[semio_framework_async_macros::async_test]
async fn park_buffers_completions_and_resume_delivers_them_in_order() {
    let runtime = ManualRuntime::new(0).await;
    let actors = ActorScopeRegistry::new();
    let events = Arc::new(EventRouter::new());
    let injector = RecordingEnvelopeInjector::new().await;
    let sink = EnvelopeCompletionSink::new(actors.clone(), events, Arc::new(injector.clone())).await;
    let package_scope = runtime.open_scope(ScopeOwner::Service("pkg"), None).await;
    let scope = actors.activate(&runtime, 7, 0, &package_scope).await;

    scope.cancel.park().await;
    sink.complete(7, 0, encode_event(&Event::Timer { id: 1 }).await, 0);
    sink.complete(7, 0, encode_event(&Event::Timer { id: 2 }).await, 0);
    sink.complete(7, 0, encode_event(&Event::Timer { id: 3 }).await, 0);
    assert!(injector.recorded().await.is_empty(), "a parked actor's completions must be buffered, never delivered while parked");

    scope.cancel.unpark().await;
    sink.flush(7).await;

    let recorded = injector.recorded().await;
    assert_eq!(recorded.len(), 3, "resume must deliver every buffered completion");
    let ids: Vec<u64> = recorded
        .iter()
        .map(|envelope| match &envelope.payload {
            Payload::Event { bytes } => match serde_json::from_slice::<Event>(bytes).unwrap() {
                Event::Timer { id } => id,
                other => panic!("expected Event::Timer, got {other:?}"),
            },
            _ => panic!("expected Payload::Event"),
        })
        .collect();
    assert_eq!(ids, vec![1, 2, 3], "buffered completions must be delivered in the order they completed");
}
//#endregion ⏸️ParkBufferTests

//#region 🚰️BackpressureTests
/// 🚰️ A completion burst is subject to the SAME mailbox bound every other channel honours —
/// proves the BOUND (delivered count never exceeds the cap), not the internal mechanism.
#[semio_framework_async_macros::async_test]
async fn completion_burst_while_parked_is_bounded_not_unbounded() {
    let runtime = ManualRuntime::new(0).await;
    let actors = ActorScopeRegistry::new();
    let events = Arc::new(EventRouter::new());
    let injector = RecordingEnvelopeInjector::new().await;
    let sink = EnvelopeCompletionSink::new(actors.clone(), events, Arc::new(injector.clone())).await;
    let package_scope = runtime.open_scope(ScopeOwner::Service("pkg"), None).await;
    let scope = actors.activate(&runtime, 3, 0, &package_scope).await;
    scope.cancel.park().await;

    let burst = COMPLETION_MAILBOX_CAP as u64 + 200;
    for id in 0..burst {
        sink.complete(3, 0, encode_event(&Event::Timer { id }).await, 0);
    }
    scope.cancel.unpark().await;
    sink.flush(3).await;

    let delivered = injector.recorded().await.len() as u32;
    assert!(delivered <= COMPLETION_MAILBOX_CAP, "a completion burst of {burst} must never deliver more than the mailbox cap {COMPLETION_MAILBOX_CAP}, got {delivered}");
    assert!(delivered > 0, "a bounded mailbox must still deliver what it DID accept, not reject everything");
}
//#endregion 🚰️BackpressureTests

//#region 🚀️ClassificationTests
#[semio_framework_async_macros::async_test]
async fn spawn_job_and_cancel_job_are_reported_shard_owned_never_dispatched() {
    let runtime = ManualRuntime::new(0).await;
    let runtime_dyn: Arc<ManualRuntime> = Arc::new(runtime.clone());
    let (executor, injector, actors) = executor(runtime_dyn.clone()).await;
    activate(&executor, &actors, runtime_dyn.as_ref(), 1, 0).await;
    let dispatch = EffectDispatchContext { actor: 1, package: PackageId("pkg".to_string()), lane: 0, capability: None };
    let report = executor.execute(&dispatch, &[Effect::SpawnJob { job: 1, kind: "k".to_string(), input: vec![], placement: semio_framework::kernel::JobPlacement::Inline }, Effect::CancelJob { job: 1 }]).await;
    runtime.drive().await;
    assert_eq!(report.shard_owned, 2);
    assert_eq!(report.dispatched, 0);
    assert!(injector.recorded().await.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn shell_effects_are_reported_shell_owned_never_dispatched() {
    let runtime = ManualRuntime::new(0).await;
    let runtime_dyn: Arc<ManualRuntime> = Arc::new(runtime.clone());
    let (executor, _injector, actors) = executor(runtime_dyn.clone()).await;
    activate(&executor, &actors, runtime_dyn.as_ref(), 1, 0).await;
    let dispatch = EffectDispatchContext { actor: 1, package: PackageId("pkg".to_string()), lane: 0, capability: None };
    let report = executor.execute(&dispatch, &[Effect::Notify { message: "hi".to_string() }]).await;
    assert_eq!(report.shell_owned, 1);
    assert_eq!(report.dispatched, 0);
}

#[semio_framework_async_macros::async_test]
async fn router_effect_runs_through_the_retained_compute_session() {
    let runtime = ManualRuntime::new(0).await;
    let runtime_dyn: Arc<ManualRuntime> = Arc::new(runtime.clone());
    let (mut executor, injector, actors) = executor(runtime_dyn.clone()).await;
    activate(&executor, &actors, runtime_dyn.as_ref(), 1, 0).await;
    let recording_handler = Arc::new(RecordingRouterHandler(Arc::new(AtomicUsize::new(0))));
    executor.router_handler = recording_handler.clone();
    let dispatch = EffectDispatchContext { actor: 1, package: PackageId("pkg".to_string()), lane: 0, capability: None };
    executor.execute(&dispatch, &[Effect::CacheRead { req: RequestId(5), engine_id: "e".to_string(), key: "k".to_string() }]).await;
    runtime.drive().await;
    assert_eq!(recording_handler.0.load(Ordering::SeqCst), 1);
    let recorded = injector.recorded().await;
    assert_eq!(recorded.len(), 1);
    assert!(matches!(&recorded[0].payload, Payload::Event { bytes } if matches!(serde_json::from_slice::<Event>(bytes), Ok(Event::Completed { req: RequestId(5), result: RequestOutcome::Ok(output) }) if output == b"ok")));
}

#[semio_framework_async_macros::async_test]
async fn router_effect_on_a_stopped_compute_pool_returns_worker_lost_without_stranding_its_owner() {
    let runtime = ManualRuntime::new(0).await;
    let scope = runtime.open_scope(ScopeOwner::Service("stopped-router-compute"), None).await;
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    pool.shutdown();
    let compute = ComputePool::with_pool(1, pool);
    let ctx = OperationContext { actor: 1, generation: 0, trace: TraceId(91), lane: 0, deadline_ms: None, cancel: CancelToken::root().await, capability: None };
    let handler: Arc<dyn RouterEffectHandler> = Arc::new(RecordingRouterHandler(Arc::new(AtomicUsize::new(0))));
    let outcome = run_router_effect_job(&compute, &runtime, &scope, ctx, &handler, RouterEffect::CacheRead { engine_id: "e".to_string(), key: "k".to_string() }).await;
    assert!(matches!(outcome, RouterEffectJobOutcome::WorkerLost));
}
//#endregion 🚀️ClassificationTests

//#region 📡️BackboneTests
#[semio_framework_async_macros::async_test]
async fn backbone_send_is_rejected_without_the_capability() {
    struct DenyAll;
    impl CapabilityChecker for DenyAll {
        fn is_granted(&self, _actor: u64, _scope: &str) -> bool {
            false
        }
    }
    let events = Arc::new(EventRouter::new());
    let registry = BackboneRegistry::new(events, Arc::new(DenyAll)).await;
    let result = registry.send(1, "studio-42", b"payload").await;
    assert_eq!(result, Err(BackboneError::CapabilityDenied { uri: "studio-42".to_string() }));
}

#[semio_framework_async_macros::async_test]
async fn backbone_send_reaches_the_registered_transport_once_granted() {
    #[derive(Default)]
    struct RecordingTransport(Mutex<Vec<Vec<u8>>>);
    impl BackboneTransport for RecordingTransport {
        fn send(&self, _uri: &str, payload: &[u8]) -> Result<(), std::io::Error> {
            self.0.lock().unwrap().push(payload.to_vec());
            Ok(())
        }
    }
    let events = Arc::new(EventRouter::new());
    let registry = BackboneRegistry::new(events, Arc::new(AllowAllCapabilities)).await;
    let transport = Arc::new(RecordingTransport::default());
    registry.register("studio-42".to_string(), transport.clone()).await;
    registry.send(1, "studio-42", b"payload").await.expect("granted send must succeed");
    assert_eq!(*transport.0.lock().unwrap(), vec![b"payload".to_vec()]);
}

#[semio_framework_async_macros::async_test]
async fn backbone_delta_fanout_coalesces_a_burst_for_the_same_uri() {
    let events = Arc::new(EventRouter::new());
    let registry = BackboneRegistry::new(events.clone(), Arc::new(AllowAllCapabilities)).await;
    let topic = Topic("backbone.delta.studio-42".to_string());
    let actor = semio_framework_actor::ActorId(1);
    events.subscribe(topic.clone(), actor, ChannelPolicy::Coalesced { key: "studio-42".to_string(), max_items: 100, max_bytes: 1_000_000 });
    registry.fanout_delta("studio-42", b"delta-1".to_vec()).await;
    registry.fanout_delta("studio-42", b"delta-2".to_vec()).await;
    let drained = events.drain(&topic, actor).await;
    assert_eq!(drained, vec![b"delta-2".to_vec()], "a burst of deltas for the SAME uri must collapse to the latest");
}
//#endregion 📡️BackboneTests

use super::*;

fn fixture_router_grant()->semio_framework_job::RetainedCloneGrant{let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../🛎️services/🧫️fixtures/🧮️compute-retained/🔣️.json")).unwrap();serde_json::from_value(law["callerGrant"].clone()).unwrap()}
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
    cancelled: bool,
    completed: bool,
    cursor: usize,
    writer: Option<RetainedPayloadBuilder>,
}

impl InteractiveJob for RecordingRouterJob {
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,semio_framework_value::ValueError>{
        if cx.is_cancelled()||self.closing{let grant=cx.retained_grant();if grant.maximum_items<2||grant.maximum_copy_bytes<std::mem::size_of::<bool>()||grant.maximum_depth==0{return Ok(None)}self.cancelled=true;cx.consume_retained(semio_framework_job::RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<bool>(),..Default::default()})?;return JobOutcomeBorrow::admit_cancelled(cx)}
        if cx.should_yield(){return Ok(None)}
        cx.consume_fuel(1);
        if !self.yielded {
            let grant=cx.retained_grant();if grant.maximum_items<2||grant.maximum_copy_bytes<std::mem::size_of::<bool>()||grant.maximum_depth==0{return Ok(None)}
            self.yielded = true;
            cx.consume_retained(semio_framework_job::RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<bool>(),..Default::default()})?;
            return JobOutcomeBorrow::admit_yield(cx);
        }
        if self.writer.is_none(){let grant=cx.retained_grant();let copied_bytes=std::mem::size_of::<Option<RetainedPayloadBuilder>>();if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Ok(None)}self.writer=Some(RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput));cx.consume_retained(semio_framework_job::RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()})?;return Ok(None)}
        let writer=self.writer.as_mut().unwrap();if !writer.is_initialized(){writer.advance_initialization(cx)?;return Ok(None)}if self.cursor<b"ok".len(){writer.append_original(cx,b"ok",&mut self.cursor)?;return Ok(None)}if writer.published().is_none(){writer.seal(cx)?;return Ok(None)}
        if !self.completed{let grant=cx.retained_grant();let copied_bytes=std::mem::size_of::<usize>()+std::mem::size_of::<bool>();if grant.maximum_items<2||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Ok(None)}self.calls.as_ref().expect("recording router calls").fetch_add(1,Ordering::SeqCst);self.completed=true;cx.consume_retained(semio_framework_job::RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()})?;}
        JobOutcomeBorrow::admit_complete(cx,None,self.writer.as_ref().and_then(RetainedPayloadBuilder::published))
    }

    fn borrow_outcome<'a>(&'a self,original:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,semio_framework_value::ValueError>{
        if self.cancelled&&original.kind()==JobOutcomeKind::Cancelled{return original.cancelled()}if self.yielded&&original.kind()==JobOutcomeKind::Yield{return original.yielded()}if self.completed{let output=self.writer.as_ref().and_then(RetainedPayloadBuilder::published).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"recording router original output remains producer-held"))?;return original.complete(None,Some(output))}Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"recording router descriptor has no original producer witness"))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.begin_close();
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetirementTurnError};
        if let Some(writer)=self.writer.as_mut(){if writer.terminal_is_empty(){let copied_bytes=std::mem::size_of::<Option<RetainedPayloadBuilder>>();if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return InteractiveJobCloseStep::Blocked}drop(self.writer.take());return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}}}return match writer.close_step_granted(grant){Ok(step)=>InteractiveJobCloseStep::Pending{progress:step.progress()},Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}}}
        let demand=self.close_demands();
        let result=semio_framework_value::advance_retirement_turn(demand,grant,|_|{
            if self.calls.as_ref().is_some_and(|original|Arc::weak_count(original)!=0){return Ok((RetainedCloneStep::Progress(Default::default()),false));}
            let Some(original)=self.calls.take()else{return Ok((RetainedCloneStep::Complete(Default::default()),true));};
            let released_bytes=if let Some(original)=Arc::into_inner(original){drop(original);demand.release_bytes}else{0};
            Ok((RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..Default::default()}),true))
        });
        match result{Ok(step)if self.calls.is_none()=>InteractiveJobCloseStep::Complete{progress:step.progress()},Ok(step)=>InteractiveJobCloseStep::Pending{progress:step.progress()},Err(RetirementTurnError::Owner(error)|RetirementTurnError::Receipt(error))=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}}.admit(grant,self.terminal_is_empty())
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.receiving_close_demands()?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,_body:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.receiving_close_demands()?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.receiving_close_demands()?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.receiving_close_demands()?.depth)}

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.calls.is_none() && self.writer.is_none()
    }
}

impl RecordingRouterJob {
    fn receiving_close_demands(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if let Some(writer)=self.writer.as_ref(){return if writer.terminal_is_empty(){Ok(semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Option<RetainedPayloadBuilder>>(),depth:1,..Default::default()})}else{writer.retirement_demands()}}Ok(self.close_demands())}
    fn close_demands(&self)->semio_framework_value::RetirementDemand{self.calls.as_ref().map_or(Default::default(), |_|semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Option<Arc<AtomicUsize>>>(),release_bytes:semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>(),depth:1,..Default::default()})}
}

impl RouterEffectHandler for RecordingRouterHandler {
    fn create_job(&self, _effect: RouterEffect) -> Box<dyn InteractiveJob + Send> {
        Box::new(RecordingRouterJob { calls: Some(self.0.clone()), yielded: false, closing: false, cancelled:false,completed:false,cursor:0,writer:None })
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
///
/// 🐛️ `executor()`'s default `UnwiredRouterEffectHandler` always returns `Err` — this test
/// needs the NON-revoked operation to actually succeed so the two completions are
/// distinguishable by more than "which one happened to fail", so it swaps in a handler that
/// always succeeds.
#[semio_framework_async_macros::async_test]
async fn revoked_capability_cancels_only_its_own_operations_and_actor_survives() {
    let runtime = ManualRuntime::new(0).await;
    let runtime_dyn: Arc<ManualRuntime> = Arc::new(runtime.clone());
    let (mut executor, injector, actors) = executor(runtime_dyn.clone()).await;
    let scope = activate(&executor, &actors, runtime_dyn.as_ref(), 1, 0).await;
    struct AlwaysOkRouterHandler;
    impl RouterEffectHandler for AlwaysOkRouterHandler {
        fn create_job(&self, _effect: RouterEffect) -> Box<dyn InteractiveJob + Send> {
            Box::new(CompleteRouterEffectJob { output: Some(b"ok".to_vec()), writer: None, cursor: 0, closing: false })
        }
    }
    executor.router_handler = Arc::new(AlwaysOkRouterHandler);

    let revoked_cap = CapabilityTokenId(1);
    let kept_cap = CapabilityTokenId(2);

    let dispatch_revoked = EffectDispatchContext { router_turn_grant:fixture_router_grant(), actor: 1, package: PackageId("pkg".to_string()), lane: 0, capability: Some(revoked_cap) };
    let dispatch_kept = EffectDispatchContext { router_turn_grant:fixture_router_grant(), actor: 1, package: PackageId("pkg".to_string()), lane: 0, capability: Some(kept_cap) };

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

    let dispatch = EffectDispatchContext { router_turn_grant:fixture_router_grant(), actor: 1, package: PackageId("pkg".to_string()), lane: 0, capability: None };
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
    let dispatch = EffectDispatchContext { router_turn_grant:fixture_router_grant(), actor: 1, package: PackageId("pkg".to_string()), lane: 0, capability: None };
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
    let dispatch = EffectDispatchContext { router_turn_grant:fixture_router_grant(), actor: 1, package: PackageId("pkg".to_string()), lane: 0, capability: None };
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
    let dispatch = EffectDispatchContext { router_turn_grant:fixture_router_grant(), actor: 1, package: PackageId("pkg".to_string()), lane: 0, capability: None };
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
    let outcome = run_router_effect_job(&compute, &runtime, &scope, ctx, fixture_router_grant(), &handler, RouterEffect::CacheRead { engine_id: "e".to_string(), key: "k".to_string() }).await;
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

#[test]
fn router_effect_original_sources_keep_capacity_until_funded_close() {
    fn verify<J:InteractiveJob>(mut job:J,source:fn(&J)->&Option<Vec<u8>>,text:&str,grant:semio_framework_value::RetainedCloneGrant){
        let pointer=source(&job).as_ref().unwrap().as_ptr();let capacity=source(&job).as_ref().unwrap().capacity();
        job.begin_close();let mut released=0;let mut copies=0;let mut turns=0;
        while !job.terminal_is_empty(){
            let copy=job.next_close_copy_byte_demand().unwrap();let release=job.next_close_release_byte_demand().unwrap();let depth=job.next_close_depth_demand().unwrap();
            assert_eq!(job.next_close_capacity_byte_demand(copy).unwrap(),0);
            for axis in 0..4{
                let mut denied=grant;
                match axis{0=>denied.maximum_items=0,1 if copy>0=>denied.maximum_copy_bytes=copy-1,2 if release>0=>denied.maximum_release_bytes=release-1,3 if depth>0=>denied.maximum_depth=depth-1,_=>continue}
                let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(denied));
                assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                assert!(!matches!(step,InteractiveJobCloseStep::Complete{..}));
                assert_eq!(source(&job).as_ref().unwrap().as_ptr(),pointer);assert_eq!(source(&job).as_deref().unwrap(),text.as_bytes());
            }
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(grant));
            assert!(step.progress().fits(grant));assert_eq!(step.progress().retained_capacity_bytes,heap.requested_bytes);assert_eq!(step.progress().released_bytes,heap.released_bytes);
            assert!(!matches!(step,InteractiveJobCloseStep::Refused{..}|InteractiveJobCloseStep::Blocked));
            released+=step.progress().released_bytes;copies+=step.progress().copied_bytes;turns+=1;assert!(turns<16);
        }
        assert_eq!(released,capacity);assert!(copies>=std::mem::size_of::<Option<Vec<u8>>>());
        let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!(heap.released_bytes,0);
        let oracle:Vec<u8>=serde_json::from_str(&serde_json::to_string(text.as_bytes()).unwrap()).unwrap();assert_eq!(oracle,text.as_bytes());
        eprintln!("[DEBUG] original router source length{} capacity{capacity} close{turns} copied{copies} released{released} drop0",text.len());
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(fixture["grant"].clone()).unwrap();
    assert_eq!(fixture["expectedDropReleaseBytes"],0);
    for row in fixture["sources"].as_array().unwrap(){
        let text=row["text"].as_str().unwrap();
        let source=||{let mut source=Vec::with_capacity(row["capacity"].as_u64().unwrap()as usize);source.extend_from_slice(text.as_bytes());source};
        verify(FaultRouterEffectJob{detail:Some(source()),writer:Some(RetainedJobPayloadWriter::new(JobPayloadStream::Fault)),cursor:0,closing:false},|job|&job.detail,text,grant);
        verify(CompleteRouterEffectJob{output:Some(source()),writer:Some(RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput)),cursor:0,closing:false},|job|&job.output,text,grant);
    }
}

#[test]
fn router_effect_original_box_frame_has_a_separate_funded_terminal_turn(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(fixture["grant"].clone()).unwrap();
    assert_eq!(fixture["wrapper"]["separateTerminalFrame"],true);assert_eq!(fixture["wrapper"]["normalFactoryBirthQualified"],false);
    let mut source=Vec::with_capacity(fixture["sources"][0]["capacity"].as_u64().unwrap()as usize);source.extend_from_slice(fixture["sources"][0]["text"].as_str().unwrap().as_bytes());let source_capacity=source.capacity();
    let(mut owner,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||DynRouterEffectJob(Some(Box::new(FaultRouterEffectJob{detail:Some(source),writer:None,cursor:0,closing:false}))));
    let frame=std::mem::size_of_val(&**owner.0.as_ref().unwrap());assert_eq!(birth.requested_bytes,frame);assert_eq!(birth.released_bytes,0);
    owner.begin_close();let mut released=0;let mut saw_frame=false;
    for _ in 0..fixture["wrapper"]["maximumTurns"].as_u64().unwrap(){
        if owner.terminal_is_empty(){break;}
        let original=owner.0.as_ref().unwrap();let terminal=original.terminal_is_empty();let pointer=(&**original as *const dyn InteractiveJob).cast::<()>();
        let demand=owner.close_demands(grant.maximum_copy_bytes).unwrap();
        if terminal{
            saw_frame=true;assert_eq!(demand.release_bytes,frame);assert_eq!(demand.depth,1);
            let denied=semio_framework_value::RetainedCloneGrant{maximum_release_bytes:frame-1,..grant};
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(denied));assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!owner.terminal_is_empty());assert_eq!((&**owner.0.as_ref().unwrap()as *const dyn InteractiveJob).cast::<()>(),pointer);
        }
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant));assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(!matches!(step,InteractiveJobCloseStep::Refused{..}|InteractiveJobCloseStep::Blocked));released+=heap.released_bytes;
        if terminal{assert!(matches!(step,InteractiveJobCloseStep::Complete{..}));}
    }
    assert!(saw_frame&&owner.terminal_is_empty());assert_eq!(released,source_capacity+frame);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!(heap.released_bytes,0);
    eprintln!("[DEBUG] original router Box frame{frame} source{source_capacity} released{released} separate terminal frame funded drop0; factory birth unqualified");
}

#[test]
fn router_effect_recording_leases_keep_unique_shared_and_weak_backing_custody(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(fixture["grant"].clone()).unwrap();
    for case in fixture["recordingLeases"].as_array().unwrap(){
        let source=Arc::new(AtomicUsize::new(7));let bytes=semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>();
        let mut alias=(case=="alias").then(||RecordingRouterJob{calls:Some(source.clone()),yielded:false,closing:false});let weak=(case=="weak").then(||Arc::downgrade(&source));let mut owner=RecordingRouterJob{calls:Some(source),yielded:false,closing:false};owner.begin_close();
        let demand=owner.close_demands();
        for denied in [semio_framework_value::RetainedCloneGrant{maximum_items:0,..grant},semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant},semio_framework_value::RetainedCloneGrant{maximum_release_bytes:bytes-1,..grant},semio_framework_value::RetainedCloneGrant{maximum_depth:0,..grant}]{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(denied));assert_eq!(step.progress(),Default::default());assert!(!owner.terminal_is_empty());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
        if weak.is_some(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant));assert_eq!(step.progress(),Default::default());assert!(!owner.terminal_is_empty());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
        let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(weak));assert_eq!(heap.released_bytes,0);
        assert_eq!(owner.calls.as_ref().unwrap().load(Ordering::SeqCst),serde_json::from_str::<usize>("7").unwrap());
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant));assert!(owner.terminal_is_empty());assert!(step.progress().fits(grant));assert_eq!(step.progress().released_bytes,heap.released_bytes);let mut released=heap.released_bytes;
        if let Some(alias)=alias.as_mut(){assert_eq!(released,0);alias.begin_close();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||alias.close_step(grant));assert!(alias.terminal_is_empty());assert_eq!(step.progress().released_bytes,heap.released_bytes);released+=heap.released_bytes;}
        assert_eq!(released,bytes);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{drop(owner);drop(alias);});assert_eq!(heap.released_bytes,0);eprintln!("[DEBUG] original recording lease {} backing{bytes} released{released} weak-gated terminal drop0",case.as_str().unwrap());
    }
}

//! 🎛️ Async effect executor: the async services, the dispatch context and report, the executor with its effect classification and dispatch, the http response wire and the storage operations. See [`super`].

use super::*;

//#region 🎛️AsyncEffectExecutor
/// 🎛️ Every async host service this executor dispatches effects onto.
// 🔀️ dedyn-emit-runtime, O1/R3: generic over `R: HostAsyncRuntime` (this packet's own
// `HostAsyncRuntime` family) rather than `Arc<dyn HostAsyncRuntime>` — `StorageScheduler<R>` was
// already generic (see `🛎️services`), so `Arc<StorageScheduler>` bare was already ill-typed before
// this fix; `R` here makes both fields agree. `AsyncEffectExecutor::new` has zero production call
// sites repo-wide today (see `#region 🎛️AsyncEffectExecutor`'s own header note) — every construction
// is this module's own tests, all against `semio_framework_async::testkit::ManualRuntime` — so `R`
// stays a free type parameter here rather than defaulting to that test double.
pub struct AsyncServices<R: HostAsyncRuntime> {
    pub runtime: Arc<R>,
    pub http: Arc<HttpPool>,
    pub storage: Arc<StorageScheduler<R>>,
    pub timers: Arc<TimerWheel>,
    pub events: Arc<EventRouter>,
    pub compute: Arc<ComputePool>,
    pub storage_backend: Arc<dyn StorageBackend>,
}

/// 🎯️ What one `execute()` call dispatches a batch of effects on behalf of.
#[derive(Clone)]
pub struct EffectDispatchContext {
    pub router_turn_grant: semio_framework_job::RetainedCloneGrant,
    /// 🪪️ The STABLE actor identity (`OperationContext.actor`'s own convention) — the actor MUST
    /// already be `ActorScopeRegistry::activate`-d before effects are dispatched for it.
    pub actor: u64,
    pub package: PackageId,
    /// 🛣️ Bare-`u8` lane, mirroring `semio_framework_actor::Lane`'s discriminant order.
    pub lane: u8,
    /// 🔑️ The capability token authorising this whole batch, if any — every operation this batch
    /// dispatches is registered under it for revocation (see `CapabilityRevocationRegistry`).
    pub capability: Option<CapabilityTokenId>,
}

/// 📋️ What `execute()` did with one batch — for tests/observability, never load-bearing for
/// correctness (dispatch itself is the source of truth).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EffectDispatchReport {
    pub dispatched: u32,
    /// 🧵️ `SpawnJob`/`CancelJob` — stay with the shard loop by design, never dispatched here.
    pub shard_owned: u32,
    /// 🐚️ UI/shell effects (`OpenWindow`, `Navigate`, ...) — out of this executor's scope, passed
    /// through untouched for the existing shell-effect application pass.
    pub shell_owned: u32,
}

// 🧬️ O1/R11(a) — generic over the same `I: EnvelopeInjector` as `EnvelopeCompletionSink<I>`; see that
// struct's own doc for why this is the trivial-generics case, not `dyn_enum_close!`. `R:
// HostAsyncRuntime` added alongside it (dedyn-emit-runtime, same R11(a) reasoning — see
// `AsyncServices<R>`'s own doc): replaces `Arc<dyn HostAsyncRuntime>` inside `services`.
pub struct AsyncEffectExecutor<I: EnvelopeInjector, R: HostAsyncRuntime> {
    pub(super) services: AsyncServices<R>,
    pub(super) actors: ActorScopeRegistry,
    pub(super) capabilities: CapabilityRevocationRegistry,
    pub(super) sink: Arc<EnvelopeCompletionSink<I>>,
    pub(super) backbone: Arc<BackboneRegistry>,
    pub(super) router_handler: Arc<dyn RouterEffectHandler>,
    metrics: Arc<dyn EffectMetricsRecorder>,
    trace_ids: TraceIdAllocator,
}

// 🔀️ `R: HostAsyncRuntime + 'static` (not just `HostAsyncRuntime`, matching `StorageScheduler<R>`'s
// own impl bound in `🛎️services`): every dispatch method below builds a `HostFuture<()> = Pin<Box<dyn
// Future<..> + Send + 'static>>` capturing `Arc<R>` inside an `async move` block, which needs `R:
// 'static` to be nameable at all. `I: 'static` alongside it for the same reason — several of those
// same boxed blocks also capture `sink: Arc<EnvelopeCompletionSink<I>>` (R15 unmasked this: it was
// always required, but the `HostAsyncRuntime::sleep_until`/compute-job Send gap these dispatch
// methods hit first was reported before rustc's own outlives check on `I` had a chance to run).
impl<I: EnvelopeInjector + 'static, R: HostAsyncRuntime + 'static> AsyncEffectExecutor<I, R> {
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        services: AsyncServices<R>,
        actors: ActorScopeRegistry,
        capabilities: CapabilityRevocationRegistry,
        sink: Arc<EnvelopeCompletionSink<I>>,
        backbone: Arc<BackboneRegistry>,
        router_handler: Arc<dyn RouterEffectHandler>,
        metrics: Arc<dyn EffectMetricsRecorder>,
    ) -> Self {
        Self { services, actors, capabilities, sink, backbone, router_handler, metrics, trace_ids: TraceIdAllocator::new().await }
    }

    pub async fn actors(&self) -> &ActorScopeRegistry {
        &self.actors
    }

    pub async fn completion_sink(&self) -> &Arc<EnvelopeCompletionSink<I>> {
        &self.sink
    }

    /// 🛑️ `Suspend` — `CancelToken::park`: in-flight operations run to completion, but their
    /// completions accumulate in the actor's own mailbox (never dropped) until
    /// [`AsyncEffectExecutor::resume`].
    pub async fn suspend(&self, actor: u64) {
        if let Some(scope) = self.actors.scope_for(actor).await {
            scope.cancel.park().await;
        }
    }

    /// ▶️ Resume from `Suspend` — `CancelToken::unpark` then flush every completion the actor
    /// accumulated while parked, in order.
    pub async fn resume(&self, actor: u64) {
        if let Some(scope) = self.actors.scope_for(actor).await {
            scope.cancel.unpark().await;
        }
        self.sink.flush(actor).await;
    }

    /// 🔑️ Capability revoked — cancels only the child tokens registered under `capability`; the
    /// actor's own scope token is never touched, so the actor stays alive.
    pub async fn revoke_capability(&self, capability: CapabilityTokenId) {
        self.capabilities.revoke(capability).await;
    }

    /// 💥️ Trap — `cancel_scope(Actor, grace 0)`. Stale completions are dropped by generation
    /// gating once the caller re-`activate`s this actor at a bumped generation; this method only
    /// performs the mechanical cancellation and records the drain report.
    pub async fn trap(&self, actor: u64) -> Option<ScopeDrainReport> {
        let scope = self.actors.scope_for(actor).await?;
        let report = self.services.runtime.cancel_scope(&scope.owner, 0).await;
        self.metrics.record_drain(DrainOwner::Actor(actor), report);
        Some(report)
    }

    /// 🚫️ Quarantine/Disable — `cancel_scope(Package, grace 250ms)`, bench budget 6's stated pause
    /// ceiling.
    pub async fn quarantine_package(&self, package: &PackageId) -> ScopeDrainReport {
        let report = self.services.runtime.cancel_scope(&ScopeOwner::Package(package.0.clone()), 250).await;
        self.metrics.record_drain(DrainOwner::Package, report);
        report
    }

    /// 🚫️async: R10 residue shape 2 — `now_ms`/`cancel` each awaited ONCE here, at binding.
    async fn derive_ctx(&self, dispatch: &EffectDispatchContext, scope: &ScopeHandle, effect_deadline_ms: Option<u64>) -> OperationContext {
        let now_ms = self.services.runtime.now_ms().await;
        let generation = self.actors.generation_of(dispatch.actor).await.unwrap_or(0);
        let cancel = scope.cancel.child().await;
        if let Some(capability) = dispatch.capability {
            self.capabilities.track(capability, cancel.clone()).await;
        }
        OperationContext { actor: dispatch.actor, generation, trace: self.trace_ids.next().await, lane: dispatch.lane, deadline_ms: Some(clamp_deadline_ms(now_ms, effect_deadline_ms, dispatch.lane).await), cancel, capability: dispatch.capability }
    }

    /// ⚡️ Classifies and dispatches every effect in `effects` on behalf of `dispatch`. Cheap and
    /// synchronous itself — one `HostAsyncRuntime::spawn_scoped` call (or a direct, non-blocking
    /// `EventRouter` call) per effect, never an `.await` on the real work.
    ///
    /// 🚧️ Dispatching effects for an actor that was never (or no longer) activated is a
    /// caller bug, not a runtime fault — nothing to spawn into, so every effect is honestly
    /// reported as skipped rather than silently accepted.
    ///
    /// 🧵️ Stays with the shard loop — never moved here (mission's own instruction).
    ///
    /// 🐚️ Everything else is a UI/shell effect out of this executor's scope.
    pub async fn execute(&self, dispatch: &EffectDispatchContext, effects: &[Effect]) -> EffectDispatchReport {
        let Some(scope) = self.actors.scope_for(dispatch.actor).await else {
            return EffectDispatchReport::default();
        };
        let mut report = EffectDispatchReport::default();
        for effect in effects {
            match effect {
                Effect::HttpRequest { req, method, url, headers, body, .. } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_http(
                        ctx.await,
                        scope.clone(),
                        dispatch.package.clone(),
                        addressed_actor_id(dispatch.actor, self.actors.generation_of(dispatch.actor).await.unwrap_or(0)).await,
                        *req,
                        ServiceHttpRequest { method: method.clone(), url: url.clone(), headers: headers.clone(), body: body.clone().unwrap_or_default() },
                    )
                    .await;
                    report.dispatched += 1;
                }
                Effect::StorageRead { req, key } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_storage(ctx.await, scope.clone(), dispatch.package.clone(), *req, StorageOp::Read { key: key.clone() }).await;
                    report.dispatched += 1;
                }
                Effect::StorageWrite { req, key, bytes } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_storage(ctx.await, scope.clone(), dispatch.package.clone(), *req, StorageOp::Write { key: key.clone(), bytes: bytes.clone() }).await;
                    report.dispatched += 1;
                }
                Effect::StorageDelete { req, key } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_storage(ctx.await, scope.clone(), dispatch.package.clone(), *req, StorageOp::Delete { key: key.clone() }).await;
                    report.dispatched += 1;
                }
                Effect::SetTimer { id, after_ms, repeat } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_set_timer(ctx.await, scope.clone(), dispatch.package.clone(), *id, *after_ms, *repeat).await;
                    report.dispatched += 1;
                }
                Effect::PublishEvent { topic, payload } => {
                    self.dispatch_publish_event(dispatch, topic, payload).await;
                    report.dispatched += 1;
                }
                Effect::SendMessage { target, payload } => {
                    self.dispatch_send_message(dispatch, target, payload).await;
                    report.dispatched += 1;
                }
                Effect::Subscribe { topic } => {
                    self.services.events.subscribe(Topic(topic.clone()), addressed_actor_id(dispatch.actor, self.actors.generation_of(dispatch.actor).await.unwrap_or(0)).await, ChannelPolicy::LatestWins { max_bytes: 1_000_000 });
                    report.dispatched += 1;
                }
                Effect::Unsubscribe { topic } => {
                    self.services.events.unsubscribe(&Topic(topic.clone()), addressed_actor_id(dispatch.actor, self.actors.generation_of(dispatch.actor).await.unwrap_or(0)).await).await;
                    report.dispatched += 1;
                }
                Effect::BlobWrite { req, media_type, bytes } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_router_effect(ctx.await, scope.clone(), *req, dispatch.router_turn_grant, RouterEffect::BlobWrite { media_type: *media_type, bytes: bytes.clone() }).await;
                    report.dispatched += 1;
                }
                Effect::BlobLoad { req, hash } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_router_effect(ctx.await, scope.clone(), *req, dispatch.router_turn_grant, RouterEffect::BlobLoad { hash: hash.clone() }).await;
                    report.dispatched += 1;
                }
                Effect::DocumentRead { req, doc, lane } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_router_effect(ctx.await, scope.clone(), *req, dispatch.router_turn_grant, RouterEffect::DocumentRead { doc: doc.0, lane: lane.clone() }).await;
                    report.dispatched += 1;
                }
                Effect::DocumentWrite { req, doc, lane, ops } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_router_effect(ctx.await, scope.clone(), *req, dispatch.router_turn_grant, RouterEffect::DocumentWrite { doc: doc.0, lane: lane.clone(), ops: ops.clone() }).await;
                    report.dispatched += 1;
                }
                Effect::IoCompose { req, key, sources } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_router_effect(ctx.await, scope.clone(), *req, dispatch.router_turn_grant, RouterEffect::IoCompose { key: key.clone(), sources: sources.clone() }).await;
                    report.dispatched += 1;
                }
                Effect::CacheDerive { req, engine_id, input } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_router_effect(ctx.await, scope.clone(), *req, dispatch.router_turn_grant, RouterEffect::CacheDerive { engine_id: engine_id.clone(), input: input.clone() }).await;
                    report.dispatched += 1;
                }
                Effect::CacheRead { req, engine_id, key } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_router_effect(ctx.await, scope.clone(), *req, dispatch.router_turn_grant, RouterEffect::CacheRead { engine_id: engine_id.clone(), key: key.clone() }).await;
                    report.dispatched += 1;
                }
                Effect::InvokeExtension { req, extension_id, capability, request_json, .. } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_router_effect(ctx.await, scope.clone(), *req, dispatch.router_turn_grant, RouterEffect::InvokeExtension { extension_id: extension_id.clone(), capability: capability.clone(), request_json: request_json.clone() }).await;
                    report.dispatched += 1;
                }
                Effect::DispatchAction { req, action, args, delay_ms } => {
                    let ctx = self.derive_ctx(dispatch, &scope, None);
                    self.dispatch_router_effect(ctx.await, scope.clone(), *req, dispatch.router_turn_grant, RouterEffect::DispatchAction { action: action.clone(), args: args.clone(), delay_ms: *delay_ms }).await;
                    report.dispatched += 1;
                }
                Effect::SpawnJob { .. } | Effect::CancelJob { .. } => {
                    report.shard_owned += 1;
                }
                _ => {
                    report.shell_owned += 1;
                }
            }
        }
        report
    }

    /// 🐛️ `scope_for_task` is a SEPARATE clone from `scope` below — `async move` moves every
    /// variable it references (including one only ever used as `&scope`) into the future, so
    /// reusing the bare `scope` parameter here would leave nothing for `spawn_scoped(&scope, ..)`
    /// at the bottom of this function to borrow.
    async fn dispatch_http(&self, ctx: OperationContext, scope: ScopeHandle, package: PackageId, actor_id: RuntimeActorId, req: RequestId, request: ServiceHttpRequest) {
        let runtime = self.services.runtime.clone();
        let http = self.services.http.clone();
        let sink = self.sink.clone();
        let ctx_for_task = ctx.clone();
        let scope_for_task = scope.clone();
        let fut: HostFuture<()> = Box::pin(async move {
            if ctx_for_task.cancel.is_cancelled().await {
                emit_completed_err(&sink, &ctx_for_task, req, "capability-revoked", "http request cancelled before dispatch").await;
                return;
            }
            match http.request(runtime.as_ref(), &scope_for_task, ctx_for_task.clone(), package, actor_id, request).await {
                Ok(response) => emit_completed_ok(&sink, &ctx_for_task, req, encode_http_response(&response).await).await,
                Err(HttpPoolError::Compute(ComputeError::DeadlineExceeded)) => emit_completed_err(&sink, &ctx_for_task, req, "deadline-exceeded", "http request exceeded its deadline").await,
                Err(error) => emit_completed_err(&sink, &ctx_for_task, req, "http-error", error.to_string()).await,
            }
        });
        self.services.runtime.spawn_scoped(&scope, ctx, fut).await;
    }

    async fn dispatch_storage(&self, ctx: OperationContext, scope: ScopeHandle, package: PackageId, req: RequestId, op: StorageOp) {
        let storage = self.services.storage.clone();
        let backend = self.services.storage_backend.clone();
        let sink = self.sink.clone();
        let ctx_for_task = ctx.clone();
        let fut: HostFuture<()> = Box::pin(async move {
            if ctx_for_task.cancel.is_cancelled().await {
                emit_completed_err(&sink, &ctx_for_task, req, "capability-revoked", "storage op cancelled before dispatch").await;
                return;
            }
            let bytes_hint = op.byte_hint();
            let submitted = storage.submit(&ctx_for_task, package, bytes_hint, move || op.run(backend.as_ref()));
            match submitted {
                Ok(ticket) => match ticket.await_result().await {
                    Ok(bytes) => emit_completed_ok(&sink, &ctx_for_task, req, bytes).await,
                    Err(StorageError::DeadlineExceeded) => emit_completed_err(&sink, &ctx_for_task, req, "deadline-exceeded", "storage op exceeded its deadline").await,
                    Err(storage_error) => emit_completed_err(&sink, &ctx_for_task, req, "storage-error", storage_error.to_string()).await,
                },
                Err(storage_error @ StorageError::BytesQuotaExceeded { .. }) => emit_completed_err(&sink, &ctx_for_task, req, "quota-exceeded", storage_error.to_string()).await,
                Err(storage_error) => emit_completed_err(&sink, &ctx_for_task, req, "storage-error", storage_error.to_string()).await,
            }
        });
        self.services.runtime.spawn_scoped(&scope, ctx, fut).await;
    }

    /// ⏲️ `SetTimer` uses `TimerWheel::arm`/`disarm` for per-plugin QUOTA admission only — it does
    /// NOT use `TimerWheel::spawn_driver`'s shared sink-calling mechanism, because that mechanism
    /// hands the completion sink the WHEEL's own internal `TimerId` bytes
    /// (`timer.id.0.to_le_bytes()`), and `TimerId`'s inner field is private outside
    /// `semio-framework-os-services` with no accessor — there is no way to translate that back to
    /// the GUEST's own chosen `SetTimer.id` from outside that crate (see the packet report's
    /// honest gaps: this is a real, confirmed os-services API gap, not a shortcut taken here). This
    /// executor instead reserves the quota slot via `arm`, then sleeps and fires the completion
    /// itself via `runtime.sleep_until`, so the guest-chosen `id` is preserved exactly.
    ///
    /// 🩹️ terra-shard-lane: the `resolve_ready` bridge this comment used to document
    /// was a workaround for a `🛎️services` `TimerWheel::arm`/`disarm`/`armed_count`
    /// bug (holding a `std::sync::MutexGuard<WheelCore>` across their OWN internal
    /// `.await`, breaking `HostFuture<()>: Send`) — sol R9-reverted all three to sync
    /// (matching their already-tagged siblings `pop_expired`/`next_expiry_ms`) and
    /// removed the guard-across-await shape at its root. Plain `.await` now compiles
    /// (verified: no E0277), so the bridge is gone — one fewer E5 exception in this
    /// crate.
    ///
    /// 🕳️ `Effect::SetTimer` carries no `req: RequestId` to answer — there is no
    /// `Event::Completed` to emit for a quota-exceeded arm. Honest gap: a future
    /// `design-abi.md` revision could add one; until then this is silently refused,
    /// mirroring `WheelCore::arm`'s own "leaves the wheel completely untouched" contract.
    async fn dispatch_set_timer(&self, ctx: OperationContext, scope: ScopeHandle, package: PackageId, guest_timer_id: u64, after_ms: u64, repeat: bool) {
        let runtime = self.services.runtime.clone();
        let wheel = self.services.timers.clone();
        let sink = self.sink.clone();
        let now_ms = self.services.runtime.now_ms().await;
        let at_ms = now_ms.saturating_add(after_ms);
        let repeat_ms = if repeat { Some(after_ms.max(1)) } else { None };
        match wheel.arm(package, ctx.actor, ctx.generation, ctx.lane, at_ms, repeat_ms).await {
            Ok(timer_id) => {
                let ctx_for_task = ctx.clone();
                let fut: HostFuture<()> = Box::pin(async move {
                    let mut next_ms = at_ms;
                    loop {
                        runtime.sleep_until(next_ms).await;
                        if ctx_for_task.cancel.is_cancelled().await {
                            break;
                        }
                        sink.complete(ctx_for_task.actor, ctx_for_task.generation, encode_event(&Event::Timer { id: guest_timer_id }).await, ctx_for_task.lane);
                        match repeat_ms {
                            Some(step) => next_ms = next_ms.saturating_add(step),
                            None => break,
                        }
                    }
                    wheel.disarm(timer_id).await;
                });
                self.services.runtime.spawn_scoped(&scope, ctx, fut).await;
            }
            Err(TimerError::QuotaExceeded { .. }) => {}
        }
    }

    async fn dispatch_publish_event(&self, dispatch: &EffectDispatchContext, topic: &str, payload: &[u8]) {
        let topic = Topic(topic.to_string());
        for (recipient, outcome) in self.services.events.publish(&topic, None, payload).await {
            if matches!(outcome, PublishOutcome::Delivered | PublishOutcome::Collapsed) {
                self.deliver_message(recipient, MessageEndpoint::Topic { name: topic.0.clone() }, dispatch.lane).await;
            }
        }
    }

    /// 🚫️async: R13/R14 corollary — `let _ = <async call>;` suppressed the lint while
    /// silently dropping this call's future: `Effect::SendMessage` to a `Backbone`
    /// target never actually reached `BackboneRegistry::send` (capability check, lookup,
    /// transport dispatch — none of it ran), so the guest↔store sync bridge this whole
    /// module's doc comment describes stayed as broken as the deleted process-global
    /// channel it replaced. The `Result` still discards (no `RequestId` to answer with a
    /// completion for this effect), only the future is now actually driven.
    ///
    /// 🕳️ Honest gap: there is no `PluginInstanceId -> ActorId` directory in this
    /// module (that mapping lives with the instance directory this packet does not
    /// own) — `send_message` below is a documented no-op until that lookup exists.
    ///
    /// 🕳️ Same class of gap: these targets need an id -> ActorId directory this
    /// executor does not own. See the packet report's honest gaps.
    async fn dispatch_send_message(&self, dispatch: &EffectDispatchContext, target: &MessageEndpoint, payload: &[u8]) {
        match target {
            MessageEndpoint::Backbone { uri } => {
                let _ = self.backbone.send(dispatch.actor, uri, payload).await;
            }
            MessageEndpoint::PluginInstance { id } => {
                let topic = Topic(format!("__message__:{}", id.0));
                let _ = topic;
            }
            MessageEndpoint::Shell { .. } | MessageEndpoint::Extension { .. } | MessageEndpoint::Topic { .. } => {}
        }
    }

    async fn deliver_message(&self, recipient: semio_framework_actor::ActorId, source: MessageEndpoint, lane: u8) {
        let Some(generation) = self.actors.generation_of(recipient.0).await else { return };
        let topic = Topic(format!("__message_inbox__:{}", recipient.0));
        for payload in self.services.events.drain(&topic, recipient).await {
            self.sink.complete(recipient.0, generation, encode_event(&Event::Message { source: source.clone(), payload }).await, lane);
        }
    }

    /// 🐛️ Same reasoning as `dispatch_http`'s own `scope_for_task` — a separate clone so the
    /// bare `scope` below survives the `async move` block for `spawn_scoped(&scope, ..)`.
    async fn dispatch_router_effect(&self, ctx: OperationContext, scope: ScopeHandle, req: RequestId, retained: semio_framework_job::RetainedCloneGrant, effect: RouterEffect) {
        let runtime = self.services.runtime.clone();
        let compute = self.services.compute.clone();
        let sink = self.sink.clone();
        let handler = self.router_handler.clone();
        let ctx_for_task = ctx.clone();
        let scope_for_task = scope.clone();
        let fut: HostFuture<()> = Box::pin(async move {
            if ctx_for_task.cancel.is_cancelled().await {
                emit_completed_err(&sink, &ctx_for_task, req, "capability-revoked", "router effect cancelled before dispatch").await;
                return;
            }
            let result = run_router_effect_job(&compute, runtime.as_ref(), &scope_for_task, ctx_for_task.clone(), retained, &handler, effect).await;
            match result {
                RouterEffectJobOutcome::Complete(bytes) => emit_completed_ok(&sink, &ctx_for_task, req, bytes).await,
                RouterEffectJobOutcome::Cancelled => emit_completed_err(&sink, &ctx_for_task, req, "capability-revoked", "router effect cancelled").await,
                RouterEffectJobOutcome::Fault(router_error) => emit_completed_err(&sink, &ctx_for_task, req, "router-error", router_error).await,
                RouterEffectJobOutcome::DeadlineExceeded => emit_completed_err(&sink, &ctx_for_task, req, "deadline-exceeded", "router effect exceeded its deadline").await,
                RouterEffectJobOutcome::WorkerLost => emit_completed_err(&sink, &ctx_for_task, req, "worker-lost", "router effect worker lost").await,
            }
        });
        self.services.runtime.spawn_scoped(&scope, ctx, fut).await;
    }
}

#[derive(Clone, Debug, serde::Serialize, ToValue)]
struct HttpResponseWire {
    status: u16,
    headers: Vec<(String, String)>,
    pub(super) body: Vec<u8>,
}

async fn encode_http_response(response: &ServiceHttpResponse) -> Vec<u8> {
    semio_framework_pack_json::to_json_string(&HttpResponseWire { status: response.status, headers: response.headers.clone(), body: response.body.clone() }).into_bytes()
}

#[derive(Clone, Debug)]
enum StorageOp {
    Read { key: String },
    Write { key: String, bytes: Vec<u8> },
    Delete { key: String },
}

impl StorageOp {
    /// 🚫️async: E1 — `StorageScheduler::submit`'s `work` parameter is a plain sync
    /// `FnOnce() -> Result<Vec<u8>, io::Error> + Send + 'static` closure (`🛎️services`, R4 clause 3:
    /// deliberately bounded-blocking, not async) — `run` is called from inside that sync closure, and
    /// `byte_hint` is pure with no suspension point, so both stay sync. See R9.
    fn byte_hint(&self) -> u64 {
        match self {
            StorageOp::Read { key } => key.len() as u64,
            StorageOp::Write { bytes, .. } => bytes.len() as u64,
            StorageOp::Delete { .. } => 0,
        }
    }

    fn run(self, backend: &dyn StorageBackend) -> Result<Vec<u8>, std::io::Error> {
        match self {
            StorageOp::Read { key } => backend.read(&key),
            StorageOp::Write { key, bytes } => backend.write(&key, &bytes).map(|()| Vec::new()),
            StorageOp::Delete { key } => backend.delete(&key).map(|()| Vec::new()),
        }
    }
}
//#endregion 🎛️AsyncEffectExecutor

//! ⏳️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-async-imports → B1 world-collapse). Host side
//! of `world actor`'s `interface host-async` (`🧬️schema/📜️.wit`): 24 `async func` imports
//! the guest can actually `.await`, plus the 2 fire-and-forget doors (`emit`/`emit-patch`). Until
//! the collapse these were unreachable — they belonged to the now-deleted `world actor-async`, which
//! nothing instantiated; `world actor` importing `host-async` is what puts them on the live ABI.
//!
//! 🎯️ **The routing rule this whole file follows**: an async import AWAITS THE REAL HOST SERVICE
//! DIRECTLY and resolves the guest's future — it never goes through the poll world's
//! effect-envelope + `Event::Completed` round trip (`⚡️effects/🦀️.rs`'s
//! `AsyncEffectExecutor::dispatch_*` fire-and-forget-into-a-sink shape stays that world's own
//! mechanism, untouched here). `emit`/`emit-patch` are the one exception: they are the ONE-WAY
//! doors, and per the mission they DO reuse `AsyncEffectExecutor` as the single effect classifier —
//! `emit` converts its `effect` and pushes it onto `AsyncActorHostState::effect_sink`, `emit-patch`
//! pushes onto `patch_sink`; a later packet (the actor task driving `runner::run`) drains both sinks
//! once per turn and calls `AsyncEffectExecutor::execute`/applies the patches — see `## why no
//! completion round-trip` and `## streams` in the packet report.
//!
//! 🏗️ **Bindgen mount**: none of its own any more — `host_async_bindings` is an alias for
//! `🦀️.rs`'s `mod actor_bindings`, the crate's single `bindgen!` invocation.
//!
//! See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/
//! 📓️terra-async-imports-report.md`.

use std::collections::{HashMap, VecDeque};
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use semio_framework::{MediaClass, MediaForm, MediaType};
use semio_framework_actor::{ActorId as RuntimeActorId, PackageId};
use semio_framework_async::{CancelToken, CapabilityTokenId, HostAsyncRuntime, HostFuture, OperationContext, ScopeHandle, TraceId};
use semio_framework_os_services::{HttpRequest as ServiceHttpRequest, StorageError, TokioHostRuntime};
use wasmtime::component::{Accessor, Destination, HasSelf, StreamProducer, StreamReader, StreamResult, VecBuffer};
use wasmtime::{ResourceLimiter, StoreContextMut};
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};

//#region 🔌️Bindings
/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): this file no longer runs its own
/// `bindgen!`. `world actor-async` is DELETED and `world actor` imports `host-async` itself, so the
/// crate's single `bindgen!` invocation (`🦀️.rs`'s `mod actor_bindings`) now generates
/// exactly the types these 24 implementations need. The alias is kept so every `host_async_bindings::…`
/// path in this file — and in `⏳️runtime/🦀️.rs`, which a later packet rewrites — still resolves.
///
/// 🐛️ The `additional_derives` caveats that used to live here moved with the invocation; see
/// `mod actor_bindings`'s own comment for why it requests neither `Debug` nor `Clone`.
pub(crate) use super::actor_bindings as host_async_bindings;

use host_async_bindings::semio::framework::{effects as wit_effects, host_async as wit_host_async, ui as wit_ui};
//#endregion 🔌️Bindings

//#region 🪪️AsyncActorHostState
/// 🪪️ One `Store<AsyncActorHostState>` per actor (S1b's confirmed shape: `store.run_concurrent`,
/// multiplexed host-side, never `Accessor::spawn` across actors) — every field below is therefore
/// fixed for the whole lifetime of one actor generation, unlike `⚡️effects/🦀️.rs`'s
/// `ActorScopeRegistry`/`EffectDispatchContext`, which look an actor up per batch because ONE
/// `AsyncEffectExecutor` serves EVERY actor.
///
/// 🌉️ `services`/`router_handler` are the literal `Arc<crate::effects::AsyncServices>` /
/// `Arc<dyn crate::effects::RouterEffectHandler>` `AsyncEffectExecutor` itself dispatches onto —
/// reused verbatim, never re-implemented, so `HttpPool`/`StorageScheduler`/`ComputePool` have
/// exactly one calling convention in this crate.
///
/// 🔑️ `capability`/`capability_registry`: a SEPARATE, small cancel-token registry from
/// `⚡️effects/🦀️.rs`'s own `CapabilityRevocationRegistry` — that type's `track` method is
/// private to its module (only `revoke` is `pub`), so it cannot be reached from this sibling module
/// without editing `⚡️effects/🦀️.rs` (out of this packet's owned paths). See the packet
/// report's `## lease-requests` — making `track` `pub` there would let the two unify.
// 🚫️async: E0107 residue — `AsyncServices<R: HostAsyncRuntime>` is generic (see
// `⚡️effects/🦀️.rs`), but this async-world state has zero production callers today (see
// this file's own `not_wired`/`run_router_effect` doc comments), so there is no real caller to
// thread `R` from. `TokioHostRuntime` is the one production `HostAsyncRuntime` impl
// (`🛎️services/🦀️component.rs`) and matches what a real instantiation would need — pinned here
// rather than making this whole file generic, which would thread `R` through ~25 trait methods
// with no caller yet to prove the shape against. Revisit once this world is actually wired.
pub struct AsyncActorHostState {
    services: Arc<crate::effects::AsyncServices<TokioHostRuntime>>,
    router_handler: Arc<dyn crate::effects::RouterEffectHandler>,
    scope: ScopeHandle,
    actor: u64,
    generation: u16,
    package: PackageId,
    lane: u8,
    capability: Option<CapabilityTokenId>,
    capability_registry: Arc<DirectAwaitCapabilityRegistry>,
    #[allow(dead_code)]
    caps: Vec<semio_framework::kernel::BrokerCapabilityGrant>,
    trace_ids: TraceIdAllocator,
    effect_sink: Vec<semio_framework::kernel::Effect>,
    patch_sink: Vec<wit_ui::UiPatch>,
    limiter: super::BudgetLimiter,
    wasi_ctx: WasiCtx,
    resource_table: wasmtime::component::ResourceTable,
}

impl AsyncActorHostState {
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        services: Arc<crate::effects::AsyncServices<TokioHostRuntime>>,
        router_handler: Arc<dyn crate::effects::RouterEffectHandler>,
        scope: ScopeHandle,
        actor: u64,
        generation: u16,
        package: PackageId,
        lane: u8,
        capability: Option<CapabilityTokenId>,
        caps: Vec<semio_framework::kernel::BrokerCapabilityGrant>,
        wasi_ctx: WasiCtx,
    ) -> AsyncActorHostState {
        AsyncActorHostState {
            services,
            router_handler,
            scope,
            actor,
            generation,
            package,
            lane,
            capability,
            capability_registry: Arc::new(DirectAwaitCapabilityRegistry::new()),
            caps,
            trace_ids: TraceIdAllocator::new(),
            effect_sink: Vec::new(),
            patch_sink: Vec::new(),
            limiter: super::BudgetLimiter::default(),
            wasi_ctx,
            resource_table: wasmtime::component::ResourceTable::new(),
        }
    }

    /// 🚪️ Drained once per turn by the (later-packet) actor task driving `runner::run` — see the
    /// module doc's `## why no completion round-trip` cross-reference.
    pub async fn take_effects(&mut self) -> Vec<semio_framework::kernel::Effect> {
        std::mem::take(&mut self.effect_sink)
    }

    pub async fn take_patches(&mut self) -> Vec<wit_ui::UiPatch> {
        std::mem::take(&mut self.patch_sink)
    }

    /// 🔑️ Cancels every direct-await import call registered under `capability` — mirrors
    /// `CapabilityRevocationRegistry::revoke`'s own "actor stays alive" contract exactly, just
    /// against this file's own small registry (see the struct doc's `## lease-requests` note).
    pub async fn revoke_capability(&self, capability: CapabilityTokenId) {
        self.capability_registry.revoke(capability);
    }

    pub async fn limiter(&mut self) -> &mut dyn ResourceLimiter {
        &mut self.limiter
    }
}

impl WasiView for AsyncActorHostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView { ctx: &mut self.wasi_ctx, table: &mut self.resource_table }
    }
}
//#endregion 🪪️AsyncActorHostState

//#region 🆔️TraceIdAllocator
/// 🆔️ Local copy of `⚡️effects/🦀️.rs`'s own allocator — that one is `pub` but scoped to
/// `AsyncEffectExecutor`'s own `trace_ids` field; duplicated here (three lines) rather than shared,
/// same "self-contained module" call `⚡️effects/🦀️.rs`'s own `fault_bytes` doc makes.
// 🚫️async: E1 pure accessor — an `AtomicU64` counter with no suspension point, whose one
// consumer (`begin_call`, see its own tag below) cannot hold the exclusive `Accessor::with` access
// this needs across an `.await` — see R9.
struct TraceIdAllocator(AtomicU64);

impl TraceIdAllocator {
    fn new() -> TraceIdAllocator {
        TraceIdAllocator(AtomicU64::new(1))
    }

    fn next(&self) -> TraceId {
        TraceId(self.0.fetch_add(1, Ordering::SeqCst))
    }
}
//#endregion 🆔️TraceIdAllocator

//#region 🛑️Cancellation
/// 🛑️ S2's proven shape (`hang()`'s host future dropped on guest-side cancel → `DropSignal` fires)
/// generalised to every import in this file: `armed` lets normal completion `disarm()` the guard
/// (a finished call cancelling its own already-spent child token would be a misleading no-op, not a
/// real cancellation), so `Drop::drop` only ever fires for the genuine case — the guest cancels the
/// awaiting subtask and wasmtime drops THIS future mid-poll without ever reaching the tail
/// `guard.disarm()`.
struct CancelOnDrop {
    token: CancelToken,
    armed: bool,
}

// 🚫️async: E1 pure constructor/mutator — no suspension point; consumed by `begin_call`'s
// `.with()`-scoped sync context (see that fn's own tag) — R9.
impl CancelOnDrop {
    fn new(token: CancelToken) -> CancelOnDrop {
        CancelOnDrop { token, armed: true }
    }

    fn disarm(mut self) {
        self.armed = false;
    }
}

impl Drop for CancelOnDrop {
    // 🚫️async: E5 executor bridge. `Drop::drop` (E1 — `Drop` is an externally-declared trait,
    // language-fixed sync) cannot `.await`; `CancelToken::cancel` lives in `⏳️async` (outside this
    // packet's `🔌️plugin/🖥️host` path_scope, so it cannot be reverted to sync here even though its
    // own body — a bare `AtomicU8::store` — has no suspension point of its own). Same bridge shape
    // as `🚚️process-transport/🦀️.rs`'s `impl Drop for ProcessTransport`. Sound: `cancel`
    // never actually parks, so this never blocks.
    fn drop(&mut self) {
        if self.armed {
            semio_framework_async::block_on(self.token.cancel());
        }
    }
}

/// 🔑️ See `AsyncActorHostState`'s own doc for why this is a second, small registry rather than a
/// reuse of `⚡️effects/🦀️.rs`'s `CapabilityRevocationRegistry`.
#[derive(Default)]
struct DirectAwaitCapabilityRegistry(Mutex<HashMap<CapabilityTokenId, Vec<CancelToken>>>);

// 🚫️async: E1 pure Mutex-guarded bookkeeping — no suspension point; same `begin_call` consumer
// constraint as `TraceIdAllocator`/`CancelOnDrop` above — R9.
impl DirectAwaitCapabilityRegistry {
    fn new() -> DirectAwaitCapabilityRegistry {
        DirectAwaitCapabilityRegistry::default()
    }

    fn track(&self, capability: CapabilityTokenId, token: CancelToken) {
        self.0.lock().expect("DirectAwaitCapabilityRegistry mutex poisoned").entry(capability).or_default().push(token);
    }

    // 🚫️async: E5 executor bridge, same reasoning as `CancelOnDrop::drop` above — `revoke` itself
    // stays sync per this impl's own R9 tag (its consumer, `begin_call`'s sync-scoped bookkeeping,
    // needs it sync), but `CancelToken::cancel` is a genuinely-async external-crate fn this file
    // cannot revert to sync. `cancel` never actually parks (bare atomic store), so this never blocks.
    fn revoke(&self, capability: CapabilityTokenId) {
        if let Some(tokens) = self.0.lock().expect("DirectAwaitCapabilityRegistry mutex poisoned").remove(&capability) {
            for token in tokens {
                semio_framework_async::block_on(token.cancel());
            }
        }
    }
}
//#endregion 🛑️Cancellation

//#region 🧯️Fault encoding
/// 🧯️ Local copy of `🦀️.rs`'s `host_fault_bytes` / `⚡️effects/🦀️.rs`'s
/// `fault_bytes` — same three-line duplication precedent both of those already establish.
async fn fault_bytes(code: impl Into<String>, message: impl Into<String>) -> Vec<u8> {
    let code = code.into();
    dsl::encode_fault_bytes(&dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new(code), message))
}
//#endregion 🧯️Fault encoding

//#region ⏱️Deadlines
/// ⏱️ Local copy of `⚡️effects/🦀️.rs`'s own `LANE_DEADLINE_CEILING_MS`/`lane_ceiling_ms` —
/// same values, same per-lane ceiling discipline, duplicated for the same self-contained-module
/// reason as `fault_bytes` above.
const LANE_DEADLINE_CEILING_MS: [u64; 4] = [2_000, 5_000, 30_000, 120_000];

// 🚫️async: E1 pure array lookup, no suspension point — R9.
fn lane_ceiling_ms(lane: u8) -> u64 {
    LANE_DEADLINE_CEILING_MS.get(lane as usize).copied().unwrap_or(*LANE_DEADLINE_CEILING_MS.last().expect("non-empty"))
}
//#endregion ⏱️Deadlines

//#region 🧭️addressed_actor_id
/// 🧭️ Local copy of `⚡️effects/🦀️.rs`'s own `addressed_actor_id` — reconstructs the full
/// `ActorId` (stable bits from `actor`, generation bits from `generation`) `HttpPool`'s per-actor
/// `outstanding_requests` cap keys on.
async fn addressed_actor_id(actor_stable: u64, generation: u16) -> RuntimeActorId {
    let stable = RuntimeActorId(actor_stable);
    RuntimeActorId::new(stable.plugin_ordinal(), stable.kind_tag(), stable.ordinal(), generation).await
}
//#endregion 🧭️addressed_actor_id

//#region 📞️CallContext
/// 📞️ Everything one import call needs, derived fresh per call from `AsyncActorHostState` — the
/// direct-await counterpart to `⚡️effects/🦀️.rs`'s `AsyncEffectExecutor::derive_ctx`, minus
/// the cross-actor scope lookup (this state IS the one actor).
struct CallContext {
    ctx: OperationContext,
    services: Arc<crate::effects::AsyncServices<TokioHostRuntime>>,
    router_handler: Arc<dyn crate::effects::RouterEffectHandler>,
    scope: ScopeHandle,
    package: PackageId,
    actor_id: RuntimeActorId,
    guard: CancelOnDrop,
}

/// 📸️ Everything `begin_call` needs, extracted SYNCHRONOUSLY inside `Accessor::with`'s closure —
/// `wasmtime::component::Access::get()`'s `&mut AsyncActorHostState` cannot itself escape that
/// closure (its lifetime is scoped to the closure call, enforced structurally so store access can
/// never be held across an `.await`), but `begin_call` genuinely needs to await
/// `HostAsyncRuntime::now_ms`/`CancelToken::child` (both external to this packet's scope, real
/// suspension points) plus `addressed_actor_id`. So the extraction (this struct, all owned clones
/// or `Copy` fields) happens inside `.with()`, and the actual awaiting happens after, outside it —
/// see every `HostWithStore` method below for the two-step call site this produces.
struct CallSnapshot {
    actor: u64,
    generation: u16,
    lane: u8,
    capability: Option<CapabilityTokenId>,
    services: Arc<crate::effects::AsyncServices<TokioHostRuntime>>,
    router_handler: Arc<dyn crate::effects::RouterEffectHandler>,
    scope: ScopeHandle,
    package: PackageId,
    capability_registry: Arc<DirectAwaitCapabilityRegistry>,
    trace: TraceId,
}

fn snapshot_call(state: &mut AsyncActorHostState) -> CallSnapshot {
    CallSnapshot {
        actor: state.actor,
        generation: state.generation,
        lane: state.lane,
        capability: state.capability,
        services: state.services.clone(),
        router_handler: state.router_handler.clone(),
        scope: state.scope.clone(),
        package: state.package.clone(),
        capability_registry: state.capability_registry.clone(),
        trace: state.trace_ids.next(),
    }
}

async fn begin_call(snap: CallSnapshot) -> CallContext {
    let now_ms = snap.services.runtime.now_ms().await;
    let cancel = snap.scope.cancel.child().await;
    if let Some(capability) = snap.capability {
        snap.capability_registry.track(capability, cancel.clone());
    }
    let ctx = OperationContext { actor: snap.actor, generation: snap.generation, trace: snap.trace, lane: snap.lane, deadline_ms: Some(now_ms.saturating_add(lane_ceiling_ms(snap.lane))), cancel: cancel.clone(), capability: snap.capability };
    CallContext { ctx, services: snap.services, router_handler: snap.router_handler, scope: snap.scope, package: snap.package, actor_id: addressed_actor_id(snap.actor, snap.generation).await, guard: CancelOnDrop::new(cancel) }
}
//#endregion 📞️CallContext

//#region 🌊️Streams
/// 🌊️ S5's proven shape (park on empty queue, store the waker, wake from elsewhere) generalised:
/// fed either by a background `spawn_scoped` task pulling real chunks (`http-fetch`) or by a single
/// pre-filled, already-`done` queue (`blob-read`'s buffered fallback — see the struct's own call
/// sites and the packet report's `## streams`).
struct ChunkShared {
    queue: VecDeque<Vec<u8>>,
    done: bool,
    waker: Option<Waker>,
}

struct ChunkStreamProducer {
    shared: Arc<Mutex<ChunkShared>>,
}

impl StreamProducer<AsyncActorHostState> for ChunkStreamProducer {
    type Item = u8;
    type Buffer = VecBuffer<u8>;

    fn poll_produce<'a>(self: Pin<&mut Self>, cx: &mut Context<'_>, _store: StoreContextMut<'a, AsyncActorHostState>, mut destination: Destination<'a, u8, VecBuffer<u8>>, _finish: bool) -> Poll<wasmtime::Result<StreamResult>> {
        let mut shared = self.shared.lock().expect("ChunkShared mutex poisoned");
        if let Some(chunk) = shared.queue.pop_front() {
            destination.set_buffer(chunk.into());
            return Poll::Ready(Ok(StreamResult::Completed));
        }
        if shared.done {
            return Poll::Ready(Ok(StreamResult::Dropped));
        }
        shared.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

async fn wake_chunk_shared(shared: &Mutex<ChunkShared>) {
    if let Some(waker) = shared.lock().expect("ChunkShared mutex poisoned").waker.take() {
        waker.wake();
    }
}

/// 🐌️ `blob-read`'s fallback per the mission's own instruction: the `BlobLoad` router
/// job returns one buffered result — there is no chunked blob
/// backend anywhere in this codebase today, and adding one would mean editing
/// `⚡️effects/🦀️.rs`/`semio-framework-os-services`, both out of this packet's owned paths.
/// One already-`done` chunk is therefore the honest, real behaviour, not a placeholder.
async fn single_chunk_shared(bytes: Vec<u8>) -> Arc<Mutex<ChunkShared>> {
    let mut queue = VecDeque::new();
    queue.push_back(bytes);
    Arc::new(Mutex::new(ChunkShared { queue, done: true, waker: None }))
}
//#endregion 🌊️Streams

//#region 🐛️Conversions
/// 🐛️ Local copy of `🦀️.rs`'s own `wit_message_endpoint_to_kernel`. Since B1
/// world-collapse both files share ONE `bindgen!` invocation, so the two now operate on the SAME
/// `types::MessageEndpoint` — the duplication is no longer forced by nominal type distinctness and
/// is a genuine merge candidate for whichever packet next opens both files.
async fn wit_message_endpoint_to_kernel(endpoint: host_async_bindings::semio::framework::types::MessageEndpoint) -> semio_framework::kernel::MessageEndpoint {
    use host_async_bindings::semio::framework::types::MessageEndpoint as M;
    use semio_framework::kernel::MessageEndpoint as K;
    match endpoint {
        M::Shell(instance) => K::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        M::Backbone(uri) => K::Backbone { uri },
        M::PluginInstance(instance) => K::PluginInstance { id: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        M::Extension(id) => K::Extension { id },
        M::Topic(name) => K::Topic { name },
    }
}

async fn decode_dsl(bytes: &[u8]) -> Option<semio_framework::DslValue> {
    if bytes.is_empty() {
        return None;
    }
    store::pack_rt::decode_wire_value(bytes).ok()
}

async fn decode_json<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Option<T> {
    if bytes.is_empty() {
        return None;
    }
    serde_json::from_slice(bytes).ok()
}

/// 🐛️ Local copy of `🦀️.rs`'s own `wit_effect_to_kernel`, retargeted at this file's OWN
/// `effects::Effect` (see this region's own doc above for why a second copy, not a shared generic).
/// Only reached from `emit` (the one-way door) — `io-run` stays the one `Err` case, same
/// `## blocked-on` this mirrors (`Effect::IoRun` has no kernel counterpart yet, packet A3).
async fn wit_effect_to_kernel(effect: wit_effects::Effect) -> Result<semio_framework::kernel::Effect, String> {
    use semio_framework::kernel::Effect as K;
    use wit_effects::Effect as E;
    Ok(match effect {
        E::SendMessage(inner) => K::SendMessage { target: wit_message_endpoint_to_kernel(inner.target).await, payload: inner.payload },
        E::PublishEvent(inner) => K::PublishEvent { topic: inner.topic, payload: inner.payload },
        E::BlobLoad(inner) => K::BlobLoad { req: semio_framework::kernel::RequestId(inner.req), hash: inner.params.hash },
        E::BlobWrite(inner) => {
            K::BlobWrite { req: semio_framework::kernel::RequestId(inner.req), media_type: decode_json(&inner.params.media_type).await.unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }), bytes: inner.params.bytes }
        }
        E::HttpRequest(inner) => K::HttpRequest { req: semio_framework::kernel::RequestId(inner.req), method: inner.params.method, url: inner.params.url, headers: inner.params.headers, body: inner.params.body, stream: inner.params.streaming },
        E::DocumentRead(inner) => K::DocumentRead { req: semio_framework::kernel::RequestId(inner.req), doc: semio_framework::kernel::ArtifactHandle(inner.params.doc as u128), lane: inner.params.lane },
        E::DocumentWrite(inner) => K::DocumentWrite { req: semio_framework::kernel::RequestId(inner.req), doc: semio_framework::kernel::ArtifactHandle(inner.params.doc as u128), lane: inner.params.lane, ops: inner.params.ops },
        E::LinkResolve(inner) => K::LinkResolve { req: semio_framework::kernel::RequestId(inner.req), link: String::from_utf8_lossy(&inner.link).into_owned() },
        E::RegistryQuery(inner) => K::RegistryQuery { req: semio_framework::kernel::RequestId(inner.req), kind: inner.params.kind, filter: decode_dsl(&inner.params.filter).await },
        E::IoCompose(inner) => K::IoCompose { req: semio_framework::kernel::RequestId(inner.req), key: String::from_utf8_lossy(&inner.params.key).into_owned(), sources: decode_json(&inner.params.sources).await.unwrap_or_default() },
        E::IoRun(_inner) => return Err("effect io-run has no semio_framework::kernel::Effect variant yet (needs A3 to add Effect::IoRun) — see 📓️terra-B1-host-native-report.md".to_string()),
        E::CacheDerive(inner) => K::CacheDerive { req: semio_framework::kernel::RequestId(inner.req), engine_id: inner.params.engine_id, input: inner.params.input },
        E::CacheRead(inner) => K::CacheRead { req: semio_framework::kernel::RequestId(inner.req), engine_id: inner.params.engine_id, key: String::from_utf8_lossy(&inner.params.key).into_owned() },
        E::OpenWindow(inner) => {
            K::OpenWindow { req: semio_framework::kernel::RequestId(inner.req), kind: semio_framework::kernel::WindowKindId(inner.params.kind), params: decode_dsl(&inner.params.params).await.unwrap_or(semio_framework::DslValue::Null) }
        }
        E::CloseWindow(inner) => K::CloseWindow { window: semio_framework::kernel::WindowHandle(inner.window as u128) },
        // 🚫️async: R10 residue shape 1 — `decode_dsl` is async, hoisted out of `Option::and_then`'s
        // sync closure below (and at every other `.args.and_then(|bytes| decode_dsl(&bytes))` site
        // in this match).
        E::DispatchAction(inner) => {
            let args = match inner.params.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            K::DispatchAction { req: semio_framework::kernel::RequestId(inner.req), action: inner.params.action, args, delay_ms: inner.params.delay_ms }
        }
        E::InvokeExtension(inner) => {
            K::InvokeExtension { req: semio_framework::kernel::RequestId(inner.req), extension_id: inner.params.extension_id, capability: inner.params.capability, request_json: String::from_utf8_lossy(&inner.params.payload).into_owned() }
        }
        E::Notify(inner) => K::Notify { message: inner.message },
        E::ClipboardWrite(inner) => K::ClipboardWrite { fragment: decode_json(&inner.fragment).await.ok_or_else(|| "clipboard-write-effect.fragment failed to decode as JSON ClipboardFragment".to_string())? },
        E::Navigate(inner) => K::Navigate { uri: inner.uri },
        E::OpenExternalUrl(inner) => K::OpenExternalUrl { url: inner.url },
        E::SetPanel(inner) => K::SetPanel { panel_json: inner.panel_json },
        E::SetActiveUtility(inner) => K::SetActiveUtility { window_id: inner.window_id, utility_id: inner.utility_id },
        E::SetActiveTool(inner) => K::SetActiveTool { tool_id: inner.tool_id },
        E::ReplayShellCommand(inner) => {
            let args = match inner.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            K::ReplayShellCommand { action_id: inner.action_id, args }
        }
        E::SpawnPluginInstance(inner) => K::SpawnPluginInstance {
            req: semio_framework::kernel::RequestId(inner.req),
            plugin_id: inner.params.plugin_id,
            app_id: inner.params.app_id,
            os_instance_id: inner.params.os_instance_id,
            label: inner.params.label,
            document_json: inner.params.document_json,
        },
        E::OpenPluginInstance(inner) => K::OpenPluginInstance { plugin_id: inner.plugin_id, app_id: inner.app_id, os_instance_id: inner.os_instance_id },
        E::OpenDialog(inner) => {
            let args = match inner.params.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            K::OpenDialog { req: semio_framework::kernel::RequestId(inner.req), dialog_id: inner.params.dialog_id, args }
        }
        E::IconRenderExport(inner) => K::IconRenderExport { items: decode_json(&inner.items).await.unwrap_or_default() },
        E::DownloadMediaExport(inner) => K::DownloadMediaExport { filename: inner.filename, mime_type: inner.mime_type, data: inner.data, encoding: inner.encoding },
        E::RequestFileOpen(inner) => K::RequestFileOpen { req: semio_framework::kernel::RequestId(inner.req), accept: inner.params.accept, read_as: inner.params.read_as, import_action: String::new(), multiple: inner.params.multiple },
        E::RequestMediaFrames(inner) => {
            let args = match inner.params.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            K::RequestMediaFrames {
                req: semio_framework::kernel::RequestId(inner.req),
                accept: inner.params.accept,
                frame_action: String::new(),
                done_action: String::new(),
                fallback_action: String::new(),
                sample_stride: inner.params.sample_stride,
                max_frames: inner.params.max_frames,
                max_long_edge_px: inner.params.max_long_edge_px,
                fps_hint: inner.params.fps_hint,
                payload: inner.params.payload,
                args,
            }
        }
        E::LoadDocument(inner) => K::LoadDocument { pack: inner.doc_pack, spr: inner.spr },
        E::RequestSync => K::RequestSync,
        E::SetTimer(inner) => K::SetTimer { id: inner.id, after_ms: inner.after_ms as u64, repeat: inner.repeat },
        E::SpawnJob(inner) => K::SpawnJob {
            job: inner.job,
            kind: inner.kind,
            input: inner.input,
            placement: match inner.placement {
                wit_effects::JobPlacement::Inline => semio_framework::kernel::JobPlacement::Inline,
                wit_effects::JobPlacement::Isolated => semio_framework::kernel::JobPlacement::Isolated,
                wit_effects::JobPlacement::Exclusive => semio_framework::kernel::JobPlacement::Exclusive,
            },
        },
        E::CancelJob(inner) => K::CancelJob { job: inner.job },
        E::Respond(inner) => K::Respond {
            req: semio_framework::kernel::RequestId(inner.req),
            result: match inner.outcome {
                wit_effects::RespondResult::Ok(bytes) => semio_framework::kernel::RequestOutcome::Ok(bytes),
                wit_effects::RespondResult::Fault(bytes) => semio_framework::kernel::RequestOutcome::Err(bytes),
            },
        },
        E::StorageRead(inner) => K::StorageRead { req: semio_framework::kernel::RequestId(inner.req), key: inner.params.key },
        E::StorageWrite(inner) => K::StorageWrite { req: semio_framework::kernel::RequestId(inner.req), key: inner.params.key, bytes: inner.params.value },
        E::StorageDelete(inner) => K::StorageDelete { req: semio_framework::kernel::RequestId(inner.req), key: inner.params.key },
        E::RequestCapability(inner) => K::RequestCapability {
            req: semio_framework::kernel::RequestId(inner.req),
            capability: semio_framework::kernel::CapabilityRequest { id: semio_framework::kernel::CapabilityId(inner.params.id), scope: inner.params.scope, reason: inner.params.reason, optional: inner.params.optional },
        },
        E::ReleaseCapability(inner) => K::ReleaseCapability { id: semio_framework::kernel::CapabilityId(inner.id) },
        E::Subscribe(inner) => K::Subscribe { topic: inner.topic },
        E::Unsubscribe(inner) => K::Unsubscribe { topic: inner.topic },
        E::RequestInferenceProposal(inner) => K::RequestInferenceProposal {
            kind: match inner.kind {
                wit_effects::InferenceProposalKind::GisMapBoundsRegion => semio_framework::kernel::InferenceProposalKind::GisMapBoundsRegion,
            },
        },
    })
}

#[cfg(test)]
mod effect_conversion_tests {
    use super::*;

    /// 💡️ Keeps the async host's schema conversion aligned with the typed kernel proposal intent.
    #[semio_framework_async_macros::async_test]
    async fn request_inference_proposal_preserves_the_closed_kind() {
        let effect = wit_effects::Effect::RequestInferenceProposal(wit_effects::RequestInferenceProposalEffect { kind: wit_effects::InferenceProposalKind::GisMapBoundsRegion });
        let converted = wit_effect_to_kernel(effect).await.expect("proposal intent converts");
        assert!(matches!(converted, semio_framework::kernel::Effect::RequestInferenceProposal { kind: semio_framework::kernel::InferenceProposalKind::GisMapBoundsRegion }));
    }
}
//#endregion 🐛️Conversions

//#region 🌿️pure::Host
/// 🌿️ Byte-for-byte the same behaviour as `actor_bindings`'s own `pure::Host for ActorHostState` —
/// there is one world now and it imports `pure` unchanged.
impl host_async_bindings::semio::framework::pure::Host for AsyncActorHostState {
    // 🚫️async: E1 — see `actor_bindings::…::pure::Host`'s own tag above; identical WIT contract.
    fn log(&mut self, level: String, message: String) {
        eprintln!("[actor-async:{}:{level}] {message}", self.actor);
    }

    fn now_ms(&mut self) -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as i64).unwrap_or(0)
    }

    fn trace_span(&mut self, name: String) {
        eprintln!("[actor-async:{}:trace] {name}", self.actor);
    }
}
//#endregion 🌿️pure::Host

//#region 🚪️host_async::Host (emit / emit-patch)
impl wit_host_async::Host for AsyncActorHostState {
    // 🚫️async: E1 — bindgen declares `emit`/`emit-patch` sync (fire-and-forget signal pushes),
    // unlike the rest of this world's imports. `wit_effect_to_kernel` itself must stay `async`
    // (it transitively awaits `store::pack_rt::decode_wire_value`, out of this packet's scope to
    // touch), so this is a genuine E5 sync/async bridge: `block_on` is sound here because the
    // whole decode chain is pure in-memory byte-buffer decoding with no real I/O, so it resolves
    // in its first poll and never actually parks — see R9/R2 E1+E5.
    fn emit(&mut self, value: wit_effects::Effect) {
        match semio_framework_async::block_on(wit_effect_to_kernel(value)) {
            Ok(effect) => self.effect_sink.push(effect),
            Err(error) => eprintln!("[actor-async:{}] emit: {error}", self.actor),
        }
    }

    fn emit_patch(&mut self, patch: wit_ui::UiPatch) {
        self.patch_sink.push(patch);
    }
}
//#endregion 🚪️host_async::Host

//#region ⏳️host_async::HostWithStore (the 24 async imports)
/// ⏳️ Shared tail for the 9 imports `⚡️effects/🦀️.rs`'s `RouterEffectHandler` already
/// answers (`blob-load`/`blob-write`/`document-read`/`document-write`/`io-compose`/`cache-derive`/
/// `cache-read`/`invoke-extension`/`dispatch-action`) — one explicit resumable router job, driven
/// one bounded step per worker closure and awaited inline to resolve the guest's future.
async fn run_router_effect(call: &CallContext, effect: crate::effects::RouterEffect, name: &str) -> Result<Vec<u8>, Vec<u8>> {
    let result = crate::effects::run_router_effect_job(call.services.compute.as_ref(), call.services.runtime.as_ref(), &call.scope, call.ctx.clone(), &call.router_handler, effect).await;
    match result {
        crate::effects::RouterEffectJobOutcome::Complete(bytes) => Ok(bytes),
        crate::effects::RouterEffectJobOutcome::Cancelled => Err(fault_bytes("capability-revoked", format!("{name} cancelled")).await),
        crate::effects::RouterEffectJobOutcome::Fault(error) => Err(fault_bytes("router-error", error).await),
        crate::effects::RouterEffectJobOutcome::DeadlineExceeded => Err(fault_bytes("deadline-exceeded", format!("{name} exceeded its deadline")).await),
        crate::effects::RouterEffectJobOutcome::WorkerLost => Err(fault_bytes("worker-lost", format!("{name} worker lost")).await),
    }
}

/// 🚧️ Shared tail for the 10 imports with no backing host service anywhere in this codebase today
/// (`link-resolve`, `registry-query`, `io-run`, `open-window`, `open-dialog`, `spawn-plugin-instance`,
/// `request-file-open`, `request-media-frames`, `request-capability`, `spawn-job`) — every one of
/// them still gets full `OperationContext` derivation, cancellation-before-dispatch and capability
/// tracking, so the WIRING is complete; only the final "call a real backend" step is a typed fault,
/// same "fails loudly" idiom `UnwiredRouterEffectHandler`/`UnwiredStorageBackend`/
/// `UnwiredHttpTransport` already establish elsewhere in this crate/`semio-framework-os-services`.
/// See the packet report's `## honest gaps` for exactly which real service each one is blocked on.
async fn not_wired(accessor: &Accessor<AsyncActorHostState, HasSelf<AsyncActorHostState>>, name: &str) -> Result<Vec<u8>, Vec<u8>> {
    let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
    let cancelled = call.ctx.cancel.is_cancelled();
    call.guard.disarm();
    if cancelled.await {
        return Err(fault_bytes("capability-revoked", format!("{name} cancelled before dispatch")).await);
    }
    Err(fault_bytes("not-wired", format!("host-async {name} has no backing host service yet — see 📓️terra-async-imports-report.md's honest gaps")).await)
}

impl wit_host_async::HostWithStore<AsyncActorHostState> for HasSelf<AsyncActorHostState> {
    //#region 💾️storage
    async fn storage_read(accessor: &Accessor<AsyncActorHostState, Self>, params: wit_effects::StorageReadParams) -> Result<Option<Vec<u8>>, Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "storage-read cancelled before dispatch").await);
        }
        let backend = call.services.storage_backend.clone();
        let key = params.key;
        let bytes_hint = key.len() as u64;
        let key_for_closure = key.clone();
        let result = match call.services.storage.submit(&call.ctx, call.package.clone(), bytes_hint, move || backend.read(&key_for_closure)) {
            Ok(ticket) => match ticket.await_result().await {
                Ok(bytes) => Ok(Some(bytes)),
                Err(StorageError::DeadlineExceeded) => Err(fault_bytes("deadline-exceeded", "storage-read exceeded its deadline").await),
                Err(error) => Err(fault_bytes("storage-error", error.to_string()).await),
            },
            Err(error @ StorageError::BytesQuotaExceeded { .. }) => Err(fault_bytes("quota-exceeded", error.to_string()).await),
            Err(error) => Err(fault_bytes("storage-error", error.to_string()).await),
        };
        call.guard.disarm();
        result
    }

    async fn storage_write(accessor: &Accessor<AsyncActorHostState, Self>, params: wit_effects::StorageWriteParams) -> Result<(), Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "storage-write cancelled before dispatch").await);
        }
        let backend = call.services.storage_backend.clone();
        let key = params.key;
        let value = params.value;
        let bytes_hint = (key.len() + value.len()) as u64;
        let key_for_closure = key.clone();
        let result = match call.services.storage.submit(&call.ctx, call.package.clone(), bytes_hint, move || backend.write(&key_for_closure, &value).map(|()| Vec::new())) {
            Ok(ticket) => match ticket.await_result().await {
                Ok(_) => Ok(()),
                Err(StorageError::DeadlineExceeded) => Err(fault_bytes("deadline-exceeded", "storage-write exceeded its deadline").await),
                Err(error) => Err(fault_bytes("storage-error", error.to_string()).await),
            },
            Err(error @ StorageError::BytesQuotaExceeded { .. }) => Err(fault_bytes("quota-exceeded", error.to_string()).await),
            Err(error) => Err(fault_bytes("storage-error", error.to_string()).await),
        };
        call.guard.disarm();
        result
    }

    async fn storage_delete(accessor: &Accessor<AsyncActorHostState, Self>, params: wit_effects::StorageDeleteParams) -> Result<(), Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "storage-delete cancelled before dispatch").await);
        }
        let backend = call.services.storage_backend.clone();
        let key = params.key;
        let key_for_closure = key.clone();
        let result = match call.services.storage.submit(&call.ctx, call.package.clone(), 0, move || backend.delete(&key_for_closure).map(|()| Vec::new())) {
            Ok(ticket) => match ticket.await_result().await {
                Ok(_) => Ok(()),
                Err(StorageError::DeadlineExceeded) => Err(fault_bytes("deadline-exceeded", "storage-delete exceeded its deadline").await),
                Err(error) => Err(fault_bytes("storage-error", error.to_string()).await),
            },
            Err(error @ StorageError::BytesQuotaExceeded { .. }) => Err(fault_bytes("quota-exceeded", error.to_string()).await),
            Err(error) => Err(fault_bytes("storage-error", error.to_string()).await),
        };
        call.guard.disarm();
        result
    }
    //#endregion 💾️storage

    //#region 🩹️blob
    async fn blob_load(accessor: &Accessor<AsyncActorHostState, Self>, params: wit_effects::BlobLoadParams) -> Result<Vec<u8>, Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "blob-load cancelled before dispatch").await);
        }
        let result = run_router_effect(&call, crate::effects::RouterEffect::BlobLoad { hash: params.hash }, "blob-load").await;
        call.guard.disarm();
        result
    }

    async fn blob_write(accessor: &Accessor<AsyncActorHostState, Self>, params: wit_effects::BlobWriteParams) -> Result<Vec<u8>, Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "blob-write cancelled before dispatch").await);
        }
        let media_type = decode_json(&params.media_type).await.unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
        let result = run_router_effect(&call, crate::effects::RouterEffect::BlobWrite { media_type, bytes: params.bytes }, "blob-write").await;
        call.guard.disarm();
        result
    }

    /// 🐌️ Single-chunk fallback — see `single_chunk_shared`'s own doc for why: no chunked blob
    /// backend exists in this codebase, and adding one is out of this packet's owned paths.
    async fn blob_read(accessor: &Accessor<AsyncActorHostState, Self>, hash: String) -> Result<StreamReader<u8>, Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "blob-read cancelled before dispatch").await);
        }
        let result = run_router_effect(&call, crate::effects::RouterEffect::BlobLoad { hash }, "blob-read").await;
        call.guard.disarm();
        let bytes = result?;
        let shared = single_chunk_shared(bytes).await;
        // 🚫️async: R10 residue shape 1 — `fault_bytes` is async, hoisted out of `map_err`'s sync
        // closure via an explicit match.
        match accessor.with(|access| StreamReader::new(access, ChunkStreamProducer { shared })) {
            Ok(reader) => Ok(reader),
            Err(error) => Err(fault_bytes("stream-error", error.to_string()).await),
        }
    }
    //#endregion 🩹️blob

    //#region 🌐️http
    /// 🌐️ The one genuinely CHUNKED import: `HttpPool::fetch` gives real per-chunk pulls
    /// (`HttpPoolBody::next_chunk`) — a background `spawn_scoped` task OWNS the body and loops
    /// pulling real chunks into `ChunkShared`, waking the guest's stream reader as they arrive. This
    /// is what actually fixes the poll bridge's "only keeps the FINAL chunk" gap the WIT doc calls
    /// out — see the packet report's `## streams`.
    async fn http_fetch(accessor: &Accessor<AsyncActorHostState, Self>, params: wit_effects::HttpParams) -> Result<wit_host_async::HttpResponse, Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "http-fetch cancelled before dispatch").await);
        }
        let request = ServiceHttpRequest { method: params.method, url: params.url, headers: params.headers, body: params.body.unwrap_or_default() };
        let fetch_result = call.services.http.fetch(call.services.runtime.as_ref(), &call.scope, call.ctx.clone(), call.package.clone(), call.actor_id, request).await;
        call.guard.disarm();
        let (head, body) = match fetch_result {
            Ok(pair) => pair,
            Err(error) => return Err(fault_bytes("http-error", error.to_string()).await),
        };
        let shared = Arc::new(Mutex::new(ChunkShared { queue: VecDeque::new(), done: false, waker: None }));
        let shared_for_task = shared.clone();
        let pull: HostFuture<()> = Box::pin(async move {
            let mut body = body;
            loop {
                let outcome = body.next_chunk().await;
                let finished = {
                    let mut sh = shared_for_task.lock().expect("ChunkShared mutex poisoned");
                    match outcome {
                        Ok(Some(chunk)) => sh.queue.push_back(chunk),
                        _ => sh.done = true,
                    }
                    sh.done
                };
                wake_chunk_shared(&shared_for_task).await;
                if finished {
                    break;
                }
            }
        });
        call.services.runtime.spawn_scoped(&call.scope, call.ctx, pull).await;
        // 🚫️async: R10 residue shape 1 — same `fault_bytes`-inside-`map_err` reasoning as `blob_read`.
        let body_reader = match accessor.with(|access| StreamReader::new(access, ChunkStreamProducer { shared })) {
            Ok(reader) => reader,
            Err(error) => return Err(fault_bytes("stream-error", error.to_string()).await),
        };
        Ok(wit_host_async::HttpResponse { status: head.status, headers: head.headers, body: body_reader })
    }
    //#endregion 🌐️http

    //#region 📄️document
    async fn document_read(accessor: &Accessor<AsyncActorHostState, Self>, params: wit_effects::DocumentReadParams) -> Result<Vec<u8>, Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "document-read cancelled before dispatch").await);
        }
        let result = run_router_effect(&call, crate::effects::RouterEffect::DocumentRead { doc: params.doc as u128, lane: params.lane }, "document-read").await;
        call.guard.disarm();
        result
    }

    async fn document_write(accessor: &Accessor<AsyncActorHostState, Self>, params: wit_effects::DocumentWriteParams) -> Result<Vec<u8>, Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "document-write cancelled before dispatch").await);
        }
        let result = run_router_effect(&call, crate::effects::RouterEffect::DocumentWrite { doc: params.doc as u128, lane: params.lane, ops: params.ops }, "document-write").await;
        call.guard.disarm();
        result
    }
    //#endregion 📄️document

    //#region 🧩️composition / cache / extension / action
    async fn io_compose(accessor: &Accessor<AsyncActorHostState, Self>, params: wit_effects::IoComposeParams) -> Result<Vec<u8>, Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "io-compose cancelled before dispatch").await);
        }
        let key = String::from_utf8_lossy(&params.key).into_owned();
        let sources = decode_json(&params.sources).await.unwrap_or_default();
        let result = run_router_effect(&call, crate::effects::RouterEffect::IoCompose { key, sources }, "io-compose").await;
        call.guard.disarm();
        result
    }

    async fn cache_derive(accessor: &Accessor<AsyncActorHostState, Self>, params: wit_effects::CacheDeriveParams) -> Result<Vec<u8>, Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "cache-derive cancelled before dispatch").await);
        }
        let result = run_router_effect(&call, crate::effects::RouterEffect::CacheDerive { engine_id: params.engine_id, input: params.input }, "cache-derive").await;
        call.guard.disarm();
        result
    }

    async fn cache_read(accessor: &Accessor<AsyncActorHostState, Self>, params: wit_effects::CacheReadParams) -> Result<Vec<u8>, Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "cache-read cancelled before dispatch").await);
        }
        let key = String::from_utf8_lossy(&params.key).into_owned();
        let result = run_router_effect(&call, crate::effects::RouterEffect::CacheRead { engine_id: params.engine_id, key }, "cache-read").await;
        call.guard.disarm();
        result
    }

    async fn invoke_extension(accessor: &Accessor<AsyncActorHostState, Self>, params: wit_effects::InvokeExtensionParams) -> Result<Vec<u8>, Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "invoke-extension cancelled before dispatch").await);
        }
        let request_json = String::from_utf8_lossy(&params.payload).into_owned();
        let result = run_router_effect(&call, crate::effects::RouterEffect::InvokeExtension { extension_id: params.extension_id, capability: params.capability, request_json }, "invoke-extension").await;
        call.guard.disarm();
        result
    }

    async fn dispatch_action(accessor: &Accessor<AsyncActorHostState, Self>, params: wit_effects::DispatchActionParams) -> Result<Vec<u8>, Vec<u8>> {
        let call = begin_call(accessor.with(|mut access| snapshot_call(access.get()))).await;
        if call.ctx.cancel.is_cancelled().await {
            call.guard.disarm();
            return Err(fault_bytes("capability-revoked", "dispatch-action cancelled before dispatch").await);
        }
        let args = match params.args {
            Some(bytes) => decode_dsl(&bytes).await,
            None => None,
        };
        let result = run_router_effect(&call, crate::effects::RouterEffect::DispatchAction { action: params.action, args, delay_ms: params.delay_ms }, "dispatch-action").await;
        call.guard.disarm();
        result
    }
    //#endregion 🧩️composition / cache / extension / action

    //#region 🚧️not-yet-wired (no host service exists for these yet)
    async fn link_resolve(accessor: &Accessor<AsyncActorHostState, Self>, _link: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        not_wired(accessor, "link-resolve").await
    }

    async fn registry_query(accessor: &Accessor<AsyncActorHostState, Self>, _params: wit_effects::RegistryQueryParams) -> Result<Vec<u8>, Vec<u8>> {
        not_wired(accessor, "registry-query").await
    }

    async fn io_run(accessor: &Accessor<AsyncActorHostState, Self>, _params: wit_effects::IoRunParams) -> Result<Vec<u8>, Vec<u8>> {
        not_wired(accessor, "io-run").await
    }

    async fn open_window(accessor: &Accessor<AsyncActorHostState, Self>, _params: wit_effects::OpenWindowParams) -> Result<Vec<u8>, Vec<u8>> {
        not_wired(accessor, "open-window").await
    }

    async fn open_dialog(accessor: &Accessor<AsyncActorHostState, Self>, _params: wit_effects::OpenDialogParams) -> Result<Vec<u8>, Vec<u8>> {
        not_wired(accessor, "open-dialog").await
    }

    async fn spawn_plugin_instance(accessor: &Accessor<AsyncActorHostState, Self>, _params: wit_effects::SpawnPluginInstanceParams) -> Result<Vec<u8>, Vec<u8>> {
        not_wired(accessor, "spawn-plugin-instance").await
    }

    async fn request_file_open(accessor: &Accessor<AsyncActorHostState, Self>, _params: wit_effects::RequestFileOpenParams) -> Result<Vec<u8>, Vec<u8>> {
        not_wired(accessor, "request-file-open").await
    }

    async fn request_media_frames(accessor: &Accessor<AsyncActorHostState, Self>, _params: wit_effects::RequestMediaFramesParams) -> Result<Vec<u8>, Vec<u8>> {
        not_wired(accessor, "request-media-frames").await
    }

    async fn request_capability(accessor: &Accessor<AsyncActorHostState, Self>, _params: wit_effects::RequestCapabilityParams) -> Result<Vec<u8>, Vec<u8>> {
        not_wired(accessor, "request-capability").await
    }

    async fn spawn_job(accessor: &Accessor<AsyncActorHostState, Self>, _job: u64, _kind: String, _input: Vec<u8>, _placement: wit_effects::JobPlacement) -> Result<Vec<u8>, Vec<u8>> {
        not_wired(accessor, "spawn-job").await
    }
    //#endregion 🚧️not-yet-wired
}
//#endregion ⏳️host_async::HostWithStore (the 24 async imports)

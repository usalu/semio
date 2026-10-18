//! 🆔️ Actor registries and envelope injector: the trace id allocator, the actor scope registry, the capability revocation registry, the lane deadlines, the envelope injector seam and the completion mailbox. See [`super`].

use super::*;

//#region 🆔️TraceIdAllocator
/// 🆔️ Monotonic `TraceId` source, one per host — every dispatched operation gets a fresh id from
/// here, never reused, so a trace correlates exactly one operation end to end.
#[derive(Default)]
pub struct TraceIdAllocator(pub AtomicU64);

impl TraceIdAllocator {
    pub async fn new() -> Self {
        Self(AtomicU64::new(1))
    }

    pub async fn next(&self) -> TraceId {
        TraceId(self.0.fetch_add(1, Ordering::SeqCst))
    }
}
//#endregion 🆔️TraceIdAllocator

//#region 🪪️ActorScopeRegistry
/// 🪪️ What deriving an `OperationContext` for one actor's effects needs, and what
/// [`EnvelopeCompletionSink`] needs to gate/buffer a completion: the actor's own `ScopeHandle`
/// (root of every `CancelToken::child` this actor's operations derive from) plus the generation
/// snapshot completions are checked against. A restart calls [`ActorScopeRegistry::activate`]
/// again with the bumped generation — the OLD record is simply replaced; any operation still
/// holding the OLD scope's cancel token keeps its own lineage (`CancelToken::child`'s own doc), it
/// just can no longer deliver a completion once the actor's registry entry has moved on (its
/// snapshotted `generation` no longer matches — see `EnvelopeCompletionSink::complete`).
struct ActorRecord {
    pub(super) scope: ScopeHandle,
    pub(super) generation: u16,
}

#[derive(Clone)]
pub struct ActorScopeRegistry(pub(super) Arc<Mutex<HashMap<u64, ActorRecord>>>);

impl ActorScopeRegistry {
    /// 🚫️async: E1 pure constructor consumed by `impl Default` (external trait, sync-only) — R9.
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }

    /// 🌱️ (Re)activates `actor` (the STABLE identity — plugin/kind/ordinal bits, generation bits
    /// ignored, see [`addressed_actor_id`]) at `generation`, opening a fresh child scope under
    /// `package_scope`. Must be called before any of this actor's effects are dispatched, and again
    /// on every restart with the bumped generation.
    /// 🔀️ dedyn-emit-runtime, O1/R3: generic over `R: HostAsyncRuntime` (Send-ness derived
    /// structurally at each caller's own concrete `R`, never a bound on this fn) rather than `&dyn
    /// HostAsyncRuntime` — mirrors `db_storage`'s `R: HostAsyncRuntime` holders.
    pub async fn activate<R: HostAsyncRuntime>(&self, runtime: &R, actor: u64, generation: u16, package_scope: &ScopeHandle) -> ScopeHandle {
        let scope = runtime.open_scope(ScopeOwner::Actor(actor), Some(package_scope)).await;
        self.0.lock().expect("ActorScopeRegistry mutex poisoned").insert(actor, ActorRecord { scope: scope.clone(), generation });
        scope
    }

    pub async fn scope_for(&self, actor: u64) -> Option<ScopeHandle> {
        self.0.lock().expect("ActorScopeRegistry mutex poisoned").get(&actor).map(|record| record.scope.clone())
    }

    pub async fn generation_of(&self, actor: u64) -> Option<u16> {
        self.0.lock().expect("ActorScopeRegistry mutex poisoned").get(&actor).map(|record| record.generation)
    }

    pub async fn deactivate(&self, actor: u64) {
        self.0.lock().expect("ActorScopeRegistry mutex poisoned").remove(&actor);
    }
}

impl Default for ActorScopeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 🧭️ Reconstructs the addressed `ActorId` for delivery: takes the stable plugin/kind/ordinal bits
/// from `actor_stable` (whatever generation bits it happened to carry are discarded) and ORs in
/// `generation` — always correct regardless of what `actor_stable`'s own low 14 bits were, per
/// `OperationContext.generation`'s own doc ("a different concept from ActorId's packed 14-bit
/// restart-generation bits").
pub(super) async fn addressed_actor_id(actor_stable: u64, generation: u16) -> RuntimeActorId {
    let stable = RuntimeActorId(actor_stable);
    RuntimeActorId::new(stable.plugin_ordinal(), stable.kind_tag(), stable.ordinal(), generation).await
}
//#endregion 🪪️ActorScopeRegistry

//#region 🔑️CapabilityRevocationRegistry
/// 🔑️ Tracks which live `CancelToken`s were derived under which `CapabilityTokenId` — see
/// `AsyncEffectExecutor`'s per-effect dispatch, which registers every operation's OWN child token
/// here whenever it names a capability. [`CapabilityRevocationRegistry::revoke`] cancels only
/// those tokens, never the actor's own scope token, so the actor survives a capability revocation
/// (bench budget 8).
#[derive(Clone)]
pub struct CapabilityRevocationRegistry(pub(super) Arc<Mutex<HashMap<CapabilityTokenId, Vec<CancelToken>>>>);

impl CapabilityRevocationRegistry {
    /// 🚫️async: E1 pure constructor consumed by `impl Default` (external trait, sync-only) — R9.
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }

    pub(super) async fn track(&self, capability: CapabilityTokenId, token: CancelToken) {
        self.0.lock().expect("CapabilityRevocationRegistry mutex poisoned").entry(capability).or_default().push(token);
    }

    /// 🛑️ Cancels every child token registered under `capability` — cooperative: each operation's
    /// own future body is the thing that actually observes `ctx.cancel.is_cancelled()` and emits
    /// the `capability-revoked` completion (see `emit_completed_err` call sites below). Clears the
    /// registered list for `capability` so a later revoke of the same (already-revoked) id is a
    /// harmless no-op.
    pub async fn revoke(&self, capability: CapabilityTokenId) {
        let tokens = self.0.lock().expect("CapabilityRevocationRegistry mutex poisoned").remove(&capability);
        if let Some(tokens) = tokens {
            for token in tokens {
                token.cancel().await;
            }
        }
    }
}

impl Default for CapabilityRevocationRegistry {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🔑️CapabilityRevocationRegistry

//#region ⏱️Deadlines
/// ⏱️ Host-enforced deadline ceiling per lane (`OperationContext.lane`'s own bare-`u8` convention,
/// mirroring `semio_framework_actor::Lane`'s discriminant order: 0=Interactive..3=Maintenance) —
/// an effect's OWN requested deadline is clamped to never exceed this, so a misbehaving/absent
/// per-effect deadline can never starve a higher-priority lane's fair share of `ComputePool`.
const LANE_DEADLINE_CEILING_MS: [u64; 4] = [2_000, 5_000, 30_000, 120_000];

async fn lane_ceiling_ms(lane: u8) -> u64 {
    LANE_DEADLINE_CEILING_MS.get(lane as usize).copied().unwrap_or(*LANE_DEADLINE_CEILING_MS.last().expect("non-empty"))
}

/// ⏱️ Clamps `effect_deadline_ms` (if the effect stated one) to the lane's own ceiling from
/// `now_ms` — always returns `Some`, since every dispatched operation gets a host-enforced deadline
/// even when the effect itself did not ask for one.
pub(super) async fn clamp_deadline_ms(now_ms: u64, effect_deadline_ms: Option<u64>, lane: u8) -> u64 {
    let ceiling = now_ms.saturating_add(lane_ceiling_ms(lane).await);
    match effect_deadline_ms {
        Some(requested) => requested.min(ceiling),
        None => ceiling,
    }
}
//#endregion ⏱️Deadlines

//#region 📨️EnvelopeInjector
/// 📨️ The seam a live kernel/shard packet implements to actually deliver a completion envelope
/// into an actor's inbound mailbox — mirrors `CompletionSink`'s own "only re-entry point"
/// discipline one level up: [`EnvelopeCompletionSink`] is the ONE place that constructs an
/// `Envelope` from a completed effect, and `EnvelopeInjector` is the ONE place that hands it
/// onward. No implementation ships in this packet (see the module doc's own "does NOT close" note
/// and the packet report's `## honest gaps`) — [`RecordingEnvelopeInjector`] below exists for
/// tests only.
pub trait EnvelopeInjector: Send + Sync {
    async fn inject(&self, envelope: Envelope);
}

/// 🧪️ Test double recording every injected `Envelope` in order.
#[derive(Clone, Default)]
pub struct RecordingEnvelopeInjector(pub(super) Arc<Mutex<Vec<Envelope>>>);

impl RecordingEnvelopeInjector {
    pub async fn new() -> Self {
        Self::default()
    }

    pub async fn recorded(&self) -> Vec<Envelope> {
        self.0.lock().expect("RecordingEnvelopeInjector mutex poisoned").clone()
    }
}

impl EnvelopeInjector for RecordingEnvelopeInjector {
    async fn inject(&self, envelope: Envelope) {
        self.0.lock().expect("RecordingEnvelopeInjector mutex poisoned").push(envelope);
    }
}
//#endregion 📨️EnvelopeInjector

//#region 🚰️CompletionMailbox
/// 🚰️ How many completions may sit buffered for one actor (parked, or simply not yet drained)
/// before a further completion is REJECTED rather than queued without bound — see
/// `EnvelopeCompletionSink::complete`'s own doc. Generous: this is a safety bound against a
/// misbehaving/runaway service, not a steady-state throttle.
pub const COMPLETION_MAILBOX_CAP: u32 = 512;

/// 🚰️ Byte-side twin of [`COMPLETION_MAILBOX_CAP`] — `ChannelPolicy::LosslessBounded` now bounds
/// BOTH items and bytes (Phase 1, P1a); same "generous safety bound, not a steady-state throttle"
/// rationale, sized well above any real completion payload.
pub const COMPLETION_MAILBOX_MAX_BYTES: u64 = 1_000_000;

pub(super) fn completion_topic(actor: u64) -> Topic {
    Topic(format!("__effect_completions__:{actor}"))
}

/// 🧵️ `EventRouter`'s `Mailbox` payload is a bare `Vec<u8>` with no room for `lane` — this
/// prefixes the single `lane` byte so [`EnvelopeCompletionSink::flush`] can recover it on drain.
pub(super) async fn encode_mailbox_entry(lane: u8, event_bytes: &[u8]) -> Vec<u8> {
    let mut wire = Vec::with_capacity(1 + event_bytes.len());
    wire.push(lane);
    wire.extend_from_slice(event_bytes);
    wire
}

pub(super) async fn decode_mailbox_entry(wire: &[u8]) -> (u8, &[u8]) {
    match wire.split_first() {
        Some((lane, rest)) => (*lane, rest),
        None => (0, &[]),
    }
}
//#endregion 🚰️CompletionMailbox

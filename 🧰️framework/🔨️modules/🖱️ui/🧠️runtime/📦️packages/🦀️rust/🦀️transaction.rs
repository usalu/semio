//! @emoji 🔄️ The frame transaction: drain, dispatch, flush, present, reconcile, collect.
//!
//! [`FrameTransaction::step`] is the persistent scheduler-bounded state machine that ties every
//! landed sibling into one atomically published frame: `crate::EntityStore` for state and mutation leasing, `crate::DependencyTracker` for
//! actual-read dependency edges, `crate::Present`/`crate::ComponentTree` for presentation,
//! `crate::SurfaceReconciler` for keyed diffing into `ui_contract::UiPatch`, `crate::CommandGateway`
//! for the non-blocking actor seam, and `crate::PresenceHub` for the TTL-scoped presence channel.
//!
//! **The `crate::DependencyTracker` bridge.** Nothing in `crate::EntityStore`/`crate::Context` itself
//! calls into `crate::DependencyTracker` — `Context::notify` only ever queues an `crate::EntityId`
//! onto `EntityStore`'s own (`pub(crate)`) effect queues. This file is the one place that reads those
//! queued ids, before each `crate::EntityStore::flush_effects` cycle, and turns them into
//! `crate::DependencyTracker::notify_entity` calls — the wiring the module doc of every landed sibling
//! describes as this packet's job to supply.
//!
//! **Surfaces are registered `dyn`-free (U3).** [`SurfaceSlot`] pairs a type-erased presenter
//! `crate::Entity<P>` with a `crate::SurfaceReconciler`, using the same fn-pointer-vtable-behind-
//! `Box<dyn Any>` technique `ui-render`'s own `AnySurface`/`SurfaceVTable` already uses — never `dyn
//! HandleIntent`/`dyn Present`.
//!
//! No stage is `async`: cooperative progress comes from returning [`FrameTransactionStep::Yield`]
//! under `semio_framework_job::StepContext`, never from hiding CPU work in a future.

use std::any::Any;
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::{size_of, take};

//#region 🔖️Runtime

//#region 🧬️Erasure

type ErasedDispatchFn = fn(&dyn Any, &ui_contract::UiIntent, &mut crate::EntityStore) -> crate::DispatchOutcome;
type ErasedPresentFn = fn(&dyn Any, &mut crate::PresentCx<'_>) -> crate::ComponentTree;
type DeferredEffect = Box<dyn FnOnce(&mut crate::EntityStore)>;

/// 🧬️ Two monomorphized `fn` items per concrete presenter `P` — the fn-pointer-vtable technique this
/// file's top docstring points at, applied to [`crate::HandleIntent`]'s and [`crate::Present`]'s one
/// method each.
#[derive(Clone, Copy)]
struct PresenterVTable {
    dispatch: ErasedDispatchFn,
    present: ErasedPresentFn,
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn dispatch_erased<P: crate::HandleIntent + 'static>(presenter: &dyn Any, intent: &ui_contract::UiIntent, store: &mut crate::EntityStore) -> crate::DispatchOutcome {
    let entity = presenter.downcast_ref::<crate::Entity<P>>().expect("SurfaceSlot: vtable/presenter type mismatch — see SurfaceSlot::new");
    store.update(entity, |value, cx| value.on_intent(intent, cx))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn present_erased<P: crate::Present + 'static>(presenter: &dyn Any, cx: &mut crate::PresentCx<'_>) -> crate::ComponentTree {
    let entity = presenter.downcast_ref::<crate::Entity<P>>().expect("SurfaceSlot: vtable/presenter type mismatch — see SurfaceSlot::new");
    let value = cx.read(entity);
    value.present(cx)
}

/// 🧱️ One registered surface: a type-erased presenter entity plus the [`crate::SurfaceReconciler`]
/// that turns its presents into patches. Never `Box<dyn Present>`/`Box<dyn HandleIntent>` (ruling U3)
/// — see this file's top docstring.
struct SurfaceSlot {
    presenter: Box<dyn Any>,
    vtable: PresenterVTable,
    reconciler: crate::SurfaceReconciler,
}

impl SurfaceSlot {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn new<P: crate::HandleIntent + crate::Present + 'static>(presenter: crate::Entity<P>, reconciler: crate::SurfaceReconciler) -> Self {
        Self { presenter: Box::new(presenter), vtable: PresenterVTable { dispatch: dispatch_erased::<P>, present: present_erased::<P> }, reconciler }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn dispatch(&self, intent: &ui_contract::UiIntent, store: &mut crate::EntityStore) -> crate::DispatchOutcome {
        (self.vtable.dispatch)(self.presenter.as_ref(), intent, store)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn present(&self, cx: &mut crate::PresentCx<'_>) -> crate::ComponentTree {
        (self.vtable.present)(self.presenter.as_ref(), cx)
    }

    /// 🔢️ The revision `crate::dispatch::is_stale_intent` compares an incoming intent against. Reads
    /// through the reconciler's allocation-free revision accessor.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn current_revision(&self) -> ui_contract::UiRevision {
        self.reconciler.revision()
    }
}

impl Drop for SurfaceSlot {
    fn drop(&mut self) { self.reconciler.close_transaction_oracle(); }
}

//#endregion 🧬️Erasure

/// 💥️ A fault [`FrameTransaction::step`] surfaces in its result instead of hanging or silently dropping
/// work — never a panic, since one authoring bug in one presenter must not take down a frame
/// transaction driving many others.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactFault {
    /// 🌀️ The effect fixpoint (`crate::EntityStore::flush_effects`, looped) did not settle within
    /// [`EFFECT_STORM_BUDGET`] cycles — a listener is re-notifying its own source (or a cycle of
    /// sources) forever. `pending_notify`/`pending_emit_sources` name every `crate::EntityId` still
    /// queued at the moment the budget was exhausted, enough to find the offending `observe`/
    /// `subscribe` registration by cross-referencing whichever entity owns that id.
    EffectStorm { cycles: u32, pending_notify: Vec<crate::EntityId>, pending_emit_sources: Vec<crate::EntityId> },
    /// 📏️ The transaction exceeded one of its hard item, node, or byte credits. Candidate
    /// presentation work is discarded; accepted commands remain reported exactly once.
    CreditsExceeded { usage: FrameTransactionUsage, limits: FrameTransactionLimits },
}

/// 🌀️ How many `crate::EntityStore::flush_effects` cycles one frame transaction permits before
/// treating an unsettled effect fixpoint as [`TransactFault::EffectStorm`] rather than spinning
/// further (ticket-mandated).
pub const EFFECT_STORM_BUDGET: u32 = 64;

/// 📥️ How many projection deltas one [`FrameTransaction`] drains from the inbox at most —
/// bounds this step's own work so one slow projection cannot itself blow a frame budget.
pub const PROJECTION_DRAIN_LIMIT: usize = 256;

/// 📦️ Everything one completed [`FrameTransaction`] produced, ready for the embedder to ship: patches
/// for the renderer, commands that cleared the gateway this transaction, presence updates on their own
/// channel (never inside `patches`), any [`TransactFault`] worth logging, and the earliest deadline
/// (if any) that should wake the next transaction.
#[derive(Debug, Default)]
pub struct Transacted {
    pub patches: Vec<TransactionPatch>,
    pub commands: Vec<crate::Command>,
    pub presence: Vec<ui_contract::PresenceUpdate>,
    pub faults: Vec<TransactFault>,
    pub next_wake_ms: Option<u64>,
}

/// 🧪️ Test-only paired publication owner; serialization borrows its exact retained payload.
pub struct TransactionPatch {
    payload: ui_contract::UiPendingPatch,
    published: Option<crate::SurfaceReconcilePublishedPatch>,
}

impl TransactionPatch {
    fn from_ready(ready: &mut crate::SurfaceReconcileReadyPatch) -> Self {
        let mut owner = Self { payload: ui_contract::UiPendingPatch::default(), published: None };
        assert!(ready.publish_into(&mut owner.payload, &mut owner.published, crate::SurfaceReconcileReadyPatch::required_publish_bytes()).unwrap() > 0);
        assert!(ready.terminal_is_empty());
        owner
    }
}

impl std::ops::Deref for TransactionPatch {
    type Target = ui_contract::UiPatch;
    fn deref(&self) -> &Self::Target { self.payload.get().expect("test publication retains its payload") }
}

impl std::fmt::Debug for TransactionPatch {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(&**self, formatter) }
}

impl serde::Serialize for TransactionPatch {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { serde::Serialize::serialize(&**self, serializer) }
}

impl Drop for TransactionPatch {
    fn drop(&mut self) {
        while !self.payload.terminal_is_empty() { self.payload.close_step(1, 4096).unwrap(); }
        if let Some(published) = self.published.as_mut() { while !published.close_step_with_grant(1, 4096).unwrap().complete {} }
        self.published = None;
    }
}

/// 🧠️ The headless runtime for one embedder: every entity, the dependency graph presenting reads
/// through, the bounded projection inbox, the command gateway, the presence hub, and every registered
/// surface. `D` is the embedder's own projection-delta shape (`crate::ProjectionDelta`); `apply_delta`
/// is the fn-pointer thunk (never `dyn Fn`, ruling U3) that turns one drained delta into whatever
/// entity mutation it represents — this crate has no way to know that shape on its own.
pub struct UiRuntime<S: crate::CommandSink, D: crate::ProjectionDelta> {
    store: crate::EntityStore,
    tracking: crate::DependencyTracker,
    inbox: crate::ProjectionInbox<D>,
    apply_delta: fn(&mut crate::EntityStore, D),
    gateway: crate::CommandGateway<S>,
    presence: crate::PresenceHub,
    surfaces: HashMap<ui_contract::SurfaceId, SurfaceSlot>,
    /// 🌱️ Surfaces registered since the last `transact` that have never yet been presented —
    /// `crate::DependencyTracker::notify_entity` can only mark a surface dirty from a *previous*
    /// recorded read, so a brand-new surface needs this explicit seed for its unconditional first
    /// present (mirroring `crate::SurfaceReconciler::new`'s own "first reconcile always emits
    /// everything" contract).
    pending_first_present: HashSet<ui_contract::SurfaceId>,
    pending_intents: VecDeque<ui_contract::UiIntent>,
    custom_handlers: HashMap<&'static str, fn(&mut crate::EntityStore)>,
    pending_wakes: Vec<u64>,
    input_epoch: u64,
}

impl<S: crate::CommandSink, D: crate::ProjectionDelta> UiRuntime<S, D> {
    /// 🏭️ A runtime over `gateway`, whose `crate::ProjectionInbox` holds at most `inbox_capacity`
    /// distinct delta keys at once, and whose `apply_delta` thunk applies each drained delta.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(gateway: crate::CommandGateway<S>, inbox_capacity: usize, apply_delta: fn(&mut crate::EntityStore, D)) -> Self {
        Self {
            store: crate::EntityStore::new(),
            tracking: crate::DependencyTracker::default(),
            inbox: crate::ProjectionInbox::new(inbox_capacity),
            apply_delta,
            gateway,
            presence: crate::PresenceHub::new(),
            surfaces: HashMap::new(),
            pending_first_present: HashSet::new(),
            pending_intents: VecDeque::new(),
            custom_handlers: HashMap::new(),
            pending_wakes: Vec::new(),
            input_epoch: 0,
        }
    }

    /// 🗄️ Direct access to the entity store — for constructing presenter/model entities before
    /// registering a surface. `transact` is the only place this crate itself calls
    /// `crate::EntityStore::update` outside a `crate::HandleIntent` handler.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn store_mut(&mut self) -> &mut crate::EntityStore {
        self.input_epoch = self.input_epoch.wrapping_add(1);
        &mut self.store
    }

    /// 🧱️ Pairs a presenter entity with the `crate::SurfaceReconciler` that will turn its presents
    /// into patches for `surface`. The caller constructs both — `crate::EntityStore::insert` (via
    /// [`Self::store_mut`]) for the presenter, `crate::SurfaceReconciler::new` for the reconciler —
    /// registration only pairs them, never `dyn Present`/`dyn HandleIntent` (ruling U3; see
    /// [`SurfaceSlot`]'s own docstring). The next `transact` unconditionally presents this surface
    /// once, regardless of dependency-tracker dirtiness, so a freshly registered surface's first
    /// patch always arrives.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn register_surface<P: crate::HandleIntent + crate::Present + 'static>(&mut self, surface: ui_contract::SurfaceId, presenter: crate::Entity<P>, reconciler: crate::SurfaceReconciler) {
        self.input_epoch = self.input_epoch.wrapping_add(1);
        self.surfaces.insert(surface.clone(), SurfaceSlot::new(presenter, reconciler));
        self.pending_first_present.insert(surface);
    }

    /// ➕️ Queues one `ui_contract::UiIntent` for the next `transact` call to route — the only way an
    /// intent enters this runtime. `transact` drains this queue in FIFO order every call.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn submit_intent(&mut self, intent: ui_contract::UiIntent) {
        self.input_epoch = self.input_epoch.wrapping_add(1);
        self.pending_intents.push_back(intent);
    }

    /// 📥️ The bounded, coalescing entry point for a projection delta — a CQRS subscription pushes
    /// here; `transact` drains and applies it.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_delta(&mut self, delta: D) -> Result<(), crate::InboxOverflow> {
        let result = self.inbox.push(delta);
        if result.is_ok() {
            self.input_epoch = self.input_epoch.wrapping_add(1);
        }
        result
    }

    /// 🧩️ Registers the fn-pointer thunk `crate::DeferredOp::Custom(key)` invokes — a key with no
    /// registered handler is a safe no-op, never a panic (see `crate::DeferredOp::Custom`'s
    /// docstring).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn register_custom_deferred(&mut self, key: crate::DeferredKey, handler: fn(&mut crate::EntityStore)) {
        self.custom_handlers.insert(key.0, handler);
    }

    /// ⏰️ Requests `transact` report `at_ms` as (or before) its `Transacted::next_wake_ms` until that
    /// deadline has passed — the mechanism behind "compute `next_wake_ms` from the earliest pending
    /// deadline" for anything this crate cannot infer a deadline for on its own (a presenter's own
    /// animation/debounce/retry timer, say).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn request_wake(&mut self, at_ms: u64) {
        self.input_epoch = self.input_epoch.wrapping_add(1);
        self.pending_wakes.push(at_ms);
    }

    /// 📮️ Read-only access to this runtime's command gateway — e.g. for a presenter to layer
    /// `crate::OptimisticStatus` over its own projection read.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn gateway(&self) -> &crate::CommandGateway<S> {
        &self.gateway
    }
}

//#endregion 🔖️Runtime

//#region 🔖️Transact

/// 🧭️ The exact persistent frame stages, in deterministic execution order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FrameTransactionStage {
    DrainProjectionDeltas,
    RouteIntents,
    FlushEffects,
    PresentSurface,
    ReconcileTree,
    BuildRenderPackets,
    PublishSnapshot,
}

/// 🚧️ Hard credits for one consistent frame transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameTransactionLimits {
    pub max_items: usize,
    pub max_nodes: usize,
    pub max_bytes: usize,
}

impl Default for FrameTransactionLimits {
    fn default() -> Self {
        Self { max_items: 262_144, max_nodes: 262_144, max_bytes: 64 * 1024 * 1024 }
    }
}

/// 📏️ Credits consumed by the current unpublished transaction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FrameTransactionUsage {
    pub items: usize,
    pub nodes: usize,
    pub bytes: usize,
}

impl FrameTransactionUsage {
    pub fn fits(self, limits: FrameTransactionLimits) -> bool {
        self.items <= limits.max_items && self.nodes <= limits.max_nodes && self.bytes <= limits.max_bytes
    }
}

/// 🚦️ Result of one bounded [`FrameTransaction::step`] call.
#[derive(Debug)]
pub enum FrameTransactionStep {
    Yield { stage: FrameTransactionStage, usage: FrameTransactionUsage },
    Published(Transacted),
    Cancelled(Transacted),
}

struct ReconciledSurface {
    surface: ui_contract::SurfaceId,
    reconciler: Option<crate::SurfaceReconciler>,
    patch: Option<crate::SurfaceReconcileReadyPatch>,
}

impl Drop for ReconciledSurface {
    fn drop(&mut self) {
        if let Some(patch) = self.patch.as_mut() { while !patch.close_step_with_grant(1, 4096).unwrap().complete {} }
        if let Some(reconciler) = self.reconciler.as_mut() { reconciler.close_transaction_oracle(); }
    }
}

struct ActiveReconcile {
    surface: ui_contract::SurfaceId,
    generation: u64,
    job: Option<crate::SurfaceReconcileJob>,
    current: Option<crate::SurfaceReconciler>,
    ready: Option<crate::SurfaceReconcileReadyPatch>,
}

impl Drop for ActiveReconcile {
    fn drop(&mut self) {
        if let Some(job) = self.job.take() { let mut terminal = job.into_terminal(); while !terminal.close_step() {} }
        if let Some(ready) = self.ready.as_mut() { while !ready.close_step_with_grant(1, 4096).unwrap().complete {} }
        if let Some(current) = self.current.as_mut() { current.close_transaction_oracle(); }
    }
}

/// 🧪️ Test-only frame oracle. Reconciliation reads the same canonical root through an alias and
/// becomes visible only in `PublishSnapshot`, so newer input can discard obsolete trees and patches
/// without corrupting revision state or losing commands already accepted by the gateway.
pub struct FrameTransaction {
    stage: FrameTransactionStage,
    now_ms: u64,
    started: bool,
    observed_input_epoch: u64,
    force_all_surfaces: bool,
    projection_drained: usize,
    effect_cycles: u32,
    effects_settled: bool,
    limits: FrameTransactionLimits,
    usage: FrameTransactionUsage,
    transacted: Transacted,
    route_commands: VecDeque<crate::Command>,
    route_deferred: VecDeque<crate::DeferredOp>,
    deferred_effects: VecDeque<DeferredEffect>,
    pending_surfaces: VecDeque<ui_contract::SurfaceId>,
    trees: VecDeque<(ui_contract::SurfaceId, crate::ComponentTree)>,
    active_reconcile: Option<ActiveReconcile>,
    reconciled: VecDeque<ReconciledSurface>,
    commits: Vec<ReconciledSurface>,
}

impl FrameTransaction {
    /// 🌱️ Creates an idle persistent transaction. The first step captures the scheduler clock.
    pub fn new(limits: FrameTransactionLimits) -> Self {
        Self {
            stage: FrameTransactionStage::DrainProjectionDeltas,
            now_ms: 0,
            started: false,
            observed_input_epoch: 0,
            force_all_surfaces: false,
            projection_drained: 0,
            effect_cycles: 0,
            effects_settled: false,
            limits,
            usage: FrameTransactionUsage::default(),
            transacted: Transacted::default(),
            route_commands: VecDeque::new(),
            route_deferred: VecDeque::new(),
            deferred_effects: VecDeque::new(),
            pending_surfaces: VecDeque::new(),
            trees: VecDeque::new(),
            active_reconcile: None,
            reconciled: VecDeque::new(),
            commits: Vec::new(),
        }
    }

    pub fn stage(&self) -> FrameTransactionStage {
        self.stage
    }

    pub fn usage(&self) -> FrameTransactionUsage {
        self.usage
    }

    /// ⏭️ Advances bounded work until fuel or wall-clock deadline is reached, cancellation is
    /// observed, or a consistent snapshot is published.
    pub fn step<S: crate::CommandSink, D: crate::ProjectionDelta>(&mut self, runtime: &mut UiRuntime<S, D>, cx: &mut semio_framework_job::StepContext<'_>) -> FrameTransactionStep {
        if cx.is_cancelled() {
            return self.cancel();
        }
        if cx.should_yield() {
            return FrameTransactionStep::Yield { stage: self.stage, usage: self.usage };
        }
        let Some(now_us) = cx.now_us() else { return FrameTransactionStep::Yield { stage: self.stage, usage: self.usage } };
        if !self.started {
            self.started = true;
            self.now_ms = now_us / 1_000;
            self.observed_input_epoch = runtime.input_epoch;
        } else if runtime.input_epoch != self.observed_input_epoch {
            self.supersede(runtime, now_us / 1_000);
        }

        loop {
            cx.set_stage(self.stage_label());
            if self.stage == FrameTransactionStage::PublishSnapshot {
                return self.publish(runtime);
            }
            let worked = match self.stage {
                FrameTransactionStage::DrainProjectionDeltas => self.drain_projection_delta(runtime),
                FrameTransactionStage::RouteIntents => self.route_intent(runtime),
                FrameTransactionStage::FlushEffects => self.flush_effect(runtime),
                FrameTransactionStage::PresentSurface => self.present_surface(runtime),
                FrameTransactionStage::ReconcileTree => self.reconcile_tree(runtime),
                FrameTransactionStage::BuildRenderPackets => self.build_render_packet(),
                FrameTransactionStage::PublishSnapshot => unreachable!(),
            };
            if worked {
                cx.consume_fuel(1);
                if cx.is_cancelled() {
                    return self.cancel();
                }
                if !self.usage.fits(self.limits) {
                    self.credit_fault();
                }
                if cx.should_yield() {
                    return FrameTransactionStep::Yield { stage: self.stage, usage: self.usage };
                }
            }
        }
    }

    fn stage_label(&self) -> &'static str {
        match self.stage {
            FrameTransactionStage::DrainProjectionDeltas => "DrainProjectionDeltas",
            FrameTransactionStage::RouteIntents => "RouteIntents",
            FrameTransactionStage::FlushEffects => "FlushEffects",
            FrameTransactionStage::PresentSurface => "PresentSurface",
            FrameTransactionStage::ReconcileTree => "ReconcileTree",
            FrameTransactionStage::BuildRenderPackets => "BuildRenderPackets",
            FrameTransactionStage::PublishSnapshot => "PublishSnapshot",
        }
    }

    fn charge(&mut self, items: usize, nodes: usize, bytes: usize) {
        self.usage.items = self.usage.items.saturating_add(items);
        self.usage.nodes = self.usage.nodes.saturating_add(nodes);
        self.usage.bytes = self.usage.bytes.saturating_add(bytes);
    }

    fn drain_projection_delta<S: crate::CommandSink, D: crate::ProjectionDelta>(&mut self, runtime: &mut UiRuntime<S, D>) -> bool {
        if self.projection_drained >= PROJECTION_DRAIN_LIMIT {
            self.stage = FrameTransactionStage::RouteIntents;
            return false;
        }
        let mut delta = Vec::with_capacity(1);
        runtime.inbox.drain_into(1, &mut delta);
        if let Some(delta) = delta.pop() {
            (runtime.apply_delta)(&mut runtime.store, delta);
            self.projection_drained += 1;
            self.charge(1, 0, size_of::<D>());
            true
        } else {
            self.stage = FrameTransactionStage::RouteIntents;
            false
        }
    }

    fn route_intent<S: crate::CommandSink, D: crate::ProjectionDelta>(&mut self, runtime: &mut UiRuntime<S, D>) -> bool {
        if let Some(command) = self.route_commands.pop_front() {
            if command.credited_clone().is_some_and(|outbound| runtime.gateway.try_submit(outbound).is_ok()) {
                self.transacted.commands.push(command);
            }
            self.charge(1, 0, size_of::<crate::Command>());
            return true;
        }
        if let Some(op) = self.route_deferred.pop_front() {
            self.apply_deferred(runtime, op);
            self.charge(1, 0, size_of::<crate::DeferredOp>());
            return true;
        }
        let Some(intent) = runtime.pending_intents.pop_front() else {
            self.stage = FrameTransactionStage::FlushEffects;
            return false;
        };
        if let Some(slot) = runtime.surfaces.get(&intent.surface) {
            if !crate::is_stale_intent(intent.revision, slot.current_revision(), crate::DEFAULT_REVISION_TOLERANCE) {
                if let crate::DispatchOutcome::HandledWith { commands, deferred } = slot.dispatch(&intent, &mut runtime.store) {
                    self.route_commands.extend(commands);
                    self.route_deferred.extend(deferred);
                }
            }
        }
        self.charge(1, 0, size_of::<ui_contract::UiIntent>().saturating_add(intent.node_key.len()));
        true
    }

    fn apply_deferred<S: crate::CommandSink, D: crate::ProjectionDelta>(&mut self, runtime: &mut UiRuntime<S, D>, op: crate::DeferredOp) {
        match op {
            crate::DeferredOp::SubmitCommand(command) => self.route_commands.push_back(command),
            crate::DeferredOp::PublishPresence(update) => {
                runtime.presence.record_own(update.surface.clone(), update.node_key.clone(), update.own, update.ttl_ms);
                for peer in update.peers {
                    runtime.presence.record_peer(update.surface.clone(), update.node_key.clone(), peer, update.ttl_ms, self.now_ms);
                }
            }
            crate::DeferredOp::Custom(key) => {
                if let Some(handler) = runtime.custom_handlers.get(key.0).copied() {
                    handler(&mut runtime.store);
                }
            }
        }
    }

    fn flush_effect<S: crate::CommandSink, D: crate::ProjectionDelta>(&mut self, runtime: &mut UiRuntime<S, D>) -> bool {
        if let Some(effect) = self.deferred_effects.pop_front() {
            effect(&mut runtime.store);
            self.charge(1, 0, size_of::<usize>());
            return true;
        }
        if self.effects_settled {
            runtime.store.flush_releases();
            self.prepare_surfaces(runtime);
            self.stage = FrameTransactionStage::PresentSurface;
            return false;
        }
        for &id in runtime.store.effects.notify.iter() {
            runtime.tracking.notify_entity(id);
        }
        let did_work = runtime.store.flush_effects();
        self.effect_cycles = self.effect_cycles.saturating_add(1);
        self.charge(1, 0, 0);
        if did_work && self.effect_cycles >= EFFECT_STORM_BUDGET {
            let pending_notify = runtime.store.effects.notify.iter().copied().collect();
            let pending_emit_sources = runtime.store.effects.emit.iter().map(|(id, _)| *id).collect();
            self.transacted.faults.push(TransactFault::EffectStorm { cycles: self.effect_cycles, pending_notify, pending_emit_sources });
            self.effects_settled = true;
            self.deferred_effects.extend(runtime.store.drain_deferred());
        } else if !did_work {
            self.effects_settled = true;
            self.deferred_effects.extend(runtime.store.drain_deferred());
        }
        true
    }

    fn prepare_surfaces<S: crate::CommandSink, D: crate::ProjectionDelta>(&mut self, runtime: &mut UiRuntime<S, D>) {
        let mut surfaces: HashSet<ui_contract::SurfaceId> = runtime.tracking.drain_dirty().collect();
        surfaces.extend(runtime.pending_first_present.drain());
        if self.force_all_surfaces {
            surfaces.extend(runtime.surfaces.keys().cloned());
            self.force_all_surfaces = false;
        }
        let mut surfaces: Vec<_> = surfaces.into_iter().collect();
        surfaces.sort();
        self.pending_surfaces.extend(surfaces);
    }

    fn present_surface<S: crate::CommandSink, D: crate::ProjectionDelta>(&mut self, runtime: &mut UiRuntime<S, D>) -> bool {
        let Some(surface) = self.pending_surfaces.pop_front() else {
            self.stage = FrameTransactionStage::ReconcileTree;
            return false;
        };
        if let Some(slot) = runtime.surfaces.get(&surface) {
            runtime.tracking.begin(surface.clone());
            let mut cx = crate::PresentCx::new(&mut runtime.tracking, &runtime.store);
            let tree = slot.present(&mut cx);
            runtime.tracking.finish(surface.clone());
            self.charge(1, 0, surface.0.len());
            self.trees.push_back((surface, tree));
        }
        true
    }

    fn reconcile_tree<S: crate::CommandSink, D: crate::ProjectionDelta>(&mut self, runtime: &mut UiRuntime<S, D>) -> bool {
        if self.active_reconcile.is_none() {
            let Some((surface, tree)) = self.trees.pop_front() else {
                self.stage = FrameTransactionStage::BuildRenderPackets;
                return false;
            };
            if let Some(slot) = runtime.surfaces.get(&surface) {
                let generation = self.observed_input_epoch.checked_add(1).expect("test transaction generation exhausted");
                let reservation = crate::SurfaceReconcileReservation::try_new(generation).expect("test transaction admits paired output before reconciliation");
                let current = slot.reconciler.transaction_reader();
                let job = match crate::SurfaceReconcileJob::try_new_reserved(current, tree, reservation) {
                    Ok(job) => job,
                    Err(mut rejected) => { while !rejected.close_step() {} self.credit_fault(); return true; }
                };
                self.active_reconcile = Some(ActiveReconcile { surface, generation, job: Some(job), current: None, ready: None });
                self.charge(1, 0, size_of::<ActiveReconcile>());
                return true;
            } else {
                return true;
            }
        }
        let active = self.active_reconcile.as_mut().expect("active reconcile was initialized");
        if active.current.is_some() {
            let mut active = self.active_reconcile.take().unwrap();
            self.reconciled.push_back(ReconciledSurface { surface: active.surface.clone(), reconciler: active.current.take(), patch: active.ready.take() });
            self.charge(1, 0, size_of::<ReconciledSurface>());
            return true;
        }
        let job = active.job.as_mut().unwrap();
        if job.is_ready() {
            assert!(job.take_ready_into(&mut active.current, &mut active.ready, crate::SurfaceReconcileJob::required_ready_transfer_bytes()).unwrap());
            self.charge(1, 0, crate::SurfaceReconcileJob::required_ready_transfer_bytes());
            return true;
        }
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(active.generation), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence);
        let before = job.transaction_usage();
        let outcome = job.drive_one(&mut context);
        let after = job.transaction_usage();
        self.charge(1, after.nodes.checked_sub(before.nodes).expect("test job node census is monotonic"), after.bytes.checked_sub(before.bytes).expect("test job byte census is monotonic"));
        match outcome {
            crate::SurfaceReconcileJobStep::MoreWork | crate::SurfaceReconcileJobStep::Ready => {},
            crate::SurfaceReconcileJobStep::Fault => self.credit_fault(),
        }
        true
    }

    fn build_render_packet(&mut self) -> bool {
        let Some(mut candidate) = self.reconciled.pop_front() else {
            self.stage = FrameTransactionStage::PublishSnapshot;
            return false;
        };
        if let Some(patch) = candidate.patch.as_mut() {
            self.transacted.patches.push(TransactionPatch::from_ready(patch));
        }
        candidate.patch = None;
        self.commits.push(candidate);
        self.charge(1, 0, 0);
        true
    }

    fn supersede<S: crate::CommandSink, D: crate::ProjectionDelta>(&mut self, runtime: &UiRuntime<S, D>, now_ms: u64) {
        self.pending_surfaces.clear();
        self.trees.clear();
        self.active_reconcile = None;
        self.reconciled.clear();
        self.commits.clear();
        self.transacted.patches.clear();
        self.force_all_surfaces = true;
        self.projection_drained = 0;
        self.effect_cycles = 0;
        self.effects_settled = false;
        self.stage = FrameTransactionStage::DrainProjectionDeltas;
        self.now_ms = now_ms;
        self.observed_input_epoch = runtime.input_epoch;
    }

    fn credit_fault(&mut self) {
        if !self.transacted.faults.iter().any(|fault| matches!(fault, TransactFault::CreditsExceeded { .. })) {
            self.transacted.faults.push(TransactFault::CreditsExceeded { usage: self.usage, limits: self.limits });
        }
        self.pending_surfaces.clear();
        self.trees.clear();
        self.active_reconcile = None;
        self.reconciled.clear();
        self.commits.clear();
        self.transacted.patches.clear();
        self.stage = FrameTransactionStage::PublishSnapshot;
    }

    fn publish<S: crate::CommandSink, D: crate::ProjectionDelta>(&mut self, runtime: &mut UiRuntime<S, D>) -> FrameTransactionStep {
        for mut candidate in self.commits.drain(..) {
            if let Some(slot) = runtime.surfaces.get_mut(&candidate.surface) {
                slot.reconciler.close_transaction_oracle();
                slot.reconciler = candidate.reconciler.take().unwrap();
            }
        }
        runtime.presence.expire(self.now_ms);
        self.transacted.presence = runtime.presence.flush();
        runtime.pending_wakes.retain(|&at| at > self.now_ms);
        self.transacted.next_wake_ms = runtime.pending_wakes.iter().copied().min();
        let output = take(&mut self.transacted);
        self.reset(runtime.input_epoch);
        FrameTransactionStep::Published(output)
    }

    fn cancel(&mut self) -> FrameTransactionStep {
        self.transacted.patches.clear();
        let output = take(&mut self.transacted);
        self.reset(self.observed_input_epoch);
        self.force_all_surfaces = true;
        FrameTransactionStep::Cancelled(output)
    }

    fn reset(&mut self, input_epoch: u64) {
        self.stage = FrameTransactionStage::DrainProjectionDeltas;
        self.started = false;
        self.observed_input_epoch = input_epoch;
        self.force_all_surfaces = false;
        self.projection_drained = 0;
        self.effect_cycles = 0;
        self.effects_settled = false;
        self.usage = FrameTransactionUsage::default();
        self.route_commands.clear();
        self.route_deferred.clear();
        self.deferred_effects.clear();
        self.pending_surfaces.clear();
        self.trees.clear();
        self.active_reconcile = None;
        self.reconciled.clear();
        self.commits.clear();
    }
}

#[cfg(test)]
impl<S: crate::CommandSink, D: crate::ProjectionDelta> UiRuntime<S, D> {
    fn transact(&mut self, now_ms: u64) -> Transacted {
        fn clock() -> Option<u64> {
            Some(0)
        }
        let mut transaction = FrameTransaction::new(FrameTransactionLimits::default());
        transaction.now_ms = now_ms;
        transaction.started = true;
        transaction.observed_input_epoch = self.input_epoch;
        let operation = semio_framework_job::allocate_operation_id();
        let mut preview_sequence = 0;
        loop {
            let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(u64::MAX, u64::MAX), semio_framework_job::CancelToken::root_now(), clock, &mut preview_sequence);
            match transaction.step(self, &mut cx) {
                FrameTransactionStep::Yield { .. } => continue,
                FrameTransactionStep::Published(output) => return output,
                FrameTransactionStep::Cancelled(_) => unreachable!("live test token"),
            }
        }
    }
}

//#endregion 🔖️Transact

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    //#region 🔖️Fixtures

    fn surface(value: &str) -> ui_contract::SurfaceId {
        ui_contract::SurfaceId::try_from(value).expect("bounded fixture surface")
    }

    fn ui_text(value: &str) -> ui_contract::UiText {
        ui_contract::UiText::try_from_str(value).expect("bounded fixture text")
    }

    struct Model {
        value: i32,
    }

    struct FakePresenter {
        model: crate::Entity<Model>,
        count: i32,
    }

    impl crate::Present for FakePresenter {
        fn present(&self, cx: &mut crate::PresentCx<'_>) -> crate::ComponentTree {
            let model = cx.read(&self.model);
            let label = format!("{}:{}", self.count, model.value);
            crate::ComponentTree::new(crate::TreeNode::try_new("root", ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::try_from(label).expect("bounded fixture label"), emphasize: None, data_attributes: None })).expect("bounded fixture node"))
        }
    }

    impl crate::HandleIntent for FakePresenter {
        fn on_intent(&mut self, intent: &ui_contract::UiIntent, cx: &mut crate::Context<'_, Self>) -> crate::DispatchOutcome {
            match intent.trigger {
                ui_contract::Trigger::Delta => {
                    self.count += 1;
                    cx.notify();
                    crate::DispatchOutcome::HandledWith { commands: vec![test_command(intent.seq)], deferred: vec![] }
                }
                ui_contract::Trigger::Commit => {
                    cx.notify();
                    crate::DispatchOutcome::HandledWith {
                        commands: vec![],
                        deferred: vec![crate::DeferredOp::PublishPresence(ui_contract::PresenceUpdate {
                            surface: intent.surface.clone(),
                            node_key: intent.node_key.to_string(),
                            own: ui_contract::OwnPresence { hovered: true, ..Default::default() },
                            peers: vec![],
                            ttl_ms: 4_000,
                        })],
                    }
                }
                _ => crate::DispatchOutcome::Unhandled,
            }
        }
    }

    struct FakeDelta {
        target: crate::Entity<Model>,
        value: i32,
    }

    impl crate::ProjectionDelta for FakeDelta {
        type Key = crate::EntityId;
        fn key(&self) -> crate::EntityId {
            self.target.id()
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn apply_fake_delta(store: &mut crate::EntityStore, delta: FakeDelta) {
        let FakeDelta { target, value } = delta;
        store.update(&target, |model, cx| {
            model.value = value;
            cx.notify();
        });
    }

    struct AlwaysAcceptsSink;
    impl crate::CommandSink for AlwaysAcceptsSink {
        fn try_send(&self, _command: crate::Command) -> Result<(), crate::SinkFull> {
            Ok(())
        }
    }

    struct FailsAfterSink {
        calls: Cell<u32>,
        accepts: u32,
    }
    impl crate::CommandSink for FailsAfterSink {
        fn try_send(&self, _command: crate::Command) -> Result<(), crate::SinkFull> {
            let seen = self.calls.get();
            self.calls.set(seen + 1);
            if seen < self.accepts {
                Ok(())
            } else {
                Err(crate::SinkFull)
            }
        }
    }

    fn test_command(seq: u64) -> crate::Command {
        crate::Command { id: crate::CommandId(seq), correlation: crate::CorrelationId(seq), payload: ui_contract::UiValue::Null }
    }

    fn test_intent(surface: ui_contract::SurfaceId, node: &crate::Entity<FakePresenter>, revision: ui_contract::UiRevision, trigger: ui_contract::Trigger, seq: u64) -> ui_contract::UiIntent {
        ui_contract::UiIntent { surface, revision, node: ui_contract::UiNodeId(node.id().0), node_key: ui_text("root"), trigger, action: ui_contract::ActionId::try_v1("test", "act").expect("bounded fixture action"), args: None, input: None, seq }
    }

    fn test_runtime() -> UiRuntime<AlwaysAcceptsSink, FakeDelta> {
        UiRuntime::new(crate::CommandGateway::new(10, AlwaysAcceptsSink), 16, apply_fake_delta)
    }

    fn register_test_surface<S: crate::CommandSink>(runtime: &mut UiRuntime<S, FakeDelta>, surface: ui_contract::SurfaceId) -> (crate::Entity<FakePresenter>, crate::Entity<Model>) {
        let model = runtime.store_mut().insert(Model { value: 0 });
        let presenter = runtime.store_mut().insert(FakePresenter { model: model.clone(), count: 0 });
        runtime.register_surface(surface.clone(), presenter.clone(), crate::SurfaceReconciler::new(surface));
        (presenter, model)
    }

    fn step_once<S: crate::CommandSink, D: crate::ProjectionDelta>(transaction: &mut FrameTransaction, runtime: &mut UiRuntime<S, D>, fuel: u64) -> FrameTransactionStep {
        fn clock() -> Option<u64> {
            Some(0)
        }
        let mut preview_sequence = 0;
        let mut cx = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(),
            semio_framework_job::Generation(0),
            semio_framework_job::StepBudget::new(fuel, u64::MAX),
            semio_framework_job::CancelToken::root_now(),
            clock,
            &mut preview_sequence,
        );
        transaction.step(runtime, &mut cx)
    }

    fn drive_stepped<S: crate::CommandSink, D: crate::ProjectionDelta>(transaction: &mut FrameTransaction, runtime: &mut UiRuntime<S, D>, fuel: u64) -> (Transacted, usize) {
        let mut yields = 0;
        loop {
            match step_once(transaction, runtime, fuel) {
                FrameTransactionStep::Yield { .. } => yields += 1,
                FrameTransactionStep::Published(output) => return (output, yields),
                FrameTransactionStep::Cancelled(_) => unreachable!("live test token"),
            }
        }
    }

    //#endregion 🔖️Fixtures

    //#region 🔖️IntentMutatesAndPatches
    #[test]
    fn an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch() {
        let mut runtime = test_runtime();
        let surface = surface("s");
        let (presenter, _model) = register_test_surface(&mut runtime, surface.clone());
        runtime.transact(0); // 🌱️ unconditional first present, baseline revision 1

        runtime.submit_intent(test_intent(surface.clone(), &presenter, ui_contract::UiRevision(1), ui_contract::Trigger::Delta, 1));
        let transacted = runtime.transact(0);

        assert_eq!(transacted.patches.len(), 1);
        assert_eq!(transacted.patches[0].surface, surface);
        assert!(!transacted.patches[0].ops.is_empty());
        assert_eq!(transacted.commands, vec![test_command(1)]);
    }
    //#endregion 🔖️IntentMutatesAndPatches

    //#region 🔖️StaleIntentDropped
    #[test]
    fn a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command() {
        let mut runtime = test_runtime();
        let surface = surface("s");
        let (presenter, model) = register_test_surface(&mut runtime, surface.clone());
        runtime.transact(0); // revision 1

        // 🐌️ advance the surface's revision independently of any intent, so the gap exceeds tolerance
        runtime.store_mut().update(&model, |m, cx| {
            m.value = 1;
            cx.notify();
        });
        runtime.transact(0); // revision 2
        runtime.store_mut().update(&model, |m, cx| {
            m.value = 2;
            cx.notify();
        });
        let bumped = runtime.transact(0); // revision 3
        assert_eq!(bumped.patches.len(), 1);

        runtime.submit_intent(test_intent(surface, &presenter, ui_contract::UiRevision(0), ui_contract::Trigger::Delta, 99));
        let transacted = runtime.transact(0);

        assert!(transacted.patches.is_empty(), "a stale intent must never reach the reconciler as a patch");
        assert!(transacted.commands.is_empty(), "a stale intent's handler must never run, so its command must never appear");
    }
    //#endregion 🔖️StaleIntentDropped

    //#region 🔖️BulkCoalescing
    #[test]
    fn a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch() {
        let mut runtime = test_runtime();
        let surface = surface("s");
        let (_presenter, model) = register_test_surface(&mut runtime, surface);
        runtime.transact(0); // baseline

        for value in 1..=5 {
            runtime.push_delta(FakeDelta { target: model.clone(), value }).expect("fits: same key coalesces");
        }
        let transacted = runtime.transact(0);

        assert_eq!(transacted.patches.len(), 1, "a burst of same-key deltas must still yield exactly one patch");
    }
    //#endregion 🔖️BulkCoalescing

    //#region 🔖️UnreadEntityProducesNoPatch
    #[test]
    fn an_entity_notified_but_not_read_by_any_surface_produces_no_patch() {
        let mut runtime = test_runtime();
        let surface = surface("s");
        register_test_surface(&mut runtime, surface);
        runtime.transact(0); // baseline: establishes the surface's real read set

        let untracked = runtime.store_mut().insert(0i32);
        runtime.store_mut().update(&untracked, |value, cx| {
            *value += 1;
            cx.notify();
        });
        let transacted = runtime.transact(0);

        assert!(transacted.patches.is_empty(), "an entity no surface reads must never dirty a present");
    }
    //#endregion 🔖️UnreadEntityProducesNoPatch

    //#region 🔖️EffectStorm
    #[test]
    fn the_effect_fixpoint_terminates_and_a_pathological_observer_hits_the_storm_budget() {
        let mut runtime = test_runtime();
        let entity = runtime.store_mut().insert(0i32);
        let looping = entity.clone();
        let _subscription = runtime.store_mut().update(&entity, |_, cx| cx.observe(&looping, |_, cx2| cx2.notify()));
        runtime.store_mut().update(&entity, |_, cx| cx.notify());

        let transacted = runtime.transact(0);

        assert_eq!(transacted.faults.len(), 1);
        match &transacted.faults[0] {
            TransactFault::EffectStorm { cycles, pending_notify, .. } => {
                assert_eq!(*cycles, EFFECT_STORM_BUDGET);
                assert!(pending_notify.contains(&entity.id()), "the fault must name the still-looping entity");
            }
            TransactFault::CreditsExceeded { .. } => panic!("default credits must not be exhausted"),
        }
    }
    //#endregion 🔖️EffectStorm

    //#region 🔖️GatewayBackpressure
    #[test]
    fn a_full_command_mailbox_surfaces_backpressure_without_blocking_the_transaction() {
        let mut runtime: UiRuntime<FailsAfterSink, FakeDelta> = UiRuntime::new(crate::CommandGateway::new(10, FailsAfterSink { calls: Cell::new(0), accepts: 1 }), 16, apply_fake_delta);
        let surface = surface("s");
        let (presenter, _model) = register_test_surface(&mut runtime, surface.clone());
        runtime.transact(0);

        runtime.submit_intent(test_intent(surface.clone(), &presenter, ui_contract::UiRevision(1), ui_contract::Trigger::Delta, 1));
        runtime.submit_intent(test_intent(surface, &presenter, ui_contract::UiRevision(2), ui_contract::Trigger::Delta, 2));
        let transacted = runtime.transact(0);

        assert_eq!(transacted.commands.len(), 1, "only the sink-accepted command should be reported, without the transaction blocking or panicking");
    }
    //#endregion 🔖️GatewayBackpressure

    //#region 🔖️NextWake
    #[test]
    fn next_wake_ms_is_none_when_idle_and_some_earliest_when_a_deadline_is_pending() {
        let mut runtime = test_runtime();
        assert_eq!(runtime.transact(0).next_wake_ms, None);

        runtime.request_wake(500);
        runtime.request_wake(200);
        assert_eq!(runtime.transact(100).next_wake_ms, Some(200));
        assert_eq!(runtime.transact(250).next_wake_ms, Some(500));
        assert_eq!(runtime.transact(9_999).next_wake_ms, None);
    }
    //#endregion 🔖️NextWake

    //#region 🔖️PresenceOwnChannel
    #[test]
    fn presence_flushes_on_its_own_channel_and_never_appears_in_a_patch() {
        let mut runtime = test_runtime();
        let surface = surface("s");
        let (presenter, _model) = register_test_surface(&mut runtime, surface.clone());
        runtime.transact(0);

        runtime.submit_intent(test_intent(surface, &presenter, ui_contract::UiRevision(1), ui_contract::Trigger::Commit, 7));
        let transacted = runtime.transact(0);

        assert_eq!(transacted.presence.len(), 1);
        assert!(transacted.presence[0].own.hovered);
        assert!(transacted.patches.is_empty(), "a Commit trigger that only publishes presence must not itself produce a document patch");
    }
    //#endregion 🔖️PresenceOwnChannel

    //#region 🔖️IndependentSurfaces
    #[test]
    fn two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other() {
        let mut runtime = test_runtime();
        let surface_a = surface("a");
        let surface_b = surface("b");
        let (_presenter_a, model_a) = register_test_surface(&mut runtime, surface_a.clone());
        let (_presenter_b, _model_b) = register_test_surface(&mut runtime, surface_b);
        let baseline = runtime.transact(0);
        assert_eq!(baseline.patches.len(), 2, "both freshly registered surfaces present once");

        runtime.store_mut().update(&model_a, |m, cx| {
            m.value = 42;
            cx.notify();
        });
        let transacted = runtime.transact(0);

        assert_eq!(transacted.patches.len(), 1);
        assert_eq!(transacted.patches[0].surface, surface_a);
    }
    //#endregion 🔖️IndependentSurfaces

    //#region 🔖️ResumableStress
    #[test]
    fn one_fuel_slices_bound_an_intent_storm_and_preserve_fifo_output() {
        let mut runtime: UiRuntime<AlwaysAcceptsSink, FakeDelta> = UiRuntime::new(crate::CommandGateway::new(128, AlwaysAcceptsSink), 16, apply_fake_delta);
        let surface = surface("s");
        let (presenter, _) = register_test_surface(&mut runtime, surface.clone());
        runtime.transact(0);
        for seq in 0..32 {
            runtime.submit_intent(test_intent(surface.clone(), &presenter, ui_contract::UiRevision(1), ui_contract::Trigger::Delta, seq));
        }

        let mut transaction = FrameTransaction::new(FrameTransactionLimits::default());
        let (output, yields) = drive_stepped(&mut transaction, &mut runtime, 1);

        assert!(yields >= 64, "routing and command submission must each yield as independent units");
        assert_eq!(output.commands, (0..32).map(test_command).collect::<Vec<_>>());
    }

    #[test]
    fn an_effect_storm_remains_resumable_and_retains_the_cycle_fault_semantics() {
        let mut runtime = test_runtime();
        let entity = runtime.store_mut().insert(0i32);
        let looping = entity.clone();
        let _subscription = runtime.store_mut().update(&entity, |_, cx| cx.observe(&looping, |_, cx2| cx2.notify()));
        runtime.store_mut().update(&entity, |_, cx| cx.notify());

        let mut transaction = FrameTransaction::new(FrameTransactionLimits::default());
        let (output, yields) = drive_stepped(&mut transaction, &mut runtime, 1);

        assert!(yields >= EFFECT_STORM_BUDGET as usize);
        assert!(matches!(output.faults.as_slice(), [TransactFault::EffectStorm { cycles: EFFECT_STORM_BUDGET, .. }]));
    }

    #[test]
    fn repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command() {
        let mut runtime = test_runtime();
        let surface = surface("s");
        let (presenter, model) = register_test_surface(&mut runtime, surface.clone());
        runtime.transact(0);
        runtime.submit_intent(test_intent(surface.clone(), &presenter, ui_contract::UiRevision(1), ui_contract::Trigger::Delta, 7));

        let mut transaction = FrameTransaction::new(FrameTransactionLimits::default());
        while transaction.stage() < FrameTransactionStage::ReconcileTree {
            assert!(matches!(step_once(&mut transaction, &mut runtime, 1), FrameTransactionStep::Yield { .. }));
        }
        for value in 1..=8 {
            runtime.push_delta(FakeDelta { target: model.clone(), value }).expect("coalesced resize-style input");
            assert!(matches!(step_once(&mut transaction, &mut runtime, 1), FrameTransactionStep::Yield { .. }));
        }
        runtime.push_delta(FakeDelta { target: model, value: 99 }).expect("latest input");
        let (output, _) = drive_stepped(&mut transaction, &mut runtime, 1);

        assert_eq!(output.commands, vec![test_command(7)]);
        assert_eq!(output.patches.len(), 1);
        let snapshot = runtime.surfaces.get(&surface).expect("surface").reconciler.snapshot();
        assert!(serde_json::to_string(&snapshot).expect("snapshot json").contains("1:99"), "only the newest projection may be published");
    }

    #[test]
    fn cancellation_discards_an_active_node_cursor_without_advancing_the_surface_revision() {
        fn clock() -> Option<u64> {
            Some(0)
        }
        let mut runtime = test_runtime();
        let surface = surface("s");
        register_test_surface(&mut runtime, surface.clone());
        let mut transaction = FrameTransaction::new(FrameTransactionLimits::default());
        while transaction.active_reconcile.is_none() {
            assert!(matches!(step_once(&mut transaction, &mut runtime, 1), FrameTransactionStep::Yield { .. }));
        }
        assert_eq!(runtime.surfaces.get(&surface).expect("surface").current_revision(), ui_contract::UiRevision(0));

        let token = semio_framework_job::CancelToken::root_now();
        token.cancel_now();
        let mut preview_sequence = 0;
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), token, clock, &mut preview_sequence);
        assert!(matches!(transaction.step(&mut runtime, &mut cx), FrameTransactionStep::Cancelled(_)));
        assert_eq!(runtime.surfaces.get(&surface).expect("surface").current_revision(), ui_contract::UiRevision(0));

        let (output, _) = drive_stepped(&mut transaction, &mut runtime, 1);
        assert_eq!(output.patches.len(), 1, "a later live slice must re-present the discarded candidate");
        assert_eq!(output.patches[0].revision, ui_contract::UiRevision(1));
    }

    #[test]
    fn deterministic_surface_order_is_independent_of_hash_map_insertion_order() {
        fn build(order: [&str; 2]) -> UiRuntime<AlwaysAcceptsSink, FakeDelta> {
            let mut runtime = test_runtime();
            for surface in order {
                register_test_surface(&mut runtime, ui_contract::SurfaceId::try_from(surface).expect("bounded fixture surface"));
            }
            runtime
        }

        let mut left = build(["b", "a"]);
        let mut right = build(["a", "b"]);
        let (left_output, _) = drive_stepped(&mut FrameTransaction::new(FrameTransactionLimits::default()), &mut left, 1);
        let (right_output, _) = drive_stepped(&mut FrameTransaction::new(FrameTransactionLimits::default()), &mut right, 1);

        assert_eq!(serde_json::to_string(&left_output.patches).unwrap(), serde_json::to_string(&right_output.patches).unwrap());
        assert_eq!(left_output.patches.iter().map(|patch| patch.surface.0.as_str()).collect::<Vec<_>>(), vec!["a", "b"]);
    }

    #[test]
    fn an_expired_wall_clock_budget_returns_before_consuming_input() {
        fn clock() -> Option<u64> {
            Some(10)
        }
        let mut runtime = test_runtime();
        let target = runtime.store_mut().insert(Model { value: 0 });
        runtime.push_delta(FakeDelta { target, value: 1 }).expect("delta");
        let mut transaction = FrameTransaction::new(FrameTransactionLimits::default());
        let mut preview_sequence = 0;
        let mut cx = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(),
            semio_framework_job::Generation(0),
            semio_framework_job::StepBudget::new(1, 10),
            semio_framework_job::CancelToken::root_now(),
            clock,
            &mut preview_sequence,
        );

        assert!(matches!(transaction.step(&mut runtime, &mut cx), FrameTransactionStep::Yield { usage: FrameTransactionUsage { items: 0, .. }, .. }));
        assert_eq!(runtime.inbox.len(), 1);
    }

    #[test]
    fn transaction_canonical_job_preserves_independent_node_credit() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔄️transaction/🧪️fixture.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let mut runtime = test_runtime();
            register_test_surface(&mut runtime, surface("node-credit"));
            let mut transaction = FrameTransaction::new(FrameTransactionLimits {
                max_items: fixture["maximumItems"].as_u64().unwrap() as usize,
                max_nodes: row["maximumNodes"].as_u64().unwrap() as usize,
                max_bytes: fixture["maximumBytes"].as_u64().unwrap() as usize,
            });
            let (output, _) = drive_stepped(&mut transaction, &mut runtime, 1);
            assert_eq!(output.patches.len(), row["patches"].as_u64().unwrap() as usize);
            assert_eq!(output.faults.iter().any(|fault| matches!(fault, TransactFault::CreditsExceeded { .. })), row["creditFault"].as_bool().unwrap());
        }
    }

    #[test]
    fn hard_credits_fault_before_any_candidate_snapshot_is_published() {
        let mut runtime = test_runtime();
        register_test_surface(&mut runtime, surface("s"));
        let mut transaction = FrameTransaction::new(FrameTransactionLimits { max_items: 0, max_nodes: 0, max_bytes: 0 });
        let (output, _) = drive_stepped(&mut transaction, &mut runtime, 1);

        assert!(output.patches.is_empty());
        assert!(matches!(output.faults.as_slice(), [TransactFault::CreditsExceeded { .. }]));
        assert_eq!(runtime.surfaces.get(&surface("s")).unwrap().current_revision(), ui_contract::UiRevision(0));
    }
    //#endregion 🔖️ResumableStress
}
//#endregion 🧪️Tests

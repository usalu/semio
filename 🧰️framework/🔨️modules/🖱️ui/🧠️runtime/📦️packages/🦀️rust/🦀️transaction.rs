//! @emoji 🔄️ The frame transaction: drain, dispatch, flush, present, reconcile, collect.
//!
//! [`UiRuntime::transact`] is the run-to-completion step that ties every landed sibling into one
//! atomic frame (ruling U1 — no `async fn`, no suspension point, no mutable entity reference survives
//! across a step): `crate::EntityStore` for state and mutation leasing, `crate::DependencyTracker` for
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
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1. No `block_on` either — none is needed here.

use std::any::Any;
use std::collections::{HashMap, HashSet, VecDeque};

//#region 🔖️Runtime

//#region 🧬️Erasure

type ErasedDispatchFn = fn(&dyn Any, &ui_contract::UiIntent, &mut crate::EntityStore) -> crate::DispatchOutcome;
type ErasedPresentFn = fn(&dyn Any, &mut crate::PresentCx<'_>) -> crate::ComponentTree;

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
    /// through `crate::SurfaceReconciler::snapshot`, the only accessor its landed API exposes for
    /// this — see this packet's report for the cheaper accessor this leaves as a registrar-request.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn current_revision(&self) -> ui_contract::UiRevision {
        self.reconciler.snapshot().revision
    }
}

//#endregion 🧬️Erasure

/// 💥️ A fault [`UiRuntime::transact`] surfaces in its result instead of hanging or silently dropping
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
}

/// 🌀️ How many `crate::EntityStore::flush_effects` cycles [`UiRuntime::transact`] loops before
/// treating an unsettled effect fixpoint as [`TransactFault::EffectStorm`] rather than spinning
/// further (ticket-mandated).
pub const EFFECT_STORM_BUDGET: u32 = 64;

/// 📥️ How many projection deltas one [`UiRuntime::transact`] call drains from the inbox at most —
/// bounds this step's own work so one slow projection cannot itself blow a frame budget.
pub const PROJECTION_DRAIN_LIMIT: usize = 256;

/// 📦️ Everything one [`UiRuntime::transact`] call produced, ready for the embedder to ship: patches
/// for the renderer, commands that cleared the gateway this transaction, presence updates on their own
/// channel (never inside `patches`), any [`TransactFault`] worth logging, and the earliest deadline
/// (if any) that should wake the next transaction.
#[derive(Debug, Default)]
pub struct Transacted {
    pub patches: Vec<ui_contract::UiPatch>,
    pub commands: Vec<crate::Command>,
    pub presence: Vec<ui_contract::PresenceUpdate>,
    pub faults: Vec<TransactFault>,
    pub next_wake_ms: Option<u64>,
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
        }
    }

    /// 🗄️ Direct access to the entity store — for constructing presenter/model entities before
    /// registering a surface. `transact` is the only place this crate itself calls
    /// `crate::EntityStore::update` outside a `crate::HandleIntent` handler.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn store_mut(&mut self) -> &mut crate::EntityStore {
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
        self.surfaces.insert(surface.clone(), SurfaceSlot::new(presenter, reconciler));
        self.pending_first_present.insert(surface);
    }

    /// ➕️ Queues one `ui_contract::UiIntent` for the next `transact` call to route — the only way an
    /// intent enters this runtime. `transact` drains this queue in FIFO order every call.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn submit_intent(&mut self, intent: ui_contract::UiIntent) {
        self.pending_intents.push_back(intent);
    }

    /// 📥️ The bounded, coalescing entry point for a projection delta — a CQRS subscription pushes
    /// here; `transact` drains and applies it.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_delta(&mut self, delta: D) -> Result<(), crate::InboxOverflow> {
        self.inbox.push(delta)
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

impl<S: crate::CommandSink, D: crate::ProjectionDelta> UiRuntime<S, D> {
    /// 🔄️ The run-to-completion frame transaction (ruling U1: no `async fn`, no suspension point, no
    /// mutable entity reference survives across a step; tagged below). Order, exactly:
    /// 1. drain a bounded number of projection deltas and apply them;
    /// 2. route queued intents through `crate::HandleIntent`, honouring the revision guard;
    /// 3. flush effects to a fixpoint under [`EFFECT_STORM_BUDGET`];
    /// 4. present every surface the dependency tracker (or a fresh registration) marks dirty;
    /// 5. reconcile each presented tree into at most one `ui_contract::UiPatch`;
    /// 6. collect gateway output, flush expired presence, compute `next_wake_ms`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn transact(&mut self, now_ms: u64) -> Transacted {
        let mut transacted = Transacted::default();

        self.drain_and_apply_deltas();
        self.route_intents(now_ms, &mut transacted.commands);
        self.flush_effects_to_fixpoint(&mut transacted.faults);
        let dirty_trees = self.present_dirty_surfaces();
        self.reconcile_trees(dirty_trees, &mut transacted.patches);

        self.presence.expire(now_ms);
        transacted.presence = self.presence.flush();
        self.pending_wakes.retain(|&at| at > now_ms);
        transacted.next_wake_ms = self.pending_wakes.iter().copied().min();

        transacted
    }

    //#region 🔢️Step1Inbox
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn drain_and_apply_deltas(&mut self) {
        let mut deltas = Vec::new();
        self.inbox.drain_into(PROJECTION_DRAIN_LIMIT, &mut deltas);
        for delta in deltas {
            (self.apply_delta)(&mut self.store, delta);
        }
    }
    //#endregion 🔢️Step1Inbox

    //#region 🔢️Step2Dispatch
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn route_intents(&mut self, now_ms: u64, commands_out: &mut Vec<crate::Command>) {
        let intents: Vec<ui_contract::UiIntent> = self.pending_intents.drain(..).collect();
        for intent in intents {
            let Some(slot) = self.surfaces.get(&intent.surface) else { continue };
            if crate::is_stale_intent(intent.revision, slot.current_revision(), crate::DEFAULT_REVISION_TOLERANCE) {
                continue;
            }
            let outcome = slot.dispatch(&intent, &mut self.store);
            self.apply_outcome(outcome, now_ms, commands_out);
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn apply_outcome(&mut self, outcome: crate::DispatchOutcome, now_ms: u64, commands_out: &mut Vec<crate::Command>) {
        let crate::DispatchOutcome::HandledWith { commands, deferred } = outcome else { return };
        for command in commands {
            self.try_submit(command, commands_out);
        }
        for op in deferred {
            match op {
                crate::DeferredOp::SubmitCommand(command) => self.try_submit(command, commands_out),
                crate::DeferredOp::PublishPresence(update) => self.publish_presence(update, now_ms),
                crate::DeferredOp::Custom(key) => {
                    if let Some(handler) = self.custom_handlers.get(key.0).copied() {
                        handler(&mut self.store);
                    }
                }
            }
        }
    }

    /// 📤️ Non-blocking submit through the gateway (ruling U1: a full mailbox is backpressure, never a
    /// block). A refused command is simply absent from `commands_out` — this transaction's report of
    /// what actually went out — rather than the transaction stalling or panicking on it.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn try_submit(&mut self, command: crate::Command, commands_out: &mut Vec<crate::Command>) {
        if self.gateway.try_submit(command.clone()).is_ok() {
            commands_out.push(command);
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn publish_presence(&mut self, update: ui_contract::PresenceUpdate, now_ms: u64) {
        self.presence.record_own(update.surface.clone(), update.node_key.clone(), update.own, update.ttl_ms);
        for peer in update.peers {
            self.presence.record_peer(update.surface.clone(), update.node_key.clone(), peer, update.ttl_ms, now_ms);
        }
    }
    //#endregion 🔢️Step2Dispatch

    //#region 🔢️Step3EffectStorm
    /// 🌀️ Loops `crate::EntityStore::flush_effects` — itself one bounded cycle per call — to a
    /// fixpoint, bridging `EntityStore`'s own queued notifications into `crate::DependencyTracker`
    /// before each cycle (see this file's top docstring). A cycle that still did work at
    /// [`EFFECT_STORM_BUDGET`] is reported as [`TransactFault::EffectStorm`], not spun on further.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn flush_effects_to_fixpoint(&mut self, faults: &mut Vec<TransactFault>) {
        let mut cycles = 0u32;
        loop {
            for &id in self.store.effects.notify.iter() {
                self.tracking.notify_entity(id);
            }
            let did_work = self.store.flush_effects();
            cycles += 1;
            if !did_work {
                break;
            }
            if cycles >= EFFECT_STORM_BUDGET {
                let pending_notify: Vec<crate::EntityId> = self.store.effects.notify.iter().copied().collect();
                let pending_emit_sources: Vec<crate::EntityId> = self.store.effects.emit.iter().map(|(id, _)| *id).collect();
                faults.push(TransactFault::EffectStorm { cycles, pending_notify, pending_emit_sources });
                break;
            }
        }
        for effect in self.store.drain_deferred() {
            effect(&mut self.store);
        }
        self.store.flush_releases();
    }
    //#endregion 🔢️Step3EffectStorm

    //#region 🔢️Step4Present
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn present_dirty_surfaces(&mut self) -> Vec<(ui_contract::SurfaceId, crate::ComponentTree)> {
        let mut pending: Vec<ui_contract::SurfaceId> = self.tracking.drain_dirty().collect();
        for surface in self.pending_first_present.drain() {
            if !pending.contains(&surface) {
                pending.push(surface);
            }
        }

        let mut trees = Vec::with_capacity(pending.len());
        for surface in pending {
            let Some(slot) = self.surfaces.get(&surface) else { continue };
            self.tracking.begin(surface.clone());
            let mut cx = crate::PresentCx::new(&mut self.tracking, &self.store);
            let tree = slot.present(&mut cx);
            self.tracking.finish(surface.clone());
            trees.push((surface, tree));
        }
        trees
    }
    //#endregion 🔢️Step4Present

    //#region 🔢️Step5Reconcile
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn reconcile_trees(&mut self, trees: Vec<(ui_contract::SurfaceId, crate::ComponentTree)>, patches_out: &mut Vec<ui_contract::UiPatch>) {
        for (surface, tree) in trees {
            let Some(slot) = self.surfaces.get_mut(&surface) else { continue };
            if let Some(patch) = slot.reconciler.reconcile(&tree) {
                patches_out.push(patch);
            }
        }
    }
    //#endregion 🔢️Step5Reconcile
}

//#endregion 🔖️Transact

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    //#region 🔖️Fixtures

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
            crate::ComponentTree::new(crate::TreeNode::new("root", ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::from(label), emphasize: None, data_attributes: None })))
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
                            node_key: intent.node_key.clone(),
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

    fn test_intent(surface: ui_contract::SurfaceId, node: crate::Entity<FakePresenter>, revision: ui_contract::UiRevision, trigger: ui_contract::Trigger, seq: u64) -> ui_contract::UiIntent {
        ui_contract::UiIntent { surface, revision, node: ui_contract::UiNodeId(node.id().0), node_key: "root".into(), trigger, action: ui_contract::ActionId::v1("test", "act"), args: None, input: None, seq }
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

    //#endregion 🔖️Fixtures

    //#region 🔖️IntentMutatesAndPatches
    #[test]
    fn an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch() {
        let mut runtime = test_runtime();
        let surface = ui_contract::SurfaceId::from("s");
        let (presenter, _model) = register_test_surface(&mut runtime, surface.clone());
        runtime.transact(0); // 🌱️ unconditional first present, baseline revision 1

        runtime.submit_intent(test_intent(surface.clone(), presenter.clone(), ui_contract::UiRevision(1), ui_contract::Trigger::Delta, 1));
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
        let surface = ui_contract::SurfaceId::from("s");
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

        runtime.submit_intent(test_intent(surface, presenter, ui_contract::UiRevision(0), ui_contract::Trigger::Delta, 99));
        let transacted = runtime.transact(0);

        assert!(transacted.patches.is_empty(), "a stale intent must never reach the reconciler as a patch");
        assert!(transacted.commands.is_empty(), "a stale intent's handler must never run, so its command must never appear");
    }
    //#endregion 🔖️StaleIntentDropped

    //#region 🔖️BulkCoalescing
    #[test]
    fn a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch() {
        let mut runtime = test_runtime();
        let surface = ui_contract::SurfaceId::from("s");
        let (_presenter, model) = register_test_surface(&mut runtime, surface.clone());
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
        let surface = ui_contract::SurfaceId::from("s");
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
        }
    }
    //#endregion 🔖️EffectStorm

    //#region 🔖️GatewayBackpressure
    #[test]
    fn a_full_command_mailbox_surfaces_backpressure_without_blocking_the_transaction() {
        let mut runtime: UiRuntime<FailsAfterSink, FakeDelta> = UiRuntime::new(crate::CommandGateway::new(10, FailsAfterSink { calls: Cell::new(0), accepts: 1 }), 16, apply_fake_delta);
        let surface = ui_contract::SurfaceId::from("s");
        let (presenter, _model) = register_test_surface(&mut runtime, surface.clone());
        runtime.transact(0);

        runtime.submit_intent(test_intent(surface.clone(), presenter.clone(), ui_contract::UiRevision(1), ui_contract::Trigger::Delta, 1));
        runtime.submit_intent(test_intent(surface, presenter, ui_contract::UiRevision(2), ui_contract::Trigger::Delta, 2));
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
        let surface = ui_contract::SurfaceId::from("s");
        let (presenter, _model) = register_test_surface(&mut runtime, surface.clone());
        runtime.transact(0);

        runtime.submit_intent(test_intent(surface, presenter, ui_contract::UiRevision(1), ui_contract::Trigger::Commit, 7));
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
        let surface_a = ui_contract::SurfaceId::from("a");
        let surface_b = ui_contract::SurfaceId::from("b");
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
}
//#endregion 🧪️Tests

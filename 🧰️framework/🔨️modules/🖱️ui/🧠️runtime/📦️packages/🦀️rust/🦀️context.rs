//! @emoji 🎛️ `Context<T>`: notify, emit, subscribe, observe, defer, spawn_local — and the effect
//! queues behind them.
//!
//! Every parameter here is a sync `FnMut`/`FnOnce`, never an async block — that is what makes "no
//! mutable entity reference crosses an await" a type-level guarantee instead of a convention (ruling
//! U1). `notify`/`emit`/`defer` only *queue*; running a listener while its subject is leased is
//! exactly the reentrancy the lease exists to prevent, so nothing here runs inline. `spawn_local` is
//! the one async-shaped door: it *takes* a future and hands it to the embedder's executor, it does
//! not itself await, and its `'static` bound makes it impossible for the future to capture a `&mut T`
//! or a `Context` — both borrow a lifetime shorter than `'static` by construction.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync.

use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::rc::{Rc, Weak};

use super::entity::{Entity, EntityStore, Handle, WeakEntity};
use super::tracking::EntityId;

//#region 🔖️Context

//#region 📥️ Effect queues

pub type DeferredEffect = Box<dyn FnOnce(&mut EntityStore)>;
pub(crate) type ListenerCallback = Box<dyn FnMut(&mut EntityStore, Option<&dyn Any>)>;

pub(crate) struct ListenerEntry {
    pub(crate) source: EntityId,
    pub(crate) alive: Weak<Handle>,
    pub(crate) call: ListenerCallback,
}

/// ⏳️ A future queued by `Context::spawn_local`, handed to the embedder's own executor. This crate
/// never polls it — `future` is `pub` precisely so the embedder can take it and drive it however its
/// platform requires (the owned browser microtask executor on wasm, a native local executor on host).
pub struct PendingTask {
    pub id: u64,
    pub future: Pin<Box<dyn Future<Output = ()> + 'static>>,
}

/// 🎟️ An opaque handle to a future queued via `Context::spawn_local`. Detached fire-and-forget: this
/// runtime does not itself track completion or support cancellation — the embedder's executor owns
/// that, exactly as it owns polling.
pub struct Task {
    id: u64,
}

impl Task {
    /// 🪪️ The id assigned to this task's [`PendingTask`] at spawn time.
    pub fn id(&self) -> u64 {
        self.id
    }
}

/// ✂️ An RAII token for a `subscribe`/`observe` registration. Dropping it detaches the listener; a
/// listener whose source entity has died is dropped rather than invoked, independent of this token.
pub struct Subscription {
    id: u64,
    pending_detach: Weak<RefCell<Vec<u64>>>,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Some(queue) = self.pending_detach.upgrade() {
            queue.borrow_mut().push(self.id);
        }
    }
}

/// 📬️ The queues every [`Context`] method writes into. Lives inside `EntityStore` as a field so a
/// lease can hand out a disjoint borrow of it alongside a disjoint borrow of the slots — this is the
/// exact seam `runtime-transact`'s flush loop drains through [`EntityStore::flush_effects`],
/// [`EntityStore::drain_deferred`], and [`EntityStore::drain_tasks`].
pub(crate) struct EffectQueues {
    pub(crate) notify: VecDeque<EntityId>,
    pub(crate) emit: VecDeque<(EntityId, Box<dyn Any>)>,
    pub(crate) defer: VecDeque<DeferredEffect>,
    pub(crate) tasks: VecDeque<PendingTask>,
    pub(crate) listeners: HashMap<u64, ListenerEntry>,
    next_listener_id: u64,
    next_task_id: u64,
    pending_detach: Rc<RefCell<Vec<u64>>>,
}

impl EffectQueues {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn new() -> Self {
        Self { notify: VecDeque::new(), emit: VecDeque::new(), defer: VecDeque::new(), tasks: VecDeque::new(), listeners: HashMap::new(), next_listener_id: 0, next_task_id: 0, pending_detach: Rc::new(RefCell::new(Vec::new())) }
    }

    fn register(&mut self, source: EntityId, alive: Weak<Handle>, call: impl FnMut(&mut EntityStore, Option<&dyn Any>) + 'static) -> Subscription {
        let id = self.next_listener_id;
        self.next_listener_id += 1;
        self.listeners.insert(id, ListenerEntry { source, alive, call: Box::new(call) });
        Subscription { id, pending_detach: Rc::downgrade(&self.pending_detach) }
    }

    fn spawn(&mut self, future: impl Future<Output = ()> + 'static) -> Task {
        let id = self.next_task_id;
        self.next_task_id += 1;
        self.tasks.push_back(PendingTask { id, future: Box::pin(future) });
        Task { id }
    }

    pub(crate) fn prune_detached(&mut self) {
        let ids: Vec<u64> = self.pending_detach.borrow_mut().drain(..).collect();
        for id in ids {
            self.listeners.remove(&id);
        }
    }
}

//#endregion 📥️ Effect queues

/// 🧭️ Borrows the runtime's effect queues and names the entity currently under a mutation lease.
/// Handed to a closure by [`EntityStore::update`] — never constructed directly.
pub struct Context<'a, T: 'static> {
    entity: Entity<T>,
    effects: &'a mut EffectQueues,
}

impl<'a, T: 'static> Context<'a, T> {
    pub(crate) fn new(entity: Entity<T>, effects: &'a mut EffectQueues) -> Self {
        Self { entity, effects }
    }

    /// 🎫️ A fresh strong handle to the entity currently under lease.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn entity(&self) -> Entity<T> {
        self.entity.clone()
    }

    /// 🪶️ A non-owning handle to the entity currently under lease.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn weak_entity(&self) -> WeakEntity<T> {
        self.entity.downgrade()
    }

    /// 🔔️ Queues a "this entity changed" notification for every `observe` listener registered on it.
    /// Queued, not run inline — delivery happens at `EntityStore::flush_effects`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn notify(&mut self) {
        self.effects.notify.push_back(self.entity.id);
    }

    /// 📣️ Queues a typed event for every `subscribe` listener registered on this entity. Queued, not
    /// run inline — delivery happens at `EntityStore::flush_effects`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn emit<E: 'static>(&mut self, event: E) {
        self.effects.emit.push_back((self.entity.id, Box::new(event)));
    }

    /// ⏭️ Queues an effect to run against the `EntityStore` at the next safe boundary the embedder
    /// chooses (`runtime-transact`'s flush loop, via `EntityStore::drain_deferred`) — never inline,
    /// never during this or any other entity's lease.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn defer(&mut self, effect: impl FnOnce(&mut EntityStore) + 'static) {
        self.effects.defer.push_back(Box::new(effect));
    }

    /// 🧵️ Hands `future` to the embedder's executor and returns immediately — this method is not
    /// itself async. The `'static` bound is load-bearing: it makes it impossible for `future` to
    /// capture a `&mut T` or this `Context`, both of which borrow a lifetime shorter than `'static`.
    /// The future must re-enter later through a `WeakEntity::upgrade`, never through a captured
    /// reference.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn spawn_local(&mut self, future: impl Future<Output = ()> + 'static) -> Task {
        self.effects.spawn(future)
    }

    /// 🔗️ Invokes `listener` whenever `source` emits an `E` via `Context::emit`, for as long as the
    /// returned [`Subscription`] is alive and `source` itself has not been released.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn subscribe<U: 'static, E: 'static>(&mut self, source: &Entity<U>, mut listener: impl FnMut(&mut T, &E, &mut Context<'_, T>) + 'static) -> Subscription {
        let observer = self.entity.clone();
        let alive = Rc::downgrade(&source.handle);
        self.effects.register(source.id, alive, move |store, event| {
            if let Some(event) = event.and_then(|event| event.downcast_ref::<E>()) {
                store.update(&observer, |value, cx| listener(value, event, cx));
            }
        })
    }

    /// 🔗️ Invokes `listener` whenever `source` notifies via `Context::notify`, for as long as the
    /// returned [`Subscription`] is alive and `source` itself has not been released.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn observe<U: 'static>(&mut self, source: &Entity<U>, mut listener: impl FnMut(&mut T, &mut Context<'_, T>) + 'static) -> Subscription {
        let observer = self.entity.clone();
        let alive = Rc::downgrade(&source.handle);
        self.effects.register(source.id, alive, move |store, _event| {
            store.update(&observer, |value, cx| listener(value, cx));
        })
    }
}

//#endregion 🔖️Context

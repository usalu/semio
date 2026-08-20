//! @emoji 🪪️ Generational entity store, `Entity<T>`/`WeakEntity<T>`, and the mutation lease.
//!
//! `Entity<T>` is a cheap `Clone` handle into [`EntityStore`], deliberately **not** `Send`/`Sync` —
//! this runtime is single-owner and runs inside single-threaded wasm guests and native hosts alike.
//! [`EntityStore::update`] takes the value *out* of its slot for the closure's duration (the lease),
//! so a reentrant `update`/`read` of the same entity is a detected error, never an alias. Dropping the
//! last strong handle only *queues* a release; the slot is actually freed at [`EntityStore::flush_releases`],
//! a safe boundary a presenter can never observe mid-transaction (ruling U1, ticket
//! `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`). `dyn Any` is the sanctioned heterogeneous
//! erasure here per U3 (it is a std trait, not first-party).
//!
//! `crate::tracking::EntityId` is the one identity type — a `(index, generation)` pair packed into its
//! single `u64`, packed and unpacked only here via a private inherent impl, since this file is the only
//! one that needs the slot/generation split. `DependencyTracker` and everyone else only ever sees the
//! opaque `u64`.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync.

use std::any::Any;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use super::context::{Context, EffectQueues, PendingTask};
use super::tracking::EntityId;

//#region 🔖️Entity

impl EntityId {
    fn from_parts(slot: u32, generation: u32) -> Self {
        Self(((slot as u64) << 32) | generation as u64)
    }

    fn slot(self) -> u32 {
        (self.0 >> 32) as u32
    }

    fn generation(self) -> u32 {
        self.0 as u32
    }
}

/// 🫀️ The shared strong-refcount token behind every clone of an [`Entity<T>`]. Its `Drop` is the
/// release trigger: it only *queues* the id, never frees the slot itself — freeing happens at
/// [`EntityStore::flush_releases`], a safe effect boundary owned by the `runtime-transact` packet.
pub(crate) struct Handle {
    pub(crate) id: EntityId,
    pub(crate) release_queue: Weak<RefCell<Vec<EntityId>>>,
}

impl Drop for Handle {
    fn drop(&mut self) {
        if let Some(queue) = self.release_queue.upgrade() {
            queue.borrow_mut().push(self.id);
        }
    }
}

/// 🎫️ A cheap `Clone` strong handle to a `T` living in an [`EntityStore`]. Not `Send`/`Sync` by
/// construction — `Rc<Handle>` already forbids it; `PhantomData<Rc<T>>` documents that this
/// conceptually owns a `T` too.
pub struct Entity<T: 'static> {
    pub(crate) id: EntityId,
    pub(crate) handle: Rc<Handle>,
    _marker: PhantomData<Rc<T>>,
}

impl<T: 'static> Entity<T> {
    /// 🪪️ The stable generational address of this entity, shared with `crate::DependencyTracker`.
    pub fn id(&self) -> EntityId {
        self.id
    }

    /// 👁️ Reads through to `store` — the same lookup as [`EntityStore::read`], as a convenience for a
    /// caller (e.g. `crate::PresentCx`) that already holds a store reference alongside the entity.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn read<'s>(&self, store: &'s EntityStore) -> &'s T {
        store.read(self)
    }

    /// 🪶️ A non-owning handle that upgrades back to a strong [`Entity<T>`] only while at least one
    /// strong handle is still alive — this is what lets a spawned task reference a screen without
    /// keeping it alive past its owner.
    pub fn downgrade(&self) -> WeakEntity<T> {
        WeakEntity { id: self.id, handle: Rc::downgrade(&self.handle), _marker: PhantomData }
    }
}

impl<T: 'static> Clone for Entity<T> {
    fn clone(&self) -> Self {
        Self { id: self.id, handle: self.handle.clone(), _marker: PhantomData }
    }
}

impl<T: 'static> PartialEq for Entity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: 'static> Eq for Entity<T> {}

/// 🪶️ A non-owning reference to an entity. [`WeakEntity::upgrade`] fails once the last strong
/// [`Entity<T>`] handle has dropped, even if the slot has not been physically freed yet.
pub struct WeakEntity<T: 'static> {
    pub(crate) id: EntityId,
    pub(crate) handle: Weak<Handle>,
    _marker: PhantomData<Rc<T>>,
}

impl<T: 'static> WeakEntity<T> {
    /// 🪪️ The generational address this weak handle targets.
    pub fn id(&self) -> EntityId {
        self.id
    }

    /// ⬆️ Reconstructs a strong [`Entity<T>`], or `None` if every strong handle has already dropped.
    pub fn upgrade(&self) -> Option<Entity<T>> {
        self.handle.upgrade().map(|handle| Entity { id: self.id, handle, _marker: PhantomData })
    }
}

impl<T: 'static> Clone for WeakEntity<T> {
    fn clone(&self) -> Self {
        Self { id: self.id, handle: self.handle.clone(), _marker: PhantomData }
    }
}

//#endregion 🔖️Entity

//#region 🔖️Store

enum SlotPayload {
    Vacant,
    Occupied(Box<dyn Any>),
    Leased,
}

struct Slot {
    generation: u32,
    payload: SlotPayload,
}

/// 🗄️ A heterogeneous generational store of typed presentation state — the runtime's single source
/// of truth for every `Entity<T>`. Also carries the effect queues [`Context`] writes into, since a
/// lease must be able to hand out disjoint borrows of "the slots" and "the queues" at once.
pub struct EntityStore {
    slots: Vec<Slot>,
    free: Vec<u32>,
    release_queue: Rc<RefCell<Vec<EntityId>>>,
    pub(crate) effects: EffectQueues,
}

impl EntityStore {
    /// 🌱️ An empty store.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        Self { slots: Vec::new(), free: Vec::new(), release_queue: Rc::new(RefCell::new(Vec::new())), effects: EffectQueues::new() }
    }

    /// ➕️ Allocates a slot and returns the first strong [`Entity<T>`] handle to it.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn insert<T: 'static>(&mut self, value: T) -> Entity<T> {
        let slot = self.free.pop().unwrap_or_else(|| {
            self.slots.push(Slot { generation: 0, payload: SlotPayload::Vacant });
            (self.slots.len() - 1) as u32
        });
        let generation = self.slots[slot as usize].generation;
        self.slots[slot as usize].payload = SlotPayload::Occupied(Box::new(value));
        let id = EntityId::from_parts(slot, generation);
        let handle = Rc::new(Handle { id, release_queue: Rc::downgrade(&self.release_queue) });
        Entity { id, handle, _marker: PhantomData }
    }

    /// 👁️ Reads the entity, or `None` if it is leased, released, or the id is stale.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn try_read<T: 'static>(&self, entity: &Entity<T>) -> Option<&T> {
        let slot = self.slots.get(entity.id.slot() as usize)?;
        if slot.generation != entity.id.generation() {
            return None;
        }
        match &slot.payload {
            SlotPayload::Occupied(value) => value.downcast_ref::<T>(),
            _ => None,
        }
    }

    /// 👁️ [`EntityStore::try_read`], panicking instead of returning `None`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn read<T: 'static>(&self, entity: &Entity<T>) -> &T {
        self.try_read(entity).expect("🚫️ entity read rejected: leased, released, or a stale generation")
    }

    /// 🧹️ Actually frees every queued release, bumping each slot's generation so no stale handle can
    /// ever resolve to the new occupant. This is the *only* place a slot is freed — call it at a safe
    /// effect boundary (end of a transaction), never mid-lease.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn flush_releases(&mut self) {
        let pending: Vec<EntityId> = self.release_queue.borrow_mut().drain(..).collect();
        for id in pending {
            if let Some(slot) = self.slots.get_mut(id.slot() as usize) {
                if slot.generation == id.generation() {
                    slot.payload = SlotPayload::Vacant;
                    slot.generation = slot.generation.wrapping_add(1);
                    self.free.push(id.slot());
                }
            }
        }
    }

    /// 🔔️ Dispatches exactly the `notify`/`emit` effects queued *before* this call to their
    /// listeners — one bounded cycle, not a live drain. A listener that queues a fresh notify lands
    /// in the *next* cycle, which is what lets `runtime-transact` loop this to a fixpoint under its
    /// own 64-cycle EffectStorm budget instead of one call spinning unboundedly on its own. Returns
    /// whether this cycle did anything. Listener invocation re-enters through [`EntityStore::update`],
    /// so a listener that mutates its own observed entity hits the same nested-lease guard as any
    /// other caller.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn flush_effects(&mut self) -> bool {
        self.effects.prune_detached();
        let notifications: Vec<EntityId> = self.effects.notify.drain(..).collect();
        let emissions: Vec<(EntityId, Box<dyn Any>)> = self.effects.emit.drain(..).collect();
        let did_work = !notifications.is_empty() || !emissions.is_empty();
        for source in notifications {
            self.dispatch(source, None);
        }
        for (source, event) in emissions {
            self.dispatch(source, Some(&*event));
        }
        did_work
    }

    /// 📤️ Hands `runtime-transact` every deferred effect queued since the last drain, in order. Each
    /// closure takes `&mut EntityStore` — the caller invokes them at its own safe point, outside any
    /// lease.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn drain_deferred(&mut self) -> Vec<Box<dyn FnOnce(&mut EntityStore)>> {
        self.effects.defer.drain(..).collect()
    }

    /// 📤️ Hands the embedder every future queued via `Context::spawn_local` since the last drain.
    /// This crate never polls a future itself — the embedder's executor owns that.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn drain_tasks(&mut self) -> Vec<PendingTask> {
        self.effects.tasks.drain(..).collect()
    }

    fn dispatch(&mut self, source: EntityId, event: Option<&dyn Any>) {
        let ids: Vec<u64> = self.effects.listeners.iter().filter(|(_, entry)| entry.source == source && entry.alive.upgrade().is_some()).map(|(id, _)| *id).collect();
        for id in ids {
            if let Some(mut entry) = self.effects.listeners.remove(&id) {
                (entry.call)(self, event);
                if entry.alive.upgrade().is_some() {
                    self.effects.listeners.insert(id, entry);
                }
            }
        }
    }
}

impl Default for EntityStore {
    fn default() -> Self {
        Self::new()
    }
}

//#endregion 🔖️Store

//#region 🔖️Lease

impl EntityStore {
    /// 🔒️ Leases the entity out of its slot for the closure's duration — this is what makes a nested
    /// `update`/`read` of the same entity a detected error rather than an alias. The value is
    /// restored to its slot by an RAII guard, so it survives even if the closure panics.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn update<T: 'static, R>(&mut self, entity: &Entity<T>, update: impl FnOnce(&mut T, &mut Context<'_, T>) -> R) -> R {
        let boxed = self.take_for_lease::<T>(entity.id);

        struct PutBack<'s, T: 'static> {
            slots: &'s mut Vec<Slot>,
            slot: u32,
            value: Option<Box<T>>,
        }

        impl<'s, T: 'static> Drop for PutBack<'s, T> {
            fn drop(&mut self) {
                if let Some(value) = self.value.take() {
                    self.slots[self.slot as usize].payload = SlotPayload::Occupied(value);
                }
            }
        }

        let mut guard = PutBack { slots: &mut self.slots, slot: entity.id.slot(), value: Some(boxed) };
        let mut cx = Context::new(entity.clone(), &mut self.effects);
        update(guard.value.as_mut().unwrap(), &mut cx)
    }

    fn take_for_lease<T: 'static>(&mut self, id: EntityId) -> Box<T> {
        let slot = self.slots.get_mut(id.slot() as usize).expect("🚫️ unknown entity slot");
        assert_eq!(slot.generation, id.generation(), "🚫️ stale entity id — its slot was released and reused");
        match &slot.payload {
            SlotPayload::Leased => {
                panic!("🚫️ nested update/read of a leased entity — rejected, not aliased")
            }
            SlotPayload::Vacant => panic!("🚫️ update on a vacant/released entity"),
            SlotPayload::Occupied(_) => {}
        }
        match std::mem::replace(&mut slot.payload, SlotPayload::Leased) {
            SlotPayload::Occupied(value) => match value.downcast::<T>() {
                Ok(value) => value,
                Err(original) => {
                    slot.payload = SlotPayload::Occupied(original);
                    panic!("🚫️ entity type mismatch");
                }
            },
            _ => unreachable!(),
        }
    }

    /// ↩️ Hand-restores a leased value. Production code never needs this — [`Self::update`]'s
    /// `PutBack` drop guard restores on the normal path *and* on unwind. It exists so a test can take
    /// a lease, provoke a failure against it, and then put the value back to assert nothing was lost.
    #[cfg(test)]
    fn restore_after_lease<T: 'static>(&mut self, id: EntityId, value: Box<T>) {
        self.slots[id.slot() as usize].payload = SlotPayload::Occupied(value);
    }
}

//#endregion 🔖️Lease

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn stale_entity_id_never_resolves_to_new_occupant() {
        let mut store = EntityStore::new();
        let e1 = store.insert(1i32);
        let id1 = e1.id();
        drop(e1);
        store.flush_releases();
        let e2 = store.insert(2i32);
        assert_eq!(e2.id().slot(), id1.slot());
        assert_ne!(e2.id().generation(), id1.generation());
        let stale: Entity<i32> = Entity { id: id1, handle: Rc::new(Handle { id: id1, release_queue: Weak::new() }), _marker: PhantomData };
        assert!(store.try_read(&stale).is_none());
        assert_eq!(store.try_read(&e2), Some(&2));
    }

    #[test]
    fn release_is_queued_until_flush_releases() {
        let mut store = EntityStore::new();
        let e1 = store.insert(1i32);
        let id1 = e1.id();
        drop(e1);
        let e2 = store.insert(2i32);
        assert_ne!(e2.id().slot(), id1.slot());
        store.flush_releases();
        let e3 = store.insert(3i32);
        assert_eq!(e3.id().slot(), id1.slot());
    }

    #[test]
    fn weak_entity_upgrade_fails_after_last_strong_drops() {
        let mut store = EntityStore::new();
        let e = store.insert(42i32);
        let weak = e.downgrade();
        assert!(weak.upgrade().is_some());
        drop(e);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn nested_lease_is_rejected_not_aliased() {
        let mut store = EntityStore::new();
        let e = store.insert(1i32);
        let taken = store.take_for_lease::<i32>(e.id);
        let result = catch_unwind(AssertUnwindSafe(|| store.take_for_lease::<i32>(e.id)));
        assert!(result.is_err());
        store.restore_after_lease(e.id, taken);
        assert_eq!(*store.read(&e), 1);
    }

    #[test]
    fn read_during_lease_is_rejected() {
        let mut store = EntityStore::new();
        let e = store.insert(7i32);
        let taken = store.take_for_lease::<i32>(e.id);
        assert!(store.try_read(&e).is_none());
        store.restore_after_lease(e.id, taken);
        assert_eq!(store.try_read(&e), Some(&7));
    }

    #[test]
    fn value_restored_after_panicking_closure() {
        let mut store = EntityStore::new();
        let e = store.insert(10i32);
        let result = catch_unwind(AssertUnwindSafe(|| {
            store.update(&e, |_, _cx| panic!("boom"));
        }));
        assert!(result.is_err());
        assert_eq!(*store.read(&e), 10);
    }

    #[test]
    fn effects_queue_rather_than_run_inline() {
        let mut store = EntityStore::new();
        let source = store.insert(0i32);
        let observer = store.insert(0i32);
        let counter = Rc::new(Cell::new(0));
        let counter_clone = counter.clone();
        let _sub = store.update(&observer, |_, cx| {
            cx.observe(&source, move |t, _cx| {
                *t += 1;
                counter_clone.set(counter_clone.get() + 1);
            })
        });
        store.update(&source, |_, cx| cx.notify());
        assert_eq!(counter.get(), 0);
        store.flush_effects();
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn dropped_subscription_stops_delivering() {
        let mut store = EntityStore::new();
        let source = store.insert(0i32);
        let observer = store.insert(0i32);
        let counter = Rc::new(Cell::new(0));
        let counter_clone = counter.clone();
        let sub = store.update(&observer, |_, cx| {
            cx.observe(&source, move |_, _cx| {
                counter_clone.set(counter_clone.get() + 1);
            })
        });
        store.update(&source, |_, cx| cx.notify());
        store.flush_effects();
        assert_eq!(counter.get(), 1);
        drop(sub);
        store.update(&source, |_, cx| cx.notify());
        store.flush_effects();
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn defer_effects_queue_rather_than_run_inline() {
        let mut store = EntityStore::new();
        let e = store.insert(0i32);
        let flag = Rc::new(Cell::new(false));
        let flag_clone = flag.clone();
        store.update(&e, |_, cx| {
            cx.defer(move |_store| flag_clone.set(true));
        });
        assert!(!flag.get());
        let deferred = store.drain_deferred();
        assert_eq!(deferred.len(), 1);
        assert!(!flag.get());
    }

    #[test]
    fn spawn_local_queues_future_for_the_embedder() {
        let mut store = EntityStore::new();
        let e = store.insert(0i32);
        store.update(&e, |_, cx| {
            cx.spawn_local(async {});
        });
        assert_eq!(store.drain_tasks().len(), 1);
    }
}

//! 📤️ Pre-admitted shared output entries retain the exact paired publication owner.

use super::{SurfaceReconcileJob, SurfaceReconcileReadyPatch, SurfaceReconciler};
use std::mem::size_of;
use std::sync::{atomic::{AtomicBool, Ordering}, Mutex, MutexGuard, TryLockError};
use ui_contract::UiValueRetirementStep;

//#region 🗃️Registry
const SLOTS: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Key { index: usize, epoch: u64 }

struct Entry { epoch: u64, queue: Option<Key>, generation: u64, next: Option<usize>, ready: Option<SurfaceReconcileReadyPatch> }
impl Entry { const EMPTY: Self = Self { epoch: 0, queue: None, generation: 0, next: None, ready: None }; }

struct Queue { epoch: u64, occupied: bool, closing: bool, entries: usize, length: usize, head: Option<usize>, tail: Option<usize> }
impl Queue { const EMPTY: Self = Self { epoch: 0, occupied: false, closing: false, entries: 0, length: 0, head: None, tail: None }; }

struct Registry { entries: [Entry; SLOTS], queues: [Queue; SLOTS], entry_cursor: usize, queue_cursor: usize }
static REGISTRY: Mutex<Registry> = Mutex::new(Registry { entries: [const { Entry::EMPTY }; SLOTS], queues: [const { Queue::EMPTY }; SLOTS], entry_cursor: 0, queue_cursor: 0 });
static ENTRY_RETURNS: [AtomicBool; SLOTS] = [const { AtomicBool::new(false) }; SLOTS];
static QUEUE_RETURNS: [AtomicBool; SLOTS] = [const { AtomicBool::new(false) }; SLOTS];

fn registry() -> Result<Option<MutexGuard<'static, Registry>>, &'static str> {
    match REGISTRY.try_lock() { Ok(guard) => Ok(Some(guard)), Err(TryLockError::WouldBlock) => Ok(None), Err(TryLockError::Poisoned(_)) => Err("surface output registry is poisoned") }
}

impl Registry {
    fn queue(&self, key: Key) -> Result<&Queue, &'static str> {
        self.queues.get(key.index).filter(|queue| queue.occupied && queue.epoch == key.epoch).ok_or("surface output queue authority is stale")
    }
    fn entry(&self, key: Key, queue: Key, generation: u64) -> Result<&Entry, &'static str> {
        self.entries.get(key.index).filter(|entry| entry.epoch == key.epoch && entry.queue == Some(queue) && entry.generation == generation).ok_or("surface output entry authority is stale")
    }
    fn release_entry(&mut self, index: usize) -> Result<(), &'static str> {
        let entry = &self.entries[index];
        let key = entry.queue.ok_or("surface output entry has no queue")?;
        if entry.ready.is_some() || entry.next.is_some() { return Err("surface output entry still owns a payload"); }
        let entries = self.queue(key)?.entries.checked_sub(1).ok_or("surface output entry count underflow")?;
        self.queues[key.index].entries = entries;
        self.entries[index].queue = None;
        self.entries[index].generation = 0;
        Ok(())
    }
    fn close_queue_one(&mut self, key: Key, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        let queue = self.queue(key)?;
        if let Some(index) = queue.head {
            let entry = &mut self.entries[index];
            let ready = entry.ready.as_mut().ok_or("surface output linked entry has no payload")?;
            if !ready.terminal_is_empty() {
                let mut step = ready.close_step_with_grant(1, bytes)?;
                step.complete = false;
                return Ok(step);
            }
            let next = entry.next;
            let length = self.queues[key.index].length.checked_sub(1).ok_or("surface output length underflow")?;
            self.entries[index].ready = None;
            self.entries[index].next = None;
            self.release_entry(index)?;
            let queue = &mut self.queues[key.index];
            queue.head = next; queue.length = length;
            if next.is_none() { queue.tail = None; }
            return Ok(UiValueRetirementStep { progressed: true, released_items: 1, ..Default::default() });
        }
        if queue.entries != 0 { return Ok(Default::default()); }
        self.queues[key.index].occupied = false;
        self.queues[key.index].closing = false;
        Ok(UiValueRetirementStep { complete: true, progressed: true, released_items: 1, released_bytes: 0 })
    }
}
//#endregion 🗃️Registry

//#region 🎟️Reservation
/// 🎟️ One exact queue entry reserved before its producer is invoked.
#[derive(Debug)]
pub struct SurfaceReconcileOutputReservation { key: Option<Key>, queue: Key, generation: u64 }

impl SurfaceReconcileOutputReservation {
    pub fn generation(&self) -> u64 { self.generation }
    pub fn terminal_is_empty(&self) -> bool { self.key.is_none() }
    pub fn close_step(&mut self, items: usize) -> Result<UiValueRetirementStep, &'static str> {
        let Some(key) = self.key else { return Ok(UiValueRetirementStep { complete: true, ..Default::default() }); };
        if items == 0 { return Ok(Default::default()); }
        let Some(mut registry) = registry()? else { return Ok(Default::default()); };
        let entry = registry.entry(key, self.queue, self.generation)?;
        if entry.ready.is_some() { return Err("surface output reservation already contains a payload"); }
        registry.release_entry(key.index)?;
        self.key = None;
        Ok(UiValueRetirementStep { complete: true, progressed: true, released_items: 1, released_bytes: 0 })
    }
}

impl Drop for SurfaceReconcileOutputReservation {
    fn drop(&mut self) {
        if let Some(key) = self.key.take() { ENTRY_RETURNS[key.index].store(true, Ordering::Release); }
    }
}
//#endregion 🎟️Reservation

//#region 📤️Outputs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceReconcileOutputTransfer { Pending, Empty, Published }

/// 📬️ A small affine FIFO handle over one shared pool, not a per-transaction payload array.
#[derive(Debug, Default)]
pub struct SurfaceReconcileOutputs { key: Option<Key>, closing: bool }

impl SurfaceReconcileOutputs {
    pub const fn static_backing_bytes() -> usize { size_of::<Mutex<Registry>>() + size_of::<[AtomicBool; SLOTS]>() * 2 }
    pub const fn required_reservation_bytes() -> usize { size_of::<SurfaceReconcileOutputReservation>() + size_of::<Queue>() + size_of::<Key>() * 2 + size_of::<u64>() }
    pub const fn required_transfer_bytes() -> usize { size_of::<SurfaceReconcileReadyPatch>() * 2 }
    pub const fn required_job_transfer_bytes() -> usize { SurfaceReconcileJob::required_ready_transfer_bytes() + size_of::<SurfaceReconcileOutputReservation>() * 2 + size_of::<Option<usize>>() * 3 + size_of::<usize>() * 2 }
    pub fn terminal_is_empty(&self) -> bool { self.key.is_none() }

    pub fn try_reserve(&mut self, generation: u64, admitted_bytes: usize) -> Result<Option<SurfaceReconcileOutputReservation>, &'static str> {
        if self.closing || generation == 0 || admitted_bytes < Self::required_reservation_bytes() { return Ok(None); }
        match super::register_surface_reconcile_backing(admitted_bytes) { Ok(true) => {}, Ok(false) | Err(ui_contract::UiResidentFault::Contended) => return Ok(None), Err(error) => return Err(error.reason()) }
        let Some(mut registry) = registry()? else { return Ok(None); };
        let Some(index) = registry.entries.iter().enumerate().position(|(index, entry)| entry.queue.is_none() && entry.epoch != u64::MAX && !ENTRY_RETURNS[index].load(Ordering::Acquire)) else { return Ok(None); };
        let epoch = registry.entries[index].epoch.checked_add(1).ok_or("surface output entry epoch exhausted")?;
        let queue_key = if let Some(key) = self.key {
            if registry.queue(key)?.closing { return Ok(None); }
            key
        } else {
            let Some(index) = registry.queues.iter().enumerate().position(|(index, queue)| !queue.occupied && queue.epoch != u64::MAX && !QUEUE_RETURNS[index].load(Ordering::Acquire)) else { return Ok(None); };
            Key { index, epoch: registry.queues[index].epoch.checked_add(1).ok_or("surface output queue epoch exhausted")? }
        };
        let entries = registry.queues[queue_key.index].entries.checked_add(1).filter(|count| *count <= SLOTS).ok_or("surface output entry count overflow")?;
        if self.key.is_none() {
            registry.queues[queue_key.index] = Queue { epoch: queue_key.epoch, occupied: true, ..Queue::EMPTY };
            self.key = Some(queue_key);
        }
        registry.queues[queue_key.index].entries = entries;
        let entry = &mut registry.entries[index];
        entry.epoch = epoch; entry.queue = Some(queue_key); entry.generation = generation;
        Ok(Some(SurfaceReconcileOutputReservation { key: Some(Key { index, epoch }), queue: queue_key, generation }))
    }

    pub fn put(&mut self, reservation: &mut Option<SurfaceReconcileOutputReservation>, source: &mut Option<SurfaceReconcileReadyPatch>, admitted_bytes: usize) -> Result<bool, &'static str> {
        if self.closing || admitted_bytes < Self::required_transfer_bytes() { return Ok(false); }
        let (Some(queue_key), Some(owner), Some(ready)) = (self.key, reservation.as_ref(), source.as_ref()) else { return Ok(false); };
        let key = owner.key.ok_or("surface output reservation is already released")?;
        if owner.queue != queue_key || owner.generation != ready.generation() { return Err("surface output publication does not match its reservation"); }
        let Some(mut registry) = registry()? else { return Ok(false); };
        let queue = registry.queue(queue_key)?;
        if queue.closing { return Ok(false); }
        let tail = queue.tail;
        let length = queue.length.checked_add(1).filter(|count| *count <= SLOTS).ok_or("surface output length overflow")?;
        if registry.entry(key, queue_key, owner.generation)?.ready.is_some() { return Err("surface output reservation is already published"); }
        registry.entries[key.index].ready = source.take();
        if let Some(tail) = tail { registry.entries[tail].next = Some(key.index); }
        let queue = &mut registry.queues[queue_key.index];
        if queue.head.is_none() { queue.head = Some(key.index); }
        queue.tail = Some(key.index); queue.length = length;
        reservation.as_mut().unwrap().key = None;
        reservation.take();
        Ok(true)
    }

    pub fn receive_job_into(&mut self, reservation: &mut Option<SurfaceReconcileOutputReservation>, job: &mut SurfaceReconcileJob, current: &mut Option<SurfaceReconciler>, admitted_bytes: usize) -> Result<SurfaceReconcileOutputTransfer, &'static str> {
        if self.closing || current.is_some() || admitted_bytes < Self::required_job_transfer_bytes() { return Ok(SurfaceReconcileOutputTransfer::Pending); }
        let (Some(queue_key), Some(owner)) = (self.key, reservation.as_ref()) else { return Ok(SurfaceReconcileOutputTransfer::Pending); };
        let key = owner.key.ok_or("surface output reservation is already released")?;
        if owner.queue != queue_key || owner.generation != job.generation() { return Err("surface output job does not match its reservation"); }
        let Some(mut registry) = registry()? else { return Ok(SurfaceReconcileOutputTransfer::Pending); };
        let queue = registry.queue(queue_key)?;
        if queue.closing { return Ok(SurfaceReconcileOutputTransfer::Pending); }
        let tail = queue.tail;
        let length = queue.length.checked_add(1).filter(|count| *count <= SLOTS).ok_or("surface output length overflow")?;
        let entries = queue.entries.checked_sub(1).ok_or("surface output entry count underflow")?;
        let entry = registry.entry(key, queue_key, owner.generation)?;
        if entry.ready.is_some() || entry.next.is_some() { return Err("surface output reservation is already published"); }
        if !job.take_ready_into(current, &mut registry.entries[key.index].ready, SurfaceReconcileJob::required_ready_transfer_bytes())? { return Ok(SurfaceReconcileOutputTransfer::Pending); }
        let outcome = if registry.entries[key.index].ready.is_some() {
            if let Some(tail) = tail { registry.entries[tail].next = Some(key.index); }
            let queue = &mut registry.queues[queue_key.index];
            if queue.head.is_none() { queue.head = Some(key.index); }
            queue.tail = Some(key.index); queue.length = length;
            SurfaceReconcileOutputTransfer::Published
        } else {
            registry.entries[key.index].queue = None;
            registry.entries[key.index].generation = 0;
            registry.queues[queue_key.index].entries = entries;
            SurfaceReconcileOutputTransfer::Empty
        };
        reservation.as_mut().expect("retained output reservation").key = None;
        *reservation = None;
        Ok(outcome)
    }

    pub fn take_front_into(&mut self, target: &mut Option<SurfaceReconcileReadyPatch>, admitted_bytes: usize) -> Result<bool, &'static str> {
        if self.closing || target.is_some() || admitted_bytes < Self::required_transfer_bytes() { return Ok(false); }
        let Some(key) = self.key else { return Ok(false); };
        let Some(mut registry) = registry()? else { return Ok(false); };
        let queue = registry.queue(key)?;
        if queue.closing { return Ok(false); }
        let Some(index) = queue.head else { return Ok(false); };
        let length = queue.length.checked_sub(1).ok_or("surface output length underflow")?;
        if registry.entries[index].ready.is_none() { return Err("surface output linked entry has no payload"); }
        let next = registry.entries[index].next;
        *target = registry.entries[index].ready.take();
        registry.entries[index].next = None;
        registry.release_entry(index)?;
        let queue = &mut registry.queues[key.index];
        queue.head = next; queue.length = length;
        if next.is_none() { queue.tail = None; }
        Ok(true)
    }

    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if items == 0 || bytes == 0 { return Ok(Default::default()); }
        let Some(key) = self.key else { self.closing = true; return Ok(UiValueRetirementStep { complete: true, ..Default::default() }); };
        let Some(mut registry) = registry()? else { return Ok(Default::default()); };
        registry.queue(key)?;
        self.closing = true;
        registry.queues[key.index].closing = true;
        let step = registry.close_queue_one(key, bytes)?;
        if step.complete { self.key = None; }
        Ok(step)
    }

    /// 🔁️ One returned entry or one queue position advances without waiting for a mutex.
    pub fn drain_one(items: usize, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if items == 0 || bytes == 0 { return Ok(Default::default()); }
        let Some(mut registry) = registry()? else { return Ok(Default::default()); };
        let index = registry.entry_cursor;
        if ENTRY_RETURNS[index].load(Ordering::Acquire) {
            registry.release_entry(index)?;
            ENTRY_RETURNS[index].store(false, Ordering::Release);
            registry.entry_cursor = (index + 1) % SLOTS;
            return Ok(UiValueRetirementStep { progressed: true, released_items: 1, ..Default::default() });
        }
        registry.entry_cursor = (index + 1) % SLOTS;
        let index = registry.queue_cursor;
        if QUEUE_RETURNS[index].load(Ordering::Acquire) {
            let key = Key { index, epoch: registry.queues[index].epoch };
            registry.queue(key)?;
            registry.queues[index].closing = true;
            let mut step = registry.close_queue_one(key, bytes)?;
            if step.complete { QUEUE_RETURNS[index].store(false, Ordering::Release); }
            registry.queue_cursor = (index + 1) % SLOTS;
            step.complete = false;
            return Ok(step);
        }
        registry.queue_cursor = (index + 1) % SLOTS;
        Ok(Default::default())
    }
}

impl Drop for SurfaceReconcileOutputs {
    fn drop(&mut self) {
        if let Some(key) = self.key.take() { QUEUE_RETURNS[key.index].store(true, Ordering::Release); }
    }
}
//#endregion 📤️Outputs

//#region 🧪️ExactReturns
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_output_pool_contended_drop_preserves_reserved_entry_until_exact_drain() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture.json")).unwrap();
        let mut queue = SurfaceReconcileOutputs::default();
        let reservation = queue.try_reserve(81, 32768).unwrap().unwrap();
        let entry_key = reservation.key.unwrap();
        let queue_key = queue.key.unwrap();
        let guard = REGISTRY.lock().unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        let worker = std::thread::spawn(move || { drop(reservation); drop(queue); tx.send(()).unwrap(); });
        let waited = rx.recv_timeout(std::time::Duration::from_millis(100)).is_err();
        assert!(guard.entry(entry_key, queue_key, 81).is_ok());
        drop(guard);
        worker.join().unwrap();
        for _ in 0..SLOTS * 3 { SurfaceReconcileOutputs::drain_one(1, 1).unwrap(); }
        let registry = REGISTRY.lock().unwrap();
        assert!(!registry.queues[queue_key.index].occupied);
        assert!(registry.entries[entry_key.index].queue.is_none());
        assert!(!ENTRY_RETURNS[entry_key.index].load(Ordering::Acquire));
        assert!(!QUEUE_RETURNS[queue_key.index].load(Ordering::Acquire));
        assert_eq!(waited, fixture["dropWaits"].as_bool().unwrap());
        eprintln!("[DEBUG] output-pool held-mutex-drop-waits={waited} exact-return-drained=true");
    }

    #[test]
    fn surface_output_pool_defers_reuse_and_rejects_stale_epoch_after_final_return() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture.json")).unwrap();
        let mut queue = SurfaceReconcileOutputs::default();
        let mut reservations: Vec<_> = (1..=64).map(|generation| queue.try_reserve(generation, 32768).unwrap().unwrap()).collect();
        let old = reservations.pop().unwrap();
        let old_key = old.key.unwrap();
        let queue_key = old.queue;
        let old_generation = old.generation;
        drop(old);
        assert_eq!(queue.try_reserve(100, 32768).unwrap().is_some(), fixture["returnedEntryReusableBeforeDrain"].as_bool().unwrap());
        for _ in 0..SLOTS { SurfaceReconcileOutputs::drain_one(1, 1).unwrap(); }
        let mut replacement = queue.try_reserve(100, 32768).unwrap().unwrap();
        let new_key = replacement.key.unwrap();
        assert_eq!(old_key.index, new_key.index);
        assert_eq!(new_key.epoch, old_key.epoch + 1);
        assert_eq!(REGISTRY.lock().unwrap().entry(old_key, queue_key, old_generation).is_ok(), fixture["staleEpochAccepted"].as_bool().unwrap());
        while !replacement.close_step(1).unwrap().complete {}
        drop(replacement);
        assert!(!ENTRY_RETURNS[new_key.index].load(Ordering::Acquire));
        for owner in &mut reservations { while !owner.close_step(1).unwrap().complete {} }
        while !queue.close_step(1, 1).unwrap().complete {}
        eprintln!("[DEBUG] output-pool reuse-before-drain=false exact-epoch={} explicit-close-no-second-return=true", new_key.epoch);
    }

    #[test]
    fn surface_output_pool_zero_grant_and_busy_registry_leave_authority_unchanged() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture.json")).unwrap();
        let mut queue = SurfaceReconcileOutputs::default();
        assert!(!queue.close_step(0, 4096).unwrap().progressed);
        let mut reservation = queue.try_reserve(91, 32768).unwrap().unwrap();
        let key = queue.key;
        let entry = reservation.key;
        let guard = REGISTRY.lock().unwrap();
        assert!(!queue.close_step(1, 4096).unwrap().progressed);
        assert!(!reservation.close_step(1).unwrap().progressed);
        assert!(queue.try_reserve(92, 32768).unwrap().is_none());
        assert_eq!(queue.key, key);
        assert_eq!(reservation.key, entry);
        assert!(!queue.closing);
        drop(guard);
        assert_eq!(queue.close_step(1, 0).unwrap().progressed, fixture["zeroGrantMutates"].as_bool().unwrap());
        while !reservation.close_step(1).unwrap().complete {}
        while !queue.close_step(1, 1).unwrap().complete {}
        eprintln!("[DEBUG] output-pool busy-refusal-exact=true zero-grant-mutates=false static-bytes={}", SurfaceReconcileOutputs::static_backing_bytes());
    }
}
//#endregion 🧪️ExactReturns

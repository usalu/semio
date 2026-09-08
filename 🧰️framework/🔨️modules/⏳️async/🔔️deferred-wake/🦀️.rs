//! 🔔️ Fixed pool-owned waker transfer, isolated from job and maintenance admission.

use super::{Mutex, PoisonError, Waker};
use std::sync::atomic::{AtomicU64, Ordering};

pub const WORKER_DEFERRED_WAKE_PARTITIONS: usize = 64;
pub const WORKER_DEFERRED_WAKES_PER_PARTITION: usize = 32;
pub const WORKER_DEFERRED_WAKE_CAPACITY: usize = WORKER_DEFERRED_WAKE_PARTITIONS * WORKER_DEFERRED_WAKES_PER_PARTITION;
const _: () = assert!(WORKER_DEFERRED_WAKE_PARTITIONS <= u8::MAX as usize);
const _: () = assert!(WORKER_DEFERRED_WAKES_PER_PARTITION <= u8::MAX as usize);
static NEXT_DEFERRED_WAKE_POOL_ID: AtomicU64 = AtomicU64::new(1);

/// 🎫️ One generation-qualified partition reserved by a subsystem controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkerDeferredWakeTicket {
    pool: u64,
    partition: u8,
    generation: u64,
}

/// 🚧️ A rejected transfer returns its exact waker owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerDeferredWakeError {
    Capacity,
    GenerationExhausted,
    Occupied,
    Stale,
    Closed,
    Shutdown,
}

/// 🧷️ Exact rejected transfer; no fallback callback is invoked.
pub struct WorkerDeferredWakeRejected {
    error: WorkerDeferredWakeError,
    waker: Waker,
}

impl std::fmt::Debug for WorkerDeferredWakeRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("WorkerDeferredWakeRejected").field("error", &self.error).finish()
    }
}

impl WorkerDeferredWakeRejected {
    pub fn error(&self) -> WorkerDeferredWakeError {
        self.error
    }

    pub fn into_waker(self) -> Waker {
        self.waker
    }
}

struct Partition {
    generation: u64,
    closing: bool,
    entries: [Option<Waker>; WORKER_DEFERRED_WAKES_PER_PARTITION],
}

struct State {
    next_generation: u64,
    closed: bool,
    entries: [Option<Partition>; WORKER_DEFERRED_WAKE_PARTITIONS],
    cursor: usize,
}

pub(super) struct WorkerDeferredWakeInvocation {
    waker: Waker,
}

impl WorkerDeferredWakeInvocation {
    pub(super) fn run(self) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.waker.wake()));
    }
}

pub(super) struct WorkerDeferredWakeRegistry {
    identity: u64,
    state: Mutex<State>,
}

impl WorkerDeferredWakeRegistry {
    pub(super) fn new() -> Self {
        let identity = NEXT_DEFERRED_WAKE_POOL_ID.try_update(Ordering::Relaxed, Ordering::Relaxed, |value| value.checked_add(1)).expect("WorkerPool deferred-wake identity exhausted");
        Self { identity, state: Mutex::new(State { next_generation: 1, closed: false, entries: std::array::from_fn(|_| None), cursor: 0 }) }
    }

    pub(super) fn install(&self) -> Result<WorkerDeferredWakeTicket, WorkerDeferredWakeError> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.closed {
            return Err(WorkerDeferredWakeError::Shutdown);
        }
        let partition = state.entries.iter().position(Option::is_none).ok_or(WorkerDeferredWakeError::Capacity)?;
        let generation = state.next_generation;
        state.next_generation = generation.checked_add(1).ok_or(WorkerDeferredWakeError::GenerationExhausted)?;
        state.entries[partition] = Some(Partition { generation, closing: false, entries: std::array::from_fn(|_| None) });
        Ok(WorkerDeferredWakeTicket { pool: self.identity, partition: partition as u8, generation })
    }

    fn exact<'a>(&self, state: &'a mut State, ticket: WorkerDeferredWakeTicket) -> Result<&'a mut Partition, WorkerDeferredWakeError> {
        if ticket.pool != self.identity {
            return Err(WorkerDeferredWakeError::Stale);
        }
        state.entries.get_mut(usize::from(ticket.partition)).and_then(Option::as_mut).filter(|entry| entry.generation == ticket.generation).ok_or(WorkerDeferredWakeError::Stale)
    }

    pub(super) fn defer_wake(&self, ticket: WorkerDeferredWakeTicket, slot: usize, waker: Waker) -> Result<(), WorkerDeferredWakeRejected> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.closed {
            return Err(WorkerDeferredWakeRejected { error: WorkerDeferredWakeError::Shutdown, waker });
        }
        let partition = match self.exact(&mut state, ticket) {
            Ok(partition) => partition,
            Err(error) => return Err(WorkerDeferredWakeRejected { error, waker }),
        };
        if partition.closing {
            return Err(WorkerDeferredWakeRejected { error: WorkerDeferredWakeError::Closed, waker });
        }
        let Some(entry) = partition.entries.get_mut(slot) else {
            return Err(WorkerDeferredWakeRejected { error: WorkerDeferredWakeError::Capacity, waker });
        };
        if entry.is_some() {
            return Err(WorkerDeferredWakeRejected { error: WorkerDeferredWakeError::Occupied, waker });
        }
        *entry = Some(waker);
        Ok(())
    }

    pub(super) fn pending(&self, ticket: WorkerDeferredWakeTicket, slot: usize) -> Result<bool, WorkerDeferredWakeError> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        Ok(self.exact(&mut state, ticket)?.entries.get(slot).is_some_and(Option::is_some))
    }

    pub(super) fn remove(&self, ticket: WorkerDeferredWakeTicket) -> Result<bool, WorkerDeferredWakeError> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let partition = self.exact(&mut state, ticket)?;
        partition.closing = true;
        if partition.entries.iter().any(Option::is_some) {
            return Ok(false);
        }
        state.entries[usize::from(ticket.partition)] = None;
        Ok(true)
    }

    pub(super) fn has_pending(&self) -> bool {
        self.state.lock().unwrap_or_else(PoisonError::into_inner).entries.iter().flatten().any(|partition| partition.entries.iter().any(Option::is_some))
    }

    pub(super) fn shutdown(&self) {
        self.state.lock().unwrap_or_else(PoisonError::into_inner).closed = true;
    }

    pub(super) fn select(&self) -> Option<WorkerDeferredWakeInvocation> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let selected = (0..WORKER_DEFERRED_WAKE_CAPACITY).map(|offset| (state.cursor + offset) % WORKER_DEFERRED_WAKE_CAPACITY).find(|flat| {
            let partition = flat / WORKER_DEFERRED_WAKES_PER_PARTITION;
            let slot = flat % WORKER_DEFERRED_WAKES_PER_PARTITION;
            state.entries[partition].as_ref().is_some_and(|partition| partition.entries[slot].is_some())
        })?;
        state.cursor = (selected + 1) % WORKER_DEFERRED_WAKE_CAPACITY;
        let partition = selected / WORKER_DEFERRED_WAKES_PER_PARTITION;
        let slot = selected % WORKER_DEFERRED_WAKES_PER_PARTITION;
        let waker = state.entries[partition].as_mut().expect("selected exact deferred-wake partition").entries[slot].take().expect("selected exact deferred waker");
        Some(WorkerDeferredWakeInvocation { waker })
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

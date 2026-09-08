//! 🔔️ Fixed, generation-qualified, coalescing work independent of queued closures.
use super::{Job, Lane, Mutex, PoisonError, VecDeque, LANE_COUNT};
use std::sync::atomic::{AtomicU64, Ordering};

/// 📏️ Each pool pre-admits this many reusable maintenance slots.
pub const WORKER_MAINTENANCE_CAPACITY: usize = 64;
const _: () = assert!(WORKER_MAINTENANCE_CAPACITY <= u8::MAX as usize);
static NEXT_POOL_ID: AtomicU64 = AtomicU64::new(1);

/// 🎟️ Only the issuing pool and live generation accept this hook ticket.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkerMaintenanceTicket {
    pool: u64,
    slot: u8,
    generation: u64,
}

/// 👣️ A finite callback yields, sleeps until another request, retains a fault, or retires its
/// callback-owned terminal slot. `Retire` requires sole close authority and must not race an
/// external `remove_maintenance_hook` owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerMaintenanceStep {
    More,
    Idle,
    Fault,
    Retire,
}

/// 🚧️ No refusal consumes a callback or invokes its owner's cleanup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerMaintenanceError {
    Capacity,
    GenerationExhausted,
    Stale,
    Closed,
    Shutdown,
}

/// 📣️ Requests coalesce without inserting a job or consuming another slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerMaintenanceRequest {
    Requested,
    Coalesced,
}

/// 🧭️ A plain function plus two fixed words keeps callbacks allocation-free.
pub type WorkerMaintenanceCallback = fn([u64; 2]) -> WorkerMaintenanceStep;

#[derive(Clone, Copy)]
struct Hook {
    generation: u64,
    lane: Lane,
    callback: WorkerMaintenanceCallback,
    context: [u64; 2],
    requested: bool,
    running: bool,
    closing: bool,
}

struct State {
    next_generation: u64,
    closed: bool,
    entries: [Option<Hook>; WORKER_MAINTENANCE_CAPACITY],
    cursor: [usize; LANE_COUNT],
    hook_first: [bool; LANE_COUNT],
}

pub(super) struct WorkerMaintenanceRegistry {
    identity: u64,
    state: Mutex<State>,
}

pub(super) struct Invocation {
    ticket: WorkerMaintenanceTicket,
    callback: WorkerMaintenanceCallback,
    context: [u64; 2],
}

pub(super) enum PoolWork {
    Job(Job),
    Maintenance(Invocation),
    DeferredWake(super::deferred_wake::WorkerDeferredWakeInvocation),
}

impl PoolWork {
    pub(super) fn run(self, registry: &WorkerMaintenanceRegistry) {
        match self {
            Self::Job(job) => job(),
            Self::Maintenance(invocation) => {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (invocation.callback)(invocation.context))).unwrap_or(WorkerMaintenanceStep::Fault);
                registry.finish(&invocation, result);
            }
            Self::DeferredWake(invocation) => invocation.run(),
        }
    }
}

impl WorkerMaintenanceRegistry {
    pub(super) fn new() -> Self {
        let identity = NEXT_POOL_ID.try_update(Ordering::Relaxed, Ordering::Relaxed, |value| value.checked_add(1)).expect("WorkerPool maintenance identity exhausted");
        Self { identity, state: Mutex::new(State { next_generation: 1, closed: false, entries: [None; WORKER_MAINTENANCE_CAPACITY], cursor: [0; LANE_COUNT], hook_first: [false; LANE_COUNT] }) }
    }

    pub(super) fn install(&self, lane: Lane, callback: WorkerMaintenanceCallback, context: [u64; 2]) -> Result<WorkerMaintenanceTicket, WorkerMaintenanceError> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.closed {
            return Err(WorkerMaintenanceError::Shutdown);
        }
        let slot = state.entries.iter().position(Option::is_none).ok_or(WorkerMaintenanceError::Capacity)?;
        let generation = state.next_generation;
        state.next_generation = generation.checked_add(1).ok_or(WorkerMaintenanceError::GenerationExhausted)?;
        state.entries[slot] = Some(Hook { generation, lane, callback, context, requested: false, running: false, closing: false });
        Ok(WorkerMaintenanceTicket { pool: self.identity, slot: slot as u8, generation })
    }

    fn exact<'a>(&self, state: &'a mut State, ticket: WorkerMaintenanceTicket) -> Result<&'a mut Hook, WorkerMaintenanceError> {
        if ticket.pool != self.identity {
            return Err(WorkerMaintenanceError::Stale);
        }
        state.entries.get_mut(usize::from(ticket.slot)).and_then(Option::as_mut).filter(|entry| entry.generation == ticket.generation).ok_or(WorkerMaintenanceError::Stale)
    }

    pub(super) fn request(&self, ticket: WorkerMaintenanceTicket) -> Result<WorkerMaintenanceRequest, WorkerMaintenanceError> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.closed {
            return Err(WorkerMaintenanceError::Shutdown);
        }
        let entry = self.exact(&mut state, ticket)?;
        if entry.closing {
            return Err(WorkerMaintenanceError::Closed);
        }
        let result = if entry.requested { WorkerMaintenanceRequest::Coalesced } else { WorkerMaintenanceRequest::Requested };
        entry.requested = true;
        Ok(result)
    }

    pub(super) fn remove(&self, ticket: WorkerMaintenanceTicket) -> Result<bool, WorkerMaintenanceError> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let entry = self.exact(&mut state, ticket)?;
        entry.closing = true;
        entry.requested = false;
        if entry.running {
            return Ok(false);
        }
        state.entries[usize::from(ticket.slot)] = None;
        Ok(true)
    }

    pub(super) fn has_pending(&self, lane: Option<Lane>) -> bool {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        !state.closed && state.entries.iter().flatten().any(|entry| entry.requested && !entry.running && !entry.closing && lane.is_none_or(|lane| lane == entry.lane))
    }

    pub(super) fn shutdown(&self) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.closed = true;
        for entry in state.entries.iter_mut().flatten() {
            entry.requested = false;
        }
    }

    pub(super) fn select(&self, lane: Lane, queue: &mut VecDeque<Job>) -> Option<PoolWork> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.closed {
            return queue.pop_front().map(PoolWork::Job);
        }
        let ordinal = lane.index();
        let slot = (0..WORKER_MAINTENANCE_CAPACITY)
            .map(|offset| (state.cursor[ordinal] + offset) % WORKER_MAINTENANCE_CAPACITY)
            .find(|slot| state.entries[*slot].is_some_and(|entry| entry.lane == lane && entry.requested && !entry.running && !entry.closing));
        if !queue.is_empty() && (slot.is_none() || !state.hook_first[ordinal]) {
            state.hook_first[ordinal] = true;
            return queue.pop_front().map(PoolWork::Job);
        }
        let slot = slot?;
        state.cursor[ordinal] = (slot + 1) % WORKER_MAINTENANCE_CAPACITY;
        state.hook_first[ordinal] = false;
        let entry = state.entries[slot].as_mut().expect("selected exact maintenance hook");
        entry.requested = false;
        entry.running = true;
        Some(PoolWork::Maintenance(Invocation { ticket: WorkerMaintenanceTicket { pool: self.identity, slot: slot as u8, generation: entry.generation }, callback: entry.callback, context: entry.context }))
    }

    pub(super) fn select_hook(&self, lane: Lane) -> Option<PoolWork> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.closed {
            return None;
        }
        let ordinal = lane.index();
        let slot = (0..WORKER_MAINTENANCE_CAPACITY)
            .map(|offset| (state.cursor[ordinal] + offset) % WORKER_MAINTENANCE_CAPACITY)
            .find(|slot| state.entries[*slot].is_some_and(|entry| entry.lane == lane && entry.requested && !entry.running && !entry.closing))?;
        state.cursor[ordinal] = (slot + 1) % WORKER_MAINTENANCE_CAPACITY;
        let entry = state.entries[slot].as_mut().expect("selected exact maintenance hook");
        entry.requested = false;
        entry.running = true;
        Some(PoolWork::Maintenance(Invocation { ticket: WorkerMaintenanceTicket { pool: self.identity, slot: slot as u8, generation: entry.generation }, callback: entry.callback, context: entry.context }))
    }

    fn finish(&self, invocation: &Invocation, step: WorkerMaintenanceStep) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let closed = state.closed;
        let slot = usize::from(invocation.ticket.slot);
        let retire = {
            let entry = self.exact(&mut state, invocation.ticket).expect("running maintenance slot cannot be retired or reused");
            assert!(entry.running, "maintenance invocation finished twice");
            step == WorkerMaintenanceStep::Retire
        };
        if retire {
            state.entries[slot] = None;
            return;
        }
        let entry = self.exact(&mut state, invocation.ticket).expect("running maintenance slot cannot be retired or reused");
        entry.running = false;
        if closed || entry.closing {
            entry.requested = false;
        } else if step == WorkerMaintenanceStep::More {
            entry.requested = true;
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

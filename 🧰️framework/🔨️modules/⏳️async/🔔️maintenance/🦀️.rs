//! 🔔️ Fixed, generation-qualified, coalescing work independent of queued closures.
use super::{Job, Lane, Mutex, PoisonError, VecDeque, LANE_COUNT};
use std::sync::atomic::{AtomicU64, Ordering};

/// 📏️ Each pool pre-admits this many reusable maintenance slots.
pub const WORKER_MAINTENANCE_CAPACITY: usize = 64;
const _: () = assert!(WORKER_MAINTENANCE_CAPACITY <= u8::MAX as usize);
static NEXT_POOL_ID: AtomicU64 = AtomicU64::new(1);

/// 🎟️ Only the issuing pool and live generation accept this hook ticket.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkerMaintenanceTicket { pool: u64, slot: u8, generation: u64 }

/// 👣️ A finite callback yields, sleeps until another request, or retains a fault.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerMaintenanceStep { More, Idle, Fault }

/// 🚧️ No refusal consumes a callback or invokes its owner's cleanup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerMaintenanceError { Capacity, GenerationExhausted, Stale, Closed, Shutdown }

/// 📣️ Requests coalesce without inserting a job or consuming another slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerMaintenanceRequest { Requested, Coalesced }

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

pub(super) struct WorkerMaintenanceRegistry { identity: u64, state: Mutex<State> }

pub(super) struct Invocation { ticket: WorkerMaintenanceTicket, callback: WorkerMaintenanceCallback, context: [u64; 2] }

pub(super) enum PoolWork { Job(Job), Maintenance(Invocation) }

impl PoolWork {
    pub(super) fn run(self, registry: &WorkerMaintenanceRegistry) {
        match self {
            Self::Job(job) => job(),
            Self::Maintenance(invocation) => {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (invocation.callback)(invocation.context))).unwrap_or(WorkerMaintenanceStep::Fault);
                registry.finish(invocation, result);
            }
        }
    }
}

impl WorkerMaintenanceRegistry {
    pub(super) fn new() -> Self {
        let identity = NEXT_POOL_ID.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| value.checked_add(1)).expect("WorkerPool maintenance identity exhausted");
        Self { identity, state: Mutex::new(State { next_generation: 1, closed: false, entries: [None; WORKER_MAINTENANCE_CAPACITY], cursor: [0; LANE_COUNT], hook_first: [false; LANE_COUNT] }) }
    }

    pub(super) fn install(&self, lane: Lane, callback: WorkerMaintenanceCallback, context: [u64; 2]) -> Result<WorkerMaintenanceTicket, WorkerMaintenanceError> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.closed { return Err(WorkerMaintenanceError::Shutdown); }
        let slot = state.entries.iter().position(Option::is_none).ok_or(WorkerMaintenanceError::Capacity)?;
        let generation = state.next_generation;
        state.next_generation = generation.checked_add(1).ok_or(WorkerMaintenanceError::GenerationExhausted)?;
        state.entries[slot] = Some(Hook { generation, lane, callback, context, requested: false, running: false, closing: false });
        Ok(WorkerMaintenanceTicket { pool: self.identity, slot: slot as u8, generation })
    }

    fn exact<'a>(&self, state: &'a mut State, ticket: WorkerMaintenanceTicket) -> Result<&'a mut Hook, WorkerMaintenanceError> {
        if ticket.pool != self.identity { return Err(WorkerMaintenanceError::Stale); }
        state.entries.get_mut(usize::from(ticket.slot)).and_then(Option::as_mut).filter(|entry| entry.generation == ticket.generation).ok_or(WorkerMaintenanceError::Stale)
    }

    pub(super) fn request(&self, ticket: WorkerMaintenanceTicket) -> Result<WorkerMaintenanceRequest, WorkerMaintenanceError> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.closed { return Err(WorkerMaintenanceError::Shutdown); }
        let entry = self.exact(&mut state, ticket)?;
        if entry.closing { return Err(WorkerMaintenanceError::Closed); }
        let result = if entry.requested { WorkerMaintenanceRequest::Coalesced } else { WorkerMaintenanceRequest::Requested };
        entry.requested = true;
        Ok(result)
    }

    pub(super) fn remove(&self, ticket: WorkerMaintenanceTicket) -> Result<bool, WorkerMaintenanceError> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let entry = self.exact(&mut state, ticket)?;
        entry.closing = true;
        entry.requested = false;
        if entry.running { return Ok(false); }
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
        for entry in state.entries.iter_mut().flatten() { entry.requested = false; }
    }

    pub(super) fn select(&self, lane: Lane, queue: &mut VecDeque<Job>) -> Option<PoolWork> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.closed { return queue.pop_front().map(PoolWork::Job); }
        let ordinal = lane.index();
        let slot = (0..WORKER_MAINTENANCE_CAPACITY).map(|offset| (state.cursor[ordinal] + offset) % WORKER_MAINTENANCE_CAPACITY).find(|slot| state.entries[*slot].is_some_and(|entry| entry.lane == lane && entry.requested && !entry.running && !entry.closing));
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

    fn finish(&self, invocation: Invocation, step: WorkerMaintenanceStep) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let closed = state.closed;
        let entry = self.exact(&mut state, invocation.ticket).expect("running maintenance slot cannot be retired or reused");
        assert!(entry.running, "maintenance invocation finished twice");
        entry.running = false;
        if closed || entry.closing { entry.requested = false; }
        else if step == WorkerMaintenanceStep::More { entry.requested = true; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).unwrap() }
    fn idle(_: [u64; 2]) -> WorkerMaintenanceStep { WorkerMaintenanceStep::Idle }

    #[test]
    fn worker_maintenance_matches_neutral_retention_and_aba_lifecycle() {
        let registry = WorkerMaintenanceRegistry::new();
        let mut owners = std::collections::BTreeMap::new();
        let mut running = None;
        for step in fixture()["lifecycle"].as_array().unwrap() {
            let owner = step["owner"].as_str().unwrap();
            let actual = match step["action"].as_str().unwrap() {
                "install" => { owners.insert(owner.to_string(), registry.install(Lane::Io, idle, [0; 2]).unwrap()); "installed" }
                "request" => match registry.request(owners[owner]) {
                    Ok(WorkerMaintenanceRequest::Requested) => "requested", Ok(WorkerMaintenanceRequest::Coalesced) => "coalesced",
                    Err(WorkerMaintenanceError::Stale) => "stale", Err(WorkerMaintenanceError::Closed) => "closed", other => panic!("unexpected request {other:?}"),
                },
                "take" => { let Some(PoolWork::Maintenance(invocation)) = registry.select(Lane::Io, &mut VecDeque::new()) else { panic!("requested hook was not selected") }; assert_eq!(invocation.ticket, owners[owner]); running = Some(invocation); "running" }
                "remove" => if registry.remove(owners[owner]).unwrap() { "removed" } else { "pending" },
                action => {
                    let disposition = match action { "finish-more" => WorkerMaintenanceStep::More, "finish-idle" => WorkerMaintenanceStep::Idle, "finish-fault" => WorkerMaintenanceStep::Fault, _ => unreachable!() };
                    registry.finish(running.take().unwrap(), disposition);
                    if registry.has_pending(None) { "requested" } else { "idle" }
                }
            };
            assert_eq!(actual, step["expected"].as_str().unwrap(), "{step}");
        }
        assert!(registry.state.lock().unwrap().entries.iter().all(Option::is_none));
        eprintln!("[DEBUG] fixed maintenance slots matched all 20 neutral coalescing, running-close, concurrent-wake, fault, and ABA transitions");
    }

    #[test]
    fn worker_maintenance_capacity_and_pool_identity_are_exact() {
        let registry = WorkerMaintenanceRegistry::new();
        let other = WorkerMaintenanceRegistry::new();
        let mut tickets = Vec::new();
        assert_eq!(WORKER_MAINTENANCE_CAPACITY, fixture()["capacity"].as_u64().unwrap() as usize);
        for _ in 0..WORKER_MAINTENANCE_CAPACITY { tickets.push(registry.install(Lane::Io, idle, [0; 2]).unwrap()); }
        assert_eq!(registry.install(Lane::Io, idle, [0; 2]), Err(WorkerMaintenanceError::Capacity));
        assert_eq!(other.request(tickets[0]), Err(WorkerMaintenanceError::Stale));
        let old = tickets.remove(0);
        assert!(registry.remove(old).unwrap());
        let fresh = registry.install(Lane::Io, idle, [0; 2]).unwrap();
        assert_eq!(old.slot, fresh.slot);
        assert_ne!(old.generation, fresh.generation);
        assert_eq!(registry.request(old), Err(WorkerMaintenanceError::Stale));
        tickets.push(fresh);
        for ticket in tickets { assert!(registry.remove(ticket).unwrap()); }
        registry.state.lock().unwrap().next_generation = u64::MAX;
        assert_eq!(registry.install(Lane::Io, idle, [0; 2]), Err(WorkerMaintenanceError::GenerationExhausted));
        eprintln!("[DEBUG] maintenance admission fenced foreign pools, capacity+1, retired generations, and exhausted generation without wrapping");
    }
}

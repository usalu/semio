//! 🎚️ Mailbox priority, capabilities, and open profiles.

use crate::db_durability::DurabilityClass;
use crate::*;

//#region 🔖️Priority
/// @emoji 🚦️ The six bounded mailbox lanes every document actor's inbox is split into
/// (`db_actor`'s deficit-round-robin scheduler drains them by weight; admission sheds the lowest
/// first under backpressure). Declaration order is priority order, highest first.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Priority {
    /// @emoji 🛑️ Supervision/control messages (shutdown, generation bump) — never shed, never queued behind anything.
    System,
    /// @emoji 🩹️ WAL replay / crash-recovery traffic, run before the actor accepts ordinary work.
    Recovery,
    /// @emoji ✍️ Ordinary command submissions (the actor's core job).
    Command,
    /// @emoji 🔎️ One-shot queries against canonical/historical state.
    Query,
    /// @emoji 📡️ Live-query change notifications to subscribers.
    Live,
    /// @emoji 🌫️ Ephemeral preview publishes — lowest priority, the only lane ever shed under
    /// backpressure (previews are never durable and never allowed to delay a command, per the
    /// contract's preview law).
    Preview,
}

impl Priority {
    /// @emoji 📋️ Every lane, in priority order — the shape `db_actor`'s mailbox array indexes by.
    pub const ALL: [Priority; 6] = [Priority::System, Priority::Recovery, Priority::Command, Priority::Query, Priority::Live, Priority::Preview];

    /// @emoji 🔢️ A dense `0..6` index matching declaration order, for array-indexed mailbox storage.
    pub fn rank(self) -> usize {
        match self {
            Priority::System => 0,
            Priority::Recovery => 1,
            Priority::Command => 2,
            Priority::Query => 3,
            Priority::Live => 4,
            Priority::Preview => 5,
        }
    }

    /// @emoji ✂️ True only for `Preview` — the contract's "shed-previews-first admission" law: a
    /// full mailbox drops the oldest preview rather than ever rejecting/blocking a higher lane.
    pub fn sheddable(self) -> bool {
        matches!(self, Priority::Preview)
    }

    /// @emoji ⚖️ Default deficit-round-robin weight per lane (this crate's own choice — the
    /// contract fixes the lane set and shedding law, not the exact weights). Halves lane-to-lane
    /// so a starved low lane still makes bounded progress without letting `Preview` traffic
    /// compete meaningfully with `Command`.
    pub fn default_weight(self) -> u32 {
        match self {
            Priority::System => 64,
            Priority::Recovery => 32,
            Priority::Command => 16,
            Priority::Query => 8,
            Priority::Live => 4,
            Priority::Preview => 1,
        }
    }
}
//#endregion 🔖️Priority

//#region 🔖️Capabilities
/// @emoji 🧰️ What a particular `Database` instance supports — negotiated at `open` time from the
/// storage backend's own `StorageCapabilities` (`db_storage`) plus enabled Cargo features, and
/// surfaced to clients (e.g. so `framework/sync` knows whether to offer preview publishing).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct DbCapabilities {
    pub preview: bool,
    pub historical_query: bool,
    pub live_query: bool,
    pub cluster: bool,
    pub max_durability: DurabilityClass,
}
//#endregion 🔖️Capabilities

//#region 🔖️Config
/// @emoji 🎛️ Which of the family's built-in default profiles a `Database::open` call selects —
/// `db_config`-equivalent defaults live entirely in this crate (see `DbConfig::for_profile`) so
/// every crate constructing a config in a test gets the same baseline.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Profile {
    /// @emoji 🧪️ Deterministic, low-latency defaults for unit/integration tests: `Memory`
    /// durability (no real fsync cost), tight limits (catches runaway fixtures fast).
    Test,
    /// @emoji 🛠️ A local developer loop: `Os` durability (survives a process crash, not a power
    /// loss), generous limits.
    Dev,
    /// @emoji 🏭️ Production defaults: `Fsync` durability, the family's full resource ceilings.
    Prod,
}

/// @emoji 🚦️ Per-`Priority`-lane mailbox bounds, indexed by `Priority::rank`.
#[derive(Clone, Copy, Debug)]
pub struct MailboxCapacities([u32; 6]);

impl MailboxCapacities {
    /// @emoji 🟰️ The same bound for every lane.
    pub fn uniform(capacity: u32) -> Self {
        Self([capacity; 6])
    }

    /// @emoji 📖️ The bound for `priority`'s lane.
    pub fn get(&self, priority: Priority) -> u32 {
        self.0[priority.rank()]
    }

    /// @emoji ✏️ Overrides the bound for `priority`'s lane.
    pub fn set(&mut self, priority: Priority, capacity: u32) {
        self.0[priority.rank()] = capacity;
    }
}

impl Default for MailboxCapacities {
    fn default() -> Self {
        Self::uniform(1_024)
    }
}

/// @emoji ⚙️ Everything a `Database::open` needs beyond the storage backend itself: limits,
/// default durability, capability negotiation inputs, and mailbox sizing.
#[derive(Clone, Debug)]
pub struct DbConfig {
    pub profile: Profile,
    pub limits: DbLimits,
    pub default_durability: DurabilityClass,
    pub capabilities: DbCapabilities,
    pub mailbox_capacities: MailboxCapacities,
}

impl DbConfig {
    /// @emoji 🏗️ Builds the family's well-justified defaults for `profile` (see `Profile`'s doc
    /// for the reasoning behind each choice) — the starting point every `Database::open_at`
    /// (zero-touch) call and every crate's tests should build from rather than hand-rolling limits.
    pub fn for_profile(profile: Profile) -> DbConfig {
        let (default_durability, limits, mailbox_capacity) = match profile {
            Profile::Test => (DurabilityClass::Memory, DbLimits { max_command_bytes: 64 * 1024, max_batch_commands: 64, ..DbLimits::default() }, 64),
            Profile::Dev => (DurabilityClass::Os, DbLimits::default(), 1_024),
            Profile::Prod => (DurabilityClass::Fsync, DbLimits::default(), 65_536),
        };
        DbConfig {
            profile,
            limits,
            default_durability,
            capabilities: DbCapabilities { preview: true, historical_query: true, live_query: true, cluster: matches!(profile, Profile::Prod), max_durability: default_durability },
            mailbox_capacities: MailboxCapacities::uniform(mailbox_capacity),
        }
    }
}
//#endregion 🔖️Config

//#region 🔖️WorkerSubmitRetry
/// @emoji 🔁️ The attempt a worker-pool submission refused with `kind` retries with, or `None` when
/// the refusal is terminal for its owner.
///
/// `Contended` says another thread held that lane's queue lock for an instant (idle workers scan
/// and steal from every queue), never that the pool is full, so it spends no attempt: counting it
/// refused a document's edit after eight 1 ms lock races while 24 documents were busy (ticket
/// 26/09/23 H9 session 12, growth e2e g14). `Saturated` (a full lane) spends one attempt of
/// `limit`; `Shutdown` and `Poisoned` never retry. The decision table is the language-agnostic
/// fixture `🧫️fixtures/🔁️worker-submit-retry/🔣️.json`.
pub fn worker_submit_retry_attempt(kind: semio_framework_async::WorkerSubmitErrorKind, attempt: u8, limit: u8) -> Option<u8> {
    match kind {
        semio_framework_async::WorkerSubmitErrorKind::Contended => Some(attempt),
        semio_framework_async::WorkerSubmitErrorKind::Saturated if attempt < limit => Some(attempt + 1),
        semio_framework_async::WorkerSubmitErrorKind::Saturated | semio_framework_async::WorkerSubmitErrorKind::Shutdown | semio_framework_async::WorkerSubmitErrorKind::Poisoned => None,
    }
}
//#endregion 🔖️WorkerSubmitRetry

//#region 🔖️AdmissionWaiters
/// @emoji 🎟️ The tasks waiting for a fixed-slot admission to free a slot, in a table of `N` entries
/// that never grows. A retained owner's exact admission still refuses at capacity (its laws pin
/// the refusal and the owners it hands back); an async caller that wants the slot rather than the
/// refusal registers here and is woken whenever a slot is released, then claims again. Every
/// waiter is woken on release (at most `N`), so no wake is lost to a waiter that was cancelled
/// between its wake and its claim.
pub struct AdmissionWaiters<const N: usize> {
    entries: [Option<(u64, std::task::Waker)>; N],
    next_id: u64,
}

impl<const N: usize> AdmissionWaiters<N> {
    /// @emoji 🆕️ An empty table, usable in a `static`.
    pub const fn new() -> Self {
        Self { entries: [const { None }; N], next_id: 1 }
    }

    /// @emoji ✍️ Registers `waker` under `*id` (allocating the id on first use) or refreshes it;
    /// `false` when all `N` entries are taken by other waiters.
    pub fn register(&mut self, id: &mut Option<u64>, waker: &std::task::Waker) -> bool {
        if let Some(current) = *id {
            if let Some((_, held)) = self.entries.iter_mut().flatten().find(|(entry, _)| *entry == current) {
                if !held.will_wake(waker) {
                    held.clone_from(waker);
                }
                return true;
            }
        }
        let Some(free) = self.entries.iter_mut().find(|entry| entry.is_none()) else { return false };
        let assigned = id.unwrap_or_else(|| {
            let next = self.next_id;
            self.next_id = self.next_id.wrapping_add(1).max(1);
            next
        });
        *free = Some((assigned, waker.clone()));
        *id = Some(assigned);
        true
    }

    /// @emoji 🧹️ Removes the waiter `id`, if it is still registered.
    pub fn remove(&mut self, id: u64) {
        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.as_ref().is_some_and(|(held, _)| *held == id)) {
            *entry = None;
        }
    }

    /// @emoji 📣️ Clones every registered waker, for the releasing owner to wake after it drops its lock.
    pub fn wakers(&self) -> [Option<std::task::Waker>; N] {
        std::array::from_fn(|index| self.entries[index].as_ref().map(|(_, waker)| waker.clone()))
    }

    /// @emoji 🔢️ How many waiters are registered.
    pub fn len(&self) -> usize {
        self.entries.iter().flatten().count()
    }

    /// @emoji 🈳️ Whether no waiter is registered.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Default for AdmissionWaiters<N> {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🔖️AdmissionWaiters

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

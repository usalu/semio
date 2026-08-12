//! 🎚️ Mailbox priority, capabilities, and open profiles.

use crate::*;
use crate::db_ids::DbError;
use crate::db_durability::DurabilityClass;

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

#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️Priority
    #[test]
    fn priority_rank_matches_declaration_order() {
        for (index, priority) in Priority::ALL.iter().enumerate() {
            assert_eq!(priority.rank(), index);
        }
        assert!(Priority::System < Priority::Command);
        assert!(Priority::Command < Priority::Preview);
    }

    #[test]
    fn only_preview_is_sheddable() {
        for priority in Priority::ALL {
            assert_eq!(priority.sheddable(), priority == Priority::Preview);
        }
    }

    #[test]
    fn default_weights_are_strictly_decreasing_by_priority_order() {
        let weights: Vec<u32> = Priority::ALL.iter().map(|priority| priority.default_weight()).collect();
        for window in weights.windows(2) {
            assert!(window[0] > window[1], "weights must strictly decrease: {weights:?}");
        }
    }
    //#endregion 🔖️Priority

    //#region 🔖️Config
    #[test]
    fn mailbox_capacities_get_set_round_trip_per_lane() {
        let mut capacities = MailboxCapacities::uniform(10);
        assert_eq!(capacities.get(Priority::Command), 10);
        capacities.set(Priority::Preview, 2);
        assert_eq!(capacities.get(Priority::Preview), 2);
        assert_eq!(capacities.get(Priority::System), 10);
    }

    #[test]
    fn profile_defaults_order_durability_test_below_dev_below_prod() {
        let test_config = DbConfig::for_profile(Profile::Test);
        let dev_config = DbConfig::for_profile(Profile::Dev);
        let prod_config = DbConfig::for_profile(Profile::Prod);
        assert!(test_config.default_durability < dev_config.default_durability);
        assert!(dev_config.default_durability < prod_config.default_durability);
        assert!(!test_config.capabilities.cluster);
        assert!(prod_config.capabilities.cluster);
        assert_eq!(test_config.capabilities.max_durability, test_config.default_durability);
    }

    #[test]
    fn test_profile_has_tighter_limits_than_prod() {
        let test_config = DbConfig::for_profile(Profile::Test);
        let prod_config = DbConfig::for_profile(Profile::Prod);
        assert!(test_config.limits.max_command_bytes < prod_config.limits.max_command_bytes);
        assert!(test_config.limits.max_batch_commands < prod_config.limits.max_batch_commands);
    }
    //#endregion 🔖️Config
}

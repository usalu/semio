//! 🕰️ The hybrid logical clock and the decided-but-unpersisted directory event. A leaf: it depends on neither the artifact authority nor any sibling module.

use directory::os_directory::{DirectoryActor, DirectoryEventBody, Hlc};

//#region 🔖️Events
pub fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

/// ⏱️ A hybrid logical clock: wall-clock milliseconds plus a same-millisecond tiebreak
/// counter, monotone across `tick()` calls regardless of how the OS clock jitters. One instance
/// lives behind `DirectoryService`'s write lock (see `//#region 🔖️Service`) — the lock is what
/// makes a `HubClock`'s stream of `tick()`s a total order across every command this hub instance
/// executes, which `append_events` then turns into a dense backend-assigned `seq`.
#[derive(Clone, Copy, Debug, Default)]
pub struct HubClock {
    pub physical_ms: i64,
    pub logical: u32,
}

impl HubClock {
    pub fn new() -> Self {
        Self::default()
    }

    /// ⏭️ Advances the clock by one tick and returns its new `Hlc`. Same millisecond as the
    /// last tick ⇒ the logical counter increments; a later millisecond ⇒ it resets to 0. Time never
    /// runs backward even if the OS clock does (the physical component only ever holds or advances).
    pub fn tick(&mut self) -> Hlc {
        let observed = now_ms();
        if observed > self.physical_ms {
            self.physical_ms = observed;
            self.logical = 0;
        } else {
            self.logical += 1;
        }
        Hlc { physical_ms: self.physical_ms, logical: self.logical }
    }
}

/// ✉️ One decided-but-not-yet-persisted directory event: everything `decide` (see
/// `//#region 🔖️Decider`) can determine on its own. `append_events` (the `EventLog` trait,
/// implemented once per backend) turns this into a full `DirectoryEvent` by assigning the
/// backend-dense `seq`, minting a uuid v7 `id`, and stamping `recorded_at_ms`.
#[derive(Clone, Debug)]
pub struct NewDirectoryEvent {
    pub hlc: Hlc,
    pub actor: DirectoryActor,
    pub space_id: Option<String>,
    pub user_id: Option<String>,
    pub body: DirectoryEventBody,
}
//#endregion 🔖️Events

//! ⏱️ Replay turn admission belongs to execution ownership.
/// ⏱️ How far one turn of a deferred reprojection replays (design §7: ≤ 4 ms slices): until the turn's wall deadline on
/// `now_us` — `wall_us` after the turn began, or the runtime's own turn deadline ([`ArtifactStore::step_reprojection`]) —
/// and at most `operations` operations, whichever comes first; every turn replays at least one operation. `now_us` is the
/// job clock ([`semio_framework_job::default_now_us`]); laws inject their own.
#[derive(Clone, Copy, Debug)]
pub struct ReplayTurnBudget {
    pub wall_us: u64,
    pub operations: usize,
    pub now_us: fn() -> Option<u64>,
}

impl ReplayTurnBudget {
    /// 🕓️ `wall_us` of the job clock per turn, with `operations` as the secondary cap.
    pub fn wall(wall_us: u64, operations: usize) -> Self {
        Self { wall_us, operations, now_us: semio_framework_job::default_now_us }
    }

    /// 🔢️ `operations` per turn and no wall limit: what a law that counts operations replays with.
    pub fn operations(operations: usize) -> Self {
        Self { wall_us: u64::MAX, operations, now_us: semio_framework_job::default_now_us }
    }

    /// ⌛️ The turn's deadline on `now_us`: the runtime's `deadline_us`, else `wall_us` from now (now when the clock cannot
    /// be read); `None` without a wall limit.
    pub(crate) fn turn_deadline(&self, deadline_us: Option<u64>) -> Option<u64> {
        deadline_us.or_else(|| (self.wall_us != u64::MAX).then(|| (self.now_us)().map_or(0, |now| now.saturating_add(self.wall_us))))
    }

    /// 🛎️ Whether a turn that replayed `replayed` operations ends: its cap is reached or its deadline passed on `now_us`
    /// (an unreadable clock ends it).
    pub(crate) fn turn_ends(&self, replayed: usize, deadline_us: Option<u64>) -> bool {
        replayed >= self.operations || deadline_us.is_some_and(|deadline| (self.now_us)().is_none_or(|now| now >= deadline))
    }
}


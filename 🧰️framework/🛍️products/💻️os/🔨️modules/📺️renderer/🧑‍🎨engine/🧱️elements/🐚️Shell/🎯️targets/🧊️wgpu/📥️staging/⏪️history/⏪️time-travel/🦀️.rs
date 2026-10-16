//! ⏪️ `shell-history` staging of `time-travel`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::time_travel::*;
use semio_framework::kernel::{HistoryReprojection, HistoryReprojectionStatus, HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage};

impl HistoryState {
    /// ⏪️ The live session status, if a history edit is open.
    pub fn history_time_travel(&self) -> Option<&HistoryTimeTravel> {
        self.history_time_travel.as_ref()
    }

    /// 📡️ Takes the history change one history patch says replays before this replica adopts it (absent = none waits).
    pub(crate) fn observe_history_reprojection(&mut self, reprojection: Option<&HistoryReprojection>) {
        self.history_reprojection = reprojection.cloned();
    }
}


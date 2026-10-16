//! 🎷️ Staged slice of the `plugin-app-impl` band of the mounted artifact instance: its `instance-children` members (WP6 step S0 of the instance family).
//!
//! 🔌️ The `PluginApp` implementation of the mounted artifact instance with the framework-reserved settle helpers and the maintenance stage constants.
use super::*;
use crate::tool_jobs::tool_run::TOOL_RUN_TRACE_DELTA_BYTES;

/// 🌡️ Occupancy of the child-root or child-member retirement registry at which maintenance spends out-of-turn steps on
/// it — a quarter of the fixed [`ARTIFACT_LIVE_OUTPUT_SLOTS`], as the stores' displaced-owner pressure is of theirs.
pub(crate) const MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY: usize = ARTIFACT_LIVE_OUTPUT_SLOTS / 4;

impl<M: SpaceMember + MemberFactory + 'static> ChildComposition<M> {
    /// 🌡️ True while the child-root or child-member retirement registry holds at least
    /// [`MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY`] retirements. An app that re-points a slot at a new
    /// content-addressed child per edit (writer mints one per keystroke) admits one of each per commit, which the single
    /// fair step per turn drains once per rotation: a typing run saturated the fixed slots with
    /// `interactive-job.child-root-retirement-saturated`.
    pub(crate) fn child_retirements_under_pressure(&self) -> bool {
        self.child_content_retirements.len() >= MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY || self.child_member_retirements.len() >= MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY
    }
}

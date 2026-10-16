//! 📽️ The capability trait of the projection rebuild.

use super::*;

//#region ProjectionRebuild
/// 📽️ Projection rebuild: truncates every projection table and replays the whole event log.
pub trait ProjectionRebuild: Send + Sync + 'static {
    /// 🔁️ Truncates every projection table and replays the whole log from `seq` 1 through
    /// each event's projection (`//#region 🔖️Projections`) — returns the number of events replayed
    /// (which must equal `head_seq()` afterward). `POST /admin/api/directory/rebuild` (contract C2).
    async fn rebuild_projections(&self) -> DirectoryResult<u64>;
    /// 🔁️ Bounded replay with monotonic progress and success-only transaction commit.
    async fn rebuild_projections_controlled(&self, control: &dyn ProjectionRebuildControl) -> DirectoryResult<u64>;
}
//#endregion ProjectionRebuild

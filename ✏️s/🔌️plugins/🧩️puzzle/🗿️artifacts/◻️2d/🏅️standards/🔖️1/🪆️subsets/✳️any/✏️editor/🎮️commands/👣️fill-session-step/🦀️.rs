//! 🖌️ `fill-session-step` command.

use crate::editor::puzzle2d::config::{Puzzle2dFillLifecycle, Puzzle2dFillRuntime};

/// 👣️ Resumes a fill session. The placements a previous session accepted are already committed to
/// the document, so the remaining count plus the stored seed is the entire continuation state —
/// there is no live owner to re-attach to. Returns `None` when nothing is left to search, having
/// settled the lifecycle.
pub fn fill_session_step(runtime: &mut Puzzle2dFillRuntime) -> Result<Option<(u32, u64)>, &'static str> {
    let accepted = u32::try_from(runtime.fill_job_accepted_count).unwrap_or(u32::MAX);
    let remaining = runtime.fill_count.saturating_sub(accepted);
    let resumable = matches!(runtime.fill_job_lifecycle, Puzzle2dFillLifecycle::Capturing | Puzzle2dFillLifecycle::Queued | Puzzle2dFillLifecycle::Running | Puzzle2dFillLifecycle::CheckpointReady | Puzzle2dFillLifecycle::Applying);
    if remaining == 0 || !resumable {
        runtime.fill_job_lifecycle = if runtime.fill_job_fault_code.is_some() { Puzzle2dFillLifecycle::Faulted } else { Puzzle2dFillLifecycle::Completed };
        return Ok(None);
    }
    Ok(Some((remaining, runtime.fill_job_seed.max(1))))
}

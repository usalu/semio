//! 🩺️ Publishes a validated coordinate repair under the original session owner.
use super::{RetainedAllocationEffect, RetainedCloneProgress, Session, SolidHealing, ValueError};

impl Session {
    /// 📤️ Commits one original solid's funded repair.
    pub fn commit_healing(&self, handle: &str, job: &mut SolidHealing, effect: &mut RetainedAllocationEffect<'_>) -> Result<(bool, RetainedCloneProgress), ValueError> {
        let (ready, _, receipt) = self.with_open_kernel(|kernel| kernel.commit_healing(handle, job, effect))?;
        Ok((ready, receipt))
    }
}

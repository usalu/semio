//! 🪟️ Group visibility: the shared decision bit with which a staged history group becomes visible to every direct reader at once.
//!
//! Holds the `🧩️GroupHistoryVisibility` region of the module; the root re-exports its items.

use super::*;

//#region 🧩️GroupHistoryVisibility
/// 🪟 A shared decision bit used by prepared read roots; only its unique publisher can switch it.
pub struct ArtifactGroupVisibility {
    state: std::sync::atomic::AtomicU8,
}

pub struct ArtifactGroupReadDecision<'a> {
    pub(super) visibility: &'a ArtifactGroupVisibility,
    pub(super) committed: bool,
}

impl ArtifactGroupReadDecision<'_> {
    pub fn committed_for(&self, visibility: &ArtifactGroupVisibility) -> Result<bool, ()> {
        if std::ptr::eq(self.visibility, visibility) {
            Ok(self.committed)
        } else {
            Err(())
        }
    }
}

/// 🗝️ Unique low-level publication owner; preparation and freshness remain the Store coordinator's responsibility.
pub struct ArtifactGroupVisibilityOwner {
    view: std::sync::Arc<ArtifactGroupVisibility>,
}

impl ArtifactGroupVisibilityOwner {
    pub fn new() -> Self {
        Self { view: std::sync::Arc::new(ArtifactGroupVisibility { state: std::sync::atomic::AtomicU8::new(0) }) }
    }

    pub fn view(&self) -> std::sync::Arc<ArtifactGroupVisibility> {
        std::sync::Arc::clone(&self.view)
    }

    pub fn commit(&mut self) -> bool {
        self.view.state.compare_exchange(0, 1, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()
    }

    pub fn abort(&mut self) -> bool {
        self.view.state.compare_exchange(0, 2, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()
    }
}

impl Drop for ArtifactGroupVisibilityOwner {
    fn drop(&mut self) {
        let _ = self.abort();
    }
}

impl ArtifactGroupVisibility {
    pub fn capture(&self) -> ArtifactGroupReadDecision<'_> {
        ArtifactGroupReadDecision { visibility: self, committed: self.committed() }
    }

    /// 🟢️ Reads whether the shared owner published its complete staged group.
    pub fn committed(&self) -> bool {
        self.state.load(std::sync::atomic::Ordering::Acquire) == 1
    }

    /// 🟡️ Reads whether the shared owner still permits private candidate construction.
    pub fn pending(&self) -> bool {
        self.state.load(std::sync::atomic::Ordering::Acquire) == 0
    }
}

pub(super) struct ArtifactHistoryGroupSuffix {
    pub(super) visibility: std::sync::Arc<ArtifactGroupVisibility>,
    pub(super) head: Option<u32>,
    pub(super) tail: Option<u32>,
    pub(super) len: usize,
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️group-history-visibility/🦀️.rs"]
mod group_history_visibility_tests;
//#endregion 🧩️GroupHistoryVisibility

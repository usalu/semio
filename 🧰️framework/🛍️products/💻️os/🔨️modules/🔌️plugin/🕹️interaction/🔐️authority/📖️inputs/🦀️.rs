//! 🔒️ Exact immutable document/config read ownership retained alongside a local interaction query.

use std::mem::ManuallyDrop;
use store::{ArtifactStoreOneItemGrant, SnapshotRead, SnapshotRetirementStep};

//#region 🔒️FrozenInputRoots
struct InputReadState<D, C> {
    document: Option<SnapshotRead<D>>,
    config: Option<SnapshotRead<C>>,
    closing: bool,
}

/// 🔒️ Frozen input roots remain retained until each Store registry ACCEPTS its return. Acceptance
/// is the whole contract: reclaiming the accepted slot back into an owned-value retirement is the
/// Store's own one-slot-per-step maintenance cursor, whose arrival is bounded by the registry's
/// fixed capacity and therefore must never gate a live query's terminal reply.
pub(crate) struct LocalInteractionInputReads<D, C> {
    owned: ManuallyDrop<InputReadState<D, C>>,
}

impl<D, C> LocalInteractionInputReads<D, C> {
    /// 🧯️ Failed capture still retains every successfully issued lease until exact registry return.
    pub(crate) fn from_optional(document: Option<SnapshotRead<D>>, config: Option<SnapshotRead<C>>) -> Self {
        Self { owned: ManuallyDrop::new(InputReadState { document, config, closing: false }) }
    }

    pub(crate) fn begin_close(&mut self) {
        self.owned.closing = true;
    }

    pub(crate) fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {
        if self.terminal_is_empty() {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if !self.owned.closing || grant.maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if let Some(read) = self.owned.document.take() {
            if !read.return_to_registry() {
                return Err("local-interaction.document-read-return".into());
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(read) = self.owned.config.take() {
            if !read.return_to_registry() {
                return Err("local-interaction.config-read-return".into());
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.owned.closing && self.owned.document.is_none() && self.owned.config.is_none()
    }
}

impl<D, C> Drop for LocalInteractionInputReads<D, C> {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            if !std::thread::panicking() {
                panic!("local interaction input reads dropped before both exact Store returns completed");
            }
            return;
        }
        unsafe {
            ManuallyDrop::drop(&mut self.owned);
        }
    }
}
//#endregion 🔒️FrozenInputRoots

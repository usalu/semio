//! 🔒️ Exact immutable document/config read ownership retained alongside a local interaction query.

use std::mem::ManuallyDrop;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use store::SnapshotRead;

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

    pub(crate) fn retirement_demands(&self) -> RetirementDemand {
        RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() }
    }

    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(idle));
        }
        if grant.maximum_depth < self.retirement_demands().depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "local interaction input return exceeds admitted depth"));
        }
        if !self.owned.closing || grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(idle));
        }
        if let Some(read) = self.owned.document.take() {
            if !read.return_to_registry() {
                return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "local-interaction.artifact-read-return"));
            }
        } else if let Some(read) = self.owned.config.take() {
            if !read.return_to_registry() {
                return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "local-interaction.config-read-return"));
            }
        } else {
            return Ok(RetainedCloneStep::Complete(idle));
        }
        let progress = RetainedCloneProgress { copied_items: 1, ..idle };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
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

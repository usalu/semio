//! 📨️ A whole patch remains bound to its private typed retirement cursor through handback.

use crate::{UiPatch, UiTypedRetirementCursor, UiValueRetirementStep};
use std::mem::ManuallyDrop;

//#region 📨️WholePatchOwner
#[derive(Default)]
pub struct UiPendingPatch {
    patch: ManuallyDrop<Option<UiPatch>>,
    retirement: UiTypedRetirementCursor,
    closing: bool,
}

impl UiPendingPatch {
    /// 🎟️ The producer admits the fixed owner and inline patch move before filling this source.
    pub fn source_mut(&mut self) -> Result<&mut Option<UiPatch>, &'static str> {
        if self.closing {
            return Err("pending whole patch is closing");
        }
        Ok(&mut self.patch)
    }
    pub fn get(&self) -> Option<&UiPatch> {
        if self.closing {
            None
        } else {
            self.patch.as_ref()
        }
    }

    /// 📏️ Exact operation-page backing only; inline ownership and nested payloads are separate credits.
    pub fn retained_operation_bytes(&self) -> usize {
        self.patch.as_ref().map_or(0, |patch| patch.ops.allocated_bytes())
    }

    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if items == 0 || bytes == 0 {
            return Ok(Default::default());
        }
        self.closing = true;
        self.retirement.advance(&mut *self.patch, items, bytes)
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.patch.is_none() && (!self.closing || self.retirement.terminal_is_empty())
    }

    /// 🧾️ Quotes the same next typed descendant before its physical allocation can be released.
    pub fn original_retirement_demand(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        if self.terminal_is_empty(){return Ok(Default::default())}
        let metadata=size_of::<UiTypedRetirementCursor>()+size_of::<bool>();
        let body=copy.saturating_sub(metadata);
        let release=self.retirement.original_release_demand(&*self.patch).map_err(|error|ValueError::literal(ValueRefusalKind::InvariantViolated,error))?;
        Ok(RetirementDemand{copy_bytes:metadata.checked_add(body).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original patch retirement quote overflows"))?,release_bytes:release,depth:<Option<UiPatch> as crate::UiTypedRetire>::DEPTH,..Default::default()})
    }

    /// ♻️ Advances only the already-quoted original patch and returns its actual logical and physical work.
    pub fn close_original_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
        let mut work=0;
        let result=(||{
            if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
            let demand=self.original_retirement_demand(grant.maximum_copy_bytes)?;
            let metadata=size_of::<UiTypedRetirementCursor>()+size_of::<bool>();
            if grant.maximum_items==0||grant.maximum_copy_bytes<=metadata||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
            let step=self.close_step(1,grant.maximum_copy_bytes-metadata).map_err(|error|ValueError::literal(ValueRefusalKind::InvariantViolated,error))?;
            work=u64::from(step.progressed);
            let progress=if step.progressed{RetainedCloneProgress{copied_items:1,copied_bytes:metadata+step.released_bytes,released_bytes:demand.release_bytes,..Default::default()}}else{Default::default()};
            Ok(if step.complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
        })();
        RetainedWorkOutcome{result,work}
    }
}

impl std::fmt::Debug for UiPendingPatch {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("UiPendingPatch").field("owned", &self.patch.is_some()).field("closing", &self.closing).finish()
    }
}

impl Drop for UiPendingPatch {
    fn drop(&mut self) {
        if !self.terminal_is_empty() && !std::thread::panicking() {
            panic!("whole patch requires exact typed retirement");
        }
    }
}
//#endregion 📨️WholePatchOwner

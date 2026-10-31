//! 🧪️ One erased test owner preserves authentic refusal and terminal-ticket integration fixtures.
use crate::retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress};
pub(super) struct RefusingCopyDemand { pub(super) owner: Option<u64> }

impl crate::ErasedSnapshotRetirement for RefusingCopyDemand {
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"copy demand has no authority"))}
    fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{if grant.maximum_items==0{return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let items=usize::from(self.owner.take().is_some());crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:items,..Default::default()})),work:items as u64}}
    fn terminal_is_empty(&self)->bool{self.owner.is_none()}
    fn next_copy_byte_demand(&self)->Result<usize,crate::ValueError>{Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"copy demand has no authority"))}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,crate::ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(0)}
    fn next_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(0)}
}

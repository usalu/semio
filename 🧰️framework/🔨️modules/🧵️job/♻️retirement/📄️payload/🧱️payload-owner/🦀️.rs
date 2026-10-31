//! ♻️ Original payload and writer closure: the owners of the payload crate retire their own pages, ledger and staged backing.
use super::*;
use crate::{RetirementDemand,RetainedWorkOutcome};

fn parent(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"job payload parent depth overflow"))?;Ok(demand)}
pub fn ledger_demands(ledger:&Option<Arc<JobPayloadOperationLedger>>)->RetirementDemand{if ledger.is_none(){return Default::default()}RetirementDemand{release_bytes:semio_framework_value_retirement::retirement::allocation_return::shared_allocation_bytes::<JobPayloadOperationLedger>(),depth:1,..Default::default()}}
pub fn close_ledger(ledger:&mut Option<Arc<JobPayloadOperationLedger>>,grant:RetainedCloneGrant)->RetainedWorkOutcome{
 if ledger.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}
 let demand=ledger_demands(ledger);if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
 let original=ledger.take().unwrap();let released_bytes=if let Some(original)=Arc::into_inner(original){drop(original);demand.release_bytes}else{0};
 RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()})),work:1}
}

impl RetainedJobPayload {
    pub fn next_close_byte_demand(&self)->usize{self.retirement_demands().expect("original job payload demand").release_bytes}
    /// 📏️ Borrows the exact original page or separately retained operation ledger before effects.
    pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{if self.page_count>0{Ok(RetirementDemand{release_bytes:self.live_pages().last().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original payload page count lacks backing"))?.source.allocated_capacity_bytes(),depth:if self.ledger.is_some(){3}else{1},..Default::default()})}else{Ok(ledger_demands(&self.ledger))}}
    /// 🎟️ Releases one original page or ledger with the original caller's complete authority.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
        if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}
        let demand=match self.retirement_demands(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
        if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
        if self.page_count==0{return close_ledger(&mut self.ledger,grant);}
        let page=self.pop_page().unwrap();let extent=page.source.allocated_capacity_bytes();if let Some(ledger)=self.ledger.as_ref(){ledger.release(self.stream,extent);}drop(page);
        RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:extent,..Default::default()})),work:1}
    }
}


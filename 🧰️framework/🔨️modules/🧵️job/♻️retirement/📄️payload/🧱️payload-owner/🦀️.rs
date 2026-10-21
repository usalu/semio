//! ♻️ Original payload and writer closure: the owners of the payload crate retire their own pages, ledger and staged backing.
use super::*;
use semio_framework_value::{RetirementDemand,RetirementTurnError,advance_retirement_turn};

fn turn(demand:RetirementDemand,grant:RetainedCloneGrant,owner:impl FnOnce(RetainedCloneGrant)->Result<(RetainedCloneStep,bool),ValueError>)->Result<RetainedCloneStep,ValueError>{advance_retirement_turn(demand,grant,owner).map_err(|error|match error{RetirementTurnError::Owner(error)|RetirementTurnError::Receipt(error)=>error})}
fn parent(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"job payload parent depth overflow"))?;Ok(demand)}
pub fn ledger_demands(ledger:&Option<Arc<JobPayloadOperationLedger>>)->RetirementDemand{let Some(original)=ledger.as_ref()else{return Default::default()};let _=original;RetirementDemand{copy_bytes:size_of::<Option<Arc<JobPayloadOperationLedger>>>()+2*size_of::<usize>()+size_of::<JobPayloadOperationLedger>(),release_bytes:semio_framework_value::shared_retirement_allocation_bytes::<JobPayloadOperationLedger>(),depth:1,..Default::default()}}
pub fn close_ledger(ledger:&mut Option<Arc<JobPayloadOperationLedger>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=ledger_demands(ledger);turn(demand,grant,|_|{let Some(original)=ledger.take()else{return Ok((RetainedCloneStep::Complete(Default::default()),true));};let released_bytes=if let Some(original)=Arc::into_inner(original){drop(original);semio_framework_value::shared_retirement_allocation_bytes::<JobPayloadOperationLedger>()}else{0};Ok((RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..Default::default()}),true))})}

impl RetainedJobPayload {
    pub fn next_close_byte_demand(&self)->usize{self.retirement_demands().expect("original job payload demand").release_bytes}
    /// 📏️ Borrows the exact original page or separately retained operation ledger before effects.
    pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{if self.page_count>0{Ok(RetirementDemand{copy_bytes:2*size_of::<Option<JobPayloadPage>>()+2*size_of::<usize>()+if self.ledger.is_some(){JobPayloadOperationLedger::reservation_copy_bytes()}else{0},release_bytes:self.pages.get(self.page_count-1).and_then(Option::as_ref).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original payload page count lacks backing"))?.source.allocated_capacity_bytes(),depth:if self.ledger.is_some(){3}else{1},..Default::default()})}else{Ok(ledger_demands(&self.ledger))}}
    /// 🎟️ Releases one original page or ledger with the original caller's complete authority.
    pub fn close_step(&mut self, grant: RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=self.retirement_demands()?;turn(demand,grant,|grant|{
        if self.page_count==0{return close_ledger(&mut self.ledger,grant).map(|step|(step,self.terminal_is_empty()));}
        let index=self.page_count-1;if self.pages[index].is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job page prefix lacks its actual page"))}
        let page=self.pages[index].take().unwrap();let extent=page.source.allocated_capacity_bytes();self.page_count-=1;self.length-=page.length;if let Some(ledger)=self.ledger.as_ref(){ledger.release(self.stream,extent);}drop(page);
        Ok((RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:extent,..Default::default()}),self.terminal_is_empty()))
    })}
}

impl RetainedJobPayloadWriter {
    pub fn next_close_byte_demand(&self)->usize{self.retirement_demands().expect("original job writer demand").release_bytes}
    /// 📏️ Keeps staged page backing, its original ledger and committed payload demands distinct.
    pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{
        if let Some((_,source,_))=self.staged.as_ref(){return Ok(RetirementDemand{release_bytes:source.allocated_capacity_bytes(),depth:1,..Default::default()});}if let Some(source)=self.rejected.as_ref(){return Ok(RetirementDemand{release_bytes:source.allocated_capacity_bytes(),depth:1,..Default::default()});}
        if self.closing_ledger.is_some(){return Ok(ledger_demands(&self.closing_ledger));}
        if let Some(payload)=self.payload.as_ref(){return if payload.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{parent(payload.retirement_demands()?)};}
        Ok(RetirementDemand{depth:usize::from(!self.sealed),..Default::default()})
    }
    /// 🎛️ Advances exactly one physical frontier while retaining all original downstream owners.
    pub fn close_step(&mut self, grant: RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=self.retirement_demands()?;turn(demand,grant,|grant|{
        self.sealed=true;
        if self.staged.is_some(){let stream=self.payload.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"staged original job page lacks its stream owner"))?.stream;let(ledger,source,_)=self.staged.take().unwrap();let extent=source.allocated_capacity_bytes();ledger.release(stream,extent);*self.closing_ledger=Some(ledger);drop(source);return Ok((RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:extent,..Default::default()}),false));}
        if self.rejected.is_some(){let extent=self.rejected.as_ref().unwrap().allocated_capacity_bytes();drop(self.rejected.take());return Ok((RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:extent,..Default::default()}),false));}
        if self.closing_ledger.is_some(){let step=close_ledger(&mut self.closing_ledger,grant)?;return Ok((RetainedCloneStep::Progress(step.progress()),false));}
        if let Some(payload)=self.payload.as_mut(){if !payload.terminal_is_empty(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=payload.close_step(child)?;return Ok((RetainedCloneStep::Progress(step.progress()),false));}drop(self.payload.take());return Ok((RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),true));}
        Ok((RetainedCloneStep::Complete(Default::default()),self.terminal_is_empty()))
    })}
}

//! 🚼️ Factory ticket construction retains each original alias and partial child under caller authority.
use super::*;

pub struct FactoryRetirementAdmissionError {
 pub error:ValueError,
 pub original:Option<Arc<dyn FactoryRetirement>>,
 pub ticket:Option<Box<dyn FactoryRetirementTicket>>,
 pub progress:RetainedCloneProgress,
}
impl std::fmt::Debug for FactoryRetirementAdmissionError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("FactoryRetirementAdmissionError").field("error",&self.error).field("original_address",&self.original.as_ref().map(|source|Arc::as_ptr(source)as*const()as usize)).field("has_ticket",&self.ticket.is_some()).field("progress",&self.progress).finish()}}
pub trait FactoryRetirementTicket:ErasedSnapshotRetirement {
 fn preparation_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn prepare_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome;
 fn preparation_is_complete(&self)->bool;
}
pub struct FactoryChildSlot {
 pub original:ManuallyDrop<Option<Arc<dyn FactoryRetirement>>>,
 pub ticket:ManuallyDrop<Option<Box<dyn FactoryRetirementTicket>>>,
}
impl FactoryChildSlot {
 pub const fn empty()->Self{Self{original:ManuallyDrop::new(None),ticket:ManuallyDrop::new(None)}}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.ticket.is_none()}
 pub fn preparation_is_complete(&self)->bool{self.original.is_none()&&self.ticket.as_ref().is_some_and(|ticket|ticket.preparation_is_complete())}
 pub fn preparation_demands(&self,source:&dyn FactoryRetirement,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(ticket)=self.ticket.as_ref(){return ticket.preparation_demands(body);}
  if let Some(original)=self.original.as_ref(){return Ok(RetirementDemand{copy_bytes:original.factory_retirement_copy_byte_demand(),capacity_bytes:original.factory_retirement_birth_bytes(),depth:original.factory_retirement_depth_demand(),..Default::default()});}
  let _=source;Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})
 }
 pub fn prepare_step(&mut self,source:&dyn FactoryRetirement,capture:impl FnOnce()->Arc<dyn FactoryRetirement>,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if self.preparation_is_complete(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  let demand=match self.preparation_demands(source,grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !permits(demand,grant){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if let Some(ticket)=self.ticket.as_mut(){return ticket.prepare_step(grant)}
  if self.original.is_none(){*self.original=Some(capture());return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}}
  let turn=match self.original.take(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("original absent").into()),work:0}}.preborn_factory_retirement(grant);let result=match turn.result{Ok((ticket,progress))=>{*self.ticket=Some(ticket);Ok(RetainedCloneStep::Progress(progress))},Err(error)=>{*self.original=error.original;*self.ticket=error.ticket;Err(error.error.with_retained_progress(error.progress))}};RetainedWorkOutcome{result,work:turn.work}
 }
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(original)=self.original.as_ref(){return self.preparation_demands(original.as_ref(),body);}self.ticket.as_ref().map_or(Ok(Default::default()),|ticket|factory_ticket_demands(ticket,body))}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if self.original.is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original child requires funded parent preparation before native close")),work:0}}
  close_factory_ticket(&mut self.ticket,grant)
 }

}
impl Drop for FactoryChildSlot{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"factory child slot abandoned its original alias or ticket");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.ticket);}}}}
pub(super) fn permits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

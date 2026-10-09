//! 🚼️ Factory ticket construction retains each original alias and partial child under caller authority.
use super::*;
use std::mem::size_of;

pub struct FactoryRetirementAdmissionError {
 pub error:ValueError,
 pub original:Option<Arc<dyn FactoryRetirement>>,
 pub ticket:Option<Box<dyn FactoryRetirementTicket>>,
 pub progress:RetainedCloneProgress,
}
impl std::fmt::Debug for FactoryRetirementAdmissionError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("FactoryRetirementAdmissionError").field("error",&self.error).field("original_address",&self.original.as_ref().map(|source|Arc::as_ptr(source)as*const()as usize)).field("has_ticket",&self.ticket.is_some()).field("progress",&self.progress).finish()}}
pub trait FactoryRetirementTicket:ErasedSnapshotRetirement {
 fn preparation_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn prepare_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
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
  if let Some(original)=self.original.as_ref(){return Ok(RetirementDemand{copy_bytes:original.factory_retirement_copy_byte_demand()+size_of::<Option<Arc<dyn FactoryRetirement>>>()+size_of::<Option<Box<dyn FactoryRetirementTicket>>>(),capacity_bytes:original.factory_retirement_birth_bytes(),depth:original.factory_retirement_depth_demand(),..Default::default()});}
  let _=source;Ok(RetirementDemand{copy_bytes:size_of::<Arc<dyn FactoryRetirement>>()+2*size_of::<usize>()+size_of::<Option<Arc<dyn FactoryRetirement>>>(),depth:1,..Default::default()})
 }
 pub fn prepare_step(&mut self,source:&dyn FactoryRetirement,capture:impl FnOnce()->Arc<dyn FactoryRetirement>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.preparation_is_complete(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let d=self.preparation_demands(source,grant.maximum_copy_bytes)?;if !permits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(ticket)=self.ticket.as_mut(){return ticket.prepare_step(grant);}
  if self.original.is_none(){*self.original=Some(capture());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
  let header=size_of::<Option<Arc<dyn FactoryRetirement>>>()+size_of::<Option<Box<dyn FactoryRetirementTicket>>>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-header,..grant};
  match self.original.take().unwrap().preborn_factory_retirement(child){
   Ok((ticket,p))=>{*self.ticket=Some(ticket);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:p.copied_bytes+header,..p}))},
   Err(e)=>{*self.original=e.original;*self.ticket=e.ticket;Err(e.error.with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:e.progress.copied_bytes+header,..e.progress}))}
  }
 }
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(original)=self.original.as_ref(){return self.preparation_demands(original.as_ref(),body);}self.ticket.as_ref().map_or(Ok(Default::default()),|ticket|factory_ticket_demands(ticket,body))}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.original.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original child requires funded parent preparation before native close"));}
  close_factory_ticket(&mut self.ticket,grant)
 }
}
impl Drop for FactoryChildSlot{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"factory child slot abandoned its original alias or ticket");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.ticket);}}}}
pub(super) fn permits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

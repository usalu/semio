//! 🚼️ Factory admission keeps the original source and every producer reply in resident custody.
use super::*;
type AdmissionTurn=RetainedWorkOutcome<RetainedCloneProgress>;
/// 🧾️ Only the actual borrowed producer can deposit its ticket and recorded reply.
pub struct FactoryProducerReply<'reply,O:?Sized,E>{output:&'reply mut Option<Box<O>>,reply:&'reply mut Option<RetainedWorkOutcome<RetainedCloneProgress,E>>,attempted:&'reply mut Option<u128>}
/// 🔏️ The reply borrow remains exclusive until the producer invocation ends.
pub struct FactoryProducerReplyComplete<'reply,E>{_reply:&'reply mut Option<RetainedWorkOutcome<RetainedCloneProgress,E>>}
impl<'reply,O:?Sized,E> FactoryProducerReply<'reply,O,E>{
 pub fn borrow(output:&'reply mut Option<Box<O>>,reply:&'reply mut Option<RetainedWorkOutcome<RetainedCloneProgress,E>>,attempted:&'reply mut Option<u128>)->Result<Self,ValueError>{if reply.is_some(){return Err(ValueError::invariant("factory reply is already recorded"))}*attempted=None;Ok(Self{output,reply,attempted})}
 pub fn has_output(&self)->bool{self.output.is_some()}
 pub fn publish(&mut self,output:Box<O>){assert!(self.output.is_none(),"factory original output slot occupied");*self.output=Some(output)}
 pub fn performed(&mut self,effect:&mut RetainedAllocationEffect<'_>,work:u64)->Result<bool,ValueError>{let admitted=effect.performed(work)?;*self.attempted=Some(self.attempted.unwrap_or(0).checked_add(u128::from(work)).ok_or_else(||ValueError::invariant("factory producer invocation work overflow"))?);Ok(admitted)}
 pub fn complete(self,turn:RetainedWorkOutcome<RetainedCloneProgress,E>)->FactoryProducerReplyComplete<'reply,E>{assert!(self.reply.is_none(),"factory producer reply already recorded");assert_eq!(self.attempted.unwrap_or(0),turn.work,"factory reply changed performed work");*self.reply=Some(turn);FactoryProducerReplyComplete{_reply:self.reply}}
 pub(crate) fn complete_observed(self,effect:&RetainedAllocationEffect<'_>,boundary:OriginalWorkPerformedBoundary,result:Result<RetainedCloneProgress,E>)->FactoryProducerReplyComplete<'reply,E>{let work=effect.performed_since(boundary).expect("factory producer retains its original work receipt");*self.attempted=Some(work);self.complete(RetainedWorkOutcome{result,work})}
 pub fn pending(self)->FactoryProducerReplyComplete<'reply,E>{FactoryProducerReplyComplete{_reply:self.reply}}
}
pub trait FactoryRetirementTicket:ErasedSnapshotRetirement{
 fn preparation_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn preparation_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn prepare_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll;
 fn preparation_is_complete(&self)->bool;
 fn original_slot(&mut self)->Option<&mut dyn std::any::Any>;
}
#[derive(Clone,Copy,PartialEq,Eq)]
enum AdmissionPhase{Empty,Started,Prepared,Failed}
struct Admission{phase:AdmissionPhase,demand:RetirementDemand,maximum_work:u64,reply:ManuallyDrop<Option<AdmissionTurn>>,attempted:Option<u128>}
impl Admission{
 const fn empty()->Self{Self{phase:AdmissionPhase::Empty,demand:RetirementDemand{copy_bytes:0,capacity_bytes:0,release_bytes:0,depth:0},maximum_work:0,reply:ManuallyDrop::new(None),attempted:None}}

 fn terminal_is_empty(&self)->bool{self.phase==AdmissionPhase::Empty&&self.reply.is_none()}
}
/// 🪆️ A private original source slot cannot be replaced during a pending producer action.
pub struct FactoryChildSlot{original:ManuallyDrop<Option<Arc<dyn FactoryRetirement>>>,ticket:ManuallyDrop<Option<Box<dyn FactoryRetirementTicket>>>,admission:Admission}
impl FactoryChildSlot{
 pub const fn empty()->Self{Self{original:ManuallyDrop::new(None),ticket:ManuallyDrop::new(None),admission:Admission::empty()}}
 pub fn new(original:Arc<dyn FactoryRetirement>)->Self{Self{original:ManuallyDrop::new(Some(original)),ticket:ManuallyDrop::new(None),admission:Admission::empty()}}
 pub fn original(&self)->Option<&Arc<dyn FactoryRetirement>>{self.original.as_ref()}
 pub fn has_ticket(&self)->bool{self.ticket.is_some()}
 pub fn admission_is_pending(&self)->bool{self.admission.phase==AdmissionPhase::Started}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.ticket.is_none()&&self.admission.terminal_is_empty()}
 pub fn preparation_is_complete(&self)->bool{self.original.is_none()&&self.ticket.as_ref().is_some_and(|ticket|ticket.preparation_is_complete())}
 fn transfer_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Arc<dyn FactoryRetirement>>>(),depth:1,..Default::default()}}
 fn capture_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Arc<dyn FactoryRetirement>>(),depth:1,..Default::default()}}
 fn source_demand(source:&dyn FactoryRetirement)->RetirementDemand{RetirementDemand{copy_bytes:source.factory_retirement_copy_byte_demand(),capacity_bytes:source.factory_retirement_birth_bytes(),depth:source.factory_retirement_depth_demand(),..Default::default()}}
 fn admission_demands(&self)->Result<RetirementDemand,ValueError>{match self.admission.phase{AdmissionPhase::Prepared=>Ok(Self::transfer_demand()),AdmissionPhase::Started=>Ok(self.admission.demand),AdmissionPhase::Failed=>self.ticket.as_ref().map_or(Ok(Default::default()),|ticket|factory_ticket_demands(ticket,0)),AdmissionPhase::Empty=>self.original.as_ref().map_or(Ok(Self::capture_demand()),|source|Ok(Self::source_demand(source.as_ref())))}}
 pub fn preparation_work_demand(&self,_source:&dyn FactoryRetirement,body:usize)->Result<u64,ValueError>{if self.preparation_is_complete(){return Ok(0)}if self.original.is_none(){if let Some(ticket)=self.ticket.as_ref(){return ticket.preparation_work_demand(body)}return Ok(1)}self.next_computational_work(body)}
 pub fn preparation_demands(&self,_source:&dyn FactoryRetirement,body:usize)->Result<RetirementDemand,ValueError>{if self.original.is_none(){if let Some(ticket)=self.ticket.as_ref(){return ticket.preparation_demands(body)}}self.admission_demands()}
 fn admit(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll{let work_start=effect.performed_work();
  let grant=effect.grant();let demand=match self.admission_demands(){Ok(demand)=>demand,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:0})};
  if !permits(demand,grant){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  if self.admission.phase==AdmissionPhase::Failed{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  if self.admission.phase==AdmissionPhase::Prepared{
   if effect.maximum_work_units()<1{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
   let Some(ticket)=self.ticket.as_mut()else{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("factory prepared reply lost its original ticket")),work:0})};
   let Some(source)=self.original.take()else{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("factory transfer lost its original source")),work:0})};
   let work=match crate::retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>{*self.original=Some(source);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work})}};
   if let Err(original)=source.transfer_factory_original(ticket.as_mut()){*self.original=Some(original);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("factory prepared ticket has a foreign typed source slot")),work})}
   self.admission=Admission::empty();return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work})
  }
  let Some(source)=self.original.as_ref()else{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("factory admission lost its original source")),work:0})};
  if self.admission.phase==AdmissionPhase::Empty{
   if self.ticket.is_some(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("factory admission ticket slot is occupied")),work:0})}
   let work=match source.factory_retirement_work_demand(){Ok(work)=>work,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:0})};
   if work>effect.maximum_work_units(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
   self.admission.phase=AdmissionPhase::Started;self.admission.demand=demand;self.admission.maximum_work=work;
   source.prepare_factory_retirement(effect,FactoryProducerReply::borrow(&mut self.ticket,&mut self.admission.reply,&mut self.admission.attempted).expect("unrecorded original factory reply"));
  }else if self.admission.reply.is_none(){source.recover_factory_retirement(effect,FactoryProducerReply{output:&mut self.ticket,reply:&mut self.admission.reply,attempted:&mut self.admission.attempted});}
  let work=effect.performed_work()-work_start;let Some(turn)=self.admission.reply.take()else{return RetainedWorkPoll::Pending{work}};
  self.admission.phase=if self.ticket.is_some(){if turn.result.is_ok(){AdmissionPhase::Prepared}else{AdmissionPhase::Failed}}else{AdmissionPhase::Empty};
  let result=turn.result.map(RetainedCloneStep::Progress);if self.admission.phase==AdmissionPhase::Empty{self.admission=Admission::empty()}RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})
 }
 pub fn prepare_step(&mut self,source:&dyn FactoryRetirement,capture:impl FnOnce()->Arc<dyn FactoryRetirement>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll{
  if self.preparation_is_complete(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
  if self.original.is_none(){if let Some(ticket)=self.ticket.as_mut(){return ticket.prepare_step(effect)}
   if !permits(Self::capture_demand(),effect.grant())||effect.maximum_work_units()<1{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
   let _=source;let work=match crate::retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work})};*self.original=Some(capture());return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:Self::capture_demand().copy_bytes,..Default::default()})),work})
  }self.admit(effect)
 }
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.original.is_some(){return self.admission_demands()}self.ticket.as_ref().map_or(Ok(Default::default()),|ticket|factory_ticket_demands(ticket,body))}
 pub fn requires_original_birth(&self)->bool{(self.original.is_none()||self.admission.phase==AdmissionPhase::Failed)&&self.ticket.as_ref().is_some_and(|ticket|ticket.requires_original_birth())}
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{if let Some(ticket)=self.ticket.as_mut(){if ticket.requires_original_birth(){return ticket.close_original_birth_step(original,grant)}}RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("factory child has no selected original birth").into()),work:0})}
 pub fn requires_original_control(&self)->bool{(self.original.is_none()||self.admission.phase==AdmissionPhase::Failed)&&self.ticket.as_ref().is_some_and(|ticket|ticket.requires_original_control())}
 pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.original.is_some(){return match self.admission.phase{AdmissionPhase::Prepared=>Ok(1),AdmissionPhase::Started=>Ok(self.admission.maximum_work),AdmissionPhase::Failed=>self.ticket.as_ref().map_or(Ok(0),|ticket|if ticket.terminal_is_empty(){Ok(1)}else{ticket.next_computational_work(body)}),AdmissionPhase::Empty=>self.original.as_ref().unwrap().factory_retirement_work_demand()}}match self.ticket.as_ref(){Some(ticket)if ticket.terminal_is_empty()=>Ok(1),Some(ticket)=>ticket.next_computational_work(body),None=>Ok(0)}}
 pub fn close_step_controlled(&mut self,effect:&mut RetainedAllocationEffect<'_>,control:&mut dyn OriginalRetirementControl)->OriginalRetirementPoll{if self.original.is_some()&&self.admission.phase!=AdmissionPhase::Failed{return self.close_step(effect)}let turn=semio_framework_value_retirement::close_factory_ticket_controlled(&mut self.ticket,effect,control);self.closed_turn(turn)}
 pub fn close_original_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{if self.original.is_some()&&self.admission.phase!=AdmissionPhase::Failed{return self.close_step(effect)}let turn=semio_framework_value_retirement::close_factory_ticket_original(&mut self.ticket,original,effect);self.closed_turn(turn)}
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  if self.original.is_some()&&self.admission.phase!=AdmissionPhase::Failed{return self.admit(effect).map_err(Into::into)}
  let turn=close_factory_ticket(&mut self.ticket,effect);self.closed_turn(turn)
 }
 fn closed_turn(&mut self,poll:OriginalRetirementPoll)->OriginalRetirementPoll{match poll{RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>{if self.ticket.is_none()&&self.admission.phase==AdmissionPhase::Failed{self.admission=Admission::empty()}RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|if self.terminal_is_empty(){step}else{RetainedCloneStep::Progress(step.progress())}),work:turn.work})}}}

}
impl Drop for FactoryChildSlot{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"factory child abandoned original source, ticket or reply");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.ticket);ManuallyDrop::drop(&mut self.admission.reply)}}}}
pub(super) fn permits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

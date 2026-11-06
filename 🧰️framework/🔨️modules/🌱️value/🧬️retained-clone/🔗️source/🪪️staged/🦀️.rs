//! 🪪️ Stages one original source admission so a cancelled caller pays its input back instead of leaking it.
use super::*;
use crate::{RetainedWorkOutcome,RetirementDemand,ValueError,retirement::{RetirementCursor,RetirementStep,controlled::ControlledRetirement}};
use std::mem::size_of;
/// 🎟️ Retains one in-flight source admission until it publishes its source or pays its input back.
pub struct StagedSourceAdmission<T:Sync+'static,A:RetireOwned+Sync>{admission:Option<RetainedCloneSourceAdmission<T,A>>}
impl<T:Sync+'static,A:RetireOwned+Sync> StagedSourceAdmission<T,A>{
 /// 🫴️ Stages a source that projects its typed view out of the borrowed original.
 pub fn borrowed(original:A,project:for<'a> fn(&'a A)->&'a T)->Self{Self{admission:Some(RetainedCloneSource::admission_borrowed(original,project))}}
 /// 📏️ Quotes the next admission action, or the empty header once published.
 pub fn next_demand(&self)->Result<RetirementDemand,ValueError>{self.admission.as_ref().map_or_else(||Ok(Default::default()),RetainedCloneSourceAdmission::next_demand)}
 /// ✍️ Advances one admission action, then moves the ready source into the caller's place.
 pub fn advance(&mut self,output:&mut Option<RetainedCloneSource<T,A>>,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome{
  if actual.is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("staged source requires an empty allocation marker")),work:0}}
  let Some(admission)=self.admission.as_mut()else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
  let turn=if admission.phase()=="ready"{admission.take_output(output,grant)}else{admission.advance(grant,actual)};
  if admission.terminal_is_empty(){self.admission=None;}turn
 }
 /// 📏️ Quotes the next paid return action of an admission that is cancelled.
 pub fn close_demand(&self)->Result<RetirementDemand,ValueError>{match self.admission.as_ref(){None=>Ok(Default::default()),Some(admission)=>if admission.phase()=="ready"{Ok(RetirementDemand{depth:1,..Default::default()})}else{admission.next_return_demand()}}}
 /// ↩️ Pays one cancelled action: a ready source moves to the caller's source place, otherwise the original input returns to the caller's input place.
 pub fn close_into(&mut self,input:&mut Option<A>,source:&mut Option<RetainedCloneSource<T,A>>,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let Some(admission)=self.admission.as_mut()else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
  let turn=if admission.phase()=="ready"{admission.take_output(source,grant)}else{let mut returned=None;let turn=admission.return_inputs(&mut returned,grant);if let Some((original,()))=returned{*input=Some(original);}turn};
  if admission.terminal_is_empty(){self.admission=None;}turn
 }
 pub fn terminal_is_empty(&self)->bool{self.admission.is_none()}
}
impl<A:RetireOwned+Sync> StagedSourceAdmission<A,A>{
 /// 🧺️ Stages a source that is its own typed view.
 pub fn owned(original:A)->Self{Self{admission:Some(RetainedCloneSource::admission_owned(original,()))}}
}
struct AdmissionRetirement<T:Sync+'static,A:RetireOwned+Sync>{admission:Option<RetainedCloneSourceAdmission<T,A>>,published:Option<ControlledRetirement<RetainedCloneSource<T,A>>>,returned:Option<ControlledRetirement<A>>}
impl<T:Sync+'static,A:RetireOwned+Sync+Send> RetirementCursor for AdmissionRetirement<T,A>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(published)=self.published.as_ref(){return published.selected_demand(body)}if let Some(returned)=self.returned.as_ref(){return returned.selected_demand(body)}if self.admission.as_ref().is_some_and(|admission|admission.phase()=="ready"){return Ok(RetirementDemand{depth:1,..Default::default()})}self.admission.as_ref().map_or_else(||Ok(Default::default()),RetainedCloneSourceAdmission::next_return_demand)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.next_demand(body)?.copy_bytes)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  let(result,work)=if let Some(published)=self.published.as_mut(){let result=published.step(grant);let work=published.last_turn_work();if published.terminal_is_empty(){self.published=None;}(result.map(|step|step.progress()),work)}
  else if let Some(admission)=self.admission.as_mut().filter(|admission|admission.phase()=="ready"){let mut source=None;let turn=admission.take_output(&mut source,grant);if let Some(source)=source{self.published=Some(ControlledRetirement::retain(source));}if admission.terminal_is_empty(){self.admission=None;}(turn.result.map(|step|step.progress()).map_err(Into::into),turn.work)}
  else if let Some(returned)=self.returned.as_mut(){let result=returned.step(grant);let work=returned.last_turn_work();if returned.terminal_is_empty(){self.returned=None;}(result.map(|step|step.progress()),work)}
  else if let Some(admission)=self.admission.as_mut(){let mut input=None;let turn=admission.return_inputs(&mut input,grant);if let Some((original,()))=input{self.returned=Some(ControlledRetirement::retain(original));}if admission.terminal_is_empty(){self.admission=None;}(turn.result.map(|step|step.progress()).map_err(Into::into),turn.work)}
  else{return RetirementStep::Complete(0)};
  match result{Err(error)=>RetirementStep::Failure(error,work),Ok(progress)=>if self.terminal_is_empty()&&progress==RetainedCloneProgress::default(){RetirementStep::Complete(work)}else{RetirementStep::Progress(progress,work)}}
 }
 fn terminal_is_empty(&self)->bool{self.admission.is_none()&&self.published.is_none()&&self.returned.is_none()}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|demand|demand.release_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl<T:Sync+'static,A:RetireOwned+Sync+Send> RetireOwned for StagedSourceAdmission<T,A>{
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{admission}=self;Box::new(AdmissionRetirement{admission,published:None,returned:None})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<AdmissionRetirement<T,A>>())}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{size_of::<Self>()}
}

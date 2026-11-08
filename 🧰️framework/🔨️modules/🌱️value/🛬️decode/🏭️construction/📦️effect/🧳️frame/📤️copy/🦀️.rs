//! \u{1f4e4}\uFE0F The original Copy action stays borrowed until its paid Source Posts settle.
use crate::NativeReceivingConstructionBinding;
use semio_framework_value_control_contract::*;
use semio_framework_value_native_outcome::OriginalConstructionAcceptedProof;
use std::mem::{size_of,ManuallyDrop};

/// \u{1f9f3}\uFE0F Exclusively retains the original producer, action and destination across paid turns.
pub struct NativeResidentCopyLoan<'a,P:OriginalEffectProducer<Output=RetainedCloneStep>>{
 authority:&'a mut OriginalEffectAuthority<P>,
 action:&'a mut Option<OriginalEffectAction<RetainedCloneStep,P::Failure>>,
 output:&'a mut Option<RetainedCloneStep>,
 copied:Option<NativeReceivingConstructionBinding>,
 cleared:Option<NativeReceivingConstructionBinding>,
 copy_accepted:bool,
 clear_accepted:bool,
 clear_reply:ManuallyDrop<Option<Result<RetainedCloneStep,ValueError>>>,
 failure:ManuallyDrop<Option<ValueError>>,
}
impl<'a,P:OriginalEffectProducer<Output=RetainedCloneStep>> NativeResidentCopyLoan<'a,P>{
 /// \u{1f331}\uFE0F Freezes the actual original owner and its empty destination before Source entry.
 pub fn new(authority:&'a mut OriginalEffectAuthority<P>,action:&'a mut Option<OriginalEffectAction<RetainedCloneStep,P::Failure>>,output:&'a mut Option<RetainedCloneStep>)->Result<Self,ValueError>{
  if output.is_some(){return Err(ValueError::invariant("resident copy destination occupied"))}
  Ok(Self{authority,action,output,copied:None,cleared:None,copy_accepted:false,clear_accepted:false,clear_reply:ManuallyDrop::new(None),failure:ManuallyDrop::new(None)})
 }
 /// \u{1f4d0}\uFE0F Quotes the original success and the actual resident binding.
 pub fn copy_demand()->Result<RetirementDemand,ValueError>{let mut demand=OriginalEffectAuthority::<P>::copy_success_demand()?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<Option<NativeReceivingConstructionBinding>>()).ok_or_else(||ValueError::invariant("resident copy extent overflow"))?;Ok(demand)}
 /// \u{1f9fe}\uFE0F Borrows the original retained infrastructure refusal.
 pub fn failure(&self)->Option<&ValueError>{self.failure.as_ref().or_else(||self.clear_reply.as_ref().and_then(|reply|reply.as_ref().err()))}
 /// \u{1f4e4}\uFE0F Copies only the recorded success into this same borrowed destination.
 pub fn copy_into<'r,'turn,'source>(&mut self,context:&P::Context<'turn,'source>,binding:NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError> where 'source:'turn{
  if self.copied.is_some()||self.cleared.is_some()||self.failure().is_some()||!binding.matches_effect(effect){return Err(ValueError::invariant("resident copy action changed"))}
  let demand=Self::copy_demand()?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()))))}
  let Some(action)=self.action.as_mut()else{return Err(ValueError::invariant("resident original action absent"))};
  let turn=self.authority.copy_success_into(context,action,self.output,effect);
  match turn.result{Ok(Some(mut progress))=>{progress.copied_bytes=progress.copied_bytes.checked_add(size_of::<Option<NativeReceivingConstructionBinding>>()).ok_or_else(||ValueError::invariant("resident copy progress overflow"))?;self.copied=Some(binding);Ok(reply.complete(Ok(RetainedCloneStep::Complete(progress))))},Ok(None)=>Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())))),Err(error)=>{*self.failure=Some(error);Ok(reply.pending())}}
 }
 /// \u{1f6c2}\uFE0F Checks the actual accepted Copy Post against this retained action.
 pub fn accept_copy(&mut self,proof:&OriginalConstructionAcceptedProof<'_>)->Result<(),ValueError>{let binding=self.copied.ok_or_else(||ValueError::invariant("resident copy has no Source action"))?;if self.copy_accepted||proof.action_identity()!=binding.action()||proof.work_binding()!=binding.work()||proof.work_action()!=binding.work_action(){return Err(ValueError::invariant("resident copy Post changed"))}self.copy_accepted=true;Ok(())}
 /// \u{1f9ec}\uFE0F Derives the original action proof from the same accepted completion and Work issuer.
 pub fn success_proof<'turn,'source>(&self,context:&P::Context<'turn,'source>,loan:&OriginalConstructionLoan<'_>,completion:&OriginalConstructionCompletion,work:&OriginalWorkBorrow<'_>,proof:&OriginalWorkActionProof)->Result<OriginalEffectSuccessProof,ValueError> where 'source:'turn{if !self.copy_accepted{return Err(ValueError::invariant("resident copy Post is not accepted"))}self.authority.success_proof(context,self.action.as_ref().ok_or_else(||ValueError::invariant("resident original action absent"))?,loan,completion,work,proof)}
 /// \u{1f4cf}\uFE0F Quotes the exact original action and private reply headers for clearance.
 pub fn clear_demand()->Result<RetirementDemand,ValueError>{let mut demand=OriginalEffectAuthority::<P>::finish_success_demand::<ValueError>()?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<Option<NativeReceivingConstructionBinding>>()).and_then(|bytes|bytes.checked_add(size_of::<Option<Result<RetainedCloneStep,ValueError>>>())).ok_or_else(||ValueError::invariant("resident clear extent overflow"))?;Ok(demand)}
 /// \u{1f9f9}\uFE0F Clears the original action before forwarding the recorded native reply.
 pub fn clear_into<'r,'turn,'source>(&mut self,context:&P::Context<'turn,'source>,proof:&OriginalEffectSuccessProof,binding:NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError> where 'source:'turn{
  if !self.copy_accepted||self.cleared.is_some()||self.clear_reply.is_some()||self.failure().is_some()||!binding.matches_effect(effect){return Err(ValueError::invariant("resident clear action changed"))}
  let demand=Self::clear_demand()?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()))))}
  {let child=OriginalEffectReply::borrow(&mut self.clear_reply)?;drop(self.authority.finish_success_into(context,self.action,proof,effect,child)?)}
  match self.clear_reply.as_ref(){Some(Ok(RetainedCloneStep::Complete(progress)))=>{let mut progress=*progress;progress.copied_bytes=progress.copied_bytes.checked_add(size_of::<Option<NativeReceivingConstructionBinding>>()+size_of::<Option<Result<RetainedCloneStep,ValueError>>>()).ok_or_else(||ValueError::invariant("resident clear progress overflow"))?;self.cleared=Some(binding);Ok(reply.complete(Ok(RetainedCloneStep::Complete(progress))))},Some(Ok(RetainedCloneStep::Progress(_)))=>{*self.clear_reply=None;Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()))))},Some(Err(_))|None=>Ok(reply.pending())}
 }
 /// \u{1f6c2}\uFE0F Checks the actual accepted clear Post before releasing the loan.
 pub fn accept_clear(&mut self,proof:&OriginalConstructionAcceptedProof<'_>)->Result<(),ValueError>{let binding=self.cleared.ok_or_else(||ValueError::invariant("resident clear has no Source action"))?;if self.clear_accepted||proof.action_identity()!=binding.action()||proof.work_binding()!=binding.work()||proof.work_action()!=binding.work_action(){return Err(ValueError::invariant("resident clear Post changed"))}self.clear_accepted=true;Ok(())}
 /// \u{1faf4}\uFE0F Returns exclusive custody only after the original accepted clear Post.
 pub fn finish(self)->Result<(),Self>{if !self.clear_accepted||self.failure().is_some()||self.action.is_some(){return Err(self)}Ok(())}
}
impl<P:OriginalEffectProducer<Output=RetainedCloneStep>> Drop for NativeResidentCopyLoan<'_,P>{fn drop(&mut self){assert!(std::thread::panicking()||self.clear_accepted&&self.failure().is_none()&&self.action.is_none(),"resident copy retains its original action or reply")}}

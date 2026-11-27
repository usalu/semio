//! 👶️ One child identity is originally issued in separate paid parent turns and lends only actual supplier inputs.
use super::*;

/// 🪢️ See `README.md`.
pub struct OriginalCallerChildSource{admission:OriginalCallerSourceAdmission,authority_actual:Option<usize>,upstream:ManuallyDrop<Option<OriginalCallerSourceLoan>>,cancel:ManuallyDrop<Option<CancelToken>>,cancel_retiring:ManuallyDrop<Option<CancelTokenRetirement>>,initial_child_ledger:ManuallyDrop<Option<JobPayloadAuthority>>,parent:ManuallyDrop<Option<OriginalCallerSourceParent>>,operation:OperationId,generation:Generation,origin:OriginalSourceBinding,parent_identity:usize,ready:bool,closing:bool}
impl OriginalCallerChildSource{
 #[cfg(any(test,feature="testkit"))]
 pub(crate) fn test_status(&self)->(bool,bool,usize,OriginalSourceBinding){(self.ready,self.closing,self.parent_identity,self.origin)}

 /// 🌱️ See `README.md`.
 pub fn new(parent:&OriginalCallerSourceParent,cx:&StepContext<'_>,operation:OperationId,generation:Generation)->Result<Self,ValueError>{if operation.0==0||generation.0==0{return Err(refused("original child source requires its declared nonzero business identity"))}parent.validate_borrowed_context(cx)?;Ok(Self{admission:OriginalCallerSourceAdmission::new(),authority_actual:None,upstream:ManuallyDrop::new(None),cancel:ManuallyDrop::new(None),cancel_retiring:ManuallyDrop::new(None),initial_child_ledger:ManuallyDrop::new(None),parent:ManuallyDrop::new(None),operation,generation,origin:OriginalSourceBinding::borrow(cx),parent_identity:parent.source_identity()?,ready:false,closing:false})}

 fn validate_parent(&self,parent:&OriginalCallerSourceParent,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{
  if !self.origin.matches(cx)||parent.source_identity()?!=self.parent_identity{return Err(refused("original child source requires its same still-retained initiating parent").into());}
  let witness=parent.child_witness(cx)?;
  let Some(upstream)=self.upstream.as_ref()else{if self.terminal_is_empty(){return Ok(())}return Err(refused("original child source has already returned its parent loan").into())};
  if self.closing{if upstream.terminal_is_empty(){return Ok(())}return upstream.validate_retirement_context(cx).map_err(OriginalTurnFailure::from)}
  let original=upstream.retained_identity().ok_or_else(||refused("original child source has no upstream strong witness"))?;
  if witness!=original{return Err(refused("original child source requires its same still-retained initiating parent").into());}
  if self.closing{upstream.validate_retirement_context(cx)?;}else{upstream.validate_context(cx)?;}Ok(())
 }
 pub fn admit_step(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate_parent(parent,cx)?;if self.closing{return Err(refused("closing original child source cannot admit another phase").into())}if self.ready{return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}if self.upstream.is_none(){parent.admit_loan(&mut self.upstream,cx)?;return Ok(false)}if self.parent.is_some(){if !metadata_fits(cx){return Ok(false)}self.ready=true;crate::receive_original_retained_work(cx,RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(metadata())),work:1})?;return Ok(true)}let unborn=self.admission.terminal_is_empty();if unborn&&self.cancel.is_none(){cx.admit_original_cancel_alias(&mut self.cancel)?;return Ok(false)}if unborn&&self.initial_child_ledger.is_none(){let turn=JobPayloadAuthority::admit(self.operation,self.generation,cx.retained_grant(),&mut self.authority_actual);let result=match turn.result{Ok(Some((ledger,progress)))=>{*self.initial_child_ledger=Some(ledger);Ok(RetainedCloneStep::Progress(progress))},Ok(None)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>Err(error)};let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};if self.authority_actual.take().is_some_and(|actual|actual!=progress.retained_capacity_bytes){return Err(refused("original child authority attempt differs from its receipt").into())}crate::receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;return Ok(false)}let grant=cx.retained_grant();let Some((turn,_))=cx.with_original_effect(grant,|_,effect|if self.admission.has_backing(){self.admission.publish(&mut self.parent,effect)}else if self.admission.has_roots(){self.admission.admit_backing(effect)}else{self.admission.prepare(&mut self.initial_child_ledger,&mut self.cancel,effect)})?else{return Ok(false)};if let Some(child)=self.parent.as_mut(){child.lend_borrowed_cancel()}crate::receive_original_retained_work(cx,turn)?;Ok(false)}
 pub fn parent(&self)->Result<&OriginalCallerSourceParent,ValueError>{if !self.ready||self.closing{return Err(refused("original child source must finish its declared paid admission before lending"))}self.parent.as_ref().ok_or_else(||refused("original child parent is absent"))}
 pub fn with_context<T>(&self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>,requested:RetainedCloneGrant,run:impl for<'turn> FnOnce(&mut StepContext<'turn>)->Result<T,OriginalTurnFailure>)->Result<OriginalChildTurnReturn<T>,OriginalTurnFailure>{
  self.validate_parent(parent,cx)?;if !self.ready&&!self.closing{return Err(refused("original child callback cannot precede its paid source admission").into())}let child_parent=self.parent.as_ref().ok_or_else(||refused("original child callback has no live child source parent"))?;let(result,receipt_refusal,before,after)=cx.lend_child_turn(child_parent,requested,run)?;Ok(OriginalChildTurnReturn{original:ManuallyDrop::new(Some((result,receipt_refusal))),before,after})
 }
 pub fn terminal_is_empty(&self)->bool{self.admission.terminal_is_empty()&&self.upstream.is_none()&&self.cancel.is_none()&&self.cancel_retiring.is_none()&&self.initial_child_ledger.is_none()&&self.parent.is_none()}
 pub fn close_step(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()});}
  self.validate_parent(parent,cx)?;
  if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Blocked);}
  let metadata_demand=RetirementDemand{depth:1,..Default::default()};
  let pending=|step|match step{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},InteractiveJobCloseStep::Blocked=>InteractiveJobCloseStep::Blocked};
  if !self.closing{return crate::close_original_retained_step(cx,metadata_demand,|_|{self.closing=true;self.ready=false;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(metadata())),work:1}});}
  if !self.admission.terminal_is_empty(){
   let(admission,ledger,cancel)=(&mut self.admission,&mut self.initial_child_ledger,&mut self.cancel);
   if admission.has_backing(){let step=crate::close_original_retained_effect_step(cx,OriginalCallerSourceAdmission::publish_demand(),|effect|admission.publish(&mut self.parent,effect).result).map(pending);if let Some(child)=self.parent.as_mut(){child.lend_borrowed_cancel()}return step}
   return crate::close_original_retained_effect_step(cx,OriginalCallerSourceAdmission::prepare_demand(),|effect|admission.return_inputs(ledger,cancel,effect).result).map(pending);
  }
  if let Some(child)=self.parent.as_mut(){
   if child.terminal_is_empty(){return crate::close_original_retained_step(cx,metadata_demand,|_|{self.parent.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(metadata())),work:1}});}
   let demand=child.retirement_demands()?;let witness=parent.cancellation()?;
   return crate::close_original_effect_step(cx,demand,|effect|child.close_witnessed(effect,Some(witness))).map(pending);
  }
  if let Some(ledger)=self.initial_child_ledger.as_mut(){
   if ledger.terminal_is_empty(){return crate::close_original_retained_step(cx,metadata_demand,|_|{self.initial_child_ledger.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(metadata())),work:1}});}
   let demand=ledger.retirement_demands();return crate::close_original_effect_step(cx,demand,|effect|semio_framework_value_control_contract::RetainedWorkPoll::Complete(ledger.close_step(effect))).map(pending);
  }
  if self.cancel.is_some(){return crate::close_original_retained_step(cx,metadata_demand,|_|{*self.cancel_retiring=Some(CancelTokenRetirement::from_token(self.cancel.take().expect("cancel absent")));RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(metadata())),work:1}});}
  if let Some(retiring)=self.cancel_retiring.as_mut(){
   if retiring.terminal_is_empty(){return crate::close_original_retained_step(cx,metadata_demand,|_|{self.cancel_retiring.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(metadata())),work:1}});}
   let witness=parent.cancellation()?;return crate::close_original_retained_effect_step(cx,metadata_demand,|effect|retiring.return_alias_step(witness,effect).result.map_err(cancel_refusal)).map(pending);
  }
  let upstream=self.upstream.as_mut().ok_or_else(||ValueError::invariant("upstream absent"))?;
  if upstream.terminal_is_empty(){return crate::close_original_retained_step(cx,metadata_demand,|_|{self.upstream.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(metadata())),work:1}});}
  upstream.close_step(parent,cx).map(pending)
 }
}
impl Drop for OriginalCallerChildSource{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original child source reached Drop before all original births and upstream source loan returned")}}

/// 🧾️ See `README.md`.
#[derive(Debug)]
pub struct OriginalChildTurnReturn<T>{original:ManuallyDrop<Option<(Result<T,OriginalTurnFailure>,Option<ValueError>)>>,before:RetainedCloneProgress,after:RetainedCloneProgress}
impl<T> OriginalChildTurnReturn<T>{pub fn raw_progress(&self)->Result<RetainedCloneProgress,ValueError>{delta(self.before,self.after)}pub fn original_receiving(&self)->(RetainedCloneProgress,RetainedCloneProgress){(self.before,self.after)}pub fn admitted(&self)->Option<&Result<T,OriginalTurnFailure>>{let(original,refusal)=self.original.as_ref()?;refusal.is_none().then_some(original)}pub fn retained_original(&self)->Option<&Result<T,OriginalTurnFailure>>{self.original.as_ref().map(|(original,_)|original)}pub fn receipt_refusal(&self)->Option<&ValueError>{self.original.as_ref().and_then(|(_,refusal)|refusal.as_ref())}pub fn into_original(mut self)->Result<Result<T,OriginalTurnFailure>,Self>{if self.receipt_refusal().is_some(){return Err(self)}Ok(self.original.take().expect("original child return extracted once").0)}pub fn terminal_is_empty(&self)->bool{self.original.is_none()}}
impl<T:RetireOwned> RetireOwned for OriginalChildTurnReturn<T>{type RetirementCursor=<Option<(Result<T,OriginalTurnFailure>,Option<ValueError>)> as RetireOwned>::RetirementCursor;fn retirement_seed(mut self)->Self::RetirementCursor{self.original.take().retirement_seed()}fn retirement_birth_bytes(&self)->Option<usize>{self.original.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
impl<T> Drop for OriginalChildTurnReturn<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original child semantic return reached Drop before admitted extraction or whole granted retirement")}}
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}


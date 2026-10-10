//! 🎟️ Original actor input and paid authorities remain in place across admitted turns.
use semio_framework_actor::RetainedTurnInput;
use semio_framework_async::{CancelToken,CancelTokenRetirement,CancelTokenRetirementError};
use semio_framework_job::{Generation,JobPayloadAuthority,OperationId,StepBudget,StepContext};
use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use std::mem::ManuallyDrop;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalActorAdmission{pub ready:bool,pub progress:RetainedCloneProgress}
#[derive(Debug)]
pub enum OriginalActorContextError{Refused(ValueError),Cancellation(CancelTokenRetirementError)}
/// 🧳️ Owns the same input until dispatch and borrows the same admitted root and payload ledger.
pub struct OriginalActorContext<T>{input:ManuallyDrop<Option<T>>,identity:Option<(OperationId,Generation)>,epoch:u64,cancel:ManuallyDrop<Option<CancelToken>>,payload:ManuallyDrop<Option<JobPayloadAuthority>>,cancel_close:ManuallyDrop<Option<CancelTokenRetirement>>,sequence:u64,ready:bool,closing:bool}
impl<T> OriginalActorContext<T>{
 pub fn new(input:T,retained:RetainedTurnInput)->Result<Self,(ValueError,T)>{if let Err(error)=retained.validate(){return Err((error,input))}Ok(Self{input:ManuallyDrop::new(Some(input)),identity:Some((OperationId(retained.operation),Generation(retained.generation))),epoch:0,cancel:ManuallyDrop::new(None),payload:ManuallyDrop::new(None),cancel_close:ManuallyDrop::new(None),sequence:0,ready:false,closing:false})}
 pub fn input(&self)->Option<&T>{self.input.as_ref()}
 fn validate(&self,retained:RetainedTurnInput)->Result<(),ValueError>{retained.validate()?;if self.identity!=Some((OperationId(retained.operation),Generation(retained.generation))){return Err(refusal("original actor context identity differs from supplied input"))}Ok(())}
 fn begin(&mut self,retained:RetainedTurnInput)->Result<(),ValueError>{self.validate(retained)?;if retained.epoch<=self.epoch{return Err(refusal("original actor context turn epoch was already used"))}self.epoch=retained.epoch;Ok(())}
 /// 🌱️ One caller-funded root or ledger birth yields before any semantic producer.
 pub fn admit_step(&mut self,retained:RetainedTurnInput)->Result<OriginalActorAdmission,ValueError>{self.begin(retained)?;if self.closing{return Err(refusal("original actor context is closing"))}if self.cancel.is_none(){let Some((original,progress))=CancelToken::admit_root(retained.grant)?else{return Ok(OriginalActorAdmission{ready:false,progress:Default::default()})};*self.cancel=Some(original);return Ok(OriginalActorAdmission{ready:false,progress})}if self.payload.is_none(){let(operation,generation)=self.identity.unwrap();let Some((original,progress))=JobPayloadAuthority::admit(operation,generation,retained.grant)?else{return Ok(OriginalActorAdmission{ready:false,progress:Default::default()})};*self.payload=Some(original);return Ok(OriginalActorAdmission{ready:false,progress})}self.ready=true;Ok(OriginalActorAdmission{ready:true,progress:Default::default()})}
 /// 🤝️ Uses the original incoming full grant and caller's external receipt storage.
 pub fn borrow_context<'a>(&'a mut self,retained:RetainedTurnInput,budget:StepBudget,recipient:&'a mut RetainedCloneProgress,clock:fn()->Option<u64>)->Result<StepContext<'a>,ValueError>{self.validate(retained)?;if !self.ready||self.closing||retained.epoch!=self.epoch||budget.retained!=retained.grant{return Err(refusal("original actor context loan differs from admitted turn"))}let(operation,generation)=self.identity.unwrap();let cancel=self.cancel.as_ref().ok_or_else(||refusal("original actor cancellation has not been admitted"))?;let payload=self.payload.as_ref().ok_or_else(||refusal("original actor payload authority has not been admitted"))?;StepContext::with_payload_authority(operation,generation,budget,cancel,clock,&mut self.sequence,recipient,payload)}
 /// ▶️ A dispatched turn borrows already admitted authorities under its next original epoch.
 pub fn begin_dispatch(&mut self,retained:RetainedTurnInput)->Result<(),ValueError>{if !self.ready||self.closing{return Err(refusal("original actor context has not reached paid readiness"))}self.begin(retained)}
 /// 📬️ Moves the original input once into its semantic consumer after both births yielded.
 pub fn take_input(&mut self)->Result<Option<T>,ValueError>{if !self.ready||self.closing||self.cancel.is_none()||self.payload.is_none(){return Err(refusal("original actor input cannot dispatch before paid authorities"))}Ok(self.input.take())}
 pub fn terminal_is_empty(&self)->bool{self.input.is_none()&&self.identity.is_none()&&self.cancel.is_none()&&self.payload.is_none()&&self.cancel_close.is_none()}
 /// 📏️ Quotes one original physical frontier or one separate enclosing metadata removal.
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{if self.input.is_some(){return Err(refusal("original actor input still requires its semantic owner"))}if let Some(payload)=self.payload.as_ref(){if !payload.terminal_is_empty(){return Ok(payload.retirement_demands())}return Ok(RetirementDemand{depth:1,..Default::default()})}if self.cancel.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}if let Some(cancel)=self.cancel_close.as_ref(){if !cancel.terminal_is_empty(){return cancel.retirement_demands()}return Ok(RetirementDemand{depth:1,..Default::default()})}Ok(RetirementDemand{depth:usize::from(self.identity.is_some()),..Default::default()})}
 /// ♻️ Each original authority closes before its separate inline removal turn.
 pub fn close_step(&mut self,retained:RetainedTurnInput)->Result<RetainedCloneStep,OriginalActorContextError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}self.begin(retained).map_err(OriginalActorContextError::Refused)?;let demand=self.retirement_demands().map_err(OriginalActorContextError::Refused)?;self.closing=true;let grant=retained.grant;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(payload)=self.payload.as_mut(){if !payload.terminal_is_empty(){return payload.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress())).map_err(OriginalActorContextError::Refused)}drop(self.payload.take());return Ok(metadata(false))}if let Some(cancel)=self.cancel.take(){*self.cancel_close=Some(CancelTokenRetirement::from_token(cancel));return Ok(metadata(false))}if let Some(cancel)=self.cancel_close.as_mut(){if !cancel.terminal_is_empty(){return cancel.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress())).map_err(OriginalActorContextError::Cancellation)}drop(self.cancel_close.take());return Ok(metadata(false))}self.identity=None;Ok(metadata(true))}
}
fn metadata(complete:bool)->RetainedCloneStep{let progress=RetainedCloneProgress{copied_items:1,..Default::default()};if complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
impl<T> Drop for OriginalActorContext<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original actor context reached Drop before paid authority retirement");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

/// 🧾️ Physical effects remain attached to the original input on semantic success or failure.
pub struct OriginalActorCall<T,E>{pub input:RetainedTurnInput,pub receipt_spent:RetainedCloneProgress,pub result:Result<T,E>}
impl<T,E> OriginalActorCall<T,E>{pub fn receipt(&self)->Result<semio_framework_actor::RetainedTurnReceipt,ValueError>{self.input.return_original(self.receipt_spent)}}
#[derive(Debug)]
pub enum OriginalActorCallError<E>{Authority(ValueError),Semantic(E)}
/// 🏛️ Holds one persistent guest context until its original authorities and binding have paid closure.
pub struct OriginalActorContextSlot{context:ManuallyDrop<Option<OriginalActorContext<()>>>,binding:Option<(u64,u64,u64)>}
impl OriginalActorContextSlot{
 pub const fn empty()->Self{Self{context:ManuallyDrop::new(None),binding:None}}
 fn validate_binding(&self,input:RetainedTurnInput)->Result<(),ValueError>{input.validate()?;if let Some((operation,generation,epoch))=self.binding{if input.operation!=operation||input.generation!=generation||input.epoch<=epoch{return Err(refusal("original guest binding differs from original identity or repeats an issued epoch"))}}Ok(())}
 pub fn admit_step(&mut self,input:RetainedTurnInput)->OriginalActorCall<OriginalActorAdmission,ValueError>{let result=(||{self.validate_binding(input)?;if self.context.is_none(){if input.grant.maximum_items==0||input.grant.maximum_depth==0{return Ok(OriginalActorAdmission{ready:false,progress:Default::default()})}let mut original=OriginalActorContext::new((),input).map_err(|(error,())|error)?;original.begin(input)?;*self.context=Some(original);self.binding=Some((input.operation,input.generation,input.epoch));return Ok(OriginalActorAdmission{ready:false,progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}let result=self.context.as_mut().unwrap().admit_step(input);if self.context.as_ref().unwrap().epoch==input.epoch{self.binding=Some((input.operation,input.generation,input.epoch))}result})();let receipt_spent=match &result{Ok(admission)=>admission.progress,Err(error)=>error.retained_progress()};OriginalActorCall{input,receipt_spent,result}}
 /// 🤝️ The semantic producer receives the exact caller recipient and both original paid authorities.
 pub fn dispatch_step<T,E>(&mut self,input:RetainedTurnInput,budget:StepBudget,recipient:&mut RetainedCloneProgress,clock:fn()->Option<u64>,produce:impl FnOnce(&mut StepContext<'_>)->Result<T,E>)->OriginalActorCall<T,OriginalActorCallError<E>>{let result=(||{self.validate_binding(input).map_err(OriginalActorCallError::Authority)?;let original=self.context.as_mut().ok_or_else(||OriginalActorCallError::Authority(refusal("original guest context has not been admitted")))?;if budget.retained!=input.grant{return Err(OriginalActorCallError::Authority(refusal("original guest body budget differs from supplied input")))}original.begin_dispatch(input).map_err(OriginalActorCallError::Authority)?;self.binding=Some((input.operation,input.generation,input.epoch));let mut cx=original.borrow_context(input,budget,recipient,clock).map_err(OriginalActorCallError::Authority)?;produce(&mut cx).map_err(OriginalActorCallError::Semantic)})();OriginalActorCall{input,receipt_spent:*recipient,result}}
 /// ♻️ Empty input removal, physical authorities and enclosing context binding each yield separately.
 pub fn close_step(&mut self,input:RetainedTurnInput)->OriginalActorCall<RetainedCloneStep,OriginalActorContextError>{let result=(||{self.validate_binding(input).map_err(OriginalActorContextError::Refused)?;let Some(original)=self.context.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()))};if original.input().is_some(){original.begin(input).map_err(OriginalActorContextError::Refused)?;self.binding=Some((input.operation,input.generation,input.epoch));if input.grant.maximum_items==0||input.grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}original.take_input().map_err(OriginalActorContextError::Refused)?;return Ok(metadata(false))}if !original.terminal_is_empty(){let result=original.close_step(input);if original.epoch==input.epoch{self.binding=Some((input.operation,input.generation,input.epoch))}return result}self.binding=Some((input.operation,input.generation,input.epoch));if input.grant.maximum_items==0||input.grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}drop(self.context.take());self.binding=None;Ok(metadata(true))})();let receipt_spent=match &result{Ok(step)=>step.progress(),Err(OriginalActorContextError::Refused(error))|Err(OriginalActorContextError::Cancellation(CancelTokenRetirementError::Refused(error)))=>error.retained_progress(),Err(OriginalActorContextError::Cancellation(CancelTokenRetirementError::Blocked(_)))=>Default::default()};OriginalActorCall{input,receipt_spent,result}}
 pub fn terminal_is_empty(&self)->bool{self.context.is_none()&&self.binding.is_none()}
}
impl Drop for OriginalActorContextSlot{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original guest context slot reached Drop before funded context closure");}}

/// 📥️ Carries the exact original empty admission receipt without transporting refusal prose.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
pub struct OriginalActorAdmissionReply{pub input:RetainedTurnInput,pub spent:RetainedCloneProgress,pub ready:bool,pub refusal:Option<ValueRefusalKind>}
impl OriginalActorAdmissionReply{
 pub fn borrow_original(call:&OriginalActorCall<OriginalActorAdmission,ValueError>)->Self{Self{input:call.input,spent:call.receipt_spent,ready:call.result.as_ref().is_ok_and(|original|original.ready),refusal:call.result.as_ref().err().map(|original|original.kind)}}
}
/// ♻️ Carries raw original closure effects even when a child is blocked or refuses receipt validation.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
pub struct OriginalActorCloseReply{pub input:RetainedTurnInput,pub spent:RetainedCloneProgress,pub complete:bool,pub blocked:bool,pub refusal:Option<ValueRefusalKind>}
impl OriginalActorCloseReply{
 pub fn borrow_original(call:&OriginalActorCall<RetainedCloneStep,OriginalActorContextError>)->Self{Self{input:call.input,spent:call.receipt_spent,complete:matches!(&call.result,Ok(RetainedCloneStep::Complete(_))),blocked:matches!(&call.result,Err(OriginalActorContextError::Cancellation(CancelTokenRetirementError::Blocked(_)))),refusal:match &call.result{Err(OriginalActorContextError::Refused(original))|Err(OriginalActorContextError::Cancellation(CancelTokenRetirementError::Refused(original)))=>Some(original.kind),_=>None}}}
}

#[path="🌉️call/🦀️.rs"]
mod guest_entry;
pub use guest_entry::{OriginalActorEntryFailure,OriginalActorGuestEntry,OriginalActorFixedCall,OriginalActorFixedKind,OriginalActorFixedReceipt};

#[cfg(all(any(feature="component-guest",feature="component-extension-guest"),target_arch="wasm32",target_env="p2"))]
#[path="🌉️call/🎭️exports/🦀️.rs"]
pub mod exports;

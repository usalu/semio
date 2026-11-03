//! ⏸️ The original Actor issuer moves with its finite asynchronous turn and receives its whole reply before polling.
use super::{OriginalActorInput,ActorGuestPhase,RetainedTurnInput,RetainedTurnReceipt,ValueError,refusal};
use semio_framework_job::{OriginalCallerSourceParent,OriginalOwnedCallerTurn,OriginalOwnedCallerReturn,OriginalOwnedCallerOutcome,OriginalTurnFailure,StepContext,RetainedCloneProgress,InteractiveJobCloseStep};
use std::mem::ManuallyDrop;

/// 🪢️ Independent source and issuer refusals retain their complete original causes.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct OriginalActorParkAdmissionFailure{pub caller:Option<OriginalTurnFailure>,pub issuer:Option<ValueError>}
impl From<ValueError> for OriginalActorParkAdmissionFailure{fn from(issuer:ValueError)->Self{Self{caller:None,issuer:Some(issuer)}}}
/// 🧳️ Owns the actual admitted Actor input and its outstanding finite source crossing throughout suspension.
pub struct OriginalActorParkedTurn<T>{issuer:ManuallyDrop<Option<OriginalActorInput<T>>>,turn:ManuallyDrop<Option<OriginalOwnedCallerTurn>>,input:Option<RetainedTurnInput>,admission_failure:ManuallyDrop<Option<ValueError>>}
/// 🎟️ The actual issuer and every partially admitted source remain together before asynchronous entry.
pub struct OriginalActorParkAdmission<T>{issuer:ManuallyDrop<Option<OriginalActorInput<T>>>,turn:ManuallyDrop<Option<OriginalOwnedCallerTurn>>}
impl<T> OriginalActorInput<T>{
 /// 🧳️ Moves the existing issuer into caller-owned pending custody without creating a new source.
 pub fn park_owned_job_turn(self)->OriginalActorParkAdmission<T>{OriginalActorParkAdmission{issuer:ManuallyDrop::new(Some(self)),turn:ManuallyDrop::new(None)}}
}
impl<T> OriginalActorParkAdmission<T>{
 pub fn issuer(&self)->Option<&OriginalActorInput<T>>{self.issuer.as_ref()}
 pub fn terminal_is_empty(&self)->bool{self.issuer.is_none()&&self.turn.is_none()}
 /// ▶️ Receives every actual source delta in its genuine issuer before returning a ready owned turn.
 pub fn advance(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<Option<OriginalActorParkedTurn<T>>,OriginalActorParkAdmissionFailure>{
  parent.validate_borrowed_context(cx)?;let issuer=self.issuer.as_mut().ok_or_else(||refusal("owned Actor admission already transferred its issuer"))?;
  let valid=issuer.issued_input().is_some_and(|input|(input.operation,input.generation)==(cx.operation().0,cx.generation().0)&&Some(input.epoch)==cx.original_source_epoch());
  if !valid||issuer.phase()!=ActorGuestPhase::Ready{return Err(refusal("owned asynchronous Actor turn requires its actual ready issuer and original epoch").into())}
  if cx.should_yield()||cx.is_cancelled()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(None)}
  let Some(admission)=issuer.checkout_guest(cx.retained_grant())?else{return Ok(None)};
  let before=cx.retained_progress();let before_work=cx.original_work_units_spent();let parked=parent.park_owned_turn(&mut self.turn,cx);let after=cx.retained_progress();let spent=RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes};
  let received=issuer.receive_guest_spent(admission,spent,u64::try_from(cx.original_work_units_spent()-before_work).unwrap_or(u64::MAX),ActorGuestPhase::Ready);
  let ready=match parked{Ok(ready)=>ready,Err(caller)=>return Err(OriginalActorParkAdmissionFailure{caller:Some(caller),issuer:received.err()})};
  if !ready{received?;return Ok(None)}
  let mut admission_failure=received.err();let Some(turn)=self.turn.as_ref()else{return Err(OriginalActorParkAdmissionFailure{caller:None,issuer:admission_failure})};let input=if admission_failure.is_none(){match issuer.checkout_guest(turn.original_grant()){Ok(Some(input))=>Some(input),Ok(None)=>{admission_failure=Some(refusal("funded original parked Actor body has no receiving grant"));None},Err(error)=>{admission_failure=Some(error);None}}}else{None};
  Ok(Some(OriginalActorParkedTurn{issuer:ManuallyDrop::new(self.issuer.take()),turn:ManuallyDrop::new(self.turn.take()),input,admission_failure:ManuallyDrop::new(admission_failure)}))
 }
 /// ♻️ A failed or cancelled admission retains its actual issuer while the same source loan closes.
 pub fn close_step(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  parent.validate_borrowed_context(cx)?;let Some(turn)=self.turn.as_mut()else{return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})};
  if !turn.admission_is_empty(){return turn.close_admission_step(parent,cx)}
  let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
  self.turn.take();let progress=RetainedCloneProgress{copied_items:1,..Default::default()};semio_framework_job::receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome::<_,ValueError>{result:Ok(semio_framework_value::RetainedCloneStep::Progress(progress)),work:1})?;Ok(InteractiveJobCloseStep::Complete{progress})
 }
 /// 📤️ Returns the same issuer through an explicit caller-owned output after all partial source custody closes.
 pub fn take_issuer(&mut self,output:&mut Option<OriginalActorInput<T>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.turn.is_some()||output.is_some(){return Err(refusal("partial parked issuer stays retained until source closure and an empty receiver").into())}if self.issuer.is_none(){return Ok(true)}let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}*output=self.issuer.take();semio_framework_job::receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome::<_,ValueError>{result:Ok(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1})?;Ok(true)
 }
}
impl<T> Drop for OriginalActorParkAdmission<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned Actor admission dropped its issuer or partial original source")}}
impl<T> OriginalActorParkedTurn<T>{
 /// 🤝️ The genuine issuer stays owned while its producer checks out and receives in the same parked context.
 pub async fn execute<R>(mut self,phase:ActorGuestPhase,run:impl for<'turn> AsyncFnOnce(RetainedTurnInput,&mut StepContext<'turn>)->Result<R,OriginalTurnFailure>)->OriginalActorParkedReturn<T,R>{
  let input=self.input;let failure=self.admission_failure.take();let returned=self.turn.take().unwrap().execute(async move|cx|{if let Some(error)=failure{return Err(error.into())}run(input.ok_or_else(||refusal("original parked producer has no actual issuer checkout"))?,cx).await}).await;
  let mut issuer=self.issuer.take().unwrap();let received=match input{Some(input)=>issuer.receive_guest_spent(input,returned.receipt(),u64::try_from(returned.work_units_spent()).unwrap_or(u64::MAX),if returned.receiving_refusal().is_none(){phase}else{ActorGuestPhase::Ready}),None=>Err(refusal("original parked body has no checked out issuer return"))};let receipt=if received.is_ok(){issuer.return_actor_turn()}else{Err(received.err().unwrap())};let(receipt,refusal)=match receipt{Ok(receipt)=>(Some(receipt),None),Err(error)=>(None,Some(error))};
  OriginalActorParkedReturn{issuer:ManuallyDrop::new(Some(issuer)),returned,receipt,issuer_refusal:ManuallyDrop::new(refusal)}
 }
}
impl<T> Drop for OriginalActorParkedTurn<T>{fn drop(&mut self){assert!(std::thread::panicking()||(self.issuer.is_none()&&self.turn.is_none()&&self.admission_failure.is_none()),"original parked Actor turn dropped before its issuing receipt returned")}}
/// 🧾️ Keeps both complete receiving failures beside the semantic reply; neither becomes a reconstructed budget.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct OriginalActorParkedOutcome<R>{pub caller:OriginalOwnedCallerOutcome<R>,pub issuer_refusal:Option<ValueError>}
/// 📬️ The actual original issuer and paid source stay retained with the whole return until caller transfer.
pub struct OriginalActorParkedReturn<T,R>{issuer:ManuallyDrop<Option<OriginalActorInput<T>>>,returned:OriginalOwnedCallerReturn<R>,receipt:Option<RetainedTurnReceipt>,issuer_refusal:ManuallyDrop<Option<ValueError>>}
impl<T,R> OriginalActorParkedReturn<T,R>{
 pub fn caller_return(&self)->&OriginalOwnedCallerReturn<R>{&self.returned}
 pub fn actor_receipt(&self)->Option<RetainedTurnReceipt>{self.receipt}
 pub fn issuer(&self)->Option<&OriginalActorInput<T>>{self.issuer.as_ref()}
 pub fn issuer_refusal(&self)->Option<&ValueError>{self.issuer_refusal.as_ref()}
 /// 📤️ A later genuine caller pays metadata only; original guest work already reached the owned issuing recipient.
 pub fn take_original_on_caller(&mut self,cx:&mut StepContext<'_>)->Result<Option<OriginalActorParkedOutcome<R>>,OriginalTurnFailure>{Ok(self.returned.take_original_on_caller(cx)?.map(|caller|OriginalActorParkedOutcome{caller,issuer_refusal:self.issuer_refusal.take()}))}
 /// 🫴️ Transfers the same issuer rather than recreating its source or receipt from scalar identity fields.
 pub fn take_original_issuer_on_caller(&mut self,cx:&mut StepContext<'_>)->Result<Option<OriginalActorInput<T>>,OriginalTurnFailure>{self.returned.validate_original_receiver(cx)?;if self.returned.original().is_some(){return Err(refusal("original Actor issuer stays retained until its whole semantic return transfers").into())}if self.issuer.is_none(){return Err(refusal("original Actor issuer was already transferred").into())}if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(None)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(self.issuer.take())}
 pub fn close_source(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{if self.issuer.is_some(){return Err(refusal("original Actor source stays retained until its actual issuer transfers").into())}self.returned.close_source(parent,cx)}
 pub fn terminal_is_empty(&self)->bool{self.issuer.is_none()&&self.issuer_refusal.is_none()&&self.returned.terminal_is_empty()}
}
impl<T,R> Drop for OriginalActorParkedReturn<T,R>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Actor reply dropped before actual issuer transfer and paid source return")}}

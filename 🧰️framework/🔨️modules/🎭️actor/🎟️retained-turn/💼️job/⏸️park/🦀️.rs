//! ⏸️ The original Actor issuer moves with its finite asynchronous turn and receives its whole reply before polling.
use super::{OriginalActorInput,ActorGuestPhase,RetainedTurnInput,RetainedTurnReceipt,ValueError,refusal};
use semio_framework_job::{OriginalCallerSourceParent,OriginalOwnedCallerTurn,OriginalOwnedCallerReturn,OriginalOwnedCallerOutcome,OriginalTurnFailure,StepContext,RetainedCloneProgress,InteractiveJobCloseStep};
use std::mem::ManuallyDrop;

/// 🚫️ A denied park retains both the original Actor input and any complete admission failure.
pub struct OriginalActorParkRefusal<T>{pub issuer:OriginalActorInput<T>,pub failure:Option<OriginalTurnFailure>}
/// 🧳️ Owns the actual admitted Actor input and its outstanding finite source crossing throughout suspension.
pub struct OriginalActorParkedTurn<T>{issuer:ManuallyDrop<Option<OriginalActorInput<T>>>,turn:ManuallyDrop<Option<OriginalOwnedCallerTurn>>,input:RetainedTurnInput,admission_failure:ManuallyDrop<Option<ValueError>>}
impl<T> OriginalActorInput<T>{
 /// 🎟️ Transfers an already-ready actual issuer; admission is paid before the sole body checkout.
 pub fn park_owned_job_turn(mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<OriginalActorParkedTurn<T>,OriginalActorParkRefusal<T>>{
  let valid=self.issued_input().is_some_and(|input|(input.operation,input.generation)==(cx.operation().0,cx.generation().0)&&Some(input.epoch)==cx.original_source_epoch());
  if !valid||self.phase()!=ActorGuestPhase::Ready{return Err(OriginalActorParkRefusal{issuer:self,failure:Some(refusal("owned asynchronous Actor turn requires its actual ready issuer and original epoch").into())})}
  if cx.should_yield()||cx.is_cancelled()||cx.fuel_remaining()<2||cx.retained_grant().maximum_items<2||cx.retained_grant().maximum_depth==0{return Err(OriginalActorParkRefusal{issuer:self,failure:None})}
  let admission=match self.checkout_guest(cx.retained_grant()){Ok(Some(input))=>input,Ok(None)=>return Err(OriginalActorParkRefusal{issuer:self,failure:None}),Err(error)=>return Err(OriginalActorParkRefusal{issuer:self,failure:Some(error.into())})};
  if admission.grant!=cx.retained_grant(){let received=self.receive_guest_spent(admission,Default::default(),ActorGuestPhase::Ready);return Err(OriginalActorParkRefusal{issuer:self,failure:Some(received.err().unwrap_or_else(||refusal("owned original Actor wallet differs from the actual issuer remaining grant")).into())})}
  let before=cx.retained_progress();let parked=parent.park_owned_turn(cx);let after=cx.retained_progress();let spent=RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes};
  let received=self.receive_guest_spent(admission,spent,ActorGuestPhase::Ready);
  let turn=match parked{Ok(Some(turn))=>turn,Ok(None)=>return Err(OriginalActorParkRefusal{issuer:self,failure:received.err().map(Into::into)}),Err(mut failure)=>{if let Err(error)=received{failure.context_refusal=Some(error)}return Err(OriginalActorParkRefusal{issuer:self,failure:Some(failure)})}};
  let mut admission_failure=received.err();let input=if admission_failure.is_none(){match self.checkout_guest(turn.original_grant()){Ok(Some(input))=>input,Ok(None)=>{admission_failure=Some(refusal("funded original parked Actor body has no receiving grant"));RetainedTurnInput{grant:turn.original_grant(),..admission}},Err(error)=>{admission_failure=Some(error);RetainedTurnInput{grant:turn.original_grant(),..admission}}}}else{RetainedTurnInput{grant:turn.original_grant(),..admission}};
  Ok(OriginalActorParkedTurn{issuer:ManuallyDrop::new(Some(self)),turn:ManuallyDrop::new(Some(turn)),input,admission_failure:ManuallyDrop::new(admission_failure)})
 }
}
impl<T> OriginalActorParkedTurn<T>{
 /// 🤝️ The genuine issuer stays owned while its producer checks out and receives in the same parked context.
 pub async fn execute<R>(mut self,phase:ActorGuestPhase,run:impl for<'turn> AsyncFnOnce(RetainedTurnInput,&mut StepContext<'turn>)->Result<R,OriginalTurnFailure>)->OriginalActorParkedReturn<T,R>{
  let input=self.input;let failure=self.admission_failure.take();let returned=self.turn.take().unwrap().execute(async move|cx|{if let Some(error)=failure{return Err(error.into())}run(input,cx).await}).await;
  let mut issuer=self.issuer.take().unwrap();let received=issuer.receive_guest_spent(input,returned.receipt(),if returned.receiving_refusal().is_none(){phase}else{ActorGuestPhase::Ready});let receipt=if received.is_ok(){issuer.return_actor_turn()}else{Err(received.err().unwrap())};let(receipt,refusal)=match receipt{Ok(receipt)=>(Some(receipt),None),Err(error)=>(None,Some(error))};
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

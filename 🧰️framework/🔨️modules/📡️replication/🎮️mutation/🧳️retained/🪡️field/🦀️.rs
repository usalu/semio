//! 🪡️ Typed field copying and owned exchange retain only authored and displaced children.
/// 🪢️ Generates the four reply-shaped close entries over fresh or recovered invocations with ordinary or original-loan control.
macro_rules! retained_close{([$($generic:tt)*] $type:ty,$dispatch:ident)=>{
 impl<$($generic)*> $type{
  /// ♻️ Records one close step of the selected child in the caller's reply.
  pub fn close_into<'r>(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:$crate::field::Reply<'r>)->Result<$crate::field::Done<'r>,semio_framework_value::ValueError>{self.$dispatch(None,body,effect,reply,false)}
  /// 🔁️ Recovers the exact selected ordinary close action.
  pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:$crate::field::Reply<'r>)->Result<$crate::field::Done<'r>,semio_framework_value::ValueError>{self.$dispatch(None,body,effect,reply,true)}
  /// 🪢️ Records one close step under the original controller.
  pub fn close_original_into<'r>(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:$crate::field::Reply<'r>)->Result<$crate::field::Done<'r>,semio_framework_value::ValueError>{if self.requires_original_birth(){return Err(semio_framework_value::ValueError::invariant("field close requires its selected original birth"))}if !self.requires_original_control(){return self.$dispatch(None,body,effect,reply,false)}self.$dispatch(Some(original),body,effect,reply,false)}
  /// 🔁️ Recovers the same original-controller close action.
  pub fn recover_close_original_into<'r>(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:$crate::field::Reply<'r>)->Result<$crate::field::Done<'r>,semio_framework_value::ValueError>{if self.flight.loanless(){return self.$dispatch(None,body,effect,reply,true)}self.$dispatch(Some(original),body,effect,reply,true)}
 }
};}
/// ⏱️ Charges one unit of this action or completes the reply with a Work refusal.
macro_rules! charge{($effect:ident,$reply:ident,$units:expr,$message:expr)=>{match $effect.performed($units){Ok(true)=>{},Ok(false)=>return Ok($reply.complete(Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,$message).into()))),Err(error)=>return Ok($reply.complete(Err(error.into())))}}}
#[path="🟰️change/🦀️.rs"]
pub mod change;
#[cfg(test)]
#[path="🧪️tests/harness.rs"]
pub(crate) mod harness;
pub use change::{RetainedFieldChange};
pub use change::record::{RetainedRecordChange, RetainedRecordChangeField, RetainedRecordFieldProjection, RetainedRecordFieldChange, RetainedRecordChangeOwner, boxed_record_field};
use semio_framework_value::{OriginalRetirementFailure, OriginalRetirementOutcome, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, RetirementPollExit, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::RetainedCloneStep;
pub use semio_framework_replication_retained_field_preparation::{RetainedFieldPreparation,RetainedFieldProducer,RetainedFieldProducerContext};


/// ⏱️ Records the work a reported total still owes the shared effect, keeping every refusal.
pub fn owe(effect:&mut RetainedAllocationEffect<'_>,base:u128,work:u128)->Result<(),ValueError>{
    let done=effect.performed_work().saturating_sub(base);
    if done>work{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained field recorded more work than it reports"))}
    let owed=u64::try_from(work-done).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"retained field work does not fit one original unit"))?;
    if owed==0||effect.performed(owed)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"retained field work exceeded its original action"))}
}
/// 🧾️ Settles the owed work and merges any work refusal with the primary failure.
pub fn settle(effect:&mut RetainedAllocationEffect<'_>,base:u128,result:Result<RetainedCloneStep,OriginalRetirementFailure>,work:u128)->OriginalRetirementOutcome{
    let result=match(result,owe(effect,base,work)){
        (result,Ok(()))=>result,
        (Ok(step),Err(refusal))=>Err(OriginalRetirementFailure::from_work_refusal(refusal.with_retained_progress(step.progress()))),
        (Err(primary),Err(refusal))=>Err(OriginalRetirementFailure::from_owned_parts(None,Some(refusal),None,None,Some(Box::new(primary)),None)),
    };
    RetainedWorkOutcome{result,work}
}
/// ⏳️ Settles a close turn and keeps a child Pending separate from genuine failures.
pub fn settle_exit(effect:&mut RetainedAllocationEffect<'_>,base:u128,result:Result<RetainedCloneStep,RetirementPollExit>,work:u128)->OriginalRetirementPoll{
    match result{
        Ok(step)=>RetainedWorkPoll::Complete(settle(effect,base,Ok(step),work)),
        Err(RetirementPollExit::Failed(failure))=>RetainedWorkPoll::Complete(settle(effect,base,Err(failure),work)),
        Err(RetirementPollExit::Pending(child))=>match owe(effect,base,work){
            Ok(())=>RetirementPollExit::finish::<RetainedCloneStep>(Err(RetirementPollExit::Pending(child)),work),
            Err(refusal)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(OriginalRetirementFailure::from_work_refusal(refusal)),work}),
        },
    }
}
/// 🧮️ Narrows an accumulated u128 work total into the remaining u64 quote.
pub fn left(maximum:u64,work:u128)->u64{maximum.saturating_sub(u64::try_from(work).unwrap_or(u64::MAX))}

/// 📏️ Quotes the original row and both resident owner slots.
pub(crate) fn field_install_demand<T>()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{let copy_bytes=std::mem::size_of::<Option<T>>().checked_mul(2).and_then(|n|n.checked_add(std::mem::size_of::<T>())).ok_or_else(||semio_framework_value::ValueError::invariant("field installation extent overflow"))?;Ok(semio_framework_value::RetirementDemand{copy_bytes,depth:1,..Default::default()})}
/// 🔑️ Exchanges the resident authored value under the actual mutation capability and effect.
pub(crate) fn install_field_into<'r,T>(authored:&mut Option<T>,row:&mut T,displaced:&mut Option<T>,_capability:super::ApplyCapability,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:semio_framework_value::retained_clone::OriginalEffectReply<'r,semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>,semio_framework_value::ValueError>{use semio_framework_value::retained_clone::{retained_clone_grant_admits,RetainedCloneProgress,RetainedCloneStep};if authored.is_none()||displaced.is_some(){return Ok(reply.complete(Err(semio_framework_value::ValueError::invariant("field installation slots are invalid"))))}let demand=match field_install_demand::<T>(){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error)))};let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error)))};if remaining<1||!retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}match effect.performed(1){Ok(true)=>{},Ok(false)=>return Ok(reply.complete(Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"field installation exceeded original Work")))),Err(error)=>return Ok(reply.complete(Err(error)))}let authored=authored.take().expect("checked original authored slot");*displaced=Some(std::mem::replace(row,authored));Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}

/// 🧳️ Owns the actual row and both mutable slots across one original installation action.
pub struct RetainedFieldInstallFrame<'a,T>{authored:&'a mut Option<T>,row:&'a mut T,displaced:&'a mut Option<T>,capability:super::ApplyCapability,turn:semio_framework_value::retained_clone::OriginalSourceTurnIdentity}
/// 🧵️ Lends the real original dispatch to the resident installation producer.
pub struct RetainedFieldInstallContext<'a>{turn:semio_framework_value::retained_clone::OriginalSourceTurnWitness<'a>}
impl<'a> RetainedFieldInstallContext<'a>{
 /// 🪪️ Creates a context from the actual retained Source turn.
 pub fn new(turn:semio_framework_value::retained_clone::OriginalSourceTurnWitness<'a>)->Self{Self{turn}}
}
impl<'a,T> RetainedFieldInstallFrame<'a,T>{
 /// 📏️ Quotes publication of the original borrowed request frame.
 pub fn admission_demand()->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Option<semio_framework_value::retained_clone::OriginalEffectAuthority<Self>>>(),depth:1,..Default::default()}}
 /// 🪢️ Admits the real row, slots and capability into the caller resident action owner.
 pub fn admit_into<'r>(authored:&'a mut Option<T>,row:&'a mut T,displaced:&'a mut Option<T>,capability:super::ApplyCapability,turn:semio_framework_value::retained_clone::OriginalSourceTurnWitness<'_>,output:&mut Option<semio_framework_value::retained_clone::OriginalEffectAuthority<Self>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:semio_framework_value::retained_clone::OriginalEffectReply<'r,semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>,semio_framework_value::ValueError>{use semio_framework_value::retained_clone::{retained_clone_grant_admits,RetainedCloneProgress,RetainedCloneStep};if authored.is_none()||displaced.is_some()||output.is_some(){return Err(semio_framework_value::ValueError::invariant("field installation admission slots are invalid"))}if effect.work_source_turn()!=Some(turn.identity()){return Err(semio_framework_value::ValueError::invariant("field installation admission changed original Source"))}let demand=Self::admission_demand();if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"field installation admission exceeded original Work"))))}*output=Some(semio_framework_value::retained_clone::OriginalEffectAuthority::new(Self{authored,row,displaced,capability,turn:turn.identity()}));Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}
 /// 📏️ Quotes the resident borrowed frame header after its action settles.
 pub fn release_demand()->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Option<Self>>(),depth:1,..Default::default()}}
 /// 📤️ Releases only the settled borrowed frame under its original Source effect.
 pub fn release_into<'r>(slot:&mut Option<Self>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:semio_framework_value::retained_clone::OriginalEffectReply<'r,semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>,semio_framework_value::ValueError>{use semio_framework_value::retained_clone::{retained_clone_grant_admits,RetainedCloneProgress,RetainedCloneStep};let Some(owner)=slot.as_ref()else{return Err(semio_framework_value::ValueError::invariant("field installation frame is absent"))};if effect.work_source_turn()!=Some(owner.turn){return Err(semio_framework_value::ValueError::invariant("field installation release changed original Source"))}let demand=Self::release_demand();if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"field installation frame release exceeded original Work"))))}drop(slot.take());Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}
 /// ✅️ Observes the actual authored-to-row exchange without releasing its frame.
 pub fn installed(&self)->bool{self.authored.is_none()&&self.displaced.is_some()}
 /// 📏️ Quotes the frozen destination and owner slots.
 pub fn demand(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{field_install_demand::<T>()}
 /// ⛽️ Quotes the native transfer of the three fixed ownership headers.
 pub fn work_demand(&self)->u64{1}
}
impl<T> semio_framework_value::retained_clone::OriginalEffectProducer for RetainedFieldInstallFrame<'_,T>{
 type Output=semio_framework_value::RetainedCloneStep;type Failure=semio_framework_value::ValueError;type Context<'turn,'source>=RetainedFieldInstallContext<'turn> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Result<(),semio_framework_value::ValueError>where 'source:'turn{if !context.turn.matches(self.turn){return Err(semio_framework_value::ValueError::invariant("field installation changed original Source turn"))}Ok(())}
 fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<semio_framework_value::retained_clone::OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}
 fn receive<'r,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:semio_framework_value::retained_clone::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,semio_framework_value::ValueError>where 'source:'turn{install_field_into(self.authored,self.row,self.displaced,self.capability,effect,reply)}
 fn recover<'r,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:&mut semio_framework_value::RetainedAllocationEffect<'_>,_:semio_framework_value::retained_clone::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,semio_framework_value::ValueError>where 'source:'turn{Err(semio_framework_value::ValueError::invariant("native field installation has no unrecorded callback"))}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 📬️ A reply recording one retained clone step.
pub type Reply<'r,T=RetainedCloneStep,E=OriginalRetirementFailure>=semio_framework_value::retained_clone::OriginalEffectReply<'r,T,E>;
/// ✅️ Proof that a retained clone step reply was recorded or kept pending.
pub type Done<'r,T=RetainedCloneStep,E=OriginalRetirementFailure>=semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,T,E>;
type Key=(semio_framework_value::retained_clone::OriginalEffectInvocationIdentity,u8,usize,u64,bool);
/// 🛰️ Retains the one original child action in flight; the child's reply lives in the turn that lends it.
#[derive(Default)]
pub struct Flight{route:Option<Key>}
impl Flight{
 /// 💤️ Reports that no child action is retained.
 pub fn idle(&self)->bool{self.route.is_none()}
 /// 🪪️ Reports that the retained action runs without an original close loan.
 pub fn loanless(&self)->bool{self.route.is_some_and(|key|!key.4)}
 /// 🪢️ Reports whether the retained action runs under an original close loan.
 pub fn loaned(&self)->Option<bool>{self.route.map(|key|key.4)}
 /// 🧭️ Reads the retained child route.
 pub fn route(&self)->Option<u8>{self.route.map(|key|key.1)}
 fn key(effect:&RetainedAllocationEffect<'_>,recover:bool,route:u8,body:usize,maximum:u64,original:Option<&mut semio_framework_value::OriginalRetirementCloseLoan<'_>>)->Result<Key,ValueError>{
  use semio_framework_value::retained_clone::OriginalEffectCallbackMode;
  let invocation=effect.invocation().ok_or_else(||ValueError::invariant("retained field action lacks its original invocation"))?;
  if invocation.mode()!=if recover{OriginalEffectCallbackMode::Recover}else{OriginalEffectCallbackMode::Receive}{return Err(ValueError::invariant("retained field action changed its invocation mode"))}
  Ok((invocation.identity(),route,body,maximum,original.is_some()))
 }
 /// 🚀️ Retains a fresh child action before it runs.
 pub fn start(&mut self,effect:&RetainedAllocationEffect<'_>,route:u8,body:usize,maximum:u64,original:Option<&mut semio_framework_value::OriginalRetirementCloseLoan<'_>>)->Result<(),ValueError>{
  if !self.idle(){return Err(ValueError::invariant("retained field has an unresolved original action"))}
  self.route=Some(Self::key(effect,false,route,body,maximum,original)?);Ok(())
 }
 /// 🔁️ Authenticates a recovery against the retained action and returns its route.
 pub fn resume(&self,effect:&RetainedAllocationEffect<'_>,body:usize,maximum:u64,original:Option<&mut semio_framework_value::OriginalRetirementCloseLoan<'_>>)->Result<u8,ValueError>{
  let stored=self.route.ok_or_else(||ValueError::invariant("retained field has no original action"))?;
  if Self::key(effect,true,stored.1,body,maximum,original)?!=stored{return Err(ValueError::invariant("retained field recovery changed its original action"))}Ok(stored.1)
 }
 /// 📤️ Takes the child result recorded in this turn's slot and clears the action once it is recorded.
 pub fn settle<T>(&mut self,stash:&mut Option<T>)->Option<T>{let result=stash.take();if result.is_some(){self.route=None}result}
}
/// 📥️ Lends this turn's slot to one child action.
pub(crate) fn slot<T,E>(stash:&mut Option<Result<T,E>>)->Result<Reply<'_,T,E>,ValueError>{semio_framework_value::retained_clone::OriginalEffectReply::borrow(stash)}
/// 📬️ Runs a child that completes natively inside the caller's own action and returns its recorded result.
pub(crate) fn native<T,E>(call:impl for<'r> FnOnce(Reply<'r,T,E>)->Result<Done<'r,T,E>,ValueError>)->Result<Result<T,E>,ValueError>{let mut slot=None;let done=call(semio_framework_value::retained_clone::OriginalEffectReply::borrow(&mut slot)?)?;drop(done);slot.take().ok_or_else(||ValueError::invariant("native retained field child left its reply pending"))}
/// ⏱️ Charges one unit of this action inside a recorded conclusion.
pub(crate) fn unit(effect:&mut RetainedAllocationEffect<'_>,units:u64,message:&'static str)->Result<(),ValueError>{if effect.performed(units)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,message))}}

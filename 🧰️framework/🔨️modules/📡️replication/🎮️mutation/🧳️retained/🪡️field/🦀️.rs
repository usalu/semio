//! 🪡️ Typed field copying and owned exchange retain only authored and displaced children.
#[path="🟰️change/🦀️.rs"]
pub mod change;
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

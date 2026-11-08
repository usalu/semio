//! 🧳️ Prepared publication and refusal fields retain their original owners until each native admission.
use super::ArtifactReplayPrepared;
use crate::ArtifactReplayRetirementFactory;
use semio_framework_value::{ErasedSnapshotRetirement,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneProgress,RetainedCloneStep},retirement::{controlled::ControlledRetirement,frame::admit_retirement_frame as admit_artifact_retirement}};
use semio_framework_value::retirement::RetireOwned;
type Factory<'a,P,M>=Option<&'a dyn ArtifactReplayRetirementFactory<P,M>>;
fn frame<T>()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn issuer<P,M>(factory:Factory<'_,P,M>)->Result<&dyn ArtifactReplayRetirementFactory<P,M>,ValueError>{factory.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"prepared replay publication retains its original without an installed retirement issuer"))}
pub fn terminal<P,M>(prepared:&ArtifactReplayPrepared<P,M>)->bool{prepared.next.is_none()&&matches!(&prepared.inverse,Ok(inverse)if inverse.terminal_is_empty())&&prepared.messages.capacity()==0&&prepared.apply_refusal.is_none()&&prepared.input_refusal.is_none()}
pub fn demands<P,M>(prepared:&ArtifactReplayPrepared<P,M>,factory:Factory<'_,P,M>)->Result<RetirementDemand,ValueError>{
 if prepared.next.is_some(){return Ok(RetirementDemand{capacity_bytes:issuer(factory)?.snapshot_birth_bytes(),depth:1,..Default::default()});}
 match &prepared.inverse{Err(_)=>return Ok(frame::<ControlledRetirement<ValueError>>()),Ok(inverse)if !inverse.terminal_is_empty()=>return Ok(RetirementDemand{capacity_bytes:issuer(factory)?.mutations_birth_bytes(),depth:1,..Default::default()}),Ok(_)=>{}}
 if prepared.messages.capacity()!=0{return Ok(frame::<crate::MutationMessageLedgerRetirement>());}
 if prepared.apply_refusal.is_some(){return Ok(frame::<ControlledRetirement<semio_framework_mutation_diff::MutationApplyError>>());}
 if prepared.input_refusal.is_some(){return Ok(frame::<ControlledRetirement<String>>());}
 Ok(Default::default())
}
fn done(result:Result<RetainedCloneStep,OriginalRetirementFailure>,work:u128)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})}
fn admit_option<T:RetireOwned>(original:&mut Option<T>,slot:&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
 let turn=semio_framework_value::retirement::admit_owned_retirement(original,effect);
 match turn.result{Ok(Some((owner,progress)))=>{*slot=Some(owner);done(Ok(RetainedCloneStep::Progress(progress)),turn.work)},Ok(None)=>done(Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())),turn.work),Err(error)=>done(Err(error.into()),turn.work)}
}
pub fn close<P,M>(prepared:&mut ArtifactReplayPrepared<P,M>,slot:&mut Option<Box<dyn ErasedSnapshotRetirement>>,factory:Factory<'_,P,M>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
 let grant=effect.grant();let idle=RetainedCloneProgress::default();
 if grant.maximum_items==0{return done(Ok(RetainedCloneStep::Progress(idle)),0);}
 let demand=match demands(prepared,factory){Ok(demand)=>demand,Err(error)=>return done(Err(error.into()),0)};
 if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return done(Ok(RetainedCloneStep::Progress(idle)),0);}
 let progress=|step:RetainedCloneStep|RetainedCloneStep::Progress(step.progress());
 if prepared.next.is_some(){return match issuer(factory){Ok(issuer)=>issuer.snapshot(&mut prepared.next,slot,effect).map(progress),Err(error)=>done(Err(error.into()),0)};}
 if !matches!(&prepared.inverse,Ok(inverse)if inverse.terminal_is_empty()){
  return match std::mem::replace(&mut prepared.inverse,Ok(Default::default())){
   Ok(original)=>{let mut owners=Some(original);let poll=match issuer(factory){Ok(issuer)=>issuer.mutations(&mut owners,slot,effect).map(progress),Err(error)=>done(Err(error.into()),0)};if let Some(original)=owners{prepared.inverse=Ok(original);}poll},
   Err(original)=>{let mut refusal=Some(original);let poll=admit_option(&mut refusal,slot,effect);if let Some(original)=refusal{prepared.inverse=Err(original);}poll}
  };
 }
 if prepared.messages.capacity()!=0{
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return done(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"prepared message frame exceeded its work ceiling").into()),1),Err(error)=>return done(Err(error.into()),1)}
  return match admit_artifact_retirement(std::mem::take(&mut prepared.messages),grant,|messages|crate::MutationMessageLedgerRetirement::new(String::new(),messages)){Ok((owner,progress))=>{*slot=Some(owner);done(Ok(RetainedCloneStep::Progress(progress)),1)},Err((error,original))=>{prepared.messages=original;done(Err(error.into()),1)}};
 }
 if prepared.apply_refusal.is_some(){return admit_option(&mut prepared.apply_refusal,slot,effect);}
 if prepared.input_refusal.is_some(){return admit_option(&mut prepared.input_refusal,slot,effect);}
 done(Ok(RetainedCloneStep::Complete(idle)),0)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

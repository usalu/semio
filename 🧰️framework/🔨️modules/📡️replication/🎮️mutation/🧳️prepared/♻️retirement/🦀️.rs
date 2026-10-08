//! 🧳️ Prepared publication and refusal fields retain their original owners until each native admission.
use super::ArtifactReplayPrepared;
use crate::ArtifactReplayRetirementFactory;
use semio_framework_value::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{controlled::ControlledRetirement,frame::admit_retirement_frame as admit_artifact_retirement}};
use semio_framework_value::retirement::RetireOwned;
type Factory<'a,P,M>=Option<&'a dyn ArtifactReplayRetirementFactory<P,M>>;
fn frame<T>()->RetirementDemand{RetirementDemand{capacity_bytes:std::mem::size_of::<T>(),depth:1,..Default::default()}}
fn issuer<P,M>(factory:Factory<'_,P,M>)->Result<&dyn ArtifactReplayRetirementFactory<P,M>,ValueError>{factory.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"prepared replay publication retains its original without an installed retirement issuer"))}
pub fn terminal<P,M>(prepared:&ArtifactReplayPrepared<P,M>)->bool{prepared.next.is_none()&&matches!(&prepared.inverse,Ok(inverse)if inverse.terminal_is_empty())&&prepared.messages.capacity()==0&&prepared.apply_refusal.is_none()&&prepared.input_refusal.is_none()}
pub fn demands<P,M>(prepared:&ArtifactReplayPrepared<P,M>,factory:Factory<'_,P,M>)->Result<RetirementDemand,ValueError>{
 if prepared.next.is_some(){return Ok(RetirementDemand{capacity_bytes:issuer(factory)?.snapshot_birth_bytes(),depth:1,..Default::default()});}
 match &prepared.inverse{Err(_)=>return Ok(frame::<ControlledRetirement<ValueError>>()),Ok(inverse)if !inverse.terminal_is_empty()=>return Ok(RetirementDemand{capacity_bytes:issuer(factory)?.mutations_birth_bytes(),depth:1,..Default::default()}),Ok(_)=>{}}
 if prepared.messages.capacity()!=0{return Ok(frame::<crate::MutationMessageLedgerRetirement>());}
 if prepared.apply_refusal.is_some(){return Ok(frame::<ControlledRetirement<crate::MutationApplyError>>());}
 if prepared.input_refusal.is_some(){return Ok(frame::<ControlledRetirement<String>>());}
 Ok(Default::default())
}
fn admit_option<T:RetireOwned>(original:&mut Option<T>,slot:&mut Option<Box<dyn ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
 match semio_framework_value::retirement::admit_owned_retirement(original.take().expect("quoted original prepared field"),grant){Ok((owner,progress))=>{*slot=Some(owner);Ok(progress)},Err((error,owner))=>{*original=Some(owner);Err(error)}}
}
pub fn close<P:Send+Sync+'static,M:Send+'static>(prepared:&mut ArtifactReplayPrepared<P,M>,slot:&mut Option<Box<dyn ErasedSnapshotRetirement>>,factory:Factory<'_,P,M>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
 let idle=RetainedCloneProgress::default();if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(idle));}let demand=demands(prepared,factory)?;if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(idle));}
 if let Some(original)=prepared.next.take(){return match issuer(factory)?.snapshot(original,grant){Ok((owner,progress))=>{*slot=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{prepared.next=Some(original);Err(error)}};}
 if !matches!(&prepared.inverse,Ok(inverse)if inverse.terminal_is_empty()){
  return match std::mem::replace(&mut prepared.inverse,Ok(Default::default())){
   Ok(original)=>match issuer(factory)?.mutations(original,grant){Ok((owner,progress))=>{*slot=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{prepared.inverse=Ok(original);Err(error)}},
   Err(original)=>match semio_framework_value::retirement::admit_owned_retirement(original,grant){Ok((owner,progress))=>{*slot=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{prepared.inverse=Err(original);Err(error)}}
  };
 }
 if prepared.messages.capacity()!=0{return match admit_artifact_retirement(std::mem::take(&mut prepared.messages),grant,|messages|crate::MutationMessageLedgerRetirement::new(String::new(),messages)){Ok((owner,progress))=>{*slot=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{prepared.messages=original;Err(error)}};}
 if prepared.apply_refusal.is_some(){return admit_option(&mut prepared.apply_refusal,slot,grant).map(RetainedCloneStep::Progress);}
 if prepared.input_refusal.is_some(){return admit_option(&mut prepared.input_refusal,slot,grant).map(RetainedCloneStep::Progress);}
 Ok(RetainedCloneStep::Complete(idle))
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

//! 🔺️ The required central retained diff owner separates immutable preflight from authorized unpublished commit.
use super::{retained_mutation_demand_fits,binding_demand};
use super::super::{ApplyCapability,MutationDiff,MutationMessage,MutationMessageLedgerRetirement};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_progress,admit_retained_clone_close}};
use std::mem::{size_of,ManuallyDrop};

/// 🧬️ A typed technology diff supplies its mandatory retained central apply cursor.
pub trait RetainedDiff<P>:MutationDiff<P>+semio_framework_value::retirement::RetireOwned+Send+Sync+'static{
 type Apply:RetainedDiffApply<P,Self>;
}
/// 🪡️ Native ordered fragments carry application ownership without claiming an independent root algebra.
pub trait RetainedDiffFragment<P>:semio_framework_value::retirement::RetireOwned+Send+Sync+Sized+'static{
 type FragmentApply:RetainedDiffApply<P,Self>;
}
impl<P,D:RetainedDiff<P>> RetainedDiffFragment<P> for D{type FragmentApply=D::Apply;}
/// 🚦️ Original preflight diagnostics remain in the cursor until their physical receipt is admitted.
pub enum RetainedDiffPreflightAdvance{Progress(RetainedCloneProgress),Ready(RetainedCloneProgress,bool)}
impl RetainedDiffPreflightAdvance{pub fn progress(&self)->RetainedCloneProgress{match self{Self::Progress(value)|Self::Ready(value,_)=>*value}}}
/// 🧾️ The original admitted or refused preflight rows transfer once before inversion and commit.
pub enum RetainedDiffPreflightStep{Progress(RetainedCloneProgress),Admitted(RetainedCloneProgress,Vec<MutationMessage>),Refused(RetainedCloneProgress,Vec<MutationMessage>)}
impl RetainedDiffPreflightStep{pub fn progress(&self)->RetainedCloneProgress{match self{Self::Progress(value)|Self::Admitted(value,_)|Self::Refused(value,_)=>*value}}}

/// 🪡️ Required typed diff plan keeps original D immutable and stages only its authored fragments.
/// Preflight borrows post read-only and resolves every fallible reference before completion.
/// Commit starts only after explicit central preflight admission and consumes staged fields.
/// Semantic refusal occurs before writes; resource failure after commit poisons unpublished post.
/// Every frontier remains closeable without altering or destroying the original diff source.
pub trait RetainedDiffApply<P,D:RetainedDiffFragment<P>>:Send+'static{
 fn new()->Self;
 fn preflight_demand(&self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&P,body:usize)->Result<RetirementDemand,ValueError>;
 fn preflight(&mut self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&P,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<RetainedDiffPreflightAdvance,ValueError>;
 fn take_messages(&mut self)->Vec<MutationMessage>;
 fn commit_demand(&self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&P,body:usize)->Result<RetirementDemand,ValueError>;
 fn commit(&mut self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&mut P,grant:RetainedCloneGrant,capability:ApplyCapability)->Result<RetainedCloneStep,ValueError>;
 fn begin_close(&mut self);
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}

/// 🧳️ Immutable original diff/base aliases and a fixed typed plan precede every central write.
pub struct RetainedDiffOwner<P,D:RetainedDiffFragment<P>>{
 apply:ManuallyDrop<Option<Box<D::FragmentApply>>>,
 diff:Option<RetainedCloneBinding>,
 original:Option<RetainedCloneBinding>,
 admitted:bool,
 refused:bool,
 finished:bool,
 closing:bool,
 pending_preflight:Option<bool>,
 messages:ManuallyDrop<Option<MutationMessageLedgerRetirement>>,
 message_index:usize,
 message_refusal:bool,
 projected:std::marker::PhantomData<fn()->P>,
}
/// 🪪️ A borrowed preflight witness ties inversion to the exact admitted D and original base.
pub struct RetainedDiffAdmission<'a,P,D:RetainedDiffFragment<P>>{diff:RetainedCloneRef<'a,D>,original:RetainedCloneRef<'a,P>}
impl<'a,P,D:RetainedDiffFragment<P>> RetainedDiffAdmission<'a,P,D>{
 pub fn diff(&self)->RetainedCloneRef<'a,D>{self.diff}
 pub fn original(&self)->RetainedCloneRef<'a,P>{self.original}
}
impl<P,D:RetainedDiffFragment<P>> RetainedDiffOwner<P,D>{
 /// 🛡️ Only an admitted live central preflight can lend these exact inverse sources.
 pub fn inverse_admission<'a>(&self,diff:RetainedCloneRef<'a,D>,original:RetainedCloneRef<'a,P>)->Result<RetainedDiffAdmission<'a,P,D>,ValueError>{
  if !self.admitted||self.refused||self.finished||self.closing||!diff.check_binding(&self.diff)?||!original.check_binding(&self.original)?{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"inverse preparation lacks the exact live central preflight"));}
  Ok(RetainedDiffAdmission{diff,original})
 }
}
/// 📦️ Quotes the fixed required plan before any diff content or projection is inspected.
pub fn retained_diff_birth_demand<P,D:RetainedDiffFragment<P>>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<D::FragmentApply>(),capacity_bytes:size_of::<D::FragmentApply>(),depth:1,..Default::default()}}
/// 🌱️ An unfunded constructor creates no plan, child alias or payload copy.
pub fn begin_retained_diff<P,D:RetainedDiffFragment<P>>(grant:RetainedCloneGrant)->Option<(RetainedDiffOwner<P,D>,RetainedCloneProgress)>{let demand=retained_diff_birth_demand::<P,D>();if !retained_mutation_demand_fits(demand,grant){return None;}Some((RetainedDiffOwner{apply:ManuallyDrop::new(Some(Box::new(D::FragmentApply::new()))),diff:None,original:None,admitted:false,refused:false,finished:false,closing:false,pending_preflight:None,messages:ManuallyDrop::new(None),message_index:0,message_refusal:false,projected:std::marker::PhantomData},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))}

impl<P,D:RetainedDiffFragment<P>> RetainedDiffOwner<P,D>{
 /// 📏️ Quotes original source capture or the next read-only typed preflight fragment.
 pub fn preflight_demand(&self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&P,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.admitted||self.refused{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"central diff preflight is already settled"));}
  if self.diff.is_none(){return Ok(RetirementDemand{copy_bytes:diff.binding_copy_bytes(),depth:1,..Default::default()});}
  if self.original.is_none(){return Ok(RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()});}
  if self.pending_preflight.is_some(){
   if let Some(messages)=self.messages.as_ref(){
    return Ok(RetirementDemand{copy_bytes:if self.message_index<messages.retained_messages().len(){size_of::<semio_framework_diagnostic::Severity>()+size_of::<usize>()+size_of::<bool>()}else{size_of::<Vec<MutationMessage>>()+size_of::<bool>()*2},depth:1,..Default::default()});
   }
   return Ok(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>()*2+size_of::<MutationMessageLedgerRetirement>(),depth:1,..Default::default()});
  }
  let mut demand=self.apply.as_ref().expect("required typed diff plan").preflight_demand(diff,original,post,body)?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<Option<bool>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"central diff preflight parent extent overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"central diff preflight parent depth overflow"))?;Ok(demand)
 }
 /// 📏️ Commit requires the original admitted preflight and its still-live sources.
 pub fn commit_demand(&self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&P,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.admitted||self.refused||self.finished||self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"central diff commit lacks live admitted preflight"));}
  let mut demand=self.apply.as_ref().expect("required typed diff plan").commit_demand(diff,original,post,body.saturating_sub(size_of::<bool>()))?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<bool>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"central diff commit parent extent overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"central diff commit parent depth overflow"))?;Ok(demand)
 }
 /// 🛑️ Closure never destroys original D or P; their source issuers retain that authority.
 pub fn begin_close(&mut self){self.closing=true;if let Some(apply)=self.apply.as_mut(){apply.begin_close();}}
 /// ♻️ Staged/displaced fragments close before original aliases and the separately paid plan body.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let Some(apply)=self.apply.as_ref()else{return Ok(Default::default());};
  if !apply.terminal_is_empty(){return apply.close_demand(body);}
  if let Some(messages)=self.messages.as_ref(){return semio_framework_value::ErasedSnapshotRetirement::next_demand(messages,body);}
  if self.original.is_some(){return binding_demand(&self.original,body);}
  if self.diff.is_some(){return binding_demand(&self.diff,body);}
  Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<D::FragmentApply>>>(),release_bytes:size_of::<D::FragmentApply>(),depth:1,..Default::default()})
 }
 /// 🪜️ Settles only funded original plan ownership and validates each child receipt before terminal release.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"central diff closure not started"));}
  let demand=self.close_demand(grant.maximum_copy_bytes)?;
  if self.apply.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let apply=self.apply.as_mut().expect("original required diff plan");
  if !apply.terminal_is_empty(){let step=apply.close_step(grant)?;let step=admit_retained_clone_close(grant,step,apply.terminal_is_empty(),"retained central diff child closure")?;return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(messages)=self.messages.as_mut(){let step=messages.close_step(grant)?;if messages.terminal_is_empty(){*self.messages=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
  if self.original.is_some(){return RetainedCloneBinding::close_one(&mut self.original,grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if self.diff.is_some(){return RetainedCloneBinding::close_one(&mut self.diff,grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  drop(self.apply.take());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))
 }
 /// 🫙️ Original aliases and the typed plan body have all settled.
 pub fn terminal_is_empty(&self)->bool{self.apply.is_none()&&self.diff.is_none()&&self.original.is_none()&&self.messages.is_none()}
}

/// 🛡️ Central read-only preflight admits receipts before transferring original diagnostics or authorizing inversion.
pub fn preflight_retained_diff<P,D:RetainedDiffFragment<P>>(owner:&mut RetainedDiffOwner<P,D>,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&P,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<RetainedDiffPreflightStep,ValueError>{
 let demand=owner.preflight_demand(diff,original,post,body)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedDiffPreflightStep::Progress(Default::default()));}
 if let Some(progress)=diff.bind(&mut owner.diff,grant)?{return Ok(RetainedDiffPreflightStep::Progress(progress));}
 if let Some(progress)=original.bind(&mut owner.original,grant)?{return Ok(RetainedDiffPreflightStep::Progress(progress));}
 if let Some(requested)=owner.pending_preflight{
  let receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  if owner.messages.is_none(){let messages=owner.apply.as_mut().expect("original typed diff plan").take_messages();*owner.messages=Some(MutationMessageLedgerRetirement::new(String::new(),messages));return Ok(RetainedDiffPreflightStep::Progress(receipt));}
  let messages=owner.messages.as_mut().expect("original preflight diagnostic ledger");
  if let Some(message)=messages.retained_messages().get(owner.message_index){owner.message_refusal|=matches!(message.level,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal);owner.message_index+=1;return Ok(RetainedDiffPreflightStep::Progress(receipt));}
  owner.admitted=requested&&!owner.message_refusal;owner.refused=!owner.admitted;
  let remaining=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<bool>()*2,..grant};let(rows,_)=messages.take_messages_controlled(remaining)?.expect("admitted original preflight message transfer");
  return Ok(if owner.admitted{RetainedDiffPreflightStep::Admitted(receipt,rows)}else{RetainedDiffPreflightStep::Refused(receipt,rows)});
 }
 let apply=owner.apply.as_mut().expect("original typed diff plan");let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Option<bool>>(),maximum_depth:grant.maximum_depth-1,..grant};let step=apply.preflight(diff,original,post,body,child,receiving).map_err(|error|{let mut receipt=error.retained_progress();if receipt!=RetainedCloneProgress::default(){receipt.copied_bytes=receipt.copied_bytes.saturating_add(size_of::<Option<bool>>());}error.with_retained_progress(receipt)})?;let mut receipt=step.progress();receipt.copied_bytes=receipt.copied_bytes.checked_add(size_of::<Option<bool>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"central diff preflight receipt overflow"))?;let receipt=admit_retained_clone_progress(grant,receipt,"retained central diff preflight")?;
 Ok(match step{RetainedDiffPreflightAdvance::Progress(_)=>RetainedDiffPreflightStep::Progress(receipt),RetainedDiffPreflightAdvance::Ready(_,admitted)=>{owner.pending_preflight=Some(admitted);RetainedDiffPreflightStep::Progress(receipt)}})
}
/// 🔑️ Only the shared central applier issues capability after original preflight and inverse preparation.
pub fn commit_retained_diff<P,D:RetainedDiffFragment<P>>(owner:&mut RetainedDiffOwner<P,D>,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&mut P,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
 let demand=owner.commit_demand(diff,original,post,grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
 if diff.bind(&mut owner.diff,grant)?.is_some()||original.bind(&mut owner.original,grant)?.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"central diff commit lost original preflight bindings"));}
 let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<bool>(),maximum_depth:grant.maximum_depth-1,..grant};let step=super::super::with_retained_apply_capability::<P,D,_>(diff,grant,|capability|owner.apply.as_mut().expect("original typed diff apply").commit(diff,original,post,child,capability))?;let mut receipt=step.progress();receipt.copied_bytes=receipt.copied_bytes.checked_add(size_of::<bool>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"central diff commit receipt overflow"))?;let receipt=admit_retained_clone_progress(grant,receipt,"retained central diff commit")?;
 if matches!(step,RetainedCloneStep::Complete(_)){owner.finished=true;Ok(RetainedCloneStep::Complete(receipt))}else{Ok(RetainedCloneStep::Progress(receipt))}
}
impl<P,D:RetainedDiffFragment<P>> Drop for RetainedDiffOwner<P,D>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"retained central diff requires complete typed plan and original alias closure");if empty{unsafe{ManuallyDrop::drop(&mut self.apply);ManuallyDrop::drop(&mut self.messages);}}}}

#[path="🪢️sequence/🦀️.rs"]
mod sequence;
pub use sequence::RetainedDiffSequence;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub(crate) mod tests;

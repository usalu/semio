//! 🔺️ The required central retained diff owner separates immutable preflight from authorized unpublished commit.
use super::{Done,Flight,Reply,conclude,retained_mutation_demand_fits,binding_demand,settle_value};
use super::super::{ApplyCapability,MutationDiff,MutationMessage,MutationMessageLedgerRetirement};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementOutcome,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_progress,admit_retained_clone_close}};
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

/// 🆚️ Required typed diff plan keeps original D immutable and stages only its authored fragments.
/// Preflight borrows post read-only and resolves every fallible reference before completion.
/// Commit starts only after explicit central preflight admission and consumes staged fields.
/// Semantic refusal occurs before writes; resource failure after commit poisons unpublished post.
/// Every frontier remains closeable without altering or destroying the original diff source.
pub trait RetainedDiffApply<P,D:RetainedDiffFragment<P>>:Send+'static{
 fn new()->Self;
 fn preflight_demand(&self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 fn preflight_work_demand(&self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
 fn preflight_reply<'r>(&mut self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightAdvance>,recover:bool)->Result<Done<'r,RetainedDiffPreflightAdvance>,ValueError>;
 fn take_messages(&mut self)->Vec<MutationMessage>;
 fn commit_demand(&self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&P,body:usize)->Result<RetirementDemand,ValueError>;
 fn commit_work_demand(&self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&P,body:usize)->Result<u64,ValueError>;
 fn commit(&mut self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&mut P,effect:&mut RetainedAllocationEffect<'_>,capability:ApplyCapability)->OriginalRetirementOutcome;
 fn begin_close(&mut self);
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn requires_original_birth(&self)->bool{false}
 fn requires_original_control(&self)->bool{false}
 fn close_original_birth_step(&mut self,_:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,_:RetainedCloneGrant)->OriginalRetirementPoll{semio_framework_value::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("typed diff plan has no original Birth").into()),work:0})}
 fn close_reply<'r>(&mut self,original:Option<&mut semio_framework_value::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>;
 fn terminal_is_empty(&self)->bool;
}

/// 🧳️ Immutable original diff/base aliases and a fixed typed plan precede every central write.
pub struct RetainedDiffOwner<P,D:RetainedDiffFragment<P>>{
 apply:ManuallyDrop<Option<Box<D::FragmentApply>>>,
 diff:Option<RetainedCloneBinding>,
 original:Option<RetainedCloneBinding>,
 post_identity:Option<(u64,usize,usize,usize)>,
 admitted:bool,
 refused:bool,
 finished:bool,
 closing:bool,
 flight:Flight,
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
pub fn retained_diff_birth_demand<P,D:RetainedDiffFragment<P>>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<D::FragmentApply>()+2*size_of::<RetainedDiffOwner<P,D>>(),capacity_bytes:size_of::<D::FragmentApply>(),depth:1,..Default::default()}}
/// 🌱️ An unfunded constructor creates no plan, child alias or payload copy.
pub fn begin_retained_diff<P,D:RetainedDiffFragment<P>>(grant:RetainedCloneGrant)->Option<(RetainedDiffOwner<P,D>,RetainedCloneProgress)>{let demand=retained_diff_birth_demand::<P,D>();if !retained_mutation_demand_fits(demand,grant){return None;}Some((RetainedDiffOwner{apply:ManuallyDrop::new(Some(Box::new(D::FragmentApply::new()))),diff:None,original:None,post_identity:None,admitted:false,refused:false,finished:false,closing:false,flight:Default::default(),pending_preflight:None,messages:ManuallyDrop::new(None),message_index:0,message_refusal:false,projected:std::marker::PhantomData},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))}

impl<P,D:RetainedDiffFragment<P>> RetainedDiffOwner<P,D>{
 /// 📏️ Quotes original source capture or the next read-only typed preflight fragment.
 pub fn preflight_demand(&self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.admitted||self.refused{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"central diff preflight is already settled"));}
  if self.post_identity.is_some_and(|identity|identity!=post.reference_identity()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"central diff live post source or projection changed"));}
  if maximum_work_units==0{return Ok(Default::default())}
  if self.diff.is_none(){return Ok(RetirementDemand{copy_bytes:diff.binding_copy_bytes(),depth:1,..Default::default()});}
  if self.original.is_none(){return Ok(RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()});}
  if self.post_identity.is_none(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<(u64,usize,usize,usize)>>(),depth:1,..Default::default()});}
  if self.pending_preflight.is_some(){
   if let Some(messages)=self.messages.as_ref(){
    return Ok(RetirementDemand{copy_bytes:if self.message_index<messages.retained_messages().len(){size_of::<semio_framework_diagnostic::Severity>()+size_of::<usize>()+size_of::<bool>()}else{size_of::<Vec<MutationMessage>>()+size_of::<bool>()*2},depth:1,..Default::default()});
   }
   return Ok(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>()*2+size_of::<MutationMessageLedgerRetirement>(),depth:1,..Default::default()});
  }
  let mut demand=self.apply.as_ref().expect("required typed diff plan").preflight_demand(diff,original,post,body,maximum_work_units)?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<Option<bool>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"central diff preflight parent extent overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"central diff preflight parent depth overflow"))?;Ok(demand)
 }
 /// ⛽️ Quotes the exact selected original binding, diagnostic or preflight child action.
 pub fn preflight_work_demand(&self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.preflight_demand(diff,original,post,body,maximum_work_units)?;if self.diff.is_some()&&!diff.check_binding(&self.diff)?||self.original.is_some()&&!original.check_binding(&self.original)?{return Err(ValueError::invariant("central diff preflight original source changed"))}if maximum_work_units==0{return Ok(0)}if self.diff.is_none()||self.original.is_none()||self.post_identity.is_none()||self.pending_preflight.is_some(){return Ok(1)}self.apply.as_ref().ok_or_else(||ValueError::invariant("original typed diff plan is absent"))?.preflight_work_demand(diff,original,post,body,maximum_work_units)}
 /// 🔏️ Commit requires the original admitted preflight and its still-live sources.
 pub fn commit_demand(&self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&P,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.admitted||self.refused||self.finished||self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"central diff commit lacks live admitted preflight"));}
  let mut demand=self.apply.as_ref().expect("required typed diff plan").commit_demand(diff,original,post,body.saturating_sub(size_of::<bool>()))?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<bool>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"central diff commit parent extent overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"central diff commit parent depth overflow"))?;Ok(demand)
 }
 /// ⛽️ Quotes the selected admitted typed commit without changing its original sources.
 pub fn commit_work_demand(&self,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&P,body:usize)->Result<u64,ValueError>{self.commit_demand(diff,original,post,body)?;if !diff.check_binding(&self.diff)?||!original.check_binding(&self.original)?{return Err(ValueError::invariant("central diff commit lost original preflight bindings"))}self.apply.as_ref().ok_or_else(||ValueError::invariant("original typed diff plan is absent"))?.commit_work_demand(diff,original,post,body.saturating_sub(size_of::<bool>()))}
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
 /// ⛽️ Quotes the exact selected child, diagnostic, binding or body release work.
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"central diff closure not started"))}let Some(apply)=self.apply.as_ref()else{return Ok(0)};if !apply.terminal_is_empty(){return apply.close_work_demand(body)}if self.messages.is_some(){return Ok(1)}if self.original.is_some(){return RetainedCloneBinding::close_work_demand(&self.original,body)}if self.diff.is_some(){return RetainedCloneBinding::close_work_demand(&self.diff,body)}Ok(1)}
 /// 👶️ Reports that the selected child Birth must run on the original controller first.
 pub fn requires_original_birth(&self)->bool{if !self.closing||!self.flight.idle(){return false}self.apply.as_ref().is_some_and(|a|!a.terminal_is_empty()&&a.requires_original_birth())}
 /// 🪪️ Reports that the selected child close needs the original controller loan.
 pub fn requires_original_control(&self)->bool{if let Some(loaned)=self.flight.loaned(){return loaned}self.apply.as_ref().is_some_and(|a|!a.terminal_is_empty()&&a.requires_original_control())}
 /// 👶️ Continues the selected child Birth on the original controller.
 pub fn close_original_birth_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{match self.apply.as_mut(){Some(a)=>a.close_original_birth_step(original,grant),None=>semio_framework_value::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("central diff Birth is not selected").into()),work:0})}}
 pub fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply(None,body,effect,reply,false)}
 pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply(None,body,effect,reply,true)}
 pub fn close_original_into<'r>(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply(Some(original),body,effect,reply,false)}
 pub fn recover_close_original_into<'r>(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply(Some(original),body,effect,reply,true)}
 fn close_reply<'r>(&mut self,mut original:Option<&mut semio_framework_value::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"central diff closure not started").into())}
  if !recover{
   if !self.flight.idle(){return Err(ValueError::invariant("central diff close requires recovery").into())}
   let demand=self.close_demand(body)?;if self.apply.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.requires_original_birth(){return Err(ValueError::invariant("central diff Birth must settle before effect").into())}
   if self.requires_original_control()&&original.is_none(){return Err(ValueError::invariant("central diff close requires original controller").into())}
   let apply=self.apply.as_ref().ok_or_else(||ValueError::invariant("original required diff plan is absent"))?;
   if apply.terminal_is_empty(){
    if let Some(messages)=self.messages.as_mut(){let turn=messages.close_step(grant);work+=u128::from(messages.last_turn_work());let step=turn?;if messages.terminal_is_empty(){*self.messages=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
    if self.original.is_some(){let turn=RetirementPollExit::settled(RetainedCloneBinding::close_one(&mut self.original,body,effect))?;work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()))}if self.diff.is_some(){let turn=RetirementPollExit::settled(RetainedCloneBinding::close_one(&mut self.diff,body,effect))?;work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()))}
    work+=1;drop(self.apply.take());return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))
   }
  }
  let looped=match self.flight.loaned(){Some(loaned)=>loaned,None=>self.apply.as_ref().is_some_and(|a|a.requires_original_control())};
  let loan=if looped{original.as_deref_mut()}else{None};
  let missing=||ValueError::invariant("central diff original controller missing");
  let step=retained_routed!(self.flight,effect,recover,1,body,0,loan,work,|r| self.apply.as_mut().ok_or_else(missing)?.close_reply(if looped{original.as_deref_mut()}else{None},body,effect,r,false),self.apply.as_mut().ok_or_else(missing)?.close_reply(if looped{original.as_deref_mut()}else{None},body,effect,r,true))?;
  let apply=self.apply.as_ref().ok_or_else(missing)?;let step=admit_retained_clone_close(grant,step,apply.terminal_is_empty(),"retained central diff child closure")?;Ok(RetainedCloneStep::Progress(step.progress()))
 })();conclude(effect,base,result,work,|step|step.progress(),reply)}
 /// 🫙️ Original aliases and the typed plan body have all settled.
 pub fn terminal_is_empty(&self)->bool{self.apply.is_none()&&self.diff.is_none()&&self.original.is_none()&&self.messages.is_none()&&self.flight.idle()}
}

/// 📖️ Central read-only preflight admits receipts before transferring original diagnostics or authorizing inversion.
pub fn preflight_retained_diff_into<'r,P,D:RetainedDiffFragment<P>>(owner:&mut RetainedDiffOwner<P,D>,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightStep>)->Result<Done<'r,RetainedDiffPreflightStep>,ValueError>{preflight_reply(owner,diff,original,post,body,maximum_work_units,effect,receiving,reply,false)}
/// 🔁️ Recovers the same pending typed plan preflight action.
pub fn recover_preflight_retained_diff_into<'r,P,D:RetainedDiffFragment<P>>(owner:&mut RetainedDiffOwner<P,D>,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightStep>)->Result<Done<'r,RetainedDiffPreflightStep>,ValueError>{preflight_reply(owner,diff,original,post,body,maximum_work_units,effect,receiving,reply,true)}
fn preflight_reply<'r,P,D:RetainedDiffFragment<P>>(owner:&mut RetainedDiffOwner<P,D>,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightStep>,recover:bool)->Result<Done<'r,RetainedDiffPreflightStep>,ValueError>{let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;let result:Result<RetainedDiffPreflightStep,RetirementPollExit>=(||{
 if !recover{
  if !owner.flight.idle(){return Err(ValueError::invariant("central diff preflight requires recovery").into())}
  let selected=owner.preflight_work_demand(diff,original,post,body,maximum_work_units)?;if selected==0{return Ok(RetainedDiffPreflightStep::Progress(Default::default()))}let demand=owner.preflight_demand(diff,original,post,body,maximum_work_units)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedDiffPreflightStep::Progress(Default::default()));}
  let turn=diff.bind(&mut owner.diff,effect);work+=turn.work;if let Some(progress)=turn.result?{return Ok(RetainedDiffPreflightStep::Progress(progress));}
  let turn=original.bind(&mut owner.original,effect);work+=turn.work;if let Some(progress)=turn.result?{return Ok(RetainedDiffPreflightStep::Progress(progress));}
  if owner.post_identity.is_none(){work+=1;owner.post_identity=Some(post.reference_identity());return Ok(RetainedDiffPreflightStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
  if let Some(requested)=owner.pending_preflight{work+=1;
   let receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
   if owner.messages.is_none(){let messages=owner.apply.as_mut().expect("original typed diff plan").take_messages();*owner.messages=Some(MutationMessageLedgerRetirement::new(String::new(),messages));return Ok(RetainedDiffPreflightStep::Progress(receipt));}
   let messages=owner.messages.as_mut().expect("original preflight diagnostic ledger");
   if let Some(message)=messages.retained_messages().get(owner.message_index){owner.message_refusal|=matches!(message.level,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal);owner.message_index+=1;return Ok(RetainedDiffPreflightStep::Progress(receipt));}
   owner.admitted=requested&&!owner.message_refusal;owner.refused=!owner.admitted;
   let remaining=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<bool>()*2,..grant};let(rows,_)=messages.take_messages_controlled(remaining)?.expect("admitted original preflight message transfer");
   return Ok(if owner.admitted{RetainedDiffPreflightStep::Admitted(receipt,rows)}else{RetainedDiffPreflightStep::Refused(receipt,rows)});
  }
 }
 let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Option<bool>>(),maximum_depth:grant.maximum_depth-1,..grant};let before=work;
 let result=retained_routed!(owner.flight,effect,recover,1,body,maximum_work_units,None,work,|r| owner.apply.as_mut().ok_or_else(||ValueError::invariant("original typed diff plan is absent"))?.preflight_reply(diff,original,post,body,maximum_work_units,&mut effect.nested(child),receiving,r,false),owner.apply.as_mut().ok_or_else(||ValueError::invariant("original typed diff plan is absent"))?.preflight_reply(diff,original,post,body,maximum_work_units,&mut effect.nested(child),receiving,r,true));
 let step=result.map_err(|error|{if work==before{return error}let mut receipt=error.retained_progress();receipt.copied_bytes=receipt.copied_bytes.saturating_add(size_of::<Option<bool>>());error.with_retained_progress(receipt)})?;let mut receipt=step.progress();receipt.copied_bytes=receipt.copied_bytes.checked_add(size_of::<Option<bool>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"central diff preflight receipt overflow"))?;let receipt=admit_retained_clone_progress(grant,receipt,"retained central diff preflight")?;
 Ok(match step{RetainedDiffPreflightAdvance::Progress(_)=>RetainedDiffPreflightStep::Progress(receipt),RetainedDiffPreflightAdvance::Ready(_,admitted)=>{owner.pending_preflight=Some(admitted);RetainedDiffPreflightStep::Progress(receipt)}})
})();conclude(effect,base,result,work,RetainedDiffPreflightStep::progress,reply)
}
/// 🔑️ Only the shared central applier issues capability after original preflight and inverse preparation.
pub fn commit_retained_diff<P,D:RetainedDiffFragment<P>>(owner:&mut RetainedDiffOwner<P,D>,diff:RetainedCloneRef<'_,D>,original:RetainedCloneRef<'_,P>,post:&mut P,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementOutcome{let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;let result:Result<RetainedCloneStep,OriginalRetirementFailure>=(||{
 let demand=owner.commit_demand(diff,original,post,grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}
 let turn=diff.bind(&mut owner.diff,effect);work+=turn.work;if turn.result?.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"central diff commit lost original preflight bindings").into())}let turn=original.bind(&mut owner.original,effect);work+=turn.work;if turn.result?.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"central diff commit lost original preflight bindings").into())}
 let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<bool>(),maximum_depth:grant.maximum_depth-1,..grant};let turn=semio_framework_mutation_diff::diff::with_retained_apply_capability::<D,_>(diff,grant,|capability|{let apply=owner.apply.as_mut().ok_or_else(||ValueError::invariant("original typed diff apply is absent"))?;Ok(apply.commit(diff,original,post,&mut effect.nested(child),capability))})?;work+=turn.work;let step=turn.result.map_err(|error|if turn.work==0{error}else{let mut progress=error.retained_progress();progress.copied_bytes=progress.copied_bytes.saturating_add(size_of::<bool>());error.with_retained_progress(progress)})?;let mut receipt=step.progress();receipt.copied_bytes=receipt.copied_bytes.checked_add(size_of::<bool>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"central diff commit receipt overflow"))?;let receipt=admit_retained_clone_progress(grant,receipt,"retained central diff commit")?;if matches!(step,RetainedCloneStep::Complete(_)){owner.finished=true;Ok(RetainedCloneStep::Complete(receipt))}else{Ok(RetainedCloneStep::Progress(receipt))}
})();settle_value(effect,base,result,work,|step|step.progress())}
impl<P,D:RetainedDiffFragment<P>> Drop for RetainedDiffOwner<P,D>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"retained central diff requires complete typed plan and original alias closure");if empty{unsafe{ManuallyDrop::drop(&mut self.apply);ManuallyDrop::drop(&mut self.messages);}}}}

#[path="🪢️sequence/🦀️.rs"]
mod sequence;
pub use sequence::RetainedDiffSequence;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub(crate) mod tests;

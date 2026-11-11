//! 🔗️ Immutable original view roots make routing aliases bounded independently of the view body.
use crate::ActionMeta;
use semio_framework::ViewModel;
use semio_framework_job::{StepContext,OriginalConstructionAdmission,OriginalConstructionDemand,OriginalTurnFailure,close_original_effect_step,close_original_retained_effect_step};
use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,OriginalRetirementFailure,RetirementDemand,ValueError,ValueRefusalKind,RetainedAllocationEffect,retained_clone::RetainedCloneClose,retirement::RetireOwned};
use std::{sync::Arc,mem::size_of};
#[path="routing.rs"]
pub mod routing;
#[path="delivery.rs"]
pub mod delivery;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
/// 🪙️ Runs one quoted pure or allocating routing action inside the turn's own performed-work effect; `None` means the quote was not admitted and nothing ran.
pub(crate) fn funded<T>(cx:&mut StepContext<'_>,demand:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<(T,RetainedCloneProgress),ValueError>)->Result<Option<T>,OriginalTurnFailure>{
 let mut output=None;
 close_original_retained_effect_step(cx,demand,|effect|{
  if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original routing action exceeds its work grant"))}
  let(value,progress)=run(effect)?;output=Some(value);Ok::<_,ValueError>(RetainedCloneStep::Progress(progress))
 })?;
 Ok(output)
}
/// 🧹️ Advances the pending ticket of one retained close inside the original retirement effect.
pub(crate) fn close_pending(cx:&mut StepContext<'_>,retirement:&mut RetainedCloneClose)->Result<(),OriginalTurnFailure>{
 let body=cx.retained_grant().maximum_copy_bytes;let d=retirement.demands(body)?;
 close_original_effect_step(cx,d,|effect|retirement.step_granted(body,effect))?;Ok(())
}
/// 🧹️ Begins the retained close of one resident owner slot inside the original retirement effect.
pub(crate) fn begin_close<T:RetireOwned>(cx:&mut StepContext<'_>,demand:RetirementDemand,retirement:&mut RetainedCloneClose,slot:&mut Option<T>)->Result<(),OriginalTurnFailure>{
 close_original_effect_step(cx,demand,|effect|{let turn=retirement.begin_granted(slot,effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{work:turn.work,result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(OriginalRetirementFailure::from)})})?;Ok(())
}
impl ActionMeta{
 /// 🌱️ The actual decoded view moves into its original native Arc receiving turn without cloning its body.
 pub fn admit_view_root(original:&mut Option<ViewModel>,output:&mut Option<Arc<ViewModel>>,frame:&mut semio_framework_value::native_decoding::NativeSharedAdmission<ViewModel>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !frame.can_advance(){return Err(semio_framework_value::ValueError::invariant("original view allocation callback requires resolution").into())}
  if output.is_some()||original.is_none(){return Ok(true)}
  let mut scope:Option<semio_framework_job::OriginalNativeReceivingOutcome>=None;let mut stepped=None;
  if !cx.with_native_receiving(&mut scope,|native,_|{stepped=Some(native.receive_arc(original,output,frame));Ok(())})?{return Ok(output.is_some())}
  let invalid=|message:&'static str|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,message);
  let scope=scope.ok_or_else(||invalid("original view receiving scope vanished"))?;
  if scope.source_refusal.is_some()||scope.call.is_err(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original view receiving scope refused").with_retained_progress(scope.retained_progress()).into())}
  let turn=stepped.ok_or_else(||invalid("original view receiving step vanished"))?;
  semio_framework_job::receive_original_receiving_work(cx,semio_framework_value::RetainedWorkOutcome{work:turn.work,result:turn.result.map(semio_framework_value::RetainedCloneStep::Progress).map_err(semio_framework_job::OriginalTurnPrimary::Receiving)})?;
  Ok(output.is_some())
 }
 /// 📏️ An actor and immutable view alias own only their concrete typed metadata headers.
 pub fn routing_alias_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>(),depth:1,..Default::default()}}
 /// 🪪️ Routing retains the actual actor, instance and admitted view under this original construction grant.
 pub fn admit_routing_alias(&self,admission:&mut OriginalConstructionAdmission,output:&mut Option<Self>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let d=Self::routing_alias_demands();semio_framework_job::admit_original_construction_settled(cx,admission,output,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:0,release_bytes:0,depth:d.depth},1,|_,_|{let mut meta=self.clone();meta.expected_document_revision=None;semio_framework_value::RetainedWorkOutcome{result:Ok((meta,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}})}
}

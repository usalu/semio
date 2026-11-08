//! 🧯️ Original turn failures retain semantic and receipt causes as separate complete owners.
use semio_framework_value_retained_clone::{ValueError,RetainedCloneProgress,OriginalRetirementFailure};
#[path="🎟️context/🦀️.rs"]
mod context;
pub use context::OriginalFailureContext;
use semio_framework_diagnostic::Fault;
#[path="🛬️receiving/🦀️.rs"]
mod receiving;
pub use receiving::OriginalReceivingPrimary;
#[path="📦️domain/🦀️.rs"]
mod domain;
pub use domain::{OriginalClosureFailure,OriginalDomainFailure,OriginalDomainPrimary,OriginalDomainReservation,OriginalDomainProjection,OriginalDomainView,admit_original_domain_failure,original_domain_failure_demand};
#[path="🫴️projection/🦀️.rs"]
mod projection;
pub use projection::{project_original_source_close_failure,key_original_source_close_failure,project_original_turn_failure,key_original_turn_failure,project_original_value_error,key_original_value_error,project_original_fault,key_original_fault,project_original_retirement_failure,key_original_retirement_failure,project_original_retirement_receiving,key_original_retirement_receiving,project_original_receiving_primary,key_original_receiving_primary};
#[path="🎟️work/🦀️.rs"]
mod work;
pub use work::OriginalWorkFailure;

/// 🪢️ Keeps an original retirement failure and a simultaneous original receiving refusal.
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
pub struct OriginalRetirementReceivingPrimary{
 pub retirement:OriginalRetirementFailure,
 pub receiving_primary:ValueError,
 pub receiving_effects:RetainedCloneProgress,
 pub receiving_receipt_refusal:Option<ValueError>,
 pub receiving_context_refusal:Option<ValueError>,
 pub receiving_work_refusal:Option<ValueError>,
}
/// ⚠️ Owns the original Source, cancellation or paid retirement close cause.
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
#[value(send_only)]
pub enum OriginalSourceCloseFailure{Value(ValueError),Cancellation(semio_framework_async::CancelTokenRetirementError),Retirement(OriginalRetirementFailure)}
impl OriginalSourceCloseFailure{
 /// 📊️ Borrows the original retained physical progress.
 pub fn retained_progress(&self)->RetainedCloneProgress{match self{Self::Value(error)=>error.retained_progress(),Self::Cancellation(semio_framework_async::CancelTokenRetirementError::Refused(error))=>error.retained_progress(),Self::Cancellation(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>Default::default(),Self::Retirement(failure)=>failure.retained_progress()}}
 /// 🗨️ Borrows original semantic text without consuming its typed cause.
 pub fn message(&self)->&str{match self{Self::Value(error)=>&error.message,Self::Cancellation(semio_framework_async::CancelTokenRetirementError::Refused(error))=>&error.message,Self::Cancellation(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>"Original cancellation close is blocked",Self::Retirement(failure)=>failure.primary().or(failure.work_refusal()).or(failure.receipt_refusal()).map_or("Original retirement refused",|cause|cause.message.as_ref())}}
}
impl semio_framework_value_native_owned::native_decoding::NativeReceivingEffectError for OriginalSourceCloseFailure{fn receiving_progress(&self)->RetainedCloneProgress{self.retained_progress()}}
/// 🧳️ The complete native refusal or application diagnostic keeps its original ownership.
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
pub enum OriginalTurnPrimary{SourceClose(OriginalSourceCloseFailure),Value(ValueError),Fault(Fault),Domain(OriginalDomainPrimary),Receiving(OriginalReceivingPrimary),NativeSource(semio_framework_job_receiving::OriginalNativeReceivingOutcome),NativeRetirementSource(semio_framework_job_receiving::OriginalNativeReceivingOutcome<Result<(),OriginalReceivingPrimary>>),Retirement(OriginalRetirementFailure),RetirementReceiving(OriginalRetirementReceivingPrimary)}
fn native_retirement_child(original:&semio_framework_job_receiving::OriginalNativeReceivingOutcome<Result<(),OriginalReceivingPrimary>>)->Option<&OriginalReceivingPrimary>{
 original.call.as_ref().ok().or_else(||original.call.as_ref().err().and_then(|failure|failure.output.as_ref())).and_then(|result|result.as_ref().err())
}
impl OriginalTurnPrimary{
 /// 📊️ Borrows the actual producer receipt independently of its diagnostic representation.
 pub fn retained_progress(&self)->RetainedCloneProgress{match self{Self::SourceClose(original)=>original.retained_progress(),Self::Value(error)=>error.retained_progress(),Self::Fault(fault)=>fault.retained_progress(),Self::Domain(original)=>original.retained_progress(),Self::Receiving(original)=>original.retained_progress(),Self::NativeSource(original)=>original.retained_progress(),Self::NativeRetirementSource(original)=>native_retirement_child(original).map_or_else(||original.retained_progress(),|child|child.retained_progress()),Self::Retirement(original)=>original.retained_progress(),Self::RetirementReceiving(original)=>original.retirement.retained_progress()}}
 /// 🗨️ Borrows the original semantic text without cloning or formatting.
 pub fn message(&self)->&str{match self{Self::SourceClose(original)=>original.message(),Self::Value(error)=>&error.message,Self::Fault(fault)=>&fault.message,Self::Domain(original)=>original.message(),Self::Receiving(original)=>original.message(),Self::NativeSource(original)=>original.message(),Self::NativeRetirementSource(original)=>native_retirement_child(original).map_or_else(||original.message(),|child|child.message()),Self::Retirement(original)=>original.primary().or(original.work_refusal()).or(original.receipt_refusal()).map_or("Original retirement refused",|cause|cause.message.as_ref()),Self::RetirementReceiving(original)=>original.retirement.primary().or(original.retirement.work_refusal()).or(original.retirement.receipt_refusal()).map_or("Original retirement refused",|cause|cause.message.as_ref())}}
}
impl From<OriginalSourceCloseFailure> for OriginalTurnPrimary{fn from(primary:OriginalSourceCloseFailure)->Self{Self::SourceClose(primary)}}
impl From<ValueError> for OriginalTurnPrimary{fn from(primary:ValueError)->Self{Self::Value(primary)}}
impl From<Fault> for OriginalTurnPrimary{fn from(primary:Fault)->Self{Self::Fault(primary)}}
impl From<OriginalRetirementFailure> for OriginalTurnPrimary{fn from(primary:OriginalRetirementFailure)->Self{Self::Retirement(primary)}}

/// 🪢️ Original semantic, receipt and context refusals move together without cloning or prose conversion.
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
#[value(send_only)]
pub struct OriginalTurnFailure{pub effects:RetainedCloneProgress,pub primary:OriginalTurnPrimary,pub receipt_refusal:Option<ValueError>,pub context_refusal:Option<ValueError>,pub work_refusal:Option<ValueError>}
impl OriginalTurnFailure{
 /// ⛽️ Borrows the independent original work settlement refusal.
 pub fn work_refusal(&self)->Option<&ValueError>{self.work_refusal.as_ref()}
 /// 🧳️ Adopts one original work refusal or returns both owners unchanged.
 pub fn with_work_refusal(mut self,refusal:ValueError)->Result<Self,(Self,ValueError)>{if self.work_refusal.is_some(){return Err((self,refusal))}self.work_refusal=Some(refusal);Ok(self)}
 /// ⏳️ Borrows the complete enclosing effect span independently of its last semantic cause.
 pub fn retained_progress(&self)->RetainedCloneProgress{self.effects}
 /// 🧾️ Records the actual failed child effects while keeping its primary and any receipt refusal.
 pub fn debit_original(primary:impl Into<OriginalTurnPrimary>,cx:&mut impl OriginalFailureContext)->Self{let primary=primary.into();Self{effects:primary.retained_progress(),receipt_refusal:cx.consume_retained(primary.retained_progress()).err(),context_refusal:None,work_refusal:None,primary}}
 /// 🧳️ Borrows the unchanged typed Source close cause.
 pub fn primary_source_close(&self)->Option<&OriginalSourceCloseFailure>{match &self.primary{OriginalTurnPrimary::SourceClose(source)=>Some(source),_=>None}}
 /// 🫴️ Lends only the original native refusal when that is the retained primary.
 pub fn primary_value(&self)->Option<&ValueError>{match &self.primary{OriginalTurnPrimary::Value(error)=>Some(error),_=>None}}
 /// 🫱️ Lends the complete original diagnostic without weakening its owned fields.
 pub fn primary_fault(&self)->Option<&Fault>{match &self.primary{OriginalTurnPrimary::Fault(fault)=>Some(fault),_=>None}}
 /// 🧬️ Lends the full admitted domain source without a diagnostic mirror.
 pub fn primary_domain(&self)->Option<&OriginalDomainPrimary>{match &self.primary{OriginalTurnPrimary::Domain(original)=>Some(original),_=>None}}
 /// 🛬️ Borrows each original construction cause independently of the enclosing receipt failures.
 pub fn primary_receiving(&self)->Option<&OriginalReceivingPrimary>{match &self.primary{OriginalTurnPrimary::Receiving(original)=>Some(original),_=>None}}
 /// ♻️ Borrows every original retirement cause, including its retained nested child.
 pub fn primary_retirement(&self)->Option<&OriginalRetirementFailure>{match &self.primary{OriginalTurnPrimary::Retirement(original)=>Some(original),_=>None}}
}
impl From<ValueError> for OriginalTurnFailure{
 fn from(primary:ValueError)->Self{Self{effects:primary.retained_progress(),primary:primary.into(),receipt_refusal:None,context_refusal:None,work_refusal:None}}
}
impl From<Fault> for OriginalTurnFailure{fn from(primary:Fault)->Self{Self{effects:primary.retained_progress(),primary:primary.into(),receipt_refusal:None,context_refusal:None,work_refusal:None}}}
impl From<OriginalSourceCloseFailure> for OriginalTurnFailure{fn from(primary:OriginalSourceCloseFailure)->Self{Self::from(OriginalTurnPrimary::SourceClose(primary))}}
impl From<OriginalRetirementFailure> for OriginalTurnFailure{fn from(primary:OriginalRetirementFailure)->Self{Self::from(OriginalTurnPrimary::Retirement(primary))}}
impl From<semio_framework_value_native_owned::native_decoding::NativeConstructionStepFailure<ValueError>> for OriginalTurnFailure {
 fn from(failure:semio_framework_value_native_owned::native_decoding::NativeConstructionStepFailure<ValueError>)->Self {
  let primary=OriginalReceivingPrimary::from_fields(failure.primary,None,None,None,None,None,None,None,None,failure.progress);
  Self { effects:failure.progress,primary:OriginalTurnPrimary::Receiving(primary),receipt_refusal:failure.receipt_refusal,context_refusal:None,work_refusal:None }
 }
}
impl From<semio_framework_value_native_owned::native_decoding::NativeConstructionStepFailure<OriginalReceivingPrimary>> for OriginalTurnFailure {
 fn from(failure:semio_framework_value_native_owned::native_decoding::NativeConstructionStepFailure<OriginalReceivingPrimary>)->Self {
  let mut primary=failure.primary.unwrap_or(OriginalReceivingPrimary::from_fields(None,None,None,None,None,None,None,None,None,failure.progress));primary.progress=failure.progress;
  Self { effects:failure.progress,primary:OriginalTurnPrimary::Receiving(primary),receipt_refusal:failure.receipt_refusal,context_refusal:None,work_refusal:None }
 }
}


impl From<OriginalTurnPrimary> for OriginalTurnFailure{fn from(primary:OriginalTurnPrimary)->Self{Self{effects:primary.retained_progress(),primary,receipt_refusal:None,context_refusal:None,work_refusal:None}}}

#[path="📦️frame/🦀️.rs"]
mod return_frame;
pub use return_frame::{OriginalReturnFrame,OriginalReturnPayload};

#[path="🫴️projection/📸️snapshot/🦀️.rs"]
mod native_snapshot_projection;
pub use native_snapshot_projection::{OriginalNativeSnapshotFailure,project_original_native_snapshot_failure,key_original_native_snapshot_failure};

#[path="🫴️projection/🔤️json/🦀️.rs"]
mod json_failure_projection;
pub use json_failure_projection::{OriginalJsonFailure,project_original_json_failure,key_original_json_failure};

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
pub use projection::{project_original_turn_failure,key_original_turn_failure,project_original_value_error,key_original_value_error,project_original_fault,key_original_fault,project_original_retirement_failure,key_original_retirement_failure,project_original_retirement_receiving,key_original_retirement_receiving,project_original_receiving_primary,key_original_receiving_primary};
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
/// 🧳️ The complete native refusal or application diagnostic keeps its original ownership.
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
pub enum OriginalTurnPrimary{Value(ValueError),Fault(Fault),Domain(OriginalDomainPrimary),Receiving(OriginalReceivingPrimary),Retirement(OriginalRetirementFailure),RetirementReceiving(OriginalRetirementReceivingPrimary)}
impl OriginalTurnPrimary{
 /// 📊️ Borrows the actual producer receipt independently of its diagnostic representation.
 pub fn retained_progress(&self)->RetainedCloneProgress{match self{Self::Value(error)=>error.retained_progress(),Self::Fault(fault)=>fault.retained_progress(),Self::Domain(original)=>original.retained_progress(),Self::Receiving(original)=>original.retained_progress(),Self::Retirement(original)=>original.retained_progress(),Self::RetirementReceiving(original)=>original.retirement.retained_progress()}}
 /// 🗨️ Borrows the original semantic text without cloning or formatting.
 pub fn message(&self)->&str{match self{Self::Value(error)=>&error.message,Self::Fault(fault)=>&fault.message,Self::Domain(original)=>original.message(),Self::Receiving(original)=>original.message(),Self::Retirement(original)=>original.primary().or(original.work_refusal()).or(original.receipt_refusal()).map_or("Original retirement refused",|cause|cause.message.as_ref()),Self::RetirementReceiving(original)=>original.retirement.primary().or(original.retirement.work_refusal()).or(original.retirement.receipt_refusal()).map_or("Original retirement refused",|cause|cause.message.as_ref())}}
}
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
impl From<OriginalRetirementFailure> for OriginalTurnFailure{fn from(primary:OriginalRetirementFailure)->Self{Self::from(OriginalTurnPrimary::Retirement(primary))}}
impl From<semio_framework_value_native_owned::native_decoding::NativeConstructionStepFailure<ValueError>> for OriginalTurnFailure {
 fn from(failure:semio_framework_value_native_owned::native_decoding::NativeConstructionStepFailure<ValueError>)->Self {
  let primary=OriginalReceivingPrimary {retirement_failure:None,work_refusal:None,work_overrun:None, primary:failure.primary,quote_refusal:None,allocation_refusal:None,allocation_failure:None,native_allocation:None,receipt_refusal:None,progress:failure.progress };
  Self { effects:failure.progress,primary:OriginalTurnPrimary::Receiving(primary),receipt_refusal:failure.receipt_refusal,context_refusal:None,work_refusal:None }
 }
}
impl From<semio_framework_value_native_owned::native_decoding::NativeConstructionStepFailure<OriginalReceivingPrimary>> for OriginalTurnFailure {
 fn from(failure:semio_framework_value_native_owned::native_decoding::NativeConstructionStepFailure<OriginalReceivingPrimary>)->Self {
  let mut primary=failure.primary.unwrap_or(OriginalReceivingPrimary {retirement_failure:None,work_refusal:None,work_overrun:None, primary:None,quote_refusal:None,allocation_refusal:None,allocation_failure:None,native_allocation:None,receipt_refusal:None,progress:failure.progress });primary.progress=failure.progress;
  Self { effects:failure.progress,primary:OriginalTurnPrimary::Receiving(primary),receipt_refusal:failure.receipt_refusal,context_refusal:None,work_refusal:None }
 }
}


impl From<OriginalTurnPrimary> for OriginalTurnFailure{fn from(primary:OriginalTurnPrimary)->Self{Self{effects:primary.retained_progress(),primary,receipt_refusal:None,context_refusal:None,work_refusal:None}}}

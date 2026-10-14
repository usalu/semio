//! 🧯️ Original turn failures retain semantic and receipt causes as separate complete owners.
use super::{StepContext,ValueError};
use semio_framework_diagnostic::Fault;

/// 🧳️ The complete native refusal or application diagnostic keeps its original ownership.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum OriginalTurnPrimary{Value(ValueError),Fault(Fault)}
impl OriginalTurnPrimary{
 /// 📊️ Borrows the actual producer receipt independently of its diagnostic representation.
 pub fn retained_progress(&self)->super::RetainedCloneProgress{match self{Self::Value(error)=>error.retained_progress(),Self::Fault(fault)=>fault.retained_progress()}}
 /// 🗨️ Borrows the original semantic text without cloning or formatting.
 pub fn message(&self)->&str{match self{Self::Value(error)=>&error.message,Self::Fault(fault)=>&fault.message}}
}
impl From<ValueError> for OriginalTurnPrimary{fn from(primary:ValueError)->Self{Self::Value(primary)}}
impl From<Fault> for OriginalTurnPrimary{fn from(primary:Fault)->Self{Self::Fault(primary)}}

/// 🪢️ Original semantic, receipt and context refusals move together without cloning or prose conversion.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct OriginalTurnFailure{pub primary:OriginalTurnPrimary,pub receipt_refusal:Option<ValueError>,pub context_refusal:Option<ValueError>}
impl OriginalTurnFailure{
 /// 🧾️ Records the actual failed child effects while keeping its primary and any receipt refusal.
 pub fn debit_original(primary:impl Into<OriginalTurnPrimary>,cx:&mut StepContext<'_>)->Self{let primary=primary.into();Self{receipt_refusal:cx.consume_retained(primary.retained_progress()).err(),context_refusal:None,primary}}
 /// 🫴️ Lends only the original native refusal when that is the retained primary.
 pub fn primary_value(&self)->Option<&ValueError>{match &self.primary{OriginalTurnPrimary::Value(error)=>Some(error),OriginalTurnPrimary::Fault(_)=>None}}
 /// 🫱️ Lends the complete original diagnostic without weakening its owned fields.
 pub fn primary_fault(&self)->Option<&Fault>{match &self.primary{OriginalTurnPrimary::Fault(fault)=>Some(fault),OriginalTurnPrimary::Value(_)=>None}}
}
impl From<ValueError> for OriginalTurnFailure{
 fn from(primary:ValueError)->Self{Self{primary:primary.into(),receipt_refusal:None,context_refusal:None}}
}
impl From<Fault> for OriginalTurnFailure{fn from(primary:Fault)->Self{Self{primary:primary.into(),receipt_refusal:None,context_refusal:None}}}


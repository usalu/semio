//! 🖥️ Actual backend instructions remain independent of original guest logical work.
use super::{RetainedTurnInput,OriginalJobBudget,ValueError,ValueRefusalKind};
use semio_framework_value_derive::{FromValue,ToValue};
/// 🎟️ Only the embedding supplier authorizes this instruction fuel for the exact original job crossing.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,FromValue,ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase",crate="::protocol::value")]
pub struct OriginalGuestExecutionBudget{pub input:RetainedTurnInput,pub fuel:u64,pub deadline_us:u64}
impl OriginalGuestExecutionBudget{
 pub fn validate_original(self,input:RetainedTurnInput,deadline_us:u64)->Result<(),ValueError>{self.input.validate()?;input.validate()?;if self.input!=input||self.deadline_us!=deadline_us{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original execution authorization differs from its original crossing"))}Ok(())}
 pub fn validate_for(self,guest:OriginalJobBudget)->Result<(),ValueError>{self.validate_original(guest.input,guest.deadline_us)}
}
/// 🧾️ Missing measurement remains explicit; an unsubmitted source can return without guest effects.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,FromValue,ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase",crate="::protocol::value")]
pub struct OriginalGuestExecutionReceipt{pub budget:OriginalGuestExecutionBudget,pub spent_fuel:Option<u64>,pub submitted:bool}
impl OriginalGuestExecutionReceipt{
 pub fn validate_for(self,budget:OriginalGuestExecutionBudget)->Result<(),ValueError>{if self.budget!=budget||self.spent_fuel.is_none()||self.spent_fuel.is_some_and(|spent|spent>budget.fuel)||(!self.submitted&&self.spent_fuel!=Some(0)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"actual execution receipt differs from its original authorization or measurement"))}Ok(())}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

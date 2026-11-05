//! 🖥️ Actual backend instructions remain independent of original guest logical work.
use super::{RetainedTurnInput,OriginalJobBudget,ValueError,ValueRefusalKind};
use semio_framework_value_derive::{FromValue,ToValue};
/// 🎟️ Only the embedding supplier authorizes this instruction fuel for the exact original job crossing.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,FromValue,ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase",crate="::semio_framework_value")]
pub struct OriginalGuestExecutionBudget{pub input:RetainedTurnInput,pub fuel:u64,pub deadline_us:u64}
impl OriginalGuestExecutionBudget{
 pub fn validate_original(self,input:RetainedTurnInput,deadline_us:u64)->Result<(),ValueError>{self.input.validate()?;input.validate()?;if self.input!=input||self.deadline_us!=deadline_us{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original execution authorization differs from its original crossing"))}Ok(())}
 pub fn validate_for(self,guest:OriginalJobBudget)->Result<(),ValueError>{self.validate_original(guest.input,guest.deadline_us)}
 /// 🔽️ A supplier may narrow a live VM authorization after actual caller work without changing its Source, epoch or clock.
 pub fn validate_narrowing(self,issued:Self)->Result<(),ValueError>{self.input.validate()?;issued.input.validate()?;let current=self.input;let original=issued.input;let a=current.grant;let b=original.grant;if (current.operation,current.generation,current.epoch,current.work.clock_binding,current.work.absolute_deadline_us)!=(original.operation,original.generation,original.epoch,original.work.clock_binding,original.work.absolute_deadline_us)||current.work.fuel>original.work.fuel||a.maximum_items>b.maximum_items||a.maximum_copy_bytes>b.maximum_copy_bytes||a.maximum_capacity_bytes>b.maximum_capacity_bytes||a.maximum_release_bytes>b.maximum_release_bytes||a.maximum_depth>b.maximum_depth||self.fuel>issued.fuel||self.deadline_us>issued.deadline_us{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original VM refinement changes its Source or increases its issued authority"))}Ok(())}
}
/// 🧾️ Missing measurement remains explicit; an unsubmitted source can return without guest effects.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,FromValue,ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase",crate="::semio_framework_value")]
pub struct OriginalGuestExecutionReceipt{pub budget:OriginalGuestExecutionBudget,pub spent_fuel:Option<u64>,pub submitted:bool}
impl OriginalGuestExecutionReceipt{
 pub fn validate_for(self,budget:OriginalGuestExecutionBudget)->Result<(),ValueError>{if self.budget!=budget||self.spent_fuel.is_none()||self.spent_fuel.is_some_and(|spent|spent>budget.fuel)||(!self.submitted&&self.spent_fuel!=Some(0)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"actual execution receipt differs from its original authorization or measurement"))}Ok(())}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

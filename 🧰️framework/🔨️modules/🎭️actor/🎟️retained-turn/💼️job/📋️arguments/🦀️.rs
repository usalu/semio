//! 📋️ Original job totals and the held loan cross as fixed stack scalars.
use super::{OriginalJobBudget,RetainedTurnInput,OriginalActorTurnWork,RetainedCloneGrant,ValueError,ValueRefusalKind};
impl OriginalJobBudget{
 pub const SCALAR_ARGUMENT_COUNT:usize=19;
 pub fn scalar_arguments(self)->[u64;Self::SCALAR_ARGUMENT_COUNT]{let grant=self.input.grant;let policy=self.retained_policy;[self.input.operation,self.input.generation,self.input.epoch,self.input.work.clock_binding,self.input.work.absolute_deadline_us,self.input.work.fuel,grant.maximum_items as u64,grant.maximum_copy_bytes as u64,grant.maximum_capacity_bytes as u64,grant.maximum_release_bytes as u64,grant.maximum_depth as u64,policy.maximum_items as u64,policy.maximum_copy_bytes as u64,policy.maximum_capacity_bytes as u64,policy.maximum_release_bytes as u64,policy.maximum_depth as u64,self.fuel,self.deadline_us,u64::from(self.cancelled)]}
 pub fn from_scalar_arguments(arguments:[u64;Self::SCALAR_ARGUMENT_COUNT])->Result<Self,ValueError>{
  if arguments[18]>1{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original job scalar cancellation is not a boolean"))}
  let extent=|index|usize::try_from(arguments[index]).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original job scalar extent exceeds this target address space"));
  let grant=|index|Ok::<_,ValueError>(RetainedCloneGrant{maximum_items:extent(index)?,maximum_copy_bytes:extent(index+1)?,maximum_capacity_bytes:extent(index+2)?,maximum_release_bytes:extent(index+3)?,maximum_depth:extent(index+4)?});
  let input=RetainedTurnInput{operation:arguments[0],generation:arguments[1],epoch:arguments[2],work:OriginalActorTurnWork{clock_binding:arguments[3],absolute_deadline_us:arguments[4],fuel:arguments[5]},grant:grant(6)?};input.validate()?;let policy=grant(11)?;if policy.maximum_items>input.grant.maximum_items||policy.maximum_copy_bytes>input.grant.maximum_copy_bytes||policy.maximum_capacity_bytes>input.grant.maximum_capacity_bytes||policy.maximum_release_bytes>input.grant.maximum_release_bytes||policy.maximum_depth>input.grant.maximum_depth||arguments[16]>input.work.fuel||arguments[17]>input.work.absolute_deadline_us{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job scalar loan exceeds source totals"))}Ok(Self{input,retained_policy:policy,fuel:arguments[16],deadline_us:arguments[17],cancelled:arguments[18]==1})
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

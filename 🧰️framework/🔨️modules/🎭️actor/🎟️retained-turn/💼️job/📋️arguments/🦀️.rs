//! 📋️ Original job authority crosses a core guest as fixed stack scalars without retained JSON metadata.
use super::{OriginalJobBudget,RetainedTurnInput,RetainedCloneGrant,ValueError,ValueRefusalKind};
impl OriginalJobBudget{
 pub fn scalar_arguments(self)->[u64;11]{let grant=self.input.grant;[self.input.operation,self.input.generation,self.input.epoch,grant.maximum_items as u64,grant.maximum_copy_bytes as u64,grant.maximum_capacity_bytes as u64,grant.maximum_release_bytes as u64,grant.maximum_depth as u64,self.fuel,self.deadline_us,u64::from(self.cancelled)]}
 pub fn from_scalar_arguments(arguments:[u64;11])->Result<Self,ValueError>{
  if arguments[10]>1{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original job scalar cancellation is not a boolean"))}
  let extent=|index|usize::try_from(arguments[index]).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original job scalar extent exceeds this target address space"));
  let input=RetainedTurnInput{operation:arguments[0],generation:arguments[1],epoch:arguments[2],grant:RetainedCloneGrant{maximum_items:extent(3)?,maximum_copy_bytes:extent(4)?,maximum_capacity_bytes:extent(5)?,maximum_release_bytes:extent(6)?,maximum_depth:extent(7)?}};input.validate()?;Ok(Self{input,fuel:arguments[8],deadline_us:arguments[9],cancelled:arguments[10]==1})
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

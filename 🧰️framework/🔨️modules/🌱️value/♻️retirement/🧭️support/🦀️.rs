//! 🧭️ Capability traversal borrows the current type path and checks every distinct sibling without allocating.
use super::RetireOwned;
use std::any::TypeId;
/// 🌿️ One borrowed type frame cuts recursive ownership edges while preserving sibling capability checks.
pub struct RetirementSupport<'a>{owner:TypeId,parent:Option<&'a RetirementSupport<'a>>}
impl RetirementSupport<'_>{
 pub fn supports<T:RetireOwned>(&self)->bool{let owner=TypeId::of::<T>();let mut cursor=Some(self);while let Some(frame)=cursor{if frame.owner==owner{return true}cursor=frame.parent}T::controlled_retirement_supported_in(&RetirementSupport{owner,parent:Some(self)})}
}
/// 🌳️ Each query owns only its stack frame; repeated queries share no mutable state or cache.
pub fn controlled_retirement_supported<T:RetireOwned>()->bool{T::controlled_retirement_supported_in(&RetirementSupport{owner:TypeId::of::<T>(),parent:None})}

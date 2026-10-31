//! 🧭️ Capability traversal borrows the current type path and checks every distinct sibling without allocating.
use super::RetireOwned;
use std::any::TypeId;
/// 🌿️ One borrowed type frame cuts recursive ownership edges while preserving sibling capability checks.
pub struct RetirementSupport<'a>{owner:TypeId,parent:Option<&'a RetirementSupport<'a>>}
impl RetirementSupport<'_>{
 pub fn supports<T:RetireOwned>(&self)->bool{let owner=TypeId::of::<T>();let mut cursor=Some(self);while let Some(frame)=cursor{if frame.owner==owner{return true}cursor=frame.parent}T::controlled_retirement_supported_in(&RetirementSupport{owner,parent:Some(self)})}
}
/// 🔁️ Path-less nested queries on the same thread inherit the active type chain, so generic containers cannot restart a recursive ownership cycle.
pub fn controlled_retirement_supported<T:RetireOwned>()->bool{
 let owner=TypeId::of::<T>();
 ACTIVE.with(|head|{
  let mut cursor=head.get();
  while !cursor.is_null(){let frame=unsafe{&*cursor};if frame.owner==owner{return true}cursor=frame.previous;}
  let frame=Frame{owner,previous:head.get()};
  let restore=Restore{head,previous:frame.previous};
  head.set(std::ptr::from_ref(&frame));
  let result=T::controlled_retirement_supported_in(&RetirementSupport{owner,parent:None});
  drop(restore);
  result
 })
}
struct Frame{owner:TypeId,previous:*const Frame}
thread_local!{static ACTIVE:std::cell::Cell<*const Frame>=const{std::cell::Cell::new(std::ptr::null())};}
struct Restore<'a>{head:&'a std::cell::Cell<*const Frame>,previous:*const Frame}
impl Drop for Restore<'_>{fn drop(&mut self){self.head.set(self.previous);}}

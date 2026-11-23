//! 🧭️ Capability traversal borrows the current type path and checks every distinct sibling without allocating.
use super::RetireOwned;
use std::any::TypeId;
/// 🌿️ One borrowed type frame cuts recursive ownership edges while preserving sibling capability checks.
pub struct RetirementSupport<'a>{owner:TypeId,parent:Option<&'a RetirementSupport<'a>>}
impl RetirementSupport<'_>{
 pub fn supports<T:RetireOwned>(&self)->bool{let owner=TypeId::of::<T>();let mut cursor=Some(self);while let Some(frame)=cursor{if frame.owner==owner{return true}cursor=frame.parent}T::controlled_retirement_supported_in(&RetirementSupport{owner,parent:Some(self)})}
}
/// 🔁️ Path-less nested queries on the same thread inherit the active type chain, so generic containers cannot restart a recursive ownership cycle.
///
/// 🧠️ A verdict is a constant of the type, so the outermost traversal of each type runs once per thread and every later birth reads a fixed direct-mapped memo that never allocates; a nested traversal can depend on the active cycle chain and is never memoized.
pub fn controlled_retirement_supported<T:RetireOwned>()->bool{
 let owner=TypeId::of::<T>();
 ACTIVE.with(|head|{
  let outermost=head.get().is_null();
  if outermost{if let Some(verdict)=recall(owner){return verdict}}
  let mut cursor=head.get();
  while !cursor.is_null(){let frame=unsafe{&*cursor};if frame.owner==owner{return true}cursor=frame.previous;}
  let frame=Frame{owner,previous:head.get()};
  let restore=Restore{head,previous:frame.previous};
  head.set(std::ptr::from_ref(&frame));
  let result=T::controlled_retirement_supported_in(&RetirementSupport{owner,parent:None});
  drop(restore);
  if outermost{remember(owner,result)}
  result
 })
}
struct Frame{owner:TypeId,previous:*const Frame}
thread_local!{static ACTIVE:std::cell::Cell<*const Frame>=const{std::cell::Cell::new(std::ptr::null())};}
const MEMO_SLOTS:usize=64;
thread_local!{static VERDICTS:[std::cell::Cell<Option<(TypeId,bool)>>;MEMO_SLOTS]=const{[const{std::cell::Cell::new(None)};MEMO_SLOTS]};}
fn memo_slot(owner:TypeId)->usize{use std::hash::{Hash,Hasher};let mut hasher=std::collections::hash_map::DefaultHasher::new();owner.hash(&mut hasher);hasher.finish()as usize%MEMO_SLOTS}
fn recall(owner:TypeId)->Option<bool>{VERDICTS.with(|memo|memo[memo_slot(owner)].get().filter(|(known,_)|*known==owner).map(|(_,verdict)|verdict))}
fn remember(owner:TypeId,verdict:bool){VERDICTS.with(|memo|memo[memo_slot(owner)].set(Some((owner,verdict))))}
struct Restore<'a>{head:&'a std::cell::Cell<*const Frame>,previous:*const Frame}
impl Drop for Restore<'_>{fn drop(&mut self){self.head.set(self.previous);}}

//! 🪢️ One audited, step-scoped lend of a turn's effect to polled futures that only receive a `Context`.
use super::{RetainedAllocationEffect,ValueError,ValueRefusalKind};
use std::sync::atomic::{AtomicPtr,Ordering};

fn absent()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"no turn effect is lent to this step")}

/// 🪙️ Resident beside a polled owner: empty between turns, holding the turn's effect only while its guard lives.
pub struct LentEffectSlot{lent:AtomicPtr<RetainedAllocationEffect<'static>>}
impl Default for LentEffectSlot{fn default()->Self{Self::new()}}
impl LentEffectSlot{
 /// 🆕️ Creates an empty slot: every `with_effect` is refused until a turn lends its effect.
 pub const fn new()->Self{Self{lent:AtomicPtr::new(std::ptr::null_mut())}}
 /// 🫴️ Lends the turn's own effect for exactly the guard's lifetime; a second lend is refused, never replaced.
 pub fn lend<'slot,'effect,'marker>(&'slot self,effect:&'effect mut RetainedAllocationEffect<'marker>)->Result<EffectLend<'slot,'effect,'marker>,ValueError>{
  let pointer=(&mut *effect as *mut RetainedAllocationEffect<'marker>).cast::<RetainedAllocationEffect<'static>>();
  if self.lent.compare_exchange(std::ptr::null_mut(),pointer,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"a turn effect is already lent to this step"))}
  Ok(EffectLend{slot:self,_effect:effect})
 }
 /// 🪙️ Runs `run` on the lent effect; with no lend, or re-entered from inside `run`, the call is refused typed and runs nothing.
 pub fn with_effect<R>(&self,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Result<R,ValueError>{
  let pointer=self.lent.swap(std::ptr::null_mut(),Ordering::AcqRel);
  if pointer.is_null(){return Err(absent())}
  let restore=Restore{slot:self,pointer};
  let result=run(unsafe{&mut *pointer});
  drop(restore);Ok(result)
 }
}
struct Restore<'slot>{slot:&'slot LentEffectSlot,pointer:*mut RetainedAllocationEffect<'static>}
impl Drop for Restore<'_>{fn drop(&mut self){let _=self.slot.lent.compare_exchange(std::ptr::null_mut(),self.pointer,Ordering::AcqRel,Ordering::Acquire);}}

/// 🛡️ Exclusive lend of one turn's effect; dropping it empties the slot, so no later poll can reach a dead effect.
pub struct EffectLend<'slot,'effect,'marker>{slot:&'slot LentEffectSlot,_effect:&'effect mut RetainedAllocationEffect<'marker>}
impl Drop for EffectLend<'_,'_,'_>{fn drop(&mut self){self.slot.lent.store(std::ptr::null_mut(),Ordering::Release);}}

#[cfg(test)]
mod tests{
 use super::*;
 use super::super::{AuthoringWallet,RetainedCloneGrant};
 fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:64,maximum_capacity_bytes:64,maximum_release_bytes:64,maximum_depth:4}}
 #[test]
 fn empty_slot_refuses_and_lend_funds_one_charge(){
  let slot=LentEffectSlot::new();
  assert!(slot.with_effect(|_|()).is_err());
  let mut wallet=AuthoringWallet::new(grant(),8);
  let done=wallet.run(grant(),4,|effect|{
   let lend=slot.lend(effect).unwrap();
   slot.with_effect(|lent|{assert!(lent.performed(2).unwrap());assert!(slot.with_effect(|_|()).is_err());}).unwrap();
   drop(lend);
   assert!(slot.with_effect(|_|()).is_err());
  }).unwrap().unwrap();
  assert_eq!(done.performed,2);
  assert_eq!(wallet.remaining_work(),6);
 }
}

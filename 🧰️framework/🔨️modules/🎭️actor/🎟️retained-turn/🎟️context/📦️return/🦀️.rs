//! 📮️ Actor-local owners return to their exact pinned Source-funded home before settlement.
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::{ManuallyDrop,size_of};
fn refused()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"actor return differs from its actual Source or pinned donor")}
/// 📮️ One Source prepays the exact inline home replacement and the exact caller donor reset.
#[repr(C)]
struct ActorReturnTarget<T>{owner:Option<T>,written:bool,borrowed:bool}
pub struct OriginalActorReturnSlot<T>{target:ManuallyDrop<ActorReturnTarget<T>>,home:ManuallyDrop<semio_framework_job::OriginalCallerReturnPlace>,donor:ManuallyDrop<semio_framework_job::OriginalCallerReturnPlace>,binding:Option<OriginalSourceBinding>,address:usize,donor_address:usize}
impl<T> OriginalActorReturnSlot<T>{
 pub const fn empty()->Self{Self{target:ManuallyDrop::new(ActorReturnTarget{owner:None,written:false,borrowed:false}),home:ManuallyDrop::new(semio_framework_job::OriginalCallerReturnPlace::empty()),donor:ManuallyDrop::new(semio_framework_job::OriginalCallerReturnPlace::empty()),binding:None,address:0,donor_address:0}}
 pub fn peek(&self)->Option<&T>{if self.target.borrowed{None}else{self.target.owner.as_ref()}}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.address!=0&&self.address!=self as*const Self as usize||self.binding.is_some_and(|binding|!binding.matches(cx)){return Err(refused().into())}Ok(())}
 fn pay(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if (if cx.original_retirement_active(){cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0||cx.fuel_remaining()==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
 fn prepare(place:&mut semio_framework_job::OriginalCallerReturnPlace,address:usize,bytes:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !place.is_admitted()&&!place.admit_sized(cx,address,bytes)?{return Ok(false)}place.qualify_sized(cx,address,bytes)?;if !place.capture_is_preissued()&&!place.preissue_capture(cx,address,bytes)?{return Ok(false)}Ok(true)}
 /// 🎟️ An empty home is still an acquired loan; all owner placement follows the final fallible receipt.
 pub fn lend_step(&mut self,out:&mut Option<T>,acquired:&mut bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if *acquired||out.is_some()||self.target.borrowed{return Err(refused().into())}if self.target.written&&!self.settle_step(cx)?{return Ok(false)}let address=self as*mut Self as usize;if self.address==0{if !Self::pay(cx,size_of::<Option<OriginalSourceBinding>>()+size_of::<usize>())?{return Ok(false)}self.address=address;self.binding=Some(OriginalSourceBinding::borrow(cx));}let donor=out as*mut Option<T> as usize;if self.donor_address==0{if !Self::pay(cx,size_of::<usize>())?{return Ok(false)}self.donor_address=donor;}if donor!=self.donor_address{return Err(refused().into())}let target=&*self.target as*const ActorReturnTarget<T> as usize;if !Self::prepare(&mut self.home,target,size_of::<ActorReturnTarget<T>>(),cx)?||!Self::prepare(&mut self.donor,donor,size_of::<Option<T>>(),cx)?{return Ok(false)}if !Self::pay(cx,size_of::<ActorReturnTarget<T>>()+size_of::<Option<T>>()+size_of::<bool>())?{return Ok(false)}let old=std::mem::replace(&mut*self.target,ActorReturnTarget{owner:None,written:false,borrowed:true});*out=old.owner;*acquired=true;Ok(true)}
 /// ⚡️ Checks every refusal before the two preissued exact destination writes.
 fn validate_return(&self,out:&Option<T>)->Result<(),ValueError>{if self.address!=self as*const Self as usize||self.donor_address!=out as*const Option<T> as usize||!self.target.borrowed||self.target.written||self.target.owner.is_some()||!self.home.capture_is_preissued()||!self.donor.capture_is_preissued(){return Err(refused())}Ok(())}
 pub fn return_owner(&mut self,out:&mut Option<T>)->Result<(),ValueError>{self.validate_return(out)?;*self.target=ActorReturnTarget{owner:out.take(),written:true,borrowed:false};Ok(())}
 /// 📤️ The final local observer bit follows every fallible Source and destination check.
 pub fn return_owner_step(&mut self,out:&mut Option<T>,written:&mut bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;self.validate_return(out)?;if *written{return Err(refused().into())}if !Self::pay(cx,size_of::<bool>())?{return Ok(false)}*self.target=ActorReturnTarget{owner:out.take(),written:true,borrowed:false};*written=true;Ok(true)}
 fn settle_place(place:&mut semio_framework_job::OriginalCallerReturnPlace,address:usize,bytes:usize,written:bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if place.terminal_is_empty(){return Ok(true)}place.qualify_sized(cx,address,bytes)?;if !written&&place.capture_is_preissued()&&!place.cancel_preissued_capture(cx,address,bytes)?{return Ok(false)}place.settle_return(cx)}
 /// 🧾️ Preissued capture alone is not proof that the inline owner was actually written.
 pub fn settle_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.terminal_is_empty(){return Ok(true)}self.validate(cx)?;if self.target.borrowed{return Ok(false)}let target=&*self.target as*const ActorReturnTarget<T> as usize;if !Self::settle_place(&mut self.home,target,size_of::<ActorReturnTarget<T>>(),self.target.written,cx)?||!Self::settle_place(&mut self.donor,self.donor_address,size_of::<Option<T>>(),self.target.written,cx)?{return Ok(false)}let empty=self.target.owner.is_none();if !Self::pay(cx,size_of::<bool>()+size_of::<usize>()+if empty{size_of::<Option<OriginalSourceBinding>>()+size_of::<usize>()}else{0})?{return Ok(false)}self.target.written=false;self.donor_address=0;if empty{self.binding=None;self.address=0;}Ok(true)}
 pub fn terminal_is_empty(&self)->bool{self.target.owner.is_none()&&!self.target.borrowed&&!self.target.written&&self.home.terminal_is_empty()&&self.donor.terminal_is_empty()&&self.binding.is_none()&&self.address==0&&self.donor_address==0}
}
impl<T> Drop for OriginalActorReturnSlot<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"actor inline return requires actual same-Source observation and controlled owner retirement")}}

#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
 use semio_framework_value::RetainedCloneGrant;
 fn clock()->Option<u64>{Some(1)}
 #[test]
 fn inline_return_uses_actual_source_claims_and_preserves_empty_loan_and_foreign_failure(){
  let grant=RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128};
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(681),Generation(1),grant,clock,1000000,100000);
  let mut foreign=OriginalJobFixtureCaller::admit(OperationId(682),Generation(1),grant,clock,1000000,100000);
  let mut slot=OriginalActorReturnSlot::<u64>::empty();let mut out=None;let mut acquired=false;
  assert!(!caller.turn(RetainedCloneGrant{maximum_items:0,..grant},|cx|slot.lend_step(&mut out,&mut acquired,cx)).unwrap());assert!(!acquired);assert!(slot.terminal_is_empty());
  for _ in 0..64{if caller.turn(grant,|cx|slot.lend_step(&mut out,&mut acquired,cx)).unwrap(){break}}assert!(acquired&&out.is_none());assert!(slot.peek().is_none());
  let mut wrong=Some(9);assert!(slot.return_owner(&mut wrong).is_err());assert_eq!(wrong,Some(9));out=Some(7);let mut written=false;assert!(foreign.turn(grant,|cx|slot.return_owner_step(&mut out,&mut written,cx)).is_err());assert_eq!(out,Some(7));assert!(!written);assert_eq!(foreign.receipt(),Default::default());assert!(!caller.turn(RetainedCloneGrant{maximum_items:0,..grant},|cx|slot.return_owner_step(&mut out,&mut written,cx)).unwrap());assert_eq!(out,Some(7));assert!(!written);assert_eq!(caller.receipt(),Default::default());assert!(caller.turn(grant,|cx|slot.return_owner_step(&mut out,&mut written,cx)).unwrap());assert!(written);assert!(out.is_none());assert_eq!(slot.peek(),Some(&7));assert!(slot.return_owner(&mut out).is_err());
  assert!(foreign.turn(grant,|cx|slot.settle_step(cx)).is_err());assert_eq!(slot.peek(),Some(&7));
  for _ in 0..64{if caller.turn(grant,|cx|slot.settle_step(cx)).unwrap(){break}}
  acquired=false;for _ in 0..64{if caller.turn(grant,|cx|slot.lend_step(&mut out,&mut acquired,cx)).unwrap(){break}}assert!(acquired);assert_eq!(out,Some(7));caller.turn(grant,|cx|{cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<u64>>(),..Default::default()})?;cx.consume_fuel(1);out.take();Ok(())}).unwrap();slot.return_owner(&mut out).unwrap();
  caller.cancel().cancel_now();for _ in 0..64{if slot.terminal_is_empty(){break}caller.turn(grant,|cx|cx.with_original_retirement(|cx|slot.settle_step(cx))).unwrap();}assert!(slot.terminal_is_empty());drop(slot);caller.retire(grant);foreign.retire(grant);
 }
}

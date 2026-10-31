//! ♻️ Exact original History test recipient retirement.
use semio_framework_value::{RetainedCloneGrant, RetirementDemand, admit_owned_retirement, owned_retirement_birth_bytes};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::mem::size_of_val;
fn observe<T>(operation:impl FnOnce()->T)->(T,usize,usize){semio_framework_trace_memory_testing::observe_backing(operation)}
fn permits(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_depth>=d.depth&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes}
pub fn close_original<T:semio_framework_value::retirement::RetireOwned>(original:T,control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->(usize,usize){let birth=owned_retirement_birth_bytes::<T>();assert!(birth<=g.maximum_capacity_bytes);control.charge(birth).unwrap();let(result,born,released)=observe(||admit_owned_retirement(original,g));let(mut owner,p)=result.result.map_err(|(error,_)|error).unwrap();assert!(p.fits(g));assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes));let(mut births,mut releases)=(born,released);for _ in 0..100000{if owner.terminal_is_empty(){let bytes=size_of_val(owner.as_ref());assert!(bytes<=g.maximum_release_bytes);let(_,born,released)=observe(||drop(owner));assert_eq!((born,released),(0,bytes));return(births,releases+released)}let d=RetirementDemand{copy_bytes:owner.next_copy_byte_demand().unwrap(),capacity_bytes:owner.next_capacity_byte_demand(g.maximum_copy_bytes).unwrap(),release_bytes:owner.next_release_byte_demand().unwrap(),depth:owner.next_depth_demand().unwrap()};assert!(permits(g,d));control.charge(d.capacity_bytes).unwrap();let(step,born,released)=observe(||owner.close_step(g).result.unwrap());assert!(step.progress().fits(g));assert_eq!((born,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=born;releases+=released}panic!("original transition recipient retirement failed to terminate")}

/// 🎟️ One explicit finite native work wallet spans all observations of its owning fixture.
pub struct NativeWorkWallet{remaining:u64,quantum:u64}
impl NativeWorkWallet{
 pub fn new(remaining:u64,quantum:u64)->Self{Self{remaining,quantum}}
}
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for NativeWorkWallet{
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,semio_framework_value::ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.remaining.min(self.quantum)))}
}
/// 🔬️ Observes only the actual native counter under the same wallet; it makes no claim about nonnative callback work.
pub fn native_turn<T,E:From<semio_framework_value::ValueError>>(control:&mut NativeDecodeControl<'_>,wallet:&mut NativeWorkWallet,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
 let before=control.actual_work();let result=control.with_work_recipient(wallet,operation);let actual=control.actual_work()-before;assert!(actual<=wallet.remaining.min(wallet.quantum));wallet.remaining-=actual;result
}

//! 🧳️ The opening Source owns its one cold controller through full parent closure.
use super::*;
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::RetainedCloneProgress;
use std::mem::size_of;
#[derive(Default)]
pub struct GuestRelayNativeState{pub(crate) controller:Option<GuestRelayColdController>}
impl NativeInstanceRetirement for GuestRelayNativeState{
 fn close_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,_:&CompiledHandle,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let Some(controller)=self.controller.as_mut()else{return Ok(true)};if !controller.terminal_is_empty(){controller.close_step(runtime,instance,cx)?;return Ok(false)}if !pay(cx)?{return Ok(false)}self.controller.take();Ok(true)
 }
 fn close_unopened(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let Some(controller)=self.controller.as_mut()else{return Ok(true)};if !controller.terminal_is_empty(){controller.close_unopened(cx)?;return Ok(false)}if !pay(cx)?{return Ok(false)}self.controller.take();Ok(true)
 }
 fn terminal_is_empty(&self)->bool{self.controller.is_none()}
}
fn pay(cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let bytes=size_of::<Option<GuestRelayColdController>>();let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}

#[cfg(test)]
mod tests{
 use super::*;use semio_framework_job::{OriginalJobFixtureCaller,OriginalNativeRetirement,OperationId,Generation};use semio_framework_value::RetainedCloneGrant;use std::sync::atomic::{AtomicU64,Ordering};
 static SEEN:AtomicU64=AtomicU64::new(0);static COUNT:AtomicU64=AtomicU64::new(0);
 fn clock()->Option<u64>{Some(1000)}
 fn resolve(identity:u64,_:[u8;32],_:&mut dyn FnMut(&GuestRuntimes,&CompiledHandle)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{SEEN.store(identity,Ordering::SeqCst);Ok(false)}
 fn custody(identity:u64,_:[u8;32],held:&mut bool,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<u64>()+size_of::<bool>(),..Default::default()})?;cx.consume_fuel(1);SEEN.store(identity,Ordering::SeqCst);if *held{COUNT.fetch_sub(1,Ordering::SeqCst);}else{COUNT.fetch_add(1,Ordering::SeqCst);}*held=!*held;Ok(())}
 fn failed_acquire(identity:u64,hash:[u8;32],held:&mut bool,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{custody(identity,hash,held,cx)?;Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"after acquire").into())}
 #[test]
 fn native_origin_uses_nonzero_cache_identity_even_for_identical_component_hashes(){
  let law:serde_json::Value=serde_json::from_str(include_str!("origin.json")).unwrap();let grant=RetainedCloneGrant{maximum_items:1000,maximum_copy_bytes:65536,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};let mut accepted=std::collections::BTreeSet::new();
  for row in law["cases"].as_array().unwrap(){let identity=row["identity"].as_u64().unwrap();let hash=[row["hash"].as_u64().unwrap()as u8;32];let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity+800),Generation(1),grant,clock,1000000,1000000);let mut home=NativeInstanceHome::opening(GuestRelayNativeOrigin{cache_identity:identity,package_hash:hash,acquire:custody,resolve,release:custody},GuestRelayNativeState::default());SEEN.store(0,Ordering::SeqCst);let valid=caller.turn(grant,|cx|Ok(home.open_step(RuntimeActorId(1),None,cx).result.is_ok())).unwrap();assert_eq!(valid,row["accepted"].as_bool().unwrap());assert_eq!(SEEN.load(Ordering::SeqCst),identity);if valid{accepted.insert((identity,hash));}for _ in 0..64{if caller.turn(grant,|cx|cx.with_original_retirement(|cx|home.close_on_source(cx))).unwrap(){break}}assert!(home.terminal_is_empty());assert_eq!(COUNT.load(Ordering::SeqCst),0);caller.retire(grant);}
  assert_eq!(accepted.len(),law["distinctAcceptedOrigins"].as_u64().unwrap()as usize);let mut caller=OriginalJobFixtureCaller::admit(OperationId(900),Generation(1),grant,clock,1000000,1000000);let mut home=NativeInstanceHome::opening(GuestRelayNativeOrigin{cache_identity:99,package_hash:[7;32],acquire:failed_acquire,resolve,release:custody},GuestRelayNativeState::default());assert!(caller.turn(grant,|cx|Ok(home.open_step(RuntimeActorId(1),None,cx).result.is_err())).unwrap());assert_eq!(COUNT.load(Ordering::SeqCst),1);for _ in 0..64{if caller.turn(grant,|cx|cx.with_original_retirement(|cx|home.close_on_source(cx))).unwrap(){break}}assert!(home.terminal_is_empty());assert_eq!(COUNT.load(Ordering::SeqCst),0);caller.retire(grant);
 }
}

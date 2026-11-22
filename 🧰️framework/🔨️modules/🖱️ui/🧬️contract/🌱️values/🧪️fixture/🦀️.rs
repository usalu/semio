//! 🧪️ A serialized native test scope opens and returns the real finite UI arena Source.
use super::*;
use semio_framework_job::{Generation,OperationId,OriginalNativeLease,OriginalTurnFailure,StepContext};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
use std::panic::{catch_unwind,AssertUnwindSafe};
use std::cell::Cell;
use std::sync::{Mutex,MutexGuard};

thread_local!{static OWNED:Cell<bool>=const{Cell::new(false)}}

static SERIAL:Mutex<()>=Mutex::new(());
fn clock()->Option<u64>{Some(1)}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:67_108_864,maximum_capacity_bytes:67_108_864,maximum_release_bytes:67_108_864,maximum_depth:64}}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:usize::MAX>>2,maximum_copy_bytes:usize::MAX>>2,maximum_capacity_bytes:usize::MAX>>2,maximum_release_bytes:usize::MAX>>2,maximum_depth:64}}
/// ⏳️ The real monotonic ceiling of one test scope: its Source clock declines construction past it, so it must outlast the slowest law on a loaded host while staying finite.
const SCOPE_DEADLINE_MICROS:u64=600_000_000;
/// 🔐️ Serializes every law of this process that touches the process-wide arena registry or its handbacks.
pub fn ui_arena_serial()->MutexGuard<'static,()>{SERIAL.lock().unwrap_or_else(|error|error.into_inner())}
/// 🏟️ One test owns this arena until every registered value and its actual Source is returned; a panicking test still closes it so the process-wide registry is free for the next test, and only a refused close leaks it.
pub struct UiArenaFixture{caller:Option<OriginalJobFixtureCaller>,lease:Option<OriginalNativeLease<UiArenaLifetime>>,_serial:Option<MutexGuard<'static,()>>}
impl UiArenaFixture{
 /// 🪆️ A scope opened while this thread's test already owns the arena is a handle onto that owner, so fixtures and a law's own scope compose without deadlocking on the serial lock; only the owner drives turns.
 pub fn new()->Self{
  if OWNED.with(Cell::get){return Self{caller:None,lease:None,_serial:None}}
  let serial=ui_arena_serial();let stock=stock();let mut caller=OriginalJobFixtureCaller::admit(OperationId(991),Generation(1),stock,clock,SCOPE_DEADLINE_MICROS,1_000_000_000);let mut input=Some(UiArenaLifetime::new(ui_value_arena_registry()));let mut lease=None;
  for _ in 0..100{if caller.turn(stock,|cx|UiArenaLifetime::admit_source(&mut input,&mut lease,cx)).unwrap(){break}}assert!(input.is_none()&&lease.is_some());
  for _ in 0..100_000{if caller.turn(stock,|cx|UiArenaLifetime::advance_on_source(lease.as_mut().unwrap(),cx)).unwrap(){break}}assert!(ui_value_arena_registry().borrow().unwrap().is_some());OWNED.with(|owned|owned.set(true));Self{caller:Some(caller),lease,_serial:Some(serial)}
 }
 /// 🧵️ Runs one funded turn of this scope's own original caller, so a test retires its owners under the very Source that admitted the arena.
 pub fn turn<R>(&mut self,work:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{self.caller.as_mut().expect("only the owning arena scope drives turns").turn(grant(),work)}
 /// 🛡️ Recovers a poisoned borrow through this same Source-owned registration.
 pub fn recover(&mut self){let caller=self.caller.as_mut().unwrap();let lease=self.lease.as_mut().unwrap();for _ in 0..100_000{if caller.turn(grant(),|cx|UiArenaLifetime::recover_on_source(lease,cx)).unwrap(){return}}panic!("test arena recovery did not finish")}
}
impl UiArenaFixture{
 fn close(&mut self){
  let stock=stock();let caller=self.caller.as_mut().unwrap();for _ in 0..100_000{let done=caller.turn(stock,|cx|{let grant=cx.retained_grant();let bytes=4096.min(grant.maximum_copy_bytes);if grant.maximum_items==0||bytes==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);close_ui_value_page_with_grant(1,bytes).map(|step|step.complete).map_err(|_|semio_framework_value::ValueError::invariant("test arena retirement refused").into())}).unwrap();if done{break}}
  for _ in 0..100_000{if self.lease.is_none(){break}caller.turn(stock,|cx|OriginalNativeLease::return_slot_step(&mut self.lease,cx)).unwrap();}assert!(self.lease.is_none());
  let mut closed=false;for _ in 0..100_000{if caller.turn(stock,|cx|cx.close_original_native_step()).unwrap(){closed=true;break}}assert!(closed,"the test arena Source closes inside its finite opportunity ceiling: stock left {:?}, fuel left {}, registry {:?}",caller.remaining_grant(),caller.fuel_remaining(),ui_value_arena_registry().borrow().map(|arena|arena.is_some()));assert!(ui_value_arena_registry().borrow().is_err());self.lease.take();self.caller.take().unwrap().retire(stock);
 }
}
struct Release;
impl Drop for Release{fn drop(&mut self){OWNED.with(|owned|owned.set(false))}}
impl Drop for UiArenaFixture{
 fn drop(&mut self){
  if self._serial.is_none(){return}let _release=Release;
  if std::thread::panicking(){if catch_unwind(AssertUnwindSafe(||self.close())).is_err(){if let Some(lease)=self.lease.take(){std::mem::forget(lease)}if let Some(caller)=self.caller.take(){std::mem::forget(caller)}}return}
  self.close()
 }
}

#[cfg(test)]
mod nested_laws{
 use super::*;
 #[test]
 fn a_second_scope_on_the_owning_thread_nests_without_deadlock_and_leaves_the_owner_registered(){
  let owner=UiArenaFixture::new();
  drop(UiArenaFixture::new());
  assert!(ui_value_arena_registry().borrow().unwrap().is_some());
  drop(owner);
  assert!(ui_value_arena_registry().borrow().is_err());
  drop(UiArenaFixture::new());
 }
}

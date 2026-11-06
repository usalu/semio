//! 📌️ One unique lifetime owner publishes a stable arena borrow and audits every returned descendant before detachment.
use super::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::retained_clone_grant_admits};
use std::{ops::{Deref,DerefMut},sync::{MutexGuard,TryLockError}};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"UI arena registration no longer witnesses its actual lifetime owner")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
struct RegisteredArena{pointer:usize,epoch:u64,closing:bool}
/// 🛡️ The mutex protects both the unique publication pointer and each live borrowed arena access.
pub struct UiArenaRegistry{state:Mutex<RegisteredArena>}
impl UiArenaRegistry{
 pub const fn new()->Self{Self{state:Mutex::new(RegisteredArena{pointer:0,epoch:0,closing:false})}}
 pub fn borrow(&'static self)->Result<Option<UiArenaBorrow>,ValueError>{let guard=match self.state.try_lock(){Ok(guard)=>guard,Err(TryLockError::WouldBlock)=>return Ok(None),Err(TryLockError::Poisoned(_))=>return Err(invalid())};if guard.pointer==0||guard.closing{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"UI arena has no accepting lifetime registration"))}Ok(Some(UiArenaBorrow{guard}))}
 /// 🔒️ Borrows the same registered backing for an existing synchronous arena operation.
 pub fn borrow_blocking(&'static self)->Result<UiArenaBorrow,ValueError>{let guard=self.state.lock().map_err(|_|invalid())?;if guard.pointer==0||guard.closing{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"UI arena has no accepting lifetime registration"))}Ok(UiArenaBorrow{guard})}
}
pub static UI_ARENA_REGISTRY:UiArenaRegistry=UiArenaRegistry::new();
pub fn ui_value_arena_registry()->&'static UiArenaRegistry{&UI_ARENA_REGISTRY}
pub struct UiArenaBorrow{guard:MutexGuard<'static,RegisteredArena>}
impl Deref for UiArenaBorrow{type Target=UiValueArena;fn deref(&self)->&UiValueArena{unsafe{&*(self.guard.pointer as*const UiValueArena)}}}
impl DerefMut for UiArenaBorrow{fn deref_mut(&mut self)->&mut UiValueArena{unsafe{&mut*(self.guard.pointer as*mut UiValueArena)}}}
/// 🔐️ The source-native lifetime payload owns this non-cloneable registration and the exact stable backing it publishes.
pub struct UiArenaRegistration{backing:UiArenaBacking,registry:&'static UiArenaRegistry,epoch:u64,registered:bool,stage:u8,cursor:usize}
impl UiArenaRegistration{
 pub fn constructor_demand()->RetirementDemand{fixed(size_of::<Self>()+size_of::<RegisteredArena>())}
 pub fn admit(registry:&'static UiArenaRegistry,input:&mut Option<UiArenaBacking>,output:&mut Option<Self>,g:RetainedCloneGrant)->RetainedWorkOutcome{let mut work=0;let result=(||{if output.is_some()||input.is_none(){return Err(invalid())}let d=Self::constructor_demand();if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let mut guard=match registry.state.try_lock(){Ok(guard)=>guard,Err(TryLockError::WouldBlock)=>return Ok(RetainedCloneStep::Progress(Default::default())),Err(TryLockError::Poisoned(_))=>return Err(invalid())};if guard.pointer!=0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"UI arena already has an actual lifetime owner"))}let epoch=guard.epoch.checked_add(1).ok_or_else(invalid)?;let pointer=input.as_ref().ok_or_else(invalid)?.pointer()?;work=1;guard.pointer=pointer;guard.epoch=epoch;guard.closing=false;*output=Some(Self{backing:input.take().ok_or_else(invalid)?,registry,epoch,registered:true,stage:0,cursor:0});Ok(RetainedCloneStep::Complete(receipt(d)))})();RetainedWorkOutcome{result,work}}
 pub fn registered(&self)->bool{self.registered}
 pub fn close_demands(&self)->Result<RetirementDemand,ValueError>{if !self.registered{return self.backing.close_demands()}Ok(fixed(size_of::<RegisteredArena>()+size_of::<(usize,u8,bool)>()+match self.stage{1=>size_of::<UiPageSlot>(),2=>size_of::<UiCollectionSlot>(),3=>size_of::<u64>(),_=>size_of::<UiValueArena>()+UI_VALUE_HANDBACK_WORDS*size_of::<u64>()}))}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{if !self.registered{return self.backing.close_step(g)}let mut work=0;let result=(||{let d=self.close_demands()?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let mut guard=match self.registry.state.try_lock(){Ok(guard)=>guard,Err(TryLockError::WouldBlock)=>return Ok(RetainedCloneStep::Progress(Default::default())),Err(TryLockError::Poisoned(error))=>error.into_inner()};if guard.epoch!=self.epoch||guard.pointer!=self.backing.pointer()?{return Err(invalid())}let arena=self.backing.arena.as_ref().ok_or_else(invalid)?;work=1;
  match self.stage{
   0=>{if arena.free_page_count!=UI_VALUE_AGGREGATE_ITEMS||arena.free_collection_count!=UI_VALUE_ADMISSION_SLOTS||arena.retirement_head!=UI_VALUE_NONE||arena.retirement_tail!=UI_VALUE_NONE||arena.retirement_len!=0||retirement::UI_VALUE_HANDBACKS.has_pending(){return Ok(RetainedCloneStep::Progress(receipt(d)))}guard.closing=true;self.stage=1;},
   1=>{if let Some(page)=arena.pages.get(self.cursor){if page.value.is_some()||page.next!=UI_VALUE_NONE{return Err(invalid().with_retained_progress(receipt(d)))}self.cursor+=1;}else{self.cursor=0;self.stage=2;}},
   2=>{if let Some(row)=arena.collections.get(self.cursor){if row.occupied||row.aliases!=0||row.retirement_parent.is_some()||row.retirement_cursor.is_some(){return Err(invalid().with_retained_progress(receipt(d)))}self.cursor+=1;}else{self.cursor=0;self.stage=3;}},
   3=>{if let Some(word)=retirement::UI_VALUE_HANDBACKS.ready.get(self.cursor){if word.load(std::sync::atomic::Ordering::Acquire)!=0{return Err(invalid().with_retained_progress(receipt(d)))}self.cursor+=1;}else{self.stage=4;}},
   4=>{guard.pointer=0;guard.closing=false;self.registered=false;self.registry.state.clear_poison();},_=>return Err(invalid())}
  Ok(RetainedCloneStep::Progress(receipt(d)))})();RetainedWorkOutcome{result,work}}
 pub fn terminal_is_empty(&self)->bool{!self.registered&&self.backing.terminal_is_empty()}
 pub fn recover_borrow_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{let mut work=0;let result=(||{if !self.registered{return Err(invalid())}let d=Self::constructor_demand();if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let mut guard=match self.registry.state.try_lock(){Ok(guard)=>guard,Err(TryLockError::WouldBlock)=>return Ok(RetainedCloneStep::Progress(Default::default())),Err(TryLockError::Poisoned(error))=>error.into_inner()};if guard.epoch!=self.epoch||guard.pointer!=self.backing.pointer()?{return Err(invalid())}work=1;guard.closing=false;self.stage=0;self.cursor=0;self.registry.state.clear_poison();Ok(RetainedCloneStep::Complete(receipt(d)))})();RetainedWorkOutcome{result,work}}
}
impl Drop for UiArenaRegistration{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"UI arena lifetime registration cannot outlive its actual paid source or physical retirement")}}

//! 📌️ One unique lifetime owner publishes a stable arena borrow and audits every returned descendant before detachment.
use super::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::retained_clone_grant_admits};
use std::{cell::UnsafeCell,marker::PhantomData,ops::{Deref,DerefMut},sync::atomic::{AtomicBool,Ordering}};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"UI arena registration no longer witnesses its actual lifetime owner")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
struct RegisteredArena{pointer:usize,epoch:u64,closing:bool}
/// 🛡️ One allocation-free poisoning lock protects both the unique publication pointer and each live borrowed arena access, so the resident static registry never lazily boxes a platform mutex outside any receipt.
pub struct UiArenaRegistry{held:AtomicBool,poisoned:AtomicBool,state:UnsafeCell<RegisteredArena>}
unsafe impl Sync for UiArenaRegistry{}
impl std::panic::RefUnwindSafe for UiArenaRegistry{}
/// 🗝️ Exclusive access to the registered pointer; a panic while it is held poisons the registry until its owner recovers it.
struct RegistryGuard<'a>{registry:&'a UiArenaRegistry,panicking:bool,_not_send:PhantomData<*mut ()>}
impl Deref for RegistryGuard<'_>{type Target=RegisteredArena;fn deref(&self)->&RegisteredArena{unsafe{&*self.registry.state.get()}}}
impl DerefMut for RegistryGuard<'_>{fn deref_mut(&mut self)->&mut RegisteredArena{unsafe{&mut*self.registry.state.get()}}}
impl Drop for RegistryGuard<'_>{fn drop(&mut self){if !self.panicking&&std::thread::panicking(){self.registry.poisoned.store(true,Ordering::Relaxed)}self.registry.held.store(false,Ordering::Release)}}
impl UiArenaRegistry{
 pub const fn new()->Self{Self{held:AtomicBool::new(false),poisoned:AtomicBool::new(false),state:UnsafeCell::new(RegisteredArena{pointer:0,epoch:0,closing:false})}}
 fn try_lock(&self)->Option<RegistryGuard<'_>>{self.held.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).ok()?;Some(RegistryGuard{registry:self,panicking:std::thread::panicking(),_not_send:PhantomData})}
 fn lock(&self)->RegistryGuard<'_>{loop{if let Some(guard)=self.try_lock(){return guard}std::hint::spin_loop();std::thread::yield_now()}}
 fn is_poisoned(&self)->bool{self.poisoned.load(Ordering::Relaxed)}
 fn clear_poison(&self){self.poisoned.store(false,Ordering::Relaxed)}
 pub fn borrow(&'static self)->Result<Option<UiArenaBorrow>,ValueError>{let Some(guard)=self.try_lock()else{return Ok(None)};if self.is_poisoned(){return Err(invalid())}if guard.pointer==0||guard.closing{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"UI arena has no accepting lifetime registration"))}Ok(Some(UiArenaBorrow{guard}))}
 /// 🔒️ Borrows the same registered backing for an existing synchronous arena operation.
 pub fn borrow_blocking(&'static self)->Result<UiArenaBorrow,ValueError>{let guard=self.lock();if self.is_poisoned(){return Err(invalid())}if guard.pointer==0||guard.closing{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"UI arena has no accepting lifetime registration"))}Ok(UiArenaBorrow{guard})}
}
pub static UI_ARENA_REGISTRY:UiArenaRegistry=UiArenaRegistry::new();
pub fn ui_value_arena_registry()->&'static UiArenaRegistry{&UI_ARENA_REGISTRY}
pub struct UiArenaBorrow{guard:RegistryGuard<'static>}
impl Deref for UiArenaBorrow{type Target=UiValueArena;fn deref(&self)->&UiValueArena{unsafe{&*(self.guard.pointer as*const UiValueArena)}}}
impl DerefMut for UiArenaBorrow{fn deref_mut(&mut self)->&mut UiValueArena{unsafe{&mut*(self.guard.pointer as*mut UiValueArena)}}}
/// 🔐️ The source-native lifetime payload owns this non-cloneable registration and the exact stable backing it publishes.
pub struct UiArenaRegistration{backing:UiArenaBacking,registry:&'static UiArenaRegistry,epoch:u64,registered:bool,stage:u8,cursor:usize}
impl UiArenaRegistration{
 pub fn constructor_demand()->RetirementDemand{fixed(size_of::<Option<Self>>()+size_of::<Option<UiArenaBacking>>()+size_of::<usize>()+size_of::<u64>()+size_of::<bool>())}
 pub fn admit(registry:&'static UiArenaRegistry,input:&mut Option<UiArenaBacking>,output:&mut Option<Self>,g:RetainedCloneGrant)->RetainedWorkOutcome{let mut work=0;let result=(||{if output.is_some()||input.is_none(){return Err(invalid())}let d=Self::constructor_demand();if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let Some(mut guard)=registry.try_lock()else{return Ok(RetainedCloneStep::Progress(Default::default()))};if registry.is_poisoned(){return Err(invalid())}if guard.pointer!=0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"UI arena already has an actual lifetime owner"))}let epoch=guard.epoch.checked_add(1).ok_or_else(invalid)?;let pointer=input.as_ref().ok_or_else(invalid)?.pointer()?;work=1;guard.pointer=pointer;guard.epoch=epoch;guard.closing=false;*output=Some(Self{backing:input.take().ok_or_else(invalid)?,registry,epoch,registered:true,stage:0,cursor:0});Ok(RetainedCloneStep::Complete(receipt(d)))})();RetainedWorkOutcome{result,work}}
 pub fn registered(&self)->bool{self.registered}
 pub fn close_demands(&self)->Result<RetirementDemand,ValueError>{if !self.registered{return self.backing.close_demands()}Ok(Self::close_phase_demand(self.stage,match self.stage{1=>self.cursor>=UI_VALUE_AGGREGATE_ITEMS,2=>self.cursor>=UI_VALUE_ADMISSION_SLOTS,3=>self.cursor>=UI_VALUE_HANDBACK_WORDS,_=>false}))}
 fn close_phase_demand(stage:u8,end:bool)->RetirementDemand{fixed(match stage{0=>size_of::<bool>()+size_of::<u8>(),1|2=>size_of::<usize>()+usize::from(end)*size_of::<u8>(),3=>if end{size_of::<u8>()}else{size_of::<usize>()},_=>size_of::<usize>()+2*size_of::<bool>()+size_of::<AtomicBool>()})}
 pub fn closure_reserve()->Result<(usize,RetirementDemand,u64),ValueError>{let mut items=0usize;let mut d=fixed(0);for(stage,count,end)in[(0,1,false),(1,UI_VALUE_AGGREGATE_ITEMS,false),(1,1,true),(2,UI_VALUE_ADMISSION_SLOTS,false),(2,1,true),(3,UI_VALUE_HANDBACK_WORDS,false),(3,1,true),(4,1,false)]{items+=count;d.copy_bytes=d.copy_bytes.checked_add(Self::close_phase_demand(stage,end).copy_bytes.checked_mul(count).ok_or_else(invalid)?).ok_or_else(invalid)?;}d.release_bytes=size_of::<UiValueArena>()+UI_VALUE_AGGREGATE_ITEMS*(size_of::<UiPageSlot>()+size_of::<usize>())+UI_VALUE_ADMISSION_SLOTS*(size_of::<UiCollectionSlot>()+size_of::<usize>());d.copy_bytes+=4*size_of::<Box<[usize]>>()+size_of::<Option<Box<UiValueArena>>>();items+=5;Ok((items,d,items as u64))}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{if !self.registered{return self.backing.close_step(g)}let mut work=0;let result=(||{let d=self.close_demands()?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let Some(mut guard)=self.registry.try_lock()else{return Ok(RetainedCloneStep::Progress(Default::default()))};if guard.epoch!=self.epoch||guard.pointer!=self.backing.pointer()?{return Err(invalid())}let arena=self.backing.arena.as_ref().ok_or_else(invalid)?;work=1;
  match self.stage{
   0=>{if arena.free_page_count!=UI_VALUE_AGGREGATE_ITEMS||arena.free_collection_count!=UI_VALUE_ADMISSION_SLOTS||arena.retirement_head!=UI_VALUE_NONE||arena.retirement_tail!=UI_VALUE_NONE||arena.retirement_len!=0||retirement::UI_VALUE_HANDBACKS.has_pending(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"UI arena descendants remain borrowed at lifetime closure"))}guard.closing=true;self.stage=1;},
   1=>{if let Some(page)=arena.pages.get(self.cursor){if page.value.is_some()||page.next!=UI_VALUE_NONE{return Err(invalid())}self.cursor+=1;}else{self.cursor=0;self.stage=2;}},
   2=>{if let Some(row)=arena.collections.get(self.cursor){if row.occupied||row.aliases!=0||row.retirement_parent.is_some()||row.retirement_cursor.is_some(){return Err(invalid())}self.cursor+=1;}else{self.cursor=0;self.stage=3;}},
   3=>{if let Some(word)=retirement::UI_VALUE_HANDBACKS.ready.get(self.cursor){if word.load(Ordering::Acquire)!=0{return Err(invalid())}self.cursor+=1;}else{self.stage=4;}},
   4=>{guard.pointer=0;guard.closing=false;self.registered=false;self.registry.clear_poison();},_=>return Err(invalid())}
  Ok(RetainedCloneStep::Progress(receipt(d)))})();RetainedWorkOutcome{result,work}}
 pub fn terminal_is_empty(&self)->bool{!self.registered&&self.backing.terminal_is_empty()}
 pub fn recovery_demand()->RetirementDemand{fixed(size_of::<bool>()+size_of::<u8>()+size_of::<usize>()+size_of::<AtomicBool>())}
 pub fn recover_borrow_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{let mut work=0;let result=(||{if !self.registered{return Err(invalid())}let d=Self::recovery_demand();if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let Some(mut guard)=self.registry.try_lock()else{return Ok(RetainedCloneStep::Progress(Default::default()))};if guard.epoch!=self.epoch||guard.pointer!=self.backing.pointer()?{return Err(invalid())}work=1;guard.closing=false;self.stage=0;self.cursor=0;self.registry.clear_poison();Ok(RetainedCloneStep::Complete(receipt(d)))})();RetainedWorkOutcome{result,work}}
}
impl Drop for UiArenaRegistration{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"UI arena lifetime registration cannot outlive its actual paid source or physical retirement")}}

//! 📦️ Shared initialized originals elect one destructive owner without moving the complete payload.
use super::shared_allocation_bytes as arc_bytes;
use crate::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::{cell::UnsafeCell,mem::{ManuallyDrop,MaybeUninit,size_of},sync::{Arc,atomic::{AtomicUsize,Ordering}}};

struct Allocation<T>{aliases:AtomicUsize,value:UnsafeCell<MaybeUninit<T>>}
/// 🪢️ Live aliases expose immutable payloads; only the elected zero-alias owner mutates initialized fields.
unsafe impl<T:Send+Sync> Sync for Allocation<T>{}

/// 🛂️ Only this issued owner enters original shared retirement; raw Arc aliases have no conversion.
/// ```compile_fail
/// use std::sync::Arc;
/// use semio_framework_value_allocation_return::original::SharedOriginalStorage;
/// let raw=Arc::new(String::from("foreign weak custody"));let weak=Arc::downgrade(&raw);
/// let _=SharedOriginalStorage::<String>::admit_arc(raw);
/// ```
/// ```compile_fail
/// use std::sync::Arc;
/// use semio_framework_value_allocation_return::original::SharedOriginalStorage;
/// let raw=Arc::new(String::from("foreign strong custody"));let alias=Arc::clone(&raw);
/// let _=SharedOriginalStorage::<String>::admit_arc(raw);
/// ```
pub struct SharedOriginalStorage<T>{source:ManuallyDrop<Option<Arc<Allocation<T>>>>}
/// 🎫️ A real fallibly allocated backing retains its empty payload until publication or paid release.
pub struct SharedOriginalPublication<'a,T:Send+Sync+'static>{birth:&'a mut SharedOriginalBirth<T>}
impl<T:Send+Sync+'static> SharedOriginalPublication<'_,T>{
 pub fn commit(self,value:T)->crate::RetainedWorkOutcome<(SharedOriginalStorage<T>,RetainedCloneProgress)>{
  let mut source=self.birth.source.take().expect("funded original publication retains its backing");let storage=Arc::get_mut(&mut source).expect("unpublished original backing is unique");storage.value.get_mut().write(value);storage.aliases.store(1,Ordering::Relaxed);
  crate::RetainedWorkOutcome{result:Ok((SharedOriginalStorage{source:ManuallyDrop::new(Some(source))},RetainedCloneProgress{copied_items:1,copied_bytes:SharedOriginalBirth::<T>::publication_demand().copy_bytes,..Default::default()})),work:u128::from(Self::work_demand())}
 }
 pub const fn work_demand()->u64{4}
}
pub struct SharedOriginalBirth<T>{source:ManuallyDrop<Option<Arc<Allocation<T>>>>}
impl<T> SharedOriginalBirth<T>{
 pub fn publication_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:std::mem::size_of::<Option<Arc<Allocation<T>>>>()+std::mem::size_of::<T>()+std::mem::size_of::<AtomicUsize>()+std::mem::size_of::<SharedOriginalStorage<T>>(),depth:1,..Default::default()}}
 pub const fn publication_work_demand()->u64{4}
 /// 📤️ Moves only admitted native headers into the resident output; no user callback runs.
 pub fn publish(&mut self,input:&mut Option<T>,output:&mut Option<SharedOriginalStorage<T>>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{
  let before=effect.performed_boundary();let result=(||{if output.is_some(){return Err(ValueError::invariant("original publication output is occupied"))}if self.source.is_none(){return Err(ValueError::invariant("original publication backing is absent"))}if input.is_none(){return Err(ValueError::invariant("original publication input is absent"))}let demand=Self::publication_demand();if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<Self::publication_work_demand(){return Ok(crate::RetainedCloneStep::Progress(Default::default()))}if !effect.performed(Self::publication_work_demand())?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original publication exceeded original work"))}
   let mut source=self.source.take().expect("funded publication backing");let storage=Arc::get_mut(&mut source).expect("unpublished backing is unique");storage.value.get_mut().write(input.take().expect("funded publication input"));storage.aliases.store(1,Ordering::Relaxed);*output=Some(SharedOriginalStorage{source:ManuallyDrop::new(Some(source))});
   Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
  })();crate::RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same native publication receipt")}
 }
 pub fn next_close_demand(&self)->crate::RetirementDemand{if self.source.is_none(){Default::default()}else{crate::RetirementDemand{copy_bytes:std::mem::size_of::<Option<Arc<Allocation<T>>>>(),release_bytes:SharedOriginalStorage::<T>::allocation_bytes(),depth:1,..Default::default()}}}
 pub fn close_work_demand(&self)->u64{u64::from(self.source.is_some())}
 pub fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{
  let mut work=0;let result=(||{if self.source.is_none(){return Ok(crate::RetainedCloneStep::Complete(Default::default()))}let demand=self.next_close_demand();if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(crate::RetainedCloneStep::Progress(Default::default()))}work=1;if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"empty shared backing release exceeded original work"))}drop(self.source.take());Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work}
 }
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
}
impl<T> Drop for SharedOriginalBirth<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"empty original birth requires granted publication or release");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source)}}}}

/// 🔗️ Synchronous alias construction stays in the actual issued allocation domain.
impl<T> Clone for SharedOriginalStorage<T>{fn clone(&self)->Self{self.alias()}}
impl<T> std::ops::Deref for SharedOriginalStorage<T>{type Target=T;fn deref(&self)->&T{self.get()}}
impl<T:std::fmt::Debug> std::fmt::Debug for SharedOriginalStorage<T>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{self.get().fmt(f)}}

pub enum SharedOriginalRelease<T>{Alias,Original(SharedOriginal<T>)}
pub struct SharedOriginal<T>{source:ManuallyDrop<Option<Arc<Allocation<T>>>>,initialized:bool}

impl<T> SharedOriginalStorage<T>{
 /// 🧊️ Issues one actual original allocation at an explicitly synchronous cold setup boundary.
 pub fn from_cold(value:T)->Self{Self{source:ManuallyDrop::new(Some(Arc::new(Allocation{aliases:AtomicUsize::new(1),value:UnsafeCell::new(MaybeUninit::new(value))})))}}
 /// 📥️ Quotes allocation and typed placement before the original slot can be taken.
 pub fn source_admission_demand()->Result<crate::RetirementDemand,ValueError>{let mut d=super::shared_uninitialized_demand::<Allocation<T>>()?;d.copy_bytes=d.copy_bytes.checked_add(std::mem::size_of::<Allocation<T>>()).and_then(|n|n.checked_add(std::mem::size_of::<Option<T>>())).and_then(|n|n.checked_add(std::mem::size_of::<Option<Self>>())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original shared source copy extent overflow"))?;Ok(d)}
 pub const fn source_admission_work_demand()->u64{4}
 pub fn admit_from(source:&mut Option<T>,target:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{
  let before=effect.performed_boundary();let result=(||{if target.is_some()||effect.actual().is_some(){return Err(ValueError::invariant("original shared recipient or marker is occupied"))}if source.is_none(){return Ok(crate::RetainedCloneStep::Complete(Default::default()))}let demand=Self::source_admission_demand()?;if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<Self::source_admission_work_demand(){return Ok(crate::RetainedCloneStep::Progress(Default::default()))}
   let mut backing=None;let turn=super::try_uninitialized_arc_into::<Allocation<T>>(&mut backing,effect);turn.result?;let Some(mut backing)=backing.take()else{return Ok(crate::RetainedCloneStep::Progress(Default::default()))};
   assert!(effect.performed(3).expect("sealed preflight metadata work"),"sealed preflight metadata work");
   let value=source.take().expect("funded original source");Arc::get_mut(&mut backing).expect("new original backing is unique").write(Allocation{aliases:AtomicUsize::new(1),value:UnsafeCell::new(MaybeUninit::new(value))});*target=Some(Self{source:ManuallyDrop::new(Some(unsafe{backing.assume_init()}))});
   Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))
  })();crate::RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same native admission receipt")}
 }

 /// 📐️ Quotes the actual private allocation payload and its returned typed storage wrapper.
 pub fn value_admission_demand()->Result<crate::RetirementDemand,ValueError>{let mut d=super::shared_uninitialized_demand::<Allocation<T>>()?;d.copy_bytes=d.copy_bytes.checked_add(std::mem::size_of::<Allocation<T>>()).and_then(|n|n.checked_add(std::mem::size_of::<Self>())).ok_or_else(||ValueError::invariant("original shared value copy extent overflow"))?;Ok(d)}
 pub const fn value_admission_work_demand()->u64{3}
 pub fn prepare_birth_demand()->Result<crate::RetirementDemand,ValueError>{let mut d=super::shared_uninitialized_demand::<Allocation<T>>()?;d.copy_bytes=d.copy_bytes.checked_add(std::mem::size_of::<AtomicUsize>()).and_then(|n|n.checked_add(std::mem::size_of::<SharedOriginalBirth<T>>())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original empty backing copy extent overflow"))?;Ok(d)}
 pub const fn prepare_birth_work_demand()->u64{3}
 pub fn prepare_birth(output:&mut Option<SharedOriginalBirth<T>>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{
  let before=effect.performed_boundary();let result=(||{if output.is_some()||effect.actual().is_some(){return Err(ValueError::invariant("original empty backing output or marker is occupied"))}let demand=Self::prepare_birth_demand()?;if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<Self::prepare_birth_work_demand(){return Ok(crate::RetainedCloneStep::Progress(Default::default()))}
   let mut source=None;let turn=super::try_uninitialized_arc_into::<Allocation<T>>(&mut source,effect);turn.result?;let Some(mut source)=source.take()else{return Ok(crate::RetainedCloneStep::Progress(Default::default()))};
   assert!(effect.performed(2).expect("sealed empty backing metadata work"),"sealed empty backing metadata work");let pointer=Arc::get_mut(&mut source).expect("new original backing is unique").as_mut_ptr();unsafe{std::ptr::addr_of_mut!((*pointer).aliases).write(AtomicUsize::new(0))};let source=unsafe{source.assume_init()};*output=Some(SharedOriginalBirth{source:ManuallyDrop::new(Some(source))});
   Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))
  })();crate::RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same empty backing receipt")}
 }

 pub fn allocation_bytes()->usize{arc_bytes::<Allocation<T>>()}
 /// 🏁️ Reports whether this exact issued alias has returned its ownership.
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
 /// 🪪️ Compares two still-live aliases of the same issued original allocation.
 pub fn same_original(&self,other:&Self)->bool{match(self.source.as_ref(),other.source.as_ref()){(Some(left),Some(right))=>Arc::ptr_eq(left,right)&&left.aliases.load(Ordering::Acquire)>0,_=>false}}
 pub fn get(&self)->&T{let source=self.source.as_ref().expect("live original alias");assert!(source.aliases.load(Ordering::Acquire)>0);unsafe{(&*source.value.get()).assume_init_ref()}}
 pub fn as_ptr(&self)->*const T{self.get()as *const T}
 pub fn identity(&self)->usize{self.source.as_ref().expect("live original alias").value.get().cast::<T>()as usize}
 pub fn aliases(&self)->usize{self.source.as_ref().expect("live original alias").aliases.load(Ordering::Acquire)}
 /// 🔍️ A unique original requires both logical and physical strong counts to be one.
 pub fn physically_unique(&self)->bool{self.source.as_ref().is_some_and(|source|source.aliases.load(Ordering::Acquire)==1&&Arc::strong_count(source)==1)}
 pub const DUPLICATE_WORK:u64=3;
 pub const fn duplicate_copy_bytes()->usize{2*std::mem::size_of::<AtomicUsize>()+std::mem::size_of::<Option<Self>>()}
 pub fn duplicate_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:Self::duplicate_copy_bytes(),depth:1,..Default::default()}}
 /// ⛽️ Borrows the actual live alias producer quote.
 pub fn duplicate_work_demand(&self)->Result<u64,ValueError>{if self.source.is_none(){Err(ValueError::invariant("original alias source is empty"))}else{Ok(Self::DUPLICATE_WORK)}}
 pub fn duplicate_into(&self,output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{
  let mut work=0;let result=(||{if output.is_some(){return Err(ValueError::invariant("original duplicate output is occupied"))}let source=self.source.as_ref().ok_or_else(||ValueError::invariant("original alias source is empty"))?;if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),Self::duplicate_demand())||effect.remaining_work_units()?<Self::DUPLICATE_WORK{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original alias duplicate metadata is not funded"))}
   work=1;if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original alias attempt exceeded its work authority"))}
   if source.aliases.try_update(Ordering::AcqRel,Ordering::Acquire,|count|(count>0).then(||count.checked_add(1)).flatten()).is_err(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original alias space exhausted"))}
   let admitted=effect.performed(2).expect("preflight sealed alias work remains available");assert!(admitted,"preflight sealed alias work remains available");work+=2;
   *output=Some(Self{source:ManuallyDrop::new(Some(Arc::clone(source)))});Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:Self::duplicate_copy_bytes(),..Default::default()}))
  })();crate::RetainedWorkOutcome{result,work}
 }

 /// 🧊️ Elects an actual original at an explicitly synchronous cold boundary.
 pub fn elect_cold(&mut self)->Option<SharedOriginal<T>>{let source=self.source.take()?;let previous=source.aliases.fetch_sub(1,Ordering::AcqRel);assert!(previous>0,"cold election owns one genuine logical alias");if previous>1{drop(source);None}else{Some(SharedOriginal{source:ManuallyDrop::new(Some(source)),initialized:true})}}
 /// 🔗️ Adds one counted alias without allocating: the shared strong count is the only state touched.
 pub fn alias(&self)->Self{let source=self.source.as_ref().expect("live original alias");let strong=Arc::clone(source);source.aliases.fetch_add(1,Ordering::AcqRel);Self{source:ManuallyDrop::new(Some(strong))}}
 /// 📏️ Bounds both actual alias-election outcomes before any logical or physical counter changes.
 pub fn retire_demand(&self)->Result<crate::RetirementDemand,ValueError>{let Some(source)=self.source.as_ref()else{return Ok(Default::default())};if source.aliases.load(Ordering::Acquire)==0{return Err(ValueError::invariant("original alias election has no live logical owner"))}let copy=std::mem::size_of::<Option<Arc<Allocation<T>>>>().checked_add(2*std::mem::size_of::<AtomicUsize>()).and_then(|n|n.checked_add(std::mem::size_of::<SharedOriginalRelease<T>>())).ok_or_else(||ValueError::invariant("original election copy extent overflow"))?;Ok(crate::RetirementDemand{copy_bytes:copy,depth:1,..Default::default()})}
 /// 📏️ Quotes the real resident election output and both possible alias-counter writes.
 pub fn retire_into_demand(&self)->Result<crate::RetirementDemand,ValueError>{if self.source.is_none(){return Ok(Default::default())}let copy_bytes=std::mem::size_of::<Option<Arc<Allocation<T>>>>().checked_add(2*std::mem::size_of::<AtomicUsize>()).and_then(|n|n.checked_add(std::mem::size_of::<Option<SharedOriginal<T>>>())).ok_or_else(||ValueError::invariant("original alias election extent overflow"))?;Ok(crate::RetirementDemand{copy_bytes,depth:1,..Default::default()})}
 /// 🧮️ Executes only the sealed native election primitive with caller-resident output.
 pub fn elect_native_into(&mut self,output:&mut Option<SharedOriginal<T>>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{
  let before=effect.performed_boundary();let result=(||{
   if output.is_some(){return Err(ValueError::invariant("original alias election destination occupied"))}
   let Some(source)=self.source.as_ref()else{return Ok(crate::RetainedCloneStep::Complete(Default::default()))};
   if source.aliases.load(Ordering::Acquire)==0{return Err(ValueError::invariant("original alias election has no live logical owner"))}
   let demand=self.retire_into_demand()?;
   if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<4{return Ok(crate::RetainedCloneStep::Progress(Default::default()))}
   if !effect.performed(3)?{return Ok(crate::RetainedCloneStep::Progress(Default::default()))}
   let source=self.source.take().expect("funded original alias retained");let previous=source.aliases.fetch_sub(1,Ordering::AcqRel);assert!(previous>0,"funded original election owns a logical alias");
   let copy_bytes=if previous>1{assert!(effect.performed(1).expect("preflighted original alias work"));drop(source);size_of::<Option<Arc<Allocation<T>>>>()+2*size_of::<AtomicUsize>()}else{*output=Some(SharedOriginal{source:ManuallyDrop::new(Some(source)),initialized:true});size_of::<Option<Arc<Allocation<T>>>>()+size_of::<AtomicUsize>()+size_of::<Option<SharedOriginal<T>>>()};
   Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:copy_bytes,..Default::default()}))
  })();crate::RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same original election receipt")}
 }
 /// 🗳️ Deposits the actual native election result before returning to its producer.
 pub fn retire_into<'reply,E:From<ValueError>>(&mut self,output:&mut Option<SharedOriginal<T>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,crate::RetainedCloneStep,E>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,E>,ValueError>{let outcome=self.elect_native_into(output,effect);Ok(reply.complete(outcome.result.map_err(Into::into)))}
 pub fn retire_work_demand(&self)->Result<u64,ValueError>{self.retire_demand()?;Ok(if self.source.is_some(){4}else{0})}
 /// 🗳️ Retains this exact alias until the complete election and possible physical alias return are funded.
 pub fn retire_step(&mut self,grant:RetainedCloneGrant,maximum:u64)->crate::RetainedWorkOutcome<Option<(SharedOriginalRelease<T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.retire_demand()?;if self.source.is_none(){return Ok(None)}if maximum<4||!crate::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}let source=self.source.take().expect("funded original election retains its source");let previous=source.aliases.fetch_sub(1,Ordering::AcqRel);assert!(previous>0,"funded original election owns a logical alias");let(output,copy)=if previous>1{drop(source);work=4;(SharedOriginalRelease::Alias,d.copy_bytes)}else{work=3;(SharedOriginalRelease::Original(SharedOriginal{source:ManuallyDrop::new(Some(source)),initialized:true}),d.copy_bytes-std::mem::size_of::<AtomicUsize>())};Ok(Some((output,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})))})();crate::RetainedWorkOutcome{result,work}}
 pub fn retire(mut self)->SharedOriginalRelease<T>{let source=self.source.take().expect("live original alias");let previous=source.aliases.fetch_sub(1,Ordering::AcqRel);assert!(previous>0);if previous>1{drop(source);SharedOriginalRelease::Alias}else{SharedOriginalRelease::Original(SharedOriginal{source:ManuallyDrop::new(Some(source)),initialized:true})}}
 pub fn try_owner(mut self)->Result<SharedOriginal<T>,Self>{let source=self.source.as_ref().expect("live original alias");if Arc::strong_count(source)!=1||source.aliases.compare_exchange(1,0,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(self);}Ok(SharedOriginal{source:ManuallyDrop::new(self.source.take()),initialized:true})}
 /// 📮️ Transfers exactly one counted alias into a sealed return slot without releasing it.
 pub fn into_raw(mut self)->*const(){Arc::into_raw(self.source.take().expect("live original alias")).cast()}
 /// 📨️ Recovers the exact initialized counted alias previously transferred by into_raw.
 /// # Safety
 /// The pointer must be live, issued for T by into_raw, and consumed once.
 pub unsafe fn from_raw(pointer:*const())->Self{Self{source:ManuallyDrop::new(Some(unsafe{Arc::from_raw(pointer.cast::<Allocation<T>>())}))}}
}
impl<T> Drop for SharedOriginalStorage<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.source.is_none(),"shared original alias must retire or transfer");}}

impl<T> SharedOriginal<T>{
 /// 🕳️ Releases the allocation of a last owner that was dropped without retirement; the initialized payload is leaked, never dropped uncontrolled.
 pub fn abandon(mut self){self.initialized=false;drop(self.source.take());}
 pub fn allocation_bytes()->usize{SharedOriginalStorage::<T>::allocation_bytes()}
 pub fn get(&self)->&T{assert!(self.initialized);let source=self.source.as_ref().expect("live original owner");unsafe{(&*source.value.get()).assume_init_ref()}}
 pub fn identity(&self)->usize{self.source.as_ref().expect("live original owner").value.get().cast::<T>()as usize}
 /// 🟢️ Reports whether the complete original payload remains initialized.
 pub fn is_initialized(&self)->bool{self.initialized}
 pub fn physical_is_unique(&self)->bool{self.source.as_ref().is_some_and(|source|Arc::strong_count(source)==1)}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
 /// 📏️ Quotes payload movement, both control fields and the actual final strong/weak counter return.
 pub fn take_value_demand(&self)->Result<crate::RetirementDemand,ValueError>{if self.source.is_none(){if self.initialized{return Err(ValueError::invariant("empty original owner remains initialized"))}return Ok(Default::default())}if !self.initialized{return Err(ValueError::invariant("partially transferred original cannot return its whole payload"))}let copy=std::mem::size_of::<T>().checked_add(std::mem::size_of::<Option<T>>()).and_then(|n|n.checked_add(std::mem::size_of::<Option<Arc<Allocation<T>>>>())).and_then(|n|n.checked_add(std::mem::size_of::<bool>())).and_then(|n|n.checked_add(2*std::mem::size_of::<AtomicUsize>())).ok_or_else(||ValueError::invariant("original payload handback copy extent overflow"))?;Ok(crate::RetirementDemand{copy_bytes:copy,release_bytes:Self::allocation_bytes(),depth:1,..Default::default()})}
 pub fn take_value_work_demand(&self)->Result<u64,ValueError>{self.take_value_demand()?;Ok(if self.source.is_some(){4}else{0})}
 /// 📤️ Returns the unique initialized payload only after its exact movement and backing release are funded.
 /// 📤️ Deposits the exact elected payload and physical release into caller-resident slots.
 pub fn take_value_into<'reply,E:From<ValueError>>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,crate::RetainedCloneStep,E>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,E>,ValueError>{
  if output.is_some(){return Ok(reply.complete(Err(ValueError::invariant("original payload output is occupied").into())))}let demand=self.take_value_demand()?;if self.source.is_none(){return Ok(reply.complete(Ok(crate::RetainedCloneStep::Complete(Default::default()))))}if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<4{return Ok(reply.complete(Ok(crate::RetainedCloneStep::Progress(Default::default()))))}if !self.physical_is_unique(){return Ok(reply.complete(Ok(crate::RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Ok(crate::RetainedCloneStep::Progress(Default::default()))))}if self.source.as_mut().and_then(Arc::get_mut).is_none(){return Ok(reply.complete(Ok(crate::RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(3)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original payload handback exceeded its work").into())))}
  let source=self.source.as_mut().expect("unique original allocation retained");let value=unsafe{Arc::get_mut(source).expect("unique original allocation remains exclusive").value.get().cast::<T>().read()};*output=Some(value);self.initialized=false;drop(self.source.take());Ok(reply.complete(Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))))
 }
 pub fn take_value_step(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>>{
  let demand=match self.take_value_demand(){Ok(demand)=>demand,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if output.is_some(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original payload output is occupied")),work:0}}if self.source.is_none(){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if remaining<4||!crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original payload uniqueness exceeded its Work quote")),work:1},Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};let unique=self.source.as_mut().and_then(Arc::get_mut).is_some();if !unique{return crate::RetainedWorkOutcome{result:Ok(None),work:1}}match effect.performed(3){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original payload handback exceeded its Work quote")),work:4},Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:1}};let source=self.source.as_mut().expect("unique original allocation retained");let value=unsafe{Arc::get_mut(source).expect("unique original allocation remains exclusive").value.get().cast::<T>().read()};*output=Some(value);self.initialized=false;drop(self.source.take());crate::RetainedWorkOutcome{result:Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})),work:4}
 }
 pub fn take_value(&mut self)->Option<T>{if !self.initialized||!self.physical_is_unique(){return None;}let source=self.source.take().unwrap();self.initialized=false;let value=unsafe{source.value.get().cast::<T>().read()};drop(source);Some(value)}
 /// 🔎️ Borrows one still-initialized aligned field of this exact allocation.
 /// # Safety
 /// The projection must remain inside T and must never name an already transferred field.
 pub unsafe fn field<F>(&self,project:fn(*const T)->*const F)->&F{let source=self.source.as_ref().expect("live original owner");unsafe{&*project(source.value.get().cast::<T>())}}
 /// 📤️ Transfers one initialized nonoverlapping aligned field once.
 /// # Safety
 /// The projection must name that field inside T; subsequent operations must never read or drop the transferred field.
 pub unsafe fn take_field<F>(&mut self,project:fn(*mut T)->*mut F)->F{let source=self.source.as_ref().expect("live original owner");self.initialized=false;unsafe{project(source.value.get().cast::<T>()).read()}}
 /// 🧹️ Releases storage after every owned field has transferred; physical aliases defer release without effects.
 /// # Safety
 /// The caller must prove no initialized payload remains.
 pub unsafe fn release_empty(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress{if self.source.is_none()||!self.physical_is_unique()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<Self::allocation_bytes(){return Default::default();}self.initialized=false;drop(self.source.take());RetainedCloneProgress{copied_items:1,released_bytes:Self::allocation_bytes(),..Default::default()}}
}
impl<T> Drop for SharedOriginal<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.source.is_none(),"shared original owner must drain its initialized payload");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

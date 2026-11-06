//! 🏠️ Original Source roots retain native parents and prepaid return inboxes before any sealed loan is published.
use super::*;
use crate::OriginalCallerReturnPlace;
use std::{any::Any,cell::UnsafeCell,marker::PhantomData,ptr::NonNull,sync::{Mutex,atomic::{AtomicU8,AtomicU64,Ordering}}};
/// 🧮️
pub const ORIGINAL_NATIVE_SOURCE_CAPACITY:usize=8;
const RETURNS:usize=32;
/// 🧹️
pub trait OriginalNativeRetirement:Send+'static{fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;fn terminal_is_empty(&self)->bool;fn close_requires_execution(&self)->bool{false}}
trait ErasedNative:Send{fn value(&mut self)->&mut dyn Any;fn close(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;fn empty(&self)->bool;fn requires_execution(&self)->bool;}
struct NativeRoot<T>{value:T,_frame:u8}
impl<T:OriginalNativeRetirement> ErasedNative for NativeRoot<T>{fn value(&mut self)->&mut dyn Any{&mut self.value}fn close(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.value.close_on_source(cx)}fn empty(&self)->bool{self.value.terminal_is_empty()}fn requires_execution(&self)->bool{self.value.close_requires_execution()}}
struct NativeSlot{generation:u64,closed_generation:u64,loans:usize,bytes:usize,root:Option<Box<dyn ErasedNative>>}
struct NativeReturn{source:Option<OriginalCallerSourceLoan>,place:OriginalCallerReturnPlace,slot:usize,generation:u64,recipient_loan:bool}
struct ReturnCell{state:AtomicU8,serial:AtomicU64,record:UnsafeCell<Option<NativeReturn>>}
unsafe impl Send for ReturnCell{}
unsafe impl Sync for ReturnCell{}
impl ReturnCell{fn new()->Self{Self{state:AtomicU8::new(0),serial:AtomicU64::new(0),record:UnsafeCell::new(None)}}}
pub(super) struct NativeSources{slots:Mutex<[NativeSlot;ORIGINAL_NATIVE_SOURCE_CAPACITY]>,returns:[ReturnCell;RETURNS]}
impl NativeSources{
 pub(super) fn new()->Self{Self{slots:Mutex::new(std::array::from_fn(|_|NativeSlot{generation:0,closed_generation:0,loans:0,bytes:0,root:None})),returns:std::array::from_fn(|_|ReturnCell::new())}}
 pub(super) fn pending(&self)->bool{self.returns.iter().any(|cell|cell.state.load(Ordering::Acquire)!=0)||self.slots.try_lock().map_or(true,|slots|slots.iter().any(|slot|slot.root.is_some()||slot.loans!=0))}
 fn reserve(&self,slot:usize,generation:u64)->Option<usize>{let index=self.returns.iter().position(|cell|cell.serial.load(Ordering::Acquire)<u64::MAX&&cell.state.compare_exchange(0,6,Ordering::AcqRel,Ordering::Acquire).is_ok())?;self.returns[index].serial.fetch_add(1,Ordering::Release);unsafe{*self.returns[index].record.get()=Some(NativeReturn{source:None,place:OriginalCallerReturnPlace::empty(),slot,generation,recipient_loan:false})};Some(index)}
}
/// 🪢️
pub struct OriginalNativeLease<T:OriginalNativeRetirement>{roots:Option<NonNull<SourceRoots>>,source_identity:usize,binding:crate::OriginalSourceBinding,slot:usize,generation:u64,inbox:usize,inbox_serial:u64,captured:bool,_type:PhantomData<fn()->T>}
/// 🪪️
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalNativeRecipientIdentity{source:usize,binding:crate::OriginalSourceBinding,slot:usize,generation:u64}
impl OriginalNativeRecipientIdentity{pub fn validate_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;if !self.binding.matches(cx)||parent.source_identity()?!=self.source{return Err(refused("native borrow changed its original Source").into())}Ok(())}}
/// 🪪️
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalNativeRecipientClosed{recipient:OriginalNativeRecipientIdentity}
impl OriginalNativeRecipientClosed{pub fn proves(&self,recipient:OriginalNativeRecipientIdentity,cx:&StepContext<'_>)->bool{self.recipient==recipient&&self.recipient.binding.matches(cx)&&cx.original_source_parent().and_then(OriginalCallerSourceParent::source_identity).is_ok_and(|source|source==self.recipient.source)}}
/// 🪦️
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalNativeRetiredWitness{recipient:OriginalNativeRecipientIdentity}
impl OriginalNativeRetiredWitness{pub fn proves(&self,recipient:OriginalNativeRecipientIdentity,cx:&StepContext<'_>)->bool{self.recipient==recipient&&self.recipient.binding.matches(cx)&&cx.original_source_parent().and_then(OriginalCallerSourceParent::source_identity).is_ok_and(|source|source==self.recipient.source)}}
unsafe impl<T:OriginalNativeRetirement> Send for OriginalNativeLease<T>{}
/// 🔐️
pub struct OriginalNativeBorrowed<'loan,T:OriginalNativeRetirement>{value:&'loan mut T,identity:OriginalNativeRecipientIdentity}
impl<T:OriginalNativeRetirement> OriginalNativeBorrowed<'_,T>{pub fn recipient_identity(&self)->OriginalNativeRecipientIdentity{self.identity}pub fn validate_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.identity.validate_context(cx)}pub fn value_mut(&mut self)->&mut T{self.value}}
/// 🤝️
pub struct OriginalNativeBorrow<'a,T:OriginalNativeRetirement>{slots:std::sync::MutexGuard<'a,[NativeSlot;ORIGINAL_NATIVE_SOURCE_CAPACITY]>,slot:usize,source:&'a OriginalCallerSourceLoan,_type:PhantomData<fn()->T>}
impl<T:OriginalNativeRetirement> OriginalNativeBorrow<'_,T>{
 pub fn validate_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.source.validate_retirement_context(cx)}
 pub fn value_mut(&mut self)->&mut T{self.slots[self.slot].root.as_mut().expect("borrowed native root remains retained").value().downcast_mut::<T>().expect("sealed native root type")}
}
impl<T:OriginalNativeRetirement> OriginalNativeLease<T>{
 pub fn recipient_identity(&self)->OriginalNativeRecipientIdentity{OriginalNativeRecipientIdentity{source:self.source_identity,binding:self.binding,slot:self.slot,generation:self.generation}}
 pub fn terminal_is_empty(&self)->bool{self.roots.is_none()}
 fn parent<'a>(&self,cx:&StepContext<'a>)->Result<&'a OriginalCallerSourceParent,OriginalTurnFailure>{let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;if parent.source_identity()?!=self.source_identity||!self.binding.matches(cx){return Err(refused("native loan requires its genuine admitting Source").into())}Ok(parent)}
 fn root(&self)->Result<&SourceRoots,OriginalTurnFailure>{let roots=self.roots.ok_or_else(||refused("native loan already acknowledged"))?;Ok(unsafe{roots.as_ref()})}
 fn source(&self)->Result<&OriginalCallerSourceLoan,OriginalTurnFailure>{if self.captured{return Err(refused("native loan has entered return").into())}let cell=&self.root()?.native.returns[self.inbox];if cell.state.load(Ordering::Acquire)!=2{return Err(refused("native loan has no published recipient").into())}unsafe{(&*cell.record.get()).as_ref().and_then(|record|record.source.as_ref()).ok_or_else(||refused("native loan lost its originating Source alias").into())}}
 /// 🧵️
 pub fn borrow_native<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<OriginalNativeBorrow<'a,T>>,OriginalTurnFailure>{
  self.parent(cx)?;let source=self.source()?;source.validate_retirement_context(cx)?;if cx.should_yield()||!metadata_fits(cx){return Ok(None)}let roots=self.root()?;let Ok(slots)=roots.native.slots.try_lock()else{return Ok(None)};let slot=&slots[self.slot];if slot.generation!=self.generation||slot.loans==0||slot.root.is_none(){return Err(refused("native stack borrow generation has returned").into())}cx.consume_retained(metadata())?;cx.consume_fuel(1);Ok(Some(OriginalNativeBorrow{slots,slot:self.slot,source,_type:PhantomData}))
 }
 /// 🔎️
 pub fn with_native(&mut self,cx:&mut StepContext<'_>,run:impl FnOnce(&mut T,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{
  self.parent(cx)?;self.source()?.validate_retirement_context(cx)?;let roots=self.root()?;let Ok(mut slots)=roots.native.slots.try_lock()else{return Ok(false)};let slot=&mut slots[self.slot];if slot.generation!=self.generation||slot.loans==0{return Err(refused("native loan generation has no live recipient").into())}let native=slot.root.as_mut().and_then(|root|root.value().downcast_mut::<T>()).ok_or_else(||refused("native loan type differs from its original recipient"))?;run(native,cx)?;Ok(true)
 }
 /// 🔐️
 pub fn with_native_borrow(&mut self,cx:&mut StepContext<'_>,run:impl for<'loan> FnOnce(OriginalNativeBorrowed<'loan,T>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{let identity=self.recipient_identity();self.with_native(cx,|value,cx|run(OriginalNativeBorrowed{value,identity},cx))}
 /// 🤝️
 pub fn with_native_pair<U:OriginalNativeRetirement>(&mut self,other:&mut OriginalNativeLease<U>,cx:&mut StepContext<'_>,run:impl FnOnce(&mut T,&mut U,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{
  self.parent(cx)?;other.parent(cx)?;self.source()?.validate_retirement_context(cx)?;other.source()?.validate_retirement_context(cx)?;if self.roots!=other.roots||self.slot==other.slot{return Err(refused("native paired borrow requires two distinct recipients of its same Source").into())}let roots=self.root()?;let Ok(mut slots)=roots.native.slots.try_lock()else{return Ok(false)};
  let (left,right)=if self.slot<other.slot{let(before,after)=slots.split_at_mut(other.slot);(&mut before[self.slot],&mut after[0])}else{let(before,after)=slots.split_at_mut(self.slot);(&mut after[0],&mut before[other.slot])};if left.generation!=self.generation||right.generation!=other.generation||left.loans==0||right.loans==0{return Err(refused("native paired borrow generation no longer has its actual recipients").into())}let left=left.root.as_mut().and_then(|root|root.value().downcast_mut::<T>()).ok_or_else(||refused("native paired left type differs from its original recipient"))?;let right=right.root.as_mut().and_then(|root|root.value().downcast_mut::<U>()).ok_or_else(||refused("native paired right type differs from its original recipient"))?;run(left,right,cx)?;Ok(true)
 }
 /// 🔐️
 pub fn with_native_pair_borrow<U:OriginalNativeRetirement>(&mut self,other:&mut OriginalNativeLease<U>,cx:&mut StepContext<'_>,run:impl for<'loan> FnOnce(OriginalNativeBorrowed<'loan,T>,OriginalNativeBorrowed<'loan,U>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{let left=self.recipient_identity();let right=other.recipient_identity();self.with_native_pair(other,cx,|a,b,cx|run(OriginalNativeBorrowed{value:a,identity:left},OriginalNativeBorrowed{value:b,identity:right},cx))}
 /// 🎟️
 pub fn lend(&self,cx:&mut StepContext<'_>,output:&mut Option<Self>)->Result<bool,OriginalTurnFailure>{
  if output.is_some(){return Err(refused("native loan receiver must be empty").into())}let parent=self.parent(cx)?;self.source()?.validate_retirement_context(cx)?;let roots=self.root()?;let Ok(mut slots)=roots.native.slots.try_lock()else{return Ok(false)};let slot=&mut slots[self.slot];if slot.generation!=self.generation||slot.root.is_none(){return Err(refused("native loan generation no longer exists").into())}let loans=slot.loans.checked_add(1).ok_or_else(||refused("native recipient loan count exhausted"))?;
  let Some(prepared)=prepare_return(parent,roots,self.slot,self.generation,cx)?else{return Ok(false)};if cx.should_yield()||cx.is_cancelled()||!metadata_fits(cx){return Ok(false)}let inbox=prepared.index;slot.loans=loans;unsafe{(&mut*roots.native.returns[inbox].record.get()).as_mut().expect("reserved native return").recipient_loan=true};*output=Some(Self{roots:self.roots,source_identity:self.source_identity,binding:self.binding,slot:self.slot,generation:self.generation,inbox,inbox_serial:roots.native.returns[inbox].serial.load(Ordering::Acquire),captured:false,_type:PhantomData});roots.native.returns[inbox].state.store(2,Ordering::Release);cx.consume_retained(metadata())?;cx.consume_fuel(1);Ok(true)
 }
 fn capture(&mut self){if self.captured||self.roots.is_none(){return}let roots=unsafe{self.roots.expect("live native witness").as_ref()};let cell=&roots.native.returns[self.inbox];assert_eq!(cell.serial.load(Ordering::Acquire),self.inbox_serial,"native return inbox belongs to its exact birth");assert_eq!(cell.state.compare_exchange(2,5,Ordering::AcqRel,Ordering::Acquire),Ok(2),"sealed native witness captures once");unsafe{(&mut*cell.record.get()).as_mut().expect("native recipient retains return").place.record_return().expect("native return capture was prepaid before publication")};self.captured=true;cell.state.store(3,Ordering::Release);}
 /// 📮️
 pub fn return_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.roots.is_none(){return Ok(true)}let parent=self.parent(cx)?;self.capture();let roots=parent.original.as_ref().ok_or_else(||refused("native acknowledgment requires its still-live admitting Source"))?.get();let cell=&roots.native.returns[self.inbox];if cell.serial.load(Ordering::Acquire)!=self.inbox_serial||close_return(parent,roots,self.inbox,cx)?{self.roots=None;return Ok(true)}Ok(false)
 }
}
impl<T:OriginalNativeRetirement> Drop for OriginalNativeLease<T>{fn drop(&mut self){self.capture();self.roots=None;}}
struct PreparedReturn<'a>{cell:&'a ReturnCell,index:usize}
impl Drop for PreparedReturn<'_>{fn drop(&mut self){if self.cell.state.load(Ordering::Acquire)==6{self.cell.state.store(1,Ordering::Release)}}}
fn prepare_return<'a>(parent:&OriginalCallerSourceParent,roots:&'a SourceRoots,slot:usize,generation:u64,cx:&mut StepContext<'_>)->Result<Option<PreparedReturn<'a>>,OriginalTurnFailure>{
 if cx.should_yield()||cx.is_cancelled()||cx.retained_grant().maximum_items<6||cx.retained_grant().maximum_depth==0||cx.fuel_remaining()<6{return Ok(None)}if !cx.admit_original_return_table()?{return Ok(None)}if cx.should_yield()||cx.is_cancelled()||cx.retained_grant().maximum_items<6||cx.retained_grant().maximum_depth==0||cx.fuel_remaining()<6{return Ok(None)}let Some(index)=roots.native.reserve(slot,generation)else{return Ok(None)};let prepared=PreparedReturn{cell:&roots.native.returns[index],index};let record=unsafe{(&mut*prepared.cell.record.get()).as_mut().expect("reserved native return")};if !record.place.admit(cx)?&&!record.place.admit(cx)?{return Ok(None)}if !parent.admit_loan(&mut record.source,cx)?{return Ok(None)}Ok(Some(prepared))
}
fn close_return(parent:&OriginalCallerSourceParent,roots:&SourceRoots,index:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 let cell=&roots.native.returns[index];let state=cell.state.load(Ordering::Acquire);if state==0{return Ok(true)}if !matches!(state,1|3)||cell.state.compare_exchange(state,4,Ordering::AcqRel,Ordering::Acquire).is_err(){return Ok(false)}
 struct Restore<'a>{cell:&'a ReturnCell,state:u8}impl Drop for Restore<'_>{fn drop(&mut self){if self.cell.state.load(Ordering::Acquire)==4{self.cell.state.store(self.state,Ordering::Release)}}}let restore=Restore{cell,state};
 let record=unsafe{(&mut*cell.record.get()).as_mut().expect("admitted native return")};
 if record.recipient_loan{let Ok(mut slots)=roots.native.slots.try_lock()else{return Ok(false)};let slot=&mut slots[record.slot];if slot.generation!=record.generation||slot.loans==0{return Err(refused("native return generation has no original recipient").into())}if cx.retirement_should_yield()||!metadata_fits(cx){return Ok(false)}cx.consume_retained(metadata())?;cx.consume_fuel(1);slot.loans-=1;record.recipient_loan=false;return Ok(false)}
 if !record.place.terminal_is_empty(){record.place.settle_return(cx)?;return Ok(false)}
 if let Some(source)=record.source.as_mut(){source.close_step(parent,cx)?;if source.terminal_is_empty(){record.source.take();}return Ok(false)}
 if cx.retirement_should_yield()||!metadata_fits(cx){return Ok(false)}cx.consume_retained(metadata())?;cx.consume_fuel(1);unsafe{(&mut*cell.record.get()).take();}cell.state.store(0,Ordering::Release);drop(restore);Ok(true)
}
impl OriginalCallerSourceParent{
 /// 🪦️
 pub fn native_recipient_closed(&self,recipient:OriginalNativeRecipientIdentity,cx:&mut StepContext<'_>)->Result<Option<OriginalNativeRecipientClosed>,OriginalTurnFailure>{self.validate_borrowed_context(cx)?;if recipient.source!=self.source_identity()?||!recipient.binding.matches(cx){return Err(refused("native recipient proof requires its original Source").into())}let roots=self.original.as_ref().ok_or_else(||refused("native recipient proof has no parent"))?.get();let Ok(slots)=roots.native.slots.try_lock()else{return Ok(None)};let closed=slots.get(recipient.slot).is_some_and(|slot|slot.closed_generation>=recipient.generation);if !closed||cx.retirement_should_yield()||!metadata_fits(cx){return Ok(None)}cx.consume_retained(metadata())?;cx.consume_fuel(1);Ok(Some(OriginalNativeRecipientClosed{recipient}))}
 pub fn native_retired_witness(&self,recipient:OriginalNativeRecipientIdentity,cx:&mut StepContext<'_>)->Result<Option<OriginalNativeRetiredWitness>,OriginalTurnFailure>{self.validate_borrowed_context(cx)?;if recipient.source!=self.source_identity()?||!recipient.binding.matches(cx){return Err(refused("native retirement evidence requires its genuine admitting Source").into())}let roots=self.original.as_ref().ok_or_else(||refused("native retirement evidence has no live original parent"))?.get();if !self.original_loans_are_returned()||roots.native.returns.iter().any(|cell|cell.state.load(Ordering::Acquire)!=0){return Ok(None)}let Ok(slots)=roots.native.slots.try_lock()else{return Ok(None)};if slots[recipient.slot].closed_generation<recipient.generation||cx.retirement_should_yield()||!metadata_fits(cx){return Ok(None)}cx.consume_retained(metadata())?;cx.consume_fuel(1);Ok(Some(OriginalNativeRetiredWitness{recipient}))}
 pub fn native_sources_pending(&self)->bool{self.original.as_ref().is_some_and(|source|source.get().native.pending())}
 /// 🎟️
 pub fn native_close_requires_execution(&self)->Option<bool>{let Some(source)=self.original.as_ref()else{return Some(false)};let native=&source.get().native;if native.returns.iter().any(|cell|matches!(cell.state.load(Ordering::Acquire),1|3)){return Some(false)}let slots=native.slots.try_lock().ok()?;Some(slots.iter().find(|slot|slot.root.is_some()&&slot.loans==0).and_then(|slot|slot.root.as_ref()).is_some_and(|root|!root.empty()&&root.requires_execution()))}
 pub fn admit_native<T:OriginalNativeRetirement>(&self,input:&mut Option<T>,output:&mut Option<OriginalNativeLease<T>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate_borrowed_context(cx)?;if let Some(loan)=output.as_ref(){loan.parent(cx)?;return Ok(true)}if !self.lending_open{return Err(refused("native birth cannot reopen a closing Source").into())}let roots=self.original.as_ref().ok_or_else(||refused("native birth Source has closed"))?.get();let Ok(mut slots)=roots.native.slots.try_lock()else{return Ok(false)};let Some(index)=slots.iter().position(|slot|slot.root.is_none()&&slot.loans==0)else{return Ok(false)};let generation=slots[index].generation.checked_add(1).ok_or_else(||refused("native recipient generation exhausted"))?;let bytes=std::mem::size_of::<NativeRoot<T>>();let demand=RetirementDemand{capacity_bytes:bytes,depth:1,..Default::default()};if cx.should_yield()||cx.is_cancelled()||cx.retained_grant().maximum_items<6||!semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(false)}if input.is_none(){return Err(refused("native birth requires its actual owned input").into())}
  let Some(prepared)=prepare_return(self,roots,index,generation,cx)?else{return Ok(false)};if cx.should_yield()||cx.is_cancelled()||!metadata_fits(cx)||cx.retained_grant().maximum_capacity_bytes<bytes{return Ok(false)}let inbox=prepared.index;let value=input.take().expect("validated native input");slots[index]=NativeSlot{generation,closed_generation:slots[index].closed_generation,loans:1,bytes,root:Some(Box::new(NativeRoot{value,_frame:0}))};unsafe{(&mut*roots.native.returns[inbox].record.get()).as_mut().expect("reserved native return").recipient_loan=true};*output=Some(OriginalNativeLease{roots:Some(NonNull::from(roots)),source_identity:self.source_identity()?,binding:crate::OriginalSourceBinding::borrow(cx),slot:index,generation,inbox,inbox_serial:roots.native.returns[inbox].serial.load(Ordering::Acquire),captured:false,_type:PhantomData});roots.native.returns[inbox].state.store(2,Ordering::Release);cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
 }
 /// ♻️
 pub fn close_native_step(&self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate_borrowed_context(cx)?;let roots=self.original.as_ref().ok_or_else(||refused("native close requires its original Source"))?.get();if let Some(index)=roots.native.returns.iter().position(|cell|matches!(cell.state.load(Ordering::Acquire),1|3)){close_return(self,roots,index,cx)?;return Ok(false)}let Ok(mut slots)=roots.native.slots.try_lock()else{return Ok(false)};let Some(index)=slots.iter().position(|slot|slot.root.is_some()&&slot.loans==0)else{return Ok(slots.iter().all(|slot|slot.root.is_none())&&roots.native.returns.iter().all(|cell|cell.state.load(Ordering::Acquire)==0))};let slot=&mut slots[index];let root=slot.root.as_mut().expect("selected native root");if !root.empty(){cx.with_original_retirement(|cx|root.close(cx))?;return Ok(false)}let demand=RetirementDemand{release_bytes:slot.bytes,depth:1,..Default::default()};if cx.retirement_should_yield()||!semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:slot.bytes,..Default::default()})?;cx.consume_fuel(1);drop(slot.root.take());slot.bytes=0;slot.closed_generation=slot.generation;Ok(slots.iter().all(|slot|slot.root.is_none())&&roots.native.returns.iter().all(|cell|cell.state.load(Ordering::Acquire)==0))
 }
}
impl StepContext<'_>{
 pub fn original_source_children_are_returned(&self)->Result<bool,ValueError>{let parent=self.original_source_parent()?;parent.validate_borrowed_context(self)?;Ok(parent.original_loans_are_returned())}
 /// 🔎️
 pub fn original_native_recipient_closed(&mut self,recipient:OriginalNativeRecipientIdentity)->Result<Option<OriginalNativeRecipientClosed>,OriginalTurnFailure>{self.original_source_parent()?.native_recipient_closed(recipient,self)}
 pub fn original_native_retired_witness(&mut self,recipient:OriginalNativeRecipientIdentity)->Result<Option<OriginalNativeRetiredWitness>,OriginalTurnFailure>{self.original_source_parent()?.native_retired_witness(recipient,self)}
 pub fn admit_original_native<T:OriginalNativeRetirement>(&mut self,input:&mut Option<T>,output:&mut Option<OriginalNativeLease<T>>)->Result<bool,OriginalTurnFailure>{self.original_source_parent()?.admit_native(input,output,self)}
 pub fn close_original_native_step(&mut self)->Result<bool,OriginalTurnFailure>{self.original_source_parent()?.close_native_step(self)}
}

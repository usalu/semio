//! 🪶️ Same-instance snapshot recipients preserve actual partial ownership across receiving turns.
use super::{admitted,refusal,refusal_code};
use crate::{sqlite_wire,Fault};
use crate::component::wasip2::semio::framework::pure;
use semio_framework_os_kernel::io::io_mechanism::{IoNativeDirection,IoRunControl};
use semio_framework_value::{ValueError,ValueRefusalKind,ErasedSnapshotRetirement,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retirement::controlled::ControlledRetirement};
use semio_framework_value::native_decoding::{NativeDecodeControl,NativeForwardedDecodeControl,allocation::NativeForwardedDecodeReceipt,NativeDecodeRetirementRecipient};
use semio_framework_value::native_encoding::{NativeEncodeControl,NativeForwardedEncodeControl,allocation::NativeForwardedEncodeReceipt,NativeEncodeRetirementRecipient,NativeEncodeAllocation};
use std::{cell::{RefCell,Cell},mem::size_of};

#[derive(semio_framework_value::RetireOwned)]
pub enum SnapshotOutput{File(sqlite_wire::SnapshotFileResult),Payload(sqlite_wire::SnapshotPayloadResult),Fault(Fault)}
/// 📤️ Keeps the exact returned wire owner until all canonical fields have original reservations.
pub trait SnapshotReturn:Sized{
 fn retain(self)->SnapshotOutput;
 fn borrow(owner:&SnapshotOutput)->Option<&Self>;
 fn take(owner:SnapshotOutput)->Option<Self>;
 fn pending(owner:sqlite_wire::SnapshotRetirement)->Self;
 fn reserve(&self,component:bool,cursor:&mut u8)->Result<(),ValueError>;
}
fn reserve_bytes(bytes:usize)->Result<(),ValueError>{if bytes==0{Ok(())}else{admitted(pure::operation_reserve_return(pure::OperationReturnAllocation::Bytes,bytes as u64))}}
fn reserve_fields(first_kind:pure::OperationReturnAllocation,first:usize,second:usize,cursor:&mut u8)->Result<(),ValueError>{if *cursor==0{if first!=0{admitted(pure::operation_reserve_return(first_kind,first as u64))?;}*cursor=1;}if *cursor==1{reserve_bytes(second)?;*cursor=2;}Ok(())}
fn reserve_rejection(value:&sqlite_wire::SnapshotRejection,cursor:&mut u8)->Result<(),ValueError>{reserve_fields(pure::OperationReturnAllocation::StringBytes,value.message.len(),value.diagnostics.len(),cursor)}
impl SnapshotReturn for sqlite_wire::SnapshotFileResult{
 fn retain(self)->SnapshotOutput{SnapshotOutput::File(self)}
 fn borrow(owner:&SnapshotOutput)->Option<&Self>{if let SnapshotOutput::File(value)=owner{Some(value)}else{None}}
 fn take(owner:SnapshotOutput)->Option<Self>{if let SnapshotOutput::File(value)=owner{Some(value)}else{None}}
 fn pending(owner:sqlite_wire::SnapshotRetirement)->Self{Self::Pending(owner)}
 fn reserve(&self,component:bool,cursor:&mut u8)->Result<(),ValueError>{if !component{return Ok(())}match self{Self::Done(value)=>reserve_fields(pure::OperationReturnAllocation::Bytes,value.bytes.len(),value.diagnostics.len(),cursor),Self::Rejected(value)=>reserve_rejection(value,cursor),Self::Pending(_)=>Ok(())}}
}
impl SnapshotReturn for sqlite_wire::SnapshotPayloadResult{
 fn retain(self)->SnapshotOutput{SnapshotOutput::Payload(self)}
 fn borrow(owner:&SnapshotOutput)->Option<&Self>{if let SnapshotOutput::Payload(value)=owner{Some(value)}else{None}}
 fn take(owner:SnapshotOutput)->Option<Self>{if let SnapshotOutput::Payload(value)=owner{Some(value)}else{None}}
 fn pending(owner:sqlite_wire::SnapshotRetirement)->Self{Self::Pending(owner)}
 fn reserve(&self,component:bool,cursor:&mut u8)->Result<(),ValueError>{if !component{return Ok(())}match self{Self::Done(value)=>reserve_fields(pure::OperationReturnAllocation::Bytes,value.bytes.len(),value.diagnostics.len(),cursor),Self::Rejected(value)=>reserve_rejection(value,cursor),Self::Pending(_)=>Ok(())}}
}
enum NativeReceipt{Decode(NativeForwardedDecodeReceipt),Encode(NativeForwardedEncodeReceipt)}
struct SnapshotOperation{ticket:u64,maximum:usize,owned:usize,body:usize,decoding:bool,return_cursor:u8,refusal:u32,native:Option<NativeReceipt>,decode:NativeDecodeRetirementRecipient,encode:NativeEncodeRetirementRecipient,output:ControlledRetirement<Option<SnapshotOutput>>}
thread_local!{static SNAPSHOT:RefCell<Option<Box<SnapshotOperation>>>=const{RefCell::new(None)};static NEXT_TICKET:Cell<u64>=const{Cell::new(0)};}
fn demand(owner:&dyn ErasedSnapshotRetirement,body:usize)->Result<sqlite_wire::SnapshotDemands,ValueError>{Ok(sqlite_wire::SnapshotDemands{items:1,copy_bytes:owner.next_copy_byte_demand()?as u64,capacity_bytes:owner.next_capacity_byte_demand(body)?as u64,release_bytes:owner.next_release_byte_demand()?as u64,depth:owner.next_depth_demand()?as u64})}
impl SnapshotOperation{
 fn native_owner(&self)->&dyn ErasedSnapshotRetirement{if self.decoding{&self.decode}else{&self.encode}}
 fn status(&self,body:usize)->Result<sqlite_wire::SnapshotRetirement,ValueError>{let native=self.native_owner();let native_pending=!native.terminal_is_empty();let output_pending=!self.output.terminal_is_empty();let demands=if native_pending{demand(native,body)?}else if output_pending{demand(&self.output,body)?}else{sqlite_wire::SnapshotDemands{items:1,copy_bytes:size_of::<Option<Box<Self>>>()as u64,release_bytes:size_of::<Self>()as u64,depth:1,..Default::default()}};Ok(sqlite_wire::SnapshotRetirement{ticket:self.ticket,owned_bytes:self.owned as u64,refusal:self.refusal,native_pending,output_pending,terminal:false,demands})}
}
impl Drop for SnapshotOperation{fn drop(&mut self){assert!(std::thread::panicking()||(self.decode.terminal_is_empty()&&self.encode.terminal_is_empty()&&self.output.terminal_is_empty()),"snapshot component lost actual retained receiving ownership");}}
/// 👓️ Returns only the actual same-instance ticket and physical next demands.
pub fn snapshot_retirement(body:usize)->Result<sqlite_wire::SnapshotRetirement,ValueError>{SNAPSHOT.with(|slot|slot.borrow().as_ref().map_or(Ok(sqlite_wire::SnapshotRetirement{terminal:true,..Default::default()}),|state|state.status(body)))}
fn reservation(owned:&Cell<usize>,maximum:usize,bytes:usize)->Result<(),ValueError>{let previous=owned.get();let next=previous.checked_add(bytes).filter(|next|*next<=maximum).ok_or_else(||refusal(2))?;admitted(pure::operation_allocation(bytes as u64,previous as u64,next as u64,maximum as u64))?;owned.set(next);Ok(())}
/// 🌱️ Holds real native recipient slots and denied output in the component that allocated them.
pub fn with_snapshot_authority<T:SnapshotReturn>(input:sqlite_wire::SnapshotInput,direction:IoNativeDirection,component:bool,operation:impl FnOnce(sqlite_wire::SnapshotInput,&mut IoRunControl<'_,'_>,&mut semio_framework::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<T,Fault>)->Result<Result<T,Fault>,ValueError>{
 let grant=input.native.native()?;let limits=input.limits.native().map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot SQL bounds exceed address space"))?;
 if SNAPSHOT.with(|slot|slot.borrow().is_some()){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"same snapshot instance still owns its original retirement ticket"))}
 let maximum=usize::try_from(pure::operation_begin().map_err(refusal)?).map_err(|_|refusal(2))?;
 let frame=size_of::<SnapshotOperation>();
 if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<frame{admitted(pure::operation_finish(0))?;return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot component recipient frame exceeds original grant"))}
 let owned=Cell::new(0usize);reservation(&owned,maximum,frame)?;
 let ticket=NEXT_TICKET.with(|next|{let ticket=next.get().checked_add(1).ok_or_else(||refusal(4))?;next.set(ticket);Ok::<_,ValueError>(ticket)})?;
 let output=ControlledRetirement::new(None::<SnapshotOutput>).map_err(|(error,_)|error)?;
 SNAPSHOT.with(|slot|*slot.borrow_mut()=Some(Box::new(SnapshotOperation{ticket,maximum,owned:frame,body:grant.maximum_copy_bytes,decoding:matches!(direction,IoNativeDirection::Decode),return_cursor:0,refusal:0,native:None,decode:NativeDecodeRetirementRecipient::new(),encode:NativeEncodeRetirementRecipient::new(),output})));
 SNAPSHOT.with(|slot|{
  let mut slot=slot.borrow_mut();let state=slot.as_mut().unwrap();
  let report=|completed:usize,total:usize|pure::operation_progress(completed as u64,total as u64,owned.get()as u64)==0;
  let mut sql_observer=|next:semio_framework::sqlite_snapshot::SqliteSnapshotProgress|report(next.completed,next.total);let mut sql_allocate=|next:NativeEncodeAllocation|reservation(&owned,maximum,next.bytes);
  let mut sql=semio_framework::sqlite_snapshot::SqliteSnapshotControl::new_forwarded(&mut sql_observer,limits,&mut sql_allocate);
  let result=match direction{
   IoNativeDirection::Decode=>{let mut observe=|next:semio_framework_value::native_decoding::NativeDecodeProgress|report(next.completed,next.total);let mut allocate=|next:semio_framework_value::native_decoding::NativeDecodeAllocation|reservation(&owned,maximum,next.bytes);let mut native=NativeDecodeControl::new_forwarded(maximum,&mut observe,&mut allocate);native.install_retirement_recipient(&mut state.decode)?;let ceiling=maximum.min(grant.maximum_capacity_bytes-frame);let result=native.scoped_maximum(ceiling,|control|{let mut run=IoRunControl::decoding(control,grant);Ok::<_,ValueError>(operation(input,&mut run,&mut sql))});state.native=Some(NativeReceipt::Decode(match native.detach(){Ok(receipt)=>receipt,Err((error,_))=>return Err(error)}));result},
   IoNativeDirection::Encode=>{let mut observe=|next:semio_framework_value::native_encoding::NativeEncodeProgress|report(next.completed,next.total);let mut allocate=|next:NativeEncodeAllocation|reservation(&owned,maximum,next.bytes);let mut native=NativeEncodeControl::new_forwarded(maximum,&mut observe,&mut allocate);native.install_retirement_recipient(&mut state.encode)?;let ceiling=maximum.min(grant.maximum_capacity_bytes-frame);let result=native.scoped_maximum(ceiling,|control|{let mut run=IoRunControl::encoding(control,grant);Ok::<_,ValueError>(operation(input,&mut run,&mut sql))});state.native=Some(NativeReceipt::Encode(match native.detach(){Ok(receipt)=>receipt,Err((error,_))=>return Err(error)}));result},
  };
  state.owned=owned.get();
  match result{
   Err(error)=>{state.refusal=refusal_code(error.kind);let finished=admitted(pure::operation_finish(state.owned as u64));finished?;Ok(Ok(T::pending(state.status(state.body)?)))},
   Ok(Err(fault))=>{*state.output.original_mut().unwrap()=Some(SnapshotOutput::Fault(fault));admitted(pure::operation_finish(state.owned as u64))?;Ok(Ok(T::pending(state.status(state.body)?)))},
   Ok(Ok(output))=>{*state.output.original_mut().unwrap()=Some(output.retain());let reserved=T::borrow(state.output.original_mut().unwrap().as_ref().unwrap()).unwrap().reserve(component,&mut state.return_cursor);let finished=admitted(pure::operation_finish(state.owned as u64));if reserved.is_err()||finished.is_err(){return Ok(Ok(T::pending(state.status(state.body)?)))}let original=state.output.take_original().flatten().ok_or_else(||refusal(4))?;Ok(Ok(T::take(original).ok_or_else(||refusal(4))?))},
  }
 })
}
/// 📤️ Reattempts the canonical handoff of the original result without reconstructing its backing.
pub fn snapshot_take<T:SnapshotReturn>(ticket:u64,component:bool)->Result<Result<T,Fault>,ValueError>{
 SNAPSHOT.with(|slot|{let mut slot=slot.borrow_mut();let state=slot.as_mut().ok_or_else(||refusal(4))?;if state.ticket!=ticket{return Err(refusal(4))}let Some(output)=state.output.original_mut().and_then(|output|output.as_ref())else{admitted(pure::operation_finish(state.owned as u64))?;return Ok(Ok(T::pending(state.status(state.body)?)))};
 if let Some(output)=T::borrow(output){let reserved=output.reserve(component,&mut state.return_cursor);let finished=admitted(pure::operation_finish(state.owned as u64));if reserved.is_err()||finished.is_err(){return Ok(Ok(T::pending(state.status(state.body)?)))}let original=state.output.take_original().flatten().ok_or_else(||refusal(4))?;return Ok(Ok(T::take(original).ok_or_else(||refusal(4))?))}
 admitted(pure::operation_finish(state.owned as u64))?;let original=state.output.take_original().flatten().ok_or_else(||refusal(4))?;match original{SnapshotOutput::Fault(fault)=>Ok(Err(fault)),_=>Err(refusal(4))}
 })
}
/// ♻️ Applies one original grant to actual recipient or returned output, preserving denied custody.
pub fn snapshot_close(ticket:u64,grant:RetainedCloneGrant)->Result<(sqlite_wire::SnapshotRetirement,RetainedCloneProgress,u32),ValueError>{
 SNAPSHOT.with(|slot|{
  let mut slot=slot.borrow_mut();let state=slot.as_mut().ok_or_else(||refusal(4))?;if state.ticket!=ticket{return Err(refusal(4))}let quoted=state.status(grant.maximum_copy_bytes)?;
  let fits=grant.maximum_items>=quoted.demands.items as usize&&grant.maximum_copy_bytes>=quoted.demands.copy_bytes as usize&&grant.maximum_capacity_bytes>=quoted.demands.capacity_bytes as usize&&grant.maximum_release_bytes>=quoted.demands.release_bytes as usize&&grant.maximum_depth>=quoted.demands.depth as usize;
  if !fits{admitted(pure::operation_finish(state.owned as u64))?;return Ok((quoted,Default::default(),0))}
  if !quoted.native_pending&&!quoted.output_pending{let progress=RetainedCloneProgress{copied_items:1,copied_bytes:quoted.demands.copy_bytes as usize,released_bytes:size_of::<SnapshotOperation>(),..Default::default()};let owned=state.owned;admitted(pure::operation_finish(owned as u64))?;drop(slot.take());return Ok((sqlite_wire::SnapshotRetirement{ticket,owned_bytes:owned as u64,terminal:true,..Default::default()},progress,0))}
  let owned=Cell::new(state.owned);let maximum=state.maximum;
  let result=match state.native.take().ok_or_else(||refusal(4))?{
   NativeReceipt::Decode(receipt)=>{let mut observe=|next:semio_framework_value::native_decoding::NativeDecodeProgress|pure::operation_progress(next.completed as u64,next.total as u64,owned.get()as u64)==0;let mut allocate=|next:semio_framework_value::native_decoding::NativeDecodeAllocation|reservation(&owned,maximum,next.bytes);let mut native=match NativeForwardedDecodeControl::rebind(receipt,&mut observe,&mut allocate,&mut state.decode){Ok(native)=>native,Err((error,receipt))=>{state.native=Some(NativeReceipt::Decode(receipt));return Err(error)}};let ceiling=native.maximum_bytes().min(native.owned_bytes().saturating_add(grant.maximum_capacity_bytes));let result=native.scoped_maximum(ceiling,|control|if quoted.native_pending{control.close_retirement_recipient(grant)}else{control.charge(quoted.demands.capacity_bytes as usize)?;state.output.step(grant)});state.native=Some(NativeReceipt::Decode(match native.detach(){Ok(receipt)=>receipt,Err((error,_))=>return Err(error)}));result},
   NativeReceipt::Encode(receipt)=>{let mut observe=|next:semio_framework_value::native_encoding::NativeEncodeProgress|pure::operation_progress(next.completed as u64,next.total as u64,owned.get()as u64)==0;let mut allocate=|next:NativeEncodeAllocation|reservation(&owned,maximum,next.bytes);let mut native=match NativeForwardedEncodeControl::rebind(receipt,&mut observe,&mut allocate,&mut state.encode){Ok(native)=>native,Err((error,receipt))=>{state.native=Some(NativeReceipt::Encode(receipt));return Err(error)}};let ceiling=native.maximum_bytes().min(native.owned_bytes().saturating_add(grant.maximum_capacity_bytes));let result=native.scoped_maximum(ceiling,|control|if quoted.native_pending{control.close_retirement_recipient(grant)}else{control.charge(quoted.demands.capacity_bytes as usize)?;state.output.step(grant)});state.native=Some(NativeReceipt::Encode(match native.detach(){Ok(receipt)=>receipt,Err((error,_))=>return Err(error)}));result},
  };
  state.owned=owned.get();let finished=admitted(pure::operation_finish(state.owned as u64));let (progress,mut cause)=match result{Ok(step)=>(step.progress(),0),Err(error)=>(error.retained_progress(),refusal_code(error.kind))};if let Err(error)=finished{if cause==0{cause=refusal_code(error.kind);}}Ok((state.status(state.body)?,progress,cause))
 })
}

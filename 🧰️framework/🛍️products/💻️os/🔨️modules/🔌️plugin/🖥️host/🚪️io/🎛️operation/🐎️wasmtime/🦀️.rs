//! 🐎️ Synchronous guest imports suspend around a scalar request while the original caller stays borrowed.
use super::{OriginalOperationReceiving,slot::{OperationSlot,OperationRequest,OperationReply,OperationReturnAllocation}};
use semio_framework_value::{ValueError,ValueRefusalKind,native_encoding::{NativeEncodeProgress,NativeEncodeAllocation}};
use std::{future::{Future,poll_fn},task::{Context,Poll}};
use wasmtime::component::{Accessor,Linker};
/// 🪟️ Native host state owns only the inline request slot and its existing diagnostic surface.
pub(crate) trait OperationHostStore:Send+'static{
 fn operation_slot(&mut self)->&mut OperationSlot;
 fn operation_log(&mut self,level:String,message:String);
 fn operation_now_ms(&mut self)->i64;
 fn operation_trace_span(&mut self,name:String);
}
/// 🚨️ Preserves the original refusal domain without formatting native diagnostic storage.
#[derive(Debug)]
pub(crate) enum ReceiveError { Refused(ValueError), Runtime(wasmtime::Error) }
/// 🫴️ Keeps an actual lifted DTO even when finishing its original receiving phase refuses.
pub(crate) struct Received<R>{pub output:R,pub refusal:Option<ValueError>}
fn code(result:Result<(),ValueError>)->OperationReply{OperationReply::Code(result.err().map_or(0,|error|crate::operation_authority::refusal_code(error.kind)))}
fn count(value:u64)->Result<usize,ValueError>{usize::try_from(value).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"foreign native request exceeds receiving address space"))}
fn resolve(receiver:&mut dyn OriginalOperationReceiving,request:OperationRequest)->OperationReply{
 match request{
  OperationRequest::Begin=>OperationReply::Begin(receiver.begin().map(|maximum|maximum as u64).map_err(|error|crate::operation_authority::refusal_code(error.kind))),
  OperationRequest::Progress{completed,total,owned}=>code((||receiver.progress(NativeEncodeProgress{completed:count(completed)?,total:count(total)?,owned_bytes:count(owned)?}))()),
  OperationRequest::Allocation{bytes,owned,next,maximum}=>code((||receiver.allocation(NativeEncodeAllocation{bytes:count(bytes)?,owned_bytes:count(owned)?,next_owned_bytes:count(next)?,maximum_bytes:count(maximum)?}))()),
  OperationRequest::ReserveReturn{kind,count:amount}=>code((||allocation_bytes(kind,count(amount)?).and_then(|bytes|receiver.reserve_return(bytes)))()),
  OperationRequest::Finish{owned}=>code(count(owned).and_then(|owned|receiver.finish(owned)))
 }
}
/// 📏️ Quotes the real receiving platform's generated DTO layouts before canonical lifting.
pub(crate) fn allocation_bytes(kind:OperationReturnAllocation,count:usize)->Result<usize,ValueError>{
 use super::return_types::{ui,effects,reactor};let size=match kind{OperationReturnAllocation::Bytes|OperationReturnAllocation::StringBytes=>1,OperationReturnAllocation::U64List=>std::mem::size_of::<u64>(),OperationReturnAllocation::UiPatchList=>std::mem::size_of::<ui::UiPatch>(),OperationReturnAllocation::PatchOpList=>std::mem::size_of::<ui::PatchOp>(),OperationReturnAllocation::EffectList=>std::mem::size_of::<effects::Effect>(),OperationReturnAllocation::PresenceUpdateList=>std::mem::size_of::<reactor::PresenceUpdate>(),OperationReturnAllocation::StringPairList=>std::mem::size_of::<(String,String)>()};count.checked_mul(size).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"canonical return exceeds receiving address space"))
}
pub(crate) fn allocation_kind(kind:super::return_types::pure::OperationReturnAllocation)->OperationReturnAllocation{use super::return_types::pure::OperationReturnAllocation as Kind;match kind{Kind::Bytes=>OperationReturnAllocation::Bytes,Kind::StringBytes=>OperationReturnAllocation::StringBytes,Kind::U64List=>OperationReturnAllocation::U64List,Kind::UiPatchList=>OperationReturnAllocation::UiPatchList,Kind::PatchOpList=>OperationReturnAllocation::PatchOpList,Kind::EffectList=>OperationReturnAllocation::EffectList,Kind::PresenceUpdateList=>OperationReturnAllocation::PresenceUpdateList,Kind::StringPairList=>OperationReturnAllocation::StringPairList}}
/// 🔎️ Measures borrowed native return facts after lifting without charging their admitted storage again.
pub(crate) fn return_bytes(walk:impl FnOnce(&mut dyn FnMut(super::return_types::pure::OperationReturnAllocation,usize)->Result<(),ValueError>)->Result<(),ValueError>)->Result<usize,ValueError>{let mut total=0usize;walk(&mut|kind,count|{total=total.checked_add(allocation_bytes(allocation_kind(kind),count)?).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"canonical return shape exceeds address space"))?;Ok(())})?;Ok(total)}
pub(crate) async fn request<T:OperationHostStore>(accessor:&Accessor<T>,request:OperationRequest)->OperationReply{let mut posted=None;poll_fn(|context|accessor.with(|mut access|access.data_mut().operation_slot().poll_request(request,&mut posted,context))).await}
fn drain<T:OperationHostStore>(accessor:&Accessor<T>,receiver:&mut dyn OriginalOperationReceiving)->Result<(),ValueError>{if let Some(request)=accessor.with(|mut access|access.data_mut().operation_slot().take_request()){let reply=resolve(receiver,request);accessor.with(|mut access|access.data_mut().operation_slot().reply(reply)).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original operation reply lost its receiving request"))?;}Ok(())}
/// 🔌️ Registers every pure import and suspends only original operation admission requests.
pub(crate) fn register<T:OperationHostStore>(linker:&mut Linker<T>)->wasmtime::Result<()>{
 linker.allow_shadowing(true);
 let result=(||{
  let mut interface=linker.instance("semio:framework/pure@1.0.0")?;
  interface.func_wrap("log",|mut store:wasmtime::StoreContextMut<'_,T>,(level,message):(String,String)|{store.data_mut().operation_log(level,message);Ok(())})?;
  interface.func_wrap("now-ms",|mut store:wasmtime::StoreContextMut<'_,T>,():()|Ok((store.data_mut().operation_now_ms(),)))?;
  interface.func_wrap("trace-span",|mut store:wasmtime::StoreContextMut<'_,T>,(name,):(String,)|{store.data_mut().operation_trace_span(name);Ok(())})?;
  interface.func_wrap_concurrent("operation-begin",|accessor,():()|Box::pin(async move{Ok((match request(accessor,OperationRequest::Begin).await{OperationReply::Begin(value)=>value,_=>Err(4)},))}))?;
  interface.func_wrap_concurrent("operation-progress",|accessor,(completed,total,owned):(u64,u64,u64)|Box::pin(async move{Ok((match request(accessor,OperationRequest::Progress{completed,total,owned}).await{OperationReply::Code(value)=>value,_=>4},))}))?;
  interface.func_wrap_concurrent("operation-allocation",|accessor,(bytes,owned,next,maximum):(u64,u64,u64,u64)|Box::pin(async move{Ok((match request(accessor,OperationRequest::Allocation{bytes,owned,next,maximum}).await{OperationReply::Code(value)=>value,_=>4},))}))?;
  interface.func_wrap_concurrent("operation-reserve-return",|accessor,(kind,count):(super::return_types::pure::OperationReturnAllocation,u64)|Box::pin(async move{Ok((match request(accessor,OperationRequest::ReserveReturn{kind:allocation_kind(kind),count}).await{OperationReply::Code(value)=>value,_=>4},))}))?;
  interface.func_wrap_concurrent("operation-finish",|accessor,(owned,):(u64,)|Box::pin(async move{Ok((match request(accessor,OperationRequest::Finish{owned}).await{OperationReply::Code(value)=>value,_=>4},))}))?;
  Ok(())
 })();linker.allow_shadowing(false);result
}
/// 🛂️ Pumps exact requests with the original borrowed caller before resuming the guest or publishing.
pub(crate) async fn receive<T:OperationHostStore,R,F:Future<Output=wasmtime::Result<R>>>(accessor:&Accessor<T>,receiver:&mut dyn OriginalOperationReceiving,operation:F)->Result<R,ReceiveError>{
 accessor.with(|mut access|access.data_mut().operation_slot().arm()).map_err(|_|ReceiveError::Refused(ValueError::literal(ValueRefusalKind::InvariantViolated,"original operation receiving slot is occupied")))?;
 let mut operation=std::pin::pin!(operation);
 let result=poll_fn(|context:&mut Context<'_>|{
  accessor.with(|mut access|access.data_mut().operation_slot().set_pump_waker(context.waker()));
  if let Err(error)=drain(accessor,receiver){return Poll::Ready(Err(ReceiveError::Refused(error)))}
  let result=operation.as_mut().poll(context);
  if let Err(error)=drain(accessor,receiver){return Poll::Ready(Err(ReceiveError::Refused(error)))}
  match result{Poll::Ready(Ok(output))=>Poll::Ready(receiver.ensure_finished().map(|_|output).map_err(ReceiveError::Refused)),Poll::Ready(Err(error))=>Poll::Ready(Err(ReceiveError::Runtime(error))),Poll::Pending=>Poll::Pending}
 }).await;
 accessor.with(|mut access|access.data_mut().operation_slot().disarm(1));result
}
/// 🧳️ Snapshot owners retain both the same instance and returned DTO on a finish denial.
pub(crate) async fn receive_retained<T:OperationHostStore,R,F:Future<Output=wasmtime::Result<R>>>(accessor:&Accessor<T>,receiver:&mut dyn OriginalOperationReceiving,operation:F)->Result<Received<R>,ReceiveError>{
 accessor.with(|mut access|access.data_mut().operation_slot().arm()).map_err(|_|ReceiveError::Refused(ValueError::literal(ValueRefusalKind::InvariantViolated,"original snapshot receiving slot is occupied")))?;
 let mut operation=std::pin::pin!(operation);
 let result=poll_fn(|context:&mut Context<'_>|{
  accessor.with(|mut access|access.data_mut().operation_slot().set_pump_waker(context.waker()));
  if let Err(error)=drain(accessor,receiver){return Poll::Ready(Err(ReceiveError::Refused(error)))}
  let result=operation.as_mut().poll(context);let refusal=drain(accessor,receiver).err();
  match result{Poll::Ready(Ok(output))=>Poll::Ready(Ok(Received{output,refusal:refusal.or_else(||receiver.ensure_finished().err())})),Poll::Ready(Err(error))=>Poll::Ready(Err(ReceiveError::Runtime(error))),Poll::Pending=>match refusal{Some(error)=>Poll::Ready(Err(ReceiveError::Refused(error))),None=>Poll::Pending}}
 }).await;
 accessor.with(|mut access|access.data_mut().operation_slot().disarm(1));result
}

//! 📨️ Inline scalar rendezvous keeps original admission outside a static foreign runtime store.
use std::task::{Context,Poll,Waker};

/// 📐️ Closed native receiving layouts belong to the actual return DTO, never to a guest byte estimate.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OperationReturnAllocation{Bytes,StringBytes,U64List,UiPatchList,PatchOpList,EffectList,PresenceUpdateList,StringPairList}
/// 🧭️ Carries exact fixed-width import arguments before a guest resumes physical work.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OperationRequest{Begin,Progress{completed:u64,total:u64,owned:u64},Allocation{bytes:u64,owned:u64,next:u64,maximum:u64},ReserveReturn{kind:OperationReturnAllocation,count:u64},Finish{owned:u64}}
/// 📬️ Returns only the original receiver's typed grant or refusal.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OperationReply{Begin(Result<u64,u32>),Code(u32)}
impl OperationRequest{fn refusal(self,code:u32)->OperationReply{match self{Self::Begin=>OperationReply::Begin(Err(code)),_=>OperationReply::Code(code)}}fn accepts(self,reply:OperationReply)->bool{matches!((self,reply),(Self::Begin,OperationReply::Begin(_)))||!matches!(self,Self::Begin)&&matches!(reply,OperationReply::Code(_))}}
/// 🔗️ Identifies only the originally armed receiving operation.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OperationLease(u64);
/// 🪟️ Owns one scalar request and reply while borrowing executor wake handles.
pub struct OperationSlot{generation:u64,active:bool,closed:u32,request:Option<OperationRequest>,inflight:Option<OperationRequest>,reply:Option<OperationReply>,pump:Option<Waker>,guest:Option<Waker>}
impl OperationSlot{
 /// 🌱️ Starts without allocating, granting capacity or retaining an original caller.
 pub fn new()->Self{Self{generation:0,active:false,closed:6,request:None,inflight:None,reply:None,pump:None,guest:None}}
 /// 🚦️ Enables requests only for an actual outer receiving pump.
 pub fn arm(&mut self)->Result<(),u32>{if self.active||self.request.is_some()||self.inflight.is_some()||self.reply.is_some(){return Err(4)}self.generation=self.generation.checked_add(1).ok_or(4u32)?;self.active=true;self.closed=1;Ok(())}
 /// 🔔️ Borrows the current outer executor's wake identity.
 pub fn set_pump_waker(&mut self,waker:&Waker){if self.pump.as_ref().is_none_or(|old|!old.will_wake(waker)){self.pump=Some(waker.clone());}}
 /// ⏳️ Suspends the guest until its exact request is answered by the original caller.
 pub fn poll_request(&mut self,request:OperationRequest,posted:&mut Option<OperationLease>,context:&mut Context<'_>)->Poll<OperationReply>{
  if !self.active{return Poll::Ready(request.refusal(self.closed))}
  if posted.is_some_and(|lease|lease.0!=self.generation){return Poll::Ready(request.refusal(4))}
  if posted.is_some(){if self.inflight!=Some(request)&&self.request!=Some(request){return Poll::Ready(request.refusal(4))}if let Some(reply)=self.reply.take(){self.inflight=None;self.guest=None;return Poll::Ready(reply)}}
  else{if self.request.is_some()||self.inflight.is_some()||self.reply.is_some(){return Poll::Ready(request.refusal(4))}self.request=Some(request);*posted=Some(OperationLease(self.generation));if let Some(pump)=&self.pump{pump.wake_by_ref();}}
  if self.guest.as_ref().is_none_or(|old|!old.will_wake(context.waker())){self.guest=Some(context.waker().clone());}Poll::Pending
 }
 /// 📤️ Transfers one pending request exactly once to the receiving pump.
 pub fn take_request(&mut self)->Option<OperationRequest>{let request=self.request.take()?;self.inflight=Some(request);Some(request)}
 /// 🛂️ Publishes only the reply shape belonging to the live original request.
 pub fn reply(&mut self,reply:OperationReply)->Result<(),u32>{if !self.active||self.reply.is_some()||!self.inflight.is_some_and(|request|request.accepts(reply)){return Err(4)}self.reply=Some(reply);if let Some(guest)=&self.guest{guest.wake_by_ref();}Ok(())}
 /// 🛑️ Revokes pending grants and wakes both sides before another guest physical effect.
 pub fn disarm(&mut self,code:u32){self.active=false;self.closed=if code==0{4}else{code};self.request=None;self.inflight=None;self.reply=None;if let Some(guest)=self.guest.take(){guest.wake();}if let Some(pump)=self.pump.take(){pump.wake();}}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

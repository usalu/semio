//! 🧭️ Typed original producer actions retain actual replies across interrupted Source turns.
#[path="🎟️effect/🦀️.rs"] pub mod effect;
use crate::{OriginalSourceTurnIdentity,ValueError,ValueRefusalKind};
#[path="📊️progress/🦀️.rs"]
pub mod progress;
use std::sync::atomic::{AtomicUsize,Ordering};
static NEXT:AtomicUsize=AtomicUsize::new(1);
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original recorded authority or action changed")}
/// 🪪️ Opaque identity of one original producer or action.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalRecordedIdentity(usize);
impl OriginalRecordedIdentity{fn issue()->Self{Self(NEXT.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|n|n.checked_add(1)).expect("original recorded identity exhausted"))}}
/// 📏️ Immutable source span and original ownership observation.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalSpanRequest{pub units:usize,pub owned_bytes:usize}
/// 🚦️ Immutable original progress checkpoint observation.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalCheckpointRequest{pub completed:usize,pub total:usize,pub owned_bytes:usize}
/// 📬️ A typed reply is deposited directly into the caller-owned action.
pub struct OriginalRecordedReply<'a,T>{slot:&'a mut Option<Result<T,ValueError>>}
/// 🧾️ The producer completes or leaves its borrowed reply pending.
pub struct OriginalRecordedReplyComplete<'a,T>{_slot:&'a mut Option<Result<T,ValueError>>}
impl<'a,T> OriginalRecordedReply<'a,T>{
 /// 📨️ Records the producer's actual verdict before returning or unwinding.
 pub fn complete(self,result:Result<T,ValueError>)->OriginalRecordedReplyComplete<'a,T>{assert!(self.slot.is_none(),"original recorded reply occupied");*self.slot=Some(result);OriginalRecordedReplyComplete{_slot:self.slot}}
 /// ⏳️ Leaves the original action pending without manufacturing a verdict.
 pub fn pending(self)->OriginalRecordedReplyComplete<'a,T>{assert!(self.slot.is_none(),"original recorded reply occupied");OriginalRecordedReplyComplete{_slot:self.slot}}
}
/// 🏭️ A resident typed producer validates each actual borrowed Source turn.
pub trait OriginalRecordedProducer{
 type Request:Copy;
 type Reply;
 type Context<'turn,'source>:?Sized where 'source:'turn;
 fn validate_context<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn;
 fn source_turn<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn;
 fn receive<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,request:Self::Request,reply:OriginalRecordedReply<'reply,Self::Reply>)->Result<OriginalRecordedReplyComplete<'reply,Self::Reply>,ValueError> where 'source:'turn;
 fn recover<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,request:Self::Request,reply:OriginalRecordedReply<'reply,Self::Reply>)->Result<OriginalRecordedReplyComplete<'reply,Self::Reply>,ValueError> where 'source:'turn;
}
/// 🧳️ Owns the exact original request, producer entry and typed reply.
pub struct OriginalRecordedAction<R:Copy,T>{authority:OriginalRecordedIdentity,identity:OriginalRecordedIdentity,source_turn:Option<OriginalSourceTurnIdentity>,request:R,entered:bool,settled:bool,reply:Option<Result<T,ValueError>>,finished:bool}
impl<R:Copy,T> OriginalRecordedAction<R,T>{
 /// 📏️ Observes the original immutable request.
 pub fn request(&self)->R{self.request}
 /// 🪪️ Observes the actual action identity.
 pub fn identity(&self)->OriginalRecordedIdentity{self.identity}
 /// 🚦️ Reports whether the producer supplied its actual reply.
 pub fn is_settled(&self)->bool{self.settled}
 /// 📨️ Transfers only the original producer's completed reply.
 pub fn take_reply(&mut self)->Option<Result<T,ValueError>>{if self.settled{self.reply.take()}else{None}}
}
impl<R:Copy,T> Drop for OriginalRecordedAction<R,T>{fn drop(&mut self){assert!(std::thread::panicking()||self.finished,"original recorded action requires its same authority to finish")}}
impl<R:Copy+Send+'static,T:Send+'static> crate::retirement::RetireOwned for OriginalRecordedAction<R,T>{
 type RetirementCursor=crate::NativeRetirementGate<Self,<() as crate::retirement::RetireOwned>::RetirementCursor>;
 fn retirement_ready(&self)->bool{false}
 fn retirement_seed(self)->Self::RetirementCursor{crate::NativeRetirementGate::Pending(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{None}
 fn controlled_retirement_supported()->bool{true}
}
enum Invocation<'a,R:Copy,T>{Validate,Receive{request:R,entered:&'a mut bool,reply:&'a mut Option<Result<T,ValueError>>},Recover{request:R,reply:&'a mut Option<Result<T,ValueError>>}}
enum InvocationResult{Validated,Called(Result<(),ValueError>),Context(ValueError)}
/// 🧭️ Owns one actual typed producer and its one outstanding action.
pub struct OriginalRecordedAuthority<P:OriginalRecordedProducer>{port:std::mem::ManuallyDrop<P>,identity:OriginalRecordedIdentity,active:Option<OriginalRecordedIdentity>}
impl<P:OriginalRecordedProducer> OriginalRecordedAuthority<P>{
 /// 🎟️ Creates a stable original producer authority.
 pub fn new(port:P)->Self{Self{port:std::mem::ManuallyDrop::new(port),identity:OriginalRecordedIdentity::issue(),active:None}}
 /// 🪪️ Identifies the actual resident producer.
 pub fn identity(&self)->OriginalRecordedIdentity{self.identity}
 /// ⏳️ Reports an outstanding original action.
 pub fn has_pending_action(&self)->bool{self.active.is_some()}
 /// 🔎️ Observes the actual resident producer without transferring it.
 pub fn inspect_port<R>(&self,query:impl FnOnce(&P)->R)->R{query(&self.port)}
 /// 🫴️ Returns the producer only when its action and reply have returned.
 pub fn into_port(self)->Result<P,Self>{if self.active.is_some(){return Err(self)}let mut owner=std::mem::ManuallyDrop::new(self);Ok(unsafe{std::mem::ManuallyDrop::take(&mut owner.port)})}
 /// 🤝️ Lends this original producer against its validated current context.
 pub fn with_loan<'turn,'source,R>(&mut self,context:&mut P::Context<'turn,'source>,operation:impl FnOnce(&mut dyn OriginalRecordedLoan<P::Request,P::Reply>)->R)->Result<R,ValueError> where 'source:'turn{self.port.validate_context(context)?;let source_turn=self.port.source_turn(context);let identity=self.identity;let active=&mut self.active;let port=&mut *self.port;let mut callback=|invocation:Invocation<'_,P::Request,P::Reply>|{if let Err(error)=port.validate_context(context){return InvocationResult::Context(error)}if port.source_turn(context)!=source_turn{return InvocationResult::Context(invalid())}match invocation{Invocation::Validate=>InvocationResult::Validated,Invocation::Receive{request,entered,reply}=>{*entered=true;InvocationResult::Called(port.receive(context,request,OriginalRecordedReply{slot:reply}).map(drop))},Invocation::Recover{request,reply}=>InvocationResult::Called(port.recover(context,request,OriginalRecordedReply{slot:reply}).map(drop))}};let mut loan=OriginalRecordedAuthorityLoan{identity,source_turn,active,callback:&mut callback};Ok(operation(&mut loan))}
}
#[cfg(not(target_arch="wasm32"))]
impl<P:OriginalRecordedProducer+Send> OriginalRecordedAuthority<P>{
 /// 🚚️ Lends the same original producer to native Send execution.
 pub fn with_send_loan<'turn,'source,R>(&mut self,context:&mut P::Context<'turn,'source>,operation:impl FnOnce(&mut dyn SendOriginalRecordedLoan<P::Request,P::Reply>)->R)->Result<R,ValueError> where 'source:'turn,P::Context<'turn,'source>:Send{self.port.validate_context(context)?;let source_turn=self.port.source_turn(context);let identity=self.identity;let active=&mut self.active;let port=&mut *self.port;let mut callback=|invocation:Invocation<'_,P::Request,P::Reply>|{if let Err(error)=port.validate_context(context){return InvocationResult::Context(error)}if port.source_turn(context)!=source_turn{return InvocationResult::Context(invalid())}match invocation{Invocation::Validate=>InvocationResult::Validated,Invocation::Receive{request,entered,reply}=>{*entered=true;InvocationResult::Called(port.receive(context,request,OriginalRecordedReply{slot:reply}).map(drop))},Invocation::Recover{request,reply}=>InvocationResult::Called(port.recover(context,request,OriginalRecordedReply{slot:reply}).map(drop))}};let mut loan=OriginalRecordedAuthorityLoan{identity,source_turn,active,callback:&mut callback};Ok(operation(&mut loan))}
}
#[cfg(target_arch="wasm32")]
impl<P:OriginalRecordedProducer> OriginalRecordedAuthority<P>{
 /// 🌐️ Lends the original local producer in guest execution.
 pub fn with_send_loan<'turn,'source,R>(&mut self,context:&mut P::Context<'turn,'source>,operation:impl FnOnce(&mut dyn SendOriginalRecordedLoan<P::Request,P::Reply>)->R)->Result<R,ValueError> where 'source:'turn{self.port.validate_context(context)?;let source_turn=self.port.source_turn(context);let identity=self.identity;let active=&mut self.active;let port=&mut *self.port;let mut callback=|invocation:Invocation<'_,P::Request,P::Reply>|{if let Err(error)=port.validate_context(context){return InvocationResult::Context(error)}if port.source_turn(context)!=source_turn{return InvocationResult::Context(invalid())}match invocation{Invocation::Validate=>InvocationResult::Validated,Invocation::Receive{request,entered,reply}=>{*entered=true;InvocationResult::Called(port.receive(context,request,OriginalRecordedReply{slot:reply}).map(drop))},Invocation::Recover{request,reply}=>InvocationResult::Called(port.recover(context,request,OriginalRecordedReply{slot:reply}).map(drop))}};let mut loan=OriginalRecordedAuthorityLoan{identity,source_turn,active,callback:&mut callback};Ok(operation(&mut loan))}
}
impl<P:OriginalRecordedProducer> Drop for OriginalRecordedAuthority<P>{fn drop(&mut self){assert!(std::thread::panicking()||self.active.is_none(),"original recorded authority retains its action");if self.active.is_none(){unsafe{std::mem::ManuallyDrop::drop(&mut self.port)}}}}
mod sealed{pub trait Sealed{}}
/// 🧭️ A sealed loan exposes only transitions against its original typed producer.
pub trait OriginalRecordedLoan<R:Copy,T>:sealed::Sealed{
 fn identity(&self)->OriginalRecordedIdentity;
 fn source_turn(&self)->Option<OriginalSourceTurnIdentity>;
 fn has_pending_action(&self)->bool;
 fn validate_context(&mut self)->Result<(),ValueError>;
 fn matches_action(&self,action:Option<&OriginalRecordedAction<R,T>>)->bool;
 fn begin(&mut self,request:R,slot:&mut Option<OriginalRecordedAction<R,T>>)->Result<(),ValueError>;
 fn recover(&mut self,action:&mut OriginalRecordedAction<R,T>)->Result<bool,ValueError>;
 fn finish(&mut self,slot:&mut Option<OriginalRecordedAction<R,T>>)->Result<(),ValueError>;
}
/// 🚚️ A native loan retains the Send original producer.
#[cfg(not(target_arch="wasm32"))]
pub trait SendOriginalRecordedLoan<R:Copy,T>:OriginalRecordedLoan<R,T>+Send{}
/// 🌐️ A guest loan retains its original local producer.
#[cfg(target_arch="wasm32")]
pub trait SendOriginalRecordedLoan<R:Copy,T>:OriginalRecordedLoan<R,T>{}
struct OriginalRecordedAuthorityLoan<'a,F:?Sized>{identity:OriginalRecordedIdentity,source_turn:Option<OriginalSourceTurnIdentity>,active:&'a mut Option<OriginalRecordedIdentity>,callback:&'a mut F}
impl<F:?Sized> OriginalRecordedAuthorityLoan<'_,F>{
 fn validate<R:Copy,T>(&mut self)->Result<(),ValueError> where F:for<'a> FnMut(Invocation<'a,R,T>)->InvocationResult{match (self.callback)(Invocation::Validate){InvocationResult::Validated=>Ok(()),InvocationResult::Context(error)=>Err(error),_=>unreachable!()}}
 fn matches<R:Copy,T>(&self,action:Option<&OriginalRecordedAction<R,T>>)->bool{action.is_some_and(|a|a.authority==self.identity&&a.source_turn==self.source_turn&&*self.active==Some(a.identity)&&!a.finished)}
 fn invoke<R:Copy,T>(&mut self,action:&mut OriginalRecordedAction<R,T>,recover:bool)->Result<bool,ValueError> where F:for<'a> FnMut(Invocation<'a,R,T>)->InvocationResult{let request=action.request;let result=if recover&&action.entered{(self.callback)(Invocation::Recover{request,reply:&mut action.reply})}else{(self.callback)(Invocation::Receive{request,entered:&mut action.entered,reply:&mut action.reply})};match result{InvocationResult::Called(result)=>{result?;if action.reply.is_some(){action.settled=true;Ok(true)}else{Ok(false)}},InvocationResult::Context(error)=>Err(error),InvocationResult::Validated=>unreachable!()}}
}
impl<F:?Sized> sealed::Sealed for OriginalRecordedAuthorityLoan<'_,F>{}
impl<R:Copy,T,F:for<'a> FnMut(Invocation<'a,R,T>)->InvocationResult+?Sized> OriginalRecordedLoan<R,T> for OriginalRecordedAuthorityLoan<'_,F>{
 fn identity(&self)->OriginalRecordedIdentity{self.identity}
 fn source_turn(&self)->Option<OriginalSourceTurnIdentity>{self.source_turn}
 fn has_pending_action(&self)->bool{self.active.is_some()}
 fn validate_context(&mut self)->Result<(),ValueError>{self.validate()}
 fn matches_action(&self,action:Option<&OriginalRecordedAction<R,T>>)->bool{self.matches(action)}
 fn begin(&mut self,request:R,slot:&mut Option<OriginalRecordedAction<R,T>>)->Result<(),ValueError>{self.validate::<R,T>()?;if self.active.is_some()||slot.is_some(){return Err(invalid())}let identity=OriginalRecordedIdentity::issue();*self.active=Some(identity);*slot=Some(OriginalRecordedAction{authority:self.identity,identity,source_turn:self.source_turn,request,entered:false,settled:false,reply:None,finished:false});self.invoke(slot.as_mut().expect("original action installed"),false).map(drop)}
 fn recover(&mut self,action:&mut OriginalRecordedAction<R,T>)->Result<bool,ValueError>{self.validate::<R,T>()?;if !self.matches(Some(action))||action.settled{return Err(invalid())}if action.reply.is_some(){action.settled=true;return Ok(true)}self.invoke(action,true)}
 fn finish(&mut self,slot:&mut Option<OriginalRecordedAction<R,T>>)->Result<(),ValueError>{self.validate::<R,T>()?;let action=slot.as_mut().ok_or_else(invalid)?;if !self.matches(Some(action))||!action.settled||action.reply.is_some(){return Err(invalid())}action.finished=true;*self.active=None;drop(slot.take());Ok(())}
}
#[cfg(not(target_arch="wasm32"))]
impl<R:Copy,T,F:for<'a> FnMut(Invocation<'a,R,T>)->InvocationResult+Send+?Sized> SendOriginalRecordedLoan<R,T> for OriginalRecordedAuthorityLoan<'_,F>{}
#[cfg(target_arch="wasm32")]
impl<R:Copy,T,F:for<'a> FnMut(Invocation<'a,R,T>)->InvocationResult+?Sized> SendOriginalRecordedLoan<R,T> for OriginalRecordedAuthorityLoan<'_,F>{}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

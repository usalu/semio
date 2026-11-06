//! 🎟️ One resident producer records its actual effect reply before return or unwind.
use semio_framework_value_control_contract::{OriginalSourceTurnIdentity,OriginalWorkActionIdentity,RetainedAllocationEffect,ValueError,ValueRefusalKind};
use std::sync::atomic::{AtomicU64,Ordering};
static IDS:AtomicU64=AtomicU64::new(1);
fn next()->u64{IDS.try_update(Ordering::Relaxed,Ordering::Relaxed,|n|n.checked_add(1)).expect("original effect identity exhausted")}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original effect producer or receipt changed")}
pub use semio_framework_value_control_contract::{OriginalEffectReply,OriginalEffectReplyComplete};
/// 🏭️ Supplies an actual resident producer and separately borrowed original context.
pub trait OriginalEffectProducer{
 type Output;
 type Failure;
 type Context<'turn,'source> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn;
 fn source_turn<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn;
 fn receive<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'reply,Self::Output,Self::Failure>,ValueError> where 'source:'turn;
 fn recover<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'reply,Self::Output,Self::Failure>,ValueError> where 'source:'turn;
}
/// 🚦️ Reports whether the original producer must start, recover, or transfer its recorded reply.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalEffectCallbackMode{Receive,Recover,Recorded}
/// 🧳️ Holds one original producer reply and its same Source and work action.
pub struct OriginalEffectAction<T,E>{authority:u64,identity:u64,source:Option<OriginalSourceTurnIdentity>,work:OriginalWorkActionIdentity,reply:Option<Result<T,E>>,settled:bool,finished:bool}
impl<T,E> OriginalEffectAction<T,E>{
 /// 📤️ Transfers the exact producer reply directly into its empty resident slot.
 pub fn transfer_reply(&mut self,output:&mut Option<Result<T,E>>)->Result<bool,ValueError>{if output.is_some(){return Err(invalid())}if !self.settled{return Ok(false)}let Some(reply)=self.reply.take()else{return Err(invalid())};*output=Some(reply);Ok(true)}
}
impl<T,E> Drop for OriginalEffectAction<T,E>{fn drop(&mut self){assert!(std::thread::panicking()||self.finished,"original effect producer action remains live")}}
/// 🏠️ Owns the actual producer and prevents replacement while an action is live.
pub struct OriginalEffectAuthority<P:OriginalEffectProducer>{producer:std::mem::ManuallyDrop<P>,identity:Option<u64>,active:Option<u64>}
impl<P:OriginalEffectProducer> OriginalEffectAuthority<P>{
 /// 🌱️ Creates an authority over this actual producer.
 pub fn new(producer:P)->Self{Self{producer:std::mem::ManuallyDrop::new(producer),identity:None,active:None}}
 /// 👁️ Observes the actual resident producer without lending its authority.
 pub fn inspect<R>(&self,query:impl FnOnce(&P)->R)->R{query(&self.producer)}
 /// 👁️ Borrows the actual original producer while preserving exclusive action custody.
 pub fn producer(&self)->&P{&self.producer}
 /// ⏳️ Reports whether the original producer action still has custody.
 pub fn has_pending_action(&self)->bool{self.active.is_some()}
 /// 🫴️ Returns the actual producer only after its original action finishes.
 pub fn into_producer(self)->Result<P,Self>{if self.active.is_some(){return Err(self)}let mut owner=std::mem::ManuallyDrop::new(self);Ok(unsafe{std::mem::ManuallyDrop::take(&mut owner.producer)})}
 /// 🔎️ Validates the actual producer, Source turn and Work action before entry.
 pub fn callback_mode<'turn,'source>(&self,context:&P::Context<'turn,'source>,effect:&RetainedAllocationEffect<'_>,action:Option<&OriginalEffectAction<P::Output,P::Failure>>)->Result<OriginalEffectCallbackMode,ValueError> where 'source:'turn{self.producer.validate_context(context)?;let source=self.producer.source_turn(context);if let(Some(parent),Some(payer))=(source,effect.work_source_turn()){if parent!=payer{return Err(invalid())}}match action{None if self.active.is_none()=>Ok(OriginalEffectCallbackMode::Receive),Some(action) if Some(action.authority)==self.identity&&self.active==Some(action.identity)&&action.source==source&&action.work==effect.work_identity().action_identity()&&!action.finished=>Ok(if action.settled||action.reply.is_some(){OriginalEffectCallbackMode::Recorded}else{OriginalEffectCallbackMode::Recover}),_=>Err(invalid())}}
 /// 🪢️ Starts or recovers only the matching producer and actual work action.
 pub fn drive<'turn,'source>(&mut self,context:&mut P::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,slot:&mut Option<OriginalEffectAction<P::Output,P::Failure>>)->Result<bool,ValueError> where 'source:'turn{
  self.producer.validate_context(context)?;let source=self.producer.source_turn(context);if let (Some(parent),Some(payer))=(source,effect.work_source_turn()){if parent!=payer{return Err(invalid())}}let work=effect.work_identity().action_identity();let fresh=slot.is_none();if fresh{if self.active.is_some(){return Err(invalid())}let authority=*self.identity.get_or_insert_with(next);let identity=next();self.active=Some(identity);*slot=Some(OriginalEffectAction{authority,identity,source,work,reply:None,settled:false,finished:false})}let action=slot.as_mut().expect("original effect action installed");if Some(action.authority)!=self.identity||self.active!=Some(action.identity)||action.source!=source||action.work!=work||action.finished{return Err(invalid())}if action.settled{return Ok(true)}if action.reply.is_some(){action.settled=true;return Ok(true)}let reply=OriginalEffectReply::borrow(&mut action.reply)?;let complete=if fresh{self.producer.receive(context,effect,reply)}else{self.producer.recover(context,effect,reply)}?;drop(complete);if action.reply.is_some(){action.settled=true;Ok(true)}else{Ok(false)}
 }
 /// 🏁️ Releases the same action after its actual reply has transferred.
 pub fn finish(&mut self,slot:&mut Option<OriginalEffectAction<P::Output,P::Failure>>)->Result<(),ValueError>{let action=slot.as_mut().ok_or_else(invalid)?;if Some(action.authority)!=self.identity||self.active!=Some(action.identity)||!action.settled||action.reply.is_some(){return Err(invalid())}action.finished=true;self.active=None;drop(slot.take());Ok(())}
}
impl<P:OriginalEffectProducer> Drop for OriginalEffectAuthority<P>{fn drop(&mut self){assert!(std::thread::panicking(),"original effect producer requires transfer or paid retirement")}}
impl<P:OriginalEffectProducer+crate::retirement::RetireOwned+Send+'static> crate::retirement::RetireOwned for OriginalEffectAuthority<P>{
 type RetirementCursor=crate::NativeRetirementGate<Self,P::RetirementCursor>;
 fn retirement_ready(&self)->bool{self.active.is_none()&&crate::retirement::RetireOwned::retirement_ready(&*self.producer)}
 fn retirement_seed(self)->Self::RetirementCursor{if !crate::retirement::RetireOwned::retirement_ready(&self){return crate::NativeRetirementGate::Pending(self)}let producer=self.into_producer().ok().expect("ready original producer");crate::NativeRetirementGate::Ready(crate::retirement::RetireOwned::retirement_seed(producer))}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_ready(self).then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}

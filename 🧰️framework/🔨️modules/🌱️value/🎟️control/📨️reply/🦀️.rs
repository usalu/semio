//! 📨️ Borrowed replies record actual results before producer return and clear only completed forwarding evidence.
use semio_framework_value_control_physical::{RetainedCloneStep,ValueError,ValueRefusalKind};

fn occupied()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original effect reply is already occupied")}
fn identity<T>(value:T)->T{value}
trait Forward<T,E>:Send{
 fn is_recorded(&self)->bool;
 fn write(&mut self,result:Result<T,E>);
}
enum Slot<'a,T,E>{Direct(&'a mut Option<Result<T,E>>),Forward(&'a mut dyn Forward<T,E>)}
impl<T,E> Slot<'_,T,E>{
 fn is_recorded(&self)->bool{match self{Self::Direct(slot)=>slot.is_some(),Self::Forward(port)=>port.is_recorded()}}
 fn write(&mut self,result:Result<T,E>){assert!(!self.is_recorded(),"original effect reply occupied");match self{Self::Direct(slot)=>**slot=Some(result),Self::Forward(port)=>port.write(result)}}
 fn reborrow(&mut self)->Slot<'_,T,E>{match self{Self::Direct(slot)=>Slot::Direct(&mut**slot),Self::Forward(port)=>Slot::Forward(&mut**port)}}
}
/// 📥️ Lends only the actual caller-owned result slot; no action, Source or currency is issued.
pub struct OriginalEffectReply<'a,T,E>{slot:Slot<'a,T,E>,normalize:fn(T)->T}
/// ✅️ Proves the producer recorded a result or explicitly retained its pending reply.
pub struct OriginalEffectReplyComplete<'a,T,E>{slot:Slot<'a,T,E>}
impl<T,E> OriginalEffectReplyComplete<'_,T,E>{
 /// 👁️ Observes the same actual result slot without taking or reconstructing its verdict.
 pub fn is_recorded(&self)->bool{self.slot.is_recorded()}
}
impl<'a,T,E> OriginalEffectReply<'a,T,E>{
 pub fn borrow(slot:&'a mut Option<Result<T,E>>)->Result<Self,ValueError>{if slot.is_some(){return Err(occupied())}Ok(Self{slot:Slot::Direct(slot),normalize:identity})}
 /// 👁️ Observes only whether the original producer has deposited its result.
 pub fn is_recorded(&self)->bool{self.slot.is_recorded()}
 /// 🔄️ Lends the same slot and native normalization for one shorter producer call.
 pub fn reborrow(&mut self)->Result<OriginalEffectReply<'_,T,E>,ValueError>{if self.is_recorded(){return Err(occupied())}Ok(OriginalEffectReply{slot:self.slot.reborrow(),normalize:self.normalize})}
 fn write(&mut self,result:Result<T,E>){self.slot.write(result.map(self.normalize))}
 pub fn complete(mut self,result:Result<T,E>)->OriginalEffectReplyComplete<'a,T,E>{self.write(result);OriginalEffectReplyComplete{slot:self.slot}}
 pub fn pending(self)->OriginalEffectReplyComplete<'a,T,E>{assert!(!self.slot.is_recorded(),"original effect reply occupied");OriginalEffectReplyComplete{slot:self.slot}}
 /// 🧵️ Returns the actual forwarded status after a nested borrowed producer has returned.
 pub fn forwarded(self)->OriginalEffectReplyComplete<'a,T,E>{OriginalEffectReplyComplete{slot:self.slot}}
 /// 🪪️ Retains the caller's original opaque invocation key until actual result deposit.
 /// The key comes from the defining authority; this scope authenticates no Source and grants no work.
 pub fn scope<'scope,K:Copy+Eq+Send>(self,resident:&'scope mut Option<K>,key:K)->Result<OriginalEffectReplyScope<'scope,'a,T,E,K>,ValueError> where T:Send,E:Send{
  if self.is_recorded(){return Err(occupied())}let recovering=resident.is_some();if resident.is_some_and(|original|original!=key){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded reply changed its original invocation"))}*resident=Some(key);Ok(OriginalEffectReplyScope{original:self,resident,recovering})
 }
}
impl<E> OriginalEffectReply<'_,RetainedCloneStep,E>{
 /// 🪆️ Records a completed child's physical progress without declaring its parent complete.
 pub fn progress(mut self)->Self{self.normalize=|step|RetainedCloneStep::Progress(step.progress());self}
}
/// 🧷️ Forwards into the same original reply and clears its exact key inside completed deposit.
pub struct OriginalEffectReplyScope<'scope,'reply,T,E,K>{original:OriginalEffectReply<'reply,T,E>,resident:&'scope mut Option<K>,recovering:bool}
impl<T:Send,E:Send,K:Copy+Eq+Send> Forward<T,E> for OriginalEffectReplyScope<'_,'_,T,E,K>{
 fn is_recorded(&self)->bool{self.original.is_recorded()}
 fn write(&mut self,result:Result<T,E>){self.original.write(result);*self.resident=None}
}
impl<'scope,'reply,T:Send,E:Send,K:Copy+Eq+Send> OriginalEffectReplyScope<'scope,'reply,T,E,K>{
 pub fn recovering(&self)->bool{self.recovering}
 pub fn is_recorded(&self)->bool{self.original.is_recorded()}
 pub fn borrow(&mut self)->Result<OriginalEffectReply<'_,T,E>,ValueError>{if self.original.is_recorded(){return Err(occupied())}Ok(OriginalEffectReply{slot:Slot::Forward(self),normalize:identity})}
 pub fn forwarded(self)->OriginalEffectReplyComplete<'reply,T,E>{self.original.forwarded()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="🏭️producer-engine/🦀️.rs"]
mod producer_engine;
#[path="🎟️work-lender/🦀️.rs"]
mod work_lender;

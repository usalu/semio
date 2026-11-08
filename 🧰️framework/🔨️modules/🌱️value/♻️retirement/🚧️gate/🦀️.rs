//! 🚧️ Retains unresolved actual owners until their original recovery completes.
use crate::{ValueError,ValueRefusalKind};
/// 🚧️ Retains an unresolved original owner or an inline ready retirement cursor.
pub enum NativeRetirementGate<P,C>{Pending(P),Ready(C)}
impl<P,C> NativeRetirementGate<P,C>{
 /// 🫴️ Returns the unchanged unresolved owner for its original recovery path.
 pub fn into_pending(self)->Result<P,Self>{match self{Self::Pending(original)=>Ok(original),ready=>Err(ready)}}
}
fn pending_retirement_refusal()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original allocation owner requires recovery before retirement")}
impl<P:Send+'static,C:crate::retirement::RetirementCursor+'static> crate::retirement::RetirementCursor for NativeRetirementGate<P,C>{
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,crate::retirement::RetirementStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,crate::retirement::RetirementStep,crate::OriginalRetirementFailure>,ValueError>{match self{Self::Ready(cursor)=>cursor.close_into(body,effect,reply),Self::Pending(_)=>Ok(reply.complete(Err(pending_retirement_refusal().into())))}}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,crate::retirement::RetirementStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,crate::retirement::RetirementStep,crate::OriginalRetirementFailure>,ValueError>{match self{Self::Ready(cursor)=>cursor.recover_close_into(body,effect,reply),Self::Pending(_)=>Ok(reply.complete(Err(pending_retirement_refusal().into())))}}
 fn close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,crate::retirement::RetirementStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,crate::retirement::RetirementStep,crate::OriginalRetirementFailure>,ValueError>{match self{Self::Ready(cursor)=>cursor.close_original_into(original,body,effect,reply),Self::Pending(_)=>Ok(reply.complete(Err(pending_retirement_refusal().into())))}}
 fn recover_close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,crate::retirement::RetirementStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,crate::retirement::RetirementStep,crate::OriginalRetirementFailure>,ValueError>{match self{Self::Ready(cursor)=>cursor.recover_close_original_into(original,body,effect,reply),Self::Pending(_)=>Ok(reply.complete(Err(pending_retirement_refusal().into())))}}

 fn requires_original_birth(&self)->bool{match self{Self::Ready(cursor)=>cursor.requires_original_birth(),Self::Pending(_)=>false}}
 fn close_original_birth_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,grant:crate::RetainedCloneGrant)->crate::retirement::RetirementStep{match self{Self::Ready(cursor)=>cursor.close_original_birth_step(original,grant),Self::Pending(_)=>crate::retirement::RetirementStep::Failure(pending_retirement_refusal().into(),0)}}
 fn requires_original_control(&self)->bool{match self{Self::Ready(cursor)=>cursor.requires_original_control(),Self::Pending(_)=>false}}
 fn close_step_controlled(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->crate::retirement::RetirementStep{match self{Self::Ready(cursor)=>cursor.close_step_controlled(body,effect,control),Self::Pending(_)=>crate::retirement::RetirementStep::Failure(pending_retirement_refusal().into(),0)}}
 fn close_original_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::retirement::RetirementStep{match self{Self::Ready(cursor)=>cursor.close_original_step(original,body,effect),Self::Pending(_)=>crate::retirement::RetirementStep::Failure(pending_retirement_refusal().into(),0)}}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{match self{Self::Ready(cursor)=>cursor.next_computational_work(body),Self::Pending(_)=>Err(pending_retirement_refusal())}}
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{match self{Self::Ready(cursor)=>cursor.next_demand(body),Self::Pending(_)=>Err(pending_retirement_refusal())}}
 
 fn terminal_is_empty(&self)->bool{match self{Self::Ready(cursor)=>cursor.terminal_is_empty(),Self::Pending(_)=>false}}
 fn next_depth_demand(&self)->Result<usize,ValueError>{match self{Self::Ready(cursor)=>cursor.next_depth_demand(),Self::Pending(_)=>Err(pending_retirement_refusal())}}
 fn next_close_byte_demand(&self)->Option<usize>{match self{Self::Ready(cursor)=>cursor.next_close_byte_demand(),Self::Pending(_)=>None}}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{match self{Self::Ready(cursor)=>cursor.next_work_byte_demand(),Self::Pending(_)=>Err(pending_retirement_refusal())}}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{match self{Self::Ready(cursor)=>cursor.next_selected_work_byte_demand(body),Self::Pending(_)=>Err(pending_retirement_refusal())}}
 fn allows_admitted_narrow_work(&self)->bool{match self{Self::Ready(cursor)=>cursor.allows_admitted_narrow_work(),Self::Pending(_)=>false}}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{match self{Self::Ready(cursor)=>cursor.next_birth_bytes(body),Self::Pending(_)=>None}}
 fn terminal_release_bytes(&self)->Option<usize>{crate::retirement::RetirementCursor::terminal_is_empty(self).then_some(std::mem::size_of::<Self>())}
}
impl<P:Send+'static,C:crate::retirement::RetirementCursor+'static> NativeRetirementGate<P,C>{

}


impl<P:semio_framework_value_control_contract::OriginalEffectProducer+crate::retirement::RetireOwned+Send+'static> crate::retirement::RetireOwned for semio_framework_value_control_contract::OriginalEffectAuthority<P>{
 type RetirementCursor=NativeRetirementGate<Self,P::RetirementCursor>;
 fn retirement_ready(&self)->bool{!self.has_pending_action()&&crate::retirement::RetireOwned::retirement_ready(self.producer())}
 fn retirement_seed(self)->Self::RetirementCursor{if !crate::retirement::RetireOwned::retirement_ready(&self){return NativeRetirementGate::Pending(self)}let producer=self.into_producer().ok().expect("ready original producer");NativeRetirementGate::Ready(crate::retirement::RetireOwned::retirement_seed(producer))}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_ready(self).then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}



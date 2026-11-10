//! 🪞️ Original native receiving accepts an actual root source or its genuine retained projection.
use super::*;
use crate::retained_clone::RetainedOwnedProjection;
pub(super) enum OriginalInput<I:Sync+'static,A:RetireOwned+Sync>{Source(RetainedCloneSource<I,A>),Projection(RetainedOwnedProjection<I>)}
impl<I:Sync+'static,A:RetireOwned+Sync> OriginalInput<I,A>{
 pub(super) fn try_borrow(&self)->Result<RetainedCloneRef<'_,I>,ValueError>{match self{Self::Source(source)=>source.try_borrow(),Self::Projection(projection)=>projection.borrow()}}
 pub(super) fn next_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{match self{Self::Source(source)=>source.next_close_demand(body),Self::Projection(projection)=>projection.next_close_demand(body)}}
 pub(super) fn close_dispatch<'r>(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{match(self,original,recover){(Self::Source(x),None,false)=>x.close_into(body,effect,reply),(Self::Source(x),None,true)=>x.recover_close_into(body,effect,reply),(Self::Source(x),Some(original),false)=>x.close_original_into(original,body,effect,reply),(Self::Source(x),Some(original),true)=>x.recover_close_original_into(original,body,effect,reply),(Self::Projection(x),None,false)=>x.close_into(body,effect,reply),(Self::Projection(x),None,true)=>x.recover_close_into(body,effect,reply),(Self::Projection(x),Some(original),false)=>x.close_original_into(original,body,effect,reply),(Self::Projection(x),Some(original),true)=>x.recover_close_original_into(original,body,effect,reply),}}
 pub(super) fn next_close_work(&self,body:usize)->Result<u64,ValueError>{match self{Self::Source(source)=>source.next_close_work_demand(body),Self::Projection(projection)=>projection.close_work_demand(body)}}
 pub(super) fn terminal_is_empty(&self)->bool{match self{Self::Source(source)=>source.terminal_is_empty(),Self::Projection(projection)=>projection.terminal_is_empty()}}
}
impl<I:Sync+'static,T:NativeReceive<I>,A:RetireOwned+Sync> NativeReceivingOriginal<I,T,A>{
 /// 🧷️ Consumes one genuine original field projection without cloning or serializing its input.
 pub fn from_projection(original:RetainedOwnedProjection<I>)->Self{Self{original:ManuallyDrop::new(Some(OriginalInput::Projection(original))),receiver:semio_framework_value_native_storage::NativeOwnedStorage::empty(),retired_header:None,output:ManuallyDrop::new(None),retirement:Default::default(),action:Default::default(),authority:ManuallyDrop::new(Some(OriginalEffectAuthority::new(NativeSlotProducer::new()))),backing:Default::default(),close_reply:ManuallyDrop::new(None),close_route:None,closing:false}}
}

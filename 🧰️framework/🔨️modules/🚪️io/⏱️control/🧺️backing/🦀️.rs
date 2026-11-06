//! 🧺️ Exact vector and text backings use the same original native allocation transaction.
use std::mem::size_of;
use super::{NativeSnapshotDecodeOwner,NativeSnapshotEncodeOwner,NativeSnapshotDecodeAllocation,NativeSnapshotEncodeAllocation,NativeSnapshotEffectProgress,NativeSnapshotFailure,RetainedCloneGrant,RetainedWorkOutcome,RetainedCloneProgress,ValueError,ValueRefusalKind};
macro_rules! exact_backing{
 ($owner:ident,$allocation:ident)=>{
  impl $owner<'_,'_>{
   /// 🧺️ Creates an exact vector backing in the caller-retained output slot.
   pub fn allocate_vector<T>(&mut self,count:usize,output:&mut Option<Vec<T>>,marker:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,allocation:&mut $allocation,depth:usize)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
    if depth==0{return RetainedWorkOutcome{work:0,result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot backing requires an original depth grant").into())}}if output.is_some()||allocation.is_unsettled(){return RetainedWorkOutcome{work:0,result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot vector requires empty retained slots").into())}}let layout=match std::alloc::Layout::array::<T>(count){Ok(layout)=>layout,Err(_)=>return RetainedWorkOutcome{work:0,result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot vector layout exceeds native limit").into())}};let bytes=layout.size();let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:bytes,maximum_depth:depth,..Default::default()};
    if bytes==0{return self.effect(demand,marker,unwind,&mut |performed,physical|{*output=Some(Vec::new());*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<T>>(),..Default::default()};Ok(())})}
    self.effect_allocated(bytes,demand,marker,unwind,allocation,&mut |performed,physical,actual|{*performed=true;*actual=Some(0);let pointer=unsafe{std::alloc::alloc(layout)}.cast::<T>();if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"snapshot vector backing allocation failed"))}*output=Some(unsafe{Vec::from_raw_parts(pointer,0,count)});*actual=Some(bytes);*physical=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<T>>(),retained_capacity_bytes:bytes,..Default::default()};Ok(())})
   }
   /// 🔤️ Creates an exact empty text backing in the caller-retained output slot.
   pub fn allocate_string(&mut self,bytes:usize,output:&mut Option<String>,marker:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,allocation:&mut $allocation,depth:usize)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
    if depth==0{return RetainedWorkOutcome{work:0,result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot backing requires an original depth grant").into())}}if output.is_some()||allocation.is_unsettled(){return RetainedWorkOutcome{work:0,result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot text requires empty retained slots").into())}}let layout=match std::alloc::Layout::array::<u8>(bytes){Ok(layout)=>layout,Err(_)=>return RetainedWorkOutcome{work:0,result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot text layout exceeds native limit").into())}};let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<String>(),maximum_capacity_bytes:bytes,maximum_depth:depth,..Default::default()};
    if bytes==0{return self.effect(demand,marker,unwind,&mut |performed,physical|{*output=Some(String::new());*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<String>(),..Default::default()};Ok(())})}
    self.effect_allocated(bytes,demand,marker,unwind,allocation,&mut |performed,physical,actual|{*performed=true;*actual=Some(0);let pointer=unsafe{std::alloc::alloc(layout)};if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"snapshot text backing allocation failed"))}*output=Some(unsafe{String::from_utf8_unchecked(Vec::from_raw_parts(pointer,0,bytes))});*actual=Some(bytes);*physical=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<String>(),retained_capacity_bytes:bytes,..Default::default()};Ok(())})
   }
  }
 };
}
exact_backing!(NativeSnapshotDecodeOwner,NativeSnapshotDecodeAllocation);
exact_backing!(NativeSnapshotEncodeOwner,NativeSnapshotEncodeAllocation);

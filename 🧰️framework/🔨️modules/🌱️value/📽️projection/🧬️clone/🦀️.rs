//! 🌳️ Immutable clone cursors retain their native projection at one funded allocated address.
use super::{ImmutableRootCursor,ImmutableRootProjection,NativeProjectionBacking,NativeSeedTemplate,SnapshotProjectionCursor};
use crate::{ValueError,RetirementDemand,OriginalRetirementFailure,OriginalRetirementPoll,retirement::RetireOwned,retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneRef,RetainedCloneStep,RetainedCloneGrant,RetainedAllocationEffect,OriginalEffectReply,OriginalEffectReplyComplete,OriginalRetirementCloseLoan,RetainedCursorRelease,RetainedCloneOwner}};
use std::mem::{ManuallyDrop,MaybeUninit,size_of};
use semio_framework_value_native_clone_backing::{NativeRetainedCloneBacking,NativeRetainedCloneState};
type Reply<'a>=OriginalEffectReply<'a,RetainedCloneStep,OriginalRetirementFailure>;
type Complete<'a>=OriginalEffectReplyComplete<'a,RetainedCloneStep,OriginalRetirementFailure>;
fn invalid()->ValueError{ValueError::invariant("immutable clone changed its retained native projection")}
impl<S:ImmutableRootProjection> ImmutableRootCursor<S>{
 const fn clone_seed_template()->NativeSeedTemplate<Self>{unsafe{NativeSeedTemplate::fields(8,Self::clone_seed_size,Self::clone_seed_write)}}
 unsafe fn clone_seed_size(target:*mut MaybeUninit<Self>,index:usize)->Option<usize>{
  fn extent<T>(_:*mut T)->usize{size_of::<T>()}
  let target=target.cast::<Self>();Some(match index{
   0=>extent(unsafe{std::ptr::addr_of_mut!((*target).binding)}),
   1=>extent(unsafe{std::ptr::addr_of_mut!((*target).output)}),
   2=>extent(unsafe{std::ptr::addr_of_mut!((*target).retirement)}),
   3=>size_of::<bool>(),4=>size_of::<bool>(),5=>size_of::<bool>(),
   6=>extent(unsafe{std::ptr::addr_of_mut!((*target).project_route)}),
   7=>extent(unsafe{std::ptr::addr_of_mut!((*target).close_route)}),
   _=>return None,
  })
 }
 unsafe fn clone_seed_write(target:*mut MaybeUninit<Self>,index:usize)->bool{
  let target=target.cast::<Self>();unsafe{match index{
   0=>std::ptr::addr_of_mut!((*target).binding).write(ManuallyDrop::new(None)),
   1=>std::ptr::addr_of_mut!((*target).output).write(ManuallyDrop::new(None)),
   2=>std::ptr::addr_of_mut!((*target).retirement).write(ManuallyDrop::new(crate::retained_clone::RetainedCloneClose::vacant())),
   3=>std::ptr::addr_of_mut!((*target).projected).write(false),
   4=>std::ptr::addr_of_mut!((*target).admitted).write(false),
   5=>std::ptr::addr_of_mut!((*target).closing).write(false),
   6=>std::ptr::addr_of_mut!((*target).project_route).write(None),
   7=>std::ptr::addr_of_mut!((*target).close_route).write(None),
   _=>return false,
  }}true
 }
}

/// 🪴️ Uses the same native bounded clone owner as other first-party retained cursors.
pub type ImmutableRootCloneCursor<S:ImmutableRootProjection+RetainedClone> = NativeRetainedCloneBacking<S,ImmutableRootCursor<S>>;
unsafe impl<S:ImmutableRootProjection+RetainedClone> NativeRetainedCloneState<S> for ImmutableRootCursor<S>{
 const NATIVE_SEED:NativeSeedTemplate<Self>=Self::clone_seed_template();
 fn has_pending_native_action(&self)->bool{self.project_route.is_some()||self.close_route.is_some()}
}
impl<S:ImmutableRootProjection+RetainedClone> RetainedCloneCursor<S> for ImmutableRootCursor<S>{
 fn advance_demands(&self,source:RetainedCloneRef<'_,S>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{<Self as SnapshotProjectionCursor<S>>::demands(self,source,body,maximum)}
 fn advance_work_demand(&self,source:RetainedCloneRef<'_,S>,body:usize,maximum:u64)->Result<u64,ValueError>{<Self as SnapshotProjectionCursor<S>>::advance_work_demand(self,source,body,maximum)}
 fn advance_into<'r>(&mut self,source:RetainedCloneRef<'_,S>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{<Self as SnapshotProjectionCursor<S>>::advance_into(self,source,body,maximum,effect,reply)}
 fn recover_advance_into<'r>(&mut self,source:RetainedCloneRef<'_,S>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{<Self as SnapshotProjectionCursor<S>>::recover_advance_into(self,source,body,maximum,effect,reply)}
 fn output_ready(&self)->bool{<Self as SnapshotProjectionCursor<S>>::ready(self)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{<Self as SnapshotProjectionCursor<S>>::take_demand(self)}
 fn take_work_demand(&self)->Result<u64,ValueError>{<Self as SnapshotProjectionCursor<S>>::take_work_demand(self)}
 fn take<'r>(&mut self,output:&mut Option<S>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{<Self as SnapshotProjectionCursor<S>>::take_into(self,output,effect,reply)}
 fn recover_take<'r>(&mut self,output:&mut Option<S>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{<Self as SnapshotProjectionCursor<S>>::recover_take_into(self,output,effect,reply)}
 fn close_started(&self)->bool{<Self as SnapshotProjectionCursor<S>>::close_started(self)}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{if self.project_route.is_some()||self.close_route.is_some(){return Err(ValueError::invariant("native immutable clone retains its original callback"))}<Self as SnapshotProjectionCursor<S>>::begin_close_demand(self)}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{RetainedCloneCursor::<S>::begin_close_demand(self)?;<Self as SnapshotProjectionCursor<S>>::begin_close_work_demand(self)}
 fn begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{<Self as SnapshotProjectionCursor<S>>::begin_close_into(self,effect,reply)}
 fn recover_begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{<Self as SnapshotProjectionCursor<S>>::recover_begin_close_into(self,effect,reply)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{<Self as SnapshotProjectionCursor<S>>::close_demands(self,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{<Self as SnapshotProjectionCursor<S>>::close_work_demand(self,body)}
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{<Self as SnapshotProjectionCursor<S>>::close_into(self,body,effect,reply)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{<Self as SnapshotProjectionCursor<S>>::recover_close_into(self,body,effect,reply)}
 fn requires_original_birth(&self)->bool{<Self as SnapshotProjectionCursor<S>>::requires_original_birth(self)}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{<Self as SnapshotProjectionCursor<S>>::close_original_birth_step(self,original,grant)}
 fn requires_original_control(&self)->bool{<Self as SnapshotProjectionCursor<S>>::requires_original_control(self)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{<Self as SnapshotProjectionCursor<S>>::close_original_into(self,original,body,effect,reply)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{<Self as SnapshotProjectionCursor<S>>::recover_close_original_into(self,original,body,effect,reply)}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(<Self as SnapshotProjectionCursor<S>>::close_demands(self,self.close_route.map_or(0,|route|route.4))?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(<Self as SnapshotProjectionCursor<S>>::close_demands(self,body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(<Self as SnapshotProjectionCursor<S>>::close_demands(self,self.close_route.map_or(0,|route|route.4))?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(<Self as SnapshotProjectionCursor<S>>::close_demands(self,self.close_route.map_or(0,|route|route.4))?.depth)}
 fn terminal_is_empty(&self)->bool{<Self as SnapshotProjectionCursor<S>>::terminal_is_empty(self)}
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{<Self as SnapshotProjectionCursor<S>>::native_terminal_release(self)}
}

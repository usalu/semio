//! 🔗️ Guarded immutable projections rooted in genuine retained source ownership.

use super::{RetainedCloneBinding, RetainedCloneProjection, RetainedCloneRef};
use crate::{ValueError, ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::{mem::{ManuallyDrop,size_of},ptr::NonNull};

/// 🧷️ Keeps a native field stable through its immutable root lease until explicit alias closure.
pub struct RetainedOwnedProjection<T: ?Sized + Sync> {
    value: NonNull<T>,
    binding: ManuallyDrop<Option<RetainedCloneBinding>>,
    projection: RetainedCloneProjection,
}

/// 🚚️ Shared native references remain immutable and their root owner is Send + Sync.
unsafe impl<T: ?Sized + Sync> Send for RetainedOwnedProjection<T> {}
/// 🪢️ Root-backed projections expose only shared references tied to their own borrow.
unsafe impl<T: ?Sized + Sync> Sync for RetainedOwnedProjection<T> {}

impl<T: ?Sized + Sync> RetainedOwnedProjection<T> {
    /// 📏️ Quotes the original sealed alias before any projection callback executes.
    pub fn constructor_demand()->crate::RetirementDemand { crate::RetirementDemand { copy_bytes:RetainedCloneBinding::alias_copy_bytes()+std::mem::size_of::<Self>(),depth:1,..Default::default() } }
    pub const fn constructor_work_demand()->u64{RetainedCloneBinding::alias_work_demand()+1}
    pub(super) fn admit_constructor(grant:RetainedCloneGrant)->Result<(),ValueError> {
        if grant.maximum_items==0 { return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original projection requires one admitted item")); }
        if grant.maximum_depth==0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original projection requires admitted depth")); }
        if grant.maximum_copy_bytes<Self::constructor_demand().copy_bytes { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original projection alias copy is not funded")); }
        Ok(())
    }
    /// 📖️ Borrows the projected field for the lifetime of this actual owning alias.
    pub fn borrow(&self) -> Result<RetainedCloneRef<'_, T>, ValueError> {
        let lease = self.binding.as_ref().and_then(|binding|binding.lease.as_ref()).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "owned projection is closed"))?;
        Ok(RetainedCloneRef { value: unsafe { self.value.as_ref() }, lease, projection: self.projection })
    }

    /// 🪪️ Observes the same issued source and exact projected path without constructing another alias.
    pub fn matches_source(&self,source:RetainedCloneRef<'_,T>)->bool{self.binding.as_ref().is_some_and(|binding|source.matches_binding(binding))}

    /// ♻️ Releases one alias while the source authority continues to retain the root lease.
    pub fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll {let grant=effect.grant();
        RetainedCloneBinding::close_one(&mut self.binding,body,effect)
    }
    pub fn requires_original_birth(&self)->bool{RetainedCloneBinding::requires_original_birth(&self.binding)}
    pub fn requires_original_control(&self)->bool{RetainedCloneBinding::requires_original_control(&self.binding)}
    /// 📥️ Lends the actual original alias close reply without moving its owner.
    pub fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{RetainedCloneBinding::close_into(&mut self.binding,body,effect,reply)}
    /// 🔁️ Recovers the same recorded alias close without selecting another child.
    pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{RetainedCloneBinding::recover_close_into(&mut self.binding,body,effect,reply)}
    /// 🧵️ Lends the same original close control to the retained alias producer.
    pub fn close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{RetainedCloneBinding::close_original_into(&mut self.binding,original,body,effect,reply)}
    /// 🪡️ Recovers the same original-control alias close and reply.
    pub fn recover_close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{RetainedCloneBinding::recover_close_original_into(&mut self.binding,original,body,effect,reply)}

    pub fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{RetainedCloneBinding::close_original_birth_step(&mut self.binding,original,grant)}
    pub fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{RetainedCloneBinding::close_original_step(&mut self.binding,original,body,effect)}
    /// ⛽️ Quotes the selected original alias closure action before effects.
    pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{RetainedCloneBinding::close_work_demand(&self.binding,body)}
    /// 🎟️ Quotes the original alias frontier once under its caller's complete body allowance.
    pub fn close_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{RetainedCloneBinding::close_demand(&self.binding,body)}
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::copy_demand(&self.binding)}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{RetainedCloneBinding::capacity_demand(&self.binding,body)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::release_demand(&self.binding)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::depth_demand(&self.binding)}
    /// 🎯️ Lends the same selected original alias or child event under actual remaining copy authority.
    pub fn next_close_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{RetainedCloneBinding::close_demand(&self.binding,body)}

    /// ✅️ Reports that the projection owns neither a native alias nor a root lease.
    pub fn terminal_is_empty(&self) -> bool { self.binding.is_none() }
}

impl<T: ?Sized + Sync> Drop for RetainedOwnedProjection<T> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "owned projection reached Drop before exact alias closure");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.binding); } }
    }
}




semio_framework_value_native_clone_engine::retained_projection_admission_engine!();
semio_framework_value_native_clone_engine::retained_native_field_projection_engine!();

/// 🧭️ One immutable array index projects only its actual retained native element.
pub struct RetainedNativeArrayIndexProjection<T,const N:usize>{index:usize,marker:std::marker::PhantomData<fn()->T>}
impl<T:Sync,const N:usize> RetainedNativeArrayIndexProjection<T,N>{
 /// 🪪️ Selects a real array position without invoking an accessor.
 pub fn new(index:usize)->Result<Self,ValueError>{if index>=N{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native array projection index is outside its original array"))}Ok(Self{index,marker:std::marker::PhantomData})}
 /// 📏️ Quotes the exact original alias and resident element pointer.
 pub fn demand(&self,source:RetainedCloneRef<'_,[T;N]>,output:&RetainedOwnedProjectionAdmission<T>)->Result<crate::RetirementDemand,ValueError>{if !output.terminal_is_empty(){return Err(ValueError::invariant("native array projection output occupied"))}let mut demand=source.binding_demand(&output.binding)?.ok_or_else(||ValueError::invariant("native array projection binding occupied"))?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<Option<NonNull<T>>>()).ok_or_else(||ValueError::invariant("native array projection extent overflow"))?;Ok(demand)}
 /// ⛽️ Quotes the fixed index selection and real original alias work.
 pub fn work_demand(&self,source:RetainedCloneRef<'_,[T;N]>,output:&RetainedOwnedProjectionAdmission<T>)->Result<u64,ValueError>{self.demand(source,output)?;source.binding_work_demand(&output.binding)?.checked_add(1).ok_or_else(||ValueError::invariant("native array projection Work overflow"))}
 /// 📥️ Publishes the indexed alias under the original Source and borrowed reply.
 pub fn admit_into<'r>(&self,source:RetainedCloneRef<'_,[T;N]>,output:&mut RetainedOwnedProjectionAdmission<T>,effect:&mut crate::retained_clone::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{let demand=self.demand(source,output)?;let work=self.work_demand(source,output)?;if effect.work_source_turn().is_none(){return Err(ValueError::invariant("native array projection lacks original Source"))}if effect.remaining_work_units()?<work||!crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native array projection exceeded original Work"))))}let element=source.project(self.index,|array|&array[self.index]);output.admit(element,effect,reply)}
}

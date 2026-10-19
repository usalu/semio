//! 🔗️ Guarded immutable projections rooted in genuine retained source ownership.

use super::{RetainedCloneBinding, RetainedCloneProjection, RetainedCloneRef};
use crate::{ValueError, ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::{mem::ManuallyDrop,ptr::NonNull};

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
    pub fn constructor_demand()->crate::RetirementDemand { crate::RetirementDemand { copy_bytes:RetainedCloneBinding::alias_copy_bytes(),depth:1,..Default::default() } }
    pub(super) fn admit_constructor(grant:RetainedCloneGrant)->Result<(),ValueError> {
        if grant.maximum_items==0 { return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original projection requires one admitted item")); }
        if grant.maximum_depth==0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original projection requires admitted depth")); }
        if grant.maximum_copy_bytes<Self::constructor_demand().copy_bytes { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original projection alias copy is not funded")); }
        Ok(())
    }
    /// 🛡️ Requires a projection whose lease retains its actual immutable native root.
    pub(super) unsafe fn from_source(source:RetainedCloneRef<'_,T>,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{
        Self::admit_constructor(grant)?;
        let mut binding=None;let progress=source.bind(&mut binding,grant)?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original projection alias omitted its birth"))?;
        if binding.is_none(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original projection alias copy is not funded"));}
        Ok((Self{value:NonNull::from(source.get()),binding:ManuallyDrop::new(binding),projection:source.projection},progress))
    }

    /// 📖️ Borrows the projected field for the lifetime of this actual owning alias.
    pub fn borrow(&self) -> Result<RetainedCloneRef<'_, T>, ValueError> {
        let lease = self.binding.as_ref().and_then(|binding|binding.lease.as_ref()).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "owned projection is closed"))?;
        Ok(RetainedCloneRef { value: unsafe { self.value.as_ref() }, lease, projection: self.projection })
    }

    /// 🪪️ Observes the same issued source and exact projected path without constructing another alias.
    pub fn matches_source(&self,source:RetainedCloneRef<'_,T>)->bool{self.binding.as_ref().is_some_and(|binding|source.matches_binding(binding))}

    /// 🌿️ Projects another native child without allocation or reconstructed field ownership.
    pub fn project<U: ?Sized + Sync, F>(&self, discriminator: usize, project: F,grant:RetainedCloneGrant) -> Result<(RetainedOwnedProjection<U>,RetainedCloneProgress), ValueError>
    where F: for<'source> FnOnce(&'source T) -> &'source U {
        RetainedOwnedProjection::<U>::admit_constructor(grant)?;
        unsafe { RetainedOwnedProjection::from_source(self.borrow()?.project(discriminator, project),grant) }
    }

    /// ♻️ Releases one alias while the source authority continues to retain the root lease.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        RetainedCloneBinding::close_one(&mut self.binding,grant)
    }
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



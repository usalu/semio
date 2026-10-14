//! 📦️ Native mesh page custody in the existing arena, with paid source claims and loans.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mesh3dResourceIdentity {
    pub operation: u64,
    pub generation: u64,
    pub revision: u64,
    pub cache_epoch: u64,
    pub source_slot: usize,
}

#[derive(Debug)]
pub struct Mesh3dResourceOwner {
    lease: Option<Mesh3dLease>,
    identity: Mesh3dResourceIdentity,
    closing: bool,
    native_closing: bool,
}

#[derive(Debug)]
pub struct Mesh3dResourceLoan {
    lease: Option<Mesh3dLease>,
    identity: Mesh3dResourceIdentity,
}

fn metadata_admitted(grant: RetainedCloneGrant) -> bool { grant.maximum_items != 0 && grant.maximum_depth != 0 }

fn metadata_progress() -> RetainedCloneProgress { RetainedCloneProgress { copied_items: 1, ..Default::default() } }

impl Mesh3dResourceOwner {
    pub fn admit(source: &mut Option<Mesh3dLease>, identity: Mesh3dResourceIdentity, grant: RetainedCloneGrant) -> Result<Option<(Self, RetainedCloneProgress)>, Mesh3dFault> {
        let Some(lease) = *source else { return Ok(None); };
        if identity.operation == 0 || identity.generation != lease.generation || identity.revision != lease.revision || identity.cache_epoch == 0 { return Err(Mesh3dFault::Stale); }
        if !metadata_admitted(grant) { return Ok(None); }
        let mut authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let original = authority.ready_mut(lease)?;
        if original.resource_claimed || original.resource_loans != 0 { return Err(Mesh3dFault::Order); }
        original.resource_claimed = true;
        source.take();
        Ok(Some((Self { lease: Some(lease), identity, closing: false, native_closing: false }, metadata_progress())))
    }

    pub fn identity(&self) -> Mesh3dResourceIdentity { self.identity }

    pub fn lease(&self) -> Option<Mesh3dLease> { self.lease }

    pub fn loan(&self, grant: RetainedCloneGrant) -> Result<Option<(Mesh3dResourceLoan, RetainedCloneProgress)>, Mesh3dFault> {
        if self.closing || self.lease.is_none() { return Err(Mesh3dFault::Closing); }
        if !metadata_admitted(grant) { return Ok(None); }
        let lease = self.lease.unwrap();
        let mut authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let original = authority.ready_mut(lease)?;
        if !original.resource_claimed || original.resource_closing { return Err(Mesh3dFault::Closing); }
        let loans = original.resource_loans.checked_add(1).ok_or(Mesh3dFault::ItemCapacity)?;
        original.resource_loans = loans;
        Ok(Some((Mesh3dResourceLoan { lease: Some(lease), identity: self.identity }, metadata_progress())))
    }

    pub fn close_demands(&self, work: usize) -> Result<RetirementDemand, Mesh3dFault> {
        let Some(lease) = self.lease else { return Ok(RetirementDemand::default()); };
        if self.native_closing { return mesh3d_close_demands(lease, work); }
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, Mesh3dFault> {
        let Some(lease) = self.lease else { return Ok(Mesh3dOwnershipStep { complete: true, ..Default::default() }); };
        if self.native_closing {
            let step = mesh3d_close_step(lease, grant)?;
            if step.complete { self.lease.take(); }
            return Ok(step);
        }
        if !metadata_admitted(grant) { return Ok(Mesh3dOwnershipStep::default()); }
        let mut authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let original = authority.ready_mut(lease)?;
        if !original.resource_claimed { return Err(Mesh3dFault::Order); }
        if !self.closing {
            original.resource_closing = true;
            self.closing = true;
            return Ok(Mesh3dOwnershipStep { progress: metadata_progress(), ..Default::default() });
        }
        if original.resource_loans != 0 { return Ok(Mesh3dOwnershipStep::default()); }
        authority.begin_close_resource(lease)?;
        self.native_closing = true;
        Ok(Mesh3dOwnershipStep { progress: metadata_progress(), ..Default::default() })
    }

    pub fn terminal_is_empty(&self) -> bool { self.lease.is_none() }
}

impl Mesh3dResourceLoan {
    pub fn identity(&self) -> Mesh3dResourceIdentity { self.identity }

    pub fn lease(&self) -> Option<Mesh3dLease> { self.lease }

    pub fn lease_ref(&self) -> Option<&Mesh3dLease> { self.lease.as_ref() }

    pub fn loan(&self, grant: RetainedCloneGrant) -> Result<Option<(Self, RetainedCloneProgress)>, Mesh3dFault> {
        let Some(lease) = self.lease else { return Err(Mesh3dFault::Closing); };
        if !metadata_admitted(grant) { return Ok(None); }
        let mut authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let original = authority.ready_mut(lease)?;
        if !original.resource_claimed || original.resource_closing { return Err(Mesh3dFault::Closing); }
        original.resource_loans = original.resource_loans.checked_add(1).ok_or(Mesh3dFault::ItemCapacity)?;
        Ok(Some((Self { lease: Some(lease), identity: self.identity }, metadata_progress())))
    }

    pub fn return_step(&mut self, grant: RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, Mesh3dFault> {
        let Some(lease) = self.lease else { return Ok(Mesh3dOwnershipStep { complete: true, ..Default::default() }); };
        if !metadata_admitted(grant) { return Ok(Mesh3dOwnershipStep::default()); }
        let mut authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let original = authority.ready_mut(lease)?;
        if !original.resource_claimed || original.resource_loans == 0 { return Err(Mesh3dFault::Order); }
        original.resource_loans -= 1;
        self.lease.take();
        Ok(Mesh3dOwnershipStep { complete: true, progress: metadata_progress() })
    }

    pub fn terminal_is_empty(&self) -> bool { self.lease.is_none() }
}

impl Drop for Mesh3dResourceOwner {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "original native mesh resource dropped before paid close"); }
}

impl Drop for Mesh3dResourceLoan {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "original native mesh resource loan dropped before paid return"); }
}

#[path="📥️receiving/🦀️.rs"]
mod receiving;
pub use receiving::{Mesh3dResourceReceiving,Mesh3dReceivingStep,MESH3D_RESOURCE_COMPONENT_COUNT_CAPACITY};

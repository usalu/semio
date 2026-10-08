//! 📦️ Exact typed mutation batches retained across erased member admission and bounded close.

use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::{RetireOwned, controlled::ControlledRetirement}};
use std::{any::Any, mem::{ManuallyDrop, size_of}};

pub(crate) trait BatchRetirement: Send {
    fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
    fn demands(&self) -> Result<(usize, usize), ValueError>;
    fn next_copy_byte_demand(&self) -> usize;
}
impl<M: RetireOwned> BatchRetirement for ControlledRetirement<Vec<M>> {
    fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.step(grant) }
    fn terminal_is_empty(&self) -> bool { self.terminal_is_empty() }
    fn demands(&self) -> Result<(usize, usize), ValueError> { Ok((self.next_capacity_byte_demand(self.next_copy_byte_demand())?, self.next_release_byte_demand()?)) }
    fn next_copy_byte_demand(&self) -> usize { ControlledRetirement::next_copy_byte_demand(self) }
}
impl<M: RetireOwned> BatchRetirement for ControlledRetirement<std::collections::VecDeque<M>> {
    fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.step(grant) }
    fn terminal_is_empty(&self) -> bool { self.terminal_is_empty() }
    fn demands(&self) -> Result<(usize, usize), ValueError> { Ok((self.next_capacity_byte_demand(self.next_copy_byte_demand())?, self.next_release_byte_demand()?)) }
    fn next_copy_byte_demand(&self) -> usize { ControlledRetirement::next_copy_byte_demand(self) }
}
pub(crate) struct SourceRetirementIssuer<M: Send + 'static> {
    pub(crate) birth_bytes: usize,
    pub(crate) maximum_depth: usize,
    pub(crate) begin: fn(std::collections::VecDeque<M>) -> Box<dyn BatchRetirement>,
}
trait BatchOwner: Send {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn close(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
    fn demands(&self) -> Result<(usize, usize), ValueError>;
    fn next_copy_byte_demand(&self) -> usize;
}
struct TypedBatch<M: Send + 'static> {
    values: ManuallyDrop<Option<Vec<M>>>,
    retirement: ManuallyDrop<Option<Box<dyn BatchRetirement>>>,
    birth_bytes: usize,
    retire: fn(Vec<M>) -> Box<dyn BatchRetirement>,
    retire_source: fn(std::collections::VecDeque<M>) -> Box<dyn BatchRetirement>,
    source_birth_bytes: usize,
    source_maximum_depth: usize,
    closing: bool,
}
impl<M: Send + 'static> BatchOwner for TypedBatch<M> {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn close(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.closing = true;
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.values.is_some() {
            if self.birth_bytes > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            *self.retirement = Some((self.retire)(self.values.take().unwrap()));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: self.birth_bytes, ..Default::default() }));
        }
        if let Some(retirement) = self.retirement.as_mut() {
            if !retirement.terminal_is_empty() { return retirement.step(grant); }
            if self.birth_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.retirement.take();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: self.birth_bytes, ..Default::default() }));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    fn terminal_is_empty(&self) -> bool { self.values.is_none() && self.retirement.is_none() }
    fn next_copy_byte_demand(&self) -> usize { self.retirement.as_ref().map_or(0, |retirement| retirement.next_copy_byte_demand()) }
    fn demands(&self) -> Result<(usize, usize), ValueError> {
        if self.values.is_some() { return Ok((self.birth_bytes, 0)); }
        match self.retirement.as_ref() {
            Some(retirement) if retirement.terminal_is_empty() => Ok((0, self.birth_bytes)),
            Some(retirement) => retirement.demands(),
            None => Ok((0, 0)),
        }
    }
}
impl<M: Send + 'static> Drop for TypedBatch<M> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "owned member batch abandoned before exact typed retirement or transfer");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.values); ManuallyDrop::drop(&mut self.retirement); } }
    }
}

/// 📦️ First-party erased owner whose exact mutation type survives every refused admission.
pub struct MemberStoreOwnedBatch { owner: Option<Box<dyn BatchOwner>>, owner_bytes: usize }
impl MemberStoreOwnedBatch {
    /// 📐️ Names the exact erased typed scaffold allocation before ownership is transferred.
    pub fn scaffold_byte_demand<M: RetireOwned>() -> usize { size_of::<TypedBatch<M>>() }
    /// 🪪️ Borrows admission authority before moving the original parent mutation source.
    pub fn admit<M:RetireOwned>(values:&mut Option<Vec<M>>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        if values.is_none()||grant.maximum_items==0||grant.maximum_capacity_bytes<Self::scaffold_byte_demand::<M>(){return Ok(None);}
        if !Vec::<M>::controlled_retirement_supported(){return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"member batch has no controlled typed retirement authority"));}
        match Self::try_new(values.take().unwrap(),grant){
            Ok(admitted)=>Ok(Some(admitted)),
            Err((error,original))=>{*values=Some(original);Err(error)},
        }
    }
    /// 🎟️ Admits only the complete typed scaffold allocation and returns the original vector on refusal.
    pub fn try_new<M: RetireOwned>(values: Vec<M>, grant: RetainedCloneGrant) -> Result<(Self, RetainedCloneProgress), (ValueError, Vec<M>)> {
        if !Vec::<M>::controlled_retirement_supported() { return Err((ValueError::new(ValueRefusalKind::UnsupportedOwner, "member batch has no controlled typed retirement authority"), values)); }
        let owner_bytes = size_of::<TypedBatch<M>>();
        let progress = RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: owner_bytes, ..Default::default() };
        if !progress.fits(grant) { return Err((ValueError::new(ValueRefusalKind::AllocationFailed, "member batch scaffold exceeds its admitted allocation grant"), values)); }
        let owner = TypedBatch { values: ManuallyDrop::new(Some(values)), retirement: ManuallyDrop::new(None), birth_bytes: size_of::<ControlledRetirement<Vec<M>>>(), retire: |values| Box::new(ControlledRetirement::new(values).unwrap_or_else(|_| unreachable!("admitted typed retirement authority remains exact"))), retire_source: |values| Box::new(ControlledRetirement::new(values).unwrap_or_else(|_| unreachable!("exact admitted forward source retirement authority"))), source_birth_bytes: size_of::<ControlledRetirement<std::collections::VecDeque<M>>>(), source_maximum_depth: grant.maximum_depth, closing: false };
        Ok((Self { owner: Some(Box::new(owner)), owner_bytes }, progress))
    }
    /// 🔎️ Borrows the original typed source without cloning or removing any owner.
    pub fn mutations<M: Send + 'static>(&self) -> Option<&[M]> { self.owner.as_ref()?.as_any().downcast_ref::<TypedBatch<M>>()?.values.as_deref() }
    pub(crate) fn source_retirement_issuer<M: Send + 'static>(&self) -> Option<SourceRetirementIssuer<M>> {
        let owner = self.owner.as_ref()?.as_any().downcast_ref::<TypedBatch<M>>()?;
        Some(SourceRetirementIssuer { birth_bytes: owner.source_birth_bytes, maximum_depth: owner.source_maximum_depth, begin: owner.retire_source })
    }
    pub(crate) fn take_mutations<M: Send + 'static>(&mut self) -> Option<Vec<M>> {
        let owner = self.owner.as_mut()?.as_any_mut().downcast_mut::<TypedBatch<M>>()?;
        if owner.closing { return None; }
        owner.values.take()
    }
    pub(crate) fn restore_mutations<M: Send + 'static>(&mut self, values: Vec<M>) {
        let owner = self.owner.as_mut().unwrap().as_any_mut().downcast_mut::<TypedBatch<M>>().expect("restoration uses the exact admitted mutation owner");
        assert!(owner.values.is_none() && owner.retirement.is_none() && !owner.closing);
        *owner.values = Some(values);
    }
    /// 📐️ Returns separate next constructor and indivisible physical release extents.
    pub fn next_demands(&self) -> Result<(usize, usize), ValueError> {
        match self.owner.as_ref() {
            Some(owner) if owner.terminal_is_empty() => Ok((0, self.owner_bytes)),
            Some(owner) => owner.demands(),
            None => Ok((0, 0)),
        }
    }
    /// 🧮️ Borrows logical payload work separately from constructor capacity and physical release.
    pub fn next_copy_byte_demand(&self) -> usize { self.owner.as_ref().map_or(0, |owner| owner.next_copy_byte_demand()) }
    /// ♻️ Returns each owned allocation under its own capacity or release axis.
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let Some(owner) = self.owner.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        if !owner.terminal_is_empty() { return owner.close(grant); }
        let progress = RetainedCloneProgress { copied_items: 1, released_bytes: self.owner_bytes, ..Default::default() };
        if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.owner.take();
        Ok(RetainedCloneStep::Complete(progress))
    }
    pub fn terminal_is_empty(&self) -> bool { self.owner.is_none() }
}
impl Drop for MemberStoreOwnedBatch {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "erased member batch dropped before terminal owner release"); }
}

/// 🪪️ Exact member authority paired with the still-owned typed mutation batch.
pub struct MemberStoreOwnedBatchRequest {
    pub operation: semio_framework_job::OperationId,
    pub expected_generation: u64,
    pub expected_revision: [u8; 32],
    pub actor: String,
    pub group_id: Option<String>,
    pub transaction: Option<crate::os_spr::TransactionRef>,
    pub mutations: MemberStoreOwnedBatch,
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

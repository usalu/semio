//! 📦️ Exact typed mutation batches retained across erased member admission and bounded close.

use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::{RetireOwned, controlled::ControlledRetirement}};
use std::{any::Any, mem::{ManuallyDrop, size_of}};

pub(crate) trait BatchRetirement: Send {
    fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
    fn demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;
}
impl<T: RetireOwned> BatchRetirement for ControlledRetirement<T> {
    fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.step(grant) }
    fn terminal_is_empty(&self) -> bool { self.terminal_is_empty() }
    fn demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Ok(semio_framework_value::RetirementDemand { copy_bytes: self.next_copy_byte_demand()?, capacity_bytes: self.next_capacity_byte_demand(maximum_body_bytes)?, release_bytes: self.next_release_byte_demand()?, depth: self.next_depth_demand()? }) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_copy_byte_demand(self) }
}
type BatchAdmission<T> = Result<(Box<dyn BatchRetirement>, RetainedCloneProgress), (ValueError, T)>;
fn admit_batch_retirement<T: RetireOwned>(values: T, grant: RetainedCloneGrant) -> BatchAdmission<T> {
    semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(values, grant).map(|(owner, progress)| (owner as Box<dyn BatchRetirement>, progress))
}
pub(crate) struct SourceRetirementIssuer<M: Send + 'static> {
    pub(crate) birth_bytes: usize,
    pub(crate) begin: fn(std::collections::VecDeque<M>, RetainedCloneGrant) -> BatchAdmission<std::collections::VecDeque<M>>,
}
trait BatchOwner: Send {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn close(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
    fn demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;
}
struct TypedBatch<M: Send + 'static> {
    values: ManuallyDrop<Option<Vec<M>>>,
    retirement: ManuallyDrop<Option<Box<dyn BatchRetirement>>>,
    birth_bytes: usize,
    retire: fn(Vec<M>, RetainedCloneGrant) -> BatchAdmission<Vec<M>>,
    retire_source: fn(std::collections::VecDeque<M>, RetainedCloneGrant) -> BatchAdmission<std::collections::VecDeque<M>>,
    source_birth_bytes: usize,
    closing: bool,
}
impl<M: Send + 'static> BatchOwner for TypedBatch<M> {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn close(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth < self.demands(grant.maximum_copy_bytes)?.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "owned batch retirement exceeds admitted depth")); }
        self.closing = true;
        if self.values.is_some() {
            if self.birth_bytes > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            return match (self.retire)(self.values.take().unwrap(), child) {
                Ok((owner, progress)) => { *self.retirement = Some(owner); Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original)) => { *self.values = Some(original); Err(error) },
            };
        }
        if let Some(retirement) = self.retirement.as_mut() {
            if !retirement.terminal_is_empty() {
                let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
                let step = retirement.step(child)?;
                return semio_framework_value::retained_clone::admit_retained_clone_close(child, step, retirement.terminal_is_empty(), "owned batch child").map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            let bytes = std::mem::size_of_val(retirement.as_ref());
            if bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.retirement.take();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    fn terminal_is_empty(&self) -> bool { self.values.is_none() && self.retirement.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.retirement.as_ref().map_or(Ok(0), |retirement| retirement.next_copy_byte_demand()) }
    fn demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if self.values.is_some() { return Ok(RetirementDemand { capacity_bytes: self.birth_bytes, depth: 2, ..Default::default() }); }
        match self.retirement.as_ref() {
            Some(retirement) if retirement.terminal_is_empty() => Ok(RetirementDemand { release_bytes: std::mem::size_of_val(retirement.as_ref()), depth: 1, ..Default::default() }),
            Some(retirement) => { let mut demand = retirement.demands(maximum_body_bytes)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "owned batch depth overflow"))?; Ok(demand) },
            None => Ok(RetirementDemand::default()),
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
        if grant.maximum_items == 0 { return Err((ValueError::literal(ValueRefusalKind::WorkLimit, "member batch constructor requires an admitted item"), values)); }
        if grant.maximum_depth == 0 { return Err((ValueError::literal(ValueRefusalKind::DepthLimit, "member batch constructor requires admitted depth"), values)); }
        let owner_bytes = size_of::<TypedBatch<M>>();
        let progress = RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: owner_bytes, ..Default::default() };
        if !progress.fits(grant) { return Err((ValueError::new(ValueRefusalKind::AllocationFailed, "member batch scaffold exceeds its admitted allocation grant"), values)); }
        let owner = TypedBatch { values: ManuallyDrop::new(Some(values)), retirement: ManuallyDrop::new(None), birth_bytes: size_of::<ControlledRetirement<Vec<M>>>(), retire: admit_batch_retirement::<Vec<M>>, retire_source: admit_batch_retirement::<std::collections::VecDeque<M>>, source_birth_bytes: size_of::<ControlledRetirement<std::collections::VecDeque<M>>>(), closing: false };
        Ok((Self { owner: Some(Box::new(owner)), owner_bytes }, progress))
    }
    /// 🔎️ Borrows the original typed source without cloning or removing any owner.
    pub fn mutations<M: Send + 'static>(&self) -> Option<&[M]> { self.owner.as_ref()?.as_any().downcast_ref::<TypedBatch<M>>()?.values.as_deref() }
    pub(crate) fn source_retirement_issuer<M: Send + 'static>(&self) -> Option<SourceRetirementIssuer<M>> {
        let owner = self.owner.as_ref()?.as_any().downcast_ref::<TypedBatch<M>>()?;
        Some(SourceRetirementIssuer { birth_bytes: owner.source_birth_bytes, begin: owner.retire_source })
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
    pub fn next_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        match self.owner.as_ref() {
            Some(owner) if owner.terminal_is_empty() => Ok(semio_framework_value::RetirementDemand { release_bytes: self.owner_bytes, depth: 1, ..Default::default() }),
            Some(owner) => owner.demands(maximum_body_bytes),
            None => Ok(Default::default()),
        }
    }
    /// 🧮️ Borrows logical payload work separately from constructor capacity and physical release.
    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.owner.as_ref().map_or(Ok(0), |owner| owner.next_copy_byte_demand()) }
    /// ♻️ Returns each owned allocation under its own capacity or release axis.
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let Some(owner) = self.owner.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "member batch shell requires admitted depth")); }
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
    pub actor: semio_framework_value::SharedUtf8,
    pub group_id: Option<String>,
    pub transaction: Option<crate::os_spr::TransactionRef>,
    pub mutations: MemberStoreOwnedBatch,
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

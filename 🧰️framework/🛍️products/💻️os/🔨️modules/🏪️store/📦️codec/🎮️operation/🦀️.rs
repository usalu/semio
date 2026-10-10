//! 🎟️ Original codec inputs and catalog sources remain retained across real supplied-grant turns.
use crate::os_store::{DocumentStoreOwners, DocumentStoreOwnersAdmissionError, ErasedSnapshotRetirement};
use crate::Mutation;
use semio_framework_value::{FromValue, ToValue, RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::ManuallyDrop;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactCodecCatalogStep { Blocked, Progress, Ready, Closed }

/// 🏪️ Holds original borrowed codec bytes beside their genuine typed constructor and receipt recipient.
pub struct RetainedArtifactCodecCatalog<'a, P, M>
where P: Clone + ToValue + FromValue, M: Clone + ToValue + FromValue + Mutation<P> {
    sources: [&'a [u8]; 3],
    original: ManuallyDrop<Option<DocumentStoreOwners<P, M>>>,
    failure: ManuallyDrop<Option<ValueError>>,
    failure_close: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    receipt: Option<(RetainedCloneGrant, RetainedCloneProgress)>,
    closing: bool,
    done: bool,
}

impl<'a, P, M> RetainedArtifactCodecCatalog<'a, P, M>
where P: Clone + ToValue + FromValue, M: Clone + ToValue + FromValue + Mutation<P> {
    /// 🪶️ Borrows all three original inputs without constructing any native owner or backing.
    pub fn new(pack: &'a [u8], spr: &'a [u8], commands: &'a [u8]) -> Self {
        Self { sources: [pack, spr, commands], original: ManuallyDrop::new(None), failure: ManuallyDrop::new(None), failure_close: ManuallyDrop::new(None), receipt: None, closing: false, done: false }
    }
    /// 🔭️ Exposes exactly the original borrowed inputs while constructor and cancellation turns run.
    pub fn sources(&self) -> [&'a [u8]; 3] { self.sources }
    /// 🧾️ Borrows the actual unchanged caller grant and its accepted producer receipt.
    pub fn receipt(&self) -> Option<&(RetainedCloneGrant, RetainedCloneProgress)> { self.receipt.as_ref() }
    /// 📬️ Transfers the original receipt once; further owner work waits until this recipient acts.
    pub fn take_receipt(&mut self) -> Option<(RetainedCloneGrant, RetainedCloneProgress)> { self.receipt.take() }
    /// 🏁 Requires genuine factory ticket readiness and a paid original receipt recipient.
    pub fn is_prepared(&self) -> bool { !self.closing && self.failure.is_none() && self.receipt.is_none() && self.original.as_ref().is_some_and(DocumentStoreOwners::constructor_is_complete) }
    /// 🛑️ Records cancellation intent while every original source and receipt remains retained.
    pub fn cancel(&mut self) { self.closing = true; }
    /// 🫴️ Transfers a genuinely prepared original catalog into its caller's empty typed receiving slot.
    pub fn take_prepared(&mut self, recipient: &mut Option<DocumentStoreOwners<P, M>>, grant: RetainedCloneGrant) -> Result<ArtifactCodecCatalogStep, ValueError> {
        if recipient.is_some() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "original codec catalog recipient is occupied")); }
        if !self.is_prepared() || grant.maximum_items == 0 || grant.maximum_depth < 2 { return Ok(ArtifactCodecCatalogStep::Blocked); }
        *recipient = self.original.take();
        self.done = true;
        self.record(grant, item())?;
        Ok(ArtifactCodecCatalogStep::Ready)
    }
    /// 📏️ Quotes the next original child under a separate parent depth without granting any currency.
    pub fn demands(&self, body: usize, source: impl FnOnce() -> Result<RetirementDemand, ValueError>) -> Result<RetirementDemand, ValueError> {
        if self.receipt.is_some() || self.done { return Ok(Default::default()); }
        if self.closing || self.failure.is_some() { return self.close_demands(body); }
        let demand = match self.original.as_ref() { Some(owners) if owners.constructor_is_complete() => return Ok(Default::default()), Some(owners) => owners.constructor_demands(body)?, None => source()? };
        parent(demand)
    }
    /// 🏗️ Executes one actual source birth or factory ticket turn under independently supplied authority.
    pub fn advance(&mut self, grant: RetainedCloneGrant, source: impl FnOnce() -> Result<RetirementDemand, ValueError>, build: impl FnOnce(RetainedCloneGrant) -> Result<(DocumentStoreOwners<P, M>, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, M>>) -> Result<ArtifactCodecCatalogStep, ValueError> {
        if self.receipt.is_some() || self.closing || self.done || self.failure.is_some() { return Ok(ArtifactCodecCatalogStep::Blocked); }
        if self.is_prepared() { return Ok(ArtifactCodecCatalogStep::Ready); }
        let demand = self.demands(grant.maximum_copy_bytes, source)?;
        if !covers(grant, demand) { return Ok(ArtifactCodecCatalogStep::Blocked); }
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let progress = if self.original.is_some() { self.advance_catalog(child)? } else {
            match build(child) {
                Ok((owners, progress)) => { *self.original = Some(owners); progress }
                Err(error) => { *self.original = error.owners; *self.failure = Some(error.error); self.closing = true; RetainedCloneProgress { copied_items: error.progress.copied_items.max(1), ..error.progress } }
            }
        };
        if progress == Default::default() { return Ok(ArtifactCodecCatalogStep::Blocked); }
        self.record(grant, progress)?;
        Ok(if self.original.as_ref().is_some_and(DocumentStoreOwners::constructor_is_complete) && self.failure.is_none() { ArtifactCodecCatalogStep::Ready } else { ArtifactCodecCatalogStep::Progress })
    }
    fn advance_catalog(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let owners = self.original.as_mut().unwrap();
        match owners.admit_constructor(grant) { Ok(progress) => Ok(progress), Err((error, progress)) => { *self.failure = Some(error); self.closing = true; Ok(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }) } }
    }
    /// 📏️ Borrows exact original error, ticket and catalog closure demands before any transfer.
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.receipt.is_some() || self.done { return Ok(Default::default()); }
        let demand = if let Some(owner) = self.failure_close.as_ref() { crate::os_store::artifact_retirement_box_demands(owner, body)? }
        else if self.failure.is_some() { crate::os_store::artifact_retirement_owned_birth_demands(&self.failure)? }
        else if let Some(owners) = self.original.as_ref() { if owners.uninstalled_owners_terminal_is_empty() { RetirementDemand { depth: 1, ..Default::default() } } else { owners.uninstalled_owners_demands(body)? } }
        else { RetirementDemand { depth: 1, ..Default::default() } };
        parent(demand)
    }
    /// ♻️ Closes one genuine original child while preserving its actual unchanged-grant receipt.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<ArtifactCodecCatalogStep, ValueError> {
        if self.receipt.is_some() { return Ok(ArtifactCodecCatalogStep::Blocked); }
        if self.done { return Ok(ArtifactCodecCatalogStep::Closed); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if !covers(grant, demand) { return Ok(ArtifactCodecCatalogStep::Blocked); }
        self.closing = true;
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let progress = if self.failure_close.is_some() { crate::os_store::artifact_retirement_box_close_step(&mut self.failure_close, child)?.progress() }
        else if self.failure.is_some() { crate::os_store::artifact_retirement_admit_owned(&mut self.failure, &mut self.failure_close, child)?.progress() }
        else if let Some(owners) = self.original.as_mut() { if owners.uninstalled_owners_terminal_is_empty() { drop(self.original.take()); item() } else { close_catalog(owners, child)?.progress() } }
        else { self.done = true; item() };
        if progress == Default::default() { return Ok(ArtifactCodecCatalogStep::Blocked); }
        self.record(grant, progress)?;
        Ok(ArtifactCodecCatalogStep::Progress)
    }
    fn record(&mut self, grant: RetainedCloneGrant, progress: RetainedCloneProgress) -> Result<(), ValueError> {
        self.receipt = Some((grant, progress));
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "original retained codec catalog").map(|_| ())
    }
    /// 🫙️ Includes the original receipt frontier after physical child custody becomes empty.
    pub fn terminal_is_empty(&self) -> bool { self.done && self.original.is_none() && self.failure.is_none() && self.failure_close.is_none() && self.receipt.is_none() }
}
impl<P, M> Drop for RetainedArtifactCodecCatalog<'_, P, M>
where P: Clone + ToValue + FromValue, M: Clone + ToValue + FromValue + Mutation<P> {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "original codec catalog requires controlled custody and receipt closure"); if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.original); ManuallyDrop::drop(&mut self.failure); ManuallyDrop::drop(&mut self.failure_close); } } }
}
fn close_catalog<P, M>(owners: &mut DocumentStoreOwners<P, M>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
where P: Clone + ToValue + FromValue, M: Clone + ToValue + FromValue + Mutation<P> {
    let step = owners.close_uninstalled_owners_step(grant)?;
    semio_framework_value::retained_clone::admit_retained_clone_close(grant, step, owners.uninstalled_owners_terminal_is_empty(), "original codec catalog sources")
}
fn parent(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.max(1).checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "codec catalog parent depth overflow"))?; Ok(demand) }
fn covers(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool { grant.maximum_items > 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth }
fn item() -> RetainedCloneProgress { RetainedCloneProgress { copied_items: 1, ..Default::default() } }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

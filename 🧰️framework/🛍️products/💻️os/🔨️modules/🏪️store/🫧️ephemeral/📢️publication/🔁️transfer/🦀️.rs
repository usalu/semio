//! 🔁️ Ownership-only ephemeral replacement with domain admission and bounded retirement.

use super::{
    ArtifactEphemeralOneItemPreparation, ArtifactEphemeralOneItemPreparationFactory, ArtifactEphemeralOneItemPreparationRequest, ArtifactOwnedValueRetirementFactory, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint,
    ArtifactStoreOneItemGrant,
};
use std::sync::Arc;

/// 📏️ Maximum inline owner metadata moved by one transfer turn; heap payload remains owned.
pub const ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES: usize = 256;

/// 🏭️ Shares replacement lifecycle while domains supply constant-work admission and transfer.
pub struct ArtifactEphemeralTransferPreparationFactory<P, M> {
    preflight: fn(&M) -> Result<ArtifactStoreOneItemFootprint, String>,
    transfer: fn(M) -> P,
    state_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
}

impl<P, M> ArtifactEphemeralTransferPreparationFactory<P, M> {
    /// 🧩️ Transfer must move admitted ownership without cloning, traversing or discarding payload.
    pub fn new(
        preflight: fn(&M) -> Result<ArtifactStoreOneItemFootprint, String>,
        transfer: fn(M) -> P,
        state_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
        mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
    ) -> Self {
        Self { preflight, transfer, state_retirement, mutation_retirement }
    }
}

impl<P: Send + Sync + 'static, M: Send + 'static> ArtifactEphemeralOneItemPreparationFactory<P, M> for ArtifactEphemeralTransferPreparationFactory<P, M> {
    fn preflight(&self, mutation: &M) -> Result<ArtifactStoreOneItemFootprint, String> {
        if size_of::<P>() > ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES || size_of::<M>() > ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES {
            return Err("ephemeral transfer inline ownership exceeds its fixed metadata bound".into());
        }
        let footprint = (self.preflight)(mutation)?;
        if footprint.work_items != 1 || !footprint.is_admissible() {
            return Err("ephemeral transfer requires one admitted ownership transfer".into());
        }
        Ok(footprint)
    }

    fn begin(&self, request: ArtifactEphemeralOneItemPreparationRequest<P, M>) -> Result<Box<dyn ArtifactEphemeralOneItemPreparation<P, M>>, ArtifactEphemeralOneItemPreparationRequest<P, M>> {
        if self.preflight(&request.mutation).is_err() {
            return Err(request);
        }
        Ok(Box::new(super::ephemeral_preparation::ArtifactEphemeralTaskPreparation::new(request, Box::new(TransferTask { transfer: self.transfer }), self.state_retirement.clone(), self.mutation_retirement.clone())))
    }
}

struct TransferTask<P, M> {
    transfer: fn(M) -> P,
}

impl<P, M> super::ArtifactEphemeralPreparationTask<P, M> for TransferTask<P, M> {
    fn advance(&mut self, _: &P, mutation: &mut Option<M>, grant: ArtifactStoreOneItemGrant) -> Result<super::ArtifactEphemeralPreparationTaskStep<P>, String> {
        if grant.maximum_items == 0 {
            return Ok(super::ArtifactEphemeralPreparationTaskStep::Blocked);
        }
        let Some(mutation) = mutation.take() else { return Ok(super::ArtifactEphemeralPreparationTaskStep::Blocked) };
        let root = (self.transfer)(mutation);
        Ok(super::ArtifactEphemeralPreparationTaskStep::Prepared { root, checkpoint: ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, ..Default::default() } })
    }
    fn begin_close(&mut self) {}
 }
impl<P, M> crate::os_store::ErasedSnapshotRetirement for TransferTask<P, M> {
    fn close_step(&mut self, _: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())) }
    fn terminal_is_empty(&self) -> bool { true }
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }

}

#[cfg(test)]
use super::{ArtifactEphemeralBaseRead, ArtifactStoreOneItemPreparationStep, ErasedSnapshotRetirement, ReturnedSnapshotReadRetirement};

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

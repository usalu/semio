//! 🔁️ Ownership-only ephemeral replacement with domain admission and bounded retirement.

use super::{
    ArtifactEphemeralOneItemPreparation, ArtifactEphemeralOneItemPreparationFactory, ArtifactEphemeralOneItemPreparationRequest, ArtifactOwnedValueRetirementFactory, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint,
    ArtifactStoreOneItemGrant,
};
use std::sync::Arc;

/// 📏️ Maximum inline owner metadata moved by one transfer turn; heap payload remains owned.
pub const ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES: usize = 256;

/// 🏭️ Shares replacement lifecycle while domains supply constant-work admission and transfer.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ArtifactEphemeralTransferPreparationFactory<P: Send + Sync + 'static, M: Send + 'static> {
    preflight: fn(&M) -> Result<ArtifactStoreOneItemFootprint, String>,
    transfer: fn(M) -> P,
    #[factory_child]
    state_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    #[factory_child]
    mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
}

impl<P: Send + Sync + 'static, M: Send + 'static> ArtifactEphemeralTransferPreparationFactory<P, M> {
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

pub(super) struct TransferTask<P, M> {
    pub(super) transfer: fn(M) -> P,
}

impl<P, M> super::ArtifactEphemeralPreparationTask<P, M> for TransferTask<P, M> {
    fn advance(&mut self, _: &P, mutation: &mut Option<M>, grant: ArtifactStoreOneItemGrant) -> Result<super::ArtifactEphemeralPreparationTaskStep<P>, semio_framework_value::ValueError> {
        let capacity=semio_framework_value::shared_retirement_allocation_bytes::<P>();
        let copy=size_of::<P>()+size_of::<M>();
        if !grant.permits_one() || grant.maximum_capacity_bytes < capacity || grant.maximum_copy_bytes < copy {
            return Ok(super::ArtifactEphemeralPreparationTaskStep::Blocked);
        }
        let Some(mutation) = mutation.take() else { return Ok(super::ArtifactEphemeralPreparationTaskStep::Blocked) };
        let root = Arc::new((self.transfer)(mutation));
        let ownership=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes: capacity,released_bytes:0};
        Ok(super::ArtifactEphemeralPreparationTaskStep::Prepared { root, checkpoint: ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1,completed_bytes:copy as u64, ..Default::default() },ownership })
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

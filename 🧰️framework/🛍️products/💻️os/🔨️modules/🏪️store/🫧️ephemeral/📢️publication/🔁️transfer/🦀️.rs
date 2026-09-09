//! 🔁️ Ownership-only ephemeral replacement with domain admission and bounded retirement.

use super::{
    ArtifactEphemeralOneItemPreparation, ArtifactEphemeralOneItemPreparationFactory,
    ArtifactEphemeralOneItemPreparationRequest, ArtifactOwnedValueRetirementFactory, ArtifactStoreOneItemCheckpoint,
    ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant,
    SnapshotRetirementStep,
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
        Ok(Box::new(super::ephemeral_preparation::ArtifactEphemeralTaskPreparation::new(
            request, Box::new(TransferTask { transfer: self.transfer }), self.state_retirement.clone(), self.mutation_retirement.clone(),
        )))
    }
}

struct TransferTask<P, M> {
    transfer: fn(M) -> P,
}

impl<P, M> super::ArtifactEphemeralPreparationTask<P, M> for TransferTask<P, M> {
    fn advance(&mut self, _: &P, mutation: &mut Option<M>, grant: ArtifactStoreOneItemGrant) -> Result<super::ArtifactEphemeralPreparationTaskStep<P>, String> {
        if grant.maximum_items == 0 { return Ok(super::ArtifactEphemeralPreparationTaskStep::Blocked); }
        let Some(mutation) = mutation.take() else { return Ok(super::ArtifactEphemeralPreparationTaskStep::Blocked) };
        let root = (self.transfer)(mutation);
        Ok(super::ArtifactEphemeralPreparationTaskStep::Prepared { root, checkpoint: ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, ..Default::default() } })
    }
    fn begin_close(&mut self) {}
    fn close_step(&mut self, _: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> { Ok(SnapshotRetirementStep::Complete) }
    fn terminal_is_empty(&self) -> bool { true }
}

#[cfg(test)]
use super::{ArtifactEphemeralBaseRead, ArtifactStoreOneItemPreparationStep, ErasedSnapshotRetirement, ReturnedSnapshotReadRetirement};

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

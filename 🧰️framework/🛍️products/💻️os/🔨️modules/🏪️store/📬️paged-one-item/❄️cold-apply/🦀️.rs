//! ❄️ Cold one-item preparation: a family whose mutations carry only the explicit cold reference semantics (`ColdMutation`) decides, inverts and applies them on an owned scratch projection, one funded turn per step.
//!
//! Retained families use `mutation_apply_preparation_factory`; this is the cold sibling, never a default of it.
use crate::OneItemOwners;
use semio_framework_os_spr::{apply_diff_cold, ColdMutation, Edit, Mutation, MutationDiff};
use semio_framework_os_store_command::HistoryLane;
use semio_framework_os_store_one_item::{ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep, ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES};
use semio_framework_os_store_one_item_publication::batch_digest::{admit_artifact_batch_digest, ArtifactStoreBatchDigest};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemPrepared, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest};
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::{marker::PhantomData, mem::size_of, sync::Arc};

/// 🏭️ The cold preparation factory of one snapshot and mutation family.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ColdMutationApplyFactory<S, M>(PhantomData<fn() -> (S, M)>);

impl<S, M> Default for ColdMutationApplyFactory<S, M> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// 🏭️ Every cold mutation family answers the central document lane through its explicit cold reference semantics.
pub fn cold_mutation_apply_preparation_factory<S, M>() -> Arc<dyn ArtifactStoreOneItemPreparationFactory<S, M>>
where
    S: Clone + RetireOwned + Send + Sync + 'static,
    M: Mutation<S> + ColdMutation<S> + ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
{
    Arc::new(ColdMutationApplyFactory::<S, M>::default())
}

struct ColdMutationApplyPreparation<S: 'static, M: 'static> {
    owners: OneItemOwners<S, M>,
    checkpoint: ArtifactStoreOneItemCheckpoint,
    completed_bytes: usize,
    phase: u8,
    cancelled: bool,
}

fn fault(reason: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, reason)
}

impl<S, M> ArtifactStoreOneItemPreparationFactory<S, M> for ColdMutationApplyFactory<S, M>
where
    S: Clone + RetireOwned + Send + Sync + 'static,
    M: Mutation<S> + ColdMutation<S> + ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
{
    fn begin_batch_digest(&self, edit: &mut Option<Box<Edit<M>>>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ArtifactStoreBatchDigest<M>>, RetainedCloneProgress)>, ValueError> {
        admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &M, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        if lane != HistoryLane::Document {
            return Err("cold mutation preparation rejected its lane".into());
        }
        Ok(ArtifactStoreOneItemFootprint::for_leaf::<S, _>(mutation, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &M, _lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> {
        Ok(RetainedCloneBirthDemand { capacity_bytes: size_of::<ColdMutationApplyPreparation<S, M>>(), depth: 1 })
    }

    fn begin_copy_byte_demand(&self, mutation: &M, lane: HistoryLane) -> Result<usize, ValueError> {
        self.begin_demand(mutation, lane)?;
        size_of::<ArtifactStoreOneItemPreparationRequest<S, M, M>>().checked_add(size_of::<ColdMutationApplyPreparation<S, M>>()).ok_or_else(|| fault("cold mutation preparation copy quote overflow"))
    }

    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<S, M, M>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<S, M>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<S, M, M>)> {
        if request.lane != HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((fault("cold mutation preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(ColdMutationApplyPreparation { owners: OneItemOwners::from_request(request), checkpoint: ArtifactStoreOneItemCheckpoint::default(), completed_bytes: 0, phase: 0, cancelled: false }), progress))
    }
}

impl<S, M> ArtifactStoreOneItemPreparation<S, M> for ColdMutationApplyPreparation<S, M>
where
    S: Clone + RetireOwned + Send + Sync + 'static,
    M: Mutation<S> + ColdMutation<S> + ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
{
    fn advance(&mut self, _body: usize, grant: ArtifactStoreOneItemGrant, _receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "cold mutation preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() || self.phase >= 2 {
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        match self.phase {
            0 => {
                let base = self.owners.base.as_ref().ok_or_else(|| fault("cold mutation preparation lost its exact base root"))?;
                let mutation = self.owners.mutation.as_ref().ok_or_else(|| fault("cold mutation preparation lost its mutation owner"))?;
                let inverse = ColdMutation::inverse(mutation, base.get())?;
                let outcome = ColdMutation::diff(mutation, base.get());
                let post = apply_diff_cold(outcome.diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "cold mutation could not produce its post root"))?;
                let mutation = self.owners.mutation.take().expect("observed original mutation owner");
                *self.owners.candidate = Some((post, inverse, mutation));
                self.completed_bytes = 1;
                self.phase = 1;
                self.checkpoint = ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.completed_bytes as u64, digest: [0; 32] };
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress::default()))
            }
            1 => {
                let Some(authority) = self.owners.authority.as_ref() else {
                    return Err(fault("cold mutation preparation lost its Store authority"));
                };
                let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| fault("cold mutation preparation lost its semantic candidate"))?;
                let prepared = match authority.prepare_one_item(authority.next_edit(mutation, inverse), Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => {
                        *self.owners.refused = Some((edit, post));
                        return Err(error);
                    }
                };
                self.phase = 2;
                self.checkpoint = ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.completed_bytes as u64, digest: prepared.edit_digest() };
                *self.owners.prepared = Some(prepared);
                Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
            }
            _ => Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default())),
        }
    }

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<S, M>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<S, M>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> {
        self.owners.close_step(grant.retained_grant())
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}

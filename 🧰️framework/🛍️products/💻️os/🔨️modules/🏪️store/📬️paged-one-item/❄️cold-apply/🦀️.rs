//! ❄️ Cold one-item preparation: a family whose mutations carry only the explicit cold reference semantics (`ColdMutation`) decides, inverts and applies them on an owned scratch projection, one funded turn per step.
//!
//! Retained families use `mutation_apply_preparation_factory`; this is the cold sibling, never a default of it.
use crate::OneItemOwners;
use semio_framework_os_spr::{apply_diff_cold, apply_diff_cold_into, ColdMutation, Edit, Mutation, MutationDiff};
use semio_framework_os_store_replay_operation::native::{NativeSemanticPreparation, SemanticEdit, SemanticEditStep, SemanticProjection};
use semio_framework_os_store_replay_operation::{ArtifactReplayPreparation, ArtifactReplayPreparationFactory, ArtifactReplayPreparationRequest};
use semio_framework_value_snapshot_projection::snapshot_projection::post::{PagedOneItemPost, PagedOneItemPostMode, PagedOneItemPostRead};
use semio_framework_value::retained_clone::{retained_clone_grant_admits, RetainedCloneClose, RetainedCloneRef};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, RetirementDemand};
use std::mem::ManuallyDrop;
use semio_framework_os_store_command::HistoryLane;
use semio_framework_os_store_one_item::{ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep, ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES};
use semio_framework_os_store_one_item_publication::batch_digest::{admit_artifact_batch_digest, ArtifactStoreBatchDigest};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemPrepared, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest};
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::{marker::PhantomData, mem::size_of, sync::Arc};

/// 📏️ The fixed Store envelope of one cold family: the bytes a mutation and the base snapshot may carry, priced by the domain's own measures.
pub struct ColdPreparationBounds<S, M> {
    pub maximum_bytes: usize,
    pub mutation_bytes: fn(&M) -> usize,
    pub snapshot_bytes: fn(&S) -> usize,
}

impl<S, M> Clone for ColdPreparationBounds<S, M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, M> Copy for ColdPreparationBounds<S, M> {}

/// 🏭️ The cold preparation factory of one snapshot and mutation family; with bounds it refuses oversize mutations and bases at preflight and meters the base clone.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ColdMutationApplyFactory<S, M> {
    marker: PhantomData<fn() -> (S, M)>,
    bounds: Option<ColdPreparationBounds<S, M>>,
}

impl<S, M> Default for ColdMutationApplyFactory<S, M> {
    fn default() -> Self {
        Self { marker: PhantomData, bounds: None }
    }
}

impl<S, M> ColdMutationApplyFactory<S, M> {
    pub fn bounded(bounds: ColdPreparationBounds<S, M>) -> Self {
        Self { marker: PhantomData, bounds: Some(bounds) }
    }
}

/// 🧊️ Every cold mutation family answers the central document lane through its explicit cold reference semantics.
pub fn cold_mutation_apply_preparation_factory<S, M>() -> Arc<dyn ArtifactStoreOneItemPreparationFactory<S, M>>
where
    S: Clone + RetireOwned + Send + Sync + 'static,
    M: Mutation<S> + ColdMutation<S> + ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
{
    Arc::new(ColdMutationApplyFactory::<S, M>::default())
}

/// 📄️ Every cold family answers the central document lane within the fixed Store envelope its domain declares.
pub fn bounded_cold_mutation_apply_preparation_factory<S, M>(bounds: ColdPreparationBounds<S, M>) -> Arc<dyn ArtifactStoreOneItemPreparationFactory<S, M>>
where
    S: Clone + RetireOwned + Send + Sync + 'static,
    M: Mutation<S> + ColdMutation<S> + ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
{
    Arc::new(ColdMutationApplyFactory::<S, M>::bounded(bounds))
}

struct ColdMutationApplyPreparation<S: 'static, M: 'static> {
    clone_bytes: Option<usize>,
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
        let Some(bounds) = self.bounds else {
            return Ok(ArtifactStoreOneItemFootprint::for_leaf::<S, _>(mutation, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES));
        };
        let bytes = (bounds.mutation_bytes)(mutation);
        if bytes > bounds.maximum_bytes {
            return Err("cold mutation exceeds its fixed Store envelope".into());
        }
        Ok(ArtifactStoreOneItemFootprint::for_leaf::<S, _>(mutation, bytes))
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
        let clone_bytes = self.bounds.map(|bounds| (bounds.snapshot_bytes)(request.base.get()));
        if self.bounds.zip(clone_bytes).is_some_and(|(bounds, bytes)| bytes > bounds.maximum_bytes) {
            return Err((fault("cold mutation preparation base exceeds its fixed Store envelope"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(ColdMutationApplyPreparation { clone_bytes, owners: OneItemOwners::from_request(request), checkpoint: ArtifactStoreOneItemCheckpoint::default(), completed_bytes: 0, phase: 0, cancelled: false }), progress))
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
                let clone_bytes = self.clone_bytes.map_or(0, |bytes| bytes.max(size_of::<S>()));
                if grant.maximum_copy_bytes < clone_bytes {
                    return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
                }
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
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress { copied_items: usize::from(clone_bytes > 0), copied_bytes: clone_bytes, ..Default::default() }))
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

fn replay_header<T>() -> RetirementDemand {
    RetirementDemand { copy_bytes: size_of::<T>(), depth: 1, ..Default::default() }
}

/// 📽️ Clones the immutable base into the owned post projection the cold edit then advances in place.
pub struct ColdReplayProjection<S: 'static> {
    snapshot: Option<(fn(&S) -> usize, usize)>,
    clone: ManuallyDrop<Option<S>>,
    retirement: RetainedCloneClose,
    cloned: bool,
    closing: bool,
}

impl<S: 'static> Default for ColdReplayProjection<S> {
    fn default() -> Self {
        Self { snapshot: None, clone: ManuallyDrop::new(None), retirement: RetainedCloneClose::default(), cloned: false, closing: false }
    }
}

impl<S: 'static> ColdReplayProjection<S> {
    pub fn bounded(snapshot_bytes: fn(&S) -> usize, maximum_bytes: usize) -> Self {
        Self { snapshot: Some((snapshot_bytes, maximum_bytes)), ..Self::default() }
    }

    fn clone_demand(&self, source: &S) -> Result<RetirementDemand, ValueError> {
        let Some((snapshot_bytes, maximum_bytes)) = self.snapshot else {
            return Ok(replay_header::<S>());
        };
        let bytes = snapshot_bytes(source);
        if bytes > maximum_bytes {
            return Err(fault("cold replay base exceeds its fixed Store envelope"));
        }
        Ok(RetirementDemand { copy_bytes: bytes.max(size_of::<S>()), depth: 1, ..Default::default() })
    }
}

impl<S: Clone + RetireOwned + Send + Sync + 'static> SemanticProjection<S> for ColdReplayProjection<S> {
    fn birth_demand(&self) -> RetirementDemand {
        replay_header::<Self>()
    }

    fn begin(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let demand = self.birth_demand();
        if !retained_clone_grant_admits(grant, demand) {
            return Err(fault("cold replay projection birth not funded"));
        }
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
    }

    fn demands(&self, source: RetainedCloneRef<'_, S>, _body: usize) -> Result<RetirementDemand, ValueError> {
        self.clone_demand(source.get())
    }

    fn advance(&mut self, source: RetainedCloneRef<'_, S>, _body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.clone_demand(source.get())?;
        if self.closing || !retained_clone_grant_admits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        *self.clone = Some(source.get().clone());
        self.cloned = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    fn take_demand(&self) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { copy_bytes: size_of::<S>() * 2, depth: 1, ..Default::default() })
    }

    fn take(&mut self, grant: RetainedCloneGrant) -> Result<Option<(S, RetainedCloneProgress)>, ValueError> {
        let demand = self.take_demand()?;
        if !retained_clone_grant_admits(grant, demand) {
            return Ok(None);
        }
        Ok(self.clone.take().map(|post| (post, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.retirement.is_empty() || self.clone.is_some() {
            return self.retirement.next_owner_demands::<S>(self.clone.is_some(), &None, body);
        }
        Ok(replay_header::<Self>())
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        if !self.retirement.is_empty() {
            return self.retirement.step_granted(grant).result.map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(step) = self.retirement.begin_granted(&mut self.clone, grant).result? {
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        self.cloned = false;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.clone.is_none() && self.retirement.is_empty() && !self.cloned
    }
}

/// ✏️ Decides, inverts and applies one cold mutation in place on the owned post projection in one funded turn.
pub struct ColdReplayEdit<S: 'static, M: 'static> {
    applied: bool,
    failed: bool,
    closing: bool,
    closed: bool,
    marker: PhantomData<fn() -> (S, M)>,
}

impl<S: 'static, M: 'static> Default for ColdReplayEdit<S, M> {
    fn default() -> Self {
        Self { applied: false, failed: false, closing: false, closed: false, marker: PhantomData }
    }
}

impl<S: Clone + RetireOwned + Send + Sync + 'static, M: Mutation<S> + ColdMutation<S> + RetireOwned + Send + Sync + 'static> SemanticEdit<S, M> for ColdReplayEdit<S, M> {
    fn birth_demand(&self) -> RetirementDemand {
        replay_header::<Self>()
    }

    fn begin(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let demand = self.birth_demand();
        if !retained_clone_grant_admits(grant, demand) {
            return Err(fault("cold replay edit birth not funded"));
        }
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
    }

    fn post_mode(&self) -> PagedOneItemPostMode {
        PagedOneItemPostMode::Mutable
    }

    fn demands(&self, _before: RetainedCloneRef<'_, S>, _input: RetainedCloneRef<'_, M>, _post: PagedOneItemPostRead<'_, S>, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { copy_bytes: size_of::<S>(), capacity_bytes: size_of::<M>(), depth: 1, ..Default::default() })
    }

    fn advance(&mut self, before: RetainedCloneRef<'_, S>, input: RetainedCloneRef<'_, M>, post: PagedOneItemPost<'_, S>, _body: usize, grant: RetainedCloneGrant, receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<SemanticEditStep<M>, ValueError> {
        let post = post.mutable()?;
        let demand = RetirementDemand { copy_bytes: size_of::<S>(), capacity_bytes: size_of::<M>(), depth: 1, ..Default::default() };
        if self.failed {
            return Err(fault("cold replay edit refused after a failed in-place application"));
        }
        if self.closing || self.applied || !retained_clone_grant_admits(grant, demand) {
            return Ok(SemanticEditStep::Progress(RetainedCloneProgress::default()));
        }
        receiving.admit_turn_capacity(grant.maximum_capacity_bytes)?;
        receiving.charge(demand.capacity_bytes)?;
        let mutation = input.get();
        let outcome = ColdMutation::diff(mutation, before.get());
        self.failed = true;
        apply_diff_cold_into(outcome.diff(), post).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "cold replay mutation could not produce its post root"))?;
        let inverse = ColdMutation::inverse(mutation, before.get())?;
        self.failed = false;
        self.applied = true;
        Ok(SemanticEditStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() }, inverse, Vec::new(), false))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_demand(&self, _factory: &dyn ArtifactOwnedValueRetirementFactory<M>, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(replay_header::<Self>())
    }

    fn close_step(&mut self, _factory: &dyn ArtifactOwnedValueRetirementFactory<M>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let demand = replay_header::<Self>();
        if !self.closing || !retained_clone_grant_admits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.closed = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.closed
    }
}

/// 🪢 The cold sibling of native replay: any family with explicit cold reference semantics replays its history through the shared native producer.
impl<S, M> ArtifactReplayPreparationFactory<S, M> for ColdMutationApplyFactory<S, M>
where
    S: Clone + RetireOwned + Send + Sync + 'static,
    M: Mutation<S> + ColdMutation<S> + ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
{
    fn preparation_birth_demand(&self) -> RetirementDemand {
        type Producer<S, M> = NativeSemanticPreparation<S, M, ColdReplayProjection<S>, ColdReplayEdit<S, M>>;
        RetirementDemand { copy_bytes: size_of::<ArtifactReplayPreparationRequest<S, M>>() + size_of::<Producer<S, M>>(), capacity_bytes: size_of::<Producer<S, M>>(), depth: 1, ..Default::default() }
    }

    fn begin(&self, request: ArtifactReplayPreparationRequest<S, M>, grant: RetainedCloneGrant) -> Result<(Box<dyn ArtifactReplayPreparation<S, M>>, RetainedCloneProgress), (ValueError, ArtifactReplayPreparationRequest<S, M>)> {
        let demand = ArtifactReplayPreparationFactory::preparation_birth_demand(self);
        if !retained_clone_grant_admits(grant, demand) {
            return Err((ValueError::literal(ValueRefusalKind::WorkLimit, "cold replay producer birth not funded"), request));
        }
        let projection = self.bounds.map_or_else(ColdReplayProjection::<S>::default, |bounds| ColdReplayProjection::<S>::bounded(bounds.snapshot_bytes, bounds.maximum_bytes));
        let producer = NativeSemanticPreparation::new(request, projection, ColdReplayEdit::<S, M>::default());
        Ok((Box::new(producer), RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() }))
    }
}

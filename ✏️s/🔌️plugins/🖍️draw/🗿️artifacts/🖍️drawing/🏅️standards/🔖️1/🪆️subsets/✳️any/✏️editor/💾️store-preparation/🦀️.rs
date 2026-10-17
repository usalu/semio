//! 💾️ Drawing store preparation: the one item publication preparation factory and its preparation state.

use super::*;

//#region 📬️StorePreparation
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct DrawingArtifactStorePreparationFactory;

struct DrawingArtifactStorePreparation {
    pub(super) base: std::mem::ManuallyDrop<Option<store::os_store::snapshot_read::SnapshotRead<DrawingSnapshot>>>,
    pub(super) mutation: std::mem::ManuallyDrop<Option<DrawingMutation>>,
    pub(super) inverse: std::mem::ManuallyDrop<Option<Vec<DrawingMutation>>>,
    pub(super) refused: std::mem::ManuallyDrop<Option<(::protocol::Edit<DrawingMutation>, std::sync::Arc<DrawingSnapshot>)>>,
    apply_refusal: std::mem::ManuallyDrop<Option<::protocol::MutationApplyError>>,
    authority: std::mem::ManuallyDrop<Option<std::sync::Arc<store::os_store::one_item_publication::ArtifactStoreOneItemLiveAuthority>>>,
    pub(super) prepared: std::mem::ManuallyDrop<Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<DrawingSnapshot, DrawingMutation>>>,
    mutation_retirement: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<DrawingMutation>>>>,
    snapshot_retirement: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<DrawingSnapshot>>>>,
    pub(super) active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    factories: std::mem::ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    pub(super) cancelled: bool,
    pub(super) closing: bool,
}

impl DrawingArtifactStorePreparation {
    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "preparation close depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active.as_ref() { return nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?); }
        if self.prepared.is_some() { let birth = store::os_store::one_item_publication::ArtifactStoreOneItemPrepared::<DrawingSnapshot, DrawingMutation>::retirement_birth_demand(); return Ok(semio_framework_value::RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth + 1, ..Default::default() }); }
        if self.mutation.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.mutation)?); }
        if self.inverse.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.inverse)?); }
        if self.refused.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.refused)?); }
        if self.apply_refusal.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.apply_refusal)?); }
        if self.base.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.base)?); }
        if let Some(authority) = self.authority.as_ref() { let birth = authority.retirement_birth_demand(); return Ok(semio_framework_value::RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth + 1, ..Default::default() }); }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<std::sync::Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        self.factories.iter().find_map(Option::as_ref).map_or(Ok(Default::default()), |factory| nested(factory.demands(body)?))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<DrawingSnapshot, DrawingMutation> for DrawingArtifactStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<::protocol::Edit<DrawingMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<DrawingMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &DrawingMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("drawing-artifact-lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &DrawingMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<DrawingArtifactStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<DrawingSnapshot, DrawingMutation, DrawingMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<DrawingSnapshot, DrawingMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<DrawingSnapshot, DrawingMutation, DrawingMutation>)> {
        let items = request.base.get().layers.len().saturating_add(request.base.get().assets.len());
        if request.lane != store::os_store::store_schema::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || items > DRAWING_BOUNDED_WORK_ITEMS
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "drawing preparation rejected original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        Ok((Box::new(DrawingArtifactStorePreparation {
            base: std::mem::ManuallyDrop::new(Some(request.base)),
            mutation: std::mem::ManuallyDrop::new(Some(request.mutation)),
            inverse: std::mem::ManuallyDrop::new(None),
            refused: std::mem::ManuallyDrop::new(None),
            apply_refusal: std::mem::ManuallyDrop::new(None),
            authority: std::mem::ManuallyDrop::new(Some(request.authority)),
            prepared: std::mem::ManuallyDrop::new(None),
            mutation_retirement: std::mem::ManuallyDrop::new(Some(request.mutation_retirement)),
            snapshot_retirement: std::mem::ManuallyDrop::new(Some(request.snapshot_retirement)),
            active: std::mem::ManuallyDrop::new(None),
            factories: std::mem::ManuallyDrop::new(Default::default()),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<DrawingSnapshot, DrawingMutation> for DrawingArtifactStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use ::protocol::Mutation as _;
        use semio_framework_value::{ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.refused.is_some() || self.apply_refusal.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        let progress = semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, progress));
        }
        let base = self.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "drawing-artifact-base-owner-missing"))?;
        let authority = self.authority.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "drawing-artifact-authority-missing"))?;
        let mutation = self.mutation.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "drawing-artifact-mutation-owner-missing"))?;
        let inverse = match mutation.inverse(base.get()) {
            Ok(inverse) => inverse,
            Err(error) => { *self.mutation = Some(mutation); return Err(error); }
        };
        let post = match ::protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()) {
            Ok(post) => post,
            Err(error) => {
                *self.mutation = Some(mutation);
                *self.inverse = Some(inverse);
                *self.apply_refusal = Some(error);
                return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "preparation retained the original mutation application refusal"));
            }
        };
        let edit = authority.next_edit(mutation, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => { *self.refused = Some((edit, post)); return Err(error); }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        *self.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, progress))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<DrawingSnapshot, DrawingMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<DrawingSnapshot, DrawingMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
        let empty = RetainedCloneProgress::default();
        let grant = grant.retained_grant();
        if !self.closing || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "preparation close exceeds original depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() { return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.prepared.is_some() {
            if self.mutation_retirement.is_none() || self.snapshot_retirement.is_none() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation retains its original installed issuers")); }
            let original = self.prepared.take().expect("observed original prepared candidate");
            let mutations = self.mutation_retirement.take().expect("original mutation issuer");
            let snapshots = self.snapshot_retirement.take().expect("original snapshot issuer");
            return match original.admit_retirement(mutations, snapshots, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "original prepared close birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation child changed its actual admitted birth")); } Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original, mutations, snapshots)) => { *self.prepared = Some(original); *self.mutation_retirement = Some(mutations); *self.snapshot_retirement = Some(snapshots); Err(error) },
            };
        }
        if self.mutation.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.mutation, &mut self.active, child); }
        if self.inverse.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.inverse, &mut self.active, child); }
        if self.refused.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.refused, &mut self.active, child); }
        if self.apply_refusal.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.apply_refusal, &mut self.active, child); }
        if self.base.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.base, &mut self.active, child); }
        if let Some(authority) = self.authority.take() { return match authority.retire(child) { Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "original preparation authority close birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation authority changed admitted birth")); } Ok(RetainedCloneStep::Progress(progress)) }, Err((error, original)) => { *self.authority = Some(original); Err(error) } }; }
        if let Some(factory) = self.mutation_retirement.take() { let factory: std::sync::Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factories[0] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(factory) = self.snapshot_retirement.take() { let factory: std::sync::Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factories[1] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(slot) = self.factories.iter_mut().find(|slot| slot.is_some()) { let factory = slot.as_mut().expect("original preparation factory alias"); let step = factory.step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original preparation factory close")?; if factory.terminal_is_empty() { *slot = None; } return Ok(RetainedCloneStep::Progress(step.progress())); }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }

    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.active.is_none() && self.factories.iter().all(Option::is_none) && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.inverse.is_none() && self.refused.is_none() && self.apply_refusal.is_none() && self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

impl Drop for DrawingArtifactStorePreparation {
    fn drop(&mut self) { assert!(std::thread::panicking() || (self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none() && self.inverse.is_none() && self.refused.is_none() && self.apply_refusal.is_none() && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.active.is_none() && self.factories.iter().all(Option::is_none)), "preparation must retain original owners until supplied-grant terminal closure"); }
}
//#endregion 📬️StorePreparation

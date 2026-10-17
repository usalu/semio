//! 📬️ The one-item store preparation factories of the artifact and config lanes.

use super::*;

//#region 📬️StorePreparation
/// 📬️ The document lane's one-item retained preparation. Without it every verb declaring
/// `ArtifactToolPublicationLane::Artifact` is registered with an unsupported publication contract and
/// stays dispatch-dead, whatever its classification.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct EnergyModelStorePreparationFactory;

struct EnergyModelStorePreparation {
    owners: store::OneItemOwners<EnergyModelSnapshot, EnergyModelMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<EnergyModelSnapshot, EnergyModelMutation> for EnergyModelStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<EnergyModelMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<EnergyModelMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &EnergyModelMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("the energy model store preparation rejected its lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &EnergyModelMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "energy-model-artifact-lane"));
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<EnergyModelStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<EnergyModelSnapshot, EnergyModelMutation, EnergyModelMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<EnergyModelSnapshot, EnergyModelMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<EnergyModelSnapshot, EnergyModelMutation, EnergyModelMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        let model = &request.base.get().model;
        let items = model.zones.len().saturating_add(model.spaces.len()).saturating_add(model.surfaces.len()).saturating_add(model.fenestrations.len()).saturating_add(model.materials.len()).saturating_add(model.constructions.len());
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || items > ENERGY_MODEL_RETAINED_WORK_ITEMS
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "energy-model-artifact-request-refused"), request));
        }
        Ok((Box::new(EnergyModelStorePreparation {
            owners: store::OneItemOwners::from_request(request),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<EnergyModelSnapshot, EnergyModelMutation> for EnergyModelStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::Mutation as _;
        let fault = |message: &'static str| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message);
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() || self.owners.failure.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress::default()));
        }
        let authority = self.owners.authority.as_ref().ok_or_else(|| fault("energy-model-artifact-authority-missing"))?;
        let base = self.owners.base.as_ref().ok_or_else(|| fault("energy-model-artifact-base-owner-missing"))?;
        let mutation = self.owners.mutation.take().ok_or_else(|| fault("energy-model-artifact-mutation-owner-missing"))?;
        let inverse = match mutation.inverse(base.get()) {
            Ok(inverse) => inverse,
            Err(error) => {
                *self.owners.mutation = Some(mutation);
                return Err(error);
            }
        };
        let post = match protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()) {
            Ok(post) => post,
            Err(_) => {
                *self.owners.mutation = Some(mutation);
                *self.owners.inverse = Some(inverse);
                *self.owners.failure = Some(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retained the original mutation application refusal"));
                return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retained the original mutation application refusal"));
            }
        };
        let edit = authority.next_edit(mutation, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<EnergyModelSnapshot, EnergyModelMutation>> {
        self.owners.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<EnergyModelSnapshot, EnergyModelMutation>> {
        self.owners.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.owners.begin_close();
    }
    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}
//#endregion 📬️StorePreparation

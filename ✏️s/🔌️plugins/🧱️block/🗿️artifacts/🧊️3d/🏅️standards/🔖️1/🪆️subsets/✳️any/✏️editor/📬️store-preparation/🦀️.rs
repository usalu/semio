//! 📬️ Store preparation: the one-item publication preparation of the Artifact lane and of the Config lane.

use super::*;

//#region 📬️StorePreparation

fn block3d_artifact_mutation_retained_bytes(mutation: &Block3dMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "block3d-artifact-mutation-encode-failed".to_string())
}

fn admit_block3d_artifact_mutation(mutation: &Block3dMutation) -> Result<crate::store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = block3d_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > BLOCK3D_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("block3d-artifact-mutation-envelope".into());
    }
    Ok(crate::store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct Block3dArtifactStorePreparationFactory;

pub(super) struct Block3dArtifactStorePreparation {
    owners: crate::store::OneItemOwners<Block3dSnapshot, Block3dMutation>,
    retained_bytes: usize,
    pub(super) checkpoint: crate::store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

impl crate::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Block3dSnapshot, Block3dMutation> for Block3dArtifactStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<crate::protocol::Edit<Block3dMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn crate::store::ArtifactStoreBatchDigest<Block3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> { crate::store::admit_artifact_batch_digest(edit, grant) }

    fn preflight(&self, mutation: &Block3dMutation, lane: crate::store::os_store::store_schema::HistoryLane) -> Result<crate::store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != crate::store::os_store::store_schema::HistoryLane::Document {
            return Err("block3d-artifact-lane".into());
        }
        admit_block3d_artifact_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &Block3dMutation, _lane: crate::store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Block3dArtifactStorePreparation>(), depth: 1 })
    }

    fn begin(&self, request: crate::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Block3dSnapshot, Block3dMutation, Block3dMutation>, grant: crate::store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn crate::store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Block3dSnapshot, Block3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, crate::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Block3dSnapshot, Block3dMutation, Block3dMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        let retained_bytes = block3d_artifact_mutation_retained_bytes(&request.mutation).unwrap_or(BLOCK3D_ARTIFACT_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != crate::store::os_store::store_schema::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > crate::store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > BLOCK3D_ARTIFACT_STORE_MAXIMUM_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "preparation rejected original publication authority"), request));
        }
        Ok((Box::new(Block3dArtifactStorePreparation {
            owners: crate::store::OneItemOwners::from_request(request),
            retained_bytes,
            checkpoint: crate::store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl crate::store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Block3dSnapshot, Block3dMutation> for Block3dArtifactStorePreparation {
    fn advance(&mut self, grant: crate::store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<crate::store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use crate::protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(crate::store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(crate::store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()));
        }
        if grant.maximum_copy_bytes < self.retained_bytes {
            return Ok(crate::store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let base = self.owners.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "block3d-artifact-base-owner-missing"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "block3d-artifact-mutation-owner-missing"))?;
        let inverse = mutation.inverse(base.get())?;
        let post = crate::protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation could not apply its original mutation"))?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "block3d-artifact-authority-missing"))?;
        let forward = self.owners.mutation.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "block3d-artifact-mutation-owner-missing"))?;
        let edit = authority.next_edit(forward, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = crate::store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(crate::store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: self.retained_bytes, ..Default::default() }))
    }

    fn checkpoint(&self) -> crate::store::os_store::one_item::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&crate::store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Block3dSnapshot, Block3dMutation>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<crate::store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Block3dSnapshot, Block3dMutation>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: crate::store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owners.close_step(grant.retained_grant()) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}

fn block3d_config_mutation_retained_bytes(mutation: &Block3dConfigMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "block3d-config-mutation-encode-failed".to_string())
}

fn admit_block3d_config_mutation(mutation: &Block3dConfigMutation) -> Result<crate::store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = block3d_config_mutation_retained_bytes(mutation)?;
    if retained_bytes > BLOCK3D_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("block3d-config-mutation-envelope".into());
    }
    Ok(crate::store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct Block3dConfigStorePreparationFactory;

pub(super) struct Block3dConfigStorePreparation {
    owners: crate::store::OneItemOwners<Block3dConfig, Block3dConfigMutation>,
    retained_bytes: usize,
    pub(super) checkpoint: crate::store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

impl crate::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Block3dConfig, Block3dConfigMutation> for Block3dConfigStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<crate::protocol::Edit<Block3dConfigMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn crate::store::ArtifactStoreBatchDigest<Block3dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> { crate::store::admit_artifact_batch_digest(edit, grant) }

    fn preflight(&self, mutation: &Block3dConfigMutation, lane: crate::store::os_store::store_schema::HistoryLane) -> Result<crate::store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != crate::store::os_store::store_schema::HistoryLane::Document {
            return Err("block3d-config-lane".into());
        }
        admit_block3d_config_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &Block3dConfigMutation, _lane: crate::store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Block3dConfigStorePreparation>(), depth: 1 })
    }

    fn begin(&self, request: crate::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Block3dConfig, Block3dConfigMutation, Block3dConfigMutation>, grant: crate::store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn crate::store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Block3dConfig, Block3dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, crate::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Block3dConfig, Block3dConfigMutation, Block3dConfigMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        let retained_bytes = block3d_config_mutation_retained_bytes(&request.mutation).unwrap_or(BLOCK3D_CONFIG_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != crate::store::os_store::store_schema::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > crate::store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > BLOCK3D_CONFIG_STORE_MAXIMUM_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "preparation rejected original publication authority"), request));
        }
        Ok((Box::new(Block3dConfigStorePreparation {
            owners: crate::store::OneItemOwners::from_request(request),
            retained_bytes,
            checkpoint: crate::store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl crate::store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Block3dConfig, Block3dConfigMutation> for Block3dConfigStorePreparation {
    fn advance(&mut self, grant: crate::store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<crate::store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use crate::protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(crate::store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(crate::store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()));
        }
        if grant.maximum_copy_bytes < self.retained_bytes {
            return Ok(crate::store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let base = self.owners.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "block3d-config-base-owner-missing"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "block3d-config-mutation-owner-missing"))?;
        let inverse = mutation.inverse(base.get())?;
        let post = crate::protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation could not apply its original mutation"))?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "block3d-config-authority-missing"))?;
        let forward = self.owners.mutation.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "block3d-config-mutation-owner-missing"))?;
        let edit = authority.next_edit(forward, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = crate::store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(crate::store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: self.retained_bytes, ..Default::default() }))
    }

    fn checkpoint(&self) -> crate::store::os_store::one_item::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&crate::store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Block3dConfig, Block3dConfigMutation>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<crate::store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Block3dConfig, Block3dConfigMutation>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: crate::store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owners.close_step(grant.retained_grant()) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}
//#endregion 📬️StorePreparation

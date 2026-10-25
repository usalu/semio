//! 📬️ Store preparation: the one-item publication preparation of the Config lane and of the Artifact lane.

use super::*;

//#region 📬️StorePreparation
const PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES: usize = 32_768;

pub(super) struct Puzzle3dConfigStorePreparation {
    pub(super) owners: store::OneItemOwners<Puzzle3dConfig, Puzzle3dConfigMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    completed_bytes: usize,
    pub(super) phase: u8,
    cancelled: bool,
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct Puzzle3dConfigStorePreparationFactory;

/// 🌉️ `serde_json::to_writer` streamed into a byte-counting `Write` sink so an oversize config could
/// abort mid-encode without materializing it; the in-house `semio_framework_pack_json::to_json_string` has no
/// streaming writer, so this measures the fully-encoded string's byte length against the same
/// `PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES` bound instead — same bound, same error, one buffer instead
/// of zero (`Puzzle3dConfig` is a small, fixed-shape record, never large enough for this to matter).
fn puzzle3d_config_store_bounded_bytes(value: &Puzzle3dConfig) -> Result<usize, String> {
    let encoded = to_json_string(value);
    if encoded.len() > PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Puzzle3d Config Store root exceeds its fixed envelope".to_string());
    }
    Ok(encoded.len())
}

/// 📏️ The exact retained byte cost of one Config mutation, derived from its own encoded payload and
/// bounded by the single `PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES` envelope every Config lane publication
/// obeys. Every `Puzzle3dConfigMutation` variant is admissible: an allowlist here silently killed the
/// whole `Puzzle3dScalarConfigWork` family (camera, projection, sun, LOD, grid, selectable kinds,
/// proximity radius, chunk size, voxel dims, gumball flags, vortex show/direction, contact tolerance,
/// suggestion menu, engagement input) plus both kind-weight routes, because
/// `Puzzle3dConfigStorePreparation::advance` rejects whatever this returns `None` for and the
/// operation's publication lease is cancelled behind it. Only an oversize payload is refused now, and
/// `Snapshot`'s cost stays the whole-config cost it always was — its payload *is* the config.
fn puzzle3d_config_store_mutation_bytes(mutation: &Puzzle3dConfigMutation) -> Option<usize> {
    let encoded = to_json_string(mutation).len();
    (encoded <= PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES).then_some(encoded)
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparation {
    fn advance(&mut self, body:usize,grant: store::os_store::one_item::ArtifactStoreOneItemGrant,_receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::ColdMutation;
        use protocol::Mutation as _;
        use protocol::MutationDiff as _;
        use semio_framework_value::retained_clone::RetainedCloneProgress;
        use semio_framework_value::ValueError;
        use semio_framework_value::ValueRefusalKind;
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle3d Config preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() || self.phase >= 2 {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        match self.phase {
            0 => {
                let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Config preparation lost its exact base root"))?;
                let mutation = self.owners.mutation.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Config preparation lost its mutation owner"))?;
                if puzzle3d_config_store_mutation_bytes(mutation).is_none() {
                    return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Puzzle3d Config preparation rejected its exact mutation envelope"));
                }
                let completed_bytes = puzzle3d_config_store_bounded_bytes(base.get()).map_err(|message| ValueError::new(ValueRefusalKind::OwnershipLimit, message))?;
                let inverse = mutation.inverse(base.get())?;
                let post = protocol::apply_diff_cold(mutation.diff(base.get()).diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle3d Config mutation could not produce its post root"))?;
                let mutation = self.owners.mutation.take().expect("observed original mutation owner");
                *self.owners.candidate = Some((post, inverse, mutation));
                self.completed_bytes = completed_bytes;
                self.phase = 1;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.completed_bytes as u64, digest: [0; 32] };
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress::default()))
            }
            1 => {
                let Some(authority) = self.owners.authority.as_ref() else {
                    return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Config preparation lost its Store authority"));
                };
                let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Config preparation lost its semantic candidate"))?;
                let prepared = match authority.prepare_one_item(authority.next_edit(mutation, inverse), std::sync::Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => {
                        *self.owners.refused = Some((edit, post));
                        return Err(error);
                    }
                };
                self.phase = 2;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.completed_bytes as u64, digest: prepared.edit_digest() };
                *self.owners.prepared = Some(prepared);
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
            }
            _ => Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default())),
        }
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle3dConfig, Puzzle3dConfigMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle3dConfig, Puzzle3dConfigMutation>> {
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

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Puzzle3dConfigMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Puzzle3dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Puzzle3dConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Puzzle3d Config preparation rejected its lane".into());
        }
        let retained_bytes = puzzle3d_config_store_mutation_bytes(mutation).ok_or_else(|| "Puzzle3d Config preparation rejected its exact mutation".to_string())?;
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
    }

    fn begin_demand(&self, _mutation: &Puzzle3dConfigMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Puzzle3dConfigStorePreparation>(), depth: 1 })
    }
    fn begin_copy_byte_demand(&self,input:&Puzzle3dConfigMutation,lane:store::os_store::store_schema::HistoryLane)->Result<usize,semio_framework_value::ValueError>{self.begin_demand(input,lane)?; std::mem::size_of::<store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle3dConfig, Puzzle3dConfigMutation, Puzzle3dConfigMutation>>().checked_add(std::mem::size_of::<Puzzle3dConfigStorePreparation>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original preparation header quote overflow"))}


    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle3dConfig, Puzzle3dConfigMutation, Puzzle3dConfigMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle3dConfig, Puzzle3dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle3dConfig, Puzzle3dConfigMutation, Puzzle3dConfigMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Puzzle3d Config preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Puzzle3dConfigStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), completed_bytes: 0, phase: 0, cancelled: false }), progress))
    }
}

// 🧩️ The Artifact-lane (`Puzzle3dPlaySnapshot`/`Puzzle3dMutation`) sibling of the preparation
// factory above — mirrors `Puzzle5dStorePreparationFactory`
// (`🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`), the one other puzzle-family
// app whose retained commands publish to the Artifact lane through this same trait. Unlike the
// Config preparation above, an arbitrary `Puzzle3dMutation` variant is admitted generically via
// `protocol::Mutation`/`protocol::MutationDiff` rather than an allowlist match, because
// `setActiveExample`'s work loop (`Puzzle3dSetActiveExampleWork`) emits many different mutation
// kinds (delete/create object, attraction, target volume, reference, compatibility, domain,
// catalogs). Each kind is batched to `PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK` and the Complete emit
// is ONE document-replacement edit (no coalesce key), not one store commit per item.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct Puzzle3dArtifactStorePreparationFactory;

pub(super) struct Puzzle3dArtifactStorePreparation {
    pub(super) owners: store::OneItemOwners<Puzzle3dPlaySnapshot, Puzzle3dMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    completed_bytes: usize,
    pub(super) phase: u8,
    cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparationFactory {
    fn operation_wire_source<'a>(&self, operation: &'a Puzzle3dMutation) -> Option<store::ArtifactPreparedOperationSource<'a>> {
        semio_s_artifact_puzzle_3d_mutations::io::binary::mutations::prepared_operation_wire_source(operation)
    }

    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Puzzle3dMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Puzzle3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    /// 🧾️ The forward row plus the inverse rows the leaf's payload schema declares (`x-semio-inverse-rows`): a selection
    /// leaf one setter per changed pose field of every record its attraction re-solve carries along, a scaling one per
    /// target, a removal the record and the attractions it severs.
    fn preflight(&self, mutation: &Puzzle3dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Puzzle3d Artifact preparation rejected its lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf::<Puzzle3dPlaySnapshot, _>(mutation, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &Puzzle3dMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Puzzle3dArtifactStorePreparation>(), depth: 1 })
    }
    fn begin_copy_byte_demand(&self,input:&Puzzle3dMutation,lane:store::os_store::store_schema::HistoryLane)->Result<usize,semio_framework_value::ValueError>{self.begin_demand(input,lane)?; std::mem::size_of::<store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle3dPlaySnapshot, Puzzle3dMutation, Puzzle3dMutation>>().checked_add(std::mem::size_of::<Puzzle3dArtifactStorePreparation>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original preparation header quote overflow"))}


    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle3dPlaySnapshot, Puzzle3dMutation, Puzzle3dMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle3dPlaySnapshot, Puzzle3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle3dPlaySnapshot, Puzzle3dMutation, Puzzle3dMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Puzzle3d Artifact preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Puzzle3dArtifactStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), completed_bytes: 0, phase: 0, cancelled: false }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparation {
    fn advance(&mut self, body:usize,grant: store::os_store::one_item::ArtifactStoreOneItemGrant,_receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::ColdMutation;
        use protocol::Mutation as _;
        use protocol::MutationDiff as _;
        use semio_framework_value::retained_clone::RetainedCloneProgress;
        use semio_framework_value::ValueError;
        use semio_framework_value::ValueRefusalKind;
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle3d Artifact preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() || self.phase >= 2 {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        match self.phase {
            0 => {
                let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Artifact preparation lost its exact base root"))?;
                let mutation = self.owners.mutation.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Artifact preparation lost its mutation owner"))?;
                let inverse = mutation.inverse(base.get())?;
                let post = protocol::apply_diff_cold(mutation.diff(base.get()).diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle3d Artifact mutation could not produce its post root"))?;
                let mutation = self.owners.mutation.take().expect("observed original mutation owner");
                *self.owners.candidate = Some((post, inverse, mutation));
                self.completed_bytes = 1;
                self.phase = 1;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.completed_bytes as u64, digest: [0; 32] };
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress::default()))
            }
            1 => {
                let Some(authority) = self.owners.authority.as_ref() else {
                    return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Artifact preparation lost its Store authority"));
                };
                let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Artifact preparation lost its semantic candidate"))?;
                let prepared = match authority.prepare_one_item(authority.next_edit(mutation, inverse), std::sync::Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => {
                        *self.owners.refused = Some((edit, post));
                        return Err(error);
                    }
                };
                self.phase = 2;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.completed_bytes as u64, digest: prepared.edit_digest() };
                *self.owners.prepared = Some(prepared);
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
            }
            _ => Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default())),
        }
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle3dPlaySnapshot, Puzzle3dMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle3dPlaySnapshot, Puzzle3dMutation>> {
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

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}
//#endregion 📬️StorePreparation

//! 🧰️ Kit of the puzzle 2d editor: the one-item store preparations of the config and the artifact lane. See [`super`].

use crate::{editor::puzzle2d::config::Puzzle2dConfig, editor::puzzle2d::config::Puzzle2dConfigMutation};
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;

//#region 🔖️PlayApp
//#region 🧵️RetainedCommands
//#region 📬️StorePreparation
/// 📦️ Fixed envelope one retained `Puzzle2dConfig` commit may occupy. `brush_candidates` is the only
/// variable-length field (one entry per compatible brush candidate on the open slot), so 64 KiB
/// leaves two orders of magnitude of headroom over a real slot while still refusing an unbounded root.
const PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES: usize = 65_536;

struct Puzzle2dConfigStorePreparation {
    pub owners: store::OneItemOwners<Puzzle2dConfig, Puzzle2dConfigMutation>,
    pub checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    completed_bytes: usize,
    pub phase: u8,
    pub cancelled: bool,
}

/// 🎚️ Config-lane preparation factory — the precondition every `Config` publication contract above
/// depends on (`build_config_store_one_item_preparation_factory`); mirrors
/// `Puzzle3dConfigStorePreparationFactory`.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct Puzzle2dConfigStorePreparationFactory;

/// 🌉️ Measures the fully encoded config root against the fixed store envelope. `Puzzle2dConfig` is
/// encoded through the same in-house `semio_framework_pack_json` writer its `OpBinary` codec uses, so the measured
/// length is the length the store actually retains.
fn puzzle2d_config_store_bounded_bytes(value: &Puzzle2dConfig) -> Result<usize, String> {
    let encoded = semio_framework_pack_json::to_json_string(value);
    if encoded.len() > PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Puzzle2d Config Store root exceeds its fixed envelope".to_string());
    }
    Ok(encoded.len())
}

fn puzzle2d_config_store_mutation_bytes(mutation: &Puzzle2dConfigMutation) -> Option<usize> {
    let encoded = semio_framework_pack_json::to_json_string(mutation);
    (encoded.len() <= PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES).then_some(encoded.len())
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle2dConfig, Puzzle2dConfigMutation> for Puzzle2dConfigStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::{Mutation as _, MutationDiff as _};
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle2d Config preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() || self.phase >= 2 {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        match self.phase {
            0 => {
                let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Config preparation lost its exact base root"))?;
                let mutation = self.owners.mutation.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Config preparation lost its mutation owner"))?;
                if puzzle2d_config_store_mutation_bytes(mutation).is_none() {
                    return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Puzzle2d Config preparation rejected its exact mutation envelope"));
                }
                let completed_bytes = puzzle2d_config_store_bounded_bytes(base.get()).map_err(|message| ValueError::new(ValueRefusalKind::OwnershipLimit, message))?;
                let inverse = mutation.inverse(base.get())?;
                let post = protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle2d Config mutation could not produce its post root"))?;
                let mutation = self.owners.mutation.take().expect("observed original mutation owner");
                *self.owners.candidate = Some((post, inverse, mutation));
                self.completed_bytes = completed_bytes;
                self.phase = 1;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.completed_bytes as u64, digest: [0; 32] };
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress::default()))
            }
            1 => {
                let Some(authority) = self.owners.authority.as_ref() else {
                    return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Config preparation lost its Store authority"));
                };
                let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Config preparation lost its semantic candidate"))?;
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

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle2dConfig, Puzzle2dConfigMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle2dConfig, Puzzle2dConfigMutation>> {
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

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Puzzle2dConfig, Puzzle2dConfigMutation> for Puzzle2dConfigStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Puzzle2dConfigMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Puzzle2dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Puzzle2dConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Puzzle2d Config preparation rejected its lane".into());
        }
        let retained_bytes = puzzle2d_config_store_mutation_bytes(mutation).ok_or_else(|| "Puzzle2d Config preparation rejected its exact mutation".to_string())?;
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
    }

    fn begin_demand(&self, _mutation: &Puzzle2dConfigMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Puzzle2dConfigStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle2dConfig, Puzzle2dConfigMutation, Puzzle2dConfigMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle2dConfig, Puzzle2dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle2dConfig, Puzzle2dConfigMutation, Puzzle2dConfigMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Puzzle2d Config preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Puzzle2dConfigStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), completed_bytes: 0, phase: 0, cancelled: false }), progress))
    }
}

/// 🧩️ The Artifact-lane sibling of the Config preparation above. A `Puzzle2dMutation` is admitted
/// generically through `protocol::Mutation`/`protocol::MutationDiff` rather than an allowlist,
/// because one retained completion emits whatever the granular document delta produced (create /
/// delete node, connect / disconnect handles, manifest, compatibility, catalogs) one per store turn.

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct Puzzle2dArtifactStorePreparationFactory;

struct Puzzle2dArtifactStorePreparation {
    pub owners: store::OneItemOwners<Puzzle2dPlaySnapshot, Puzzle2dMutation>,
    pub checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    completed_bytes: usize,
    pub phase: u8,
    pub cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Puzzle2dPlaySnapshot, Puzzle2dMutation> for Puzzle2dArtifactStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Puzzle2dMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Puzzle2dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    /// 🧾️ The forward row plus the inverse rows the leaf's payload schema declares (`x-semio-inverse-rows`): `delete-node`
    /// re-creates the node and re-connects every edge on its handles, a selection transform restores each target.
    fn preflight(&self, mutation: &Puzzle2dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Puzzle2d Artifact preparation rejected its lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf::<Puzzle2dPlaySnapshot, _>(mutation, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &Puzzle2dMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Puzzle2dArtifactStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle2dPlaySnapshot, Puzzle2dMutation, Puzzle2dMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle2dPlaySnapshot, Puzzle2dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle2dPlaySnapshot, Puzzle2dMutation, Puzzle2dMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Puzzle2d Artifact preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Puzzle2dArtifactStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), completed_bytes: 0, phase: 0, cancelled: false }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle2dPlaySnapshot, Puzzle2dMutation> for Puzzle2dArtifactStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::{Mutation as _, MutationDiff as _};
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle2d Artifact preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() || self.phase >= 2 {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        match self.phase {
            0 => {
                let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Artifact preparation lost its exact base root"))?;
                let mutation = self.owners.mutation.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Artifact preparation lost its mutation owner"))?;
                let inverse = mutation.inverse(base.get())?;
                let post = protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle2d Artifact mutation could not produce its post root"))?;
                let mutation = self.owners.mutation.take().expect("observed original mutation owner");
                *self.owners.candidate = Some((post, inverse, mutation));
                self.completed_bytes = 1;
                self.phase = 1;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.completed_bytes as u64, digest: [0; 32] };
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress::default()))
            }
            1 => {
                let Some(authority) = self.owners.authority.as_ref() else {
                    return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Artifact preparation lost its Store authority"));
                };
                let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Artifact preparation lost its semantic candidate"))?;
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

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle2dPlaySnapshot, Puzzle2dMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle2dPlaySnapshot, Puzzle2dMutation>> {
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
//#endregion 🧵️RetainedCommands
//#endregion 🔖️PlayApp

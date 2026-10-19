//! 📬️ Config store preparation: the byte bounds, the admission and the one-item publication preparation of the Config lane.

use super::*;
use crate::editor::sourcing::editor_vocabulary::{SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES, SOURCING_CURATION_CONFIG_TEXT_BYTES};

//#region 📬️ConfigStorePreparation
/// 📏️ The typed-in filter state's own retained byte envelope — query, module ids, typology path and
/// sort column, plus the struct and its string owners. Contributions are NOT filter text and are
/// priced against [`SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES`] instead.
const SOURCING_CURATION_CONFIG_FILTER_STORE_BYTES: usize = 768;
pub const SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES: usize = SOURCING_CURATION_CONFIG_FILTER_STORE_BYTES + SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES;
pub const SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS: usize = 256;
const SOURCING_CURATION_CONFIG_METADATA_BYTES: usize = 64;
/// 🎟️ What one config `advance`/`close_step` turn costs, and the ONLY figure the grant is ever
/// compared against. The host drives this lane with a fixed `ArtifactStoreOneItemGrant
/// { maximum_items: 1, maximum_bytes: TYPED_OPERATION_RESULT_PAGE_BYTES }` (4 KiB), so a gate that
/// scaled with `SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES` would go `Blocked` forever the moment
/// the contributions lane grew past one page — stalling every filter edit instead of failing it.
/// The config's own size is a VALIDATION (`sourcing_curation_config_bytes`), never the gate. Same
/// shape as `SOURCING_CURATION_DOCUMENT_GRANT_BYTES` below.
pub const SOURCING_CURATION_CONFIG_GRANT_BYTES: usize = 4_096;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct SourcingCurationConfigPreparationFactory;

pub struct SourcingCurationConfigPreparation {
    pub owners: store::OneItemOwners<SourcingCurationConfig, SourcingCurationConfigMutation>,
    pub checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    pub cancelled: bool,
    pub retained_bytes: usize,
}

fn sourcing_curation_config_bytes(config: &SourcingCurationConfig) -> Result<usize, String> {
    let items = config.filters.module_ids.len().saturating_add(config.filters.typology_path.len());
    if items > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS { return Err("Sourcing Config base exceeds its retained item envelope".into()); }
    let bytes = config.filters.query.len()
        .saturating_add(config.filters.module_ids.iter().map(String::len).sum::<usize>())
        .saturating_add(config.filters.typology_path.iter().map(String::len).sum::<usize>())
        .saturating_add(config.filters.sort.as_ref().map_or(0, |sort| sort.column_id.len()));
    if bytes > SOURCING_CURATION_CONFIG_TEXT_BYTES { return Err("Sourcing Config base exceeds its encoded text envelope".into()); }
    if config.contributions_json.len() > SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES { return Err("Sourcing Config base exceeds its retained contributions envelope".into()); }
    let bytes = bytes.saturating_add(config.contributions_json.len())
        .saturating_add(size_of::<SourcingCurationConfig>())
        .saturating_add(items.saturating_mul(size_of::<String>()));
    if bytes > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES { return Err("Sourcing Config base exceeds its retained byte envelope".into()); }
    Ok(bytes)
}

/// 🧺️ The shared tail every admitted config mutation pays — the mutation owner plus one `String` owner per item it
/// carries, checked against the one-item preparation envelope; answers the retained bytes.
fn sourcing_curation_config_retained_bytes(items: usize, retained_bytes: usize) -> Result<usize, String> {
    let retained_bytes = retained_bytes.saturating_add(size_of::<SourcingCurationConfigMutation>()).saturating_add(items.saturating_mul(size_of::<String>()));
    if items > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS || retained_bytes > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Sourcing Config mutation exceeds its fixed one-item preparation envelope".into());
    }
    Ok(retained_bytes)
}

pub fn sourcing_curation_config_mutation_retained_bytes(mutation: &SourcingCurationConfigMutation) -> Result<usize, String> {
    if let SourcingCurationConfigMutation::SetContributions(SetContributionsEdit { json }) = mutation {
        if json.len() > SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES { return Err("Sourcing Config contributions exceed their retained contributions envelope".into()); }
        return sourcing_curation_config_retained_bytes(1, json.len());
    }
    let (items, retained_bytes) = match mutation {
        SourcingCurationConfigMutation::SetContributions(_) => return Err("Sourcing Config preparation rejects a non-retained mutation".into()),
        SourcingCurationConfigMutation::SetFilterQuery(SetFilterQueryEdit { value }) => (1, value.len()),
        SourcingCurationConfigMutation::SetFilterModules(SetFilterModulesEdit { module_ids }) => {
            if module_ids.len() > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS { return Err("Sourcing Config module filter exceeds its retained item envelope".into()); }
            (module_ids.len().max(1), module_ids.iter().map(String::len).sum())
        }
        SourcingCurationConfigMutation::SetFilterTypology(SetFilterTypologyEdit { path }) => {
            if path.len() > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS { return Err("Sourcing Config typology exceeds its retained item envelope".into()); }
            (path.len().max(1), path.iter().map(String::len).sum())
        }
        SourcingCurationConfigMutation::SetSort(SetSortEdit { sort }) => (1, sort.as_ref().map_or(0, |sort| sort.column_id.len())),
        SourcingCurationConfigMutation::SetFilterMinAvailability(_) => (1, 0),
    };
    if retained_bytes > SOURCING_CURATION_CONFIG_TEXT_BYTES { return Err("Sourcing Config mutation exceeds its encoded text envelope".into()); }
    sourcing_curation_config_retained_bytes(items, retained_bytes)
}

pub fn refusal_of(message: String) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message)
}

pub fn prepare_sourcing_curation_config(base: &SourcingCurationConfig, mutation: &SourcingCurationConfigMutation) -> Result<(SourcingCurationConfig, Vec<SourcingCurationConfigMutation>), semio_framework_value::ValueError> {
    sourcing_curation_config_mutation_retained_bytes(mutation).map_err(refusal_of)?;
    sourcing_curation_config_bytes(base).map_err(refusal_of)?;
    let mut post = base.clone();
    let inverse = match mutation {
        SourcingCurationConfigMutation::SetFilterQuery(SetFilterQueryEdit { value }) => { post.filters.query = value.clone(); SourcingCurationConfigMutation::SetFilterQuery(SetFilterQueryEdit { value: base.filters.query.clone() }) }
        SourcingCurationConfigMutation::SetFilterTypology(SetFilterTypologyEdit { path }) => { post.filters.typology_path = path.clone(); SourcingCurationConfigMutation::SetFilterTypology(SetFilterTypologyEdit { path: base.filters.typology_path.clone() }) }
        SourcingCurationConfigMutation::SetFilterModules(SetFilterModulesEdit { module_ids }) => { post.filters.module_ids = module_ids.clone(); SourcingCurationConfigMutation::SetFilterModules(SetFilterModulesEdit { module_ids: base.filters.module_ids.clone() }) }
        SourcingCurationConfigMutation::SetFilterMinAvailability(SetFilterMinAvailabilityEdit { value }) => { post.filters.min_availability = *value; SourcingCurationConfigMutation::SetFilterMinAvailability(SetFilterMinAvailabilityEdit { value: base.filters.min_availability }) }
        SourcingCurationConfigMutation::SetSort(SetSortEdit { sort }) => { post.filters.sort = sort.clone(); SourcingCurationConfigMutation::SetSort(SetSortEdit { sort: base.filters.sort.clone() }) }
        SourcingCurationConfigMutation::SetContributions(SetContributionsEdit { json }) => { post.contributions_json = json.clone(); SourcingCurationConfigMutation::SetContributions(SetContributionsEdit { json: base.contributions_json.clone() }) }
        _ => return Err(refusal_of("Sourcing Config preparation rejects a non-retained mutation".into())),
    };
    sourcing_curation_config_bytes(&post).map_err(refusal_of)?;
    Ok((post, vec![inverse]))
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<SourcingCurationConfig, SourcingCurationConfigMutation> for SourcingCurationConfigPreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<SourcingCurationConfigMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<SourcingCurationConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &SourcingCurationConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Sourcing Config preparation rejected its lane".into());
        }
        sourcing_curation_config_mutation_retained_bytes(mutation)?;
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, SOURCING_CURATION_CONFIG_GRANT_BYTES))
    }

    fn begin_demand(&self, _mutation: &SourcingCurationConfigMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<SourcingCurationConfigPreparation>(), depth: 1 })
    }

    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<SourcingCurationConfig, SourcingCurationConfigMutation, SourcingCurationConfigMutation>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<SourcingCurationConfig, SourcingCurationConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<SourcingCurationConfig, SourcingCurationConfigMutation, SourcingCurationConfigMutation>)> {
        if self.preflight(&request.mutation, request.lane).is_err()
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > SOURCING_CURATION_CONFIG_METADATA_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Sourcing Config preparation rejected original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        Ok((Box::new(SourcingCurationConfigPreparation {
            owners: store::OneItemOwners::from_request(request),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            retained_bytes: 0,
        }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<SourcingCurationConfig, SourcingCurationConfigMutation> for SourcingCurationConfigPreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.owners.refused.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal")); }
        if self.owners.prepared.is_some() { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default())); }
        if self.owners.candidate.is_none() {
            let base = self.owners.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Sourcing Config preparation lost its exact base root"))?.get();
            sourcing_curation_config_bytes(base).map_err(refusal_of)?;
            let bytes = SOURCING_CURATION_CONFIG_GRANT_BYTES;
            if grant.maximum_copy_bytes < bytes { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
            let mutation = self.owners.mutation.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Sourcing Config preparation lost its mutation owner"))?;
            let (post, inverse) = prepare_sourcing_curation_config(base, mutation)?;
            let mutation = self.owners.mutation.take().expect("observed original mutation owner");
            *self.owners.candidate = Some((post, inverse, mutation));
            self.retained_bytes = bytes;
            self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: bytes as u64, digest: [0; 32] };
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, Default::default()));
        }
        if grant.maximum_copy_bytes < self.retained_bytes { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
        let authority = self.owners.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Sourcing Config preparation lost its Store authority"))?;
        let (post, inverse, forward) = self.owners.candidate.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Sourcing Config preparation lost its candidate"))?;
        let prepared = match authority.prepare_one_item(authority.next_edit(forward, inverse), std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => { *self.owners.refused = Some((edit, post)); return Err(error); }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<SourcingCurationConfig, SourcingCurationConfigMutation>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<SourcingCurationConfig, SourcingCurationConfigMutation>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owners.close_step(grant.retained_grant()) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}
//#endregion 📬️ConfigStorePreparation

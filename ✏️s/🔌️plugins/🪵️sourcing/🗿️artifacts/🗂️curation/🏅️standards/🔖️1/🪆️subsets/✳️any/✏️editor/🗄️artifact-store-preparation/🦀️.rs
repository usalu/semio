//! 🗄️ Artifact store preparation: the document bounds, the admission and the one-item publication preparation of the Artifact lane.

use super::*;

//#region 📬️ArtifactStorePreparation
/// 🧺️ Fixed envelope for the curation document's retained one-item publication lane. The document is a
/// curated-selection list over a content-addressed catalog child, so the only unbounded axes are the
/// curated rows and the sourcing-owned `stock_extra` overflow — both counted here rather than encoded.
const SOURCING_CURATION_DOCUMENT_MAXIMUM_ITEMS: usize = 4_096;
/// 🔤️ Longest object id (and longest single mutation payload string) the retained lane admits.
const SOURCING_CURATION_DOCUMENT_TEXT_BYTES: usize = 256;
const SOURCING_CURATION_DOCUMENT_MAXIMUM_BYTES: usize = 512 * 1_024;
const SOURCING_CURATION_DOCUMENT_METADATA_BYTES: usize = 64;
/// 🎟️ What one `advance`/`close_step` turn costs, and the ONLY figure the grant is ever compared
/// against. The host drives this lane with a fixed `ArtifactStoreOneItemGrant { maximum_items: 1,
/// maximum_bytes: TYPED_OPERATION_RESULT_PAGE_BYTES }` (4 KiB), so a gate that scaled with the
/// document — `grant.maximum_bytes < measured_base_bytes` — would go `Blocked` forever the moment a
/// curation outgrew one page, stalling the operation instead of failing it. The base's own size is a
/// VALIDATION (`sourcing_curation_document_bytes`, rejected past
/// `SOURCING_CURATION_DOCUMENT_MAXIMUM_BYTES`), never the gate. Same "demand exactly one full grant"
/// shape as the config lane above.
const SOURCING_CURATION_DOCUMENT_GRANT_BYTES: usize = 4_096;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct SourcingCurationArtifactPreparationFactory;

pub struct SourcingCurationArtifactPreparation {
    pub owners: store::OneItemOwners<CurationSnapshot, SourcingMutation>,
    pub checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    pub cancelled: bool,
    pub retained_bytes: usize,
}

/// 📏️ The retained footprint of one document base — rejected rather than truncated when the curated
/// list, the overflow catalog half, or any single id outgrows the fixed envelope.
fn sourcing_curation_document_bytes(document: &CurationSnapshot) -> Result<usize, String> {
    let items = document.curated.len().saturating_add(document.stock_extra.len());
    if items > SOURCING_CURATION_DOCUMENT_MAXIMUM_ITEMS { return Err("Sourcing Curation base exceeds its retained item envelope".into()); }
    if document.curated.iter().any(|item| item.object_id.len() > SOURCING_CURATION_DOCUMENT_TEXT_BYTES) || document.stock_extra.iter().any(|extra| extra.id.len() > SOURCING_CURATION_DOCUMENT_TEXT_BYTES) {
        return Err("Sourcing Curation base carries an object id beyond its encoded text envelope".into());
    }
    let bytes = document.curated.iter().map(|item| item.object_id.len()).sum::<usize>()
        .saturating_add(document.stock_extra.iter().map(|extra| extra.id.len().saturating_add(extra.name.len()).saturating_add(extra.module_id.len()).saturating_add(extra.typology_path.iter().map(String::len).sum::<usize>())).sum::<usize>())
        .saturating_add(size_of::<CurationSnapshot>())
        .saturating_add(items.saturating_mul(size_of::<CuratedItem>().max(size_of::<ObjectKindExtra>())));
    if bytes > SOURCING_CURATION_DOCUMENT_MAXIMUM_BYTES { return Err("Sourcing Curation base exceeds its retained byte envelope".into()); }
    Ok(bytes)
}

/// 📏️ One semantic mutation's own retained footprint. Every variant addresses exactly one curated row,
/// so the item count is one and the byte count is that row's id.
fn sourcing_curation_mutation_retained_bytes(mutation: &SourcingMutation) -> Result<usize, String> {
    let text = match mutation {
        SourcingMutation::CreateCuratedItem(payload) => payload.item.object_id.len(),
        SourcingMutation::DeleteCuratedItem(payload) => payload.object_id.len(),
        SourcingMutation::ChangeCuratedItemCount(payload) => payload.object_id.len(),
    };
    if text > SOURCING_CURATION_DOCUMENT_TEXT_BYTES { return Err("Sourcing Curation mutation exceeds its encoded text envelope".into()); }
    let retained_bytes = text.saturating_add(size_of::<SourcingMutation>()).saturating_add(size_of::<String>());
    if retained_bytes > SOURCING_CURATION_DOCUMENT_MAXIMUM_BYTES { return Err("Sourcing Curation mutation exceeds its fixed one-item preparation envelope".into()); }
    Ok(retained_bytes)
}

/// 🧮️ Runs the mutation's own semantic `diff`/`inverse` against `base` and applies the resulting diff.
/// A `Fatal`/`Error` outcome (duplicate id, missing target) is a REJECTION here, not a silent no-op —
/// the retained lane must never publish an edit whose forward diff the vocabulary refused.
fn prepare_sourcing_curation_document(base: &CurationSnapshot, mutation: &SourcingMutation) -> Result<(CurationSnapshot, Vec<SourcingMutation>), semio_framework_value::ValueError> {
    sourcing_curation_mutation_retained_bytes(mutation).map_err(refusal_of)?;
    sourcing_curation_document_bytes(base).map_err(refusal_of)?;
    let outcome = protocol::Mutation::diff(mutation, base);
    if let Some(message) = outcome.messages().iter().find(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
        return Err(refusal_of(format!("Sourcing Curation mutation was refused by its own vocabulary: {}", message.message)));
    }
    let inverse = protocol::Mutation::inverse(mutation, base)?;
    let post = protocol::apply_diff(outcome.diff(), base).map_err(|error| refusal_of(format!("Sourcing Curation mutation could not apply onto its exact base: {}", error.message)))?;
    sourcing_curation_document_bytes(&post).map_err(refusal_of)?;
    Ok((post, inverse))
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<CurationSnapshot, SourcingMutation> for SourcingCurationArtifactPreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<SourcingMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<SourcingMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &SourcingMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Sourcing Curation preparation rejected its lane".into());
        }
        sourcing_curation_mutation_retained_bytes(mutation)?;
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, SOURCING_CURATION_DOCUMENT_GRANT_BYTES))
    }

    fn begin_demand(&self, _mutation: &SourcingMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<SourcingCurationArtifactPreparation>(), depth: 1 })
    }

    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<CurationSnapshot, SourcingMutation, SourcingMutation>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<CurationSnapshot, SourcingMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<CurationSnapshot, SourcingMutation, SourcingMutation>)> {
        if self.preflight(&request.mutation, request.lane).is_err()
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > SOURCING_CURATION_DOCUMENT_METADATA_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Sourcing Curation preparation rejected original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        Ok((Box::new(SourcingCurationArtifactPreparation {
            owners: store::OneItemOwners::from_request(request),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            retained_bytes: 0,
        }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<CurationSnapshot, SourcingMutation> for SourcingCurationArtifactPreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.owners.refused.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal")); }
        if self.owners.prepared.is_some() { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default())); }
        if self.owners.candidate.is_none() {
            let base = self.owners.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Sourcing Curation preparation lost its exact base root"))?.get();
            sourcing_curation_document_bytes(base).map_err(refusal_of)?;
            let bytes = SOURCING_CURATION_DOCUMENT_GRANT_BYTES;
            if grant.maximum_copy_bytes < bytes { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
            let mutation = self.owners.mutation.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Sourcing Curation preparation lost its mutation owner"))?;
            let (post, inverse) = prepare_sourcing_curation_document(base, mutation)?;
            let mutation = self.owners.mutation.take().expect("observed original mutation owner");
            *self.owners.candidate = Some((post, inverse, mutation));
            self.retained_bytes = bytes;
            self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: bytes as u64, digest: [0; 32] };
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, Default::default()));
        }
        if grant.maximum_copy_bytes < self.retained_bytes { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
        let authority = self.owners.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Sourcing Curation preparation lost its Store authority"))?;
        let (post, inverse, forward) = self.owners.candidate.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Sourcing Curation preparation lost its candidate"))?;
        let prepared = match authority.prepare_one_item(authority.next_edit(forward, inverse), std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => { *self.owners.refused = Some((edit, post)); return Err(error); }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<CurationSnapshot, SourcingMutation>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<CurationSnapshot, SourcingMutation>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owners.close_step(grant.retained_grant()) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}
//#endregion 📬️ArtifactStorePreparation

//! 🗄️ Artifact store preparation: the document envelope and the shared cold preparation factory of the Artifact lane.

use super::*;

//#region 📬️ArtifactStorePreparation
/// 🧺️ Fixed envelope for the curation document's retained document lane. The document is a curated-selection list over a content-addressed catalog child, so the only unbounded axes are the curated rows and the sourcing-owned `stock_extra` overflow.
const SOURCING_CURATION_DOCUMENT_MAXIMUM_ITEMS: usize = 4_096;
/// 🔤️ Longest object id the retained lane admits.
const SOURCING_CURATION_DOCUMENT_TEXT_BYTES: usize = 256;
const SOURCING_CURATION_DOCUMENT_MAXIMUM_BYTES: usize = 512 * 1_024;

/// 📏️ The retained footprint of one document base; past the fixed envelope it prices as unbounded so the shared preparation refuses it at preflight.
fn sourcing_curation_document_bytes(document: &CurationSnapshot) -> usize {
    let items = document.curated.len().saturating_add(document.stock_extra.len());
    if items > SOURCING_CURATION_DOCUMENT_MAXIMUM_ITEMS || document.curated.iter().any(|item| item.object_id.len() > SOURCING_CURATION_DOCUMENT_TEXT_BYTES) || document.stock_extra.iter().any(|extra| extra.id.len() > SOURCING_CURATION_DOCUMENT_TEXT_BYTES) {
        return usize::MAX;
    }
    document.curated.iter().map(|item| item.object_id.len()).sum::<usize>()
        .saturating_add(document.stock_extra.iter().map(|extra| extra.id.len().saturating_add(extra.name.len()).saturating_add(extra.module_id.len()).saturating_add(extra.typology_path.iter().map(String::len).sum::<usize>())).sum::<usize>())
        .saturating_add(size_of::<CurationSnapshot>())
        .saturating_add(items.saturating_mul(size_of::<CuratedItem>().max(size_of::<ObjectKindExtra>())))
}

/// 📏️ One semantic mutation's own retained footprint: every variant addresses exactly one curated row, so it is that row's id.
fn sourcing_curation_mutation_bytes(mutation: &SourcingMutation) -> usize {
    let text = match mutation {
        SourcingMutation::CreateCuratedItem(payload) => payload.item.object_id.len(),
        SourcingMutation::DeleteCuratedItem(payload) => payload.object_id.len(),
        SourcingMutation::ChangeCuratedItemCount(payload) => payload.object_id.len(),
    };
    if text > SOURCING_CURATION_DOCUMENT_TEXT_BYTES { usize::MAX } else { text.saturating_add(size_of::<SourcingMutation>()).saturating_add(size_of::<String>()) }
}

/// 📏️ The fixed document envelope, priced by the domain's own measures.
fn sourcing_curation_preparation_bounds() -> store::ColdPreparationBounds<CurationSnapshot, SourcingMutation> {
    store::ColdPreparationBounds { maximum_bytes: SOURCING_CURATION_DOCUMENT_MAXIMUM_BYTES, mutation_bytes: sourcing_curation_mutation_bytes, snapshot_bytes: sourcing_curation_document_bytes }
}

/// 🏭️ The one semantic issuer of the curation document: publication, replay, report and initialization all prepare through the shared cold apply over the curation cold semantics, within the fixed document envelope.
type SourcingCurationSemanticProducer = store::ColdMutationApplyFactory<CurationSnapshot, SourcingMutation>;

/// 🔁️ The one-item preparation of the Artifact lane, from the same producer type the owner catalog installs.
pub fn sourcing_curation_artifact_preparation_factory() -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<CurationSnapshot, SourcingMutation>> {
    std::sync::Arc::new(SourcingCurationSemanticProducer::bounded(sourcing_curation_preparation_bounds()))
}

/// 📏️ Quotes the single curation semantic producer and its exact document owner catalog.
pub fn sourcing_curation_store_owners_source_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
    store::os_store::document_store_owners::bounded_artifact_store_owners_shared_semantic_source_demands::<CurationSnapshot, SourcingMutation, SourcingCurationSemanticProducer>(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 }, 0)
}

/// 🗃️ Admits the document owner catalog with the shared semantic producer installed for publication and replay. Every curation edit declares the whole document as its conflict target.
pub fn admit_sourcing_curation_store_owners(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<CurationSnapshot, SourcingMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<CurationSnapshot, SourcingMutation>> {
    store::os_store::document_store_owners::bounded_artifact_store_owners_with_shared_semantic_preparation(grant, semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 }, 0, || SourcingCurationSemanticProducer::bounded(sourcing_curation_preparation_bounds())).map(|(mut owners, progress)| {
        owners.operation_wire = owners.operation_wire.with_original_binary_receiving().with_original_target(store::os_store::whole_document_target::<SourcingMutation>);
        (owners, progress)
    })
}
//#endregion 📬️ArtifactStorePreparation

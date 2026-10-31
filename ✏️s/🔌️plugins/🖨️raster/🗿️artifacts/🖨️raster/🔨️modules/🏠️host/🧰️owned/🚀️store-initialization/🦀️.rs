//! 🚀️ raster store initialization: the document store owners the framework-default initializer builds on.

use super::*;

pub type RasterDocumentStoreOwners = store::os_store::document_store_owners::DocumentStoreOwners<RasterSnapshot, RasterMutation>;

pub type RasterOwnersAdmission = Result<(RasterDocumentStoreOwners, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<RasterSnapshot, RasterMutation>>;

pub type RasterColdIssuer = store::ColdMutationApplyFactory<RasterSnapshot, RasterMutation>;

const RASTER_PREPARATION_CHILDREN: semio_framework_value::retained_clone::RetainedCloneBirthDemand = semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 };

/// 🧊️ Installs the shared cold issuer as both publication and replay authority, so history-bearing documents replay through the framework-default initializer.
pub fn raster_document_store_owners_admission(grant: RetainedCloneGrant) -> RasterOwnersAdmission {
    RasterDocumentStoreOwners::admit_source_constructor_with_shared_semantic_preparation(grant, RASTER_PREPARATION_CHILDREN, 0, || (RasterSnapshotRetirementFactory, RasterSnapshotRetirementFactory, RasterMutationRetirementFactory, store::ArtifactStoreCursorDisposer::<RasterSnapshot, RasterMutation>::new(), RasterColdIssuer::default()))
}

/// 📏️ Quotes the source Arcs, disposer and shared cold issuer the Raster owner catalog admits before any factory ticket is born.
pub fn raster_document_store_owners_source_demands() -> Result<semio_framework_value::RetirementDemand, ValueError> {
    RasterDocumentStoreOwners::shared_semantic_source_demands::<RasterSnapshotRetirementFactory, RasterSnapshotRetirementFactory, RasterMutationRetirementFactory, store::ArtifactStoreCursorDisposer<RasterSnapshot, RasterMutation>, RasterColdIssuer>(RASTER_PREPARATION_CHILDREN, 0)
}

pub fn raster_document_store_owners() -> Result<RasterDocumentStoreOwners, ValueError> {
    let demand = raster_document_store_owners_source_demands()?;
    store::os_store::document_store_owners::fund_document_store_owners(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: demand.capacity_bytes, depth: demand.depth.max(1) }, raster_document_store_owners_admission)
}

pub(super) fn invariant(message: &str) -> ValueError {
    ValueError::new(ValueRefusalKind::InvariantViolated, message)
}

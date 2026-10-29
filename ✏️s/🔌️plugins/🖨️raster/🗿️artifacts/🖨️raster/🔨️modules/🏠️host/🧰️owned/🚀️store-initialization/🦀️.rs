//! 🚀️ raster store initialization: the document store owners the framework-default initializer builds on.

use super::*;

pub type RasterDocumentStoreOwners = store::os_store::document_store_owners::DocumentStoreOwners<RasterSnapshot, RasterMutation>;

pub type RasterOwnersAdmission = Result<(RasterDocumentStoreOwners, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<RasterSnapshot, RasterMutation>>;

pub fn raster_document_store_owners_admission(grant: RetainedCloneGrant) -> RasterOwnersAdmission {
    RasterDocumentStoreOwners::admit_source_constructor(grant, || (RasterSnapshotRetirementFactory, RasterSnapshotRetirementFactory, RasterMutationRetirementFactory, store::ArtifactStoreCursorDisposer::<RasterSnapshot, RasterMutation>::new()))
}

/// 📏️ Quotes the source Arcs and disposer the Raster owner catalog admits before any factory ticket is born.
pub fn raster_document_store_owners_source_demands() -> Result<semio_framework_value::RetirementDemand, ValueError> {
    let capacity_bytes = RasterDocumentStoreOwners::source_birth_bytes::<RasterSnapshotRetirementFactory, RasterSnapshotRetirementFactory, RasterMutationRetirementFactory, store::ArtifactStoreCursorDisposer<RasterSnapshot, RasterMutation>>()?;
    Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: 1, ..Default::default() })
}

pub fn raster_document_store_owners() -> RasterDocumentStoreOwners {
    let capacity_bytes = RasterDocumentStoreOwners::source_birth_bytes::<RasterSnapshotRetirementFactory, RasterSnapshotRetirementFactory, RasterMutationRetirementFactory, store::ArtifactStoreCursorDisposer<RasterSnapshot, RasterMutation>>().expect("Raster owner catalog source layout");
    store::os_store::document_store_owners::fund_document_store_owners(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: 1 }, raster_document_store_owners_admission).expect("funded Raster owner catalog")
}

pub(super) fn invariant(message: &str) -> ValueError {
    ValueError::new(ValueRefusalKind::InvariantViolated, message)
}

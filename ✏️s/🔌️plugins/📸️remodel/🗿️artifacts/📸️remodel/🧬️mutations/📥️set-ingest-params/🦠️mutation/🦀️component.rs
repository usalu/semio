//! 📥️ Remodel mutation — `SetIngestParams` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, params: &crate::artifacts::remodel::IngestParams) {
    next.params.ingest = params.clone();
}
//#endregion 🔖️Mutation

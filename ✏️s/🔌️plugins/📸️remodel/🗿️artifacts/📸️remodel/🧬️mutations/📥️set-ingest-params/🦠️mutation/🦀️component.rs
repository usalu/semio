//! 📥️ Remodel mutation — `SetIngestParams` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, params: &crate::artifacts::remodel::IngestParams) {
    next.params.ingest = params.clone();
}
//#endregion 🔖️Mutation

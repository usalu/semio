//! 🔺️ `remove-type` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::RemoveType;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitTypeList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RemoveType, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if !base.types.iter().any(|t| t.id == payload.id) {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("Type \"{}\" does not exist.", payload.id),
            [payload.id.clone()],
        );
    }
    let types: Vec<_> = base.types.iter().filter(|t| t.id != payload.id).cloned().collect();
    protocol::MutationOutcome::new(SemioKitDiff { types: Some(SemioKitTypeList { values: types }), ..Default::default() })
}
//#endregion 🔖️Diff

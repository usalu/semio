//! 🔺️ `rename-type` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::RenameType;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitTypeList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RenameType, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    let Some(existing) = base.types.iter().find(|t| t.id == payload.id) else {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("Type \"{}\" does not exist.", payload.id),
            [payload.id.clone()],
        );
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Type \"{}\" is already named \"{}\".", payload.id, payload.new_name));
    }
    let mut types = base.types.clone();
    if let Some(t) = types.iter_mut().find(|t| t.id == payload.id) {
        t.name = payload.new_name.clone();
    }
    protocol::MutationOutcome::new(SemioKitDiff { types: Some(SemioKitTypeList { values: types }), ..Default::default() })
}
//#endregion 🔖️Diff

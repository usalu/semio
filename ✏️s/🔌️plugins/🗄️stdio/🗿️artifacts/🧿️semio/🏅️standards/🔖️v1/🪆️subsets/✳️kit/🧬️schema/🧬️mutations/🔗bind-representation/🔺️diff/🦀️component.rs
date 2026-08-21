//! 🔺️ `bind-representation` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::BindRepresentation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitLinkList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &BindRepresentation, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if !base.types.iter().any(|t| t.id == payload.role) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Type \"{}\" does not exist.", payload.role), [payload.role.clone()]);
    }
    let new_link = store::ArtifactLink { target: payload.target.clone(), pin: payload.pin.clone(), role: payload.role.clone() };
    if base.representations.contains(&new_link) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Representation is already bound for \"{}\".", payload.role));
    }
    let mut representations = base.representations.clone();
    representations.push(new_link);
    protocol::MutationOutcome::new(SemioKitDiff { representations: Some(SemioKitLinkList { values: representations }), ..Default::default() })
}
//#endregion 🔖️Diff

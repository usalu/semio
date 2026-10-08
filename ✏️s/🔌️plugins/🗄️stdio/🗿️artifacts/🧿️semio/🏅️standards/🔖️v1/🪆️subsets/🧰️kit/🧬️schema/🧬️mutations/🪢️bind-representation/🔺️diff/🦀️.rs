//! 🔺️ Diff for `BindRepresentation`.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::standards::v1::subsets::kit::schema::diff::{SemioKitDiff};
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::BindRepresentation, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if !base.types.iter().any(|t| t.id == payload.role) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Type \"{}\" does not exist.", payload.role), [payload.role.clone()]);
    }
    let new_link = store::ArtifactLink { target: payload.target.clone(), pin: payload.pin.clone(), role: payload.role.clone() };
    if base.representations.contains(&new_link) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Representation is already bound for \"{}\".", payload.role));
    }
    protocol::MutationOutcome::new(SemioKitDiff { representations: Some(IndexedTripleDiff { added: vec![IndexAdded { index: payload.at.map_or(base.representations.len(), |at| at.min(base.representations.len())), item: new_link }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff

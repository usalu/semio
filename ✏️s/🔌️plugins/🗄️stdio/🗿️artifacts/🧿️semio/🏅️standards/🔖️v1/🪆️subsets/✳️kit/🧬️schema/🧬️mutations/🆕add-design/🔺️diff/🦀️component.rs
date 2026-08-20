//! 🔺️ `add-design` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::AddDesign;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDesignList, SemioKitDiff};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitDesign, SemioKitSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &AddDesign, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if base.designs.iter().any(|d| d.id == payload.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.duplicate-id",
            format!("A design with id \"{}\" already exists.", payload.id),
            [payload.id.clone()],
        );
    }
    let mut designs = base.designs.clone();
    designs.push(SemioKitDesign { id: payload.id.clone(), name: payload.name.clone(), pieces: Vec::new(), connections: Vec::new() });
    protocol::MutationOutcome::new(SemioKitDiff { designs: Some(SemioKitDesignList { values: designs }), ..Default::default() })
}
//#endregion 🔖️Diff

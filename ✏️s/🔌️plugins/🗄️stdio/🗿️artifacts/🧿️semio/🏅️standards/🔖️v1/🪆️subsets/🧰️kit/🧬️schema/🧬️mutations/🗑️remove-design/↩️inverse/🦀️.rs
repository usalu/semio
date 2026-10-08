//! ↩️ Inverse for `RemoveDesign`.

use crate::standards::v1::subsets::kit::schema::mutations::{add_design, edit_design, SemioKitMutation};
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveDesign, base: &SemioKitSnapshot) -> Result<Vec<SemioKitMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.designs.iter().position(|d| d.id == payload.id) {
        Some(at) => {
            let existing = &base.designs[at];
            vec![
                SemioKitMutation::EditDesign(edit_design::EditDesign { id: existing.id.clone(), pieces: existing.pieces.clone(), connections: existing.connections.clone() }),
                SemioKitMutation::AddDesign(add_design::AddDesign { id: existing.id.clone(), name: existing.name.clone(), at: Some(at) }),
            ]
        }
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

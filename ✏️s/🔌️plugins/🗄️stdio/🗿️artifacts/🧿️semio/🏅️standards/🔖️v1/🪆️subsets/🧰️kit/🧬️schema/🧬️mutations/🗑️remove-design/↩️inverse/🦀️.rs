//! ↩️ Inverse for `RemoveDesign`.

use crate::schema::mutations::{add_design, edit_design};
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveDesign, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::add_design::AddDesign> + From<crate::schema::mutations::edit_design::EditDesign>,
{
    Ok((|| {
    match base.designs.iter().position(|d| d.id == payload.id) {
        Some(at) => {
            let existing = &base.designs[at];
            vec![
                Op::from(edit_design::EditDesign { id: existing.id.clone(), pieces: existing.pieces.clone(), connections: existing.connections.clone() }),
                Op::from(add_design::AddDesign { id: existing.id.clone(), name: existing.name.clone(), at: Some(at) }),
            ]
        }
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

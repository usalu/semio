//! ↩️ Inverse for `DeleteShell`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, create_shell, delete_shell};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteShell, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    let Some(index) = base.shells.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let tail = &base.shells[index..];
    let mut undo: Vec<SemioBrepMutation> = tail
        .iter()
        .skip(1)
        .map(|x| SemioBrepMutation::DeleteShell(delete_shell::DeleteShell { id: x.id.clone() }))
        .collect();
    undo.extend(tail.iter().map(|x| SemioBrepMutation::CreateShell(create_shell::CreateShell { id: x.id.clone(), faces: x.faces.clone() })));
    undo
}
//#endregion 🔖️Inverse

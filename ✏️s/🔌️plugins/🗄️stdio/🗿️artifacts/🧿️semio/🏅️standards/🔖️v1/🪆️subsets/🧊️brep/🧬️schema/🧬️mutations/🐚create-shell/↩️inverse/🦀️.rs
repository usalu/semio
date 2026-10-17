//! ↩️ Inverse for `CreateShell`.

use crate::schema::mutations::delete_shell;
use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::CreateShell, _base: &SemioBrepSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::delete_shell::DeleteShell>,
{
    Ok((|| {
    vec![Op::from(delete_shell::DeleteShell { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse

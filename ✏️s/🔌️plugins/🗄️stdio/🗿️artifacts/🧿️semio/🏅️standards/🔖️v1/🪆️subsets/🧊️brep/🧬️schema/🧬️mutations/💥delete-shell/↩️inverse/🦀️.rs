//! ↩️ Inverse for `DeleteShell`.

use crate::schema::mutations::create_shell;
use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::DeleteShell, base: &SemioBrepSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::create_shell::CreateShell>,
{
    Ok((|| {
    let Some(index) = base.shells.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let x = &base.shells[index];
    vec![Op::from(create_shell::CreateShell { id: x.id.clone(), faces: x.faces.clone(), at: Some(index) })]

    })())
}
//#endregion 🔖️Inverse

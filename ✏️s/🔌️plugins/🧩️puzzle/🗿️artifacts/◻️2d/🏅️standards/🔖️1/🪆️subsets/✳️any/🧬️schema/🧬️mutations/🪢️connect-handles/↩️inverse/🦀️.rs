//! ↩️ Inverse for `ConnectHandles` — always a `disconnect-handles` of the id it created.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ConnectHandles, _base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::disconnect_handles::disconnect_handles(payload.id.clone())]

    })())
}
//#endregion 🔖️Inverse

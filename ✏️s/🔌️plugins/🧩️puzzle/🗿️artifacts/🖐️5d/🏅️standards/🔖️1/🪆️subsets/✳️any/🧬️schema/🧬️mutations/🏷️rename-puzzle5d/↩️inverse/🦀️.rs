//! ↩️ Inverse for `RenamePuzzle5d` — restores the BASE label.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &super::RenamePuzzle5d, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::RenamePuzzle5d>,
{
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::rename_puzzle5d(base.label.clone())]

    })())
}
//#endregion 🔖️Inverse

//! ↩️ Inverse for `ChangePartKind` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::ChangePartKind, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_part_kind::ChangePartKind>,
{
    Ok((|| {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_part_kind::change_part_kind(item.id.clone(), item.part_kind.clone())]

    })())
}
//#endregion 🔖️Inverse

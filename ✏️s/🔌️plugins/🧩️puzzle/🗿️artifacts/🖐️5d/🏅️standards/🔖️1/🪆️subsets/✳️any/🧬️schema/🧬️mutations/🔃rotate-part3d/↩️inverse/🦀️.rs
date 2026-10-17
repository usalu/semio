//! ↩️ Inverse for `RotatePart3d` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::RotatePart3d, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rotate_part_3d::RotatePart3d>,
{
    Ok((|| {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::rotate_part_3d::rotate_part_3d(item.id.clone(), item.part_3d.orientation)]

    })())
}
//#endregion 🔖️Inverse

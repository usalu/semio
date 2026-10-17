//! ↩️ Inverse for `EditPart3dLabel` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::EditPart3dLabel, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::EditPart3dLabel>,
{
    Ok((|| {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::edit_part_3d_label(item.id.clone(), item.part_3d.label.clone())]

    })())
}
//#endregion 🔖️Inverse

//! ↩️ Inverse for `ScalePart3d` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::ScalePart3d, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::scale_part_3d::ScalePart3d>,
{
    Ok((|| {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::scale_part_3d::scale_part_3d(item.id.clone(), item.part_3d.scale)]

    })())
}
//#endregion 🔖️Inverse

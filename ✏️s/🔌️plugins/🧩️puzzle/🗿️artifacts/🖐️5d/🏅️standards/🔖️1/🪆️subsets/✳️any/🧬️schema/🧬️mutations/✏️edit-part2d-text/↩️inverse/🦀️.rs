//! ↩️ Inverse for `EditPart2dText` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::EditPart2dText, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::EditPart2dText>,
{
    Ok((|| {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::edit_part_2d_text(item.id.clone(), item.part_2d.text.clone())]

    })())
}
//#endregion 🔖️Inverse

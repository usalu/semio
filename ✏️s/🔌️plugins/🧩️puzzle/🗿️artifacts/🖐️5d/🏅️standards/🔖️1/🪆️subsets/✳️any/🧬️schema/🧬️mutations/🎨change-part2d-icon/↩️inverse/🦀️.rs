//! ↩️ Inverse for `ChangePart2dIcon` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::ChangePart2dIcon, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::ChangePart2dIcon>,
{
    Ok((|| {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::change_part_2d_icon(item.id.clone(), item.part_2d.icon_kind.clone())]

    })())
}
//#endregion 🔖️Inverse

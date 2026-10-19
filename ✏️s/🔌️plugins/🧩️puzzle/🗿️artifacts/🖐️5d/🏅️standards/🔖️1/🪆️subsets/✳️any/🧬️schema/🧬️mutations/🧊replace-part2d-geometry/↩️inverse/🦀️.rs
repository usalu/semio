//! ↩️ Inverse for `ReplacePart2dGeometry` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::ReplacePart2dGeometry, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::ReplacePart2dGeometry>,
{
    Ok((|| {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::replace_part_2d_geometry(item.id.clone(), item.part_2d.shape.clone(), item.part_2d.radius, item.part_2d.width, item.part_2d.height)]

    })())
}
//#endregion 🔖️Inverse

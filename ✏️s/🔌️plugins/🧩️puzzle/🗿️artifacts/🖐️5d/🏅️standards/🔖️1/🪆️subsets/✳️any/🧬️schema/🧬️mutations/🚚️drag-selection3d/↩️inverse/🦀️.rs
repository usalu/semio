//! ↩️ Inverse for `DragSelection3d` — the absolute setters restoring every BASE pose field the transform changes (exact,
//! never a negated parameter that would accumulate float error). Nothing moved ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{move_part_2d, move_part_3d, move_target_volume, puzzle5d_selection, Puzzle5dMutation, PUZZLE5D_FLAT_TO_WORLD};
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DragSelection3d, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    let Ok(selection) = puzzle5d_selection(base, &payload.targets, true) else { return Ok(Vec::new()) };
    if !payload.offset.iter().all(|value| value.is_finite()) {
        return Ok(Vec::new());
    }
    let [dx, dy, dz] = payload.offset;
    let moved = |origin: [f64; 3]| [origin[0] + dx, origin[1] + dy, origin[2] + dz];
    let mut steps = Vec::new();
    for part in &selection.parts {
        if part.part_2d.x + dx / PUZZLE5D_FLAT_TO_WORLD != part.part_2d.x || part.part_2d.y - dy / PUZZLE5D_FLAT_TO_WORLD != part.part_2d.y {
            steps.push(move_part_2d(part.id.clone(), part.part_2d.x, part.part_2d.y));
        }
        if moved(part.part_3d.origin) != part.part_3d.origin {
            steps.push(move_part_3d(part.id.clone(), part.part_3d.origin));
        }
    }
    steps.extend(selection.volumes.iter().filter(|volume| moved(volume.origin) != volume.origin).map(|volume| move_target_volume(volume.id.clone(), volume.origin)));
    Ok(steps)
}
//#endregion 🔖️Inverse

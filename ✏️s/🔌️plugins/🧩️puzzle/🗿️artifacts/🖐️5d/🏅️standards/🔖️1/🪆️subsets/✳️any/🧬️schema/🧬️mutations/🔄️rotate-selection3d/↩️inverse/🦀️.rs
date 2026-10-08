//! ↩️ Inverse for `RotateSelection3d` — the absolute setters restoring every BASE pose field the transform changes (exact,
//! never a negated parameter that would accumulate float error). Nothing moved ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection, quat_from_axis_angle, quat_mul, rotate_part_3d, rotate_target_volume, Puzzle5dMutation};
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RotateSelection3d, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    let Ok(selection) = puzzle5d_selection(base, &payload.targets, true) else { return Ok(Vec::new()) };
    if !(payload.axis.iter().all(|value| value.is_finite()) && payload.angle.is_finite()) {
        return Ok(Vec::new());
    }
    let turn = quat_from_axis_angle(payload.axis[0], payload.axis[1], payload.axis[2], payload.angle);
    if payload.angle == 0.0 || turn == [0.0, 0.0, 0.0, 1.0] {
        return Ok(Vec::new());
    }
    let turned = |orientation: Option<[f64; 4]>| Some(quat_mul(turn, orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])));
    let parts = selection.parts.iter().filter(|part| turned(part.part_3d.orientation) != part.part_3d.orientation).map(|part| rotate_part_3d(part.id.clone(), part.part_3d.orientation));
    let volumes = selection.volumes.iter().filter(|volume| turned(volume.orientation) != volume.orientation).map(|volume| rotate_target_volume(volume.id.clone(), volume.orientation));
    Ok(parts.chain(volumes).collect())
}
//#endregion 🔖️Inverse

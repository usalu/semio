//! 🚫️ never a negated parameter that would accumulate float error). Nothing moved ⇒ `Vec::new()`.
//! ↩️ Inverse for `RotateSelection3d` — the absolute setters restoring every BASE pose field the transform changes (exact,
use crate::standards::v1::subsets::any::schema::diff::puzzle5d_selection;
use crate::standards::v1::subsets::any::schema::mutations::{quat_from_axis_angle, quat_mul, rotate_part_3d, rotate_target_volume};

use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::RotateSelection3d, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rotate_part_3d::RotatePart3d> + From<crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::RotateTargetVolume>,
{
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
    let mut inverse = parts.chain(volumes).collect::<Vec<_>>();
    inverse.reverse();
    Ok(inverse)
}
//#endregion 🔖️Inverse

//! ↩️ Inverse for `RotateSelection` — the absolute setters restoring every BASE orientation the turn
//! changes (exact, never a negated angle that would accumulate float error). Nothing turned ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{move_object, puzzle3d_selection, puzzle3d_selection_follow, quat_from_axis_angle, quat_mul, replace_attraction_geometry, rotate_object, rotate_target_volume, scale_object, Puzzle3dMutation, Puzzle3dPose, ReplaceAttractionGeometry, PUZZLE3D_IDENTITY_QUATERNION};
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RotateSelection, base: &Puzzle3dSnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    let Ok(selection) = puzzle3d_selection(base, &payload.targets) else { return Ok(Vec::new()) };
    if !(payload.axis.iter().all(|value| value.is_finite()) && payload.angle.is_finite()) {
        return Ok(Vec::new());
    }
    let turn = quat_from_axis_angle(payload.axis[0], payload.axis[1], payload.axis[2], payload.angle);
    if payload.angle == 0.0 || turn == PUZZLE3D_IDENTITY_QUATERNION {
        return Ok(Vec::new());
    }
    let turned = |orientation: Option<[f64; 4]>| Some(quat_mul(turn, orientation.unwrap_or(PUZZLE3D_IDENTITY_QUATERNION)));
    let solved = puzzle3d_selection_follow(base, &payload.targets, &selection, &|object| Puzzle3dPose { origin: object.origin, orientation: turned(object.orientation), scale: object.scale }, true);
    let mut steps = Vec::new();
    for (object, pose) in base.objects.iter().zip(&solved.poses) {
        let Some(pose) = pose else { continue };
        if pose.origin != object.origin {
            steps.push(move_object(object.id.clone(), object.origin));
        }
        if pose.orientation != object.orientation {
            steps.push(rotate_object(object.id.clone(), object.orientation));
        }
        if pose.scale != object.scale {
            steps.push(scale_object(object.id.clone(), object.scale));
        }
    }
    steps.extend(selection.volumes.iter().filter(|volume| turned(volume.orientation) != volume.orientation).map(|volume| rotate_target_volume(volume.id.clone(), volume.orientation)));
    for entry in &solved.attractions {
        let Some(attraction) = base.attractions.iter().find(|attraction| attraction.id == entry.id) else { continue };
        steps.push(replace_attraction_geometry(ReplaceAttractionGeometry { id: attraction.id.clone(), new_gap: attraction.gap, new_shift: attraction.shift, new_rise: attraction.rise, new_rotation: attraction.rotation, new_turn: attraction.turn, new_tilt: attraction.tilt, new_x: attraction.x, new_y: attraction.y }));
    }
    Ok(steps)
}
//#endregion 🔖️Inverse

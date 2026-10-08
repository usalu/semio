//! ↩️ Inverse for `DragSelection` — the absolute setters restoring every BASE origin the drag moves
//! (exact, never a negated offset that would accumulate float error). Nothing moved ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{move_object, move_target_volume, puzzle3d_selection, puzzle3d_selection_follow, replace_attraction_geometry, rotate_object, scale_object, Puzzle3dMutation, Puzzle3dPose, ReplaceAttractionGeometry};
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DragSelection, base: &Puzzle3dSnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    let Ok(selection) = puzzle3d_selection(base, &payload.targets) else { return Ok(Vec::new()) };
    if !payload.offset.iter().all(|value| value.is_finite()) || payload.offset == [0.0; 3] {
        return Ok(Vec::new());
    }
    let [dx, dy, dz] = payload.offset;
    let moved = |origin: [f64; 3]| [origin[0] + dx, origin[1] + dy, origin[2] + dz];
    let solved = puzzle3d_selection_follow(base, &payload.targets, &selection, &|object| Puzzle3dPose { origin: moved(object.origin), orientation: object.orientation, scale: object.scale }, true);
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
    steps.extend(selection.volumes.iter().filter(|volume| moved(volume.origin) != volume.origin).map(|volume| move_target_volume(volume.id.clone(), volume.origin)));
    for entry in &solved.attractions {
        let Some(attraction) = base.attractions.iter().find(|attraction| attraction.id == entry.id) else { continue };
        steps.push(replace_attraction_geometry(ReplaceAttractionGeometry { id: attraction.id.clone(), new_gap: attraction.gap, new_shift: attraction.shift, new_rise: attraction.rise, new_rotation: attraction.rotation, new_turn: attraction.turn, new_tilt: attraction.tilt, new_x: attraction.x, new_y: attraction.y }));
    }
    Ok(steps)
}
//#endregion 🔖️Inverse

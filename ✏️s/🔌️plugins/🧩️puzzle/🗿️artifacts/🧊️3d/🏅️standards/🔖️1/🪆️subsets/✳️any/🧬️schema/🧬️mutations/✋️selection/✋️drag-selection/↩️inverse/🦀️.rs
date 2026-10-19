//! (exact, never a negated offset that would accumulate float error). Nothing moved ⇒ `Vec::new()`.
//! ↩️ Inverse for `DragSelection` — the absolute setters restoring every BASE origin the drag moves
use super::mutation::DragSelection;
use {semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::MoveObject, semio_s_artifact_puzzle_3d_mutations_volume::move_target_volume::mutation::MoveTargetVolume, semio_s_artifact_puzzle_3d_mutations_pose::rotate_object::mutation::RotateObject, semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::ScaleObject};
use {semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::move_object, semio_s_artifact_puzzle_3d_mutations_volume::move_target_volume::mutation::move_target_volume, crate::puzzle3d_selection, crate::puzzle3d_selection_follow, semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::replace_attraction_geometry, semio_s_artifact_puzzle_3d_mutations_pose::rotate_object::mutation::rotate_object, semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::scale_object, semio_s_artifact_puzzle_3d_model::Puzzle3dPose, semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::ReplaceAttractionGeometry};
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<DragSelection> + From<MoveObject> + From<MoveTargetVolume> + From<ReplaceAttractionGeometry> + From<RotateObject> + From<ScaleObject>>(payload: &super::mutation::DragSelection, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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

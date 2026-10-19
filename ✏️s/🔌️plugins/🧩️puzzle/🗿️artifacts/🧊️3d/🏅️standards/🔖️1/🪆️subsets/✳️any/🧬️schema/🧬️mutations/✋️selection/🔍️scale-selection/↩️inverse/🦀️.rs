//! 🎯️ (exact, never a reciprocal factor that would accumulate float error). Nothing scaled ⇒ `Vec::new()`.
//! ↩️ Inverse for `ScaleSelection` — the absolute setters restoring every BASE scale the scaling changes
use super::mutation::ScaleSelection;
use {semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::ScaleObject, semio_s_artifact_puzzle_3d_mutations_volume::scale_target_volume::mutation::ScaleTargetVolume};
use {semio_s_artifact_puzzle_3d_model::puzzle3d_scaled, crate::puzzle3d_selection, semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::scale_object, semio_s_artifact_puzzle_3d_mutations_volume::scale_target_volume::mutation::scale_target_volume};
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ScaleSelection> + From<ScaleObject> + From<ScaleTargetVolume>>(payload: &super::mutation::ScaleSelection, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let Ok(selection) = puzzle3d_selection(base, &payload.targets) else { return Ok(Vec::new()) };
    if !payload.factors.iter().all(|value| value.is_finite() && *value > 0.0) {
        return Ok(Vec::new());
    }
    let factors = payload.factors;
    let objects = selection.objects.iter().filter(|object| Some(puzzle3d_scaled(object.scale, factors)) != object.scale).map(|object| scale_object(object.id.clone(), object.scale));
    let volumes = selection.volumes.iter().filter(|volume| Some(puzzle3d_scaled(volume.scale, factors)) != volume.scale).map(|volume| scale_target_volume(volume.id.clone(), volume.scale));
    Ok(objects.chain(volumes).collect())
}
//#endregion 🔖️Inverse

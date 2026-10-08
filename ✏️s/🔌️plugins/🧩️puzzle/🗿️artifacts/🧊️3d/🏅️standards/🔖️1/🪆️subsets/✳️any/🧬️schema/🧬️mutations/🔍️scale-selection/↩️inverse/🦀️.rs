//! ↩️ Inverse for `ScaleSelection` — the absolute setters restoring every BASE scale the scaling changes
//! (exact, never a reciprocal factor that would accumulate float error). Nothing scaled ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{puzzle3d_scaled, puzzle3d_selection, scale_object, scale_target_volume, Puzzle3dMutation};
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ScaleSelection, base: &Puzzle3dSnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
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

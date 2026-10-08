//! ↩️ Inverse for `ScaleSelection3d` — the absolute setters restoring every BASE pose field the transform changes (exact,
//! never a negated parameter that would accumulate float error). Nothing moved ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_scaled, puzzle5d_selection, scale_part_3d, scale_target_volume, Puzzle5dMutation};
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ScaleSelection3d, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    let Ok(selection) = puzzle5d_selection(base, &payload.targets, true) else { return Ok(Vec::new()) };
    if !payload.factors.iter().all(|value| value.is_finite() && *value > 0.0) || payload.factors == [1.0; 3] {
        return Ok(Vec::new());
    }
    let factors = payload.factors;
    let parts = selection.parts.iter().filter(|part| Some(puzzle5d_scaled(part.part_3d.scale, factors)) != part.part_3d.scale).map(|part| scale_part_3d(part.id.clone(), part.part_3d.scale));
    let volumes = selection.volumes.iter().filter(|volume| Some(puzzle5d_scaled(volume.scale, factors)) != volume.scale).map(|volume| scale_target_volume(volume.id.clone(), volume.scale));
    Ok(parts.chain(volumes).collect())
}
//#endregion 🔖️Inverse

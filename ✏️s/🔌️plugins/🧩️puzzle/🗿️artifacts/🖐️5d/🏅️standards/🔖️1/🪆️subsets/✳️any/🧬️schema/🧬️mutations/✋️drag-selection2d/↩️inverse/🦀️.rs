//! 🚫️ never a negated parameter that would accumulate float error). Nothing moved ⇒ `Vec::new()`.
//! ↩️ Inverse for `DragSelection2d` — the absolute setters restoring every BASE pose field the transform changes (exact,
use crate::standards::v1::subsets::any::schema::mutations::move_part_2d;

use crate::standards::v1::subsets::any::schema::diff::{puzzle5d_selection};
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::DragSelection2d, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::move_part_2d::MovePart2d>,
{
    let Ok(selection) = puzzle5d_selection(base, &payload.targets, false) else { return Ok(Vec::new()) };
    if !(payload.dx.is_finite() && payload.dy.is_finite()) {
        return Ok(Vec::new());
    }
    let (dx, dy) = (payload.dx, payload.dy);
    let mut inverse = selection.parts.iter().filter(|part| part.part_2d.x + dx != part.part_2d.x || part.part_2d.y + dy != part.part_2d.y).map(|part| move_part_2d(part.id.clone(), part.part_2d.x, part.part_2d.y)).collect::<Vec<_>>();
    inverse.reverse();
    Ok(inverse)
}
//#endregion 🔖️Inverse

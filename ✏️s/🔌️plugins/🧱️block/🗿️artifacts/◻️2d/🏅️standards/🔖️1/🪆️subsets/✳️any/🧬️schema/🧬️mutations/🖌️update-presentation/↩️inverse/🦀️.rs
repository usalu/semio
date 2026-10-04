//! ↩️ Inverse for `UpdatePresentation`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdatePresentation, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::update_presentation::update_presentation(
        base.presentation.shape.clone(),
        base.presentation.radius,
        base.presentation.width,
        base.presentation.height,
        base.presentation.color.clone(),
        base.presentation.icon_kind.clone(),
    )]

    })())
}
//#endregion 🔖️Inverse

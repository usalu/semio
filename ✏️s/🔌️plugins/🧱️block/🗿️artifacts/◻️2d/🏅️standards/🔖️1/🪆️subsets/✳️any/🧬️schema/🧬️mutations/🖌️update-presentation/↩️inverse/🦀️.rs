//! ↩️ Inverse for `UpdatePresentation`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::update_presentation::UpdatePresentation, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_presentation::UpdatePresentation> {
    Ok((|| {
    vec![crate::update_presentation::update_presentation(
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

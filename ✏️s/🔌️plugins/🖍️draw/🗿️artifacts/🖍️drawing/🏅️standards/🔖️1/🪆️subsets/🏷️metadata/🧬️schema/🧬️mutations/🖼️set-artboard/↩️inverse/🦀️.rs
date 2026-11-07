//! ↩️ Inverse for `SetArtboard` — reconstructs the undo from BASE state, never from post-state.
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<super::mutation::SetArtboard>>(_payload: &super::mutation::SetArtboard, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(vec![super::mutation::set_artboard(super::mutation::copy(&base.artboard))])
}
//#endregion 🔖️Inverse

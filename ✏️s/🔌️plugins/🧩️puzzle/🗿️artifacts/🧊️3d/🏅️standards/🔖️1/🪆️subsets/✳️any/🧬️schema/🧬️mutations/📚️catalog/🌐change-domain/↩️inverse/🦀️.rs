//! ↩️ Inverse for `ChangeDomain` — restores the BASE domain.
use super::mutation::ChangeDomain;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeDomain>>(_payload: &super::mutation::ChangeDomain, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::change_domain::mutation::change_domain(base.domain.clone())]

    })())
}
//#endregion 🔖️Inverse

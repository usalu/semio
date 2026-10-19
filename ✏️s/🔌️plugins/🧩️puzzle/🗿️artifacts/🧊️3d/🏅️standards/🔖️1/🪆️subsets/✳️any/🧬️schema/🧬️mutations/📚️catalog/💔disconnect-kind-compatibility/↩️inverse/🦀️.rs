//! 📸️ the captured BASE row. Missing target ⇒ `Vec::new()`.
//! ↩️ Inverse for `DisconnectKindCompatibility` — reconstructs a `connect-kind-compatibility` of
use super::mutation::DisconnectKindCompatibility;
use crate::connect_kind_compatibility::mutation::ConnectKindCompatibility;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<DisconnectKindCompatibility> + From<ConnectKindCompatibility>>(payload: &super::mutation::DisconnectKindCompatibility, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(at) = base.meta.kind_compatibility.iter().position(|row| row.source == payload.source && row.target == payload.target) else {
        return Vec::new();
    };
    let row = &base.meta.kind_compatibility[at];
    vec![crate::connect_kind_compatibility::mutation::connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity, Some(at))]

    })())
}
//#endregion 🔖️Inverse

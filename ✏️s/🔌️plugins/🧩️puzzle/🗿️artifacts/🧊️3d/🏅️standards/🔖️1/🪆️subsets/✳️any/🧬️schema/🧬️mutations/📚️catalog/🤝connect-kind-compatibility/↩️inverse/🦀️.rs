//! pair it created.
//! ↩️ Inverse for `ConnectKindCompatibility` — always a `disconnect-kind-compatibility` of the
use super::mutation::ConnectKindCompatibility;
use crate::disconnect_kind_compatibility::mutation::DisconnectKindCompatibility;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ConnectKindCompatibility> + From<DisconnectKindCompatibility>>(payload: &super::mutation::ConnectKindCompatibility, _base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::disconnect_kind_compatibility::mutation::disconnect_kind_compatibility(payload.source.clone(), payload.target.clone())]

    })())
}
//#endregion 🔖️Inverse

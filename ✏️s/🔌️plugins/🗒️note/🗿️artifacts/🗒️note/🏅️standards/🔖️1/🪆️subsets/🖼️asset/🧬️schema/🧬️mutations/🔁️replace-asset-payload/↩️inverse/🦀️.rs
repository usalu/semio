//! ↩️ Inverse for `ReplaceAssetPayload`.
use super::ReplaceAssetPayload;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceAssetPayload, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.assets.get(&payload.key) {
        Some(prior) => vec![NoteMutation::ReplaceAssetPayload(ReplaceAssetPayload { key: payload.key.clone(), new_asset: prior.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

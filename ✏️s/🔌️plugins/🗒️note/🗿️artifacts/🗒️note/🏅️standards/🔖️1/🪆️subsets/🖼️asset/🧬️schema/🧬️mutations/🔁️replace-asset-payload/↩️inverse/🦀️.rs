//! ↩️ Inverse for `ReplaceAssetPayload`.
use super::ReplaceAssetPayload;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &ReplaceAssetPayload, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::asset::schema::mutations::replace_asset_payload::ReplaceAssetPayload>,
{
    Ok((|| {
    match base.assets.get(&payload.key) {
        Some(prior) => vec![Op::from(ReplaceAssetPayload { key: payload.key.clone(), new_asset: prior.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

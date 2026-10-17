//! ↩️ Inverse for `CreateAsset`.
use super::CreateAsset;
use crate::standards::v1::subsets::asset::schema::mutations::delete_asset::DeleteAsset;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &CreateAsset, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::asset::schema::mutations::delete_asset::DeleteAsset>,
{
    Ok(match base.assets.contains_key(&payload.key) {
        true => Vec::new(),
        false => vec![Op::from(DeleteAsset { key: payload.key.clone() })],
    })
}
//#endregion 🔖️Inverse

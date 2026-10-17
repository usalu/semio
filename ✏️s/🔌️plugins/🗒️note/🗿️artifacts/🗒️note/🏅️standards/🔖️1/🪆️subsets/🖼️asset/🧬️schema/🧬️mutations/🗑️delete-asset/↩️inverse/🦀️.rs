//! ↩️ Inverse for `DeleteAsset`.
use super::DeleteAsset;
use crate::standards::v1::subsets::asset::schema::mutations::create_asset::CreateAsset;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &DeleteAsset, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::asset::schema::mutations::create_asset::CreateAsset>,
{
    Ok((|| {
    match base.assets.get(&payload.key) {
        Some(prior) => vec![Op::from(CreateAsset { key: payload.key.clone(), asset: prior.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse

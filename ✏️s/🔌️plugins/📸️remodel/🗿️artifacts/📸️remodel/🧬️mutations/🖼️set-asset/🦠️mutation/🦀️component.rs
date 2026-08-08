//! 🖼️ Remodel mutation — `SetAsset` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, key: &str, value: &Option<crate::artifacts::remodel::ImageAsset>) {
    match value { Some(value) => { next.assets.insert(key.to_string(), value.clone()); } None => { next.assets.remove(key); } }
}
//#endregion 🔖️Mutation

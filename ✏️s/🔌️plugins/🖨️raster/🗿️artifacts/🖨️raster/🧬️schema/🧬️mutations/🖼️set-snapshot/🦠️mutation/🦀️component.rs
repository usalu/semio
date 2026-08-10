//! 🖼️ Raster mutation — `SetSnapshot` payload + builder + apply.
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖼️ `SetSnapshot` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: RasterSnapshot,
}

pub fn set_snapshot(snapshot: RasterSnapshot) -> RasterMutation {
    RasterMutation::SetSnapshot { snapshot }
}

pub fn apply(doc: &mut RasterSnapshot, snapshot: &RasterSnapshot) {
    *doc = snapshot.clone();
}
//#endregion 🔖️Mutation

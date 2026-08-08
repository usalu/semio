//! 🖼️ Draw mutation — `SetSnapshot` payload + builder + apply.
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};


//#region 🔖️Mutation
/// 🖼️ `SetSnapshot` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: DrawSnapshot,
}

pub fn set_snapshot(snapshot: DrawSnapshot) -> DrawMutation {
    DrawMutation::SetSnapshot { snapshot }
}

pub fn apply(doc: &mut DrawSnapshot, snapshot: &DrawSnapshot) {
    *doc = snapshot.clone();
}
//#endregion 🔖️Mutation

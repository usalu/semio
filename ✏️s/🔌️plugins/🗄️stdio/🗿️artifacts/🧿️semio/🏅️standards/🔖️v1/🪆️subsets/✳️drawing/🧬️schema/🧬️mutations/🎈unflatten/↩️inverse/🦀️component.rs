//! ↩️ `unflatten` — undo is `flatten` at the same address: `base` (pre-unflatten state) is
//! deterministically the flattened form `flatten` itself would have produced (this pair is only
//! ever ridden as `flatten` then `unflatten`, never called standalone against an arbitrary
//! subtree), so re-running `flatten` against `base` reconstructs exactly what `unflatten` just
//! replaced.

use super::mutation::UnflattenNode;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{flatten, SemioDrawingMutation};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &UnflattenNode, _base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    vec![SemioDrawingMutation::Flatten(flatten::mutation::FlattenNode { at: payload.at.clone() })]
}
//#endregion 🔖️Inverse

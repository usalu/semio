//! 🔺️ Diff fragment yielded by `SetLayerVisible`.
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `SetLayerVisible` mutation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetLayerVisibleDiff {
    pub mutation: Option<DrawMutation>,
}

impl SetLayerVisibleDiff {
    pub fn from_mutation(mutation: DrawMutation) -> Self {
        Self { mutation: Some(mutation) }
    }
}

impl MutationDiff<DrawSnapshot> for SetLayerVisibleDiff {
    fn apply(&self, projection: &DrawSnapshot) -> DrawSnapshot {
        match &self.mutation {
            Some(mutation) => {
                let diff = <DrawMutation as protocol::Mutation<DrawSnapshot>>::diff(mutation, projection);
                <DrawDiff as MutationDiff<DrawSnapshot>>::apply(&diff, projection)
            }
            None => projection.clone(),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.mutation.is_some() {
            *self = other;
        }
    }
}
//#endregion 🔖️Diff

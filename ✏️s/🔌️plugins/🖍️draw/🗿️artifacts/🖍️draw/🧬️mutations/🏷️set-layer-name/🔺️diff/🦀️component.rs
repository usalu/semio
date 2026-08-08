//! 🔺️ Diff fragment yielded by `SetLayerName`.
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawDocument;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `SetLayerName` mutation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetLayerNameDiff {
    pub mutation: Option<DrawMutation>,
}

impl SetLayerNameDiff {
    pub fn from_mutation(mutation: DrawMutation) -> Self {
        Self { mutation: Some(mutation) }
    }
}

impl MutationDiff<DrawDocument> for SetLayerNameDiff {
    fn apply(&self, projection: &DrawDocument) -> DrawDocument {
        match &self.mutation {
            Some(mutation) => {
                let diff = <DrawMutation as protocol::Mutation<DrawDocument>>::diff(mutation, projection);
                <DrawDiff as MutationDiff<DrawDocument>>::apply(&diff, projection)
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

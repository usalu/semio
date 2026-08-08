//! 🔺️ Diff fragment yielded by `SetIngestParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `SetIngestParams` mutation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetIngestParamsDiff {
    pub mutation: Option<RemodelMutation>,
}

impl SetIngestParamsDiff {
    pub fn from_mutation(mutation: RemodelMutation) -> Self {
        Self { mutation: Some(mutation) }
    }
}

impl MutationDiff<RemodelProjection> for SetIngestParamsDiff {
    fn apply(&self, projection: &RemodelProjection) -> RemodelProjection {
        match &self.mutation {
            Some(m) => {
                let mut next = projection.clone();
                crate::artifacts::remodel::mutations::apply_remodel_mutation_in_place(&mut next, m);
                next
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

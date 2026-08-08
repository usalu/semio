//! 🔺️ Diff fragment yielded by `SetGeoParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `SetGeoParams` mutation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetGeoParamsDiff {
    pub mutation: Option<RemodelMutation>,
}

impl SetGeoParamsDiff {
    pub fn from_mutation(mutation: RemodelMutation) -> Self {
        Self { mutation: Some(mutation) }
    }
}

impl MutationDiff<RemodelProjection> for SetGeoParamsDiff {
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

//! 🚽️ Energy model mutation — `CreateWaterSystem`: Adds one cold-water use system: the fixtures it serves, their combined peak flow, and the draw schedule the profile is read from.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚽️ `create-water-system` payload. Adds one cold-water use system: the fixtures it serves, their combined peak flow, and the draw schedule the profile is read from.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-water-system")]
pub struct CreateWaterSystem {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub fixture_count: u32,
    pub peak_flow_l_s: f64,
    pub schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_water_system(index: u32, id: crate::model::EntityId, fixture_count: u32, peak_flow_l_s: f64, schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::CreateWaterSystem(CreateWaterSystem { index, id, fixture_count, peak_flow_l_s, schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateWaterSystem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "water-system", kind: "create-water-system", record: "CreatedWaterSystem" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Water System {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

//! 💡️ Energy model mutation — `CreateLightingGain`: Adds one lighting gain to a zone: the installed power density, its schedule and the radiant/visible/return-air split that decides how much of it reaches the zone air directly.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💡️ `create-lighting-gain` payload. Adds one lighting gain to a zone: the installed power density, its schedule and the radiant/visible/return-air split that decides how much of it reaches the zone air directly.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-lighting-gain")]
pub struct CreateLightingGain {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub schedule_id: crate::model::ScheduleId,
    pub watts_per_area: f64,
    pub radiant_fraction: f64,
    pub visible_fraction: f64,
    pub return_air_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_lighting_gain(index: u32, id: crate::model::EntityId, zone_id: crate::model::EntityId, schedule_id: crate::model::ScheduleId, watts_per_area: f64, radiant_fraction: f64, visible_fraction: f64, return_air_fraction: f64) -> EnergyModelMutation {
    EnergyModelMutation::CreateLightingGain(CreateLightingGain { index, id, zone_id, schedule_id, watts_per_area, radiant_fraction, visible_fraction, return_air_fraction })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateLightingGain {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "lighting-gain", kind: "create-lighting-gain", record: "CreatedLightingGain" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Lighting Gain {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

//! 👤️ Energy model mutation — `CreatePeopleGain`: Adds one occupancy gain to a zone: the occupant density, the occupancy schedule, the metabolic activity schedule and the sensible/latent/radiant split the zone heat balance needs.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 👤️ `create-people-gain` payload. Adds one occupancy gain to a zone: the occupant density, the occupancy schedule, the metabolic activity schedule and the sensible/latent/radiant split the zone heat balance needs.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-people-gain")]
pub struct CreatePeopleGain {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub schedule_id: crate::model::ScheduleId,
    pub activity_schedule_id: crate::model::ScheduleId,
    pub people_per_area: f64,
    pub sensible_fraction: f64,
    pub latent_fraction: f64,
    pub radiant_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_people_gain(
    index: u32,
    id: crate::model::EntityId,
    zone_id: crate::model::EntityId,
    schedule_id: crate::model::ScheduleId,
    activity_schedule_id: crate::model::ScheduleId,
    people_per_area: f64,
    sensible_fraction: f64,
    latent_fraction: f64,
    radiant_fraction: f64,
) -> EnergyModelMutation {
    EnergyModelMutation::CreatePeopleGain(CreatePeopleGain { index, id, zone_id, schedule_id, activity_schedule_id, people_per_area, sensible_fraction, latent_fraction, radiant_fraction })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreatePeopleGain {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "people-gain", kind: "create-people-gain", record: "CreatedPeopleGain" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create People Gain {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

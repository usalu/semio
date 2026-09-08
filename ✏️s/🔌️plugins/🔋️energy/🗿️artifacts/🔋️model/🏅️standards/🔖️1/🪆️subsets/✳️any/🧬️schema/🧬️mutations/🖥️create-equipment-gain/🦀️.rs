//! 🖥️ Energy model mutation — `CreateEquipmentGain`: Adds one electric equipment gain to a zone — ANSI/ASHRAE 140 §5.2's 200 W internal load is exactly this entity at 60 % radiative.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🖥️ `create-equipment-gain` payload. Adds one electric equipment gain to a zone — ANSI/ASHRAE 140 §5.2's 200 W internal load is exactly this entity at 60 % radiative.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-equipment-gain")]
pub struct CreateEquipmentGain {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub schedule_id: crate::model::ScheduleId,
    pub watts_per_area: f64,
    pub radiant_fraction: f64,
    pub latent_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_equipment_gain(index: u32, id: crate::model::EntityId, zone_id: crate::model::EntityId, schedule_id: crate::model::ScheduleId, watts_per_area: f64, radiant_fraction: f64, latent_fraction: f64) -> EnergyModelMutation {
    EnergyModelMutation::CreateEquipmentGain(CreateEquipmentGain { index, id, zone_id, schedule_id, watts_per_area, radiant_fraction, latent_fraction })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateEquipmentGain {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "equipment-gain", kind: "create-equipment-gain", record: "CreatedEquipmentGain" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Equipment Gain {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

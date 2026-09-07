//! 🧍️ Energy model mutation — `ChangeIdealLoadsSystemOutdoorAirPerPerson`: Sets the ventilation rate the system draws per occupant, in m³/s.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧍️ `change-ideal-loads-system-outdoor-air-per-person` payload. Sets the ventilation rate the system draws per occupant, in m³/s.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-outdoor-air-per-person")]
pub struct ChangeIdealLoadsSystemOutdoorAirPerPerson {
    pub id: crate::model::EntityId,
    pub new_outdoor_air_per_person_m3_s: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_ideal_loads_system_outdoor_air_per_person(id: crate::model::EntityId, new_outdoor_air_per_person_m3_s: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeIdealLoadsSystemOutdoorAirPerPerson(ChangeIdealLoadsSystemOutdoorAirPerPerson { id, new_outdoor_air_per_person_m3_s })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeIdealLoadsSystemOutdoorAirPerPerson {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ideal-loads-system", kind: "change-ideal-loads-system-outdoor-air-per-person", record: "ChangedIdealLoadsSystemOutdoorAirPerPerson" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change ideal loads system {} outdoor air rate per person to {:?}", self.id.0, self.new_outdoor_air_per_person_m3_s)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

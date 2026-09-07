//! 🔵️ Energy model mutation — `ChangeIdealLoadsSystemMinCoolingSupplyAirTemp`: Sets the coldest air the ideal-loads system may deliver, in °C.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔵️ `change-ideal-loads-system-min-cooling-supply-air-temp` payload. Sets the coldest air the ideal-loads system may deliver, in °C.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-min-cooling-supply-air-temp")]
pub struct ChangeIdealLoadsSystemMinCoolingSupplyAirTemp {
    pub id: crate::model::EntityId,
    pub new_min_cooling_supply_air_temp_c: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_ideal_loads_system_min_cooling_supply_air_temp(id: crate::model::EntityId, new_min_cooling_supply_air_temp_c: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeIdealLoadsSystemMinCoolingSupplyAirTemp(ChangeIdealLoadsSystemMinCoolingSupplyAirTemp { id, new_min_cooling_supply_air_temp_c })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeIdealLoadsSystemMinCoolingSupplyAirTemp {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ideal-loads-system", kind: "change-ideal-loads-system-min-cooling-supply-air-temp", record: "ChangedIdealLoadsSystemMinCoolingSupplyAirTemp" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change ideal loads system {} minimum cooling supply air temperature to {:?}", self.id.0, self.new_min_cooling_supply_air_temp_c)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

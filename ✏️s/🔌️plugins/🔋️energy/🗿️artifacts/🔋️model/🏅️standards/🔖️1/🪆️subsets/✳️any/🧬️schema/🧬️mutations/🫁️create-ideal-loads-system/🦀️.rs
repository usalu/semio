//! 🫁️ Energy model mutation — `CreateIdealLoadsSystem`: Creates the ideal-loads air system that serves one zone. Both capacity limits are optional — `present` false is the autosized reading, and then the value has to be zero.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🫁️ `create-ideal-loads-system` payload. Creates the ideal-loads air system that serves one zone. Both capacity limits are optional — `present` false is the autosized reading, and then the value has to be zero.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-ideal-loads-system")]
pub struct CreateIdealLoadsSystem {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub max_heating_supply_air_temp_c: f64,
    pub min_cooling_supply_air_temp_c: f64,
    pub max_heating_capacity_present: bool,
    pub max_heating_capacity_w: f64,
    pub max_cooling_capacity_present: bool,
    pub max_cooling_capacity_w: f64,
    pub outdoor_air_per_person_m3_s: f64,
    pub outdoor_air_per_area_m3_s_m2: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_ideal_loads_system(
    id: crate::model::EntityId,
    zone_id: crate::model::EntityId,
    max_heating_supply_air_temp_c: f64,
    min_cooling_supply_air_temp_c: f64,
    max_heating_capacity_present: bool,
    max_heating_capacity_w: f64,
    max_cooling_capacity_present: bool,
    max_cooling_capacity_w: f64,
    outdoor_air_per_person_m3_s: f64,
    outdoor_air_per_area_m3_s_m2: f64,
) -> EnergyModelMutation {
    EnergyModelMutation::CreateIdealLoadsSystem(CreateIdealLoadsSystem {
        id,
        zone_id,
        max_heating_supply_air_temp_c,
        min_cooling_supply_air_temp_c,
        max_heating_capacity_present,
        max_heating_capacity_w,
        max_cooling_capacity_present,
        max_cooling_capacity_w,
        outdoor_air_per_person_m3_s,
        outdoor_air_per_area_m3_s_m2,
    })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateIdealLoadsSystem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "ideal-loads-system", kind: "create-ideal-loads-system", record: "CreatedIdealLoadsSystem" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create ideal loads system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

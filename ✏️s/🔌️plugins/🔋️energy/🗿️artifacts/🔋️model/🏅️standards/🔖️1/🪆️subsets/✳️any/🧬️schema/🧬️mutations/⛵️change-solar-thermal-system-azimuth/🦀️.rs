//! ⛵️ Energy model mutation — `ChangeSolarThermalSystemAzimuth`: Sets azimuth on one solar thermal system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛵️ `change-solar-thermal-system-azimuth` payload. Sets azimuth on one solar thermal system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-solar-thermal-system-azimuth")]
pub struct ChangeSolarThermalSystemAzimuth {
    pub id: crate::model::EntityId,
    pub new_azimuth_deg: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_solar_thermal_system_azimuth(id: crate::model::EntityId, new_azimuth_deg: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSolarThermalSystemAzimuth(ChangeSolarThermalSystemAzimuth { id, new_azimuth_deg })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSolarThermalSystemAzimuth {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "solar-thermal-system", kind: "change-solar-thermal-system-azimuth", record: "ChangedSolarThermalSystemAzimuth" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Solar Thermal System Azimuth of solar thermal system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

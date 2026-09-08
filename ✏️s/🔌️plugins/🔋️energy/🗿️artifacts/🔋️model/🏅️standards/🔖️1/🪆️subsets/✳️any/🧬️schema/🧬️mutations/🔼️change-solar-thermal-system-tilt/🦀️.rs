//! 🔼️ Energy model mutation — `ChangeSolarThermalSystemTilt`: Sets tilt on one solar thermal system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔼️ `change-solar-thermal-system-tilt` payload. Sets tilt on one solar thermal system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-solar-thermal-system-tilt")]
pub struct ChangeSolarThermalSystemTilt {
    pub id: crate::model::EntityId,
    pub new_tilt_deg: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_solar_thermal_system_tilt(id: crate::model::EntityId, new_tilt_deg: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSolarThermalSystemTilt(ChangeSolarThermalSystemTilt { id, new_tilt_deg })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSolarThermalSystemTilt {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "solar-thermal-system", kind: "change-solar-thermal-system-tilt", record: "ChangedSolarThermalSystemTilt" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Solar Thermal System Tilt of solar thermal system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

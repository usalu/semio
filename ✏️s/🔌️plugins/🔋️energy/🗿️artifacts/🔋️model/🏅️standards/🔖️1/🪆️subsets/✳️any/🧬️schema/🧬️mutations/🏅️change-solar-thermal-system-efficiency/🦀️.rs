//! 🏅️ Energy model mutation — `ChangeSolarThermalSystemEfficiency`: Sets collector efficiency on one solar thermal system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏅️ `change-solar-thermal-system-efficiency` payload. Sets collector efficiency on one solar thermal system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-solar-thermal-system-efficiency")]
pub struct ChangeSolarThermalSystemEfficiency {
    pub id: crate::model::EntityId,
    pub new_efficiency: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_solar_thermal_system_efficiency(id: crate::model::EntityId, new_efficiency: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSolarThermalSystemEfficiency(ChangeSolarThermalSystemEfficiency { id, new_efficiency })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSolarThermalSystemEfficiency {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "solar-thermal-system", kind: "change-solar-thermal-system-efficiency", record: "ChangedSolarThermalSystemEfficiency" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Solar Thermal System Efficiency of solar thermal system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

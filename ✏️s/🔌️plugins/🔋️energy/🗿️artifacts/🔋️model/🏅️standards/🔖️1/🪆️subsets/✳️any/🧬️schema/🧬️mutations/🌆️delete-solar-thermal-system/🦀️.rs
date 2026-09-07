//! 🌆️ Energy model mutation — `DeleteSolarThermalSystem`: Removes one solar thermal collector loop. Nothing in the document references one, so this never refuses for use.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌆️ `delete-solar-thermal-system` payload. Removes one solar thermal collector loop. Nothing in the document references one, so this never refuses for use.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-solar-thermal-system")]
pub struct DeleteSolarThermalSystem {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_solar_thermal_system(id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteSolarThermalSystem(DeleteSolarThermalSystem { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteSolarThermalSystem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "solar-thermal-system", kind: "delete-solar-thermal-system", record: "DeletedSolarThermalSystem" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete Solar Thermal System {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

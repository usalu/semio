//! 🍵️ Energy model mutation — `ChangeShwSystemHeaterCapacity`: Sets heater capacity (W) on one service hot water system, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🍵️ `change-shw-system-heater-capacity` payload. Sets heater capacity (W) on one service hot water system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-shw-system-heater-capacity")]
pub struct ChangeShwSystemHeaterCapacity {
    pub id: crate::model::EntityId,
    pub new_heater_capacity_w: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_shw_system_heater_capacity(id: crate::model::EntityId, new_heater_capacity_w: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeShwSystemHeaterCapacity(ChangeShwSystemHeaterCapacity { id, new_heater_capacity_w })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeShwSystemHeaterCapacity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "service-hot-water-system", kind: "change-shw-system-heater-capacity", record: "ChangedShwSystemHeaterCapacity" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Shw System Heater Capacity of service hot water system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

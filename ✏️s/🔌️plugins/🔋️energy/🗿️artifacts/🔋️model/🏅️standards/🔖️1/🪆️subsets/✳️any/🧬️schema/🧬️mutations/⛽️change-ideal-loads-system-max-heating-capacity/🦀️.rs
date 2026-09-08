//! ⛽️ Energy model mutation — `ChangeIdealLoadsSystemMaxHeatingCapacity`: Sets or clears the ideal-loads heating capacity limit in watts. Cleared means autosized.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛽️ `change-ideal-loads-system-max-heating-capacity` payload. Sets or clears the ideal-loads heating capacity limit in watts. Cleared means autosized.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-max-heating-capacity")]
pub struct ChangeIdealLoadsSystemMaxHeatingCapacity {
    pub id: crate::model::EntityId,
    pub new_capacity_present: bool,
    pub new_max_heating_capacity_w: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_ideal_loads_system_max_heating_capacity(id: crate::model::EntityId, new_capacity_present: bool, new_max_heating_capacity_w: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeIdealLoadsSystemMaxHeatingCapacity(ChangeIdealLoadsSystemMaxHeatingCapacity { id, new_capacity_present, new_max_heating_capacity_w })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeIdealLoadsSystemMaxHeatingCapacity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ideal-loads-system", kind: "change-ideal-loads-system-max-heating-capacity", record: "ChangedIdealLoadsSystemMaxHeatingCapacity" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change ideal loads system {} maximum heating capacity to {:?}", self.id.0, self.new_capacity_present.then_some(self.new_max_heating_capacity_w))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

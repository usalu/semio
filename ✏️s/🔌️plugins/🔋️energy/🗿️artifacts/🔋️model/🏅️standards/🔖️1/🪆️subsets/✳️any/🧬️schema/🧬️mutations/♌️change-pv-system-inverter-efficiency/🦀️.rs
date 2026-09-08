//! ♌️ Energy model mutation — `ChangePvSystemInverterEfficiency`: Sets inverter efficiency on one pv system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ♌️ `change-pv-system-inverter-efficiency` payload. Sets inverter efficiency on one pv system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-pv-system-inverter-efficiency")]
pub struct ChangePvSystemInverterEfficiency {
    pub id: crate::model::EntityId,
    pub new_inverter_efficiency: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_pv_system_inverter_efficiency(id: crate::model::EntityId, new_inverter_efficiency: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangePvSystemInverterEfficiency(ChangePvSystemInverterEfficiency { id, new_inverter_efficiency })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangePvSystemInverterEfficiency {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "pv-system", kind: "change-pv-system-inverter-efficiency", record: "ChangedPvSystemInverterEfficiency" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Pv System Inverter Efficiency of pv system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

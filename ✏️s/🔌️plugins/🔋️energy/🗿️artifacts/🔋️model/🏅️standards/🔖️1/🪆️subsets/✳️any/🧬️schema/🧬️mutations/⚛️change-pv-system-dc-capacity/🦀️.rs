//! ⚛️ Energy model mutation — `ChangePvSystemDcCapacity`: Sets DC capacity (W) on one pv system, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚛️ `change-pv-system-dc-capacity` payload. Sets DC capacity (W) on one pv system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-pv-system-dc-capacity")]
pub struct ChangePvSystemDcCapacity {
    pub id: crate::model::EntityId,
    pub new_dc_capacity_w: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_pv_system_dc_capacity(id: crate::model::EntityId, new_dc_capacity_w: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangePvSystemDcCapacity(ChangePvSystemDcCapacity { id, new_dc_capacity_w })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangePvSystemDcCapacity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "pv-system", kind: "change-pv-system-dc-capacity", record: "ChangedPvSystemDcCapacity" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Pv System Dc Capacity of pv system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

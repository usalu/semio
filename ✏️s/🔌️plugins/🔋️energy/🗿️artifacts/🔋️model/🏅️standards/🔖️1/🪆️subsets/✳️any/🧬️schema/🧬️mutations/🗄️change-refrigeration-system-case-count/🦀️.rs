//! 🗄️ Energy model mutation — `ChangeRefrigerationSystemCaseCount`: Sets how many display cases one refrigeration system carries.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🗄️ `change-refrigeration-system-case-count` payload. Sets how many display cases one refrigeration system carries.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-refrigeration-system-case-count")]
pub struct ChangeRefrigerationSystemCaseCount {
    pub id: crate::model::EntityId,
    pub new_case_count: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_refrigeration_system_case_count(id: crate::model::EntityId, new_case_count: u32) -> EnergyModelMutation {
    EnergyModelMutation::ChangeRefrigerationSystemCaseCount(ChangeRefrigerationSystemCaseCount { id, new_case_count })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeRefrigerationSystemCaseCount {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "refrigeration-system", kind: "change-refrigeration-system-case-count", record: "ChangedRefrigerationSystemCaseCount" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Refrigeration System Case Count of refrigeration system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

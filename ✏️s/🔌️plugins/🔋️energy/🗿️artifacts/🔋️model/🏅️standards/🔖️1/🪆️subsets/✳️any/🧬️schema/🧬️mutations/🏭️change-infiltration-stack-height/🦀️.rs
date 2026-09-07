//! 🏭️ Energy model mutation — `ChangeInfiltrationStackHeight`: Sets stack height (m) on one infiltration, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏭️ `change-infiltration-stack-height` payload. Sets stack height (m) on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-stack-height")]
pub struct ChangeInfiltrationStackHeight {
    pub id: crate::model::EntityId,
    pub new_stack_height_m: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_infiltration_stack_height(id: crate::model::EntityId, new_stack_height_m: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeInfiltrationStackHeight(ChangeInfiltrationStackHeight { id, new_stack_height_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeInfiltrationStackHeight {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "infiltration", kind: "change-infiltration-stack-height", record: "ChangedInfiltrationStackHeight" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Infiltration Stack Height of infiltration {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

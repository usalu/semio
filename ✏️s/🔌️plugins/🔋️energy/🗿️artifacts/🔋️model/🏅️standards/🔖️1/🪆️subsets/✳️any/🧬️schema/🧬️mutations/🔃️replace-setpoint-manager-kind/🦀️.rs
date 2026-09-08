//! 🔃️ Energy model mutation — `ReplaceSetpointManagerKind`: Swaps the whole tagged control law of one setpoint manager. `replace`, not `change`, because the payload shape genuinely differs per variant (taxonomy rule for tagged unions); the four outdoor-air-reset limits are carried flattened beside the variant's wire name and must be zero for every other variant.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔃️ `replace-setpoint-manager-kind` payload. Swaps the whole tagged control law of one setpoint manager. `replace`, not `change`, because the payload shape genuinely differs per variant (taxonomy rule for tagged unions); the four outdoor-air-reset limits are carried flattened beside the variant's wire name and must be zero for every other variant.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-setpoint-manager-kind")]
pub struct ReplaceSetpointManagerKind {
    pub id: crate::model::EntityId,
    pub new_kind: String,
    pub new_low_outdoor_c: f64,
    pub new_high_outdoor_c: f64,
    pub new_low_setpoint_c: f64,
    pub new_high_setpoint_c: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_setpoint_manager_kind(id: crate::model::EntityId, new_kind: String, new_low_outdoor_c: f64, new_high_outdoor_c: f64, new_low_setpoint_c: f64, new_high_setpoint_c: f64) -> EnergyModelMutation {
    EnergyModelMutation::ReplaceSetpointManagerKind(ReplaceSetpointManagerKind { id, new_kind, new_low_outdoor_c, new_high_outdoor_c, new_low_setpoint_c, new_high_setpoint_c })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ReplaceSetpointManagerKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "setpoint-manager", kind: "replace-setpoint-manager-kind", record: "ReplacedSetpointManagerKind" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Replace setpoint manager {} control law with {}", self.id.0, self.new_kind)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

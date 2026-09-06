//! ✂️ Energy model mutation — `DisconnectReferencedModel`: Removes the relationship to the geometry model. Refused when no relationship exists.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ✂️ `disconnect-referenced-model` payload. Removes the relationship to the geometry model. Refused when no relationship exists.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "disconnect-referenced-model")]
pub struct DisconnectReferencedModel {}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_referenced_model() -> EnergyModelMutation {
    EnergyModelMutation::DisconnectReferencedModel(DisconnectReferencedModel {})
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DisconnectReferencedModel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "referenced-model", kind: "disconnect-referenced-model", record: "DisconnectedReferencedModel" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Disconnect the referenced model".to_string()
    }
}
//#endregion 🔖️Mutation

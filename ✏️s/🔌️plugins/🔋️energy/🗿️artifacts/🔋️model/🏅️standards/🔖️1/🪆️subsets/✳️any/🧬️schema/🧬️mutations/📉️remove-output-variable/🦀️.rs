//! 📉️ Energy model mutation — `RemoveOutputVariable`: Detaches one reporting registration addressed by its `(name, key)` natural composite key.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📉️ `remove-output-variable` payload. Detaches one reporting registration addressed by its `(name, key)` natural composite key.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-output-variable")]
pub struct RemoveOutputVariable {
    pub name: String,
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_output_variable(name: String, key: String) -> EnergyModelMutation {
    EnergyModelMutation::RemoveOutputVariable(RemoveOutputVariable { name, key })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RemoveOutputVariable {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "output-variable", kind: "remove-output-variable", record: "RemovedOutputVariable" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Remove output variable \"{}\" for \"{}\"", self.name, self.key)
    }

    fn target(&self) -> Vec<String> {
        vec![self.name.clone(), self.key.clone()]
    }
}
//#endregion 🔖️Mutation

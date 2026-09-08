//! 🔢️ Energy model mutation — `ChangeModelVersion`: Sets the model document's own version string.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔢️ `change-model-version` payload. Sets the model document's own version string.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-model-version")]
pub struct ChangeModelVersion {
    pub new_version: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_model_version(new_version: String) -> EnergyModelMutation {
    EnergyModelMutation::ChangeModelVersion(ChangeModelVersion { new_version })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeModelVersion {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "model", kind: "change-model-version", record: "ChangedModelVersion" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change energy model version to \"{}\"", self.new_version)
    }
}
//#endregion 🔖️Mutation

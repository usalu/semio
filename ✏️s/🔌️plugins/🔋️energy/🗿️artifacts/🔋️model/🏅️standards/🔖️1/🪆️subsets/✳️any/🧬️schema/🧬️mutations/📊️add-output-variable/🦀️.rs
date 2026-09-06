//! 📊️ Energy model mutation — `AddOutputVariable`: Attaches one reporting registration to the document-root output-variable set. `(name, key)` is the natural composite key — `OutputVariableSpec` carries no id — so a duplicate registration is refused rather than silently doubled.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📊️ `add-output-variable` payload. Attaches one reporting registration to the document-root output-variable set. `(name, key)` is the natural composite key — `OutputVariableSpec` carries no id — so a duplicate registration is refused rather than silently doubled.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-output-variable")]
pub struct AddOutputVariable {
    pub name: String,
    pub key: String,
    pub reporting_frequency: crate::model::OutputReportFrequency,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_output_variable(name: String, key: String, reporting_frequency: crate::model::OutputReportFrequency) -> EnergyModelMutation {
    EnergyModelMutation::AddOutputVariable(AddOutputVariable { name, key, reporting_frequency })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for AddOutputVariable {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "output-variable", kind: "add-output-variable", record: "AddedOutputVariable" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Add output variable \"{}\" for \"{}\"", self.name, self.key)
    }

    fn target(&self) -> Vec<String> {
        vec![self.name.clone(), self.key.clone()]
    }
}
//#endregion 🔖️Mutation

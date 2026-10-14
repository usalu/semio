//! 📊️ Energy model mutation — `AddOutputVariable`: Attaches one reporting registration to the document-root output-variable set. `(name, key)` is the natural composite key — `OutputVariableSpec` carries no id — so a duplicate registration is refused rather than silently doubled.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📊️ `add-output-variable` payload. Attaches one reporting registration to the document-root output-variable set. `(name, key)` is the natural composite key — `OutputVariableSpec` carries no id — so a duplicate registration is refused rather than silently doubled.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-output-variable")]
pub struct AddOutputVariable {
    pub name: String,
    pub key: String,
    pub reporting_frequency: crate::model::OutputReportFrequency,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn add_output_variable<Op: From<AddOutputVariable>>(name: String, key: String, reporting_frequency: crate::model::OutputReportFrequency, index: Option<u32>) -> Op {
    AddOutputVariable { name, key, reporting_frequency, index }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for AddOutputVariable
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<AddOutputVariable> + From<crate::RemoveOutputVariable>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "output-variable", kind: "add-output-variable", record: "AddedOutputVariable" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add output variable \"{}\" for \"{}\"", self.name, self.key), &format!("Ausgabevariable \"{}\" für \"{}\" hinzufügen", self.name, self.key))
    }

    fn target(&self) -> Vec<String> {
        vec![self.name.clone(), self.key.clone()]
    }
}
//#endregion 🔖️Mutation

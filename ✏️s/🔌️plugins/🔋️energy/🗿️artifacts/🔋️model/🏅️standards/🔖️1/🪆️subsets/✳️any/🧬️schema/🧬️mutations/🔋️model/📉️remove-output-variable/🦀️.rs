//! 📉️ Energy model mutation — `RemoveOutputVariable`: Detaches one reporting registration addressed by its `(name, key)` natural composite key.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📉️ `remove-output-variable` payload. Detaches one reporting registration addressed by its `(name, key)` natural composite key.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-output-variable")]
pub struct RemoveOutputVariable {
    pub name: String,
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_output_variable<Op: From<RemoveOutputVariable>>(name: String, key: String) -> Op {
    RemoveOutputVariable { name, key }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RemoveOutputVariable
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RemoveOutputVariable> + From<crate::AddOutputVariable>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "output-variable", kind: "remove-output-variable", record: "RemovedOutputVariable" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove output variable \"{}\" for \"{}\"", self.name, self.key), &format!("Ausgabevariable \"{}\" für \"{}\" entfernen", self.name, self.key))
    }

    fn target(&self) -> Vec<String> {
        vec![self.name.clone(), self.key.clone()]
    }
}
//#endregion 🔖️Mutation

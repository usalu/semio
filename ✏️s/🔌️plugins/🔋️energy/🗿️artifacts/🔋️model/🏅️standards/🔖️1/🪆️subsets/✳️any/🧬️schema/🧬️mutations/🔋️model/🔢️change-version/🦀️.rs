//! 🔢️ Energy model mutation — `ChangeModelVersion`: Sets the model document's own version string.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔢️ `change-model-version` payload. Sets the model document's own version string.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-model-version")]
pub struct ChangeModelVersion {
    pub new_version: String,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_model_version<Op: From<ChangeModelVersion>>(new_version: String) -> Op {
    ChangeModelVersion { new_version }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeModelVersion
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeModelVersion>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "model", kind: "change-model-version", record: "ChangedModelVersion" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change energy model version to \"{}\"", self.new_version), &format!("Energiemodellversion auf \"{}\" ändern", self.new_version))
    }
}
//#endregion 🔖️Mutation

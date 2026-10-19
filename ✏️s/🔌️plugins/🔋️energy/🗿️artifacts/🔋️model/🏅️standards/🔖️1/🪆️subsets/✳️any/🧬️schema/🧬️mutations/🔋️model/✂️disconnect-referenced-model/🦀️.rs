//! ✂️ Energy model mutation — `DisconnectReferencedModel`: Removes the relationship to the geometry model. Refused when no relationship exists.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ✂️ `disconnect-referenced-model` payload. Removes the relationship to the geometry model. Refused when no relationship exists.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "disconnect-referenced-model")]
pub struct DisconnectReferencedModel {}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn disconnect_referenced_model<Op: From<DisconnectReferencedModel>>() -> Op {
    DisconnectReferencedModel {}.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for DisconnectReferencedModel
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<DisconnectReferencedModel> + From<crate::ConnectReferencedModel>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "referenced-model", kind: "disconnect-referenced-model", record: "DisconnectedReferencedModel" };





    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Disconnect the referenced model", "Referenziertes Modell trennen")
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for DisconnectReferencedModel
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<DisconnectReferencedModel> + From<crate::ConnectReferencedModel>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

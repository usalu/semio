//! 🧾️ Energy model mutation — `ChangeSizingObjectSizingType`: Swaps which load the sizing run solves for.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧾️ `change-sizing-object-sizing-type` payload. Swaps which load the sizing run solves for.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-sizing-object-sizing-type")]
pub struct ChangeSizingObjectSizingType {
    pub id: crate::model::EntityId,
    pub new_sizing_type: crate::model::SizingType,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_sizing_object_sizing_type<Op: From<ChangeSizingObjectSizingType>>(id: crate::model::EntityId, new_sizing_type: crate::model::SizingType) -> Op {
    ChangeSizingObjectSizingType { id, new_sizing_type }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSizingObjectSizingType
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSizingObjectSizingType>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sizing-object", kind: "change-sizing-object-sizing-type", record: "ChangedSizingObjectSizingType" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change sizing object {} sizing type to {:?}", self.id.0, self.new_sizing_type), &format!("Auslegungsart von Auslegungsobjekt {} auf {:?} ändern", self.id.0, self.new_sizing_type))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeSizingObjectSizingType
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeSizingObjectSizingType>,
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

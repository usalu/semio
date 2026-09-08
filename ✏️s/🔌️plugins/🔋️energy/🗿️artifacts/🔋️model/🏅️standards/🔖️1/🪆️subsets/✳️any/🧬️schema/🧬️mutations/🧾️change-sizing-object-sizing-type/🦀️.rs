//! 🧾️ Energy model mutation — `ChangeSizingObjectSizingType`: Swaps which load the sizing run solves for.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧾️ `change-sizing-object-sizing-type` payload. Swaps which load the sizing run solves for.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-sizing-object-sizing-type")]
pub struct ChangeSizingObjectSizingType {
    pub id: crate::model::EntityId,
    pub new_sizing_type: crate::model::SizingType,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_sizing_object_sizing_type(id: crate::model::EntityId, new_sizing_type: crate::model::SizingType) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSizingObjectSizingType(ChangeSizingObjectSizingType { id, new_sizing_type })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSizingObjectSizingType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sizing-object", kind: "change-sizing-object-sizing-type", record: "ChangedSizingObjectSizingType" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change sizing object {} sizing type to {:?}", self.id.0, self.new_sizing_type)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

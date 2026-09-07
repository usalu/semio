//! 🌥️ Energy model mutation — `ChangeSizingObjectDesignDayType`: Swaps which design day the sizing run reads.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌥️ `change-sizing-object-design-day-type` payload. Swaps which design day the sizing run reads.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-sizing-object-design-day-type")]
pub struct ChangeSizingObjectDesignDayType {
    pub id: crate::model::EntityId,
    pub new_design_day_type: crate::model::DesignDayType,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_sizing_object_design_day_type(id: crate::model::EntityId, new_design_day_type: crate::model::DesignDayType) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSizingObjectDesignDayType(ChangeSizingObjectDesignDayType { id, new_design_day_type })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSizingObjectDesignDayType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sizing-object", kind: "change-sizing-object-design-day-type", record: "ChangedSizingObjectDesignDayType" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change sizing object {} design day type to {:?}", self.id.0, self.new_design_day_type)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

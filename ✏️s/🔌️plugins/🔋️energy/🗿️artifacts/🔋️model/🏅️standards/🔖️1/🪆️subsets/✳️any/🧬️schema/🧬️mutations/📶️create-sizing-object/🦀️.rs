//! 📶️ Energy model mutation — `CreateSizingObject`: Declares that one zone is autosized against a design day: which load the sizing run solves for and which design day it reads.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📶️ `create-sizing-object` payload. Declares that one zone is autosized against a design day: which load the sizing run solves for and which design day it reads.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-sizing-object")]
pub struct CreateSizingObject {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub sizing_type: crate::model::SizingType,
    pub design_day_type: crate::model::DesignDayType,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_sizing_object(id: crate::model::EntityId, zone_id: crate::model::EntityId, sizing_type: crate::model::SizingType, design_day_type: crate::model::DesignDayType) -> EnergyModelMutation {
    EnergyModelMutation::CreateSizingObject(CreateSizingObject { id, zone_id, sizing_type, design_day_type })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateSizingObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "sizing-object", kind: "create-sizing-object", record: "CreatedSizingObject" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create sizing object {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

//! 🪑️ Energy model mutation — `CreateSpace`: Adds one space inside an existing zone. The owning zone must already exist, so the document never carries a space whose zone the reports cannot resolve.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪑️ `create-space` payload. Adds one space inside an existing zone. The owning zone must already exist, so the document never carries a space whose zone the reports cannot resolve.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-space")]
pub struct CreateSpace {
    pub id: crate::model::EntityId,
    pub name: String,
    pub zone_id: crate::model::EntityId,
    pub floor_area_m2: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_space(id: crate::model::EntityId, name: String, zone_id: crate::model::EntityId, floor_area_m2: f64) -> EnergyModelMutation {
    EnergyModelMutation::CreateSpace(CreateSpace { id, name, zone_id, floor_area_m2 })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateSpace {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "space", kind: "create-space", record: "CreatedSpace" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create space \"{}\"", self.name)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

//! 🏗️ Energy model mutation — `CreateConstruction`: Adds one layered construction, outside-to-inside layer order. Every layer must already name a material the document defines, so a construction is never born dangling.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏗️ `create-construction` payload. Adds one layered construction, outside-to-inside layer order. Every layer must already name a material the document defines, so a construction is never born dangling.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-construction")]
pub struct CreateConstruction {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub name: String,
    pub layer_material_ids: Vec<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_construction(index: u32, id: crate::model::EntityId, name: String, layer_material_ids: Vec<crate::model::EntityId>) -> EnergyModelMutation {
    EnergyModelMutation::CreateConstruction(CreateConstruction { index, id, name, layer_material_ids })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateConstruction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "construction", kind: "create-construction", record: "CreatedConstruction" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Construction {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

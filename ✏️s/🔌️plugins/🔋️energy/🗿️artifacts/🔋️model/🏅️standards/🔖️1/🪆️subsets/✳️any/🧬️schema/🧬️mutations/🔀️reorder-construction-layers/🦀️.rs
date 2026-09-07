//! 🔀️ Energy model mutation — `ReorderConstructionLayers`: Restates a construction's whole layer sequence. The new sequence must be a permutation of the one the construction already holds — adding or dropping a layer is `add-construction-layer`'s and `remove-construction-layer`'s job, so a reorder can never change which materials a wall is made of.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔀️ `reorder-construction-layers` payload. Restates a construction's whole layer sequence. The new sequence must be a permutation of the one the construction already holds — adding or dropping a layer is `add-construction-layer`'s and `remove-construction-layer`'s job, so a reorder can never change which materials a wall is made of.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "reorder-construction-layers")]
pub struct ReorderConstructionLayers {
    pub id: crate::model::EntityId,
    pub new_layer_material_ids: Vec<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn reorder_construction_layers(id: crate::model::EntityId, new_layer_material_ids: Vec<crate::model::EntityId>) -> EnergyModelMutation {
    EnergyModelMutation::ReorderConstructionLayers(ReorderConstructionLayers { id, new_layer_material_ids })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ReorderConstructionLayers {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "construction-layers", kind: "reorder-construction-layers", record: "ReorderedConstructionLayers" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Reorder construction {} into {} layers", self.id.0, self.new_layer_material_ids.len())
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

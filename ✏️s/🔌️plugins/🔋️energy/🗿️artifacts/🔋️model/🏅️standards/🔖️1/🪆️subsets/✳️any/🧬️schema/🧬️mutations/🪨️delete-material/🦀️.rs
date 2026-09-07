//! 🪨️ Energy model mutation — `DeleteMaterial`: Removes one material definition. Refused while any construction still names it as a layer — the alternative would be to cascade into constructions, which cascade into surfaces.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪨️ `delete-material` payload. Removes one material definition. Refused while any construction still names it as a layer — the alternative would be to cascade into constructions, which cascade into surfaces.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-material")]
pub struct DeleteMaterial {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_material(id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteMaterial(DeleteMaterial { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteMaterial {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "material", kind: "delete-material", record: "DeletedMaterial" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete Material {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

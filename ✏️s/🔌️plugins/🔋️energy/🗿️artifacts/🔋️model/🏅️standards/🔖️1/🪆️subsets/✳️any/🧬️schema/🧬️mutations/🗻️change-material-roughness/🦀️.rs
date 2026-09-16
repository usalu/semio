//! 🗻️ Energy model mutation — `ChangeMaterialRoughness`: Sets the surface roughness class of one material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🗻️ `change-material-roughness` payload. Sets the surface roughness class of one material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-material-roughness")]
pub struct ChangeMaterialRoughness {
    pub id: crate::model::EntityId,
    pub new_roughness: crate::model::SurfaceRoughness,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_material_roughness(id: crate::model::EntityId, new_roughness: crate::model::SurfaceRoughness) -> EnergyModelMutation {
    EnergyModelMutation::ChangeMaterialRoughness(ChangeMaterialRoughness { id, new_roughness })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeMaterialRoughness {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material", kind: "change-material-roughness", record: "ChangedMaterialRoughness" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Material Roughness of material {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

//! 👁️ Energy model mutation — `ChangeMaterialVisibleAbsorptance`: Sets visible absorptance on one material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 👁️ `change-material-visible-absorptance` payload. Sets visible absorptance on one material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-material-visible-absorptance")]
pub struct ChangeMaterialVisibleAbsorptance {
    pub id: crate::model::EntityId,
    pub new_visible_absorptance: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_material_visible_absorptance(id: crate::model::EntityId, new_visible_absorptance: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeMaterialVisibleAbsorptance(ChangeMaterialVisibleAbsorptance { id, new_visible_absorptance })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeMaterialVisibleAbsorptance {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material", kind: "change-material-visible-absorptance", record: "ChangedMaterialVisibleAbsorptance" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Material Visible Absorptance of material {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

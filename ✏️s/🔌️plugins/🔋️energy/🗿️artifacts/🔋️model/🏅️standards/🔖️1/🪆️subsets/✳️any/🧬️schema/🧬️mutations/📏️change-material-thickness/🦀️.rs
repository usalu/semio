//! 📏️ Energy model mutation — `ChangeMaterialThickness`: Sets thickness (m) on one material, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📏️ `change-material-thickness` payload. Sets thickness (m) on one material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-material-thickness")]
pub struct ChangeMaterialThickness {
    pub id: crate::model::EntityId,
    pub new_thickness_m: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_material_thickness(id: crate::model::EntityId, new_thickness_m: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeMaterialThickness(ChangeMaterialThickness { id, new_thickness_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeMaterialThickness {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material", kind: "change-material-thickness", record: "ChangedMaterialThickness" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Material Thickness of material {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

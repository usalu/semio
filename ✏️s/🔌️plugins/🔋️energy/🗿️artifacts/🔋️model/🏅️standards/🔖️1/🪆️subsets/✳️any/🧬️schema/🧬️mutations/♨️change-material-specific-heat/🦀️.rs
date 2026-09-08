//! ♨️ Energy model mutation — `ChangeMaterialSpecificHeat`: Sets specific heat (J/kg·K) on one material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ♨️ `change-material-specific-heat` payload. Sets specific heat (J/kg·K) on one material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-material-specific-heat")]
pub struct ChangeMaterialSpecificHeat {
    pub id: crate::model::EntityId,
    pub new_specific_heat_j_kg_k: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_material_specific_heat(id: crate::model::EntityId, new_specific_heat_j_kg_k: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeMaterialSpecificHeat(ChangeMaterialSpecificHeat { id, new_specific_heat_j_kg_k })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeMaterialSpecificHeat {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material", kind: "change-material-specific-heat", record: "ChangedMaterialSpecificHeat" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Material Specific Heat of material {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

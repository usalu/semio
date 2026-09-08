//! 🔥️ Energy model mutation — `ChangeMaterialConductivity`: Sets conductivity (W/m·K) on one material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔥️ `change-material-conductivity` payload. Sets conductivity (W/m·K) on one material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-material-conductivity")]
pub struct ChangeMaterialConductivity {
    pub id: crate::model::EntityId,
    pub new_conductivity_w_m_k: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_material_conductivity(id: crate::model::EntityId, new_conductivity_w_m_k: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeMaterialConductivity(ChangeMaterialConductivity { id, new_conductivity_w_m_k })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeMaterialConductivity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material", kind: "change-material-conductivity", record: "ChangedMaterialConductivity" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Material Conductivity of material {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

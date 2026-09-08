//! ⚖️ Energy model mutation — `ChangeMaterialDensity`: Sets density (kg/m³) on one material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚖️ `change-material-density` payload. Sets density (kg/m³) on one material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-material-density")]
pub struct ChangeMaterialDensity {
    pub id: crate::model::EntityId,
    pub new_density_kg_m3: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_material_density(id: crate::model::EntityId, new_density_kg_m3: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeMaterialDensity(ChangeMaterialDensity { id, new_density_kg_m3 })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeMaterialDensity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material", kind: "change-material-density", record: "ChangedMaterialDensity" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Material Density of material {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

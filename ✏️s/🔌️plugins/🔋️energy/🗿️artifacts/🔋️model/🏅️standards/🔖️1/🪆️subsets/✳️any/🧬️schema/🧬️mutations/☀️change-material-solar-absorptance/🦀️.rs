//! ☀️ Energy model mutation — `ChangeMaterialSolarAbsorptance`: Sets solar absorptance on one material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ☀️ `change-material-solar-absorptance` payload. Sets solar absorptance on one material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-material-solar-absorptance")]
pub struct ChangeMaterialSolarAbsorptance {
    pub id: crate::model::EntityId,
    pub new_solar_absorptance: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_material_solar_absorptance(id: crate::model::EntityId, new_solar_absorptance: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeMaterialSolarAbsorptance(ChangeMaterialSolarAbsorptance { id, new_solar_absorptance })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeMaterialSolarAbsorptance {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material", kind: "change-material-solar-absorptance", record: "ChangedMaterialSolarAbsorptance" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Material Solar Absorptance of material {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

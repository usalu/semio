//! 🧱️ Energy model mutation — `CreateMaterial`: Adds one opaque material layer definition at a stated position in the model's material list. Thickness, conductivity, density and specific heat are the four the conduction transfer functions integrate; the three absorptances close the surface radiation balance.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧱️ `create-material` payload. Adds one opaque material layer definition at a stated position in the model's material list. Thickness, conductivity, density and specific heat are the four the conduction transfer functions integrate; the three absorptances close the surface radiation balance.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-material")]
pub struct CreateMaterial {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub name: String,
    pub thickness_m: f64,
    pub conductivity_w_m_k: f64,
    pub density_kg_m3: f64,
    pub specific_heat_j_kg_k: f64,
    pub thermal_absorptance: f64,
    pub solar_absorptance: f64,
    pub visible_absorptance: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_material(index: u32, id: crate::model::EntityId, name: String, thickness_m: f64, conductivity_w_m_k: f64, density_kg_m3: f64, specific_heat_j_kg_k: f64, thermal_absorptance: f64, solar_absorptance: f64, visible_absorptance: f64) -> EnergyModelMutation {
    EnergyModelMutation::CreateMaterial(CreateMaterial { index, id, name, thickness_m, conductivity_w_m_k, density_kg_m3, specific_heat_j_kg_k, thermal_absorptance, solar_absorptance, visible_absorptance })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateMaterial {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "material", kind: "create-material", record: "CreatedMaterial" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Material {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

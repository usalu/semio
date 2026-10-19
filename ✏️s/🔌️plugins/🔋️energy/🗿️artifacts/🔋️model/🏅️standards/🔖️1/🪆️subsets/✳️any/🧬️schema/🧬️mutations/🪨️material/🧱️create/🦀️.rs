//! 🧱️ Energy model mutation — `CreateMaterial`: Adds one opaque material layer definition at a stated position in the model's material list. Thickness, conductivity, density and specific heat are the four the conduction transfer functions integrate; the three absorptances close the surface radiation balance.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧱️ `create-material` payload. Adds one opaque material layer definition at a stated position in the model's material list. Thickness, conductivity, density and specific heat are the four the conduction transfer functions integrate; the three absorptances close the surface radiation balance.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-material")]
pub struct CreateMaterial {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub name: String,
    pub roughness: crate::model::SurfaceRoughness,
    pub thickness_m: f64,
    pub conductivity_w_m_k: f64,
    pub density_kg_m3: f64,
    pub specific_heat_j_kg_k: f64,
    pub thermal_absorptance: f64,
    pub solar_absorptance: f64,
    pub visible_absorptance: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_material<Op: From<CreateMaterial>>(
    index: u32,
    id: crate::model::EntityId,
    name: String,
    roughness: crate::model::SurfaceRoughness,
    thickness_m: f64,
    conductivity_w_m_k: f64,
    density_kg_m3: f64,
    specific_heat_j_kg_k: f64,
    thermal_absorptance: f64,
    solar_absorptance: f64,
    visible_absorptance: f64,
) -> Op {
    CreateMaterial { index, id, name, roughness, thickness_m, conductivity_w_m_k, density_kg_m3, specific_heat_j_kg_k, thermal_absorptance, solar_absorptance, visible_absorptance }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateMaterial
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateMaterial> + From<crate::DeleteMaterial>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create Material {} at index {}", self.id.0, self.index), &format!("Material {} an Index {} erstellen", self.id.0, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateMaterial
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateMaterial> + From<crate::DeleteMaterial>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "material", kind: "create-material", record: "CreatedMaterial" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

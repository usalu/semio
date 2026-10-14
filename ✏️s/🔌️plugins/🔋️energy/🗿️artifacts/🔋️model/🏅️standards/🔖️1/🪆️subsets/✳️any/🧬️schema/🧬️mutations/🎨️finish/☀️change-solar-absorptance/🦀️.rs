//! ☀️ Energy model mutation — `ChangeMaterialSolarAbsorptance`: Sets solar absorptance on one material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ☀️ `change-material-solar-absorptance` payload. Sets solar absorptance on one material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-material-solar-absorptance")]
pub struct ChangeMaterialSolarAbsorptance {
    pub id: crate::model::EntityId,
    pub new_solar_absorptance: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_material_solar_absorptance<Op: From<ChangeMaterialSolarAbsorptance>>(id: crate::model::EntityId, new_solar_absorptance: f64) -> Op {
    ChangeMaterialSolarAbsorptance { id, new_solar_absorptance }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeMaterialSolarAbsorptance
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeMaterialSolarAbsorptance>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material", kind: "change-material-solar-absorptance", record: "ChangedMaterialSolarAbsorptance" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Material Solar Absorptance of material {}", self.id.0), &format!("Solaren Absorptionsgrad von Material {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

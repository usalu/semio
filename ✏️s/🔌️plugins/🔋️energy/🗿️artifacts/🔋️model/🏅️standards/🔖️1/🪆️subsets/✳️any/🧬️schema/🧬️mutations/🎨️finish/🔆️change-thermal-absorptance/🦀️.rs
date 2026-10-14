//! 🔆️ Energy model mutation — `ChangeMaterialThermalAbsorptance`: Sets thermal absorptance on one material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔆️ `change-material-thermal-absorptance` payload. Sets thermal absorptance on one material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-material-thermal-absorptance")]
pub struct ChangeMaterialThermalAbsorptance {
    pub id: crate::model::EntityId,
    pub new_thermal_absorptance: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_material_thermal_absorptance<Op: From<ChangeMaterialThermalAbsorptance>>(id: crate::model::EntityId, new_thermal_absorptance: f64) -> Op {
    ChangeMaterialThermalAbsorptance { id, new_thermal_absorptance }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeMaterialThermalAbsorptance
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeMaterialThermalAbsorptance>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material", kind: "change-material-thermal-absorptance", record: "ChangedMaterialThermalAbsorptance" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Material Thermal Absorptance of material {}", self.id.0), &format!("Thermischen Absorptionsgrad von Material {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

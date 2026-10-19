//! ♨️ Energy model mutation — `ChangeMaterialSpecificHeat`: Sets specific heat (J/kg·K) on one material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ♨️ `change-material-specific-heat` payload. Sets specific heat (J/kg·K) on one material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-material-specific-heat")]
pub struct ChangeMaterialSpecificHeat {
    pub id: crate::model::EntityId,
    pub new_specific_heat_j_kg_k: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_material_specific_heat<Op: From<ChangeMaterialSpecificHeat>>(id: crate::model::EntityId, new_specific_heat_j_kg_k: f64) -> Op {
    ChangeMaterialSpecificHeat { id, new_specific_heat_j_kg_k }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeMaterialSpecificHeat
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeMaterialSpecificHeat>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material", kind: "change-material-specific-heat", record: "ChangedMaterialSpecificHeat" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Material Specific Heat of material {}", self.id.0), &format!("Spezifische Wärmekapazität von Material {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeMaterialSpecificHeat
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeMaterialSpecificHeat>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }





    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

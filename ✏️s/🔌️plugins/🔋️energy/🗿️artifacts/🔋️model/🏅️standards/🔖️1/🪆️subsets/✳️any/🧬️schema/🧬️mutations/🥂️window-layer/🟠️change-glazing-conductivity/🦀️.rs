//! 🟠️ Energy model mutation — `ChangeGlazingMaterialConductivity`: Sets pane conductivity (W/m·K) on one glazing material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟠️ `change-glazing-material-conductivity` payload. Sets pane conductivity (W/m·K) on one glazing material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-glazing-material-conductivity")]
pub struct ChangeGlazingMaterialConductivity {
    pub id: crate::model::EntityId,
    pub new_conductivity_w_m_k: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_glazing_material_conductivity<Op: From<ChangeGlazingMaterialConductivity>>(id: crate::model::EntityId, new_conductivity_w_m_k: f64) -> Op {
    ChangeGlazingMaterialConductivity { id, new_conductivity_w_m_k }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeGlazingMaterialConductivity
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeGlazingMaterialConductivity>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Glazing Material Conductivity of glazing material {}", self.id.0), &format!("Wärmeleitfähigkeit von Verglasungsmaterial {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeGlazingMaterialConductivity
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeGlazingMaterialConductivity>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "glazing-material", kind: "change-glazing-material-conductivity", record: "ChangedGlazingMaterialConductivity" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

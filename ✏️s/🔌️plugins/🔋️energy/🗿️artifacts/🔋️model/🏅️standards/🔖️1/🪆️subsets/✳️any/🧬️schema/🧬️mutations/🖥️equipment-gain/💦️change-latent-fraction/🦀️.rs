//! 💦️ Energy model mutation — `ChangeEquipmentGainLatentFraction`: Sets latent fraction on one equipment gain, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💦️ `change-equipment-gain-latent-fraction` payload. Sets latent fraction on one equipment gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-equipment-gain-latent-fraction")]
pub struct ChangeEquipmentGainLatentFraction {
    pub id: crate::model::EntityId,
    pub new_latent_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_equipment_gain_latent_fraction<Op: From<ChangeEquipmentGainLatentFraction>>(id: crate::model::EntityId, new_latent_fraction: f64) -> Op {
    ChangeEquipmentGainLatentFraction { id, new_latent_fraction }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeEquipmentGainLatentFraction
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeEquipmentGainLatentFraction>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "equipment-gain", kind: "change-equipment-gain-latent-fraction", record: "ChangedEquipmentGainLatentFraction" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Equipment Gain Latent Fraction of equipment gain {}", self.id.0), &format!("Latenten Anteil von Gerätewärmegewinn {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeEquipmentGainLatentFraction
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeEquipmentGainLatentFraction>,
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

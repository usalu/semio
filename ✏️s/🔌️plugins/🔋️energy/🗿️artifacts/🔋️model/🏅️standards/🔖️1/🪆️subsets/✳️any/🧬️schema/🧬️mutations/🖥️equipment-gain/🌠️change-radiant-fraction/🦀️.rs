//! 🌠️ Energy model mutation — `ChangeEquipmentGainRadiantFraction`: Sets radiant fraction on one equipment gain, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌠️ `change-equipment-gain-radiant-fraction` payload. Sets radiant fraction on one equipment gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-equipment-gain-radiant-fraction")]
pub struct ChangeEquipmentGainRadiantFraction {
    pub id: crate::model::EntityId,
    pub new_radiant_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_equipment_gain_radiant_fraction<Op: From<ChangeEquipmentGainRadiantFraction>>(id: crate::model::EntityId, new_radiant_fraction: f64) -> Op {
    ChangeEquipmentGainRadiantFraction { id, new_radiant_fraction }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeEquipmentGainRadiantFraction
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeEquipmentGainRadiantFraction>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "equipment-gain", kind: "change-equipment-gain-radiant-fraction", record: "ChangedEquipmentGainRadiantFraction" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Equipment Gain Radiant Fraction of equipment gain {}", self.id.0), &format!("Strahlungsanteil von Gerätewärmegewinn {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

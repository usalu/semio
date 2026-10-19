//! 🎗️ Energy model mutation — `ChangeZoneEquipmentPriority`: Sets the rank this equipment holds in its zone's own equipment list.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎗️ `change-zone-equipment-priority` payload. Sets the rank this equipment holds in its zone's own equipment list.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-equipment-priority")]
pub struct ChangeZoneEquipmentPriority {
    pub id: crate::model::EntityId,
    pub new_priority: u8,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_zone_equipment_priority<Op: From<ChangeZoneEquipmentPriority>>(id: crate::model::EntityId, new_priority: u8) -> Op {
    ChangeZoneEquipmentPriority { id, new_priority }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeZoneEquipmentPriority
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeZoneEquipmentPriority>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone-equipment", kind: "change-zone-equipment-priority", record: "ChangedZoneEquipmentPriority" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change zone equipment {} priority to {:?}", self.id.0, self.new_priority), &format!("Priorität von Zonengerät {} auf {:?} ändern", self.id.0, self.new_priority))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeZoneEquipmentPriority
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeZoneEquipmentPriority>,
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

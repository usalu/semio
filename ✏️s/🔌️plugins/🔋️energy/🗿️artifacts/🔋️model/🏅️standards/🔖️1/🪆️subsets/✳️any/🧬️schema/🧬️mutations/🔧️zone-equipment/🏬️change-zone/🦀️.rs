//! 🏬️ Energy model mutation — `ChangeZoneEquipmentZone`: Moves one piece of zone equipment to another zone.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏬️ `change-zone-equipment-zone` payload. Moves one piece of zone equipment to another zone.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-equipment-zone")]
pub struct ChangeZoneEquipmentZone {
    pub id: crate::model::EntityId,
    pub new_zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_zone_equipment_zone<Op: From<ChangeZoneEquipmentZone>>(id: crate::model::EntityId, new_zone_id: crate::model::EntityId) -> Op {
    ChangeZoneEquipmentZone { id, new_zone_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeZoneEquipmentZone
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeZoneEquipmentZone>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone-equipment", kind: "change-zone-equipment-zone", record: "ChangedZoneEquipmentZone" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change zone equipment {} zone to {}", self.id.0, self.new_zone_id.0), &format!("Zone von Zonengerät {} auf {} ändern", self.id.0, self.new_zone_id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeZoneEquipmentZone
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeZoneEquipmentZone>,
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

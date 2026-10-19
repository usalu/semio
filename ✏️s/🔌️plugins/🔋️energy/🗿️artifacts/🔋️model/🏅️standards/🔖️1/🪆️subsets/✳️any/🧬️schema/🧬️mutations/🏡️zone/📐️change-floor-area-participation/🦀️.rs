//! 📐️ Energy model mutation — `ChangeZoneFloorAreaParticipation`: Sets whether the zone's floor area counts toward the building total the normalized reports divide by.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📐️ `change-zone-floor-area-participation` payload. Sets whether the zone's floor area counts toward the building total the normalized reports divide by.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-floor-area-participation")]
pub struct ChangeZoneFloorAreaParticipation {
    pub id: crate::model::EntityId,
    pub new_part_of_total_floor_area: bool,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_zone_floor_area_participation<Op: From<ChangeZoneFloorAreaParticipation>>(id: crate::model::EntityId, new_part_of_total_floor_area: bool) -> Op {
    ChangeZoneFloorAreaParticipation { id, new_part_of_total_floor_area }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeZoneFloorAreaParticipation
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeZoneFloorAreaParticipation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone", kind: "change-zone-floor-area-participation", record: "ChangedZoneFloorAreaParticipation" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change zone {} floor-area participation to {}", self.id.0, self.new_part_of_total_floor_area), &format!("Geschossflächenanteil von Zone {} auf {} ändern", self.id.0, self.new_part_of_total_floor_area))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeZoneFloorAreaParticipation
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeZoneFloorAreaParticipation>,
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

//! 🔓️ Energy model mutation — `RemoveThermalEnclosureZone`: Takes one zone back out of a thermal enclosure; the zone itself survives.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔓️ `remove-thermal-enclosure-zone` payload. Takes one zone back out of a thermal enclosure; the zone itself survives.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-thermal-enclosure-zone")]
pub struct RemoveThermalEnclosureZone {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_thermal_enclosure_zone<Op: From<RemoveThermalEnclosureZone>>(id: crate::model::EntityId, zone_id: crate::model::EntityId) -> Op {
    RemoveThermalEnclosureZone { id, zone_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RemoveThermalEnclosureZone
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RemoveThermalEnclosureZone> + From<crate::AddThermalEnclosureZone>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "thermal-enclosure", kind: "remove-thermal-enclosure-zone", record: "RemovedThermalEnclosureZone" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove zone {} from thermal enclosure {}", self.zone_id.0, self.id.0), &format!("Zone {} aus Zonenverbund {} entfernen", self.zone_id.0, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for RemoveThermalEnclosureZone
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<RemoveThermalEnclosureZone> + From<crate::AddThermalEnclosureZone>,
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

//! 🔒️ Energy model mutation — `AddThermalEnclosureZone`: Puts one zone inside a thermal enclosure.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔒️ `add-thermal-enclosure-zone` payload. Puts one zone inside a thermal enclosure.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-thermal-enclosure-zone")]
pub struct AddThermalEnclosureZone {
    pub id: crate::model::EntityId,
    pub index: u32,
    pub zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn add_thermal_enclosure_zone<Op: From<AddThermalEnclosureZone>>(id: crate::model::EntityId, index: u32, zone_id: crate::model::EntityId) -> Op {
    AddThermalEnclosureZone { id, index, zone_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for AddThermalEnclosureZone
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<AddThermalEnclosureZone> + From<crate::RemoveThermalEnclosureZone>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "thermal-enclosure", kind: "add-thermal-enclosure-zone", record: "AddedThermalEnclosureZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add zone {} to thermal enclosure {}", self.zone_id.0, self.id.0), &format!("Zone {} zu Zonenverbund {} hinzufügen", self.zone_id.0, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

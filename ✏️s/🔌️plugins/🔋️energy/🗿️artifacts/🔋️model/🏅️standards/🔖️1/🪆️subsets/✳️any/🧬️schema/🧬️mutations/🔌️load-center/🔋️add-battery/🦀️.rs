//! 🔋️ Energy model mutation — `AddElectricalLoadCenterBattery`: Attaches one battery to a load centre, so the centre may charge and discharge it against the net bus balance.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔋️ `add-electrical-load-center-battery` payload. Attaches one battery to a load centre, so the centre may charge and discharge it against the net bus balance.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-electrical-load-center-battery")]
pub struct AddElectricalLoadCenterBattery {
    pub id: crate::model::EntityId,
    pub index: u32,
    pub battery_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn add_electrical_load_center_battery<Op: From<AddElectricalLoadCenterBattery>>(id: crate::model::EntityId, index: u32, battery_id: crate::model::EntityId) -> Op {
    AddElectricalLoadCenterBattery { id, index, battery_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for AddElectricalLoadCenterBattery
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<AddElectricalLoadCenterBattery> + From<crate::RemoveElectricalLoadCenterBattery>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add battery {} to electrical load center {}", self.battery_id.0, self.id.0), &format!("Batterie {} zu Stromverteiler {} hinzufügen", self.battery_id.0, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for AddElectricalLoadCenterBattery
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<AddElectricalLoadCenterBattery> + From<crate::RemoveElectricalLoadCenterBattery>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "electrical-load-center", kind: "add-electrical-load-center-battery", record: "AddedElectricalLoadCenterBattery" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

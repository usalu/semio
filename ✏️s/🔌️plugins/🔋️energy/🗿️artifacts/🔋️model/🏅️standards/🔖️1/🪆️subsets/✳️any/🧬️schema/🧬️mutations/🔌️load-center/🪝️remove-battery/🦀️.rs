//! 🪝️ Energy model mutation — `RemoveElectricalLoadCenterBattery`: Detaches one battery from a load centre; the battery itself survives, unattached.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪝️ `remove-electrical-load-center-battery` payload. Detaches one battery from a load centre; the battery itself survives, unattached.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-electrical-load-center-battery")]
pub struct RemoveElectricalLoadCenterBattery {
    pub id: crate::model::EntityId,
    pub battery_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_electrical_load_center_battery<Op: From<RemoveElectricalLoadCenterBattery>>(id: crate::model::EntityId, battery_id: crate::model::EntityId) -> Op {
    RemoveElectricalLoadCenterBattery { id, battery_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RemoveElectricalLoadCenterBattery
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RemoveElectricalLoadCenterBattery> + From<crate::AddElectricalLoadCenterBattery>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "electrical-load-center", kind: "remove-electrical-load-center-battery", record: "RemovedElectricalLoadCenterBattery" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove battery {} from electrical load center {}", self.battery_id.0, self.id.0), &format!("Batterie {} aus Stromverteiler {} entfernen", self.battery_id.0, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for RemoveElectricalLoadCenterBattery
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<RemoveElectricalLoadCenterBattery> + From<crate::AddElectricalLoadCenterBattery>,
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

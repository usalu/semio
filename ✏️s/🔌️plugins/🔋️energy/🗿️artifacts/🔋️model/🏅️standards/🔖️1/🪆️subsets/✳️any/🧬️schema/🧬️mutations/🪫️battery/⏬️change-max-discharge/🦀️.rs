//! ⏬️ Energy model mutation — `ChangeBatteryMaxDischarge`: Sets maximum discharge power (W) on one battery, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⏬️ `change-battery-max-discharge` payload. Sets maximum discharge power (W) on one battery, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-battery-max-discharge")]
pub struct ChangeBatteryMaxDischarge {
    pub id: crate::model::EntityId,
    pub new_max_discharge_w: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_battery_max_discharge<Op: From<ChangeBatteryMaxDischarge>>(id: crate::model::EntityId, new_max_discharge_w: f64) -> Op {
    ChangeBatteryMaxDischarge { id, new_max_discharge_w }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeBatteryMaxDischarge
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeBatteryMaxDischarge>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "battery", kind: "change-battery-max-discharge", record: "ChangedBatteryMaxDischarge" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Battery Max Discharge of battery {}", self.id.0), &format!("Maximale Entladeleistung von Batterie {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

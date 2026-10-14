//! 🧻️ Energy model mutation — `ChangeHumidistatDehumidifyingThrottleRange`: Sets the proportional band the dehumidifying setpoint is approached over, in percent relative humidity.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧻️ `change-humidistat-dehumidifying-throttle-range` payload. Sets the proportional band the dehumidifying setpoint is approached over, in percent relative humidity.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-humidistat-dehumidifying-throttle-range")]
pub struct ChangeHumidistatDehumidifyingThrottleRange {
    pub id: crate::model::EntityId,
    pub new_dehumidifying_throttle_range: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_humidistat_dehumidifying_throttle_range(id: crate::model::EntityId, new_dehumidifying_throttle_range: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeHumidistatDehumidifyingThrottleRange(ChangeHumidistatDehumidifyingThrottleRange { id, new_dehumidifying_throttle_range })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeHumidistatDehumidifyingThrottleRange {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "humidistat", kind: "change-humidistat-dehumidifying-throttle-range", record: "ChangedHumidistatDehumidifyingThrottleRange" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change humidistat {} dehumidifying throttle range to {:?}", self.id.0, self.new_dehumidifying_throttle_range), &format!("Entfeuchtungs-Proportionalbereich von Feuchteregler {} auf {:?} ändern", self.id.0, self.new_dehumidifying_throttle_range))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

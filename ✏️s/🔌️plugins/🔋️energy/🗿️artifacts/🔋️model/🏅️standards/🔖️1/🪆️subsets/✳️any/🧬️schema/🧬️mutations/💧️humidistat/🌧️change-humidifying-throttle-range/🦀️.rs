//! 🌧️ Energy model mutation — `ChangeHumidistatHumidifyingThrottleRange`: Sets the proportional band the humidifying setpoint is approached over, in percent relative humidity.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌧️ `change-humidistat-humidifying-throttle-range` payload. Sets the proportional band the humidifying setpoint is approached over, in percent relative humidity.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-humidistat-humidifying-throttle-range")]
pub struct ChangeHumidistatHumidifyingThrottleRange {
    pub id: crate::model::EntityId,
    pub new_humidifying_throttle_range: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_humidistat_humidifying_throttle_range<Op: From<ChangeHumidistatHumidifyingThrottleRange>>(id: crate::model::EntityId, new_humidifying_throttle_range: f64) -> Op {
    ChangeHumidistatHumidifyingThrottleRange { id, new_humidifying_throttle_range }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeHumidistatHumidifyingThrottleRange
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeHumidistatHumidifyingThrottleRange>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "humidistat", kind: "change-humidistat-humidifying-throttle-range", record: "ChangedHumidistatHumidifyingThrottleRange" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change humidistat {} humidifying throttle range to {:?}", self.id.0, self.new_humidifying_throttle_range), &format!("Befeuchtungs-Proportionalbereich von Feuchteregler {} auf {:?} ändern", self.id.0, self.new_humidifying_throttle_range))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

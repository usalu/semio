//! 🪔️ Energy model mutation — `ChangeDaylightZoneIlluminanceTarget`: Sets the working-plane illuminance the electric lighting dims toward, in lux.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪔️ `change-daylight-zone-illuminance-target` payload. Sets the working-plane illuminance the electric lighting dims toward, in lux.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-daylight-zone-illuminance-target")]
pub struct ChangeDaylightZoneIlluminanceTarget {
    pub id: crate::model::EntityId,
    pub new_illuminance_target_lux: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_daylight_zone_illuminance_target<Op: From<ChangeDaylightZoneIlluminanceTarget>>(id: crate::model::EntityId, new_illuminance_target_lux: f64) -> Op {
    ChangeDaylightZoneIlluminanceTarget { id, new_illuminance_target_lux }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeDaylightZoneIlluminanceTarget
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeDaylightZoneIlluminanceTarget>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change daylight zone {} illuminance target to {:?}", self.id.0, self.new_illuminance_target_lux), &format!("Soll-Beleuchtungsstärke von Tageslichtzone {} auf {:?} ändern", self.id.0, self.new_illuminance_target_lux))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeDaylightZoneIlluminanceTarget
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeDaylightZoneIlluminanceTarget>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "daylight-zone", kind: "change-daylight-zone-illuminance-target", record: "ChangedDaylightZoneIlluminanceTarget" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

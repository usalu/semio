//! 💡️ Energy model mutation — `CreateLightingGain`: Adds one lighting gain to a zone: the installed power density, its schedule and the radiant/visible/return-air split that decides how much of it reaches the zone air directly.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💡️ `create-lighting-gain` payload. Adds one lighting gain to a zone: the installed power density, its schedule and the radiant/visible/return-air split that decides how much of it reaches the zone air directly.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-lighting-gain")]
pub struct CreateLightingGain {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub schedule_id: crate::schedule::ScheduleId,
    pub watts_per_area: f64,
    pub radiant_fraction: f64,
    pub visible_fraction: f64,
    pub return_air_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_lighting_gain<Op: From<CreateLightingGain>>(
    index: u32,
    id: crate::model::EntityId,
    zone_id: crate::model::EntityId,
    schedule_id: crate::schedule::ScheduleId,
    watts_per_area: f64,
    radiant_fraction: f64,
    visible_fraction: f64,
    return_air_fraction: f64,
) -> Op {
    CreateLightingGain { index, id, zone_id, schedule_id, watts_per_area, radiant_fraction, visible_fraction, return_air_fraction }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateLightingGain
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateLightingGain> + From<crate::DeleteLightingGain>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "lighting-gain", kind: "create-lighting-gain", record: "CreatedLightingGain" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create Lighting Gain {} at index {}", self.id.0, self.index), &format!("Beleuchtungswärmegewinn {} an Index {} erstellen", self.id.0, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateLightingGain
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateLightingGain> + From<crate::DeleteLightingGain>,
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

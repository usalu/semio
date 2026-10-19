//! 🔳️ Energy model mutation — `ChangeIdealLoadsSystemOutdoorAirPerArea`: Sets the ventilation rate the system draws per square metre of floor, in m³/s·m².

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔳️ `change-ideal-loads-system-outdoor-air-per-area` payload. Sets the ventilation rate the system draws per square metre of floor, in m³/s·m².
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-outdoor-air-per-area")]
pub struct ChangeIdealLoadsSystemOutdoorAirPerArea {
    pub id: crate::model::EntityId,
    pub new_outdoor_air_per_area_m3_s_m2: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_ideal_loads_system_outdoor_air_per_area<Op: From<ChangeIdealLoadsSystemOutdoorAirPerArea>>(id: crate::model::EntityId, new_outdoor_air_per_area_m3_s_m2: f64) -> Op {
    ChangeIdealLoadsSystemOutdoorAirPerArea { id, new_outdoor_air_per_area_m3_s_m2 }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeIdealLoadsSystemOutdoorAirPerArea
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeIdealLoadsSystemOutdoorAirPerArea>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change ideal loads system {} outdoor air rate per floor area to {:?}", self.id.0, self.new_outdoor_air_per_area_m3_s_m2), &format!("Außenluftrate pro Geschossfläche von Ideallastsystem {} auf {:?} ändern", self.id.0, self.new_outdoor_air_per_area_m3_s_m2))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeIdealLoadsSystemOutdoorAirPerArea
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeIdealLoadsSystemOutdoorAirPerArea>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ideal-loads-system", kind: "change-ideal-loads-system-outdoor-air-per-area", record: "ChangedIdealLoadsSystemOutdoorAirPerArea" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

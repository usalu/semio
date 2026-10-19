//! 🧍️ Energy model mutation — `ChangeIdealLoadsSystemOutdoorAirPerPerson`: Sets the ventilation rate the system draws per occupant, in m³/s.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧍️ `change-ideal-loads-system-outdoor-air-per-person` payload. Sets the ventilation rate the system draws per occupant, in m³/s.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-outdoor-air-per-person")]
pub struct ChangeIdealLoadsSystemOutdoorAirPerPerson {
    pub id: crate::model::EntityId,
    pub new_outdoor_air_per_person_m3_s: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_ideal_loads_system_outdoor_air_per_person<Op: From<ChangeIdealLoadsSystemOutdoorAirPerPerson>>(id: crate::model::EntityId, new_outdoor_air_per_person_m3_s: f64) -> Op {
    ChangeIdealLoadsSystemOutdoorAirPerPerson { id, new_outdoor_air_per_person_m3_s }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeIdealLoadsSystemOutdoorAirPerPerson
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeIdealLoadsSystemOutdoorAirPerPerson>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change ideal loads system {} outdoor air rate per person to {:?}", self.id.0, self.new_outdoor_air_per_person_m3_s), &format!("Außenluftrate pro Person von Ideallastsystem {} auf {:?} ändern", self.id.0, self.new_outdoor_air_per_person_m3_s))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeIdealLoadsSystemOutdoorAirPerPerson
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeIdealLoadsSystemOutdoorAirPerPerson>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ideal-loads-system", kind: "change-ideal-loads-system-outdoor-air-per-person", record: "ChangedIdealLoadsSystemOutdoorAirPerPerson" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

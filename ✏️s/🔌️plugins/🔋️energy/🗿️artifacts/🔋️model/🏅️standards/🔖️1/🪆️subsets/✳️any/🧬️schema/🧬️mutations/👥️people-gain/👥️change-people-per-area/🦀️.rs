//! 👥️ Energy model mutation — `ChangePeopleGainPeoplePerArea`: Sets occupant density (people/m²) on one people gain, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 👥️ `change-people-gain-people-per-area` payload. Sets occupant density (people/m²) on one people gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-people-gain-people-per-area")]
pub struct ChangePeopleGainPeoplePerArea {
    pub id: crate::model::EntityId,
    pub new_people_per_area: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_people_gain_people_per_area<Op: From<ChangePeopleGainPeoplePerArea>>(id: crate::model::EntityId, new_people_per_area: f64) -> Op {
    ChangePeopleGainPeoplePerArea { id, new_people_per_area }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangePeopleGainPeoplePerArea
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangePeopleGainPeoplePerArea>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change People Gain People Per Area of people gain {}", self.id.0), &format!("Personen pro Fläche von Personenwärmegewinn {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangePeopleGainPeoplePerArea
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangePeopleGainPeoplePerArea>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "people-gain", kind: "change-people-gain-people-per-area", record: "ChangedPeopleGainPeoplePerArea" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

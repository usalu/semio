//! 🌞️ Energy model mutation — `ChangePeopleGainSensibleFraction`: Sets sensible fraction on one people gain, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌞️ `change-people-gain-sensible-fraction` payload. Sets sensible fraction on one people gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-people-gain-sensible-fraction")]
pub struct ChangePeopleGainSensibleFraction {
    pub id: crate::model::EntityId,
    pub new_sensible_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_people_gain_sensible_fraction<Op: From<ChangePeopleGainSensibleFraction>>(id: crate::model::EntityId, new_sensible_fraction: f64) -> Op {
    ChangePeopleGainSensibleFraction { id, new_sensible_fraction }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangePeopleGainSensibleFraction
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangePeopleGainSensibleFraction>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change People Gain Sensible Fraction of people gain {}", self.id.0), &format!("Sensiblen Anteil von Personenwärmegewinn {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangePeopleGainSensibleFraction
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangePeopleGainSensibleFraction>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "people-gain", kind: "change-people-gain-sensible-fraction", record: "ChangedPeopleGainSensibleFraction" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

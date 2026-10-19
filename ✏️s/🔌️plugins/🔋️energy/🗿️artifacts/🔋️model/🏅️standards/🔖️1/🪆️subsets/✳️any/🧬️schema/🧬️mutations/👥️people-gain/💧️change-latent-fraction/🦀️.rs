//! 💧️ Energy model mutation — `ChangePeopleGainLatentFraction`: Sets latent fraction on one people gain, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💧️ `change-people-gain-latent-fraction` payload. Sets latent fraction on one people gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-people-gain-latent-fraction")]
pub struct ChangePeopleGainLatentFraction {
    pub id: crate::model::EntityId,
    pub new_latent_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_people_gain_latent_fraction<Op: From<ChangePeopleGainLatentFraction>>(id: crate::model::EntityId, new_latent_fraction: f64) -> Op {
    ChangePeopleGainLatentFraction { id, new_latent_fraction }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangePeopleGainLatentFraction
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangePeopleGainLatentFraction>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change People Gain Latent Fraction of people gain {}", self.id.0), &format!("Latenten Anteil von Personenwärmegewinn {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangePeopleGainLatentFraction
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangePeopleGainLatentFraction>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "people-gain", kind: "change-people-gain-latent-fraction", record: "ChangedPeopleGainLatentFraction" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

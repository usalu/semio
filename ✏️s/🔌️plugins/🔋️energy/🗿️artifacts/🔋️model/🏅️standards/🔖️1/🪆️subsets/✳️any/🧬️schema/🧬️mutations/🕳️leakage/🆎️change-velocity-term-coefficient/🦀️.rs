//! 🆎️ Energy model mutation — `ChangeInfiltrationVelocityTermCoefficient`: Sets the wind velocity term coefficient C on one infiltration, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🆎️ `change-infiltration-velocity-term-coefficient` payload. Sets the wind velocity term coefficient C on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-velocity-term-coefficient")]
pub struct ChangeInfiltrationVelocityTermCoefficient {
    pub id: crate::model::EntityId,
    pub new_velocity_term_coefficient: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_infiltration_velocity_term_coefficient<Op: From<ChangeInfiltrationVelocityTermCoefficient>>(id: crate::model::EntityId, new_velocity_term_coefficient: f64) -> Op {
    ChangeInfiltrationVelocityTermCoefficient { id, new_velocity_term_coefficient }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeInfiltrationVelocityTermCoefficient
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeInfiltrationVelocityTermCoefficient>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "infiltration", kind: "change-infiltration-velocity-term-coefficient", record: "ChangedInfiltrationVelocityTermCoefficient" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Infiltration Velocity Term Coefficient of infiltration {}", self.id.0), &format!("Linearen Windkoeffizienten von Infiltration {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeInfiltrationVelocityTermCoefficient
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeInfiltrationVelocityTermCoefficient>,
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

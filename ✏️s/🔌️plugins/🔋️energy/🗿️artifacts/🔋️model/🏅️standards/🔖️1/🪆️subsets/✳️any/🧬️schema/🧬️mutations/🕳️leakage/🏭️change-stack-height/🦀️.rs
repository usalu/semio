//! 🏭️ Energy model mutation — `ChangeInfiltrationStackHeight`: Sets stack height (m) on one infiltration, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏭️ `change-infiltration-stack-height` payload. Sets stack height (m) on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-stack-height")]
pub struct ChangeInfiltrationStackHeight {
    pub id: crate::model::EntityId,
    pub new_stack_height_m: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_infiltration_stack_height<Op: From<ChangeInfiltrationStackHeight>>(id: crate::model::EntityId, new_stack_height_m: f64) -> Op {
    ChangeInfiltrationStackHeight { id, new_stack_height_m }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeInfiltrationStackHeight
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeInfiltrationStackHeight>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "infiltration", kind: "change-infiltration-stack-height", record: "ChangedInfiltrationStackHeight" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Infiltration Stack Height of infiltration {}", self.id.0), &format!("Kaminhöhe von Infiltration {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeInfiltrationStackHeight
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeInfiltrationStackHeight>,
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

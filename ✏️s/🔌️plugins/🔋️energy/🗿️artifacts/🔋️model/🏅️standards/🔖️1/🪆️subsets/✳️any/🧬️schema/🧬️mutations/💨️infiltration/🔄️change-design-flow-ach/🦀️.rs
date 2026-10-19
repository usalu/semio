//! 🔄️ Energy model mutation — `ChangeInfiltrationDesignFlowAch`: Sets design flow (air changes per hour) on one infiltration, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔄️ `change-infiltration-design-flow-ach` payload. Sets design flow (air changes per hour) on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-design-flow-ach")]
pub struct ChangeInfiltrationDesignFlowAch {
    pub id: crate::model::EntityId,
    pub new_design_flow_ach: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_infiltration_design_flow_ach<Op: From<ChangeInfiltrationDesignFlowAch>>(id: crate::model::EntityId, new_design_flow_ach: f64) -> Op {
    ChangeInfiltrationDesignFlowAch { id, new_design_flow_ach }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeInfiltrationDesignFlowAch
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeInfiltrationDesignFlowAch>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "infiltration", kind: "change-infiltration-design-flow-ach", record: "ChangedInfiltrationDesignFlowAch" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Infiltration Design Flow Ach of infiltration {}", self.id.0), &format!("Bemessungsluftwechsel von Infiltration {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeInfiltrationDesignFlowAch
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeInfiltrationDesignFlowAch>,
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

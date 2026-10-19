//! 🦋️ Energy model mutation — `ChangeOutdoorAirSystemMinOaFlow`: Sets the outdoor air flow the system always draws, in m³/s.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🦋️ `change-outdoor-air-system-min-oa-flow` payload. Sets the outdoor air flow the system always draws, in m³/s.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-outdoor-air-system-min-oa-flow")]
pub struct ChangeOutdoorAirSystemMinOaFlow {
    pub id: crate::model::EntityId,
    pub new_min_oa_flow_m3_s: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_outdoor_air_system_min_oa_flow<Op: From<ChangeOutdoorAirSystemMinOaFlow>>(id: crate::model::EntityId, new_min_oa_flow_m3_s: f64) -> Op {
    ChangeOutdoorAirSystemMinOaFlow { id, new_min_oa_flow_m3_s }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeOutdoorAirSystemMinOaFlow
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeOutdoorAirSystemMinOaFlow>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "outdoor-air-system", kind: "change-outdoor-air-system-min-oa-flow", record: "ChangedOutdoorAirSystemMinOaFlow" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change outdoor air system {} minimum outdoor air flow to {:?}", self.id.0, self.new_min_oa_flow_m3_s), &format!("Minimalen Außenluftvolumenstrom von Außenluftsystem {} auf {:?} ändern", self.id.0, self.new_min_oa_flow_m3_s))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeOutdoorAirSystemMinOaFlow
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeOutdoorAirSystemMinOaFlow>,
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

//! ⛓️ Energy model mutation — `ChangeOutdoorAirSystemAirLoop`: Moves the outdoor air system to another air loop.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛓️ `change-outdoor-air-system-air-loop` payload. Moves the outdoor air system to another air loop.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-outdoor-air-system-air-loop")]
pub struct ChangeOutdoorAirSystemAirLoop {
    pub id: crate::model::EntityId,
    pub new_air_loop_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_outdoor_air_system_air_loop<Op: From<ChangeOutdoorAirSystemAirLoop>>(id: crate::model::EntityId, new_air_loop_id: crate::model::EntityId) -> Op {
    ChangeOutdoorAirSystemAirLoop { id, new_air_loop_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeOutdoorAirSystemAirLoop
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeOutdoorAirSystemAirLoop>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change outdoor air system {} air loop to {}", self.id.0, self.new_air_loop_id.0), &format!("Luftkreislauf von Außenluftsystem {} auf {} ändern", self.id.0, self.new_air_loop_id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeOutdoorAirSystemAirLoop
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeOutdoorAirSystemAirLoop>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "outdoor-air-system", kind: "change-outdoor-air-system-air-loop", record: "ChangedOutdoorAirSystemAirLoop" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

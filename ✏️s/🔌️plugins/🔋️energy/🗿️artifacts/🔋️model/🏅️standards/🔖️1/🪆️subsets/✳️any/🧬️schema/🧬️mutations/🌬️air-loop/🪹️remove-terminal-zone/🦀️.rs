//! 🪹️ Energy model mutation — `RemoveAirLoopTerminalZone`: Takes one zone off the air loop's terminal list. Refused when the loop does not serve it.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪹️ `remove-air-loop-terminal-zone` payload. Takes one zone off the air loop's terminal list. Refused when the loop does not serve it.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-air-loop-terminal-zone")]
pub struct RemoveAirLoopTerminalZone {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_air_loop_terminal_zone<Op: From<RemoveAirLoopTerminalZone>>(id: crate::model::EntityId, zone_id: crate::model::EntityId) -> Op {
    RemoveAirLoopTerminalZone { id, zone_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RemoveAirLoopTerminalZone
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RemoveAirLoopTerminalZone> + From<crate::AddAirLoopTerminalZone>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove terminal zone {} from air loop {}", self.zone_id.0, self.id.0), &format!("Versorgte Zone {} aus Luftkreislauf {} entfernen", self.zone_id.0, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string(), self.zone_id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for RemoveAirLoopTerminalZone
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<RemoveAirLoopTerminalZone> + From<crate::AddAirLoopTerminalZone>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "air-loop", kind: "remove-air-loop-terminal-zone", record: "RemovedAirLoopTerminalZone" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

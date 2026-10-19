//! 🪺️ Energy model mutation — `AddAirLoopTerminalZone`: Puts one more zone on the air loop's terminal list, at its ascending position.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪺️ `add-air-loop-terminal-zone` payload. Puts one more zone on the air loop's terminal list, at its ascending position.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-air-loop-terminal-zone")]
pub struct AddAirLoopTerminalZone {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn add_air_loop_terminal_zone<Op: From<AddAirLoopTerminalZone>>(id: crate::model::EntityId, zone_id: crate::model::EntityId, index: Option<u32>) -> Op {
    AddAirLoopTerminalZone { id, zone_id, index }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for AddAirLoopTerminalZone
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<AddAirLoopTerminalZone> + From<crate::RemoveAirLoopTerminalZone>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add terminal zone {} to air loop {}", self.zone_id.0, self.id.0), &format!("Versorgte Zone {} zu Luftkreislauf {} hinzufügen", self.zone_id.0, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string(), self.zone_id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for AddAirLoopTerminalZone
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<AddAirLoopTerminalZone> + From<crate::RemoveAirLoopTerminalZone>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "air-loop", kind: "add-air-loop-terminal-zone", record: "AddedAirLoopTerminalZone" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

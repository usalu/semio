//! ↘️ Energy model mutation — `ChangeAirLoopReturnNode`: Sets the node the air loop returns to. Node zero is the unset reading, so it is refused.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ↘️ `change-air-loop-return-node` payload. Sets the node the air loop returns to. Node zero is the unset reading, so it is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-air-loop-return-node")]
pub struct ChangeAirLoopReturnNode {
    pub id: crate::model::EntityId,
    pub new_return_node_id: u32,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_air_loop_return_node<Op: From<ChangeAirLoopReturnNode>>(id: crate::model::EntityId, new_return_node_id: u32) -> Op {
    ChangeAirLoopReturnNode { id, new_return_node_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeAirLoopReturnNode
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeAirLoopReturnNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "air-loop", kind: "change-air-loop-return-node", record: "ChangedAirLoopReturnNode" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change air loop {} return node to {:?}", self.id.0, self.new_return_node_id), &format!("Rückluftknoten von Luftkreislauf {} auf {:?} ändern", self.id.0, self.new_return_node_id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeAirLoopReturnNode
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeAirLoopReturnNode>,
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

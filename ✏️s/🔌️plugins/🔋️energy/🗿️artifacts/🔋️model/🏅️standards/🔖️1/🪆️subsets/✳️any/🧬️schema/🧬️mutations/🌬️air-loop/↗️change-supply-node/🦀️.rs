//! ↗️ Energy model mutation — `ChangeAirLoopSupplyNode`: Sets the node the air loop supplies from. Node zero is the unset reading, so it is refused.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ↗️ `change-air-loop-supply-node` payload. Sets the node the air loop supplies from. Node zero is the unset reading, so it is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-air-loop-supply-node")]
pub struct ChangeAirLoopSupplyNode {
    pub id: crate::model::EntityId,
    pub new_supply_node_id: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_air_loop_supply_node(id: crate::model::EntityId, new_supply_node_id: u32) -> EnergyModelMutation {
    EnergyModelMutation::ChangeAirLoopSupplyNode(ChangeAirLoopSupplyNode { id, new_supply_node_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeAirLoopSupplyNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "air-loop", kind: "change-air-loop-supply-node", record: "ChangedAirLoopSupplyNode" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change air loop {} supply node to {:?}", self.id.0, self.new_supply_node_id), &format!("Zuluftknoten von Luftkreislauf {} auf {:?} ändern", self.id.0, self.new_supply_node_id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

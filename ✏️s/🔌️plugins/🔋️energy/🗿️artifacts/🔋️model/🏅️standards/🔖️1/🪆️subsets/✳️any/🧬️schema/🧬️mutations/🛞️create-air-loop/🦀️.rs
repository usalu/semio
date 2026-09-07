//! 🛞️ Energy model mutation — `CreateAirLoop`: Creates one air loop with its supply and return node ids, its design supply air flow and the zones its terminals serve. The terminal list is kept ascending and free of duplicates so a later membership removal's undo is exact.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛞️ `create-air-loop` payload. Creates one air loop with its supply and return node ids, its design supply air flow and the zones its terminals serve. The terminal list is kept ascending and free of duplicates so a later membership removal's undo is exact.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-air-loop")]
pub struct CreateAirLoop {
    pub id: crate::model::EntityId,
    pub name: String,
    pub supply_node_id: u32,
    pub return_node_id: u32,
    pub design_supply_air_flow_m3_s: f64,
    pub terminal_zone_ids: Vec<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_air_loop(id: crate::model::EntityId, name: String, supply_node_id: u32, return_node_id: u32, design_supply_air_flow_m3_s: f64, terminal_zone_ids: Vec<crate::model::EntityId>) -> EnergyModelMutation {
    EnergyModelMutation::CreateAirLoop(CreateAirLoop { id, name, supply_node_id, return_node_id, design_supply_air_flow_m3_s, terminal_zone_ids })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateAirLoop {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "air-loop", kind: "create-air-loop", record: "CreatedAirLoop" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create air loop {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation

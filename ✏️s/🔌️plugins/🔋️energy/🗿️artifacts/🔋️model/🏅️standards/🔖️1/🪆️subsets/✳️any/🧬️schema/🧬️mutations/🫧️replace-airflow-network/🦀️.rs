//! 🫧️ Energy model mutation — `ReplaceAirflowNetwork`: Swaps the document-root airflow-network singleton whole. `present` false detaches it, so one kind covers both attach and detach. `zone_ids[i]` pairs with `node_ids[i]`: the payload carries the two halves of `AirflowNetworkDefinition::zone_node_ids` as parallel lists because its tuple element type has no `dsl::DslField` (see the ticket ledger's follow-up note).

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🫧️ `replace-airflow-network` payload. Swaps the document-root airflow-network singleton whole. `present` false detaches it, so one kind covers both attach and detach. `zone_ids[i]` pairs with `node_ids[i]`: the payload carries the two halves of `AirflowNetworkDefinition::zone_node_ids` as parallel lists because its tuple element type has no `dsl::DslField` (see the ticket ledger's follow-up note).
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-airflow-network")]
pub struct ReplaceAirflowNetwork {
    pub present: bool,
    pub zone_ids: Vec<u32>,
    pub node_ids: Vec<u32>,
    pub outdoor_node_id: u32,
    pub link_ids: Vec<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_airflow_network(present: bool, zone_ids: Vec<u32>, node_ids: Vec<u32>, outdoor_node_id: u32, link_ids: Vec<u32>) -> EnergyModelMutation {
    EnergyModelMutation::ReplaceAirflowNetwork(ReplaceAirflowNetwork { present, zone_ids, node_ids, outdoor_node_id, link_ids })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ReplaceAirflowNetwork {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "airflow-network", kind: "replace-airflow-network", record: "ReplacedAirflowNetwork" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        if self.present { format!("Replace airflow network with {} zone nodes", self.zone_ids.len()) } else { "Detach the airflow network".to_string() }
    }
}
//#endregion 🔖️Mutation

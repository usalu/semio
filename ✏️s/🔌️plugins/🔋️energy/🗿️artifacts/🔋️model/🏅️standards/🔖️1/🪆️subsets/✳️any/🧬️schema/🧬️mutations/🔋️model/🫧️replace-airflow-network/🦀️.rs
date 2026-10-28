//! 🫧️ Energy model mutation — `ReplaceAirflowNetwork`: Swaps the document-root airflow-network singleton whole. `present` false detaches it, so one kind covers both attach and detach. `zone_ids[i]` pairs with `node_ids[i]`: the payload carries the two halves of `AirflowNetworkDefinition::zone_node_ids` as parallel lists because its tuple element type has no `dsl::DslField` (see the ticket ledger's follow-up note).

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🫧️ `replace-airflow-network` payload. Swaps the document-root airflow-network singleton whole. `present` false detaches it, so one kind covers both attach and detach. `zone_ids[i]` pairs with `node_ids[i]`: the payload carries the two halves of `AirflowNetworkDefinition::zone_node_ids` as parallel lists because its tuple element type has no `dsl::DslField` (see the ticket ledger's follow-up note).
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
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

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn replace_airflow_network<Op: From<ReplaceAirflowNetwork>>(present: bool, zone_ids: Vec<u32>, node_ids: Vec<u32>, outdoor_node_id: u32, link_ids: Vec<u32>) -> Op {
    ReplaceAirflowNetwork { present, zone_ids, node_ids, outdoor_node_id, link_ids }.into()
}

//#endregion 🔖️Mutation

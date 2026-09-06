//! 🔺️ Sparse diff builder for `ReplaceAirflowNetwork` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceAirflowNetwork, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if payload.zone_ids.len() != payload.node_ids.len() {
        return protocol::MutationOutcome::error("mutation.invalid-payload", format!("An airflow network pairs one node id per zone id, got {} zone ids and {} node ids.", payload.zone_ids.len(), payload.node_ids.len()), Vec::<String>::new());
    }
    if !payload.present && !(payload.zone_ids.is_empty() && payload.link_ids.is_empty()) {
        return protocol::MutationOutcome::error("mutation.invalid-payload", "A detached airflow network carries no zone nodes and no links.", Vec::<String>::new());
    }
    let network = payload.present.then(|| crate::model::AirflowNetworkDefinition {
        zone_node_ids: payload.zone_ids.iter().copied().zip(payload.node_ids.iter().copied()).map(|(zone, node)| (crate::model::EntityId(zone), node)).collect(),
        outdoor_node_id: payload.outdoor_node_id,
        link_ids: payload.link_ids.clone(),
    });
    if base.model.airflow_network == network {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The airflow network already has this value.");
    }
    let mut model = base.model.clone();
    model.airflow_network = network;
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff

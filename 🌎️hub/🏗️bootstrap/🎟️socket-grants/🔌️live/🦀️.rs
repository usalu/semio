//! 🛣️ The `socket_grants` handlers that act over `HubState`.

use semio_hub_state::HubState;
use semio_hub_ledgers::socket_grants::{SocketBindingGuardV1, SocketBindingKeyV1, SocketBindingValidityV1, SocketGrantRecordV1};
use crate::document_socket::document_plan_socket_validity;
use semio_hub_session_routes::access_grants::socket_binding_validity;

//#region 🔖️State
pub(crate) async fn socket_live_authority(state: &HubState, record: &SocketGrantRecordV1, live_id: &str) -> Result<Vec<SocketBindingGuardV1>, SocketBindingValidityV1> {
    socket_live_authority_with_bindings(state, record, live_id, record.bindings()).await
}

pub(crate) async fn socket_live_authority_with_bindings(state: &HubState, record: &SocketGrantRecordV1, live_id: &str, bindings: Vec<SocketBindingKeyV1>) -> Result<Vec<SocketBindingGuardV1>, SocketBindingValidityV1> {
    let admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.share_bindings(bindings)).await.map_err(|_| SocketBindingValidityV1::Unavailable)?;
    let validity = socket_binding_validity(state, &record.subject, &record.audience).await;
    if validity != SocketBindingValidityV1::Active {
        return Err(validity);
    }
    let plan_validity = document_plan_socket_validity(state, record, None).await;
    if plan_validity != SocketBindingValidityV1::Active {
        return Err(plan_validity);
    }
    if !state.socket_grants.is_live(record, live_id) {
        return Err(SocketBindingValidityV1::Unauthorized);
    }
    Ok(admission)
}
//#endregion 🔖️State

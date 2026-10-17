//! 🪪️ Arc delegate of the gateway backend: the delegating implementation for a shared workspace. See [`super`].

use super::*;

/// 🔌️ Delegates to the inner `HeadlessWorkspace` — legal under the orphan rule because
/// `GatewayBackend` is a LOCAL trait (this crate's `🧭️protocol` facet), even though `Arc` is foreign.
/// Lets `bin.rs`/`build_server_with_workspace` share ONE live `Arc<HeadlessWorkspace>` between the
/// `GatewayBackend` slot `McpServer::new` takes and every other place this packet needs the same
/// workspace (the `context_resolve` tool override, `open_artifact_channel`) — never two divergent
/// workspace instances answering for the same open folder/hub.
impl GatewayBackend for Arc<HeadlessWorkspace> {
    fn resolve_context(&self, principal: &str) -> Result<ContextSummary, GatewayError> {
        (**self).resolve_context(principal)
    }

    fn search_capabilities(&self, query: &str) -> Result<Vec<SearchHit>, GatewayError> {
        (**self).search_capabilities(query)
    }

    fn describe_capabilities(&self, capability_id: &str) -> Result<serde_json::Value, GatewayError> {
        (**self).describe_capabilities(capability_id)
    }

    fn prepare_action(&self, capability_id: &str, input: serde_json::Value, expected_revision: Option<RevisionStamp>) -> Result<PreparedActionReport, GatewayError> {
        (**self).prepare_action(capability_id, input, expected_revision)
    }

    fn invoke_action(&self, prepared_handle: &str, idempotency_key: Option<&str>) -> Result<InvocationReport, GatewayError> {
        (**self).invoke_action(prepared_handle, idempotency_key)
    }

    fn read_resource(&self, uri: &str) -> Result<Vec<ResourceContent>, GatewayError> {
        (**self).read_resource(uri)
    }

    fn list_resources(&self) -> Result<Vec<Resource>, GatewayError> {
        (**self).list_resources()
    }
}

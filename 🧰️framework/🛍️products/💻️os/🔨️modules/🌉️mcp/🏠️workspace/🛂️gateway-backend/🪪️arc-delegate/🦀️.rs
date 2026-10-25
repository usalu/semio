//! 🪪️ Arc delegate of the gateway backend: the delegating implementation for a shared workspace. See [`super`].

use super::*;

/// 🔌️ Delegates to the inner `HeadlessWorkspace` — legal under the orphan rule because
/// `SharedWorkspace` is a LOCAL newtype of the foreign `Arc`.
/// Lets `bin.rs`/`build_server_with_workspace` share ONE live `Arc<HeadlessWorkspace>` between the
/// `GatewayBackend` slot `McpServer::new` takes and every other place this packet needs the same
/// workspace (the `context_resolve` tool override, `open_artifact_channel`) — never two divergent
/// workspace instances answering for the same open folder/hub.
/// 🔌️ The shared workspace as a gateway backend slot, a local type because the backend trait lives in the wire crate.
pub struct SharedWorkspace(pub Arc<HeadlessWorkspace>);

impl GatewayBackend for SharedWorkspace {
    fn resolve_context(&self, principal: &str) -> Result<ContextSummary, GatewayError> {
        self.0.resolve_context(principal)
    }

    fn search_capabilities(&self, query: &str) -> Result<Vec<SearchHit>, GatewayError> {
        self.0.search_capabilities(query)
    }

    fn describe_capabilities(&self, capability_id: &str) -> Result<serde_json::Value, GatewayError> {
        self.0.describe_capabilities(capability_id)
    }

    fn prepare_action(&self, capability_id: &str, input: serde_json::Value, expected_revision: Option<RevisionStamp>) -> Result<PreparedActionReport, GatewayError> {
        self.0.prepare_action(capability_id, input, expected_revision)
    }

    fn invoke_action(&self, prepared_handle: &str, idempotency_key: Option<&str>) -> Result<InvocationReport, GatewayError> {
        self.0.invoke_action(prepared_handle, idempotency_key)
    }

    fn read_resource(&self, uri: &str) -> Result<Vec<ResourceContent>, GatewayError> {
        self.0.read_resource(uri)
    }

    fn list_resources(&self) -> Result<Vec<Resource>, GatewayError> {
        self.0.list_resources()
    }
}

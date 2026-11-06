# Design Notes

## Preserved Source Rationale

<a id="source-notes-5"></a>

### 🦀️.rs:5

🔌️ Delegates to the inner `HeadlessWorkspace` — legal under the orphan rule because
`SharedWorkspace` is a LOCAL newtype of the foreign `Arc`.
Lets `bin.rs`/`build_server_with_workspace` share ONE live `Arc<HeadlessWorkspace>` between the
`GatewayBackend` slot `McpServer::new` takes and every other place this packet needs the same
workspace (the `context_resolve` tool override, `open_artifact_channel`) — never two divergent
workspace instances answering for the same open folder/hub.
🔌️ The shared workspace as a gateway backend slot, a local type because the backend trait lives in the wire crate.

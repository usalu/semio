# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🧠️ Context broker + resource projection — packet `P2-catalog`/`W2-workspace-resources`,
`📋️master.md` §3.5. `context.resolve` returns the token-cheap `ContextSummary` (P1a's `🧬️schema`);
`WorkspaceResourceRegistry` (this file's `resources/list`/`resources/read` implementor) serves the
UNION of `semio://capability[/{id}]` (real, from the compiled `Catalog`, always) and
`semio://workspace[/artifacts]`/`semio://artifact/{id}[/…]` (real, delegated to the bound
`GatewayBackend` — progressive: with no workspace bound the URIs are still LISTED, but a `read`
against one is a structured, retryable `PLUGIN_UNAVAILABLE` naming the binding it needs, never
fabricated content — this module's brief §2.3: "do not fake workspace data").

<a id="source-notes-51"></a>

### 🦀️.rs:51

🗂️ The real `ResourceRegistry` this crate registers into `McpServer` — the UNION of the compiled
`Catalog`'s resources (`semio://capability`[/{id}], token-budgeted via `truncate_to_budget`,
unchanged from this file's earlier `CatalogResourceRegistry`) and, when a workspace is bound,
everything the live `GatewayBackend` reports (`semio://workspace`, `semio://workspace/artifacts`,
`semio://artifact/{id}[/…]`). Holds the SAME `Arc<HeadlessWorkspace>` `McpServer::backend` wraps
(`build_server_with_workspace` clones it once more — a cheap refcount bump, never a second,
divergent `HeadlessWorkspace` instance answering for the same folder/hub).

Every URI has exactly one owner, so `list` never reports one twice: this registry declares the
tier-independent `semio://capability`, `semio://workspace` and `semio://workspace/artifacts`, the
bound backend lists only its live per-document resources.

`list`/`templates` never depend on whether a workspace is bound — mirrors `🦀️.rs`'s
`DECLARED_STUB_TOOL_NAMES` convention: a resource's PRESENCE never depends on tier, only its
`read` RESULT does. With no workspace bound, a read against a workspace URI is a structured,
retryable `PLUGIN_UNAVAILABLE` naming the binding it needs (`--folder`/`--hub`) — never the
fabricated body this region used to synthesize. `subscribe`/`unsubscribe` stay accepted no-ops:
`GatewayBackend` itself declares no change-stream method for either registry to delegate to.

<a id="source-notes-199"></a>

### 🦀️.rs:199

🔔️ Validates that `uri` is one THIS registry can serve — the subscription set itself lives on
the connection (`McpServer::subscriptions`), because a subscription is per-client and this
registry is shared. `Ok(())` therefore means "this URI exists and changes to it will be
pushed", and a URI nothing here projects is a loud `NOT_FOUND` rather than the silent
forever-wait the unconditional `Ok(())` used to promise
(`📓️g7-mcp-agent-and-collaboration-audit.md` §6 P1.6).

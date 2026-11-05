
## 🆕️ Creates `artifact_id` as a REAL artifact of `schema`, seeded from the owning plugin's own

freshly-opened document (`AppCommand::ReadArtifact` → the guest's `ReadDocument`) and persisted
under that plugin's real schema id. The bytes are host-opaque throughout — this crate never
decodes a plugin document, it moves the guest's own pack+spr into the folder event log the
same way `store::sync` does, which is why `kind` can finally mean something without this host
owning a single plugin type.

⏳️ Every activation phase it runs is reported to `scope`, and a cancel observed at any phase
boundary — the last one is just before persisting — creates nothing.

## 🧭️ `capability_id` → its owning plugin id — the routing key `open_routing_channel`'s

`RoutingArtifactChannel` resolves a `PureCommand` through, and what `prepare_action` below
resolves FIRST (before any channel exchange) both to fail fast on a bad capability id and to
pick the right `plugin_instance_slot`. Replaces the deleted `resolve_default_plugin_id`'s
"exactly one plugin or bust": multiple simultaneously-registered plugins are no longer
ambiguous — each capability id routes to exactly the plugin its own catalog entry names.

## 🎬️ Lazily builds (and caches) the real `ActionAdapter` `prepare_action`/`invoke_action`

delegate to — fresh, in-process `HandleTable`/`IdempotencyStore`/`InMemoryAuditSink` (this
workspace's own private mutation-protocol bookkeeping, never shared with `🦀️.rs`'s
separate root-owned adapter the `action_prepare`/`action_invoke` MCP tools already drive
directly — see this struct's own field doc for why duplicating that wiring here is fine: nothing
else in this crate calls `GatewayBackend::prepare_action`/`invoke_action` today). Never fails to
build on its own any more — a `RoutingArtifactChannel` opens no plugin until a command actually
names one, so "which plugin" errors surface per-call (`plugin_id_for`), not at construction.

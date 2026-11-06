# Design Notes

## Preserved Source Rationale

<a id="source-notes-11"></a>

### 🦀️.rs:11

🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P2-catalog: the glob re-exports
above already bring every facet's public items into this module's namespace unqualified (matching
this file's pre-existing convention — `StdioTransport`/`GatewayError`/`HttpTransportOptions` below
are never module-qualified either), so P2's own new code below deliberately stays unqualified too;
the one real ambiguity this crate now has (P1a's `schema::SearchHit` vs this packet's own BM25
ranking type) is resolved by naming the latter `RankedHit`, not by qualifying paths.
🎬️ packet P6-actions-policy: `actions`/`policy` add no further name collisions (`ActionAdapter`/
`ArtifactChannel`/`UnboundArtifactChannel`/`InvokeRequest`/`SagaReport`/`UndoRedoReport` and
`AgentPrincipal`/`PolicyEngine`/`AutoApprovePolicy`/`ApprovalGate` are all novel names crate-wide).

<a id="source-notes-21"></a>

### 🦀️.rs:21

🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P2-catalog: the small, stable
direct-tool core `📋️master.md` §"MCP tool names" names (`context_resolve`, `capabilities_search`,
`capabilities_describe`) — built as real `CapabilityDefinition`s (not just ad hoc `Tool`s)
so they compile INTO the catalog like every other capability (searchable, describable,
`semio://capability/{id}`-readable) and their `input_schema`/`output_schema`/`title`/`description`
have exactly one source of truth, read back by `tool_from_capability` below.

<a id="source-notes-116"></a>

### 🦀️.rs:116

🗂️ Compiles the live gateway's catalog from the REAL installed plugin registry
(`registry::discover_catalog_source`, ticket 26/08/29/AI-MCP-END-TO-END packet W1) — never the
hand-written `🧫️note_and_cad_source()` fixture, which stays reserved for
`🗂️catalog`/`🔎️search`/`🧠️context`/`🧪️conformance`'s own tests. `discover_catalog_source(None)`
resolves the repo/space root itself and degrades to gateway-only capabilities (never a panic,
never a fallback to fixture data) when no plugin is installed or discoverable; if a real install
somehow yields a colliding capability id across two plugins, `compile()`'s `Err` degrades the same
way — a running gateway server with SOME broken plugin data must still start and serve its core
tools, never crash on launch.

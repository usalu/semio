# 🌉️ `semio-os` — the OS's machine interface

One MCP server that exposes the whole OS to an LLM client, as a policy-enforcing façade over the
*same* semantic capabilities the UI uses. It is not a second application runtime: it never touches a
store or a React state directly, and every write it performs travels the ordinary action → mutation →
VCS → backbone path, so a live shell sees an agent's edit exactly as it sees a human collaborator's.

## Run it

```bash
bun ./📜️script.ts dev mcp stdio os          # what .mcp.json launches
bun ./📜️script.ts dev mcp http os           # Streamable HTTP, port 6300
bun ./📜️script.ts dev mcp stdio os -- --folder <space> --scopes artifact.read,artifact.write
```

`.mcp.json` exposes exactly two servers: `repo` (this codebase's own tooling) and `semio` (this).

## Dual-era protocol — deliberate, not legacy baggage

MCP `2026-07-28` is **stateless**: no `initialize`, a per-request `_meta` protocol version, and a
required `server/discover`. Every client that exists today — the installed `@modelcontextprotocol/sdk`
(1.30.0, capped at `2025-11-25`), the IDE clients — speaks the older handshake era. The spec's own
compatibility matrix makes a **dual-era server** the only configuration that works for both, and
explicitly blesses it. We serve `["2026-07-28", "2025-11-25", "2025-06-18"]`, choosing the era from
how the client opens, with a single handler layer beneath. Do not "simplify" this away.

## Surface

Twenty stable tools; the long tail of plugin capabilities is reached through the catalog rather than
by advertising thousands of tools:

- discovery — `capabilities_search`, `capabilities_describe`, `context_resolve`
- authoring — `action_prepare`, `action_invoke`, `action_cancel`, `transaction_begin|commit|rollback`
- history — `history_undo`, `history_redo`
- artifacts — `artifact_create|open|validate|export|snapshot`
- jobs / UI — `job_get`, `job_cancel`, `ui_focus`, `ui_reveal`

Resources are `semio://…` URIs (workspace, artifact + schema/snapshot/selection/validation/history/diff,
window, ui/active-context, capability, plugin, extension, transaction, job, audit).

## Mutation protocol

`Observe → Prepare → Preview → Approve → Commit → Verify`, over the existing channel frames:
`PureCommand` is a true dry-run, the commit is a two-phase `TransactionPrepare`/`TransactionCommit`
(so a stale `expectedRevision` surfaces as `REVISION_CONFLICT` rather than a lost update), and an
`undoToken` maps to `TransactionUndo{group_id}`. Idempotency keys make a retried invoke replay its
stored report instead of mutating twice.

## Safety

The agent is an ordinary OS principal, never an administrator. Its scopes map onto the kernel
`Broker`'s `CapabilityId`s, a capability whose declared scopes exceed the principal's is refused with
`PERMISSION_DENIED` **and** an audit row, destructive capabilities require approval (MCP elicitation
when the client supports it, otherwise a parked request surfaced in the shell's approvals dialog), and
`ui.raw.*` is a separate privileged scope rather than a convenience. Plugin-authored text is treated
as untrusted data: it can influence search ranking, never policy.

## Layout

| facet | role |
|---|---|
| `🧭️protocol` | JSON-RPC, dual-era lifecycle, tool/resource/prompt registries |
| `🚚️transport` | stdio + Streamable HTTP (axum), Origin + bearer checks |
| `🗂️catalog` / `🔎️search` | `CapabilityDefinition` compilation from manifests, deterministic BM25 |
| `🧠️context` | context broker, resource projection, token budgeting |
| `🎬️actions` / `🛡️policy` | the mutation lifecycle; scopes, approvals, quotas |
| `🎫️handles` / `📒️audit` | handle table + idempotency; the append-only audit lane |
| `🧵️bridge` | the loopback WebSocket a live shell dials, Rust SSOT + TS twin codec |
| `🏠️workspace` | headless workspace (actor kernel + wasmtime + artifact host) |

Schema shape is enforced at one choke point (`ToolRegistry::register`): boolean sub-schemas are
normalised to their object form and draft-07 documents are converted to 2020-12, because the official
SDK's validation rejects both — a single non-conforming tool takes down the entire `tools/list`.

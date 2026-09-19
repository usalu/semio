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

Twenty-seven stable tools; the long tail of plugin capabilities is reached through the catalog rather than
by advertising thousands of tools:

- discovery — `capabilities_search`, `capabilities_describe`, `context_resolve`
- authoring — `action_prepare`, `action_invoke`, `action_cancel`, `transaction_begin|commit|rollback`
- history — `history_undo`, `history_redo`
- artifacts — `artifact_create|open|validate|export|snapshot`
- inference — `inference_list`, `inference_get`, `inference_run` (any plugin-declared inference
  service, executed in that plugin's own guest), plus the hub-backed job quartet
  `inference_submit|events|cancel|approve`
- jobs / UI — `job_get`, `job_cancel`, `ui_focus`, `ui_reveal`

Resources are `semio://…` URIs. Listed: `capability`, `workspace`, `workspace/artifacts`, one
`artifact/{id}` and one `artifact/{id}/inference` per open artifact, `window`, `ui/active-context`,
`ui/agent-messages`, `ui/selection`. Templated: `capability/{id}`, `artifact/{artifactId}`,
`artifact/{artifactId}/inference/{field}`, `window/{windowId}`, `job/{jobId}`. An artifact's readable
sub-resources are `schema`, `validation`, `history` and `inference[/{field}]` — a hub-origin workspace
answers the first three with a typed, retryable `PLUGIN_UNAVAILABLE` naming what is still missing,
never a fabricated body. A hub-bound workspace additionally lists per-document `descriptor` and
`checkpoint` scope resources.

## No model provider — the agent is the client, not a dependency

There is no LLM or model-provider client in this crate, and there will not be one: CLAUDE.md forbids
runtime dependencies on external libraries, and no model credential exists anywhere in this repo. "AI
integration" here means the opposite direction — the OS exposes itself as a controllable substrate to
whichever external agent the developer already runs (Claude Code, Codex, …) over MCP, and that agent
drives the ordinary action → mutation → VCS → backbone path like any other principal. `inference_run`
is not an exception: it executes a **plugin's own declared inference service** (a native/wasm
computation the plugin ships, e.g. GIS Map's geometry pass or WFC's solver), never a call to a model
provider. The in-shell agent panel is a *view and a steering surface* for that external agent — it
renders the live `tools/call` traffic and sends the human's turns back over `/bridge` — not a chat
client that talks to a model itself.

## Mutation protocol

`Observe → Prepare → Preview → Approve → Commit → Verify`, over the existing channel frames:
`PureCommand` is a true dry-run, the commit is a two-phase `TransactionPrepare`/`TransactionCommit`
(so a stale `expectedRevision` surfaces as `REVISION_CONFLICT` rather than a lost update), and an
`undoToken` maps to `TransactionUndo{group_id}`. Idempotency keys make a retried invoke replay its
stored report instead of mutating twice.

## Safety

The agent is an ordinary OS principal, never an administrator. Its scopes map onto the kernel
`Broker`'s `CapabilityId`s, a capability whose declared scopes exceed the principal's is refused with
`PERMISSION_DENIED` **and** an audit row, and `ui.raw.*` is a separate privileged scope rather than a
convenience. Plugin-authored text is treated as untrusted data: it can influence search ranking, never
policy.

### Approving a destructive capability

A capability whose manifest declares `approval: whenDestructive|always` parks an `appr_` handle
(`🛡️policy`'s `gate_approval`) and `action_invoke` then offers it, in this order, to the only three
actors that may decide (`🛡️policy::ApprovalCoordinator`):

1. **`--auto-approve never|readonly|all`** — a launch-time human decision, parsed off argv by
   `🏗️bootstrap` for both transports. `never` is the default; `readonly` waives the gate only for a
   capability that is neither destructive nor a writer. An auto-approved capability never parks a
   handle at all.
2. **MCP elicitation** — a real `elicitation/create` server→client request, sent only when the
   connected client advertised `capabilities.elicitation` at `initialize`. `accept` with
   `content.approve == true` approves; `decline`, `cancel`, an error response and a client that
   closes are all refusals, never an approval by default. Client lines that arrive while the human is
   deciding are deferred back onto the serve loop in arrival order, never dropped.
3. **The live OS shell** — `GatewayToShell::ApprovalRequested` over `/bridge`, decided by the human in
   the `🤖️AgentApprovals` dialog or inline in `💬️AgentChatPanel`, answered by
   `ShellToGateway::Approval`. Bounded by `SHELL_APPROVAL_TIMEOUT_MS`; a shell that never answers
   times out into a refusal, never into an approval.

With none of the three available, `action_invoke` answers `APPROVAL_REQUIRED` whose `details` name
every lane it tried, why each was closed, and the remedy. It is never a silent proceed and never a
silent block.

**There is deliberately no `approval_resolve`/`action_approve` MCP tool.** Every MCP tool is callable
by the agent and by nobody else, so such a tool would let an agent approve its own destructive action —
exactly what the gate exists to prevent. `action_invoke`'s `approvalHandle` input remains what it always
was: a way to replay a decision one of the three actors above already made.

### Binding a workspace

A gateway launched with neither `--folder <dir>` nor `--hub <url> --space <id>` runs on
`UnboundArtifactChannel`: every mutation-protocol call answers a typed, retryable `PLUGIN_UNAVAILABLE`
naming both flags. The scripted `MockArtifactChannel` is `#[cfg(test)]` and reachable from no
production path, so a call can never run against a stand-in that looks like a real commit. Every client
config this repo ships (`.mcp.json`, `.cursor/`, `.vscode/`, `.windsurf/`, `.kiro/`, `.codex/`) binds
`--folder .`.

### Attaching a live shell from stdio

`stdio` mode is what every client config launches, and it now offers the same loopback `/bridge` the
`http` transport does. On start it looks for a live os session in
`~/.semio/agent/bridge/sessions/` (`🛰️rendezvous`); finding one, it binds a **bridge-only** listener
(no `/mcp` on that socket — this process's MCP surface is stdin/stdout) and publishes an owner-only
(`0600`) offer in `~/.semio/agent/bridge/offers/<pid>.json` carrying the `ws://` url and a per-process
admission proof, removed when the process exits. Admission is never the hub fd-3 credential: a stdio
gateway inherits none and must not fabricate one. Finding no live session, no listener is bound and
every bridge-dependent tool (`ui_focus`, `ui_reveal`, agent presence, shell approvals) answers a typed
`PLUGIN_UNAVAILABLE` naming the sessions directory and the live-session count as of that call — never a
silent no-op. `--no-bridge` opts out entirely.

## Layout

| facet | role |
|---|---|
| `🧭️protocol` | JSON-RPC, dual-era lifecycle, tool/resource/prompt registries |
| `🚚️transport` | stdio + Streamable HTTP (axum), Origin + bearer checks |
| `🗂️catalog` / `🔎️search` | `CapabilityDefinition` compilation from manifests, deterministic BM25 |
| `🧠️context` | context broker, resource projection, token budgeting |
| `🔀️dispatch` / `🛡️policy` | the mutation lifecycle; scopes, approvals, quotas |
| `🎫️handles` / `📒️audit` | handle table + idempotency; the append-only audit lane |
| `🧵️bridge` | the loopback WebSocket a live shell dials, Rust SSOT + TS twin codec |
| `🛰️rendezvous` | how a stdio gateway and a live os session find each other (`~/.semio/agent/bridge`) |
| `🏠️workspace` | headless workspace (actor kernel + wasmtime + artifact host) |
| `🗿️artifact` / `🖥️ui` | the `artifact_*` tools; the `ui_*`/`job_*` tools and their shell forwarding |
| `💡️inference` | `inference_*` — a plugin's own declared service in its own guest, plus the hub job lane |
| `📇️registry` / `💬️prompts` | live installed-plugin discovery; the protocol-teaching prompt set |
| `🧪️conformance` / `⚠️errors` | the deterministic catalog conformance runner; the twelve frozen error codes |
| `🏗️bootstrap` | the `semio-os-mcp stdio\|http` CLI (`--folder`/`--hub`/`--scopes`/`--auto-approve`/`--no-bridge`) |
| `🧬️schema` | the `os.mcp` schema registry — every wire type and every tool `inputSchema`/`outputSchema` |

Schema shape is enforced at one choke point (`ToolRegistry::register`): boolean sub-schemas are
normalised to their object form and draft-07 documents are converted to 2020-12, because the official
SDK's validation rejects both — a single non-conforming tool takes down the entire `tools/list`.

## Schemas

Every schema this scope publishes lives in `🧬️schema/🦀️.rs`'s `schemas()` — the gateway wire types,
the MCP protocol types (mirroring the external spec), and every tool `inputSchema`/`outputSchema`
shape the facets stamp a `semio://capability/{id}/{input|output}` `$id` onto. No facet declares a
schema of its own.

`🧬️schema/🔣️.json` (draft-07, `$id: https://json.schemas.assets.semio-tech.com/os/mcp/component.json`) and
`🧬️schema/🟦️.ts` (types plus a dependency-free `parse<ExportId>` per export) are **generated** from
that registry by `bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror`, which runs the binary's
own `semio-os-mcp schemas` emitter; `schema-mirror-check` fails on drift, and the Rust law
`the_json_mirror_publishes_exactly_the_registry_exports` asserts the two key sets agree.

Nested modules own their own contracts the same way — `🏠️workspace/🧬️schema/🔣️.json`,
`🏠️workspace/🔗️remote/🧬️schema/🔣️.json` and `💡️inference/🧬️schema/🔣️.json`. Fixtures under
`🧫️fixtures`/`🧪️fixtures` hold data only.

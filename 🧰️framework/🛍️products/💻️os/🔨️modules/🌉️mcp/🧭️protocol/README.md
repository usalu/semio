# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🧭️ Dual-era JSON-RPC 2.0 + MCP protocol core (`📓️design-decisions.md` D1 is the controlling
requirement — read it before touching this file). A single [`McpServer`] dispatcher serves BOTH
eras through ONE handler layer: **modern** (`2026-07-28`+) is stateless — every request carries
its protocol version in `_meta.io.modelcontextprotocol/protocolVersion`, no `initialize`, no
session; **legacy** (`2025-11-25`, `2025-06-18`) negotiates once via `initialize`/
`notifications/initialized`. Which era a connection speaks is decided by its OPENING request
(`server/discover`/`_meta` ⇒ modern, `initialize` ⇒ legacy) and recorded on the dispatcher — every
later request on that connection is routed through the exact same `tools/*`/`resources/*`/
`prompts/*` handlers regardless of era. This is a spec interoperability requirement (every MCP
client installed in this repo today is legacy-era — see D1's SDK survey), not a compatibility
shim: CLAUDE.md's "no legacy support" rule does not apply to an external protocol we do not own.

<a id="source-notes-166"></a>

### 🦀️.rs:166

📨️ A JSON-RPC 2.0 request. `id: None` (the field entirely absent on the wire) makes it a
notification — no response is ever sent for one, success or failure. An explicit `"id":null`
deserializes to `Some(JsonRpcId::Null)`, distinct from absence (see [`deserialize_some`]).
🌉️ The `#[value(...)]` twin of [`deserialize_some`] — same "present key, including an explicit
`null`, always goes through `T::from_value`; only a truly ABSENT key falls back to `default`"
split the derive's own doc calls the `deserialize_double_option` shape.

<a id="source-notes-419"></a>

### 🦀️.rs:419

🌉️ Hand-written, not derived: `#[value(...)]` has no way to express `skip_serializing_if` on an
enum VARIANT's own named field (same gap as `CapabilityOwner`'s hand-written impl in
`🗂️catalog/🦀️.rs` — the derive's internally-tagged codegen never reads that attribute at variant-
field position) — a derive here would silently start writing `name`/`mimeType`/`text`/`blob` as
`null` instead of omitting them, changing the wire shape. Mirrors
`#[serde(tag = "type", rename_all = "snake_case")]` exactly, field-for-field with the
omit-if-`None` behavior serde's own `skip_serializing_if` gives it (and the matching implicit
"missing `Option` field decodes to `None`" serde gives every `Option<T>` field).

<a id="source-notes-855"></a>

### 🦀️.rs:855

🔌️ Everything a real OS/plugin integration provides — later packets (P1b/P2/P6) implement this
against the live plugin host; THIS crate never references `semio-framework`,
`semio-os-kernel`, the plugin host, the channel, or the actor crate (they are mid-rewrite
by the peer `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ticket — §2.6 of this packet's brief).
🔀️ dedyn-fw-os-misc, O1/R11: closed 3-implementor set (`NullBackend`, `HeadlessWorkspace`, its
`Arc<HeadlessWorkspace>` delegation impl) — `#[dyn_enum]` here + `dyn_enum_close!` at
`🏠️workspace`'s `GatewayBackends` (defined alongside `HeadlessWorkspace`, the module both non-Null
variants live in) closes it into an enum instead of `Box<dyn GatewayBackend>`.

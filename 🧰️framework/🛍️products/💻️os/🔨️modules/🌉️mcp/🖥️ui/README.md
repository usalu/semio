# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🖥️ UI information, UI actions, and jobs — ticket 26/08/29/AI-MCP-END-TO-END packet
W6-ui-and-jobs. `ui_focus`/`ui_reveal` forward a `ShellCommand` (JSON-encoded, matching
`💻️os/🔨️modules/🖥️shell`'s own wire vocabulary) to the attached `/bridge` shell as a
`GatewayToShell::ShellCommand` frame and block on its `ShellCommandResult` (bounded by
[`SHELL_COMMAND_TIMEOUT_MS`], never forever). `semio://window[/…]`/`semio://ui/active-context`/
`semio://ui/selection` project the SAME `ShellState` mirror `🧵️bridge::BridgeHandle::last_shell_state`
already holds — never a second, invented source of truth. `job_get`/`job_cancel` read/act through
[`job_registry`], a plugin-agnostic, process-wide job tracker any producer (W5's inference, a
future tessellation job, …) mints ids into via [`JobRegistry::begin`] and reports progress/result
into via [`JobRegistry::report_progress`]/[`JobRegistry::succeed`]/[`JobRegistry::fail`] — this
facet never reaches into the plugin-host reactor's own `job: u64` ids directly (those are
per-guest-instance and require a live `GuestInstance` this crate's tool layer never holds); a
producer that spawns one is the bridge between the two id spaces. Every tool this file registers
stays PRESENT in `tools/list` and every resource stays LISTED regardless of tier — only a `call`/
`read` result varies (bare: no bridge at all; headless: a bridge exists but no shell has dialed
it yet, a normal, retryable state; attached: a live shell). Never a panic, never a protocol-level
failure, never fabricated UI state — see `read_ui_resource`'s doc for the one deliberate honesty
trade-off (`semio://ui/selection` omits per-artifact object selection, which `ShellState` does
not model).

<a id="source-notes-216"></a>

### 🦀️.rs:216

🧭️ The attached shell's last full `ShellState` snapshot, decoded as raw JSON — `🧵️bridge`'s own
design note says `state`/`command` bytes are opaque, AgentBridge encodes/decodes them as JSON on
the renderer side, so this facet mirrors that instead of taking a new compile-time dependency on
`semio-os-shell`'s Rust types (see this file's W6 report for the tradeoff). A
`ShellStatePatch`-only connection (no full snapshot received yet) is a typed, retryable
`PLUGIN_UNAVAILABLE`, never a guess at merging a patch onto nothing.

<a id="source-notes-394"></a>

### 🦀️.rs:394

💬️ The receiving half of the agent chat panel: every turn the human typed at the agent since the
last read, oldest first. A READ DRAINS the queue — MCP resources have no server-push, so the
agent polls this uri, and a turn it has already been handed must not be handed to it twice.
Bare/headless tiers answer the same typed, retryable `PLUGIN_UNAVAILABLE` every other UI resource
does; an attached shell that has simply said nothing answers an empty list, which is a real
answer, not a gap.

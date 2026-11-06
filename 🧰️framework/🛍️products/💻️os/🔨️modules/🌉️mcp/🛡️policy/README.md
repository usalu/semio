# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🛡️ Policy engine — packet `P6-actions-policy`, `📋️master.md` §3.4. `AgentPrincipal` carries the
effective, ALREADY-EXPANDED `kernel::CapabilityId` scope set a session was granted (from the CLI
flags P1a's `StdioOptions`/`HttpOptions` already parse); [`PolicyEngine`] is the pure decision
layer `🔀️dispatch`' `ActionAdapter` calls for scope enforcement and the approval gate. Depends on
`semio_framework::manifest::{kernel, ApprovalMode}` only — the SAME one hop `🗂️catalog` already
takes (D8: no plugin/channel/actor dependency), and on `crate::handles` (P1b) for `HandleTable`/
`Attachment`/`HandleKind`/`SessionHandle`. Zero dependency on `crate::audit` — every decision this
facet makes is reported by ITS CALLER (`🔀️dispatch`, which owns the `AuditSink`), so this facet
stays testable without an audit sink at all.

<a id="source-notes-26"></a>

### 🦀️.rs:26

📇️ `📋️master.md` §3.4's scope table, verbatim — the left column is the CLI-facing MCP scope name
(`--scopes artifact.write,ui.control`), the right column the `kernel::CapabilityId` strings it
expands to. Parametrized families (`fs.read:<root>`, `fs.write:<root>`, `http:<origin>`,
`secrets:<name>`) expand to a `<prefix>*` wildcard grant here — [`AgentPrincipal::grants`] treats a
`<prefix>*` entry as "every concrete scope under this prefix", matching `🧪️conformance`'s own
`KNOWN_SCOPE_PREFIXES` four-family list.

💡️ Local ADMISSION only: it grants the four hub-backed inference job tools the right to be
CALLED by this connection. It is never hub authorization — the hub re-runs its own live
`Author`/session/authorization-generation predicate on accept, claim, every checkpoint,
offer, read, cancel and approval, and an `Admin` is not implicitly allowed there.

💬️ Writing PROSE into the shell's agent panel is neither of its neighbours: it changes no
shell state, opens no window and navigates nothing (so `ui.control`'s four grants are all
wrong for it), yet it does write into a surface a human reads (so `ui.observe` is too weak).
Its own row, expanding to one capability of its own, is the honest shape — and it means a
read-only agent granted `ui.observe,conversation.write` can still answer the human.

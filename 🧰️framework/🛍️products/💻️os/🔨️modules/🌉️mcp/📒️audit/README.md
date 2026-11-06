# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

📒️ Agent audit lane (`📌️sol-P1b-packet.md` §2.3, `📋️master.md` §3.4 verbatim field list) —
append-only [`AgentAuditEvent`] rows behind [`AuditSink`], the seam a later packet's real
event-sourced `os.agent.audit` lane (via `ArtifactHost`) implements instead of [`FileAuditSink`].
**Secrets and full sensitive args never reach a sink**: [`redact_input`] produces `input_redacted`
from the raw call arguments BEFORE anything is constructed, and `mod quick`'s
`sensitive_field_never_reaches_the_sink` test proves it end-to-end through a real [`AuditSink`].

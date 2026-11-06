# Design Notes

## Preserved Source Rationale

<a id="source-notes-5"></a>

### 🦀️.rs:5

#region 💬️AgentConversation
💬️ The gateway's own emitter for the agent-conversation frames — the ONE place `tools/call`
traffic becomes something a shell can render. It is deliberately not a second log: it carries no
state beyond the invocation counter, reads nothing, and publishes only what the real dispatch
path hands it, so the panel shows the agent's actual calls or nothing at all.

Every emission is best-effort by construction. A gateway with no `/bridge` (stdio), a bridge with
no shell attached, or a shell whose outbox is momentarily saturated must never fail or delay a
tool call — the agent's work is the authority, the shell's view of it is a projection.

<a id="source-notes-37"></a>

### 🦀️.rs:37

🛠️ Announces one tool call and returns its invocation id. Emitted BEFORE the handler runs, so
a long tool is visible in the shell while it is still running rather than only afterwards;
the `AgentPresence{active:true}` frame is what turns the existing presence dot amber.

🛑️ Every tool call is a real job in the ONE process-wide registry, keyed by the invocation id
the panel already renders — so the panel's cancel control, `job_cancel` and
`semio://job/{id}` all act on the same record, and a handler that polls
`is_cancel_requested` sees the human's stop. `report_progress` moves it off `Pending`
(a `Pending` job finishes as `Cancelled` immediately; this one is genuinely running).

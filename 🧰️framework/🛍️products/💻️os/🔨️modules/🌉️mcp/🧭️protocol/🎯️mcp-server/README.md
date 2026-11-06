# Design Notes

## Preserved Source Rationale

<a id="source-notes-290"></a>

### 🦀️.rs:290

📈️ Held across the whole call: every job the tool mints while this guard is alive is bound
to the client's token, so its progress is PUSHED as `notifications/progress` instead of
only being readable by polling `job_get`.

💬️ The ONE real dispatch point every tool call passes through — so the shell's agent
panel shows the agent's actual calls, in order, with their real arguments and outcomes,
rather than a second bookkeeping path that could drift from what ran.
💬️ …except for a tool whose own result IS a conversation frame: `conversation_reply`
publishes the agent's prose itself, and a tool-call row carrying the same sentence as its
`arguments` would print every turn twice (`SELF_PUBLISHING_CONVERSATION_TOOLS`).

🔔️ A committed mutation, an undo/redo, a created artifact — the declared table in
`📣️notify` says which tool changed which resource, and every connection subscribed
to one of those URIs is told. This is the one call site that makes
`"resources": {"subscribe": true}` real.

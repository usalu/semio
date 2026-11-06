# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

📣️ Server→client notifications — the one lane every unprompted MCP message leaves this process
through (`notifications/resources/updated`, `notifications/resources/list_changed`,
`notifications/progress`), plus the bookkeeping that decides WHICH connection gets each one.

Three real seams, no polling anywhere:
- [`NotificationSink`]/[`NotificationSlot`] — one per connection, filled by whichever transport
  owns the socket (`StdioLines` writes a line; `HttpEventPublisher` pushes onto the SSE event
  log). A server with no sink bound publishes nothing and costs nothing: every call site below is
  a no-op rather than a branch each caller re-invents.
- [`ResourceSubscriptions`] — one per connection, the live answer to `resources/subscribe`. The
  process-wide [`ResourceUpdateBroker`] holds WEAK references to every live one, so a closed
  connection's set disappears with its server rather than accumulating.
- [`ProgressScope`] — the `_meta.progressToken` of the `tools/call` currently running on THIS
  thread. A job minted inside that scope is bound to the token, so every later
  [`JobRegistry`](crate::ui::JobRegistry) progress report of that job id is pushed as a real
  `notifications/progress`, and an incoming `notifications/cancelled` naming that request id is
  mapped onto the very `job_registry().request_cancel` path `job_cancel` uses.

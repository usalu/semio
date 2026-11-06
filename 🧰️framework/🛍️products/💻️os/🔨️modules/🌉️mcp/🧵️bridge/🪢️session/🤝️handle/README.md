# Design Notes

## Preserved Source Rationale

<a id="source-notes-12"></a>

### 🦀️.rs:12

🖇️ The seam a later packet reaches live `/bridge` connections through WITHOUT this facet
depending on theirs — P6's policy engine routes a parked approval to a connected shell via
[`send_to`](Self::send_to)/[`broadcast`](Self::broadcast); a future `ui.*` tool pushes a
`ShellCommand` the same way; `semio://ui/shell`/`context_resolve` read
[`last_shell_state`](Self::last_shell_state). None of that wiring happens in THIS file
(`📓️sol-P1c-packet.md` §3: "do not wire it into P6's files … just publish the API") — obtain a
`BridgeHandle` from the live [`crate::HttpTransportRun::bridge`] owner; the Axum router is a
test-only differential oracle.

<a id="source-notes-68"></a>

### 🦀️.rs:68

📝️ Records the effect of one received [`ShellToGateway`] frame against its connection —
`Hello`/`Ping`/`Bye` never reach here (the read loop handles all three inline).

🛑️ A cancel is connection-independent and takes no connection state: it flips the SAME
cooperative flag `job_cancel` flips, on the job `AgentConversation::begin_tool_call` opened
under this invocation id. Handled before the connections lock so the two locks never nest.
A `NotFound`/`PreconditionFailed` means the call already finished between the human's click
and this frame — the honest outcome of a cancel arriving late, not an error to report.

💬️ Outside the connections lock: the subscription broker takes its own locks, and a human
turn that only becomes visible on the agent's next poll is a channel with a one-poll
latency floor. Subscribed clients are told now; unsubscribed ones still poll as before.

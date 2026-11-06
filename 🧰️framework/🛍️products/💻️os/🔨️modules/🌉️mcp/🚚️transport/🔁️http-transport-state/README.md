# Design Notes

## Preserved Source Rationale

<a id="source-notes-689"></a>

### 🦀️.rs:689

💬️ A human turn typed at the agent is recorded exactly like every other shell→gateway
observation: onto its own connection, for `semio://ui/agent-messages` to drain.

🛑️ A cancel for a running tool call — `BridgeHandle::record` routes it to the one
process-wide job registry rather than to this connection's own state.

🗿️ The shell's reply to a `GatewayToShell::AppCommand` — the artifact route's own
response frame. It used to terminate the connection as `Unsupported` (nothing on the
gateway had ever sent an `AppCommand`, so nothing could answer one); `🐚️channel`'s
`ShellArtifactChannel` is the consumer that makes it a real reply.

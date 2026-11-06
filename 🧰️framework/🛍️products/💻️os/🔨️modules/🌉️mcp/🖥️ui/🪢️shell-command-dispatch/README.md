# Design Notes

## Preserved Source Rationale

<a id="source-notes-70"></a>

### 🦀️.rs:70

📤️ Encodes `command_json` the same way `AgentBridge`'s renderer-side hook does (opaque JSON
bytes inside `GatewayToShell::ShellCommand.command` — `🧵️bridge`'s own design note: the codec
never interprets these bytes), sends it to the active shell connection, and blocks (bounded by
`timeout`, polling `BridgeHandle::last_command_result`) for a `ShellCommandResult` whose
`in_reply_to` matches the `seq` this call minted. A stale/unrelated result already sitting on the
connection is never mistaken for this call's reply (`in_reply_to` must match exactly).

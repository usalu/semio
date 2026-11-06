# Design Notes

## Preserved Source Rationale

<a id="source-notes-26"></a>

### 🦀️.rs:26

📻️ Newline-delimited JSON-RPC over `input`/`output`, with a THIRD writer (`log`) for every
diagnostic line — generic over the log writer so tests exercise the exact same code path a real
`stdin`/`stdout`/`stderr` wiring uses, in-memory, without touching the process's real file
descriptors. `input`/`output` are owned by the shared [`StdioLines`] channel instead, because a
server-initiated `elicitation/create` issued from inside a tool call must reach the very same
descriptors while the serve loop is suspended in `dispatch`.

<a id="source-notes-90"></a>

### 🦀️.rs:90

🔁️ One line in → zero-or-one line out, until EOF (client closed stdin) or a hard io error. A
blank line is skipped silently (not an error — some clients send a trailing newline). A batch
that dispatches to zero responses (all-notification batch) writes nothing, per JSON-RPC. A line
that is a RESPONSE to a request this server itself issued (elicitation) never reaches here: the
waiting elicitation consumed it, and every other line it saw was deferred back onto this loop.

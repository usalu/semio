# Design Notes

## Preserved Source Rationale

<a id="source-notes-165"></a>

### 🦀️.rs:165

🌐️ Streamable HTTP transport (`📌️sol-P1b-packet.md` §2.1) — a single `/mcp` endpoint: `POST` for
every JSON-RPC request/notification of EITHER era (dispatched through the exact same [`McpServer`]
stdio uses, no era logic duplicated here), `GET` for this gateway's legacy resumable server→client
stream (`Last-Event-ID` replay against [`EventLog`]) — the 2026-07-28 revision itself removed GET
entirely, but this dual-era gateway also serves `2025-11-25`/`2025-06-18` clients, whose Streamable
HTTP shape still has one (`📓️design-decisions.md` D1). Every request is checked against `Origin`
(reject non-loopback/non-`null`/non-allowlisted with `403`) and the bearer token (`401`) before
anything else runs.

<a id="source-notes-204"></a>

### 🦀️.rs:204

🧪️ Builds the real axum [`Router`] WITHOUT binding a socket — the foreground, deterministic
entry point every `/mcp` HTTP test in this crate drives through an owned one-shot adapter (no
port allocation, no background process, no timing races). `/bridge` is a real websocket
upgrade, which `oneshot` cannot drive (it never performs a genuine hyper connection upgrade) —
tests exercising `/bridge` bind a real ephemeral (`:0`) socket instead (`mod long` in this file
and in `🧵️bridge/🦀️.rs`), same as P1a/P1b's own websocket tests already did.

<a id="source-notes-227"></a>

### 🦀️.rs:227

🌉️ Binds the SAME loopback listener, serving `/bridge` only — no `McpServer`, so `/mcp`
answers 404. `run_stdio` uses this so a live os session can attach to a gateway whose MCP
surface is stdin/stdout. The bound port is read back off the run, because the caller binds
`:0` and only then knows what to publish into its rendezvous offer.

🧹️ Nobody drains a bridge-only run's terminal connections (`run_stdio` never calls
[`HttpTransportRun::wait`]), so it closes them itself from the start. Retaining them leaked one
slot per shell connection that ever ended until [`HTTP_CONNECTION_CAPACITY`] was reached, after
which the listener parked for good: every later shell's TCP connect was accepted by the kernel
and never answered (measured 2026-09-26: an 8.6 h old gateway holding exactly 64 CLOSED sockets).

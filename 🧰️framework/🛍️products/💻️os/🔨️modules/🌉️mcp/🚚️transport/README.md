# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🚚️ Transport-level framing over an arbitrary duplex channel — `StdioTransport` (P1a, newline-
delimited JSON-RPC on stdin/stdout) and `HttpTransport` (P1b, retained nonblocking Streamable HTTP
and WebSocket turns on the process-wide WorkerPool I/O lane; Axum/Tokio is test-oracle only,
`📓️luna-mcpspec-audit.md` §A.7 + the spec page fetched live for this packet — resumable legacy GET
is this gateway's own dual-era accommodation, NOT part of the 2026-07-28 revision, which removed
the GET stream endpoint and protocol-level sessions entirely; see this file's `🔖️HttpTransport`
region doc for the exact split). **All diagnostic output goes to a SEPARATE log writer for stdio,
never to the response stream** — a stray byte on stdout corrupts every later line the client tries
to parse as JSON-RPC (`luna-mcpspec-audit.md`'s stdio guidance).

`McpTransport::serve` takes `server: McpServer` BY VALUE for stdio. HTTP transfers the same exact
owner through `HttpTransport::start` into `HttpTransportRun`; the process entry may synchronously
wait on that opaque completion, but no transport or pool turn constructs/drives a second runtime.

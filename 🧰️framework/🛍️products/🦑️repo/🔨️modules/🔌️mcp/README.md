# 🔌️ MCP

The repo Model Context Protocol server is named `repo` — the stdio binary and `serverInfo.name` are both `repo`, for every client profile. It is the developer surface: goals, tickets, and the rest of the repository. It is not the user MCP.

The repo Model Context Protocol server: the bounded JSON-RPC contract, the session state machine, the
line-delimited stdio transport, request routing, the hash-chained `semio.mcp.event/1` log, the
per-IDE profiles and the repository tool, resource and prompt surface. `initialize` accepts members it
does not know, and end of input never unmakes a request the peer already delivered.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-mcp` (the `repo` binary lives in `⌨️cli`)
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/mcp`, entry point in `📦️packages/🐹️go/🚀️bin`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; the Model Context Protocol TypeScript SDK and Node `crypto` are registered as
third-party references in `🔮️oracle/🔣️.json`.

`🤝️jsonrpc-handshake`, `📋️capability-listing`, `📞️tool-call-roundtrip`, `🔗️event-log-chain`.

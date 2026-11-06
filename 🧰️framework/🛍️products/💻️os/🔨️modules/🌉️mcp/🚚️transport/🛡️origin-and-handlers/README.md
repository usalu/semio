# Design Notes

## Preserved Source Rationale

<a id="source-notes-91"></a>

### 🦀️.rs:91

📮️ `POST /mcp` — the single endpoint for every JSON-RPC request/notification, either era. Modern
requests (a body carrying `_meta.protocolVersion`) additionally require the `MCP-Protocol-Version`
header to be PRESENT and to MATCH the body value (`400 HeaderMismatch` otherwise, per the spec page
fetched for this packet — see this file's module doc); legacy requests (`initialize`, or any
already-negotiated legacy call) pass straight through to [`McpServer::dispatch`], exactly as stdio
does. A JSON-RPC notification (absent `id`) is `202 Accepted` with no body; a request gets `200 OK`
`application/json`.

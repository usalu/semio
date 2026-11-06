# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

⚠️ Gateway-domain error type and its JSON-RPC/MCP-tool-result projections, shared by every
capability surface `semio-os-mcp` exposes. The code set mirrors
`📋️master.md` §3.3 verbatim (frozen — a later packet that needs a new failure mode maps it onto
one of these twelve, it does not add a thirteenth). This facet has ZERO dependency on
`crate::protocol` — errors flow outward TO the protocol layer (which builds a JSON-RPC error
response or a `CallToolResult{isError:true}` payload from a `GatewayError`), never the reverse,
so a future backend crate can construct one without pulling in JSON-RPC framing at all.

<a id="source-notes-53"></a>

### 🦀️.rs:53

🔢️ The nearest-fit JSON-RPC 2.0 standard error code for this gateway code when it surfaces as
a PROTOCOL-level failure (as opposed to a tool-result `isError:true` payload) — see
`luna-mcpspec-audit.md` §"Error Codes (JSON-RPC)". Client-fault codes (bad input, unknown
target, conflicting precondition) map to Invalid Params; everything else — the gateway itself
or a downstream plugin/policy refusing — maps to Internal Error, since none of them are the
caller having sent a malformed request.

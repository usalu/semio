# Design Notes

## Preserved Source Rationale

<a id="source-notes-63"></a>

### 🦀️.rs:63

📨️ The bridge's single `ApprovalRequested.summary` string, JSON-encoded in the shape both
approval surfaces parse — `💬️AgentChatPanel`/`🤖️AgentApprovals`' `parseApprovalSummary` and
its wgpu twin `parse_approval_summary`. The frame's wire shape is unchanged (tag 3 carries
one string); this payload IS the schema, and `🧫️fixtures/🛡️approval-summary` is the one file
both parsers are asserted against.

⏱️ `timeoutMs` is a DURATION, never a deadline: the shell starts its countdown when the
frame arrives, so a browser clock that disagrees with the gateway's cannot make an approval
look already-expired (or never-expiring) to the human deciding it.

<a id="source-notes-91"></a>

### 🦀️.rs:91

⛩️ The real resolution chain for a parked approval, in priority order:
1. **MCP elicitation** — the spec's own human-in-the-loop primitive, asked of the CONNECTED client
   (Claude Code, Codex, …), used only when that client advertised `capabilities.elicitation`.
2. **The live OS shell** — `GatewayToShell::ApprovalRequested` over `/bridge`, decided by the human
   in `🤖️AgentApprovals`/`💬️AgentChatPanel`, answered by `ShellToGateway::Approval`.
3. **Neither** — a typed, non-retryable `APPROVAL_REQUIRED` naming both closed lanes.

🔐️ There is deliberately **no `approval_resolve` MCP tool**. Every MCP tool is callable by the
agent and by nothing else, so such a tool would let the agent approve its own destructive action,
which is the entire thing the gate exists to prevent. The only actors that may decide are the
client's human (lane 1), the shell's human (lane 2), and the human who typed `--auto-approve` at
launch (`AutoApprovePolicy`, applied one layer up in [`PolicyEngine::requires_approval`] so an
auto-approved capability never parks a handle at all). `action_invoke`'s existing `approvalHandle`
input stays what it always was: a way to REPLAY a decision one of those three already made.

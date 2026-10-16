//! 🙋️ Approval coordinator of the consent crate: Stdio lines, elicitation and the approval coordinator.

use crate::elicitation::{ElicitationAction, ElicitationSlot, ElicitationUnavailable};
use semio_framework_os_mcp_bridge::session::{BridgeHandle, ShellConnectionId};
use semio_framework_os_mcp_bridge::shell_command_dispatch::{BridgeSlot, active_shell_connection};
use semio_framework_os_mcp_bridge_codec::bridge::{ApprovalDecision, ApprovalWithdrawal, GatewayToShell};
use semio_framework_os_mcp_foundation::rendezvous::live_os_sessions;
use semio_framework_os_mcp_wire::notify::active_request_cancel_requested;

//#region 🔖️ApprovalCoordinator

/// ⏱️ How long the shell lane waits for a human to press a button in the OS approval dialog before
/// giving up. Long enough for a real decision, short enough that an agent is never wedged forever on
/// a shell whose human walked away; the agent may simply invoke again.
pub const SHELL_APPROVAL_TIMEOUT_MS: u64 = 120_000;

const SHELL_APPROVAL_POLL_INTERVAL_MS: u64 = 25;

/// 🛤️ Which lane produced (or failed to produce) a decision — carried into the audit row and into
/// the `ApprovalRequired` error's `details` so a caller learns WHY nobody could be asked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApprovalChannel {
    Elicitation,
    Shell,
}

impl ApprovalChannel {
    pub fn as_str(self) -> &'static str {
        match self {
            ApprovalChannel::Elicitation => "elicitation",
            ApprovalChannel::Shell => "shell",
        }
    }
}

/// 🗳️ What [`ApprovalCoordinator::resolve`] concluded. `Unreachable` carries one `details` object
/// naming every lane it tried and why each was unusable — the shipped binary never answers
/// `APPROVAL_REQUIRED` without saying what would have to change. `Cancelled` is the call itself being
/// stopped while it waited ([`crate::notify::active_request_cancel_requested`]): nobody decided.
#[derive(Clone, Debug, PartialEq)]
pub enum ApprovalResolution {
    Approved { channel: ApprovalChannel },
    Denied { channel: ApprovalChannel, note: Option<String> },
    Unreachable { details: serde_json::Value },
    Cancelled,
}

/// 🧾️ Everything a human needs shown to decide one parked approval.
pub struct ApprovalRequest<'a> {
    pub approval_handle: &'a str,
    pub capability_id: &'a str,
    pub capability_title: &'a str,
    /// 📖️ The verb's own published description, in the session's locale — the sentence an agent
    /// found it by, so the human reads the same claim the agent acted on.
    pub capability_description: &'a str,
    /// 🗿️ The artifact kind the verb belongs to, when it declares one: "what is about to change".
    pub artifact_kind: Option<&'a str>,
    pub principal_id: &'a str,
    pub diff_summary: &'a serde_json::Value,
}

impl ApprovalRequest<'_> {
    /// 📨️ The bridge's single `ApprovalRequested.summary` string, JSON-encoded in the shape both
    /// approval surfaces parse — `💬️AgentChatPanel`/`🤖️AgentApprovals`' `parseApprovalSummary` and
    /// its wgpu twin `parse_approval_summary`. The frame's wire shape is unchanged (tag 3 carries
    /// one string); this payload IS the schema, and `🧫️fixtures/🛡️approval-summary` is the one file
    /// both parsers are asserted against.
    ///
    /// ⏱️ `timeoutMs` is a DURATION, never a deadline: the shell starts its countdown when the
    /// frame arrives, so a browser clock that disagrees with the gateway's cannot make an approval
    /// look already-expired (or never-expiring) to the human deciding it.
    fn shell_summary(&self, timeout_ms: u64) -> String {
        serde_json::json!({
            "capabilityId": self.capability_id,
            "capabilityTitle": self.capability_title,
            "description": self.capability_description,
            "artifactKind": self.artifact_kind,
            "diffSummary": format!("{} — {}", self.capability_title, self.diff_summary),
            "risk": "high",
            "requestedBy": self.principal_id,
            "timeoutMs": timeout_ms,
        })
        .to_string()
    }

    fn elicitation_message(&self) -> String {
        format!("{} ({}) is a destructive action requested by {}. Change summary: {}", self.capability_title, self.capability_id, self.principal_id, self.diff_summary)
    }
}

/// ⛩️ The real resolution chain for a parked approval, in priority order:
/// 1. **MCP elicitation** — the spec's own human-in-the-loop primitive, asked of the CONNECTED client
///    (Claude Code, Codex, …), used only when that client advertised `capabilities.elicitation`.
/// 2. **The live OS shell** — `GatewayToShell::ApprovalRequested` over `/bridge`, decided by the human
///    in `🤖️AgentApprovals`/`💬️AgentChatPanel`, answered by `ShellToGateway::Approval`.
/// 3. **Neither** — a typed, non-retryable `APPROVAL_REQUIRED` naming both closed lanes.
///
/// 🔐️ There is deliberately **no `approval_resolve` MCP tool**. Every MCP tool is callable by the
/// agent and by nothing else, so such a tool would let the agent approve its own destructive action,
/// which is the entire thing the gate exists to prevent. The only actors that may decide are the
/// client's human (lane 1), the shell's human (lane 2), and the human who typed `--auto-approve` at
/// launch (`AutoApprovePolicy`, applied one layer up in [`PolicyEngine::requires_approval`] so an
/// auto-approved capability never parks a handle at all). `action_invoke`'s existing `approvalHandle`
/// input stays what it always was: a way to REPLAY a decision one of those three already made.
pub struct ApprovalCoordinator {
    pub elicitation: Option<ElicitationSlot>,
    pub bridge: Option<BridgeSlot>,
    shell_timeout_ms: u64,
    shell_attach_grace_ms: Option<u64>,
}

/// ⏳️ How long an approval waits for a live OS session to dial a bridge nobody has attached to yet: a
/// shell polls the rendezvous at most every `BRIDGE_DISCOVERY_MAX_INTERVAL_MS` (30 s,
/// `🔗️AgentBridge/🛰️offer`), plus the time to dial. Without it an agent that asked for approval right
/// after connecting was refused with "no OS shell is attached" while the human's shell was seconds
/// from attaching (measured 2026-09-26, 2 of 4 user-path runs on hub 7800).
pub const SHELL_ATTACH_GRACE_MS: u64 = 35_000;

impl ApprovalCoordinator {
    pub fn new(elicitation: Option<ElicitationSlot>, bridge: Option<BridgeSlot>) -> Self {
        Self { elicitation, bridge, shell_timeout_ms: SHELL_APPROVAL_TIMEOUT_MS, shell_attach_grace_ms: None }
    }

    /// ⏳️ Overrides [`SHELL_ATTACH_GRACE_MS`]; without it the grace applies only while the rendezvous
    /// names a live OS session that can be expected to dial.
    #[must_use]
    pub fn with_shell_attach_grace_ms(mut self, grace_ms: u64) -> Self {
        self.shell_attach_grace_ms = Some(grace_ms);
        self
    }

    /// 🐚️ The shell connection to ask: the one attached now, or the first a live OS session attaches
    /// within the grace. `None` once the grace is spent, or at once when no live session exists.
    fn await_shell_connection(&self, bridge: &BridgeHandle) -> Option<ShellConnectionId> {
        let grace_ms = self.shell_attach_grace_ms.unwrap_or_else(|| if live_os_sessions().is_empty() { 0 } else { SHELL_ATTACH_GRACE_MS });
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(grace_ms);
        loop {
            if let Some(connection) = active_shell_connection(bridge) {
                return Some(connection);
            }
            if std::time::Instant::now() >= deadline || active_request_cancel_requested() {
                return None;
            }
            std::thread::sleep(std::time::Duration::from_millis(SHELL_APPROVAL_POLL_INTERVAL_MS));
        }
    }

    #[must_use]
    pub fn with_shell_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.shell_timeout_ms = timeout_ms;
        self
    }

    pub fn resolve(&self, request: &ApprovalRequest<'_>) -> ApprovalResolution {
        let elicitation_state = match self.resolve_by_elicitation(request) {
            Ok(resolution) => return resolution,
            Err(state) => state,
        };
        let shell_state = match self.resolve_by_shell(request) {
            Ok(resolution) => return resolution,
            Err(state) => state,
        };
        ApprovalResolution::Unreachable {
            details: serde_json::json!({
                "approvalHandle": request.approval_handle,
                "channels": { "elicitation": elicitation_state, "shell": shell_state },
                "remedy": "connect an MCP client that advertises `elicitation`, attach an OS shell to the gateway bridge, or relaunch with --auto-approve readonly|all",
            }),
        }
    }

    fn resolve_by_elicitation(&self, request: &ApprovalRequest<'_>) -> Result<ApprovalResolution, &'static str> {
        let Some(channel) = self.elicitation.as_ref().and_then(|slot| slot.get()) else { return Err("no server-initiated request channel on this transport") };
        match channel.request_boolean(&request.elicitation_message(), request.capability_title) {
            Ok(ElicitationAction::Accept) => Ok(ApprovalResolution::Approved { channel: ApprovalChannel::Elicitation }),
            Ok(ElicitationAction::Decline) => Ok(ApprovalResolution::Denied { channel: ApprovalChannel::Elicitation, note: Some("the client's human declined".to_string()) }),
            Ok(ElicitationAction::Cancel) => Ok(ApprovalResolution::Denied { channel: ApprovalChannel::Elicitation, note: Some("the client cancelled the elicitation".to_string()) }),
            Err(ElicitationUnavailable::ClientDoesNotSupportIt) => Err("the connected client did not advertise capabilities.elicitation"),
            Err(ElicitationUnavailable::NoChannel) => Err("no server-initiated request channel on this transport"),
            Err(ElicitationUnavailable::ClientClosed) => Err("the client closed the connection while the elicitation was pending"),
            Err(ElicitationUnavailable::TimedOut) => Err("the connected client did not answer the elicitation in time"),
            Err(ElicitationUnavailable::Malformed(_)) => Err("the elicitation round trip failed at the transport"),
        }
    }

    /// 🐚️ Asks the most recently connected shell, and takes back what it published whenever the answer
    /// stops mattering there: `ApprovalWithdrawn{cancelled}` when the agent's call is cancelled,
    /// `{timed_out}` when the countdown runs out, and `{superseded}` when a newer shell connects — which
    /// is then asked instead, with the time that remains. A shell never keeps a decidable affordance
    /// for a request nobody is waiting on.
    fn resolve_by_shell(&self, request: &ApprovalRequest<'_>) -> Result<ApprovalResolution, &'static str> {
        let Some(bridge) = self.bridge.as_ref().and_then(|slot| slot.get()) else { return Err("this gateway is serving no /bridge — no OS shell can be asked") };
        let Some(mut connection) = self.await_shell_connection(bridge) else { return Err("a /bridge is running but no OS shell is attached to it") };
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(self.shell_timeout_ms);
        let publish = |connection| {
            let remaining_ms = u64::try_from(deadline.saturating_duration_since(std::time::Instant::now()).as_micros().div_ceil(1_000)).unwrap_or(u64::MAX).max(1);
            bridge.send_to(connection, GatewayToShell::ApprovalRequested { approval_id: request.approval_handle.to_string(), summary: request.shell_summary(remaining_ms) })
        };
        let withdraw = |connection, reason| {
            let _ = bridge.send_to(connection, GatewayToShell::ApprovalWithdrawn { approval_id: request.approval_handle.to_string(), reason });
        };
        if !publish(connection) {
            return Err("the attached shell's connection closed while the approval was being published");
        }
        loop {
            if let Some((approval_id, decision, note)) = bridge.last_approval(connection) {
                if approval_id == request.approval_handle {
                    let resolved = GatewayToShell::ApprovalResolved { approval_id, decision };
                    let _ = bridge.send_to(connection, resolved);
                    return Ok(match decision {
                        ApprovalDecision::Deny => ApprovalResolution::Denied { channel: ApprovalChannel::Shell, note },
                        ApprovalDecision::Once | ApprovalDecision::Session => ApprovalResolution::Approved { channel: ApprovalChannel::Shell },
                    });
                }
            }
            if active_request_cancel_requested() {
                withdraw(connection, ApprovalWithdrawal::Cancelled);
                return Ok(ApprovalResolution::Cancelled);
            }
            if std::time::Instant::now() >= deadline {
                withdraw(connection, ApprovalWithdrawal::TimedOut);
                return Err("the attached OS shell did not answer the approval request in time");
            }
            if let Some(newest) = active_shell_connection(bridge).filter(|newest| *newest != connection) {
                withdraw(connection, ApprovalWithdrawal::Superseded);
                connection = newest;
                if !publish(connection) {
                    return Err("the newer shell's connection closed while the approval was being moved to it");
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(SHELL_APPROVAL_POLL_INTERVAL_MS));
        }
    }
}

//#endregion 🔖️ApprovalCoordinator

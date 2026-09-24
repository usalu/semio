//! ✅️ wgpu twin of the `🤖️AgentApprovals` element (`🟦️.tsx`, 160 lines) — the `os.agent.approvals`
//! human-in-the-loop modal. It lists every parked approval request delivered by `🔗️AgentBridge`'s
//! `approvalRequested` frames with React `AgentApprovalAffordance`'s lines (verb or title, capability,
//! description, target, change summary, requested-by, risk, live countdown — [`approval_row_lines`])
//! and offers the three decisions that go back as an `Approval` frame.
//!
//! 🪟️ The React original is a Radix `Dialog` that opens purely from `approvals.length > 0` and can
//! be dismissed without deciding (a newly arrived approval re-opens it). This file owns the same
//! open/dismiss rule plus the layout math; the paint and hit registration live in the shell's own
//! immediate-mode chrome (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `render_agent_approvals_step`), the way
//! every other wgpu overlay in that shell is drawn.
//!
//! 🎯️ `parse_approval_summary` is a straight port of the React file's own parser, including its
//! documented reason for existing: the wire's `ApprovalRequested.summary` is ONE string (SSOT tag 3),
//! so a richer producer JSON-encodes `{capabilityId, diffSummary, risk, requestedBy}` into it and a
//! plain-text producer still renders as the diff summary rather than a blank dialog.

use crate::agent_bridge::{agent_label, ApprovalDecision, PendingAgentApproval};
use ui_wgpu::wgpu::{Locale, Rect, Rgba, Theme};

//#region 🔖️ParseSummary
/// ⚠️ How dangerous the requested capability is, as the producer declared it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApprovalRisk {
    Low,
    Medium,
    High,
}

impl ApprovalRisk {
    fn from_wire(value: &str) -> Option<Self> {
        match value {
            "low" => Some(ApprovalRisk::Low),
            "medium" => Some(ApprovalRisk::Medium),
            "high" => Some(ApprovalRisk::High),
            _ => None,
        }
    }

    /// 🏷️ The `data-semio-agent-approval-risk` value the React twin stamps.
    pub fn as_str(self) -> &'static str {
        match self {
            ApprovalRisk::Low => "low",
            ApprovalRisk::Medium => "medium",
            ApprovalRisk::High => "high",
        }
    }

    /// 🎨️ Badge tint from shared theme tokens — the React twin's `sky`/`amber`/`red` classes are the
    /// palette spellings of these three.
    pub fn color(self, theme: &Theme) -> Rgba {
        match self {
            ApprovalRisk::Low => theme.accent,
            ApprovalRisk::Medium => theme.warning,
            ApprovalRisk::High => theme.error,
        }
    }

    pub fn label(self, locale: Locale) -> String {
        match self {
            ApprovalRisk::Low => agent_label("Low", "Niedrig", locale),
            ApprovalRisk::Medium => agent_label("Medium", "Mittel", locale),
            ApprovalRisk::High => agent_label("High", "Hoch", locale),
        }
    }
}

/// 📄️ The display fields a summary can carry — the wgpu twin of the React
/// `ParsedApprovalSummary`, pinned to it by the shared `🧫️fixtures/🛡️summary` rows.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParsedApprovalSummary {
    pub capability_id: Option<String>,
    /// 🏷️ The verb's own published label — WHAT is about to happen.
    pub capability_title: Option<String>,
    /// 📖️ The verb's published description, the sentence the agent found it by.
    pub description: Option<String>,
    /// 🗿️ The artifact kind the verb belongs to — WHAT it happens to.
    pub artifact_kind: Option<String>,
    pub diff_summary: String,
    pub risk: Option<ApprovalRisk>,
    pub requested_by: Option<String>,
    /// ⏱️ The gateway's wait, as a DURATION from this frame's arrival; `None` for a producer that
    /// sends none and for a non-positive value, neither of which can be counted down.
    pub timeout_ms: Option<u64>,
}

/// 🔍️ Parses the single wire `summary` string: a JSON object carrying at least one of
/// `capabilityId`/`risk`/`requestedBy` yields the rich shape, anything else (including malformed
/// JSON, an array, or a bare string) falls back to treating the whole value as the diff summary.
/// The producer is `🌉️mcp/🛡️policy`'s `ApprovalRequest::shell_summary`, and every row of
/// `🧫️fixtures/🛡️summary` is asserted here AND on the React bank.
pub fn parse_approval_summary(summary: &str) -> ParsedApprovalSummary {
    let fallback = || ParsedApprovalSummary { diff_summary: summary.to_string(), ..ParsedApprovalSummary::default() };
    let Ok(serde_json::Value::Object(value)) = serde_json::from_str::<serde_json::Value>(summary) else {
        return fallback();
    };
    let text = |field: &str| value.get(field).and_then(|value| value.as_str()).filter(|value| !value.is_empty()).map(str::to_string);
    let capability_id = text("capabilityId");
    let risk = value.get("risk").and_then(|value| value.as_str()).and_then(ApprovalRisk::from_wire);
    let requested_by = text("requestedBy");
    if capability_id.is_none() && risk.is_none() && requested_by.is_none() {
        return fallback();
    }
    let diff_summary = value.get("diffSummary").and_then(|value| value.as_str()).map_or_else(|| summary.to_string(), str::to_string);
    let timeout_ms = value.get("timeoutMs").and_then(serde_json::Value::as_u64).filter(|timeout| *timeout > 0);
    ParsedApprovalSummary { capability_id, capability_title: text("capabilityTitle"), description: text("description"), artifact_kind: text("artifactKind"), diff_summary, risk, requested_by, timeout_ms }
}

/// ⏱️ Whole seconds left of `timeout_ms` counted from the frame's arrival — the wgpu twin of
/// `approvalSecondsRemaining`. `None` when nothing was named to count.
#[must_use]
pub fn approval_seconds_remaining(timeout_ms: Option<u64>, requested_at_ms: f64, now_ms: f64) -> Option<u64> {
    let timeout_ms = timeout_ms?;
    #[allow(clippy::cast_precision_loss)]
    let remaining = (requested_at_ms + timeout_ms as f64 - now_ms) / 1000.0;
    if remaining <= 0.0 {
        return Some(0);
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Some(remaining.ceil() as u64)
}
//#endregion 🔖️ParseSummary

//#region 🌐️Labels
pub fn approvals_title(locale: Locale) -> String {
    agent_label("Agent Approvals", "Agent-Freigaben", locale)
}

pub fn approvals_description(locale: Locale) -> String {
    agent_label("Review what the agent wants to do before it runs.", "Prüfe, was der Agent tun möchte, bevor es ausgeführt wird.", locale)
}

pub fn approvals_empty(locale: Locale) -> String {
    agent_label("No pending approvals", "Keine ausstehenden Freigaben", locale)
}

pub fn approvals_capability_label(locale: Locale) -> String {
    agent_label("Capability", "Fähigkeit", locale)
}

pub fn approvals_diff_label(locale: Locale) -> String {
    agent_label("Change summary", "Änderungszusammenfassung", locale)
}

pub fn approvals_requested_by_label(locale: Locale) -> String {
    agent_label("Requested by", "Angefragt von", locale)
}

pub fn approvals_risk_label(locale: Locale) -> String {
    agent_label("Risk", "Risiko", locale)
}

/// 🏷️ The subject prefix when the producer named the verb's own title (`os.agent.chat.approvalVerb`).
pub fn approvals_verb_label(locale: Locale) -> String {
    agent_label("Action", "Aktion", locale)
}

/// 🏷️ The subject prefix when it did not (`os.agent.chat.approvalRole`).
pub fn approvals_role_label(locale: Locale) -> String {
    agent_label("Approval", "Freigabe", locale)
}

/// 🗿️ What the verb applies to (`os.agent.chat.approvalTarget`).
pub fn approvals_target_label(locale: Locale) -> String {
    agent_label("Applies to", "Betrifft", locale)
}

/// ⏳️ A pending request the producer named no wait for (`os.agent.chat.approvalPending`).
pub fn approvals_pending_label(locale: Locale) -> String {
    agent_label("Waiting for your decision", "Wartet auf deine Entscheidung", locale)
}

/// ⏱️ The live countdown (`os.agent.chat.approvalCountdown`).
pub fn approvals_countdown_label(seconds: u64, locale: Locale) -> String {
    match locale {
        Locale::De => format!("Noch {seconds} s für die Entscheidung"),
        _ => format!("{seconds}s left to decide"),
    }
}

/// ⌛️ A wait that ran out (`os.agent.chat.approvalExpired`).
pub fn approvals_expired_label(locale: Locale) -> String {
    agent_label("Out of time — the agent was refused", "Zeit abgelaufen — der Agent wurde abgelehnt", locale)
}

/// 🔘️ Button copy per decision — `Deny` / `Approve Once` / `Approve for Session`.
pub fn approvals_decision_label(decision: ApprovalDecision, locale: Locale) -> String {
    match decision {
        ApprovalDecision::Deny => agent_label("Deny", "Ablehnen", locale),
        ApprovalDecision::Once => agent_label("Approve Once", "Einmal genehmigen", locale),
        ApprovalDecision::Session => agent_label("Approve for Session", "Für Sitzung genehmigen", locale),
    }
}
/// 🪦️ Why a request left without a decision — React's `os.agent.approvals.withdrawn*`.
pub fn approvals_withdrawal_label(reason: crate::agent_bridge::ApprovalWithdrawal, locale: Locale) -> String {
    match reason {
        crate::agent_bridge::ApprovalWithdrawal::Cancelled => agent_label("Withdrawn — the agent's request was cancelled", "Zurückgezogen — die Anfrage des Agenten wurde abgebrochen", locale),
        crate::agent_bridge::ApprovalWithdrawal::TimedOut => agent_label("Withdrawn — nobody decided in time", "Zurückgezogen — niemand hat rechtzeitig entschieden", locale),
        crate::agent_bridge::ApprovalWithdrawal::Superseded => agent_label("Withdrawn — moved to your newer window", "Zurückgezogen — in dein neueres Fenster verschoben", locale),
    }
}
//#endregion 🌐️Labels

//#region 🔖️Layout
/// 📐️ `max-w-lg` — the React dialog's own width cap.
pub const APPROVALS_MODAL_WIDTH: f32 = 512.0;

/// 📐️ `max-h-96` — the React list's own scroll cap.
pub const APPROVALS_LIST_MAX_HEIGHT: f32 = 384.0;

/// 📐️ Decision-button width; three of them sit on one row under each request.
pub const APPROVALS_DECISION_WIDTH: f32 = 150.0;

/// 🆔️ Control-id prefix every approval control registers under.
pub const APPROVALS_CONTROL_PREFIX: &str = "shell.agent.approval";

/// 🆔️ The dialog's own dismiss control (`showCloseButton` on the React dialog).
pub const APPROVALS_CLOSE_CONTROL_ID: &str = "shell.agent.approvals.close";

/// 🆔️ The control id one decision button on one request registers under — the id the shell's hit
/// handler parses back into `(approval_id, decision)`.
pub fn approval_decision_control_id(approval_id: &str, decision: ApprovalDecision) -> String {
    format!("{APPROVALS_CONTROL_PREFIX}.{approval_id}.{}", decision.control_suffix())
}

/// 🔎️ The inverse of [`approval_decision_control_id`] — `None` for any id that is not one of this
/// modal's decision buttons.
pub fn parse_approval_decision_control_id(control_id: &str) -> Option<(String, ApprovalDecision)> {
    let rest = control_id.strip_prefix(APPROVALS_CONTROL_PREFIX)?.strip_prefix('.')?;
    let (approval_id, suffix) = rest.rsplit_once('.')?;
    let decision = ApprovalDecision::ALL.into_iter().find(|decision| decision.control_suffix() == suffix)?;
    (!approval_id.is_empty()).then(|| (approval_id.to_string(), decision))
}

/// 🎨️ How one affordance line is tinted: body text, muted, or the risk badge's own token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApprovalLineTone {
    Text,
    Muted,
    Risk(ApprovalRisk),
}

/// 📝️ One text line of a pending request's affordance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApprovalLine {
    pub text: String,
    pub tone: ApprovalLineTone,
}

/// 📝️ The lines a pending request shows, in React `AgentApprovalAffordance`'s order: WHAT happens
/// (the verb's title, else its id, else the change itself), the capability id beside a title, the
/// description, WHAT it applies to, the change summary, WHO asked, the risk, and the polite countdown
/// (`seconds_left` from [`approval_seconds_remaining`]; `None` = no wait was named).
pub fn approval_row_lines(parsed: &ParsedApprovalSummary, seconds_left: Option<u64>, locale: Locale) -> Vec<ApprovalLine> {
    let line = |text: String, tone: ApprovalLineTone| ApprovalLine { text, tone };
    let subject = parsed.capability_title.as_deref().or(parsed.capability_id.as_deref()).unwrap_or(parsed.diff_summary.as_str());
    let prefix = if parsed.capability_title.is_some() { approvals_verb_label(locale) } else { approvals_role_label(locale) };
    let mut lines = vec![line(format!("{prefix}: {subject}"), ApprovalLineTone::Text)];
    if let (Some(capability), Some(_)) = (parsed.capability_id.as_ref(), parsed.capability_title.as_ref()) {
        lines.push(line(format!("{}: {capability}", approvals_capability_label(locale)), ApprovalLineTone::Muted));
    }
    if let Some(description) = parsed.description.as_ref() {
        lines.push(line(description.clone(), ApprovalLineTone::Muted));
    }
    if let Some(kind) = parsed.artifact_kind.as_ref() {
        lines.push(line(format!("{}: {kind}", approvals_target_label(locale)), ApprovalLineTone::Muted));
    }
    lines.push(line(format!("{}: {}", approvals_diff_label(locale), parsed.diff_summary), ApprovalLineTone::Text));
    if let Some(requested_by) = parsed.requested_by.as_ref() {
        lines.push(line(format!("{}: {requested_by}", approvals_requested_by_label(locale)), ApprovalLineTone::Muted));
    }
    if let Some(risk) = parsed.risk {
        lines.push(line(format!("{}: {}", approvals_risk_label(locale), risk.label(locale)), ApprovalLineTone::Risk(risk)));
    }
    let countdown = match seconds_left {
        None => approvals_pending_label(locale),
        Some(0) => approvals_expired_label(locale),
        Some(seconds) => approvals_countdown_label(seconds, locale),
    };
    lines.push(line(countdown, ApprovalLineTone::Muted));
    lines
}

/// 📏️ How many text lines one pending request occupies — exactly [`approval_row_lines`]' count, which
/// does not depend on the countdown's value.
pub fn approval_row_line_count(parsed: &ParsedApprovalSummary) -> usize {
    3 + usize::from(parsed.capability_id.is_some() && parsed.capability_title.is_some())
        + usize::from(parsed.description.is_some())
        + usize::from(parsed.artifact_kind.is_some())
        + usize::from(parsed.requested_by.is_some())
        + usize::from(parsed.risk.is_some())
}

/// 📏️ One request's total height: its text lines, the decision-button row, and the `py-3` gutter
/// the React `<li>` carries.
pub fn approval_row_height(parsed: &ParsedApprovalSummary, theme: &Theme) -> f32 {
    let lines = approval_row_line_count(parsed) as f32 * (theme.font_size_small * 1.6);
    lines + theme.gap_standard + theme.control_height + theme.padding_standard
}

/// 📐️ The modal itself, centred, sized to its content and clamped so it never exceeds the
/// viewport — header, list (capped at [`APPROVALS_LIST_MAX_HEIGHT`]) and footer gutter.
pub fn approvals_modal_rect(width: f32, height: f32, row_heights: &[f32], theme: &Theme) -> Rect {
    let header = theme.padding_standard + theme.font_size_body * 1.6 + theme.font_size_small * 1.6;
    let list: f32 = row_heights.iter().sum::<f32>().max(theme.font_size_small * 2.0).min(APPROVALS_LIST_MAX_HEIGHT);
    let modal_h = (header + list + theme.padding_standard * 2.0).min((height - theme.padding_standard * 2.0).max(1.0));
    let modal_w = APPROVALS_MODAL_WIDTH.min((width - theme.padding_standard * 2.0).max(1.0));
    Rect::new((width - modal_w) * 0.5, (height - modal_h) * 0.5, modal_w, modal_h)
}

/// 📐️ Where the scrollable request list starts inside the modal.
pub fn approvals_list_rect(modal: Rect, theme: &Theme) -> Rect {
    let header = theme.padding_standard + theme.font_size_body * 1.6 + theme.font_size_small * 1.6;
    Rect::new(modal.x + theme.padding_standard, modal.y + header, (modal.w - theme.padding_standard * 2.0).max(1.0), (modal.h - header - theme.padding_standard).max(1.0))
}

/// 📐️ The three decision buttons on one request row, left to right: deny, once, session.
pub fn approval_decision_rects(row: Rect, theme: &Theme) -> [(ApprovalDecision, Rect); 3] {
    let y = row.y + row.h - theme.padding_standard * 0.5 - theme.control_height;
    let step = APPROVALS_DECISION_WIDTH + theme.gap_standard;
    [
        (ApprovalDecision::Deny, Rect::new(row.x, y, APPROVALS_DECISION_WIDTH, theme.control_height)),
        (ApprovalDecision::Once, Rect::new(row.x + step, y, APPROVALS_DECISION_WIDTH, theme.control_height)),
        (ApprovalDecision::Session, Rect::new(row.x + step * 2.0, y, APPROVALS_DECISION_WIDTH, theme.control_height)),
    ]
}
//#endregion 🔖️Layout

//#region 🔖️Model
/// 🪟️ The dialog's open/dismiss state — the wgpu half of React's `useState(dismissed)` plus the
/// `useEffect` that clears it whenever a new request arrives.
#[derive(Clone, Debug, Default)]
pub struct AgentApprovalsModel {
    dismissed: bool,
    last_seen_count: usize,
}

impl AgentApprovalsModel {
    /// 🔔️ Call once per frame with the live queue: a request count that grew since the last frame
    /// re-opens a dismissed dialog, exactly as the React `useEffect` on `approvals.length` does.
    pub fn observe(&mut self, approvals: &[PendingAgentApproval]) {
        if approvals.len() > self.last_seen_count {
            self.dismissed = false;
        }
        self.last_seen_count = approvals.len();
    }

    /// 👁️ Open purely when something is pending and the human has not dismissed it.
    pub fn is_open(&self, approvals: &[PendingAgentApproval]) -> bool {
        !approvals.is_empty() && !self.dismissed
    }

    pub fn dismiss(&mut self) {
        self.dismissed = true;
    }
}
//#endregion 🔖️Model

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;

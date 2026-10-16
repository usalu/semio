//! 📥️ The gateway-to-shell frame enum, the conversation text law and the encoded page writer.

use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_value::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use super::wire;
use super::shared_types::{ApprovalDecision, ApprovalWithdrawal, BridgeRefusal};

//#region 🔖️GatewayToShell

/// 📤️ Gateway→Shell frames, tag 0..12 in this exact declaration order (`📋️master.md` §2.2); a new
/// variant only ever goes last. Wire fixture: `🧫️fixtures/📨️frames.json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "variant", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "variant", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum GatewayToShell {
    Welcome { bridge_version: u16, connection: String, principal: String },
    ShellCommand { seq: u64, command: Vec<u8> },
    AppCommand { seq: u64, instance_id: String, command: Vec<u8> },
    ApprovalRequested { approval_id: String, summary: String },
    ApprovalResolved { approval_id: String, decision: ApprovalDecision },
    AgentPresence { active: bool, label: String, invocation_id: Option<String> },
    Pong,
    Bye { reason: String },
    /// 🛠️ One tool the connected agent is invoking RIGHT NOW, emitted from the gateway's own
    /// `tools/call` dispatch before the handler runs. `arguments` is the call's arguments rendered
    /// as JSON and truncated to [`AGENT_CONVERSATION_MAX_TEXT`]; the shell renders it, never
    /// re-executes it.
    AgentToolCall { invocation_id: String, tool_name: String, arguments: String },
    /// 🧾️ The terminal outcome of the `AgentToolCall` with the same `invocation_id` — `ok` is the
    /// inverse of the result's own `isError`, `summary` its first text block (or the gateway error
    /// message), truncated the same way.
    AgentToolResult { invocation_id: String, tool_name: String, ok: bool, summary: String },
    /// 💬️ One chunk of the connected agent's OWN free-text turn — the channel that lets an agent
    /// explain, ask a clarifying question, or answer at all, rather than only act. `reply_id`
    /// identifies the turn (every chunk of one turn repeats it, so a shell appends instead of
    /// stacking rows); `in_reply_to` is the `message_id` of the `ShellToGateway::AgentMessage` this
    /// answers, or `None` for a turn the agent opened itself; `complete` marks the last chunk, so a
    /// shell can show a turn as still arriving without inventing a heuristic. The text carries no
    /// locale: it is the agent's own words, and the only translated strings on the surface are the
    /// role nouns around them.
    AgentReply { reply_id: String, in_reply_to: Option<String>, text: String, complete: bool },
    /// 🪦️ The approval request `approval_id` is withdrawn — its call was cancelled, it timed out, or it
    /// moved to a newer shell connection — so the shell retires its affordance and says why.
    ApprovalWithdrawn { approval_id: String, reason: ApprovalWithdrawal },
    /// 🚫️ The gateway will not serve this connection — see [`BridgeRefusal`]; `gateway_version` is the
    /// gateway's own [`BRIDGE_VERSION`], so a shell can say which side is older. The gateway closes the
    /// socket right after it.
    Refused { reason: BridgeRefusal, gateway_version: u16 },
}

/// 💬️ Tools whose OWN result is already a conversation frame, so `tools/call` must not also publish
/// an `AgentToolCall`/`AgentToolResult` pair for them: `conversation_reply`'s arguments ARE the
/// prose the panel is about to render, and a tool-call row would print every sentence the agent says
/// twice. The audit sink still records the call — this list governs the shell projection only.
pub const SELF_PUBLISHING_CONVERSATION_TOOLS: &[&str] = &["conversation_reply"];

/// ✂️ The hard cap on every human-readable string the agent-conversation frames carry. The bridge
/// outbox admits a bounded number of bounded frames, so an agent calling a tool with a megabyte of
/// arguments must not be able to starve it — the text is truncated with a visible marker rather
/// than silently dropped or allowed through.
pub const AGENT_CONVERSATION_MAX_TEXT: usize = 2_048;

/// ✂️ [`AGENT_CONVERSATION_MAX_TEXT`], applied on a char boundary so the result is always UTF-8.
pub fn truncate_conversation_text(text: &str) -> String {
    if text.len() <= AGENT_CONVERSATION_MAX_TEXT {
        return text.to_string();
    }
    let mut end = AGENT_CONVERSATION_MAX_TEXT;
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}…", &text[..end])
}

impl GatewayToShell {
    pub fn encoded_len(&self) -> Option<usize> {
        match self {
            GatewayToShell::Welcome { connection, principal, .. } => 3usize.checked_add(bridge_wire_field_len(connection.len())?)?.checked_add(bridge_wire_field_len(principal.len())?),
            GatewayToShell::ShellCommand { command, .. } => 9usize.checked_add(bridge_wire_field_len(command.len())?),
            GatewayToShell::AppCommand { instance_id, command, .. } => 9usize.checked_add(bridge_wire_field_len(instance_id.len())?)?.checked_add(bridge_wire_field_len(command.len())?),
            GatewayToShell::ApprovalRequested { approval_id, summary } => 1usize.checked_add(bridge_wire_field_len(approval_id.len())?)?.checked_add(bridge_wire_field_len(summary.len())?),
            GatewayToShell::ApprovalResolved { approval_id, .. } => 2usize.checked_add(bridge_wire_field_len(approval_id.len())?),
            GatewayToShell::AgentPresence { label, invocation_id, .. } => {
                let base = 3usize.checked_add(bridge_wire_field_len(label.len())?)?;
                invocation_id.as_ref().map_or(Some(base), |value| base.checked_add(bridge_wire_field_len(value.len())?))
            }
            GatewayToShell::Pong => Some(1),
            GatewayToShell::Bye { reason } => 1usize.checked_add(bridge_wire_field_len(reason.len())?),
            GatewayToShell::AgentToolCall { invocation_id, tool_name, arguments } => {
                1usize.checked_add(bridge_wire_field_len(invocation_id.len())?)?.checked_add(bridge_wire_field_len(tool_name.len())?)?.checked_add(bridge_wire_field_len(arguments.len())?)
            }
            GatewayToShell::AgentToolResult { invocation_id, tool_name, summary, .. } => {
                2usize.checked_add(bridge_wire_field_len(invocation_id.len())?)?.checked_add(bridge_wire_field_len(tool_name.len())?)?.checked_add(bridge_wire_field_len(summary.len())?)
            }
            GatewayToShell::AgentReply { reply_id, in_reply_to, text, .. } => {
                let base = 3usize.checked_add(bridge_wire_field_len(reply_id.len())?)?.checked_add(bridge_wire_field_len(text.len())?)?;
                in_reply_to.as_ref().map_or(Some(base), |value| base.checked_add(bridge_wire_field_len(value.len())?))
            }
            GatewayToShell::ApprovalWithdrawn { approval_id, .. } => 2usize.checked_add(bridge_wire_field_len(approval_id.len())?),
            GatewayToShell::Refused { .. } => Some(4),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            GatewayToShell::Welcome { bridge_version, connection, principal } => {
                wire::write_u8(&mut buf, 0);
                wire::write_u16(&mut buf, *bridge_version);
                wire::write_string(&mut buf, connection);
                wire::write_string(&mut buf, principal);
            }
            GatewayToShell::ShellCommand { seq, command } => {
                wire::write_u8(&mut buf, 1);
                wire::write_u64(&mut buf, *seq);
                wire::write_bytes(&mut buf, command);
            }
            GatewayToShell::AppCommand { seq, instance_id, command } => {
                wire::write_u8(&mut buf, 2);
                wire::write_u64(&mut buf, *seq);
                wire::write_string(&mut buf, instance_id);
                wire::write_bytes(&mut buf, command);
            }
            GatewayToShell::ApprovalRequested { approval_id, summary } => {
                wire::write_u8(&mut buf, 3);
                wire::write_string(&mut buf, approval_id);
                wire::write_string(&mut buf, summary);
            }
            GatewayToShell::ApprovalResolved { approval_id, decision } => {
                wire::write_u8(&mut buf, 4);
                wire::write_string(&mut buf, approval_id);
                wire::write_u8(&mut buf, decision.to_tag());
            }
            GatewayToShell::AgentPresence { active, label, invocation_id } => {
                wire::write_u8(&mut buf, 5);
                wire::write_bool(&mut buf, *active);
                wire::write_string(&mut buf, label);
                wire::write_option_string(&mut buf, invocation_id);
            }
            GatewayToShell::Pong => wire::write_u8(&mut buf, 6),
            GatewayToShell::Bye { reason } => {
                wire::write_u8(&mut buf, 7);
                wire::write_string(&mut buf, reason);
            }
            GatewayToShell::AgentToolCall { invocation_id, tool_name, arguments } => {
                wire::write_u8(&mut buf, 8);
                wire::write_string(&mut buf, invocation_id);
                wire::write_string(&mut buf, tool_name);
                wire::write_string(&mut buf, arguments);
            }
            GatewayToShell::AgentToolResult { invocation_id, tool_name, ok, summary } => {
                wire::write_u8(&mut buf, 9);
                wire::write_string(&mut buf, invocation_id);
                wire::write_string(&mut buf, tool_name);
                wire::write_bool(&mut buf, *ok);
                wire::write_string(&mut buf, summary);
            }
            GatewayToShell::AgentReply { reply_id, in_reply_to, text, complete } => {
                wire::write_u8(&mut buf, 10);
                wire::write_string(&mut buf, reply_id);
                wire::write_option_string(&mut buf, in_reply_to);
                wire::write_string(&mut buf, text);
                wire::write_bool(&mut buf, *complete);
            }
            GatewayToShell::ApprovalWithdrawn { approval_id, reason } => {
                wire::write_u8(&mut buf, 11);
                wire::write_string(&mut buf, approval_id);
                wire::write_u8(&mut buf, reason.to_tag());
            }
            GatewayToShell::Refused { reason, gateway_version } => {
                wire::write_u8(&mut buf, 12);
                wire::write_u8(&mut buf, reason.to_tag());
                wire::write_u16(&mut buf, *gateway_version);
            }
        }
        buf
    }

    pub fn copy_encoded_page(&self, offset: usize, output: &mut [u8]) -> usize {
        let mut writer = BridgeEncodedPageWriter::new(offset, output);
        match self {
            Self::Welcome { bridge_version, connection, principal } => {
                writer.push(&[0]);
                writer.push(&bridge_version.to_le_bytes());
                writer.field(connection.as_bytes());
                writer.field(principal.as_bytes());
            }
            Self::ShellCommand { seq, command } => {
                writer.push(&[1]);
                writer.push(&seq.to_le_bytes());
                writer.field(command);
            }
            Self::AppCommand { seq, instance_id, command } => {
                writer.push(&[2]);
                writer.push(&seq.to_le_bytes());
                writer.field(instance_id.as_bytes());
                writer.field(command);
            }
            Self::ApprovalRequested { approval_id, summary } => {
                writer.push(&[3]);
                writer.field(approval_id.as_bytes());
                writer.field(summary.as_bytes());
            }
            Self::ApprovalResolved { approval_id, decision } => {
                writer.push(&[4]);
                writer.field(approval_id.as_bytes());
                writer.push(&[decision.to_tag()]);
            }
            Self::AgentPresence { active, label, invocation_id } => {
                writer.push(&[5, *active as u8]);
                writer.field(label.as_bytes());
                writer.push(&[invocation_id.is_some() as u8]);
                if let Some(invocation_id) = invocation_id {
                    writer.field(invocation_id.as_bytes());
                }
            }
            Self::Pong => writer.push(&[6]),
            Self::Bye { reason } => {
                writer.push(&[7]);
                writer.field(reason.as_bytes());
            }
            Self::AgentToolCall { invocation_id, tool_name, arguments } => {
                writer.push(&[8]);
                writer.field(invocation_id.as_bytes());
                writer.field(tool_name.as_bytes());
                writer.field(arguments.as_bytes());
            }
            Self::AgentToolResult { invocation_id, tool_name, ok, summary } => {
                writer.push(&[9]);
                writer.field(invocation_id.as_bytes());
                writer.field(tool_name.as_bytes());
                writer.push(&[*ok as u8]);
                writer.field(summary.as_bytes());
            }
            Self::AgentReply { reply_id, in_reply_to, text, complete } => {
                writer.push(&[10]);
                writer.field(reply_id.as_bytes());
                writer.push(&[in_reply_to.is_some() as u8]);
                if let Some(in_reply_to) = in_reply_to {
                    writer.field(in_reply_to.as_bytes());
                }
                writer.field(text.as_bytes());
                writer.push(&[*complete as u8]);
            }
            Self::ApprovalWithdrawn { approval_id, reason } => {
                writer.push(&[11]);
                writer.field(approval_id.as_bytes());
                writer.push(&[reason.to_tag()]);
            }
            Self::Refused { reason, gateway_version } => {
                writer.push(&[12, reason.to_tag()]);
                writer.push(&gateway_version.to_le_bytes());
            }
        }
        writer.written
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, GatewayError> {
        let mut reader = wire::Reader::new(bytes);
        let tag = reader.read_u8()?;
        let frame = match tag {
            0 => GatewayToShell::Welcome { bridge_version: reader.read_u16()?, connection: reader.read_string()?, principal: reader.read_string()? },
            1 => GatewayToShell::ShellCommand { seq: reader.read_u64()?, command: reader.read_bytes()? },
            2 => GatewayToShell::AppCommand { seq: reader.read_u64()?, instance_id: reader.read_string()?, command: reader.read_bytes()? },
            3 => GatewayToShell::ApprovalRequested { approval_id: reader.read_string()?, summary: reader.read_string()? },
            4 => GatewayToShell::ApprovalResolved { approval_id: reader.read_string()?, decision: ApprovalDecision::from_tag(reader.read_u8()?)? },
            5 => GatewayToShell::AgentPresence { active: reader.read_bool()?, label: reader.read_string()?, invocation_id: reader.read_option_string()? },
            6 => GatewayToShell::Pong,
            7 => GatewayToShell::Bye { reason: reader.read_string()? },
            8 => GatewayToShell::AgentToolCall { invocation_id: reader.read_string()?, tool_name: reader.read_string()?, arguments: reader.read_string()? },
            9 => GatewayToShell::AgentToolResult { invocation_id: reader.read_string()?, tool_name: reader.read_string()?, ok: reader.read_bool()?, summary: reader.read_string()? },
            10 => GatewayToShell::AgentReply { reply_id: reader.read_string()?, in_reply_to: reader.read_option_string()?, text: reader.read_string()?, complete: reader.read_bool()? },
            11 => GatewayToShell::ApprovalWithdrawn { approval_id: reader.read_string()?, reason: ApprovalWithdrawal::from_tag(reader.read_u8()?)? },
            12 => GatewayToShell::Refused { reason: BridgeRefusal::from_tag(reader.read_u8()?)?, gateway_version: reader.read_u16()? },
            other => return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown GatewayToShell tag {other}"))),
        };
        reader.finish()?;
        Ok(frame)
    }
}

struct BridgeEncodedPageWriter<'a> {
    pub offset: usize,
    pub position: usize,
    pub output: &'a mut [u8],
    pub written: usize,
}

impl<'a> BridgeEncodedPageWriter<'a> {
    pub(super) fn new(offset: usize, output: &'a mut [u8]) -> Self {
        Self { offset, position: 0, output, written: 0 }
    }

    pub fn field(&mut self, bytes: &[u8]) {
        self.push(&(bytes.len() as u32).to_le_bytes());
        self.push(bytes);
    }

    pub fn push(&mut self, bytes: &[u8]) {
        let start = self.position;
        self.position += bytes.len();
        let overlap_start = self.offset.max(start);
        let overlap_end = (self.offset + self.output.len()).min(self.position);
        if overlap_start >= overlap_end {
            return;
        }
        let source = overlap_start - start;
        let count = overlap_end - overlap_start;
        let target = overlap_start - self.offset;
        self.output[target..target + count].copy_from_slice(&bytes[source..source + count]);
        self.written = self.written.max(target + count);
    }
}

pub fn bridge_wire_field_len(bytes: usize) -> Option<usize> {
    u32::try_from(bytes).ok()?;
    4usize.checked_add(bytes)
}

//#endregion 🔖️GatewayToShell

//! 📤️ The shell-to-gateway frame enum and its hand-rolled codec.

use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_value::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use super::wire;
use super::shared_types::{ApprovalDecision, BridgeFlags, BridgeInstanceRef, ShellKind};

//#region 🔖️ShellToGateway

/// 📨️ Shell→Gateway frames, tag 0..10 in this exact declaration order (`📋️master.md` §2.2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "variant", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "variant", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShellToGateway {
    Hello { bridge_version: u16, shell_kind: ShellKind, shell_session_id: String, principal_actor: String, flags: BridgeFlags },
    ShellState { revision: u64, state: Vec<u8> },
    ShellStatePatch { revision: u64, base_revision: u64, patch: Vec<u8> },
    Instances { entries: Vec<BridgeInstanceRef> },
    AppFrames { in_reply_to: u64, instance_id: String, frames: Vec<Vec<u8>> },
    ShellCommandResult { in_reply_to: u64, ok: bool, fault: Option<String> },
    Approval { approval_id: String, decision: ApprovalDecision, note: Option<String> },
    Ping,
    Bye,
    /// 💬️ One human turn typed into the shell's own agent chat panel, addressed to the connected
    /// MCP agent. `message_id` is minted by the shell and is what the gateway's
    /// `semio://ui/agent-messages` resource pages from, so the agent reads each turn exactly once
    /// and never has to guess whether it already saw one.
    AgentMessage { message_id: String, text: String },
    /// 🛑️ The human pressed cancel on a tool call still shown as running in the shell's agent panel.
    /// `invocation_id` is the exact id `AgentConversation::begin_tool_call` published, which is also
    /// the id that call's `crate::ui::job_registry()` entry is keyed by — so this frame reaches the
    /// SAME cooperative cancellation the `job_cancel` MCP tool flips, never a second mechanism.
    AgentCancel { invocation_id: String },
}

impl ShellToGateway {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            ShellToGateway::Hello { bridge_version, shell_kind, shell_session_id, principal_actor, flags } => {
                wire::write_u8(&mut buf, 0);
                wire::write_u16(&mut buf, *bridge_version);
                wire::write_u8(&mut buf, shell_kind.to_tag());
                wire::write_string(&mut buf, shell_session_id);
                wire::write_string(&mut buf, principal_actor);
                wire::write_u8(&mut buf, flags.to_bits());
            }
            ShellToGateway::ShellState { revision, state } => {
                wire::write_u8(&mut buf, 1);
                wire::write_u64(&mut buf, *revision);
                wire::write_bytes(&mut buf, state);
            }
            ShellToGateway::ShellStatePatch { revision, base_revision, patch } => {
                wire::write_u8(&mut buf, 2);
                wire::write_u64(&mut buf, *revision);
                wire::write_u64(&mut buf, *base_revision);
                wire::write_bytes(&mut buf, patch);
            }
            ShellToGateway::Instances { entries } => {
                wire::write_u8(&mut buf, 3);
                wire::write_u32(&mut buf, entries.len() as u32);
                for entry in entries {
                    entry.encode(&mut buf);
                }
            }
            ShellToGateway::AppFrames { in_reply_to, instance_id, frames } => {
                wire::write_u8(&mut buf, 4);
                wire::write_u64(&mut buf, *in_reply_to);
                wire::write_string(&mut buf, instance_id);
                wire::write_bytes_vec(&mut buf, frames);
            }
            ShellToGateway::ShellCommandResult { in_reply_to, ok, fault } => {
                wire::write_u8(&mut buf, 5);
                wire::write_u64(&mut buf, *in_reply_to);
                wire::write_bool(&mut buf, *ok);
                wire::write_option_string(&mut buf, fault);
            }
            ShellToGateway::Approval { approval_id, decision, note } => {
                wire::write_u8(&mut buf, 6);
                wire::write_string(&mut buf, approval_id);
                wire::write_u8(&mut buf, decision.to_tag());
                wire::write_option_string(&mut buf, note);
            }
            ShellToGateway::Ping => wire::write_u8(&mut buf, 7),
            ShellToGateway::Bye => wire::write_u8(&mut buf, 8),
            ShellToGateway::AgentMessage { message_id, text } => {
                wire::write_u8(&mut buf, 9);
                wire::write_string(&mut buf, message_id);
                wire::write_string(&mut buf, text);
            }
            ShellToGateway::AgentCancel { invocation_id } => {
                wire::write_u8(&mut buf, 10);
                wire::write_string(&mut buf, invocation_id);
            }
        }
        buf
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, GatewayError> {
        let mut reader = wire::Reader::new(bytes);
        let tag = reader.read_u8()?;
        let frame = match tag {
            0 => ShellToGateway::Hello {
                bridge_version: reader.read_u16()?,
                shell_kind: ShellKind::from_tag(reader.read_u8()?)?,
                shell_session_id: reader.read_string()?,
                principal_actor: reader.read_string()?,
                flags: BridgeFlags::from_bits(reader.read_u8()?),
            },
            1 => ShellToGateway::ShellState { revision: reader.read_u64()?, state: reader.read_bytes()? },
            2 => ShellToGateway::ShellStatePatch { revision: reader.read_u64()?, base_revision: reader.read_u64()?, patch: reader.read_bytes()? },
            3 => {
                let len = reader.read_u32()? as usize;
                let mut entries = Vec::with_capacity(len);
                for _ in 0..len {
                    entries.push(BridgeInstanceRef::decode(&mut reader)?);
                }
                ShellToGateway::Instances { entries }
            }
            4 => ShellToGateway::AppFrames { in_reply_to: reader.read_u64()?, instance_id: reader.read_string()?, frames: reader.read_bytes_vec()? },
            5 => ShellToGateway::ShellCommandResult { in_reply_to: reader.read_u64()?, ok: reader.read_bool()?, fault: reader.read_option_string()? },
            6 => ShellToGateway::Approval { approval_id: reader.read_string()?, decision: ApprovalDecision::from_tag(reader.read_u8()?)?, note: reader.read_option_string()? },
            7 => ShellToGateway::Ping,
            8 => ShellToGateway::Bye,
            9 => ShellToGateway::AgentMessage { message_id: reader.read_string()?, text: reader.read_string()? },
            10 => ShellToGateway::AgentCancel { invocation_id: reader.read_string()? },
            other => return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown ShellToGateway tag {other}"))),
        };
        reader.finish()?;
        Ok(frame)
    }
}

//#endregion 🔖️ShellToGateway

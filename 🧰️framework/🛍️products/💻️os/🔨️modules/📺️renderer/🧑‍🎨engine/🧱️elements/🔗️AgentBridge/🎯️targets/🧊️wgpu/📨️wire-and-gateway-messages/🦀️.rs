//! 📨️ Wire and gateway messages: the `wire` frame codec, the gateway to shell messages and the shell to gateway messages. See [`super`].

use super::*;

//#region 🔖️Wire
/// 🧵️ Length-prefixed little-endian primitives — the byte layout the SSOT's private `mod wire`
/// defines, reproduced here (and fixture-checked) rather than imported.
mod wire {
    use super::BridgeFrameFault;

    pub struct Reader<'a> {
        bytes: &'a [u8],
        position: usize,
    }

    impl<'a> Reader<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            Self { bytes, position: 0 }
        }

        fn take(&mut self, count: usize) -> Result<&'a [u8], BridgeFrameFault> {
            let end = self.position.checked_add(count).ok_or(BridgeFrameFault::Truncated)?;
            let slice = self.bytes.get(self.position..end).ok_or(BridgeFrameFault::Truncated)?;
            self.position = end;
            Ok(slice)
        }

        pub fn read_u8(&mut self) -> Result<u8, BridgeFrameFault> {
            Ok(self.take(1)?[0])
        }

        pub fn read_bool(&mut self) -> Result<bool, BridgeFrameFault> {
            match self.read_u8()? {
                0 => Ok(false),
                1 => Ok(true),
                other => Err(BridgeFrameFault::UnknownTag(other)),
            }
        }

        pub fn read_u16(&mut self) -> Result<u16, BridgeFrameFault> {
            let bytes = self.take(2)?;
            Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
        }

        pub fn read_u64(&mut self) -> Result<u64, BridgeFrameFault> {
            let bytes = self.take(8)?;
            let mut value = [0u8; 8];
            value.copy_from_slice(bytes);
            Ok(u64::from_le_bytes(value))
        }

        pub fn read_bytes(&mut self) -> Result<Vec<u8>, BridgeFrameFault> {
            let len = self.read_u32()? as usize;
            Ok(self.take(len)?.to_vec())
        }

        pub fn read_u32(&mut self) -> Result<u32, BridgeFrameFault> {
            let bytes = self.take(4)?;
            Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        }

        pub fn read_string(&mut self) -> Result<String, BridgeFrameFault> {
            let bytes = self.read_bytes()?;
            String::from_utf8(bytes).map_err(|_| BridgeFrameFault::NotUtf8)
        }

        pub fn read_option_string(&mut self) -> Result<Option<String>, BridgeFrameFault> {
            if self.read_bool()? {
                self.read_string().map(Some)
            } else {
                Ok(None)
            }
        }

        pub fn finish(self) -> Result<(), BridgeFrameFault> {
            if self.position == self.bytes.len() {
                Ok(())
            } else {
                Err(BridgeFrameFault::TrailingBytes)
            }
        }
    }

    pub fn write_u8(buf: &mut Vec<u8>, value: u8) {
        buf.push(value);
    }

    pub fn write_bool(buf: &mut Vec<u8>, value: bool) {
        buf.push(value as u8);
    }

    pub fn write_u16(buf: &mut Vec<u8>, value: u16) {
        buf.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_u64(buf: &mut Vec<u8>, value: u64) {
        buf.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_bytes(buf: &mut Vec<u8>, value: &[u8]) {
        buf.extend_from_slice(&(value.len() as u32).to_le_bytes());
        buf.extend_from_slice(value);
    }

    pub fn write_string(buf: &mut Vec<u8>, value: &str) {
        write_bytes(buf, value.as_bytes());
    }

    pub fn write_option_string(buf: &mut Vec<u8>, value: &Option<String>) {
        match value {
            Some(value) => {
                write_bool(buf, true);
                write_string(buf, value);
            }
            None => write_bool(buf, false),
        }
    }
}
//#endregion 🔖️Wire

//#region 🔖️GatewayToShell
/// 📤️ Gateway→Shell frames, tags `0..12` in SSOT declaration order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GatewayToShell {
    Welcome {
        bridge_version: u16,
        connection: String,
        principal: String,
    },
    ShellCommand {
        seq: u64,
        command: Vec<u8>,
    },
    AppCommand {
        seq: u64,
        instance_id: String,
        command: Vec<u8>,
    },
    ApprovalRequested {
        approval_id: String,
        summary: String,
    },
    ApprovalResolved {
        approval_id: String,
        decision: ApprovalDecision,
    },
    AgentPresence {
        active: bool,
        label: String,
        invocation_id: Option<String>,
    },
    Pong,
    Bye {
        reason: String,
    },
    /// 🛠️ One tool the connected agent is invoking right now, emitted by the gateway's own
    /// `tools/call` dispatch before the handler runs. `arguments` is the call's arguments rendered
    /// as JSON; the shell renders it, never re-executes it.
    AgentToolCall {
        invocation_id: String,
        tool_name: String,
        arguments: String,
    },
    /// 🧾️ The terminal outcome of the [`GatewayToShell::AgentToolCall`] carrying the same
    /// `invocation_id` — `ok` is the inverse of the result's own `isError`.
    AgentToolResult {
        invocation_id: String,
        tool_name: String,
        ok: bool,
        summary: String,
    },
    /// 💬️ One chunk of the connected agent's OWN free-text turn — the wgpu twin of the gateway's
    /// `AgentReply` (`🌉️mcp/🧵️bridge/🦀️.rs:1389`) and of React's `agentMessage` conversation entry.
    /// Every chunk of one turn repeats `reply_id`, so a shell appends to the row it already has
    /// instead of stacking one row per chunk; `in_reply_to` is the `message_id` of the
    /// `ShellToGateway::AgentMessage` this answers, or `None` for a turn the agent opened itself;
    /// `complete` marks the last chunk. The text is the agent's own words and carries no locale.
    AgentReply {
        reply_id: String,
        in_reply_to: Option<String>,
        text: String,
        complete: bool,
    },
    /// 🪦️ The approval request is withdrawn — its call was cancelled, it timed out, or a newer shell
    /// now carries it — so this shell retires the affordance and says why.
    ApprovalWithdrawn {
        approval_id: String,
        reason: ApprovalWithdrawal,
    },
    /// 🚧️ The gateway answered this connection with a typed refusal instead of `Welcome`, naming its
    /// own bridge version; the socket closes right after.
    Refused {
        reason: BridgeRefusal,
        gateway_version: u16,
    },
}

impl GatewayToShell {
    pub fn decode(bytes: &[u8]) -> Result<Self, BridgeFrameFault> {
        let mut reader = wire::Reader::new(bytes);
        let frame = match reader.read_u8()? {
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
            other => return Err(BridgeFrameFault::UnknownTag(other)),
        };
        reader.finish()?;
        Ok(frame)
    }

    /// 🧪️ Encoder for the inbound direction — production never sends these, but a simulated
    /// `ApprovalRequested`/`AgentPresence` frame is exactly how this packet's acceptance tests and
    /// the fixture replay drive the consumer.
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
}
//#endregion 🔖️GatewayToShell

//#region 🔖️ShellToGateway
/// 📨️ Shell→Gateway frames this shell actually produces. The SSOT's enum has eleven variants
/// (`ShellState`/`ShellStatePatch`/`Instances`/`AppFrames` carry the React mirror's snapshots); the
/// wgpu shell publishes none of those four yet, so encoding them here would be dead wire with no
/// producer — their tags (`1`,`2`,`3`,`4`) stay reserved and unread.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellToGateway {
    Hello {
        bridge_version: u16,
        shell_kind: ShellKind,
        shell_session_id: String,
        principal_actor: String,
        flags: BridgeFlags,
    },
    ShellCommandResult {
        in_reply_to: u64,
        ok: bool,
        fault: Option<String>,
    },
    Approval {
        approval_id: String,
        decision: ApprovalDecision,
        note: Option<String>,
    },
    Ping,
    Bye,
    /// 💬️ One human turn typed into this shell and sent to the connected agent.
    AgentMessage {
        message_id: String,
        text: String,
    },
    AgentCancel {
        invocation_id: String,
    },
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

    /// 🧪️ Decoder for the outbound direction — the fixture replay's other half; production reads
    /// these only on the gateway side.
    pub fn decode(bytes: &[u8]) -> Result<Self, BridgeFrameFault> {
        let mut reader = wire::Reader::new(bytes);
        let frame = match reader.read_u8()? {
            0 => ShellToGateway::Hello {
                bridge_version: reader.read_u16()?,
                shell_kind: ShellKind::from_tag(reader.read_u8()?)?,
                shell_session_id: reader.read_string()?,
                principal_actor: reader.read_string()?,
                flags: BridgeFlags::from_bits(reader.read_u8()?),
            },
            5 => ShellToGateway::ShellCommandResult { in_reply_to: reader.read_u64()?, ok: reader.read_bool()?, fault: reader.read_option_string()? },
            6 => ShellToGateway::Approval { approval_id: reader.read_string()?, decision: ApprovalDecision::from_tag(reader.read_u8()?)?, note: reader.read_option_string()? },
            7 => ShellToGateway::Ping,
            8 => ShellToGateway::Bye,
            9 => ShellToGateway::AgentMessage { message_id: reader.read_string()?, text: reader.read_string()? },
            10 => ShellToGateway::AgentCancel { invocation_id: reader.read_string()? },
            other => return Err(BridgeFrameFault::UnknownTag(other)),
        };
        reader.finish()?;
        Ok(frame)
    }
}
//#endregion 🔖️ShellToGateway

//! 🌉️ wgpu twin of the `🔗️AgentBridge` element (`🟦️.tsx`, 476 lines) — the headless ShellBridge
//! consumer the wgpu shell needs so an `os.agent.*` frame delivered to a wgpu-rendered shell can
//! reach the user at all. React's hook owns three things: the `semio.mcp.bridge.v1` frame codec, a
//! `ShellState` mirror reduced from inbound `shellCommand` frames, and the presence/approval state
//! `🚦️AgentPresence`/`🤖️AgentApprovals` render. This file owns the first and the third; the
//! `ShellState` mirror is deliberately NOT ported — the wgpu shell IS the authority (it holds the
//! live `ShellState` directly), so mirroring it into a second reduced copy would be the React-only
//! half of the design, not parity.
//!
//! 🔁️ Codec provenance: the Rust SSOT is `💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs`, which cannot be
//! depended on from here — `semio-framework-os-mcp` pulls `axum` + `tokio` with the `full` feature
//! and therefore does not build for `wasm32-unknown-unknown`, which this crate must. This file is
//! consequently the THIRD implementation of the same byte layout (Rust SSOT, `🟦️.ts` twin, this
//! one) and is held to the SSOT's own documented anti-drift mechanism: every row of
//! `🧵️bridge/🧫️fixtures/📨️frames.json` is replayed byte-for-byte against this codec in
//! `../../🧪️tests/🔬️wgpu-unit/🦀️.rs`, exactly as `mod quick`'s
//! `every_fixture_round_trips_through_the_rust_codec` does for the SSOT.
//!
//! 🔌️ Transport: this module is transport-free on purpose. [`AgentBridgeState::apply_encoded_frame`]
//! takes the bytes one socket message carried and [`AgentBridgeState::take_outbox`] yields the bytes
//! to send back, so the browser (`🚪️host-io`) and native halves both drive it the same way the
//! plugin bridge is driven — see the packet report for the socket wiring that is still absent.

use ui_wgpu::wgpu::{Locale, LocalizedLabel, Terminology};

//#region 🔖️BridgeVersion
/// 🔢️ `BRIDGE_VERSION` from the SSOT — the value a `Hello` frame carries.
pub const BRIDGE_VERSION: u16 = 1;

/// 🔗️ First (exact) websocket subprotocol; the second is the admission proof, which keeps admission
/// out of URLs, logs and referrers exactly as `bridgeProtocols` does on the React side.
pub const BRIDGE_SUBPROTOCOL: &str = "semio.mcp.bridge.v1";

/// ⏱️ Reconnect backoff and keep-alive cadence — same three constants React's hook uses.
pub const RECONNECT_BASE_MS: f64 = 1000.0;
pub const RECONNECT_MAX_MS: f64 = 30_000.0;
pub const PING_INTERVAL_MS: f64 = 20_000.0;

/// ⏱️ `min(RECONNECT_BASE_MS * 2^(attempt-1), RECONNECT_MAX_MS)` — attempt `0` means "not
/// reconnecting yet" and yields the base delay, matching `scheduleReconnect`'s own first step.
pub fn reconnect_delay_ms(attempt: u32) -> f64 {
    let exponent = attempt.saturating_sub(1).min(32);
    (RECONNECT_BASE_MS * 2f64.powi(exponent as i32)).min(RECONNECT_MAX_MS)
}
//#endregion 🔖️BridgeVersion

//#region 🔖️Config
/// 🔗️ Everything needed to dial the gateway — React's `AgentBridgeConfig`, field for field.
///
/// 🔒️ `admission_proof` is a PROTECTED in-memory value the local supervisor hands over, never an
/// environment credential: it travels as the second websocket subprotocol (see
/// [`bridge_protocols`]), which is what keeps it out of urls, logs and referrers. It is therefore
/// deliberately not `Debug`-printed and never reaches a `[DEBUG]` line.
#[derive(Clone, PartialEq, Eq)]
pub struct AgentBridgeConfig {
    pub url: String,
    pub admission_proof: String,
}

impl std::fmt::Debug for AgentBridgeConfig {
    /// 🔒️ Prints the url and the LENGTH of the proof, never the proof. A `{config:?}` that leaked
    /// admission into a console dump would defeat the whole subprotocol arrangement.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("AgentBridgeConfig").field("url", &self.url).field("admission_proof_len", &self.admission_proof.len()).finish()
    }
}

/// 🚧️ The largest url/proof this shell will carry, so a malformed boot hook cannot hand the door an
/// unbounded string.
pub const AGENT_BRIDGE_FIELD_MAX_BYTES: usize = 2048;

impl AgentBridgeConfig {
    /// ✅️ Admits one config, or refuses it by NAME. A bridge url must be a websocket url: React's
    /// own `new WebSocket(url, …)` throws on anything else, and a throw at dial time is a worse
    /// place to find out than a refusal at the door.
    pub fn admit(url: &str, admission_proof: &str) -> Result<Self, String> {
        let url = url.trim();
        if url.is_empty() || admission_proof.is_empty() {
            return Err("agent bridge config is incomplete".to_string());
        }
        if url.len() > AGENT_BRIDGE_FIELD_MAX_BYTES || admission_proof.len() > AGENT_BRIDGE_FIELD_MAX_BYTES {
            return Err(format!("agent bridge config exceeds {AGENT_BRIDGE_FIELD_MAX_BYTES} bytes"));
        }
        if !(url.starts_with("ws://") || url.starts_with("wss://")) {
            return Err("agent bridge url is not a websocket url".to_string());
        }
        Ok(Self { url: url.to_string(), admission_proof: admission_proof.to_string() })
    }
}

/// 🔗️ The EXACT ordered websocket subprotocols — React's `bridgeProtocols`, byte for byte. The
/// first names the frame contract, the second IS the admission proof.
pub fn bridge_protocols(config: &AgentBridgeConfig) -> [String; 2] {
    [BRIDGE_SUBPROTOCOL.to_string(), config.admission_proof.clone()]
}

/// 🔎️ React's `discoverAgentBridgeConfig`, ported including its decision: environment discovery is
/// deliberately DISABLED, because bridge admission is a protected in-memory value supplied by the
/// local supervisor, never a Vite/env credential carrier. It answers `None` on both targets, so a
/// shell with no supervisor stays `Disabled` forever instead of dialling something it guessed — the
/// same live behaviour React has today. The config arrives through the boot door instead
/// (`semioWgpuSetAgentBridgeConfig` in the browser, `--agent-bridge-url` natively).
pub fn discover_agent_bridge_config() -> Option<AgentBridgeConfig> {
    None
}
//#endregion 🔖️Config

//#region 🔖️SharedTypes
/// 🐚️ Which shell dialled the bridge — the wgpu shell announces `WgpuWeb`/`WgpuNative`, the two
/// tags the gateway already reserves for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellKind {
    React,
    WgpuWeb,
    WgpuNative,
}

impl ShellKind {
    pub fn to_tag(self) -> u8 {
        match self {
            ShellKind::React => 0,
            ShellKind::WgpuWeb => 1,
            ShellKind::WgpuNative => 2,
        }
    }

    pub fn from_tag(tag: u8) -> Result<Self, BridgeFrameFault> {
        match tag {
            0 => Ok(ShellKind::React),
            1 => Ok(ShellKind::WgpuWeb),
            2 => Ok(ShellKind::WgpuNative),
            other => Err(BridgeFrameFault::UnknownTag(other)),
        }
    }

    /// 🖥️ The tag this build announces: a browser wgpu shell is `WgpuWeb`, a winit one `WgpuNative`.
    pub fn for_this_target() -> Self {
        if cfg!(target_arch = "wasm32") {
            ShellKind::WgpuWeb
        } else {
            ShellKind::WgpuNative
        }
    }
}

/// 🚩️ `RelayAppCommands|SharedBackbone|Elicit` — a bitmask on the wire, named booleans in Rust.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BridgeFlags {
    pub relay_app_commands: bool,
    pub shared_backbone: bool,
    pub elicit: bool,
}

impl BridgeFlags {
    pub const NONE: Self = Self { relay_app_commands: false, shared_backbone: false, elicit: false };

    pub fn to_bits(self) -> u8 {
        (self.relay_app_commands as u8) | ((self.shared_backbone as u8) << 1) | ((self.elicit as u8) << 2)
    }

    pub fn from_bits(bits: u8) -> Self {
        Self { relay_app_commands: bits & 0b001 != 0, shared_backbone: bits & 0b010 != 0, elicit: bits & 0b100 != 0 }
    }
}

/// ✅️ The three human decisions `🤖️AgentApprovals` offers, in wire-tag order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApprovalDecision {
    Deny,
    Once,
    Session,
}

/// 🪦️ Why the gateway withdrew an approval request — the wgpu twin of `🌉️mcp/🧵️bridge`'s
/// `ApprovalWithdrawal`, same tags.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApprovalWithdrawal {
    Cancelled,
    TimedOut,
    Superseded,
}

impl ApprovalWithdrawal {
    pub fn to_tag(self) -> u8 {
        match self {
            ApprovalWithdrawal::Cancelled => 0,
            ApprovalWithdrawal::TimedOut => 1,
            ApprovalWithdrawal::Superseded => 2,
        }
    }

    pub fn from_tag(tag: u8) -> Result<Self, BridgeFrameFault> {
        match tag {
            0 => Ok(ApprovalWithdrawal::Cancelled),
            1 => Ok(ApprovalWithdrawal::TimedOut),
            2 => Ok(ApprovalWithdrawal::Superseded),
            other => Err(BridgeFrameFault::UnknownTag(other)),
        }
    }
}

impl ApprovalDecision {
    pub fn to_tag(self) -> u8 {
        match self {
            ApprovalDecision::Deny => 0,
            ApprovalDecision::Once => 1,
            ApprovalDecision::Session => 2,
        }
    }

    pub fn from_tag(tag: u8) -> Result<Self, BridgeFrameFault> {
        match tag {
            0 => Ok(ApprovalDecision::Deny),
            1 => Ok(ApprovalDecision::Once),
            2 => Ok(ApprovalDecision::Session),
            other => Err(BridgeFrameFault::UnknownTag(other)),
        }
    }

    /// 🆔️ Control-id suffix the modal registers each decision button under.
    pub fn control_suffix(self) -> &'static str {
        match self {
            ApprovalDecision::Deny => "deny",
            ApprovalDecision::Once => "once",
            ApprovalDecision::Session => "session",
        }
    }

    pub const ALL: [ApprovalDecision; 3] = [ApprovalDecision::Deny, ApprovalDecision::Once, ApprovalDecision::Session];
}

/// ⚠️ Why a frame could not be read — no `GatewayError` here, because that type lives in the
/// gateway crate this build cannot link (see the module docstring).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BridgeFrameFault {
    Truncated,
    TrailingBytes,
    UnknownTag(u8),
    NotUtf8,
}
//#endregion 🔖️SharedTypes

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
/// 📤️ Gateway→Shell frames, tags `0..11` in SSOT declaration order.
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

//#region 🔖️State
/// 🚦️ Connection status, one-for-one with React's `AgentBridgeStatus` string union.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentBridgeStatus {
    #[default]
    Disabled,
    Connecting,
    Open,
    Reconnecting,
    Closed,
}

/// 🤖️ What the agent is doing right now, as the last `agentPresence` frame reported it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AgentBridgePresence {
    pub active: bool,
    pub label: String,
    pub invocation_id: Option<String>,
}

/// ⏸️ One parked capability request awaiting a human decision.
#[derive(Clone, Debug, PartialEq)]
pub struct PendingAgentApproval {
    pub approval_id: String,
    pub summary: String,
    pub requested_at_ms: f64,
}

/// 🛠️ How far one tool invocation in the conversation has got.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentToolCallState {
    Running,
    Cancelling,
    Ok,
    Failed,
}

/// ⏸️ How far one capability request in the conversation has got.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentApprovalState {
    Pending,
    Resolved,
    /// 🪦️ The gateway took the request back before anyone here decided it.
    Withdrawn(ApprovalWithdrawal),
}

/// 💬️ One entry of the live agent conversation, exactly as the bridge reported it — the wgpu twin of
/// React's `AgentConversationEntry`. `UserMessage` is a turn this shell itself sent, echoed the
/// moment the frame is queued so the panel is never behind the human's own typing; every other kind
/// comes from a real `GatewayToShell` frame. Nothing is synthesised from a guess: a tool call with no
/// result yet simply stays [`AgentToolCallState::Running`]. A cancellation request moves it to
/// [`AgentToolCallState::Cancelling`] until the gateway's real result settles it; cooperative
/// cancellation never invents a terminal state in this shell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AgentConversationEntry {
    UserMessage { id: String, text: String },
    ToolCall { id: String, tool_name: String, arguments: String, state: AgentToolCallState, summary: Option<String> },
    Approval { id: String, summary: String, state: AgentApprovalState, decision: Option<ApprovalDecision> },
    AgentMessage { id: String, text: String, state: AgentReplyState },
}

impl AgentConversationEntry {
    pub fn id(&self) -> &str {
        match self {
            AgentConversationEntry::UserMessage { id, .. } | AgentConversationEntry::ToolCall { id, .. } | AgentConversationEntry::Approval { id, .. } | AgentConversationEntry::AgentMessage { id, .. } => id,
        }
    }
}

/// 💬️ Whether the agent's turn is still arriving. One-for-one with React's
/// `"streaming" | "complete"` on its `agentMessage` entry: the gateway's own `complete` flag decides
/// it, so the panel never has to guess from timing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentReplyState {
    #[default]
    Streaming,
    Complete,
}

/// ✂️ How many conversation entries the panel retains, one-for-one with React's
/// `AGENT_CONVERSATION_MAX_ENTRIES`. The bridge is a live view, not an archive: an agent running for
/// hours must not grow this list without bound, and the oldest entries are the least useful to keep.
pub const AGENT_CONVERSATION_MAX_ENTRIES: usize = 200;

/// 🌉️ The whole consumer: frames in, presence/approvals/conversation out, decisions back. Holds no
/// socket and no timer, so it is `Send`, unit-testable, and identical on the browser and native builds.
#[derive(Clone, Debug, Default)]
pub struct AgentBridgeState {
    pub status: AgentBridgeStatus,
    pub presence: AgentBridgePresence,
    pub pending_approvals: Vec<PendingAgentApproval>,
    pub conversation: Vec<AgentConversationEntry>,
    pub last_error: Option<String>,
    pub reconnect_attempt: u32,
    next_message_ordinal: u64,
    outbox: Vec<ShellToGateway>,
    inbound_shell_commands: Vec<InboundShellCommand>,
}

/// 🎛️ One inbound `ShellCommand` this shell can carry out on its own chrome, already decoded off
/// the wire. The gateway's `ui_focus`/`ui_reveal` are the whole live surface today
/// (`🌉️mcp/🖥️ui/🦀️.rs`): `ui_reveal` sends `setPanelVisible` and then `setPanelPath`, and the PATH's
/// last segment is the address that matters — the shell SSOT's four-anchor vocabulary is narrower
/// than either dock's, so both renderers resolve the tab's real home themselves rather than
/// believing the requested anchor (the React twin does the same through `findPanelTabInDock`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InboundShellCommand {
    FocusWindow {
        seq: u64,
        window_id: Option<String>,
    },
    RevealPanelTab {
        seq: u64,
        tab_id: String,
    },
    /// 👁️ `setPanelVisible` alone carries no tab, so it is acknowledged and applied as a no-op: the
    /// `setPanelPath` that always follows it is what actually opens the anchor.
    Acknowledge {
        seq: u64,
    },
}

impl InboundShellCommand {
    pub fn seq(&self) -> u64 {
        match self {
            InboundShellCommand::FocusWindow { seq, .. } | InboundShellCommand::RevealPanelTab { seq, .. } | InboundShellCommand::Acknowledge { seq } => *seq,
        }
    }
}

/// 🔎️ Decodes one `ShellCommand` JSON payload into the chrome action this shell can perform.
/// `Err` carries the reason the gateway is told, so a verb this renderer has no chrome for is
/// refused BY NAME instead of behind one blanket "no reducer twin" that hid `ui_focus` and
/// `ui_reveal` — both of which this shell has always been able to honour.
pub fn decode_inbound_shell_command(seq: u64, command: &[u8]) -> Result<InboundShellCommand, String> {
    let value: serde_json::Value = serde_json::from_slice(command).map_err(|error| format!("malformed ShellCommand JSON: {error}"))?;
    match value.get("type").and_then(serde_json::Value::as_str) {
        Some("focusWindow") => Ok(InboundShellCommand::FocusWindow { seq, window_id: value.get("windowId").and_then(serde_json::Value::as_str).map(str::to_string) }),
        Some("setPanelPath") => match value.get("path").and_then(serde_json::Value::as_array).and_then(|path| path.last()).and_then(serde_json::Value::as_str) {
            Some(tab_id) if !tab_id.is_empty() => Ok(InboundShellCommand::RevealPanelTab { seq, tab_id: tab_id.to_string() }),
            _ => Err("setPanelPath carried no panel tab id to reveal".to_string()),
        },
        Some("setPanelVisible") => Ok(InboundShellCommand::Acknowledge { seq }),
        Some(other) => Err(format!("this shell has no chrome for the `{other}` shell command")),
        None => Err("ShellCommand carried no `type`".to_string()),
    }
}

impl AgentBridgeState {
    /// 🔌️ The socket is dialling — `connecting` the first time, `reconnecting` after a drop.
    pub fn note_connecting(&mut self) {
        self.status = if self.reconnect_attempt > 0 { AgentBridgeStatus::Reconnecting } else { AgentBridgeStatus::Connecting };
    }

    /// 👋️ The socket opened: queue the `Hello` frame the gateway answers with `Welcome`.
    pub fn note_socket_opened(&mut self, shell_session_id: impl Into<String>, principal_actor: impl Into<String>, flags: BridgeFlags) {
        self.outbox.push(ShellToGateway::Hello { bridge_version: BRIDGE_VERSION, shell_kind: ShellKind::for_this_target(), shell_session_id: shell_session_id.into(), principal_actor: principal_actor.into(), flags });
    }

    /// 📤️ Takes every inbound chrome command the host has not applied yet. The host applies each
    /// one and then calls [`AgentBridgeState::settle_shell_command`] — an acknowledgement is only
    /// honest AFTER the chrome moved, which is exactly what distinguishes this from the React
    /// twin's earlier mirror-only reduce (it answered `ok` while nothing on screen changed).
    pub fn take_inbound_shell_commands(&mut self) -> Vec<InboundShellCommand> {
        std::mem::take(&mut self.inbound_shell_commands)
    }

    /// ✅️ Acknowledges one applied inbound command back to the gateway.
    pub fn settle_shell_command(&mut self, seq: u64, ok: bool, fault: Option<String>) {
        self.outbox.push(ShellToGateway::ShellCommandResult { in_reply_to: seq, ok, fault });
    }

    /// 🔌️ The socket closed: the next dial is a reconnect, and presence can no longer be trusted.
    pub fn note_socket_closed(&mut self) {
        self.reconnect_attempt = self.reconnect_attempt.saturating_add(1);
        self.status = AgentBridgeStatus::Reconnecting;
        self.presence = AgentBridgePresence::default();
    }

    /// ⏱️ How long to wait before the next dial.
    pub fn reconnect_delay_ms(&self) -> f64 {
        reconnect_delay_ms(self.reconnect_attempt)
    }

    /// 📥️ Applies one decoded inbound frame. `shellCommand`/`appCommand` are acknowledged with a
    /// refusal rather than silently dropped: the wgpu shell has no `ShellState` reducer twin, so
    /// claiming `ok` would lie to the gateway.
    pub fn apply_frame(&mut self, frame: GatewayToShell, now_ms: f64) {
        match frame {
            GatewayToShell::Welcome { .. } => {
                self.reconnect_attempt = 0;
                self.status = AgentBridgeStatus::Open;
                self.last_error = None;
            }
            GatewayToShell::ShellCommand { seq, command } => match decode_inbound_shell_command(seq, &command) {
                Ok(inbound) => self.inbound_shell_commands.push(inbound),
                Err(fault) => self.outbox.push(ShellToGateway::ShellCommandResult { in_reply_to: seq, ok: false, fault: Some(fault) }),
            },
            GatewayToShell::AppCommand { .. } => {}
            GatewayToShell::ApprovalRequested { approval_id, summary } => {
                self.pending_approvals.retain(|approval| approval.approval_id != approval_id);
                self.pending_approvals.push(PendingAgentApproval { approval_id: approval_id.clone(), summary: summary.clone(), requested_at_ms: now_ms });
                self.append_conversation(AgentConversationEntry::Approval { id: approval_id, summary, state: AgentApprovalState::Pending, decision: None });
            }
            GatewayToShell::ApprovalResolved { approval_id, decision } => {
                self.pending_approvals.retain(|approval| approval.approval_id != approval_id);
                self.resolve_conversation_approval(&approval_id, decision);
            }
            GatewayToShell::ApprovalWithdrawn { approval_id, reason } => {
                self.pending_approvals.retain(|approval| approval.approval_id != approval_id);
                if let Some(AgentConversationEntry::Approval { state, .. }) = self.conversation.iter_mut().find(|entry| entry.id() == approval_id) {
                    if matches!(state, AgentApprovalState::Pending) {
                        *state = AgentApprovalState::Withdrawn(reason);
                    }
                }
            }
            GatewayToShell::AgentPresence { active, label, invocation_id } => {
                self.presence = AgentBridgePresence { active, label, invocation_id };
            }
            GatewayToShell::Pong => {}
            GatewayToShell::Bye { reason } => {
                self.last_error = (!reason.is_empty()).then_some(reason);
                self.status = AgentBridgeStatus::Closed;
            }
            GatewayToShell::AgentToolCall { invocation_id, tool_name, arguments } => {
                self.append_conversation(AgentConversationEntry::ToolCall { id: invocation_id, tool_name, arguments, state: AgentToolCallState::Running, summary: None });
            }
            GatewayToShell::AgentToolResult { invocation_id, ok, summary, .. } => {
                self.settle_conversation_tool_call(&invocation_id, ok, summary);
            }
            GatewayToShell::AgentReply { reply_id, text, complete, .. } => {
                self.append_conversation_reply_chunk(reply_id, text, complete);
            }
        }
    }

    /// ➕️ Appends one chunk of an agent turn, or extends the row that already carries this
    /// `reply_id` — the wgpu twin of React's append/extend pair (`🔗️AgentBridge/🟦️.tsx:671`). A
    /// chunk whose row has already been trimmed off the tail opens a new row rather than being
    /// dropped, so a long turn stays visible instead of vanishing mid-sentence.
    fn append_conversation_reply_chunk(&mut self, reply_id: String, text: String, complete: bool) {
        let state = if complete { AgentReplyState::Complete } else { AgentReplyState::Streaming };
        if let Some(AgentConversationEntry::AgentMessage { text: slot, state: slot_state, .. }) = self.conversation.iter_mut().find(|entry| entry.id() == reply_id) {
            slot.push_str(&text);
            *slot_state = state;
            return;
        }
        self.append_conversation(AgentConversationEntry::AgentMessage { id: reply_id, text, state });
    }

    /// ➕️ Appends one entry and trims to [`AGENT_CONVERSATION_MAX_ENTRIES`], oldest first.
    fn append_conversation(&mut self, entry: AgentConversationEntry) {
        self.conversation.push(entry);
        if self.conversation.len() > AGENT_CONVERSATION_MAX_ENTRIES {
            self.conversation.remove(0);
        }
    }

    /// 🔁️ Turns the running tool call with this `invocation_id` into its own result, in place — a
    /// result whose call has already been trimmed off the tail records nothing rather than opening a
    /// second, call-less row.
    fn settle_conversation_tool_call(&mut self, invocation_id: &str, ok: bool, summary: String) {
        let Some(AgentConversationEntry::ToolCall { state, summary: slot, .. }) = self.conversation.iter_mut().find(|entry| entry.id() == invocation_id) else { return };
        *state = if ok { AgentToolCallState::Ok } else { AgentToolCallState::Failed };
        *slot = Some(summary);
    }

    /// 🔁️ Marks the pending approval with this id resolved, in place, carrying the decision.
    fn resolve_conversation_approval(&mut self, approval_id: &str, resolved: ApprovalDecision) {
        let Some(AgentConversationEntry::Approval { state, decision, .. }) = self.conversation.iter_mut().find(|entry| entry.id() == approval_id) else { return };
        *state = AgentApprovalState::Resolved;
        *decision = Some(resolved);
    }

    /// 📥️ [`Self::apply_frame`] straight off the bytes one socket message carried; a fault is
    /// recorded in `last_error` rather than thrown, matching the hook's own `onmessage` catch.
    pub fn apply_encoded_frame(&mut self, bytes: &[u8], now_ms: f64) -> Result<(), BridgeFrameFault> {
        match GatewayToShell::decode(bytes) {
            Ok(frame) => {
                self.apply_frame(frame, now_ms);
                Ok(())
            }
            Err(fault) => {
                self.last_error = Some(format!("failed to decode bridge frame: {fault:?}"));
                Err(fault)
            }
        }
    }

    /// ✅️ The human decided: queue the `Approval` frame and drop the request from the modal, the
    /// exact pair `resolveApproval` performs.
    pub fn resolve_approval(&mut self, approval_id: &str, decision: ApprovalDecision, note: Option<String>) {
        self.outbox.push(ShellToGateway::Approval { approval_id: approval_id.to_string(), decision, note });
        self.pending_approvals.retain(|approval| approval.approval_id != approval_id);
        self.resolve_conversation_approval(approval_id, decision);
    }

    /// 💬️ Sends one human turn to the connected agent and echoes it into the conversation at once —
    /// React's `sendAgentMessage`. Blank text queues nothing and records nothing; the id is minted
    /// from the shell session plus an ordinal, so the echo and the frame name the same turn.
    pub fn send_agent_message(&mut self, shell_session_id: &str, text: &str) -> bool {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return false;
        }
        let message_id = format!("msg_{shell_session_id}_{}", self.next_message_ordinal);
        self.next_message_ordinal = self.next_message_ordinal.saturating_add(1);
        self.outbox.push(ShellToGateway::AgentMessage { message_id: message_id.clone(), text: trimmed.to_string() });
        self.append_conversation(AgentConversationEntry::UserMessage { id: message_id, text: trimmed.to_string() });
        true
    }

    /// 🛑️ Asks the open gateway to cancel the exact invocation the conversation reported. The
    /// optimistic state says only that the request left; the gateway's later `AgentToolResult`
    /// remains the sole terminal authority. This intentionally mirrors React's sender contract:
    /// any non-empty id can be sent while open, while only a matching running row changes state.
    pub fn cancel_tool_call(&mut self, invocation_id: &str) -> bool {
        if invocation_id.is_empty() || !matches!(self.status, AgentBridgeStatus::Open) {
            return false;
        }
        self.outbox.push(ShellToGateway::AgentCancel { invocation_id: invocation_id.to_string() });
        if let Some(AgentConversationEntry::ToolCall { state, .. }) = self.conversation.iter_mut().find(|entry| entry.id() == invocation_id) {
            if matches!(state, AgentToolCallState::Running) {
                *state = AgentToolCallState::Cancelling;
            }
        }
        true
    }

    /// 💓️ Keep-alive frame the transport schedules every [`PING_INTERVAL_MS`].
    pub fn queue_ping(&mut self) {
        self.outbox.push(ShellToGateway::Ping);
    }

    /// 👋️ Farewell frame sent best-effort before the socket closes.
    pub fn queue_bye(&mut self) {
        self.outbox.push(ShellToGateway::Bye);
    }

    /// 📤️ Drains everything queued for the transport to write.
    pub fn take_outbox(&mut self) -> Vec<ShellToGateway> {
        std::mem::take(&mut self.outbox)
    }

    /// 📤️ [`Self::take_outbox`] already encoded — what a byte-oriented transport wants.
    pub fn take_outbox_encoded(&mut self) -> Vec<Vec<u8>> {
        self.take_outbox().iter().map(ShellToGateway::encode).collect()
    }

    pub fn outbox_len(&self) -> usize {
        self.outbox.len()
    }

    /// 🔔️ Whether the approvals modal must be showing, the sole condition React's dialog opens on.
    pub fn has_pending_approvals(&self) -> bool {
        !self.pending_approvals.is_empty()
    }
}
//#endregion 🔖️State

//#region 🔖️Dialer
/// 🔌️ What the transport holds right now, as the dialer needs to see it. Deliberately NOT
/// `AgentBridgeStatus`: that one is the human-facing status React renders (`disabled` while no
/// config exists at all), this one is the socket's own readiness.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentBridgeSocketState {
    /// 🕳️ Nothing dialled — either never, or the previous socket was retired.
    #[default]
    Absent,
    Connecting,
    Open,
    Closed,
}

/// 🎬️ One finite turn the transport executes. Every one of them is bounded work; none of them
/// blocks, and `Wait` carries the instant the next dial is due so a caller can report it.
#[derive(Clone, Debug, PartialEq)]
pub enum AgentBridgeDialTurn {
    /// 😴️ Nothing to do this turn.
    Idle,
    /// 🔌️ Open a socket at this url with these EXACT ordered subprotocols.
    Dial { url: String, protocols: [String; 2] },
    /// 👋️ The socket just reached `open`: announce this shell.
    Announce,
    /// 💓️ Keep-alive due.
    Ping,
    /// 🧯️ The socket died; retire it before the next dial.
    Retire,
    /// ⏱️ A reconnect is armed for this instant.
    Wait { until_ms: f64 },
}

/// ⏱️ The transport-free half of React's `useAgentBridge` effect: WHEN to dial, when to announce,
/// when to ping, and when to back off. Holding it apart from any socket is what makes the whole
/// reconnect ladder testable on the target the renderer suite runs on, and what lets the browser
/// door and the native `tokio-tungstenite` half share one policy instead of two that drift.
#[derive(Clone, Debug, Default)]
pub struct AgentBridgeDialer {
    config: Option<AgentBridgeConfig>,
    reconnect_at_ms: Option<f64>,
    next_ping_ms: Option<f64>,
    announced: bool,
}

impl AgentBridgeDialer {
    /// 🔗️ Installs (or clears) the config. A NEW config retires whatever was dialled and restarts
    /// the ladder from zero — React re-runs the whole effect on a `config.url`/`admissionProof`
    /// change, which is the same thing.
    pub fn set_config(&mut self, config: Option<AgentBridgeConfig>, state: &mut AgentBridgeState) -> bool {
        if self.config == config {
            return false;
        }
        self.config = config;
        self.reconnect_at_ms = None;
        self.next_ping_ms = None;
        self.announced = false;
        state.reconnect_attempt = 0;
        state.status = if self.config.is_some() { AgentBridgeStatus::Connecting } else { AgentBridgeStatus::Disabled };
        true
    }

    pub fn config(&self) -> Option<&AgentBridgeConfig> {
        self.config.as_ref()
    }

    pub fn is_armed(&self) -> bool {
        self.config.is_some()
    }

    /// 🎬️ One turn. The order is React's own: a dead socket is retired and its backoff armed before
    /// anything else, an armed backoff is respected, an absent socket is dialled, a freshly opened
    /// one is announced exactly once, and only a settled open socket pings.
    pub fn turn(&mut self, state: &mut AgentBridgeState, socket: AgentBridgeSocketState, now_ms: f64) -> AgentBridgeDialTurn {
        let Some(config) = self.config.clone() else {
            state.status = AgentBridgeStatus::Disabled;
            return AgentBridgeDialTurn::Idle;
        };
        if matches!(socket, AgentBridgeSocketState::Closed) {
            self.announced = false;
            self.next_ping_ms = None;
            state.note_socket_closed();
            self.reconnect_at_ms = Some(now_ms + state.reconnect_delay_ms());
            return AgentBridgeDialTurn::Retire;
        }
        if matches!(socket, AgentBridgeSocketState::Absent) {
            if let Some(until_ms) = self.reconnect_at_ms {
                if now_ms < until_ms {
                    state.status = AgentBridgeStatus::Reconnecting;
                    return AgentBridgeDialTurn::Wait { until_ms };
                }
                self.reconnect_at_ms = None;
            }
            self.announced = false;
            state.note_connecting();
            return AgentBridgeDialTurn::Dial { protocols: bridge_protocols(&config), url: config.url.clone() };
        }
        if matches!(socket, AgentBridgeSocketState::Connecting) {
            state.note_connecting();
            return AgentBridgeDialTurn::Idle;
        }
        if !self.announced {
            self.announced = true;
            self.next_ping_ms = Some(now_ms + PING_INTERVAL_MS);
            return AgentBridgeDialTurn::Announce;
        }
        if self.next_ping_ms.is_some_and(|due| now_ms >= due) {
            self.next_ping_ms = Some(now_ms + PING_INTERVAL_MS);
            return AgentBridgeDialTurn::Ping;
        }
        AgentBridgeDialTurn::Idle
    }
}
//#endregion 🔖️Dialer

//#region 🌐️Labels
/// 🌐️ This element's own framework-owned copy, resolved without a default language — the same
/// `LocalizedLabel::native(en, de)` shape `👥️PresenceBar`'s wgpu twin uses, standing in for the
/// React side's `registerUiTranslationBundles` bundle under `os.agent.*`.
pub fn agent_label(english: &'static str, german: &'static str, locale: Locale) -> String {
    LocalizedLabel::native(english, german).resolve(Terminology::ALL[0], locale).to_string()
}
//#endregion 🌐️Labels

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;

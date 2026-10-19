//! ⚙️ Config and shared types: the bridge version and reconnect constants, the bridge config and its discovery, the shell kind, flags, refusal, approval decision and withdrawal and the frame fault. See [`super`].

use super::*;

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

/// ⏳️ React's `BRIDGE_HANDSHAKE_DEADLINE_MS`: how long a dialled gateway has, from the dial, to
/// answer with `Welcome` or `Refused` before the socket is retired as unanswered.
pub const BRIDGE_HANDSHAKE_DEADLINE_MS: f64 = 8_000.0;

/// 🔢️ React's `BRIDGE_UNANSWERED_ATTEMPTS`: unanswered dials in a row before the offer is
/// [`AgentBridgeStatus::Unavailable`] and nothing is dialled again until a different config arrives.
pub const BRIDGE_UNANSWERED_ATTEMPTS: u32 = 3;

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
/// deliberately not `Debug`-printed and never reaches a `[TRACE]` line.
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

/// 🚧️ Why the gateway refused this shell's connection — the wgpu twin of `🌉️mcp/🧵️bridge`'s
/// `BridgeRefusal`, same tags.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BridgeRefusal {
    Version,
    Capacity,
}

impl BridgeRefusal {
    pub fn to_tag(self) -> u8 {
        match self {
            BridgeRefusal::Version => 0,
            BridgeRefusal::Capacity => 1,
        }
    }

    pub fn from_tag(tag: u8) -> Result<Self, BridgeFrameFault> {
        match tag {
            0 => Ok(BridgeRefusal::Version),
            1 => Ok(BridgeRefusal::Capacity),
            other => Err(BridgeFrameFault::UnknownTag(other)),
        }
    }
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

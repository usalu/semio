//! 🆔️ Connection identity and limits of the bridge session: the connection id, the per-connection entry, the agent inbox message and the outbox, broadcast and retirement ceilings. See [`super`].

use super::*;

//#region 🔖️BridgeHandle
/// 🆔️ One live `/bridge` connection's id — `Copy`/`Eq`/`Hash` so it keys a map and passes around
/// freely; `Display` is the same string the connection's own `Welcome.connection` field carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShellConnectionId(pub u64);

impl std::fmt::Display for ShellConnectionId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "conn_{}", self.0)
    }
}

pub(super) struct ConnectionEntry {
    pub generation: u64,
    pub outbox: Arc<BridgeOutbox>,
    /// 🚩️ The flags this shell declared in its own `Hello`. `relay_app_commands` is what
    /// `🐚️channel::SessionChannelBinding` reads to decide whether the artifact verbs of a session
    /// may execute IN this shell: a shell that does not claim the relay is never handed one.
    pub flags: BridgeFlags,
    pub last_shell_state: Option<ShellToGateway>,
    pub last_instances: Option<Vec<BridgeInstanceRef>>,
    pub last_command_result: Option<(u64, bool, Option<String>)>,
    /// 🗿️ The shell's most recent `AppFrames` reply, by correlation id — `(in_reply_to,
    /// instance_id, frames)`. Read by `ShellArtifactChannel::exchange`, which only ever accepts the
    /// entry whose `in_reply_to` equals the `seq` it minted, exactly as `ui_focus`/`ui_reveal`
    /// already do with `last_command_result`.
    pub last_app_frames: Option<(u64, String, Vec<Vec<u8>>)>,
    pub last_approval: Option<(String, ApprovalDecision, Option<String>)>,
    /// 💬️ Human turns this shell typed at the agent, oldest first, capped at
    /// [`BRIDGE_AGENT_INBOX_MAX_ITEMS`]. Read-once: `take_agent_messages` drains it, so the
    /// `semio://ui/agent-messages` resource the agent polls never replays a turn it already read
    /// and never grows without bound if no agent is polling.
    pub(super) agent_inbox: std::collections::VecDeque<AgentInboxMessage>,
}

/// 💬️ One human turn waiting for the connected agent to read it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentInboxMessage {
    pub message_id: String,
    pub text: String,
    pub received_at_ms: u64,
}

/// 📥️ How many unread human turns one connection retains. A shell whose agent never polls drops
/// its OLDEST turn rather than growing without bound — the same fixed-credit rule the outbox uses.
pub const BRIDGE_AGENT_INBOX_MAX_ITEMS: usize = 64;

pub(crate) const BRIDGE_OUTBOX_MAX_ITEMS: usize = 64;

pub(crate) const BRIDGE_OUTBOX_MAX_BYTES: usize = 1_048_576;

pub(crate) const BRIDGE_BROADCAST_MAX_RECIPIENTS: usize = 64;

pub(super) const BRIDGE_BROADCAST_MAX_BYTES: usize = BRIDGE_OUTBOX_MAX_BYTES * BRIDGE_BROADCAST_MAX_RECIPIENTS;

pub(crate) const BRIDGE_OUTBOX_PAGE_BYTES: usize = 16_384;

pub(crate) const BRIDGE_OUTBOX_MAX_PAGES: usize = BRIDGE_OUTBOX_MAX_BYTES.div_ceil(BRIDGE_OUTBOX_PAGE_BYTES);

pub(super) const BRIDGE_BROADCAST_MAX_PENDING: usize = 64;

pub(super) const BRIDGE_RETIREMENT_MAX_PENDING: usize = 256;

pub(super) const BRIDGE_ASYNC_RETRY_MS: u64 = 1;

#[derive(Clone, Copy)]
pub(super) struct BridgeRetirementGrant {
    pub generation: u64,
}

pub(super) struct BridgeRetirementCursor {
    pub(crate) pages: [Option<Box<[u8; BRIDGE_OUTBOX_PAGE_BYTES]>>; BRIDGE_OUTBOX_MAX_PAGES],
    pub page: usize,
}
//#endregion 🔖️BridgeHandle

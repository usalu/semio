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

use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;

#[path = "⚙️config-and-shared-types/🦀️.rs"]
mod config_and_shared_types;
pub use config_and_shared_types::{
    bridge_protocols, discover_agent_bridge_config, reconnect_delay_ms, AgentBridgeConfig,
    ApprovalDecision, ApprovalWithdrawal, BridgeFlags, BridgeFrameFault, BridgeRefusal, ShellKind,
    AGENT_BRIDGE_FIELD_MAX_BYTES, BRIDGE_HANDSHAKE_DEADLINE_MS, BRIDGE_SUBPROTOCOL,
    BRIDGE_UNANSWERED_ATTEMPTS, BRIDGE_VERSION, PING_INTERVAL_MS, RECONNECT_BASE_MS,
    RECONNECT_MAX_MS,
};

#[path = "📨️wire-and-gateway-messages/🦀️.rs"]
mod wire_and_gateway_messages;
pub use wire_and_gateway_messages::{GatewayToShell, ShellToGateway};

#[path = "🧠️bridge-state-and-dialer/🦀️.rs"]
mod bridge_state_and_dialer;
pub use bridge_state_and_dialer::{
    agent_label, decode_inbound_shell_command, AgentApprovalState, AgentBridgeDialTurn,
    AgentBridgeDialer, AgentBridgePresence, AgentBridgeSocketState, AgentBridgeState,
    AgentBridgeStatus, AgentBridgeVersionMismatch, AgentConversationEntry, AgentReplyState,
    AgentToolCallState, InboundShellCommand, PendingAgentApproval, AGENT_CONVERSATION_MAX_ENTRIES,
};

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;

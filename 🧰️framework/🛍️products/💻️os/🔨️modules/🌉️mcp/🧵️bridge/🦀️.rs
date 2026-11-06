//! 🧵️ `ShellBridge` frame codec (`📋️master.md` §2.2 verbatim variant list, `📌️sol-P1b-packet.md` [Design notes](README.md#source-notes-1).

//#region 🔖️BridgeVersion

pub const BRIDGE_VERSION: u16 = 1;

//#endregion 🔖️BridgeVersion

#[path = "📏️wire/🦀️.rs"]
mod wire;
#[path = "🧱️shared-types/🦀️.rs"]
mod shared_types;
#[path = "📤️shell-to-gateway/🦀️.rs"]
mod shell_to_gateway;
#[path = "🔍️shell-decode-cursor/🦀️.rs"]
mod shell_decode_cursor;
#[path = "🧬️shell-materialize-cursor/🦀️.rs"]
mod shell_materialize_cursor;
#[path = "📥️gateway-to-shell/🦀️.rs"]
mod gateway_to_shell;

pub use gateway_to_shell::{AGENT_CONVERSATION_MAX_TEXT, GatewayToShell, SELF_PUBLISHING_CONVERSATION_TOOLS, bridge_wire_field_len, truncate_conversation_text};
pub use shared_types::{ApprovalDecision, ApprovalWithdrawal, BridgeFlags, BridgeInstanceRef, BridgeRefusal, ShellKind};
pub use shell_decode_cursor::{BRIDGE_INBOUND_MAX_FIELD_BYTES, BRIDGE_INBOUND_PAGE_BYTES, ShellDecodeFault, ShellDecodeStep, ShellFrameKind, ShellToGatewayDecodeCursor};
pub use shell_materialize_cursor::{ShellMaterializeStep, ShellToGatewayMaterializeCursor};
pub use shell_to_gateway::ShellToGateway;

//#region 🔖️BridgeServer
//#endregion 🔖️BridgeServer

//#region 🧪️Tests
//#endregion 🧪️Tests

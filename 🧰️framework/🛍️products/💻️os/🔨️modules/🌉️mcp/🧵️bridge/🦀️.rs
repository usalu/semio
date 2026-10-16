//! 🧵️ `ShellBridge` frame codec (`📋️master.md` §2.2 verbatim variant list, `📌️sol-P1b-packet.md`
//! §2.4) — hand-rolled binary framing (`tag: u8` + fields in declaration order), `BRIDGE_VERSION = 1`.
//! This file is the Rust SSOT; `🟦️.ts` is the TS twin, and `🧫️fixtures/📨️frames.json` is the
//! anti-drift mechanism proving both codecs agree byte-for-byte (`mod quick`'s
//! `every_fixture_round_trips_through_the_rust_codec` test; the TS side of the same fixtures is
//! exercised by a foreground `bun run` script, not wired into this crate's own test run).
//!
//! Field types are hand-picked for a self-contained wire format: this facet deliberately does NOT
//! depend on the peer `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ticket's `os_spr::wire` primitives
//! (`📡️spr/🧵️channel/🦀️.rs` is their exclusive, mid-rewrite territory per
//! `📌️important.md`'s collision matrix) — see this packet's report §7 for the full rationale. Opaque
//! payloads (`ShellState.state`, `ShellStatePatch.patch`, `ShellCommand.command`,
//! `AppCommand.command`, each `AppFrames.frames[i]`) are carried as length-prefixed byte blobs; what
//! goes inside them (the real packed `ShellState`) is `💻️os/🔨️modules/🖥️shell`'s concern (P9), not
//! this codec's.
//!
//! The live bridge is owned by the retained HTTP transport through `🔖️BridgeHandle`; the Axum
//! `🔖️BridgeServer` remains a `cfg(test)` differential oracle only.

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

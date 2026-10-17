//! 🪢️ Session of the bridge crate: Shell bridge connection handle, outbox and agent conversation.

use semio_framework_async::Job;
use semio_framework_async::Lane;
use semio_framework_async::ProcessKind;
use semio_framework_async::WorkerPool;
use semio_framework_async::WorkerPoolConfig;
use semio_framework_async::WorkerSubmitErrorKind;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use crate::shell_command_dispatch::BridgeSlot;
use semio_framework_os_mcp_bridge_codec::bridge::{ApprovalDecision, BridgeFlags, BridgeInstanceRef, GatewayToShell, ShellToGateway, truncate_conversation_text};
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_wire::job_registry::job_registry;
use semio_framework_os_mcp_wire::notify::agent_messages_changed;

#[path = "🆔️connection-identity-and-limits/🦀️.rs"]
mod connection_identity_and_limits;
pub use connection_identity_and_limits::*;

#[path = "📢️broadcast-cursor/🦀️.rs"]
mod broadcast_cursor;
pub use broadcast_cursor::*;

#[path = "⏳️async-authority/🦀️.rs"]
mod async_authority;
use async_authority::*;

#[path = "🎞️encoded-frames/🦀️.rs"]
mod encoded_frames;
pub use encoded_frames::*;

#[path = "📮️outbox/🦀️.rs"]
mod outbox;
pub use outbox::*;

#[path = "🤝️handle/🦀️.rs"]
mod handle;
pub use handle::*;

#[path = "💬️agent-conversation/🦀️.rs"]
mod agent_conversation;
pub use agent_conversation::*;

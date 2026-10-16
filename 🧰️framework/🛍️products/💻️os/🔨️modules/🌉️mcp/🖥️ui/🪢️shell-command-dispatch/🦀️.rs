//! 🪢️ Shell command dispatch of the bridge crate: Shell bridge connection handle, outbox and agent conversation.

use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::Instant;
use crate::session::{BridgeHandle, ShellConnectionId};
use semio_framework_os_mcp_bridge_codec::bridge::GatewayToShell;
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_foundation::rendezvous::{live_os_sessions, sessions_dir};

/// 🔌️ The late-bound `/bridge` handle. `McpServer` is built BEFORE the HTTP transport creates its
/// `BridgeHandle` (the handle is minted from the transport's own worker pool, so hoisting its
/// construction would mean a second pool), and `stdio` never serves `/bridge` at all — so the tool
/// registry captures this slot at build time and the transport fills it in once, on `start`. An
/// unfilled slot is exactly the "no bridge at all" tier, not an error.
pub type BridgeSlot = Arc<OnceLock<Arc<BridgeHandle>>>;

/// 🔎️ Resolves the slot to the live handle, if one was ever published into it.
pub fn resolve_bridge(slot: Option<&BridgeSlot>) -> Option<&Arc<BridgeHandle>> {
    slot.and_then(|slot| slot.get())
}

//#region 🔖️ShellCommandDispatch

/// ⏱️ How long `ui_focus`/`ui_reveal` block waiting for the attached shell's `ShellCommandResult`
/// before giving up — a shell that never replies must never hang a tool call forever.
const SHELL_COMMAND_TIMEOUT_MS: u64 = 4_000;

const SHELL_COMMAND_POLL_INTERVAL_MS: u64 = 20;

static SHELL_COMMAND_SEQ: AtomicU64 = AtomicU64::new(1);

fn next_shell_command_seq() -> u64 {
    SHELL_COMMAND_SEQ.fetch_add(1, Ordering::Relaxed)
}

/// 🕳️ No `BridgeHandle` at all — this gateway serves no `/bridge`, so no shell can ever attach to
/// it: it was started with `--no-bridge`, or its loopback listener could not be bound or offered
/// (the gateway's stderr names which). The details name the rendezvous a shell would have dialled.
pub fn bridge_not_running_error() -> GatewayError {
    GatewayError::new(
        GatewayErrorCode::PluginUnavailable,
        "this gateway serves no `/bridge`, so no shell can attach to it — it was started with `--no-bridge` or its loopback listener could not be offered; restart this MCP server without `--no-bridge`",
    )
    .with_details(serde_json::json!({
        "liveOsSessionsNow": live_os_sessions().len(),
        "sessionsDirectory": sessions_dir().to_string_lossy(),
        "bindWith": ["stdio (without --no-bridge)", "http"],
    }))
    .retryable()
}

/// 🕳️ A `BridgeHandle` exists but no shell connection is live — the normal headless state, not a
/// bug. The gateway's offer is already published, so a shell started now dials it on its own.
pub fn no_shell_attached_error() -> GatewayError {
    GatewayError::new(GatewayErrorCode::PluginUnavailable, "no shell is attached to `/bridge` yet — start `bun run dashboard run playground:s` (it dials this gateway on its own) and retry").retryable()
}

/// 🧭️ The connection this facet routes `ShellCommand`/`semio://window…` reads through — the
/// highest (most recently registered) `ShellConnectionId`, i.e. the most-recently-connected shell.
/// `🧵️bridge` carries no notion of "the" primary shell yet; a later packet that needs explicit
/// multi-shell targeting adds a `shellId` argument to these tools without touching this fallback.
pub fn active_shell_connection(bridge: &BridgeHandle) -> Option<ShellConnectionId> {
    bridge.connections().into_iter().max()
}

/// 📤️ Encodes `command_json` the same way `AgentBridge`'s renderer-side hook does (opaque JSON
/// bytes inside `GatewayToShell::ShellCommand.command` — `🧵️bridge`'s own design note: the codec
/// never interprets these bytes), sends it to the active shell connection, and blocks (bounded by
/// `timeout`, polling `BridgeHandle::last_command_result`) for a `ShellCommandResult` whose
/// `in_reply_to` matches the `seq` this call minted. A stale/unrelated result already sitting on the
/// connection is never mistaken for this call's reply (`in_reply_to` must match exactly).
pub fn dispatch_shell_command_with_timeout(bridge: &BridgeHandle, command_json: serde_json::Value, timeout: Duration) -> Result<(), GatewayError> {
    let connection_id = active_shell_connection(bridge).ok_or_else(no_shell_attached_error)?;
    let seq = next_shell_command_seq();
    let payload = serde_json::to_vec(&command_json).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("failed to encode ShellCommand payload: {error}")))?;
    if !bridge.send_to(connection_id, GatewayToShell::ShellCommand { seq, command: payload }) {
        return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "the attached shell's connection closed while dispatching this command").retryable());
    }
    let deadline = Instant::now() + timeout;
    loop {
        if let Some((in_reply_to, ok, fault)) = bridge.last_command_result(connection_id) {
            if in_reply_to == seq {
                return if ok { Ok(()) } else { Err(GatewayError::new(GatewayErrorCode::SideEffectRejected, fault.unwrap_or_else(|| "the shell rejected this command".to_string()))) };
            }
        }
        if Instant::now() >= deadline {
            return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("the attached shell did not reply to this command within {}ms", timeout.as_millis())).retryable());
        }
        std::thread::sleep(Duration::from_millis(SHELL_COMMAND_POLL_INTERVAL_MS));
    }
}

pub fn dispatch_shell_command(bridge: &BridgeHandle, command_json: serde_json::Value) -> Result<(), GatewayError> {
    dispatch_shell_command_with_timeout(bridge, command_json, Duration::from_millis(SHELL_COMMAND_TIMEOUT_MS))
}

//#endregion 🔖️ShellCommandDispatch

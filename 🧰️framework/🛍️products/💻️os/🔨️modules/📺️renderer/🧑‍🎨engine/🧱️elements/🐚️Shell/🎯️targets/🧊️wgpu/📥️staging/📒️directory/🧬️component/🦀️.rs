//! 📒️ `shell-directory` component structs: `DirectoryState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// 📒️ `DirectoryState`: identity, the directory client, agent bridge, inference port, hub workspace, space administration and presence.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `directory`.
pub struct DirectoryState {
    /// 💬️ The `framework.chat` composer's unsent text. The transcript itself is NOT held here:
    /// it is [`crate::agent_bridge::AgentBridgeState::conversation`], the live bridge conversation
    /// (`GatewayToShell::AgentToolCall`/`AgentToolResult`/`ApprovalRequested` in,
    /// `ShellToGateway::AgentMessage` out), exactly as React's `useAgentBridge().conversation` is.
    pub agent_chat_draft: String,
    /// 🪪️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C3 — the restored-or-
    /// minted session, `None` with no hub env (unchanged local-only behaviour) or before boot's
    /// bootstrap thread reports back. Native only, see this region's header note above.
    pub identity: Option<Identity>,
    /// 🔐️ Exact public `/auth/sessions/me` authority that verified `identity`. This is cleared with
    /// the client and never inferred from a cached identity or a mutable UI boolean.
    pub verified_session_authority: Option<DirectorySessionAuthorityV1>,
    /// 📶️ Set when `mint_or_restore` degraded to the last cached identity because the hub was
    /// unreachable — never blocks, never clears `identity` itself.
    pub identity_offline: bool,
    /// 🌱️ `S_HUB_URL`/`S_USER`/`S_DATA_DIR`, resolved once at `boot()` — `None` means "no hub env",
    /// the sentinel every identity/binding/directory code path below checks first.
    pub identity_env: Option<IdentityEnv>,
    /// 🎭️ Per-process session id — the `{sessionId}` half of contract §C0's `user:{userId}#{sessionId}`
    /// actor grammar, minted once in `ShellState::new` and stable for this process's lifetime.
    pub shell_session_id: String,
    /// ⏳️ Browser identity retry floor — see [`BROWSER_IDENTITY_RETRY_MS`].
    #[cfg(target_arch = "wasm32")]
    pub identity_retry_at_ms: u64,
    /// 📡️ Non-blocking identity result channel, completed by a retained-waker pool future.
    #[cfg(not(target_arch = "wasm32"))]
    pub identity_bootstrap_rx: Option<std::sync::mpsc::Receiver<Result<IdentityOutcome, String>>>,
    /// 🛑️ Cancellable handle for the finite-turn identity bootstrap future.
    #[cfg(not(target_arch = "wasm32"))]
    pub identity_bootstrap_task: Option<std::sync::Arc<ShellPoolFuture>>,
    /// 📇️ The hub directory client used to issue `os.directory.*` commands (§C6) — constructed once
    /// identity resolves, holding the session token.
    pub directory_client: Option<std::sync::Arc<ShellDirectoryClient>>,
    /// 🌉️ The MCP agent bridge's TRANSPORT (packet W15e). Holds the socket and the dial ladder; the
    /// consumer it feeds is `chrome_build.agent`, which stays transport-free. Disarmed — and therefore
    /// inert — until a supervisor hands over a config, exactly as React's own `discoverAgentBridgeConfig`
    /// answering `null` leaves its hook `disabled`.
    pub agent_bridge: crate::agent_bridge_door::AgentBridgeTransport,
    /// 🛰️ The scope this shell asks the agent-bridge supervisor with ([`AgentBridgeOfferScopeOwnerV1`]).
    pub agent_bridge_offer_scope: AgentBridgeOfferScopeOwnerV1,
    /// 💡️ The one retained host-owned inference port, live only while its document's execution
    /// target is verified. It is never a document command and nothing it holds is persisted.
    #[cfg(not(target_arch = "wasm32"))]
    pub inference_port: Option<std::sync::Arc<ShellInferenceRunner>>,
    /// 💡️ The last status the retained port published, rendered by the shell's own chrome.
    #[cfg(not(target_arch = "wasm32"))]
    pub inference_port_status: Option<InstalledServiceStatusV1>,
    /// 🏠️ Shell-lifetime Home projection and its fetch → terminal Config receipt → ACK → live owner.
    #[cfg(not(target_arch = "wasm32"))]
    pub directory_home: Option<DirectoryHomeProjection>,
    /// 🔌️ ONE `NativeDirectoryTransport` (cheap to `.clone()` — see its own doc), shared by every
    /// `DirectoryClient` this shell constructs so they all draw on the SAME `HttpPool` byte budget/
    /// outstanding-request accounting rather than each minting a disjoint pool.
    pub directory_transport: ShellDirectoryTransport,
    /// 🛑️ Shell-lifetime cancellation root shared by identity, requests, and the directory runner.
    pub directory_cancel: CancelToken,
    /// 🎮️ Bounded FIFO of sealed, idempotency-correlated `os.directory.*` operations plus their
    /// transient result slot — flushed opportunistically every frame once `directory_client` exists
    /// (contract §C6: "commands queue in the shell (bounded, in-memory) and flush on reconnect").
    /// Each entry keeps its exact request bytes, so a retry is the SAME command to the hub's
    /// digest-keyed idempotency store and can never mint a second invitation.
    pub directory_commands: ShellDirectoryCommandQueueV1,
    /// 🔐️ The hub workspace state behind the `/hub` overlay — hub book, session phase, space rows,
    /// roster and drafts. Target-neutral: the surface is the same retained tree on both arms, and
    /// only the transport underneath it is split.
    pub hub_workspace: crate::hub_connection::HubWorkspaceState,
    /// 🔐️ The sign-in whose network legs are in flight off the interaction state ([`ShellHubTask`]).
    pub hub_sign_in_task: Option<ShellHubTask<ShellHubSignInAnswer>>,
    /// 🏘️ The spaces read in flight, on the same kind of task.
    pub hub_spaces_task: Option<ShellHubTask<ShellHubSpacesAnswer>>,
    pub hub_workspace_open: bool,
    /// 🏛️ The one retained native space-administration operation (fixed capacity: exactly one).
    /// Its page, receipt, and one-shot invite capability are erased by every terminal transition, so
    /// an identity change, a 401/403, or a scoped 4401 leaves nothing administrable behind.
    pub space_administration: Option<ShellSpaceAdministrationOperationV1>,
    /// 🔢️ Monotonic operation epoch; a late turn for a replaced operation is dropped, never applied.
    pub space_administration_epoch: u64,
    /// 👥️ ticket §5 — shell-LOCAL presence roster from the currently attached document's
    /// `ArtifactEvent::Presence`, deliberately NOT folded into the shared kernel `ViewModel`.
    pub presence_peers: Vec<PresencePeer>,
    /// 👥️ The canonical surface id (`surface_app_id`) the CURRENTLY attached document's hub binding
    /// used, if any — `presence_peers` is scoped to this surface; a document opened without a hub
    /// binding (local-only) carries `None` and renders no roster.
    pub presence_surface: Option<String>,
    /// 🪪️ This shell's own hub-admitted presence identity on the attached document — the actor and
    /// palette index the hub's `Session` frame named. Board overlays never paint it, and the local
    /// colour is the hub's, never a guess (React's `publishLocalPresenceActorV1`).
    pub presence_self: Option<(String, u8)>,
    /// 🖱️ The last pointer position the shell saw, in logical screen space — what a board window's
    /// presence view turns into the world point under the pointer on the next heartbeat.
    pub presence_pointer: Option<(f32, f32)>,
}

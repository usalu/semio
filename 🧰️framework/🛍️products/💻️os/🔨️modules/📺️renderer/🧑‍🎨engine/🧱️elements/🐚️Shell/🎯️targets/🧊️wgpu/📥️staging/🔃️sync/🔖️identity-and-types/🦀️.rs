//! 🔃️ `shell-sync` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 🔖️IdentityPure
/// 🛟️ One rebootstrap reseed: the control the actor raised, the pair fetch in flight, and when to ask again after a failure.
pub(crate) struct ShellSyncReseed {
    pub(crate) control: semio_framework_os_kernel::os_directory::RebootstrapRequired,
    pub(crate) pending: Option<ShellDetached<Result<semio_framework_os_kernel::os_directory::CanonicalCheckpointPairV1, String>>>,
    pub(crate) retry_at_ms: f64,
}

/// 🪢️ What a hub document's seed step asks: the document's canonical checkpoint pair, admitted only as the
/// checkpoint its execution-target lease authorizes, over the shell's signed-in directory client.
#[derive(Clone)]
pub(crate) struct ShellHubDocumentSeed {
    pub(crate) client: std::sync::Arc<ShellDirectoryClient>,
    pub(crate) ctx: OperationContext,
    pub(crate) scope: semio_framework_os_kernel::os_directory::DocumentScope,
    pub(crate) checkpoint: semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1,
}

/// 🧷️ An open whose host half is prepared — the prior document retired, the socket surface and
/// execution target bound — waiting for the guest to hold the document before its actor binds.
pub(crate) struct ShellPreparedDocumentOpen {
    pub(crate) document_id: String,
    pub(crate) schema: String,
    pub(crate) bindings: Vec<PersistenceBinding>,
    pub(crate) backbone_uri: Option<String>,
    pub(crate) seed: Option<ShellHubDocumentSeed>,
    pub(crate) plugin: ProgramBridgeEntry,
    pub(crate) session: ActiveSession,
}

//#endregion 🔖️IdentityPure
//#region ShellTypes
//#region 🔖️NativeSyncChannel
/// 🧵️ One open document's live `framework/sync` actor channel held by the wgpu shell on both
/// targets. Mirrors `os-shell.tsx`'s `openArtifactSessionsRef` entry: the shell owns the `cmd_tx`/event
/// receiver while the sandboxed plugin instance's store pumps through the registered
/// `ChannelBackbone` (see `framework/product/os/core/rs`'s `ArtifactHost` canonical sequence).
pub struct ShellSyncChannel {
    pub document_id: String,
    pub document_key: ArtifactDocumentKey,
    pub actor_uri: String,
    pub instance_id: u32,
    pub plugin_id: String,
    pub binding_generation: u64,
    pub cmd_tx: ArtifactMailboxSender,
    pub events: tokio::sync::broadcast::Receiver<ArtifactEvent>,
    pub connected_at_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ShellSyncOwner {
    pub(crate) plugin_id: String,
    pub(crate) instance_id: u32,
    pub(crate) binding_generation: u64,
    pub(crate) actor_uri: String,
    pub(crate) document_key: String,
}

pub(crate) fn shell_sync_channel_owner(channel: &ShellSyncChannel) -> ShellSyncOwner {
    ShellSyncOwner { plugin_id: channel.plugin_id.clone(), instance_id: channel.instance_id, binding_generation: channel.binding_generation, actor_uri: channel.actor_uri.clone(), document_key: shell_hub_document_key(&channel.document_key) }
}

//#endregion 🔖️NativeSyncChannel
//#endregion ShellTypes


//! 🧱️ Channel record of the plugin artifact channel: the channel with its guest runtime, session and budget state, and the lazily built inference route. See [`super`].

use super::*;

//#region 🔖️ArtifactChannel
/// 🔌️ Owns real guest sessions, prepared transactions, history and explicit caller context.
#[cfg(not(target_arch = "wasm32"))]
pub struct PluginArtifactChannel {
    pub host_driver_policy:semio_plugin_host::GuestRelayDriverPolicy,
    pub operation_policy:semio_os_mcp_port::channel_port::HostOperationPolicy,
    pub(super) startup_failure:Option<semio_plugin_host::GuestRelayAdmissionFailure>,
    pub(super) host_driver:Arc<semio_plugin_host::GuestRelayWakeDriver>,
    /// 🐎️ The process-wide compiled runtime ([`shared_plugin_runtime`]), held as the `GuestRuntimes` enum: the inference lane registers it as a run origin, and it MUST be the same one that compiled `compiled`.
    /// 🧠️ The process-wide firstparty interpreter of the retained compiled component.
    pub runtime: Arc<GuestRuntimes>,
    pub compiled: semio_plugin_host::CompiledHandle,
    /// 🔌️ The plugin this channel drives. It is the id alone, never a registry ROW: the row names a
    /// repo path this channel has no use for, and a hub-authorized component has no repo path at all.
    pub plugin_id: String,
    pub descriptor: semio_framework::PackageDescriptor,
    pub app_ref: semio_framework::AppRef,
    pub actor_label: String,
    pub(super) instances: HashMap<u32, semio_plugin_host::GuestInstance>,
    pub(super) closing_instances:std::collections::BTreeSet<u32>,
    pub(super) pending_exchanges: PendingExchangeRegistry<1>,
    pub(super) pending_command_closes: semio_framework::kernel::CommandDriverRegistry<1>,
    pub(super) rejected_command_builds: semio_framework::kernel::RejectedCommandBuildRegistry<1>,
    pub(super) next_seq: u64,
    /// 🗿️ The artifact id this channel's ONE live guest session document is, when the owning [Design notes](README.md#source-notes-36).
    pub(super) session_artifact_id: Option<String>,
    /// 📥️ Per guest instance, the artifact id whose canonical pair that guest's session document
    /// currently IS — see [`Self::load_session_document`]. Empty for a guest still sitting on the
    /// plugin's own genesis document, which is every folder-lane guest and was, before ticket
    /// 26/09/18 slice M10, every hub-lane one as well.
    pub(crate) session_documents: HashMap<u32, String>,
    /// 🧵️ Document-backbone messages this guest published and this channel has not handed on yet — [Design notes](README.md#source-notes-50).
    pub(super) backbone_egress: Vec<Vec<u8>>,
    /// 🧵️ The instances whose document backbone this channel has bound — see
    /// [`Self::ensure_document_backbone`]. Cleared per instance by [`Self::discard_instance`],
    /// because a guest that was thrown away holds no binding.
    pub(super) backbone_bindings: std::collections::BTreeSet<u32>,
}

//#endregion 🔖️ArtifactChannel

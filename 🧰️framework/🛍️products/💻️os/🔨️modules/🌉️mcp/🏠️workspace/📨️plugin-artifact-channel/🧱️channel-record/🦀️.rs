//! 🧱️ Channel record of the plugin artifact channel: the channel with its guest runtime, session and budget state, and the lazily built inference route. See [`super`].

use super::*;

//#region 🔖️ArtifactChannel
/// 🔌️ Real `crate::actions::ArtifactChannel` implementation — the seam `P6-actions-policy`'s
/// `ActionAdapter` drives (`🔀️dispatch/🦀️.rs`'s own module doc: "is P7's job when it
/// implements `ArtifactChannel` for real"). `ReadHistory` is driven for REAL over
/// `GuestRuntime::execute_turn` (one retained command page per turn, scan `TurnResult.effects` for
/// `Effect::Respond{req, result}` where `req.0 == seq`, decode `RequestOutcome::Ok` as a real
/// `store::AppFrame::HistorySnapshot` — exactly the "real body" `🏃️run/🦀️.rs`'s own
/// `WasmtimeNodeHost::exchange` doc comment describes and does not itself build). The mutation
/// variants (`PureCommand`/`TransactionPrepare`/`Commit`/`Rollback`/`Undo`/`Redo`) need a real
/// `manifest::ActionInvocation{address: ActionAddress{plugin_id, app_id, mode_id, window_kind_id,
/// window_instance_id, action_id}, arguments}` — every one of those five address fields beyond
/// `plugin_id`/`app_id` is normally supplied by a LIVE window/mode the headless gateway has none of;
/// inventing placeholder values for them is a real design decision (what does a headless agent's
/// "window"/"mode" even mean) this packet is not positioned to make unilaterally in the time
/// available, so — same honesty rule as `🏃️run`'s own `run_transaction`/`undo_transaction_group` —
/// these return a well-formed, typed `"channel.not-wired"` `Fault` naming exactly what is missing,
/// never a fabricated result.
#[cfg(not(target_arch = "wasm32"))]
pub struct PluginArtifactChannel {
    pub host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy,
    pub(super) host_driver:Arc<semio_framework_plugin_host::GuestRelayWakeDriver>,
    /// 🐎️ The process-wide compiled runtime ([`shared_plugin_runtime`]), held as the `GuestRuntimes`
    /// enum rather than a bare `WasmtimeRuntime` because the inference lane's
    /// `PluginInstanceHandle` takes exactly that `Arc` — and it MUST be the same one: a
    /// `wasmtime::Component` belongs to the `Engine` that compiled it, so a second runtime could not
    /// instantiate `compiled` at all.
    pub runtime: Arc<GuestRuntimes>,
    pub compiled: semio_framework_plugin_host::CompiledHandle,
    /// 🔌️ The plugin this channel drives. It is the id alone, never a registry ROW: the row names a
    /// repo path this channel has no use for, and a hub-authorized component has no repo path at all.
    pub plugin_id: String,
    pub descriptor: semio_framework::PackageDescriptor,
    pub app_ref: semio_framework::AppRef,
    pub actor_label: String,
    pub(super) instances: HashMap<u32, semio_framework_plugin_host::GuestInstance>,
    pub(super) pending_exchanges: PendingExchangeRegistry<1>,
    pub(super) pending_command_closes: semio_framework::kernel::CommandDriverRegistry<1>,
    pub(super) rejected_command_builds: semio_framework::kernel::RejectedCommandBuildRegistry<1>,
    pub(super) next_seq: u64,
    /// 🗿️ The artifact id this channel's ONE live guest session document is, when the owning
    /// workspace has bound exactly one artifact to this plugin. Every `RevisionStamp` this channel
    /// answers names it.
    ///
    /// 🐛️ Before ticket 26/09/18 slice M8 the stamp carried `entry.plugin_id`, so the headless
    /// lane's `expectedRevision` named `"note"` while `artifact_create`/`artifact_snapshot` named
    /// `"…-typed"` — two ids for one document, and `artifact_snapshot(revisionAfter.artifactId)`
    /// answered `no such artifact: note` (`📓️ce1-client-e2e-pinning-and-puzzle-bound.md` §8 gap 4).
    pub(super) session_artifact_id: Option<String>,
    /// 📥️ Per guest instance, the artifact id whose canonical pair that guest's session document
    /// currently IS — see [`Self::load_session_document`]. Empty for a guest still sitting on the
    /// plugin's own genesis document, which is every folder-lane guest and was, before ticket
    /// 26/09/18 slice M10, every hub-lane one as well.
    pub(crate) session_documents: HashMap<u32, String>,
    /// 🧵️ Document-backbone messages this guest published and this channel has not handed on yet —
    /// the `Effect::SendMessage { target: Backbone { uri } }` payloads addressed at this channel's
    /// own actor uri. They used to be dropped on the floor: `exchange_one_turn` named every effect
    /// for diagnostics and admitted only the `Shell{instance}` reply lane, so a committed
    /// transaction's envelopes — the ONE thing that has to leave the process for anyone else to see
    /// the edit — went nowhere at all.
    pub(super) backbone_egress: Vec<Vec<u8>>,
    /// 🧵️ The instances whose document backbone this channel has bound — see
    /// [`Self::ensure_document_backbone`]. Cleared per instance by [`Self::discard_instance`],
    /// because a guest that was thrown away holds no binding.
    pub(super) backbone_bindings: std::collections::BTreeSet<u32>,
    /// 💡️ Lazily opened on the FIRST `AppCommand::Infer` — a second guest activation of the same
    /// component, dedicated to the cold `semio.infer` job lane. Deliberately separate from
    /// `instances` above: `PluginInstanceHandle` takes ownership of its `GuestInstance` and drives
    /// `start-job`/`step-job` against it from a worker pool, which the retained command-page loop
    /// `exchange_one_real` runs on the same instance cannot share.
    pub(super) inference: Option<PluginInferenceRoute>,
}

/// 💡️ One plugin's real `semio.infer` route — the SAME `ArtifactInferenceRouter` +
/// `PluginInstanceHandle` pair `🏃️run/🦀️.rs`'s own `load_runtime_recursive` builds, constructed here
/// from the committed descriptor instead of from a live `run` process. The router owns the handle
/// (which owns the `Arc<GuestRuntimes>` and the `GuestInstance`), so this struct holds only it.
#[cfg(not(target_arch = "wasm32"))]
pub(super) struct PluginInferenceRoute {
    pub router: semio_framework_plugin_host::ArtifactInferenceRouter,
}
//#endregion 🔖️ArtifactChannel

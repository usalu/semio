//! 🧵️ Reserved tool job of the tool job contract: the owned job request, the reserved input and request, the reserved job trait and state and the reserved tool job with its interactive job and drop implementations. See [`super`].

use super::*;
use crate::ArtifactInstanceOperationOwnerHandle;

#[path="../🎟️request/🔒️command/🦀️.rs"]
mod original_command;
pub use original_command::ArtifactOriginalToolCommand;

/// 🎒️ Fully owned input handed to an app-defined factory builder after exact raw admission.
pub struct ArtifactOwnedToolJobRequest<A: ArtifactApp> {
    pub source_binding: semio_framework::action_bus::OriginalToolSourceBinding,
    pub retained:RetainedCloneGrant,
    pub command: ArtifactOriginalToolCommand<A::Command>,
    pub input: Option<semio_framework::action_bus::RetainedToolWireInput>,
    pub checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    pub operation: semio_framework_job::Operation,
    pub controller_id: String,
    pub tool_id: String,
    pub payload_schema_id: String,
    pub contract: semio_framework::ToolExecutionContract,
    pub decoded_items: usize,
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub canonical_base_revision: [u8; 32],
    pub authoring_seed: String,
    pub snapshot: std::sync::Arc<A::Snapshot>,
    /// 🔐️ Exact immutable store authority retained by producers until their private owners close.
    pub snapshot_read: store::os_store::snapshot_read::SnapshotRead<A::Snapshot>,
    pub config: std::sync::Arc<A::Config>,
    pub window_config: Option<WindowConfigSnapshot>,
    pub history: std::sync::Arc<HistoryView>,
    pub interaction_state: std::sync::Arc<protocol::InteractionState>,
    pub interaction_hover: std::sync::Arc<InteractionHoverState>,
    pub context: ArtifactOwnedContextHandle<A>,
    pub instance_operation_owner: ArtifactInstanceOperationOwnerHandle,
    pub output_chunks: ArtifactOutputChunks,
    pub completion: ArtifactToolCompletion<A>,
}
#[path="../🎟️request/♻️retirement/🦀️.rs"]
mod original_request_retirement;


/// 🧭️ Owned input for an app-defined implementation of a framework-reserved route. The runtime
/// selects the exact owner/factory/tool/schema proof before constructing this value.
pub enum ArtifactReservedToolInput {
    Action { args: Option<DslValue>, interaction: protocol::InteractionState, hover: InteractionHoverState },
    Media { port: String, media: Media },
}

/// 🧊️ Immutable app state plus an owned reserved-route input. Implementations only construct a
/// persistent job here; producer work belongs in that job's bounded `step` calls.
pub struct ArtifactReservedToolJobRequest<A: ArtifactApp> {
    pub retained:RetainedCloneGrant,
    pub controller_id: String,
    pub tool_id: String,
    pub payload_schema_id: String,
    pub contract: semio_framework::ToolExecutionContract,
    pub app_instance_id: u32,
    /// 🌐️ Caller-selected ephemeral presentation axes, when a host context is present.
    pub ui_axes: Option<(Locale, Terminology)>,
    pub parent_document_id: String,
    pub canonical_base_revision: [u8; 32],
    pub authoring_seed: String,
    pub snapshot: std::sync::Arc<A::Snapshot>,
    /// 🧾️ Exact immutable store authority for retained reserved-route source traversal.
    pub snapshot_read: store::os_store::snapshot_read::SnapshotRead<A::Snapshot>,
    pub config: std::sync::Arc<A::Config>,
    pub history: std::sync::Arc<HistoryView>,
    /// 🧩️ The parent's composed children, the same view `ArtifactView` hands the synchronous
    /// `A::import_media`. A composing app's LIVE scene is its children, not its own snapshot
    /// (sequence's steps live on the `content` Flow child), and the mounted route builds the emit
    /// exclusively from the resumable job — so without this a composed app could not write a
    /// resumable importer at all and every delivery died `interactive-job.missing-reserved-builder`
    /// (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, media §8). Cheap to clone
    /// (`Option<Arc<ChildContentRoot>>`); a job releases it as soon as it has decoded.
    pub children: ChildContentView,
    pub raw_wire: Vec<u8>,
    pub input: ArtifactReservedToolInput,
    pub completion: ArtifactToolCompletion<A>,
}

/// 📬️ Completed document mutations paired with their ephemeral output.
pub(super) type ArtifactToolEmission<A> = (ArtifactMutationOutcome<<A as ArtifactApp>::Mutation, <A as ArtifactApp>::ConfigMutation, <A as ArtifactApp>::DraftMutation>, EphemeralEmit<A>);

pub use semio_framework_plugin_original_job_ownership::{ArtifactReservedJob,ArtifactReservedToolJob,ArtifactReservedToolJobState};

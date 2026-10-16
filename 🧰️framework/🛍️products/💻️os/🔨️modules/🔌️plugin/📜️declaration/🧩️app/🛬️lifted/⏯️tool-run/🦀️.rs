use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin_window_config::WindowConfigSnapshot;
use crate::ArtifactApp;
use semio_framework_tool_run::ToolRunDefinition;
use semio_framework_tool_run::ToolRunIdentity;
use std::sync::Arc;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPort;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPurpose;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunTraceKeys;

/// 🧳️ Everything an app needs to build one run or revalidate job (§3.7).
///
/// - `snapshot` is the run's base for `Run` and the committed head for `Revalidate`.
/// - `window_id` is the window instance the run was started from (`toolRunStart`'s `windowId`, else the
///   dispatching view's window), and `window_config` that window's config snapshot as of this build.
/// - `checkpoint` is the last `StepOutcome::CheckpointReady` state of this run (reconfigure `resume`).
/// - `provisional` is the provisional op list the job continues from or revalidates.
/// - `instance_owner` is the instance's retained operation owner, shared with its commands.
/// - `port` carries the host effects the job asks for and the wake of a job waiting on them.
/// - `trace_keys` allocates trace keys no record of this run uses, e.g. one per placement a revalidate job
///   re-tests; every job of the run shares it, so keys never collide across rebuilds and purposes.
/// - `entity_marks` holds `(provisional length at the end of the tick that appended it, entity)` per entity, in
///   append order: which provisional op prefix each placement's entity belongs to.
/// - `children` is the composed read the job runs on: the live owned children, whose member a member run
///   (`ToolRunDefinition::member`) edits.
/// - `member_ops` is a member run's provisional op list (that member's `OpBinary` ops), the one the job continues
///   from or revalidates; empty for a run on the document's own store.
pub struct ToolRunJobRequest<'a, A: ArtifactApp> {
    pub receiving: &'a semio_framework_tool_run::ToolRunReceivingLimits,
    pub tool_id: &'a str,
    pub definition: &'a ToolRunDefinition,
    pub purpose: ToolRunJobPurpose,
    pub identity: ToolRunIdentity,
    pub snapshot: Arc<A::Snapshot>,
    pub config: Arc<A::Config>,
    pub window_id: Option<&'a str>,
    pub window_config: Option<WindowConfigSnapshot>,
    pub checkpoint: Option<&'a [u8]>,
    pub provisional: &'a [A::Mutation],
    pub instance_owner: ArtifactInstanceOperationOwnerHandle,
    pub port: ToolRunJobPort,
    pub trace_keys: ToolRunTraceKeys,
    pub entity_marks: &'a [(u32, u64)],
    pub children: ChildContentView,
    pub member_ops: &'a [Vec<u8>],
}

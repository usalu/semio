//! 🧵️ The mounted artifact instance. See [ownership and lifecycle](README.md).
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as pack;

use semio_plugin_decl_app::ArtifactApp;
use store::{MemberFactory, NoMembers, ReferenceSpaceMember};

pub use semio_plugin_instance_typed_ops::original_typing::OriginalTypingDispatch;
pub use semio_plugin_job::original_emission::OriginalEmissionSource;

/// 🪝️ Instance extensions in route, refusal and maintenance order.
pub type InstanceExtensions = (semio_plugin_tool_run_turns::ToolRunExtension, (semio_plugin_time_travel_turns::TimeTravelExtension, ()));
use semio_plugin_time_travel::TimeTravelLedger;
use semio_plugin_tool_machine::ToolMachineRuntime;
use semio_plugin_tool_run::ToolRunLedger;

pub struct VcsArtifactApp<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory = NoMembers> {
    pub app: A,
    pub mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1,
    pub original_controller_close:Option<semio_plugin_instance_initialization::services::OriginalControllerClose<A>>,
    pub constructor: ConstructorState<A>,
    pub stores: InstanceStores<A>,
    pub roster: PeerRoster<A>,
    /// 🪙️ Parser slots retain unused original arguments and faults until paid close.
    pub ingress: ActionIngress,
    pub original_typing:OriginalTypingDispatch<A>,
    pub original_emission:OriginalEmissionSource<A>,
    pub original_typed_command:semio_plugin_command::command_admission::original_typed_command::OriginalTypedCommandSource<A>,
pub original_host_event_command:semio_plugin_command::command_admission::original_typed_command::OriginalTypedCommandSource<A>,
pub original_command_scope:semio_plugin_instance_dispatch::host_event_routing::CommandLease,
pub original_host_event_scope:Option<(semio_framework_job::OriginalSourceBinding,(semio_framework_value::retained_clone::NativeAllocationIdentity,u64))>,
    pub original_typed_tag:std::mem::ManuallyDrop<Option<semio_plugin_tool_machine::ToolTag>>,
    pub original_typed_tag_close:semio_framework_value::retained_clone::RetainedCloneClose,
    pub interaction: LocalInteraction<A>,
    pub history: CommandHistory<A>,
    pub host_events: HostEvents,
    pub txn: Transactions<A>,
    pub child_composition: ChildComposition<M>,
    pub pending: PendingReserved<A>,
    pub tools: ToolOperations<A>,
    pub media: MediaTransfers<A>,
    pub ingest: EnvelopeJobs<A, M>,
    pub maintenance: MaintenanceCursors,
    pub close: CloseCursors<A, M>,
    /// ⏯️ Local tool runs render provisional operations and write one final edit.
    pub tool_runs: ToolRunLedger<A>,
    /// ⏪️ Local history edits preview draft replays and write one final commit.
    pub time_travel: TimeTravelLedger<A>,
    /// 🎚️ Window controls render provisional values and publish one transaction on release.
    pub tool_machines: ToolMachineRuntime<A::Snapshot, A::Mutation, A::Config, A::ConfigMutation>,
}

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🔍️ Read-only inspection of this instance's tool run ledger for app-level tests.
    #[cfg(feature = "artifact-app-testing")]
    pub fn tool_run_ledger(&self) -> &ToolRunLedger<A> {
        &self.tool_runs
    }
}

pub use crate::plugin_app_impl::{FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS, settle_framework_reserved_admission, settle_framework_reserved_commit};
#[cfg(any(test, feature = "artifact-app-testing"))]
pub use crate::plugin_app_impl::drive_framework_reserved_spawn_like_host;
/// 🧬️ Renderer-neutral semantic UI vocabulary shared by every mounted plugin.
pub use semio_ui_contract::*;
impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory> Drop for VcsArtifactApp<A, M> {
    fn drop(&mut self) {
        self.tools.tool_cancellations.cancel_scope_generation();
        assert!(std::thread::panicking() || (self.pending.pending_reserved.is_empty() && self.pending.reserved_commits.is_empty()), "framework-reserved routes retire their leases through the paid close before the app drops");
    }
}

//#region 📥️StagedInstanceMounts

#[path = "📥️staging/📨️dispatch/🦀️.rs"]
mod app_dispatch;
#[path="📥️staging/📣️host-event-routing/🦀️.rs"]
mod app_host_event_routing;

#[path = "📥️staging/🔒️close-ladder/🦀️.rs"]
mod app_close_ladder;

#[path = "📥️staging/🚪️ingress/🦀️.rs"]
mod app_ingress;

#[path = "📥️staging/🪜️maintenance-ladder/🦀️.rs"]
mod app_maintenance_ladder;

#[path = "🔌️plugin-app-impl/🦀️.rs"]
#[doc(hidden)]
pub mod plugin_app_impl;

#[path = "📥️staging/🏗️ctx-builders/🦀️.rs"]
mod app_ctx_builders;
pub use semio_plugin_instance_extension::{ExtensionCtx, ExtensionRef, InstanceExtension};
pub use semio_plugin_instance_children::children_instance_root::genesis_member_schema;
pub use semio_plugin_instance_children::children_plugin_app_impl::MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY;
pub use semio_plugin_instance_docs::docs_instance_root::{artifact_handle_of, typed_operation_document_is_fresh};
pub use semio_plugin_instance_docs::docs_maintenance_ladder::item;
pub use semio_plugin_instance_docs::docs_plugin_app_impl::history_changed_event;
pub use semio_plugin_instance_ladders::ladders_document_replacement::{retire_document_window_registry_demands, retire_document_window_registry_step};
pub use semio_plugin_instance_ladders::ladders_plugin_app_impl::{MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE, MAINTENANCE_DOCUMENT_DISPLACED_STAGE, MAINTENANCE_STAGES};
pub use semio_plugin_instance_stores::stores_components::{ConstructorState, InstanceStores, PeerRoster, ActionIngress, LocalInteraction};
pub use semio_plugin_instance_history::history_components::{CommandHistory, HostEvents, Transactions};
pub use semio_plugin_instance_children::children_components::ChildComposition;
pub use semio_plugin_instance_jobs::jobs_components::{PendingReserved, ToolOperations};
pub use semio_plugin_instance_docs::docs_components::{MediaTransfers, EnvelopeJobs};
pub use semio_plugin_instance_ladders::ladders_components::{MaintenanceCursors, CloseCursors};
#[path = "📥️staging/🧬️composer-api/🦀️.rs"]
mod app_composer_api;
pub use semio_plugin_instance_render::render_bundle::{RenderCtx, RenderRef};
pub use semio_plugin_instance_dispatch::dispatch_bundle::DispatchCtx;
pub use semio_plugin_instance_typed_ops::typed_bundle::TypedCtx;
pub use semio_plugin_time_travel_turns::time_travel_bundle::{TimeTravelCtx, TimeTravelRef};
pub use semio_plugin_tool_run_turns::tool_run_bundle::{ToolRunCtx, ToolRunRef};
pub use semio_plugin_instance_commands::command_bundle::CommandCtx;
pub use semio_plugin_instance_ladders::ladder_bundle::{LadderCtx, LadderRef};
pub use semio_plugin_instance_render::render_history_view::history_command;
//#endregion 📥️StagedInstanceMounts

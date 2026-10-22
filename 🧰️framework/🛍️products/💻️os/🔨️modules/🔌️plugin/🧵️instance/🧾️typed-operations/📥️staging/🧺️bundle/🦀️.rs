//! 🏓️ Borrowed bundle of the `instance-typed-ops` use cases: `TypedCtx` (WP7 of the instance family).
//!
//! 🧺️ The fields carry the names of the composer fields they borrow, so a method body written for the composer reads the same; `X` is the registered `InstanceExtension` list.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_instance_children::children_components::ChildComposition;
use semio_framework_plugin_instance_history::history_components::{CommandHistory, HostEvents, Transactions};
use semio_framework_plugin_instance_dispatch::dispatch_bundle::DispatchCtx;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_instance_docs::docs_components::{EnvelopeJobs, MediaTransfers};
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_instance_stores::stores_components::{InstanceStores, LocalInteraction, PeerRoster};
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_instance_jobs::jobs_components::{PendingReserved, ToolOperations};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_instance_render::render_bundle::RenderCtx;
use semio_framework_plugin_time_travel::TimeTravelLedger;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolMachineRuntime;
use semio_framework_plugin_tool_run::ToolRunLedger;
use semio_framework_ui_contract::*;
use std::marker::PhantomData;

/// 🧺️ The exclusive borrows the `typed-ops` use cases need; built by the composer (`VcsArtifactApp::typed_ctx`) or reborrowed by a higher bundle.
pub struct TypedCtx<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X> {
    pub app: &'ctx mut A,
    pub mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,
    pub stores: &'ctx mut InstanceStores<A>,
    pub roster: &'ctx mut PeerRoster<A>,
    pub interaction: &'ctx mut LocalInteraction<A>,
    pub history: &'ctx mut CommandHistory<A>,
    pub host_events: &'ctx mut HostEvents,
    pub txn: &'ctx mut Transactions<A>,
    pub child_composition: &'ctx mut ChildComposition<M>,
    pub pending: &'ctx mut PendingReserved<A>,
    pub tools: &'ctx mut ToolOperations<A>,
    pub media: &'ctx mut MediaTransfers<A>,
    pub ingest: &'ctx mut EnvelopeJobs<A, M>,
    pub tool_runs: &'ctx mut ToolRunLedger<A>,
    pub time_travel: &'ctx mut TimeTravelLedger<A>,
    pub tool_machines: &'ctx mut ToolMachineRuntime<A::Snapshot, A::Mutation, A::Config, A::ConfigMutation>,
    pub extension: PhantomData<fn() -> X>,
}

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X> TypedCtx<'ctx, A, M, X> {
    /// ⬇️ Reborrows the `dispatch` bundle this bundle bridges to.
    pub fn as_dispatch(&mut self) -> DispatchCtx<'_, A, M, X> {
        DispatchCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, host_events: &mut *self.host_events, txn: &mut *self.txn, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, media: &mut *self.media, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }

    /// ⬇️ Reborrows the `render` bundle this bundle bridges to.
    pub fn as_render(&mut self) -> RenderCtx<'_, A, M, X> {
        RenderCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }
}

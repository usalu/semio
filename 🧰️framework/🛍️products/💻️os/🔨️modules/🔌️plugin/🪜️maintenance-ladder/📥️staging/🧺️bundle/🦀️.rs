//! 🐁️ Borrowed bundle of the `instance-ladders` use cases: `LadderCtx` and its shared twin `LadderRef` (WP7 of the instance family).
//!
//! 🧺️ The fields carry the names of the composer fields they borrow, so a method body written for the composer reads the same; `X` is the registered `InstanceExtension` list.
use semio_framework_plugin_instance_stores::stores_components::{ActionIngress, InstanceStores, LocalInteraction, PeerRoster};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_instance_children::children_components::ChildComposition;
use crate::ladders_components::{CloseCursors, MaintenanceCursors};
use semio_framework_plugin_instance_history::history_components::{CommandHistory, HostEvents, Transactions};
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_instance_docs::docs_components::{EnvelopeJobs, MediaTransfers};
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_instance_jobs::jobs_components::{PendingReserved, ToolOperations};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_instance_render::render_bundle::{RenderCtx, RenderRef};
use semio_framework_plugin_time_travel_turns::time_travel_bundle::{TimeTravelCtx, TimeTravelRef};
use semio_framework_plugin_time_travel::TimeTravelLedger;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolMachineRuntime;
use semio_framework_plugin_tool_run_turns::tool_run_bundle::{ToolRunCtx, ToolRunRef};
use semio_framework_plugin_tool_run::ToolRunLedger;
use semio_framework_plugin_instance_typed_ops::typed_bundle::TypedCtx;
use semio_framework_ui_contract::*;
use std::marker::PhantomData;

/// 🧺️ The exclusive borrows the `ladders` use cases need; built by the composer (`VcsArtifactApp::ladder_ctx`) or reborrowed by a higher bundle.
pub struct LadderCtx<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X> {
    pub app: &'ctx mut A,
    pub mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,
    pub stores: &'ctx mut InstanceStores<A>,
    pub roster: &'ctx mut PeerRoster<A>,
    pub ingress: &'ctx mut ActionIngress,
    pub interaction: &'ctx mut LocalInteraction<A>,
    pub history: &'ctx mut CommandHistory<A>,
    pub host_events: &'ctx mut HostEvents,
    pub txn: &'ctx mut Transactions<A>,
    pub child_composition: &'ctx mut ChildComposition<M>,
    pub pending: &'ctx mut PendingReserved<A>,
    pub tools: &'ctx mut ToolOperations<A>,
    pub media: &'ctx mut MediaTransfers<A>,
    pub ingest: &'ctx mut EnvelopeJobs<A, M>,
    pub maintenance: &'ctx mut MaintenanceCursors,
    pub close: &'ctx mut CloseCursors<A, M>,
    pub tool_runs: &'ctx mut ToolRunLedger<A>,
    pub time_travel: &'ctx mut TimeTravelLedger<A>,
    pub tool_machines: &'ctx mut ToolMachineRuntime<A::Snapshot, A::Mutation, A::Config, A::ConfigMutation>,
    pub extension: PhantomData<fn() -> X>,
}

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X> LadderCtx<'ctx, A, M, X> {
    /// 🔍️ The shared twin over the same borrows, for the `&self` members.
    pub fn shared(&self) -> LadderRef<'_, A, M, X> {
        LadderRef { app: &*self.app, stores: &*self.stores, roster: &*self.roster, ingress: &*self.ingress, interaction: &*self.interaction, history: &*self.history, txn: &*self.txn, child_composition: &*self.child_composition, pending: &*self.pending, tools: &*self.tools, media: &*self.media, ingest: &*self.ingest, maintenance: &*self.maintenance, close: &*self.close, tool_runs: &*self.tool_runs, time_travel: &*self.time_travel, tool_machines: &*self.tool_machines, extension: PhantomData }
    }

    /// ⬇️ Reborrows the `render` bundle this bundle bridges to.
    pub fn as_render(&mut self) -> RenderCtx<'_, A, M, X> {
        RenderCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }

    /// ⬇️ Reborrows the `time_travel` bundle this bundle bridges to.
    pub fn as_time_travel(&mut self) -> TimeTravelCtx<'_, A, M, X> {
        TimeTravelCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, host_events: &mut *self.host_events, txn: &mut *self.txn, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, media: &mut *self.media, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }

    /// ⬇️ Reborrows the `tool_run` bundle this bundle bridges to.
    pub fn as_tool_run(&mut self) -> ToolRunCtx<'_, A, M, X> {
        ToolRunCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }

    /// ⬇️ Reborrows the `typed` bundle this bundle bridges to.
    pub fn as_typed(&mut self) -> TypedCtx<'_, A, M, X> {
        TypedCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, host_events: &mut *self.host_events, txn: &mut *self.txn, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, media: &mut *self.media, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }
}

/// 🔍️ The shared borrows the `ladders` `&self` use cases need (`Copy`: every field is a shared reference).
pub struct LadderRef<'v, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X> {
    pub app: &'v A,
    pub stores: &'v InstanceStores<A>,
    pub roster: &'v PeerRoster<A>,
    pub ingress: &'v ActionIngress,
    pub interaction: &'v LocalInteraction<A>,
    pub history: &'v CommandHistory<A>,
    pub txn: &'v Transactions<A>,
    pub child_composition: &'v ChildComposition<M>,
    pub pending: &'v PendingReserved<A>,
    pub tools: &'v ToolOperations<A>,
    pub media: &'v MediaTransfers<A>,
    pub ingest: &'v EnvelopeJobs<A, M>,
    pub maintenance: &'v MaintenanceCursors,
    pub close: &'v CloseCursors<A, M>,
    pub tool_runs: &'v ToolRunLedger<A>,
    pub time_travel: &'v TimeTravelLedger<A>,
    pub tool_machines: &'v ToolMachineRuntime<A::Snapshot, A::Mutation, A::Config, A::ConfigMutation>,
    pub extension: PhantomData<fn() -> X>,
}

impl<'v, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X> Clone for LadderRef<'v, A, M, X> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'v, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X> Copy for LadderRef<'v, A, M, X> {}

impl<'v, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X> LadderRef<'v, A, M, X> {
    /// ⬇️ Shares the `render` view of the same borrows.
    pub fn render_ref(&self) -> RenderRef<'_, A, M, X> {
        RenderRef { app: self.app, stores: self.stores, history: self.history, child_composition: self.child_composition, tools: self.tools, ingest: self.ingest, time_travel: self.time_travel, tool_machines: self.tool_machines, extension: PhantomData }
    }

    /// ⬇️ Shares the `time_travel` view of the same borrows.
    pub fn time_travel_ref(&self) -> TimeTravelRef<'_, A, M, X> {
        TimeTravelRef { stores: self.stores, txn: self.txn, child_composition: self.child_composition, tool_runs: self.tool_runs, time_travel: self.time_travel, extension: PhantomData }
    }

    /// ⬇️ Shares the `tool_run` view of the same borrows.
    pub fn tool_run_ref(&self) -> ToolRunRef<'_, A, M, X> {
        ToolRunRef { stores: self.stores, child_composition: self.child_composition, tool_runs: self.tool_runs, extension: PhantomData }
    }
}

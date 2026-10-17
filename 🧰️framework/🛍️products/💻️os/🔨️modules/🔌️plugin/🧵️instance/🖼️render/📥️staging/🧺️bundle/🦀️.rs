//! 🏑️ Borrowed bundle of the `instance-render` use cases: `RenderCtx` and its shared twin `RenderRef` (WP7 of the instance family).
//!
//! 🧺️ The fields carry the names of the composer fields they borrow, so a method body written for the composer reads the same; `X` is the registered `InstanceExtension` list.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_instance_children::children_components::ChildComposition;
use semio_framework_plugin_instance_history::history_components::CommandHistory;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_instance_docs::docs_components::EnvelopeJobs;
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_instance_stores::stores_components::{InstanceStores, LocalInteraction, PeerRoster};
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_instance_jobs::jobs_components::{PendingReserved, ToolOperations};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_time_travel::TimeTravelLedger;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolMachineRuntime;
use semio_framework_plugin_tool_run::ToolRunLedger;
use semio_framework_ui_contract::*;
use std::marker::PhantomData;

/// 🧺️ The exclusive borrows the `render` use cases need; built by the composer (`VcsArtifactApp::render_ctx`) or reborrowed by a higher bundle.
pub struct RenderCtx<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> {
    pub app: &'ctx mut A,
    pub mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,
    pub stores: &'ctx mut InstanceStores<A>,
    pub roster: &'ctx mut PeerRoster<A>,
    pub interaction: &'ctx mut LocalInteraction<A>,
    pub history: &'ctx mut CommandHistory<A>,
    pub child_composition: &'ctx mut ChildComposition<M>,
    pub pending: &'ctx mut PendingReserved<A>,
    pub tools: &'ctx mut ToolOperations<A>,
    pub ingest: &'ctx mut EnvelopeJobs<A, M>,
    pub tool_runs: &'ctx mut ToolRunLedger<A>,
    pub time_travel: &'ctx mut TimeTravelLedger<A>,
    pub tool_machines: &'ctx mut ToolMachineRuntime<A::Snapshot, A::Mutation, A::Config, A::ConfigMutation>,
    pub extension: PhantomData<fn() -> X>,
}

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> RenderCtx<'ctx, A, M, X> {
    /// 🔍️ The shared twin over the same borrows, for the `&self` members.
    pub fn shared(&self) -> RenderRef<'_, A, M, X> {
        RenderRef { app: &*self.app, stores: &*self.stores, history: &*self.history, child_composition: &*self.child_composition, tools: &*self.tools, ingest: &*self.ingest, time_travel: &*self.time_travel, tool_machines: &*self.tool_machines, extension: PhantomData }
    }
}

/// 🔍️ The shared borrows the `render` `&self` use cases need (`Copy`: every field is a shared reference).
pub struct RenderRef<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> {
    pub app: &'v A,
    pub stores: &'v InstanceStores<A>,
    pub history: &'v CommandHistory<A>,
    pub child_composition: &'v ChildComposition<M>,
    pub tools: &'v ToolOperations<A>,
    pub ingest: &'v EnvelopeJobs<A, M>,
    pub time_travel: &'v TimeTravelLedger<A>,
    pub tool_machines: &'v ToolMachineRuntime<A::Snapshot, A::Mutation, A::Config, A::ConfigMutation>,
    pub extension: PhantomData<fn() -> X>,
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> Clone for RenderRef<'v, A, M, X> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> Copy for RenderRef<'v, A, M, X> {}


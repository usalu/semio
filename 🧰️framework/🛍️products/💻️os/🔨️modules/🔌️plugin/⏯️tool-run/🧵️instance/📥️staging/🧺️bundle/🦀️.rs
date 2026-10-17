//! 🏹️ Borrowed bundle of the `tool-run-turns` use cases: `ToolRunCtx` and its shared twin `ToolRunRef` (WP7 of the instance family).
//!
//! 🧺️ The fields carry the names of the composer fields they borrow, so a method body written for the composer reads the same; `X` is the registered `InstanceExtension` list.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_instance_children::children_components::ChildComposition;
use semio_framework_plugin_instance_history::history_components::CommandHistory;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_instance_docs::docs_components::EnvelopeJobs;
use semio_framework_plugin_instance_extension::{ExtensionCtx, ExtensionRef, InstanceExtension};
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_instance_stores::stores_components::{InstanceStores, LocalInteraction, PeerRoster};
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_instance_jobs::jobs_components::{PendingReserved, ToolOperations};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_instance_render::render_bundle::RenderCtx;
use semio_framework_plugin_time_travel::TimeTravelLedger;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolMachineRuntime;
use semio_framework_plugin_tool_run::ToolRunLedger;
use semio_framework_ui_contract::*;
use std::marker::PhantomData;

/// 🧺️ The exclusive borrows the `tr-turns` use cases need; built by the composer (`VcsArtifactApp::tool_run_ctx`) or reborrowed by a higher bundle.
pub struct ToolRunCtx<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> {
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

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> ToolRunCtx<'ctx, A, M, X> {
    /// 🔍️ The shared twin over the same borrows, for the `&self` members.
    pub fn shared(&self) -> ToolRunRef<'_, A, M, X> {
        ToolRunRef { stores: &*self.stores, child_composition: &*self.child_composition, tool_runs: &*self.tool_runs, extension: PhantomData }
    }

    /// ⬇️ Reborrows the `render` bundle this bundle bridges to.
    pub fn as_render(&mut self) -> RenderCtx<'_, A, M, X> {
        RenderCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }

    /// 🔌️ The bundle of this extension, reborrowed from the extension seam's bundle.
    pub fn of_extension<'e>(ext: &'e mut ExtensionCtx<'_, A, M, X>) -> ToolRunCtx<'e, A, M, X> {
        ToolRunCtx { app: &mut *ext.app, mounted_policy: ext.mounted_policy, stores: &mut *ext.stores, roster: &mut *ext.roster, interaction: &mut *ext.interaction, history: &mut *ext.history, child_composition: &mut *ext.child_composition, pending: &mut *ext.pending, tools: &mut *ext.tools, ingest: &mut *ext.ingest, tool_runs: &mut *ext.tool_runs, time_travel: &mut *ext.time_travel, tool_machines: &mut *ext.tool_machines, extension: PhantomData }
    }
}

/// 🔍️ The shared borrows the `tr-turns` `&self` use cases need (`Copy`: every field is a shared reference).
pub struct ToolRunRef<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> {
    pub stores: &'v InstanceStores<A>,
    pub child_composition: &'v ChildComposition<M>,
    pub tool_runs: &'v ToolRunLedger<A>,
    pub extension: PhantomData<fn() -> X>,
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> Clone for ToolRunRef<'v, A, M, X> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> Copy for ToolRunRef<'v, A, M, X> {}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> ToolRunRef<'v, A, M, X> {
    /// 🔌️ The shared bundle of this extension, shared from the extension seam's view.
    pub fn of_extension_ref<'e>(ext: &'e ExtensionRef<'_, A, M, X>) -> ToolRunRef<'e, A, M, X> {
        ToolRunRef { stores: ext.stores, child_composition: ext.child_composition, tool_runs: ext.tool_runs, extension: PhantomData }
    }
}

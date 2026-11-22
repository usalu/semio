//! 🪢️ Borrowed bundle of the `instance-members` use cases: `MembersCtx` (WP7 of the instance family).
//!
//! 🧺️ The fields carry the names of the composer fields they borrow, so a method body written for the composer reads the same; `X` is the registered `InstanceExtension` list.
use semio_plugin_instance_stores::stores_components::{ActionIngress, InstanceStores, LocalInteraction, PeerRoster};
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_instance_children::children_components::ChildComposition;
use semio_plugin_instance_ladders::ladders_components::{CloseCursors, MaintenanceCursors};
use semio_plugin_instance_ladders::ladder_bundle::LadderRef;
use semio_plugin_instance_history::history_components::{CommandHistory, HostEvents, Transactions};
use semio_plugin_instance_docs::docs_components::{EnvelopeJobs, MediaTransfers};
use semio_plugin_instance_extension::ExtensionCtx;
use semio_plugin_instance_jobs::jobs_components::{PendingReserved, ToolOperations};
use semio_plugin_instance_render::render_bundle::{RenderCtx, RenderRef};
use semio_plugin_instance_dispatch::dispatch_bundle::DispatchCtx;
use semio_plugin_instance_commands::command_bundle::CommandCtx;
use semio_plugin_time_travel_turns::time_travel_bundle::TimeTravelRef;
use semio_plugin_time_travel::TimeTravelLedger;
use semio_plugin_tool_machine::ToolMachineRuntime;
use semio_plugin_tool_run::ToolRunLedger;
use store::{MemberFactory, ReferenceSpaceMember};
use std::marker::PhantomData;

/// 🧺️ The exclusive borrows the `members` use cases need; built by the composer (`VcsArtifactApp::members_ctx`).
pub struct MembersCtx<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X> {
    pub app: &'ctx mut A,
    pub mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1,
    pub stores: &'ctx mut InstanceStores<A>,
    pub roster: &'ctx mut PeerRoster<A>,
    pub ingress: &'ctx mut ActionIngress,
    pub original_typing: &'ctx mut semio_plugin_instance_typed_ops::original_typing::OriginalTypingDispatch<A>,
    pub original_emission: &'ctx mut semio_plugin_job::original_emission::OriginalEmissionSource<A>,
    pub original_typed_command: &'ctx mut semio_plugin_command::command_admission::original_typed_command::OriginalTypedCommandSource<A>,
    pub original_typed_tag: &'ctx mut std::mem::ManuallyDrop<Option<semio_plugin_tool_machine::ToolTag>>,
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

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X> MembersCtx<'ctx, A, M, X> {
    /// ⬇️ Reborrows the `render` bundle this bundle bridges to.
    pub fn as_render(&mut self) -> RenderCtx<'_, A, M, X> {
        RenderCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }

    /// ⬇️ Reborrows the `dispatch` bundle this bundle bridges to.
    pub fn as_dispatch(&mut self) -> DispatchCtx<'_, A, M, X> {
        DispatchCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, host_events: &mut *self.host_events, txn: &mut *self.txn, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, media: &mut *self.media, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }

    /// ⬇️ Reborrows the `command` bundle this bundle bridges to.
    pub fn as_command(&mut self) -> CommandCtx<'_, A, M, X> {
        CommandCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, ingress: &mut *self.ingress, original_typing: &mut *self.original_typing, original_emission: &mut *self.original_emission, original_typed_command: &mut *self.original_typed_command, original_typed_tag: &mut *self.original_typed_tag, interaction: &mut *self.interaction, history: &mut *self.history, host_events: &mut *self.host_events, txn: &mut *self.txn, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, media: &mut *self.media, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }

    /// 🔍️ The shared `render` twin over the same borrows.
    pub fn render_ref(&self) -> RenderRef<'_, A, M, X> {
        RenderRef { app: &*self.app, stores: &*self.stores, history: &*self.history, child_composition: &*self.child_composition, tools: &*self.tools, ingest: &*self.ingest, time_travel: &*self.time_travel, tool_machines: &*self.tool_machines, extension: PhantomData }
    }

    /// 🔍️ The shared `time_travel` twin over the same borrows.
    pub fn time_travel_ref(&self) -> TimeTravelRef<'_, A, M, X> {
        TimeTravelRef { stores: &*self.stores, txn: &*self.txn, child_composition: &*self.child_composition, tool_runs: &*self.tool_runs, time_travel: &*self.time_travel, extension: PhantomData }
    }

    /// 🔍️ The shared `ladder` twin over the same borrows.
    pub fn ladder_ref(&self) -> LadderRef<'_, A, M, X> {
        LadderRef { app: &*self.app, stores: &*self.stores, roster: &*self.roster, ingress: &*self.ingress, interaction: &*self.interaction, history: &*self.history, txn: &*self.txn, child_composition: &*self.child_composition, pending: &*self.pending, tools: &*self.tools, media: &*self.media, ingest: &*self.ingest, maintenance: &*self.maintenance, close: &*self.close, tool_runs: &*self.tool_runs, time_travel: &*self.time_travel, tool_machines: &*self.tool_machines, extension: PhantomData }
    }

    /// 🧩️ The extension borrows the registered feature hooks run over.
    pub fn extension_ctx(&mut self) -> ExtensionCtx<'_, A, M, X> {
        ExtensionCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, host_events: &mut *self.host_events, txn: &mut *self.txn, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, media: &mut *self.media, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }
}

//! 🐀️ Borrowed bundle of the `instance-commands` use cases: `CommandCtx` (WP7 of the instance family).
//!
//! 🧺️ The fields carry the names of the composer fields they borrow, so a method body written for the composer reads the same; `X` is the registered `InstanceExtension` list.
use super::*;
use std::marker::PhantomData;

/// 🧺️ The exclusive borrows the `commands` use cases need; built by the composer (`VcsArtifactApp::command_ctx`) or reborrowed by a higher bundle.
pub struct CommandCtx<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> {
    pub app: &'ctx mut A,
    pub mounted_policy: crate::MountedOwnerPolicyV1,
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
    pub tool_runs: &'ctx mut ToolRunLedger<A>,
    pub time_travel: &'ctx mut TimeTravelLedger<A>,
    pub tool_machines: &'ctx mut ToolMachineRuntime<A::Snapshot, A::Mutation, A::Config, A::ConfigMutation>,
    pub extension: PhantomData<fn() -> X>,
}

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> CommandCtx<'ctx, A, M, X> {
    /// ⬇️ Reborrows the `dispatch` bundle this bundle bridges to.
    pub fn as_dispatch(&mut self) -> DispatchCtx<'_, A, M, X> {
        DispatchCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, host_events: &mut *self.host_events, txn: &mut *self.txn, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, media: &mut *self.media, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }

    /// ⬇️ Reborrows the `extension` bundle this bundle bridges to.
    pub fn as_extension(&mut self) -> ExtensionCtx<'_, A, M, X> {
        ExtensionCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, host_events: &mut *self.host_events, txn: &mut *self.txn, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, media: &mut *self.media, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }

    /// ⬇️ Reborrows the `render` bundle this bundle bridges to.
    pub fn as_render(&mut self) -> RenderCtx<'_, A, M, X> {
        RenderCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }

    /// ⬇️ Reborrows the `time_travel` bundle this bundle bridges to.
    pub fn as_time_travel(&mut self) -> TimeTravelCtx<'_, A, M, X> {
        TimeTravelCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, host_events: &mut *self.host_events, txn: &mut *self.txn, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, media: &mut *self.media, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }

    /// ⬇️ Reborrows the `typed` bundle this bundle bridges to.
    pub fn as_typed(&mut self) -> TypedCtx<'_, A, M, X> {
        TypedCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, host_events: &mut *self.host_events, txn: &mut *self.txn, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, media: &mut *self.media, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: PhantomData }
    }
}

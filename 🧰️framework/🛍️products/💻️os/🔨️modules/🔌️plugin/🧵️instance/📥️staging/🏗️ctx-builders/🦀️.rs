//! 🐂️ The composer's bundle builders: one exclusive builder per use-case crate and one shared builder per crate that has `&self` members (WP7 of the instance family).
//!
//! 🏗️ Each builder borrows the composer fields its bundle names; the extension list is fixed here to [`InstanceExtensions`].
use semio_framework_plugin_decl_emission::{ActionMeta, Emit};
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_instance_commands::command_bundle::CommandCtx;
use semio_framework_plugin_instance_dispatch::dispatch_bundle::DispatchCtx;
use semio_framework_value::DslValue;
use semio_framework::kernel::{Effect, InvocationResult};
use semio_framework_plugin_instance_extension::{ExtensionCtx, ExtensionRef, InstanceExtension};
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use crate::{InstanceExtensions, VcsArtifactApp};
use semio_framework_plugin_instance_ladders::ladder_bundle::{LadderCtx, LadderRef};
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_instance_render::render_bundle::{RenderCtx, RenderRef};
use semio_framework_plugin_time_travel_turns::time_travel_bundle::{TimeTravelCtx, TimeTravelRef};
use semio_framework_plugin_time_travel::TimeTravelLedger;
use semio_framework_plugin_tool_run_turns::tool_run_bundle::{ToolRunCtx, ToolRunRef};
use semio_framework_plugin_instance_typed_ops::typed_bundle::TypedCtx;
use semio_framework_ui_contract::*;
use std::marker::PhantomData;
use semio_framework_plugin_time_travel::TimeTravelActionOutcome;
use semio_framework_plugin_tool_run::ToolRunActionOutcome;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    #[doc(hidden)]
    pub fn render_ctx(&mut self) -> RenderCtx<'_, A, M, InstanceExtensions> {
        RenderCtx { app: &mut self.app, mounted_policy: self.mounted_policy, stores: &mut self.stores, roster: &mut self.roster, interaction: &mut self.interaction, history: &mut self.history, child_composition: &mut self.child_composition, pending: &mut self.pending, tools: &mut self.tools, ingest: &mut self.ingest, tool_runs: &mut self.tool_runs, time_travel: &mut self.time_travel, tool_machines: &mut self.tool_machines, extension: PhantomData }
    }

    #[doc(hidden)]
    pub fn render_ref(&self) -> RenderRef<'_, A, M, InstanceExtensions> {
        RenderRef { app: &self.app, stores: &self.stores, history: &self.history, child_composition: &self.child_composition, tools: &self.tools, ingest: &self.ingest, time_travel: &self.time_travel, tool_machines: &self.tool_machines, extension: PhantomData }
    }

    #[doc(hidden)]
    pub fn dispatch_ctx(&mut self) -> DispatchCtx<'_, A, M, InstanceExtensions> {
        DispatchCtx { app: &mut self.app, mounted_policy: self.mounted_policy, stores: &mut self.stores, roster: &mut self.roster, interaction: &mut self.interaction, history: &mut self.history, host_events: &mut self.host_events, txn: &mut self.txn, child_composition: &mut self.child_composition, pending: &mut self.pending, tools: &mut self.tools, media: &mut self.media, ingest: &mut self.ingest, tool_runs: &mut self.tool_runs, time_travel: &mut self.time_travel, tool_machines: &mut self.tool_machines, extension: PhantomData }
    }

    #[doc(hidden)]
    pub fn typed_ctx(&mut self) -> TypedCtx<'_, A, M, InstanceExtensions> {
        TypedCtx { app: &mut self.app, mounted_policy: self.mounted_policy, stores: &mut self.stores, roster: &mut self.roster, interaction: &mut self.interaction, history: &mut self.history, host_events: &mut self.host_events, txn: &mut self.txn, child_composition: &mut self.child_composition, pending: &mut self.pending, tools: &mut self.tools, media: &mut self.media, ingest: &mut self.ingest, tool_runs: &mut self.tool_runs, time_travel: &mut self.time_travel, tool_machines: &mut self.tool_machines, extension: PhantomData }
    }

    #[doc(hidden)]
    pub fn time_travel_ctx(&mut self) -> TimeTravelCtx<'_, A, M, InstanceExtensions> {
        TimeTravelCtx { app: &mut self.app, mounted_policy: self.mounted_policy, stores: &mut self.stores, roster: &mut self.roster, interaction: &mut self.interaction, history: &mut self.history, host_events: &mut self.host_events, txn: &mut self.txn, child_composition: &mut self.child_composition, pending: &mut self.pending, tools: &mut self.tools, media: &mut self.media, ingest: &mut self.ingest, tool_runs: &mut self.tool_runs, time_travel: &mut self.time_travel, tool_machines: &mut self.tool_machines, extension: PhantomData }
    }

    #[doc(hidden)]
    pub fn time_travel_ref(&self) -> TimeTravelRef<'_, A, M, InstanceExtensions> {
        TimeTravelRef { stores: &self.stores, txn: &self.txn, child_composition: &self.child_composition, tool_runs: &self.tool_runs, time_travel: &self.time_travel, extension: PhantomData }
    }

    #[doc(hidden)]
    pub fn tool_run_ctx(&mut self) -> ToolRunCtx<'_, A, M, InstanceExtensions> {
        ToolRunCtx { app: &mut self.app, mounted_policy: self.mounted_policy, stores: &mut self.stores, roster: &mut self.roster, interaction: &mut self.interaction, history: &mut self.history, child_composition: &mut self.child_composition, pending: &mut self.pending, tools: &mut self.tools, ingest: &mut self.ingest, tool_runs: &mut self.tool_runs, time_travel: &mut self.time_travel, tool_machines: &mut self.tool_machines, extension: PhantomData }
    }

    #[doc(hidden)]
    pub fn tool_run_ref(&self) -> ToolRunRef<'_, A, M, InstanceExtensions> {
        ToolRunRef { stores: &self.stores, child_composition: &self.child_composition, tool_runs: &self.tool_runs, extension: PhantomData }
    }

    #[doc(hidden)]
    pub fn command_ctx(&mut self) -> CommandCtx<'_, A, M, InstanceExtensions> {
        CommandCtx { app: &mut self.app, mounted_policy: self.mounted_policy, stores: &mut self.stores, roster: &mut self.roster, ingress: &mut self.ingress, interaction: &mut self.interaction, history: &mut self.history, host_events: &mut self.host_events, txn: &mut self.txn, child_composition: &mut self.child_composition, pending: &mut self.pending, tools: &mut self.tools, media: &mut self.media, ingest: &mut self.ingest, tool_runs: &mut self.tool_runs, time_travel: &mut self.time_travel, tool_machines: &mut self.tool_machines, extension: PhantomData }
    }

    #[doc(hidden)]
    pub fn ladder_ctx(&mut self) -> LadderCtx<'_, A, M, InstanceExtensions> {
        LadderCtx { app: &mut self.app, mounted_policy: self.mounted_policy, stores: &mut self.stores, roster: &mut self.roster, ingress: &mut self.ingress, interaction: &mut self.interaction, history: &mut self.history, host_events: &mut self.host_events, txn: &mut self.txn, child_composition: &mut self.child_composition, pending: &mut self.pending, tools: &mut self.tools, media: &mut self.media, ingest: &mut self.ingest, maintenance: &mut self.maintenance, close: &mut self.close, tool_runs: &mut self.tool_runs, time_travel: &mut self.time_travel, tool_machines: &mut self.tool_machines, extension: PhantomData }
    }

    #[doc(hidden)]
    pub fn ladder_ref(&self) -> LadderRef<'_, A, M, InstanceExtensions> {
        LadderRef { app: &self.app, stores: &self.stores, roster: &self.roster, ingress: &self.ingress, interaction: &self.interaction, history: &self.history, txn: &self.txn, child_composition: &self.child_composition, pending: &self.pending, tools: &self.tools, media: &self.media, ingest: &self.ingest, maintenance: &self.maintenance, close: &self.close, tool_runs: &self.tool_runs, time_travel: &self.time_travel, tool_machines: &self.tool_machines, extension: PhantomData }
    }

    pub(crate) fn extension_ctx(&mut self) -> ExtensionCtx<'_, A, M, InstanceExtensions> {
        ExtensionCtx { app: &mut self.app, mounted_policy: self.mounted_policy, stores: &mut self.stores, roster: &mut self.roster, interaction: &mut self.interaction, history: &mut self.history, host_events: &mut self.host_events, txn: &mut self.txn, child_composition: &mut self.child_composition, pending: &mut self.pending, tools: &mut self.tools, media: &mut self.media, ingest: &mut self.ingest, tool_runs: &mut self.tool_runs, time_travel: &mut self.time_travel, tool_machines: &mut self.tool_machines, extension: PhantomData }
    }

    pub(crate) fn extension_ref(&self) -> ExtensionRef<'_, A, M, InstanceExtensions> {
        ExtensionRef { stores: &self.stores, txn: &self.txn, child_composition: &self.child_composition, tool_runs: &self.tool_runs, time_travel: &self.time_travel, extension: PhantomData }
    }
}

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🎯️ B1: the shared body behind both `dispatch_typed_command` (wire bytes, decoded above)
    /// 🧪️ Test-only typed-value helper. Production callers must enter with an exact
    /// owner-qualified retained raw-page admission and cannot infer identity from a bare command.
    pub fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -> impl Future<Output = Result<InvocationResult, Fault>> + 'a + use<'a, 'b, 'context, A, M> {
        self.dispatch_ctx().dispatch_typed(command, meta, identity, cx)
    }

    /// 🧪️ Pins the continuous tools' clock (`None`: the wall clock again), so a law types, pauses and lapses deterministically
    /// however slowly its machine runs.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn set_tool_clock_ms(&mut self, now_ms: Option<u64>) {
        self.render_ctx().set_tool_clock_ms(now_ms)
    }

    /// 🧪️ The document every render seam reads now: the committed head with every open press's and typing run's provisional
    /// leaves.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn rendered_snapshot(&self) -> Arc<A::Snapshot> {
        self.render_ref().rendered_snapshot()
    }

    /// 🧪️ The app config every render seam reads now: the committed config with every held press's config mutations.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn rendered_config(&self) -> Arc<A::Config> {
        self.render_ref().rendered_config()
    }

    /// 🕰️ The fixed maintenance stage the NEXT [`PluginApp::maintenance_step`] call will run.
    /// A step-budget law reads this before each unit so an over-budget unit names its own stage
    /// instead of the stale round-robin cursor an idle early return leaves behind.
    pub fn next_maintenance_stage(&self) -> u8 {
        self.ladder_ref().next_maintenance_stage()
    }

    /// 🔍️ Read-only inspection of this instance's history-edit ledger for app-level tests.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn time_travel_ledger(&self) -> &TimeTravelLedger<A> {
        &self.time_travel
    }

    /// ⚖️ Applies one history-edit verb: delivers a pending base change first, then the verb's event with its effects.
    pub async fn apply_time_travel_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, effects: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<TimeTravelActionOutcome, Fault> {
        self.time_travel_ctx().apply_time_travel_action(action, args, meta, effects, identity, cx).await
    }

    /// ⚖️ Applies one §2.5 action to the ledger and performs the transition's immediate effect. An action that
    /// names no identity at all (neither `runId` nor `generation`, as a keyboard chord dispatches it) targets
    /// the instance's live run at its generation of this dispatch; an identity it does name is checked as is.
    pub async fn apply_tool_run_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta) -> Result<ToolRunActionOutcome, Fault> {
        self.tool_run_ctx().apply_tool_run_action(action, args, meta).await
    }

}

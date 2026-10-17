//! 🧩️ The neutral extension point of the mounted artifact instance. A feature module (history editing, tool runs, …)
//! contributes command routes, document refusals, maintenance turns and framework bodies through [`InstanceExtension`];
//! the instance consults the registered list `X` its bundles carry and never names the feature itself.
//!
//! 🧩️ Hooks come in three kinds. The static ones read only the action id. The ledger hooks read the two feature ledgers
//! (`time_travel`, `tool_runs`) the instance owns and are passed explicitly. The context hooks run over [`ExtensionCtx`]
//! or [`ExtensionRef`], the borrows the two implementations need, and build their own use-case bundle from it.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
use semio_framework_plugin_decl_emission::{ActionMeta, Emit};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_instance_children::children_components::ChildComposition;
use semio_framework_plugin_instance_history::history_components::{CommandHistory, HostEvents, Transactions};
use semio_framework_value::DslValue;
use semio_framework_plugin_instance_docs::docs_components::{EnvelopeJobs, MediaTransfers};
use semio_framework::{Fault, FaultFrom, ViewModel};
use std::future::Future;
use semio_framework_plugin_instance_stores::stores_components::{InstanceStores, LocalInteraction, PeerRoster};
use semio_framework::kernel::InvocationResult;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_instance_jobs::jobs_components::{PendingReserved, ToolOperations};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_plugin_time_travel::TimeTravelLedger;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolMachineRuntime;
use semio_framework_plugin_tool_run::ToolRunLedger;
use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;
use std::marker::PhantomData;
use semio_framework_ui_contract::BuiltNode;
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;

/// 🔌️ The exclusive borrows an extension's context hooks (`dispatch_action`, `drive_turn`, `render_body`, `after_app_render`) run over; the union of what the time-travel and tool-run bundles borrow.
pub struct ExtensionCtx<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> {
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

/// 🔍️ The shared borrows an extension's `has_pending_work` reads.
pub struct ExtensionRef<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> {
    pub stores: &'v InstanceStores<A>,
    pub txn: &'v Transactions<A>,
    pub child_composition: &'v ChildComposition<M>,
    pub tool_runs: &'v ToolRunLedger<A>,
    pub time_travel: &'v TimeTravelLedger<A>,
    pub extension: PhantomData<fn() -> X>,
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> Clone for ExtensionRef<'v, A, M, X> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X> Copy for ExtensionRef<'v, A, M, X> {}

/// 🧱️ One feature's contribution to every mounted instance. Every hook defaults to "no contribution", so an extension
/// overrides only what it owns. A registered list is a right-nested tuple `(First, (Second, ()))`: earlier entries are
/// consulted first, the first claimant of a route, refusal or body wins, and every entry's maintenance turn runs in order.
#[doc(hidden)]
pub trait InstanceExtension {
    /// 🧭️ Whether this extension owns the dispatch of `action`.
    fn claims_action(_action: &str) -> bool {
        false
    }

    /// 📜️ Whether `action` needs the document's history, so a head-only pure evaluation refuses it.
    fn reads_history(_action: &str) -> bool {
        false
    }

    /// ⏳️ Whether `action` stays admitted while the document is still loading.
    fn admitted_while_loading(_action: &str) -> bool {
        false
    }

    /// 🚦️ Dispatches one action this extension claims.
    async fn dispatch_action<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(_ctx: &mut ExtensionCtx<'_, A, M, X>, action: &str, _args: Option<&DslValue>, _meta: &ActionMeta, _identity: &mut EntityIdentityAuthority<'_>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        Err(plugin_sdk_fault(format!("no instance extension claims action '{action}'")))
    }

    /// 🔒️ The refusal of a framework-reserved `action` this extension currently freezes.
    fn reserved_action_refusal<A: ArtifactApp>(_time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>, _action: &str) -> Option<Fault> {
        None
    }

    /// 🛑️ The refusal of a `verb` that would emit artifact mutations while this extension holds them.
    fn mutation_refusal<A: ArtifactApp>(_time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>, _verb: &str) -> Option<Fault> {
        None
    }

    /// ❄️ Whether this extension currently freezes every local change to the document.
    fn freezes_document<A: ArtifactApp>(_time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>) -> bool {
        false
    }

    /// 🚫️ The refusal of a `verb` that would change the document (artifact mutations or child emits) while it is frozen.
    fn document_refusal<A: ArtifactApp>(_time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>, _verb: &str) -> Option<Fault> {
        None
    }

    /// 🔀️ The refusal of a foreign transaction `txn_id` that would change the document while it is frozen.
    fn transaction_refusal<A: ArtifactApp>(_time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>, _txn_id: &str) -> Option<Fault> {
        None
    }

    /// 🎚️ Observes one committed window-config publication.
    fn window_config_published<A: ArtifactApp>(_time_travel: &mut TimeTravelLedger<A>, _tool_runs: &mut ToolRunLedger<A>) {}

    /// 🏃️ Whether this extension owes maintenance work.
    fn has_pending_work<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(_ctx: &ExtensionRef<'_, A, M, X>) -> bool {
        false
    }

    /// 🧾️ Whether this extension holds an original receipt its recipient has not taken yet.
    fn holds_receipt<A: ArtifactApp>(_time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>) -> bool {
        false
    }

    /// 🪦️ Whether this extension retains nothing a close still has to retire.
    fn terminal_is_empty<A: ArtifactApp>(_time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>) -> bool {
        true
    }

    /// 🔁️ Drives one maintenance turn under the caller's current retained grant; `true` ends the instance's maintenance
    /// turn here.
    async fn drive_turn<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(_ctx: &mut ExtensionCtx<'_, A, M, X>, _retained: &mut RetainedCloneGrant, _identity: &mut EntityIdentityAuthority<'_>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        Ok(false)
    }

    /// 🖼️ The framework body this extension renders for `body_key`.
    fn render_body<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(_ctx: &mut ExtensionCtx<'_, A, M, X>, _body_key: &str, _view_state: &ViewModel) -> Option<UiAssemblyResult<BuiltNode>> {
        None
    }

    /// 🖌️ Decorates the app's own rendered body for `body_key` after it is retained.
    fn after_app_render<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(_ctx: &mut ExtensionCtx<'_, A, M, X>, _body_key: &str, _view_state: &ViewModel) -> Result<(), Fault> {
        Ok(())
    }
}

/// 🫙️ The empty list: no route, no refusal, no work.
impl InstanceExtension for () {}

/// 🔗️ A registered list: `First` is consulted before every entry of `Rest`.
impl<First: InstanceExtension, Rest: InstanceExtension> InstanceExtension for (First, Rest) {
    fn claims_action(action: &str) -> bool {
        First::claims_action(action) || Rest::claims_action(action)
    }

    fn reads_history(action: &str) -> bool {
        First::reads_history(action) || Rest::reads_history(action)
    }

    fn admitted_while_loading(action: &str) -> bool {
        First::admitted_while_loading(action) || Rest::admitted_while_loading(action)
    }

    async fn dispatch_action<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, action: &str, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if First::claims_action(action) {
            First::dispatch_action(ctx, action, args, meta, identity, cx).await
        } else {
            Rest::dispatch_action(ctx, action, args, meta, identity, cx).await
        }
    }

    fn reserved_action_refusal<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>, action: &str) -> Option<Fault> {
        First::reserved_action_refusal(time_travel, tool_runs, action).or_else(|| Rest::reserved_action_refusal(time_travel, tool_runs, action))
    }

    fn mutation_refusal<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>, verb: &str) -> Option<Fault> {
        First::mutation_refusal(time_travel, tool_runs, verb).or_else(|| Rest::mutation_refusal(time_travel, tool_runs, verb))
    }

    fn freezes_document<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>) -> bool {
        First::freezes_document(time_travel, tool_runs) || Rest::freezes_document(time_travel, tool_runs)
    }

    fn document_refusal<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>, verb: &str) -> Option<Fault> {
        First::document_refusal(time_travel, tool_runs, verb).or_else(|| Rest::document_refusal(time_travel, tool_runs, verb))
    }

    fn transaction_refusal<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>, txn_id: &str) -> Option<Fault> {
        First::transaction_refusal(time_travel, tool_runs, txn_id).or_else(|| Rest::transaction_refusal(time_travel, tool_runs, txn_id))
    }

    fn window_config_published<A: ArtifactApp>(time_travel: &mut TimeTravelLedger<A>, tool_runs: &mut ToolRunLedger<A>) {
        First::window_config_published(&mut *time_travel, &mut *tool_runs);
        Rest::window_config_published(time_travel, tool_runs);
    }

    fn has_pending_work<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &ExtensionRef<'_, A, M, X>) -> bool {
        First::has_pending_work(ctx) || Rest::has_pending_work(ctx)
    }

    fn holds_receipt<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>) -> bool {
        First::holds_receipt(time_travel, tool_runs) || Rest::holds_receipt(time_travel, tool_runs)
    }

    fn terminal_is_empty<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>) -> bool {
        First::terminal_is_empty(time_travel, tool_runs) && Rest::terminal_is_empty(time_travel, tool_runs)
    }

    async fn drive_turn<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, retained: &mut RetainedCloneGrant, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        *retained = cx.retained_grant();
        if First::drive_turn(ctx, retained, identity, cx).await? {
            return Ok(true);
        }
        Rest::drive_turn(ctx, retained, identity, cx).await
    }

    fn render_body<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, body_key: &str, view_state: &ViewModel) -> Option<UiAssemblyResult<BuiltNode>> {
        match First::render_body(ctx, body_key, view_state) {
            Some(body) => Some(body),
            None => Rest::render_body(ctx, body_key, view_state),
        }
    }

    fn after_app_render<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, body_key: &str, view_state: &ViewModel) -> Result<(), Fault> {
        First::after_app_render(ctx, body_key, view_state)?;
        Rest::after_app_render(ctx, body_key, view_state)
    }
}

#[doc(hidden)]
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub mod tests;

//! 🏉️ Staged `typing` members of the `instance-typed-ops` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! ⌨️ Tool dispatch and typing commits: continuous-control arguments, typing runs and their publication.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_value::DslValue;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::Fault;
use store::Mutation;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ScrubTag;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolDispatch;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolTag;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::TypingTag;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework::FaultFrom;
use semio_framework_value_derive::FromValue;
use store::MutationDiff;
use protocol::OpBinary;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value_derive::ToValue;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework::{ActionKind, FaultCode, FaultOrigin};
use semio_framework_plugin_decl_emission::ActionMeta;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use store::{MemberFactory, ReferenceSpaceMember};
use semio_framework_plugin_instance_jobs::jobs_components::ToolOperations;
use crate::typed_bundle::TypedCtx;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin_instance_dispatch::dispatch_emit;
use semio_framework_ui_contract::*;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::*;
use semio_framework_tool_machine::ScrubPhase;
use semio_framework_tool_machine::ToolAbortReason;
use semio_framework_tool_machine::ToolStep;
use semio_framework_tool_machine::TypingCommit;
use semio_framework_tool_machine::TypingPhase;
use semio_framework_tool_machine::SCRUB_ABORT_ARG;
use semio_framework_tool_machine::SCRUB_COMMIT_ARG;
use semio_framework_tool_machine::SCRUB_GESTURE_ARG;
use semio_framework_tool_machine::TYPING_BUFFER_ARG;
use semio_framework_tool_machine::TYPING_COMMIT_ARG;

//#region 🔖️Driver
impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    /// 🛠️ Reads a dispatch's continuous-tool arguments. A typing delivery (`typing`) folds into its window's run: a commit
    /// signal (`typingCommit`) publishes the run as ONE edit and never reaches the app, an edit while a history edit freezes
    /// the document drops the run and is refused `timeTravel.frozen`, every other edit runs the verb with its operation
    /// tagged. A continuous control (`gesture`, `commit`, `abort`): a host abort drops the window's press with zero trace and
    /// never reaches the app, a frozen press is dropped and refused, every other press runs the verb tagged. Windows that left
    /// the roster retire their presses and commit their runs first.
    pub async fn admit_tool_dispatch(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> ToolDispatch {
        let text = |key: &str| args.and_then(|args| args.get(key)).and_then(DslValue::as_str);
        let window = meta.view_state.as_ref().and_then(|view| view.window_id.clone()).unwrap_or_default();
        if let Some(phase) = TypingPhase::parse(text(TYPING_BUFFER_ARG), text(TYPING_COMMIT_ARG)) {
            if let Err(fault) = Box::pin(self.retire_tool_windows(meta, identity, cx)).await {
                return ToolDispatch::Settled(Err(fault));
            }
            let clock = self.tool_machines.clock();
            let frozen = X::document_refusal(&*self.time_travel, &*self.tool_runs, action);
            let (step, fault) = match phase {
                TypingPhase::Commit { reason, .. } => (self.tool_machines.typing.commit(&window, reason, clock), None),
                TypingPhase::Edit { .. } if frozen.is_some() => (Ok(self.tool_machines.typing.abort(&window, ToolAbortReason::Frozen)), frozen),
                TypingPhase::Edit { buffer } => {
                    let tool = format!("{}#{action}", self.app.instance_id().await);
                    return ToolDispatch::Tagged(ToolTag::Typing(TypingTag { window, tool, buffer }));
                }
            };
            let published = match step {
                Ok(step) => Box::pin(self.publish_typing_commits(vec![step], meta, identity, cx)).await,
                Err(refusal) => Err(Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("typing run of window {window:?} refused its commit"))),
            };
            return ToolDispatch::Settled(match (published, fault) {
                (Err(fault), _) | (Ok(()), Some(fault)) => Err(fault),
                (Ok(()), None) => Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await),
            });
        }
        let Some(phase) = ScrubPhase::parse(text(SCRUB_GESTURE_ARG), args.and_then(|args| args.get(SCRUB_COMMIT_ARG)).and_then(DslValue::as_bool), text(SCRUB_ABORT_ARG)) else {
            return ToolDispatch::Plain;
        };
        if let Err(fault) = Box::pin(self.retire_tool_windows(meta, identity, cx)).await {
            return ToolDispatch::Settled(Err(fault));
        }
        let frozen = X::document_refusal(&*self.time_travel, &*self.tool_runs, action);
        let (reason, fault) = match &phase {
            ScrubPhase::Abort { reason, .. } => (*reason, None),
            _ if frozen.is_some() => (ToolAbortReason::Frozen, frozen),
            _ => {
                let tool = format!("{}#{action}", self.app.instance_id().await);
                return ToolDispatch::Tagged(ToolTag::Scrub(ScrubTag { window, tool, phase }));
            }
        };
        self.tool_machines.presses.abort(&window, Some(phase.gesture()), reason);
        self.as_render().follow_tool_machines(true);
        ToolDispatch::Settled(match fault {
            Some(fault) => Err(fault),
            None => Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await),
        })
    }

    /// ⌨️ Before any dispatch reaches its verb: every typing run idle past its deadline commits, and every run commits before
    /// another verb (`otherVerb`) — so the verb, an undo or a history edit sees the typed text, and the history lists the run
    /// before the verb. A typing delivery of its own and a view verb (a caret report, a hover, a scroll) leave the runs open:
    /// a host ends its run on a caret jump with its own commit signal.
    pub async fn commit_typing_before(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        if self.tool_machines.typing.is_empty() {
            return Ok(());
        }
        let clock = self.tool_machines.clock();
        let mut steps = self.tool_machines.typing.lapse(clock);
        let typing = args.and_then(|args| args.get(TYPING_BUFFER_ARG)).and_then(DslValue::as_str).is_some_and(|buffer| !buffer.is_empty());
        let view = self.tools.registry.get(action).is_some_and(|definition| definition.kind == ActionKind::View);
        if !typing && !view {
            steps.extend(self.tool_machines.typing.commit_all(TypingCommit::OtherVerb, clock));
        }
        let steps = steps.into_iter().map(|(window, step)| step.map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("typing run of window {window:?} refused its commit")))).collect::<Result<Vec<_>, _>>()?;
        Box::pin(self.publish_typing_commits(steps, meta, identity, cx)).await
    }

    /// 💾️ Publishes every committed run as ONE document edit stamped with its `TransactionRef` (no
    /// description: the history row is labelled from the run's net leaves), through the verb that typed it.
    async fn publish_typing_commits(&mut self, steps: Vec<ToolStep<A::Mutation>>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let mut published = false;
        for step in steps {
            if let ToolStep::Committed(transaction, mutations) = step {
                let verb = transaction.tool.rsplit_once('#').map_or(transaction.tool.as_str(), |(_, verb)| verb).to_string();
                Box::pin(self.as_dispatch().dispatch_emit(&verb, Emit::commit_transaction(transaction, mutations), meta, identity,cx)).await?;
                published = true;
            }
        }
        if published {
            self.as_render().follow_tool_machines(true);
        }
        Ok(())
    }

    /// 🪦️ `retired`: the press of every window the dispatching view's roster no longer lists leaves zero trace; its typing
    /// run commits like a blur.
    async fn retire_tool_windows(&mut self, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let Some(view) = meta.view_state.as_ref().filter(|view| !view.window_instances.is_empty()) else { return Ok(()) };
        let keep = |window: &str| window.is_empty() || view.window_instances.iter().any(|instance| instance.id == window);
        let retired = !self.tool_machines.presses.retain_windows(keep).is_empty();
        let clock = self.tool_machines.clock();
        let committed = self.tool_machines.typing.retain_windows(keep, clock).into_iter().filter_map(|(_, step)| step.ok()).collect::<Vec<_>>();
        if retired {
            self.as_render().follow_tool_machines(true);
        }
        Box::pin(self.publish_typing_commits(committed, meta, identity, cx)).await
    }
}
//#endregion 🔖️Driver

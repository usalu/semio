//! 🏏️ Staged `actions` members of the `instance-commands` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 🚦️ Action and command dispatch, the addressed action preview and the agent lane preview.
use semio_framework_plugin_decl_emission::{ActionMeta, ChildEmit, Emit};
use semio_framework_plugin_command::command_admission::AdmittedToolCommand;
use semio_framework_plugin_job::{AgentLaneCarriage, AgentLanePublication, EmitWire, agent_lane_carriage, agent_lane_no_effect_fault, agent_lane_uncarried_fault, artifact_owned_tool_birth_grant};
use semio_framework::{AppRole, Fault, FaultCode, FaultFrom, FaultOrigin, REVERT_TO_COMMAND_ACTION_ID, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID};
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_framework_plugin_decl_app::{ArtifactApp, ArtifactOwnedContextHandle, viewer_rejects_action};
use semio_framework_plugin_operation::{ArtifactDocumentAuthority, CANCEL_TYPED_OPERATION_ACTION_ID, ToolOperationKey};
use semio_framework_plugin_decl_media::ArtifactOutputChunks;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactOwnedToolJobContext, ArtifactOwnedToolJobRequest, ArtifactOwnedToolJobSnapshots, ArtifactToolCompletion, ArtifactToolRawInput};
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_views::ArtifactView;
use std::collections::BTreeMap;
use semio_framework_plugin_command::{CATALOGUE_EXAMPLE_ACTION_ID, PURE_HISTORY_UNAVAILABLE_CODE};
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use crate::command_bundle::CommandCtx;
use semio_framework_value::DslValue;
use std::future::Future;
use semio_framework_plugin_job::reserved_jobs::{HISTORY_ACTION_IDS, is_framework_reserved_action_id};
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework::kernel::{InvocationResult, UiDirtyScope};
use semio_framework::manifest::{ActionInvocation as ManifestActionInvocation, CommandAddress as ManifestCommandAddress, CommandOwnerAddress as ManifestCommandOwnerAddress};
use std::mem::ManuallyDrop;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolTag;
use semio_framework_plugin_instance_jobs::jobs_components::ToolOperations;
use semio_framework_plugin_instance_dispatch::dispatch_emit;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_window_kits::validate_ui_dispatch_classification;
use semio_framework_ui_contract::*;
use crate::commands_instance_root::framework_reserved_action_kind;
use semio_framework_plugin_instance_docs::docs_instance_root::artifact_handle_of;
use semio_framework_plugin_instance_docs::docs_instance_root::viewer_read_only_fault;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> CommandCtx<'ctx, A, M, X> {
    async fn drive_original_host_configuration(&mut self,action:&str,source:&mut semio_framework_plugin_instance_stores::action_ingress::ActionIngressSource,metadata:&mut Option<ActionMeta>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        let declaration=A::original_host_configuration(action).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"host action has no original producer declaration"))?;
        let args=if source.assembled.is_some(){&mut source.assembled}else if source.prepared.is_some(){&mut source.prepared}else{&mut source.args};
        if !self.original_emission.admit_host(declaration,action,args,self.stores.store.content_revision_now(),cx)?{return Ok(None)}
        if let Some(meta)=metadata.as_ref(){if !self.original_emission.pin_host_metadata(meta,cx)?{return Ok(None)}}
        if source.original_tool_dispatch.is_none(){
            let mut typed: semio_framework_plugin_instance_typed_ops::typed_bundle::TypedCtx<'_,A,M,X> = semio_framework_plugin_instance_typed_ops::typed_bundle::TypedCtx {app:&mut *self.app,mounted_policy:self.mounted_policy,stores:&mut *self.stores,roster:&mut *self.roster,interaction:&mut *self.interaction,history:&mut *self.history,host_events:&mut *self.host_events,txn:&mut *self.txn,child_composition:&mut *self.child_composition,pending:&mut *self.pending,tools:&mut *self.tools,media:&mut *self.media,ingest:&mut *self.ingest,tool_runs:&mut *self.tool_runs,time_travel:&mut *self.time_travel,tool_machines:&mut *self.tool_machines,extension:std::marker::PhantomData};
            if typed.prepare_original_tool_dispatch(action,self.original_emission.host_args(),metadata,&mut *self.original_typing,&mut source.original_typing_control_ready,&mut source.original_tool_tag,&mut source.original_tool_tag_ready,&mut source.original_tool_dispatch,cx).await?.is_none(){return Ok(None)}
        }
        if source.original_tool_dispatch==Some(true){if !self.original_emission.finish_empty_host(action,metadata,cx)?{return Ok(None)}return self.drive_original_emission(cx).await}
        if !self.original_emission.pin_host_base(self.stores.store.content_revision_now(),cx)?{return Ok(None)}
        let meta=metadata.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original host metadata was not restored after typing"))?;
        if !self.original_emission.advance_host(cx)?{return Ok(None)}
        if matches!(source.original_tool_tag,Some(ToolTag::Scrub(_))){
            use semio_framework_job::{OriginalConstructionDemand,admit_original_construction};
            let bytes=std::mem::size_of::<(ToolTag,Option<ToolTag>,Option<semio_framework_plugin_tool_run::tool_jobs_tool_machine::ScrubTag>)>();
            admit_original_construction(cx,&mut source.original_press_tag,OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:0,release_bytes:0,depth:1},|_|{let Some(ToolTag::Scrub(tag))=source.original_tool_tag.take()else{unreachable!()};Ok((tag,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}))})?;
            return Ok(None)
        }
        if source.original_press_result.is_none(){
            let mut render=semio_framework_plugin_instance_render::render_bundle::RenderCtx::<A,M,X>{app:&mut *self.app,mounted_policy:self.mounted_policy,stores:&mut *self.stores,roster:&mut *self.roster,interaction:&mut *self.interaction,history:&mut *self.history,child_composition:&mut *self.child_composition,pending:&mut *self.pending,tools:&mut *self.tools,ingest:&mut *self.ingest,tool_runs:&mut *self.tool_runs,time_travel:&mut *self.time_travel,tool_machines:&mut *self.tool_machines,extension:std::marker::PhantomData};
            if !render.settle_press(&mut source.original_press_tag,self.original_emission.host_base.as_ref().unwrap(),&meta.actor,self.original_emission.host_emit.as_mut().unwrap(),cx,&mut source.original_press_result)?{return Ok(None)}
        }
        if !self.original_emission.finish_host(source.original_press_result,metadata,cx)?{return Ok(None)}
        let kind=self.tools.registry.get(action).ok_or_else(||Fault::from("original host classification lost its registered action"))?.kind;
        let previous=self.host_events.invocation_kind.replace(kind);
        let result=self.drive_original_emission(cx).await;
        self.host_events.invocation_kind=previous;
        result
    }

    pub async fn drive_original_emission(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        if self.original_emission.input.is_none()&&self.original_emission.pending(){cx.with_original_retirement(|cx|self.original_emission.close_step(cx))?;return Ok(None)}
        let mut dispatch: semio_framework_plugin_instance_dispatch::dispatch_bundle::DispatchCtx<'_,A,M,X> = semio_framework_plugin_instance_dispatch::dispatch_bundle::DispatchCtx { app: &mut *self.app, mounted_policy: self.mounted_policy, stores: &mut *self.stores, roster: &mut *self.roster, interaction: &mut *self.interaction, history: &mut *self.history, host_events: &mut *self.host_events, txn: &mut *self.txn, child_composition: &mut *self.child_composition, pending: &mut *self.pending, tools: &mut *self.tools, media: &mut *self.media, ingest: &mut *self.ingest, tool_runs: &mut *self.tool_runs, time_travel: &mut *self.time_travel, tool_machines: &mut *self.tool_machines, extension: std::marker::PhantomData };
        let mut output=None;dispatch.dispatch_original_emission(&mut self.original_emission.input,&mut output,cx).await?;Ok(output)
    }

    /// 🏷️ A direct receiving intent keeps its actual continuous tag before its arguments enter the command producer.
    pub async fn capture_original_direct_tool_tag(&mut self,action:&str,args:Option<&DslValue>,metadata:&mut Option<ActionMeta>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<bool>,semio_framework_job::OriginalTurnFailure>{
        if !self.original_typed_command.admit(cx)?{return Ok(None)}
        let catalog=self.tools.declared_tools.as_ref().and_then(semio_framework_plugin_job::original_declared_tools::DeclaredToolCatalogAlias::source).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"direct action requires its original committed tool catalog"))?;
        if !self.original_typed_command.qualify_action(catalog,action,cx)?{return Ok(None)}
        let mut typed: semio_framework_plugin_instance_typed_ops::typed_bundle::TypedCtx<'_,A,M,X> = semio_framework_plugin_instance_typed_ops::typed_bundle::TypedCtx {app:&mut *self.app,mounted_policy:self.mounted_policy,stores:&mut *self.stores,roster:&mut *self.roster,interaction:&mut *self.interaction,history:&mut *self.history,host_events:&mut *self.host_events,txn:&mut *self.txn,child_composition:&mut *self.child_composition,pending:&mut *self.pending,tools:&mut *self.tools,media:&mut *self.media,ingest:&mut *self.ingest,tool_runs:&mut *self.tool_runs,time_travel:&mut *self.time_travel,tool_machines:&mut *self.tool_machines,extension:std::marker::PhantomData};
        typed.prepare_original_tool_dispatch(action,args,metadata,&mut *self.original_typing,&mut self.original_typed_command.original_typing_ready,self.original_typed_tag,&mut self.original_typed_command.tool_tag_ready,&mut self.original_typed_command.tool_tag_dispatch,cx).await
    }

    /// 🫴️ The direct receiving frame lends its retained tag into the same atomic mounted-operation handoff.
    pub async fn drive_original_direct_typed_command(&mut self,metadata:&mut Option<ActionMeta>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        let mut dispatch: semio_framework_plugin_instance_dispatch::dispatch_bundle::DispatchCtx<'_,A,M,X> = semio_framework_plugin_instance_dispatch::dispatch_bundle::DispatchCtx {app:&mut *self.app,mounted_policy:self.mounted_policy,stores:&mut *self.stores,roster:&mut *self.roster,interaction:&mut *self.interaction,history:&mut *self.history,host_events:&mut *self.host_events,txn:&mut *self.txn,child_composition:&mut *self.child_composition,pending:&mut *self.pending,tools:&mut *self.tools,media:&mut *self.media,ingest:&mut *self.ingest,tool_runs:&mut *self.tool_runs,time_travel:&mut *self.time_travel,tool_machines:&mut *self.tool_machines,extension:std::marker::PhantomData};
        dispatch.dispatch_typed_command_inner(self.original_typed_command,self.original_typed_tag,metadata,identity,cx).await
    }

    /// 📨️ Window facts queued by this dispatch (utility switches, the `hostEvent` action's own fact) reach the app through the shared original typed source before the action runs.
    async fn drive_host_events(&mut self,action:&str,source:&semio_framework_plugin_instance_stores::action_ingress::ActionIngressSource,source_meta:&Option<ActionMeta>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Option<InvocationResult>>,semio_framework_job::OriginalTurnFailure>{
        let meta=source_meta.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original action metadata source is absent"))?;
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        let mut dispatch: semio_framework_plugin_instance_dispatch::dispatch_bundle::DispatchCtx<'_,A,M,X> = semio_framework_plugin_instance_dispatch::dispatch_bundle::DispatchCtx {app:&mut *self.app,mounted_policy:self.mounted_policy,stores:&mut *self.stores,roster:&mut *self.roster,interaction:&mut *self.interaction,history:&mut *self.history,host_events:&mut *self.host_events,txn:&mut *self.txn,child_composition:&mut *self.child_composition,pending:&mut *self.pending,tools:&mut *self.tools,media:&mut *self.media,ingest:&mut *self.ingest,tool_runs:&mut *self.tool_runs,time_travel:&mut *self.time_travel,tool_machines:&mut *self.tool_machines,extension:std::marker::PhantomData};
        Ok(match Box::pin(dispatch.host_event_turn(self.original_typed_command,action,args,meta,identity,cx)).await?{
            semio_framework_plugin_instance_dispatch::dispatch_host_events::HostEventTurn::Idle=>None,
            semio_framework_plugin_instance_dispatch::dispatch_host_events::HostEventTurn::Pending=>Some(None),
            semio_framework_plugin_instance_dispatch::dispatch_host_events::HostEventTurn::Reply(result)=>Some(Some(result)),
        })
    }

    pub async fn drive_original_typed_command(&mut self,tool_tag:&mut Option<ToolTag>,metadata:&mut Option<ActionMeta>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        let mut dispatch: semio_framework_plugin_instance_dispatch::dispatch_bundle::DispatchCtx<'_,A,M,X> = semio_framework_plugin_instance_dispatch::dispatch_bundle::DispatchCtx {app:&mut *self.app,mounted_policy:self.mounted_policy,stores:&mut *self.stores,roster:&mut *self.roster,interaction:&mut *self.interaction,history:&mut *self.history,host_events:&mut *self.host_events,txn:&mut *self.txn,child_composition:&mut *self.child_composition,pending:&mut *self.pending,tools:&mut *self.tools,media:&mut *self.media,ingest:&mut *self.ingest,tool_runs:&mut *self.tool_runs,time_travel:&mut *self.time_travel,tool_machines:&mut *self.tool_machines,extension:std::marker::PhantomData};
        dispatch.dispatch_typed_command_inner(self.original_typed_command,tool_tag,metadata,identity,cx).await
    }

    /// 🎯️ The declared original producer and authentic catalog lease prepare one exact typed command across retries.
    pub async fn prepare_original_typed_action(&mut self,action:&str,args:&mut Option<DslValue>,fault:&mut Option<Fault>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if !self.original_typed_command.admit(cx)?{return Ok(false)}
        if !self.original_typed_command.command_ready(){let factory=A::original_action_command_producer().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"application has no declared original native action command producer"))?;if !self.original_typed_command.produce_action(factory,action,args,fault,cx)?{return Ok(false)}}
        self.prepare_original_typed_admission(cx).await
    }

    pub async fn prepare_original_typed_admission(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let verb=A::command_id(self.original_typed_command.command.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original command admission requires its typed receiver output"))?).await;
        let catalog=self.tools.declared_tools.as_ref().and_then(semio_framework_plugin_job::original_declared_tools::DeclaredToolCatalogAlias::source).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native command requires its original committed tool catalog"))?;
        self.original_typed_command.admit_wire(catalog,verb,cx)
    }

    async fn prepare_original_action_source(&mut self,action:&str,source:&mut semio_framework_plugin_instance_stores::action_ingress::ActionIngressSource,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if !self.original_typed_command.admit(cx)?{return Ok(false)}
        let Some(kind)=self.original_typed_command.select_argument(source.assembled.is_some(),source.prepared.is_some(),cx)?else{return Ok(false)};
        let semio_framework_plugin_instance_stores::action_ingress::ActionIngressSource{assembled,prepared,args,fault,..}=source;
        self.prepare_original_typed_action(action,match kind{0=>assembled,1=>prepared,2=>args,_=>return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original action argument selection changed").into())},fault,cx).await
    }

    pub async fn dispatch_action(&mut self, action: &str, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, meta: &mut Option<ActionMeta>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        let Some(slot)=self.ingress.stage_action_source(action, source_args, original_fault, cx)?else{return Ok(None)};
        let Some(mut source)=self.ingress.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
        let result=self.dispatch_action_received(action,&mut source,meta,identity,cx).await;
        let terminal=!matches!(&result,Ok(None));
        self.ingress.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
        if terminal{self.ingress.pending_action_ingress_identity=None;}
        result
    }

    fn typed_with_original_typing(&mut self)->(semio_framework_plugin_instance_typed_ops::typed_bundle::TypedCtx<'_,A,M,X>,&mut semio_framework_plugin_instance_typed_ops::original_typing::OriginalTypingDispatch<A>){
        (semio_framework_plugin_instance_typed_ops::typed_bundle::TypedCtx{app:&mut *self.app,mounted_policy:self.mounted_policy,stores:&mut *self.stores,roster:&mut *self.roster,interaction:&mut *self.interaction,history:&mut *self.history,host_events:&mut *self.host_events,txn:&mut *self.txn,child_composition:&mut *self.child_composition,pending:&mut *self.pending,tools:&mut *self.tools,media:&mut *self.media,ingest:&mut *self.ingest,tool_runs:&mut *self.tool_runs,time_travel:&mut *self.time_travel,tool_machines:&mut *self.tool_machines,extension:std::marker::PhantomData::<fn()->X>},&mut *self.original_typing)
    }

    pub async fn prepare_action_typing_before(&mut self,action:&str,source:&mut semio_framework_plugin_instance_stores::action_ingress::ActionIngressSource,metadata:&mut Option<ActionMeta>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if source.original_typing_ready{return Ok(true)}let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());let(mut typed,typing)=self.typed_with_original_typing();
        if !Box::pin(typed.advance_original_typing_before(action,args,metadata,typing,cx)).await?{return Ok(false)}
        typing.finish_into(&mut source.original_typing_ready,&mut source.original_tool_dispatch,metadata,cx)
    }

    pub async fn prepare_action_tool_dispatch(&mut self,action:&str,source:&mut semio_framework_plugin_instance_stores::action_ingress::ActionIngressSource,metadata:&mut Option<ActionMeta>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<bool>,semio_framework_job::OriginalTurnFailure>{
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());let(mut typed,typing)=self.typed_with_original_typing();
        Box::pin(typed.prepare_original_tool_dispatch(action,args,metadata,typing,&mut source.original_typing_control_ready,&mut source.original_tool_tag,&mut source.original_tool_tag_ready,&mut source.original_tool_dispatch,cx)).await
    }

    async fn dispatch_action_received(&mut self,action:&str,source:&mut semio_framework_plugin_instance_stores::action_ingress::ActionIngressSource,source_meta:&mut Option<ActionMeta>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        if self.host_events.resumes.in_flight(){return Ok(None)}
        if self.host_events.delivering{if let Some(turn)=self.drive_host_events(action,source,source_meta,identity,cx).await?{return Ok(turn)}}
        if self.original_emission.host_active(){return self.drive_original_host_configuration(action,source,source_meta,cx).await}
        if self.original_typing.pending(){if self.original_typing.control_pass(){if self.prepare_action_tool_dispatch(action,source,source_meta,cx).await?.is_none(){return Ok(None)}}else if !self.prepare_action_typing_before(action,source,source_meta,cx).await?{return Ok(None)}if source.original_tool_dispatch==Some(true){self.original_emission.admit_empty(action,source_meta,cx)?;return Ok(None)}}
        if self.original_emission.input.is_some()||self.tools.original_emission_admission.is_some()||self.tools.original_emission_receipt.is_some(){return self.drive_original_emission(cx).await}
        if self.original_typed_command.input_ready()||self.original_typed_command.receipt.is_some()||self.original_typed_command.metadata.is_some(){return self.drive_original_typed_command(&mut source.original_tool_tag,source_meta,identity,cx).await}
        if self.original_typed_command.production_pending(){if !self.prepare_original_action_source(action,source,cx).await?{return Ok(None)}return self.drive_original_typed_command(&mut source.original_tool_tag,source_meta,identity,cx).await}
        let meta=source_meta.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original action metadata source is absent"))?;
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if A::ROLE == AppRole::Viewer && viewer_rejects_action(action) {
            return Err((viewer_read_only_fault(action)).into());
        }
        if action == CANCEL_TYPED_OPERATION_ACTION_ID {
            return self.tools.dispatch_operation_cancellation(args, meta).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        self.as_time_travel().dismiss_refused_reprojection(action);
        if let Some(fault) = self.ingest.document_loading_refusal(&*self.tools, action, X::admitted_while_loading(action)) {
            return Err((fault).into());
        }
        if self.time_travel.history_unavailable() && (HISTORY_ACTION_IDS.contains(&action) || action == REVERT_TO_COMMAND_ACTION_ID || X::reads_history(action)) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new(PURE_HISTORY_UNAVAILABLE_CODE), format!("'{action}' needs the document's history, which this head-only pure evaluation does not hold; use a live instance"))).into());
        }
        if let Some(turn)=self.drive_host_events(action,source,source_meta,identity,cx).await?{return Ok(turn)}
        if !self.prepare_action_typing_before(action,source,source_meta,cx).await?{return Ok(None)}
        if source.original_tool_dispatch==Some(true){self.original_emission.admit_empty(action,source_meta,cx)?;return Ok(None)}
        let meta=source_meta.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typing restored no action metadata"))?;
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if matches!(action, semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID | semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("framework.document-transfer.shell-owned"), format!("{action} is performed by the shell, never by the program"))).into());
        }
        if X::claims_action(action) {
            return Box::pin(X::dispatch_action(&mut self.as_extension(), action, args, meta, identity, cx)).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if is_framework_reserved_action_id(action) {
            return self.dispatch_framework_reserved_action(action,args,meta,cx).await;
        }
        if action == CATALOGUE_EXAMPLE_ACTION_ID && self.tools.registry.get(action).is_none() {
            let example_id = args.and_then(|value| value.get("exampleId")).and_then(DslValue::as_str).unwrap_or("");
            if let Some(document) = A::catalogue_example_document(example_id) {
                let body = document?;
                return self.dispatch_catalogue_example(action, &body, meta, identity, cx).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
        }
        let definition = self.tools.registry.get(action).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-key"), format!("UI dispatch rejected unknown action key '{action}' before command construction")))?;
        validate_ui_dispatch_classification("action", action, definition.semantics.execution.interactive_job)?;
        if A::original_host_configuration(action).is_some(){return self.drive_original_host_configuration(action,source,source_meta,cx).await}
        if source.assembled.is_none(){match self.ingest.import_staging.admit_args(args).map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))? {
            semio_framework::kernel::ImportArguments::Staged { .. } => return Ok(Some(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
            semio_framework::kernel::ImportArguments::Whole(whole) => source.assembled=Some(whole),
            semio_framework::kernel::ImportArguments::NotAnImport => (),
        }}
        let Some(settled)=self.prepare_action_tool_dispatch(action,source,source_meta,cx).await?else{return Ok(None)};
        if settled{self.original_emission.admit_empty(action,source_meta,cx)?;return Ok(None)}
        if matches!(action, SET_ACTIVE_TOOL_ACTION_ID | SET_ACTIVE_UTILITY_ACTION_ID) {
            self.original_emission.admit_empty(action,source_meta,cx)?;
            Ok(None)
        } else {
            if !self.prepare_original_action_source(action,source,cx).await?{return Ok(None)}
            self.media.require_complete_tool_operation_pipeline(self.original_typed_command.admission.as_ref().unwrap())?;
            let result=self.drive_original_typed_command(&mut source.original_tool_tag,source_meta,identity,cx).await;
            result
        }
    }

    /// 🎯️ Validates manifest structural app/mode ownership, converts the addressed
    /// invocation arguments once into the app's typed command, and enters the typed dispatch channel.
    pub async fn dispatch_command(&mut self, address: &ManifestCommandAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, source_meta: &mut Option<ActionMeta>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        if self.host_events.resumes.in_flight(){return Ok(None)}
        let command_id=address.command_id.as_str();
        let Some(slot)=self.ingress.stage_action_source(command_id, source_args, original_fault, cx)?else{return Ok(None)};
        let Some(mut source)=self.ingress.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
        let result=async {
        if self.original_emission.host_active(){return self.drive_original_host_configuration(command_id,&mut source,source_meta,cx).await}
        if self.original_typing.pending(){if self.original_typing.control_pass(){if self.prepare_action_tool_dispatch(command_id,&mut source,source_meta,cx).await?.is_none(){return Ok(None)}}else if !self.prepare_action_typing_before(command_id,&mut source,source_meta,cx).await?{return Ok(None)}if source.original_tool_dispatch==Some(true){self.original_emission.admit_empty(command_id,source_meta,cx)?;return Ok(None)}}
        if self.original_emission.input.is_some()||self.tools.original_emission_admission.is_some()||self.tools.original_emission_receipt.is_some(){return self.drive_original_emission(cx).await}
        if self.original_typed_command.input_ready()||self.original_typed_command.receipt.is_some()||self.original_typed_command.metadata.is_some(){return self.drive_original_typed_command(&mut source.original_tool_tag,source_meta,identity,cx).await}
        if self.original_typed_command.production_pending(){if !self.prepare_original_action_source(command_id,&mut source,cx).await?{return Ok(None)}return self.drive_original_typed_command(&mut source.original_tool_tag,source_meta,identity,cx).await}
        let meta=source_meta.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original command metadata source is absent"))?;
        let owner_app_id = self.app.instance_id().await.to_string();
        let (owner, definition) = match &address.owner {
            ManifestCommandOwnerAddress::App { app_id, .. } => {
                if app_id != &owner_app_id {
                    return Err((Fault::from(format!("command '{command_id}' is not owned by app {owner_app_id}"))).into());
                }
                (format!("app {owner_app_id}"), self.tools.registry.app_command(command_id).await.ok_or_else(|| Fault::from(format!("command '{command_id}' is not owned by app {owner_app_id}")))?)
            }
            ManifestCommandOwnerAddress::Mode { app_id, mode_id, .. } => {
                if app_id != &owner_app_id || active_mode_id != Some(mode_id.as_str()) {
                    return Err((Fault::from(format!("command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}"))).into());
                }
                (format!("mode {mode_id}"), self.tools.registry.mode_command(mode_id, command_id).await.ok_or_else(|| Fault::from(format!("command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}")))?)
            }
            _ => return Err((Fault::from(format!("command '{command_id}' is not app- or mode-owned"))).into()),
        };
        validate_ui_dispatch_classification(&owner, command_id, definition.semantics.execution.interactive_job)?;
        let kind = definition.kind;
        if A::original_host_configuration(command_id).is_some(){return self.drive_original_host_configuration(command_id,&mut source,source_meta,cx).await}
        let args = source.args.as_ref();
        match self.ingest.import_staging.admit_args(args).map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))? {
            semio_framework::kernel::ImportArguments::Staged { .. } => return Ok(Some(ToolOperations::<A>::empty_result(command_id, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
            semio_framework::kernel::ImportArguments::Whole(whole) => source.assembled=Some(whole),
            semio_framework::kernel::ImportArguments::NotAnImport => (),
        };
        let Some(settled)=self.prepare_action_tool_dispatch(command_id,&mut source,source_meta,cx).await?else{return Ok(None)};
        if settled{self.original_emission.admit_empty(command_id,source_meta,cx)?;return Ok(None)}
        if !self.prepare_original_action_source(command_id,&mut source,cx).await?{return Ok(None)}
        self.media.require_complete_tool_operation_pipeline(self.original_typed_command.admission.as_ref().unwrap())?;
        let previous_kind = self.host_events.invocation_kind.replace(kind);
        let result=self.drive_original_typed_command(&mut source.original_tool_tag,source_meta,identity,cx).await;
        self.host_events.invocation_kind=previous_kind;
        result
        }.await;
        let terminal=!matches!(&result,Ok(None));
        self.ingress.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
        if terminal{self.ingress.pending_action_ingress_identity=None;}
        result
    }

    /// 🔐️ The agent lane's revision fill (ticket 26/09/23, G12 session 14c): a declared
    /// [`ActionArgDef::document_revision`] / [`ActionArgDef::target_revision`] argument the agent omitted is admitted against
    /// the token its rendered binding carries right now — the document's canonical revision (what every document-scoped
    /// verb compares against on this lane), or the app's own [`ArtifactApp::agent_target_revision`]; a target the app cannot
    /// resolve is refused by name. Only this lane fills: the shell lanes pass the binding's token and the app's parser
    /// refuses its absence there; the gateway admits an omission only behind a passed MCP `expectedRevision` (the
    /// document-level stale-write guard), and its prepare→invoke pair refuses a document that moved in between.
    async fn fill_agent_revisions(&mut self, address: &semio_framework::manifest::ActionAddress, arguments: &mut BTreeMap<String, DslValue>) -> Result<(), Fault> {
        let declared = match address.window_kind_id.as_str() {
            "*" => self.tools.registry.get(&address.action_id).map(|action| action.args.clone()),
            kind => self.tools.registry.window_action(kind, &address.action_id).await.map(|action| action.args.clone()),
        }
        .unwrap_or_default();
        let omitted: Vec<_> = declared.into_iter().filter(|arg| !arguments.contains_key(&arg.id)).collect();
        for arg in omitted.iter() {
            let semio_framework::manifest::ArgSchema::String { format: Some(format), .. } = &arg.schema else { continue };
            let token = match format {
                semio_framework::manifest::ArgFormat::DocumentRevision => self.stores.store.content_revision().iter().map(|byte| format!("{byte:02x}")).collect::<String>(),
                semio_framework::manifest::ArgFormat::TargetRevision => {
                    self.as_render().refresh_cache().await?;
                    let render_operation = self.tools.live_render_operation(&self.mounted_policy, &*self.stores);
                    let parent_document_id = self.stores.store.envelope().id.clone();
                    let staged = DslValue::Object(arguments.iter().map(|(key, value)| (key.clone(), value.clone())).collect());
                    let (_, snapshot, _, history) = self.history.cache.as_ref().expect("cache refreshed above");
                    let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(&self.child_composition.child_content_root), render_operation, &parent_document_id, None).await.with_instance(self.tools.instance_operation_owner.clone());
                    A::agent_target_revision(&address.action_id, &staged, &doc).await?.ok_or_else(|| {
                        Fault::new(
                            FaultOrigin::Framework,
                            FaultCode::new("agent.target-revision-unresolved"),
                            format!("action '{}' binds its `{}` to one addressed target and this app resolves no such target for an agent — pass the target's revision", address.action_id, arg.id),
                        )
                    })?
                }
                _ => continue,
            };
            arguments.insert(arg.id.clone(), DslValue::String(token));
        }
        Ok(())
    }

    /// 🧮️ PHASE ONE of the agent lane's two-phase contract: validate, price, and produce the ops
    /// **without applying any of them**.
    ///
    /// 🧭️ Why this cannot be `AppCommand::Command`. The shell's dispatch lane APPLIES: it runs
    /// `handle_action_invocation` → `dispatch_typed_command_inner`, which mounts a typed
    /// operation whose publication ladder commits into `self.stores.store` and mints an undo group. An
    /// agent's `action_prepare` must not do that — its own protocol commits separately
    /// (`TransactionPrepare{ops}` → `TransactionCommit`), and `RoutingArtifactChannel` caches ONE
    /// guest instance per plugin, so a prepare that applied would leave the commit applying the
    /// same edit a second time. `history_undo` (which walks back exactly one application) would
    /// then leave half the edit standing — a GREEN gate that lies, which is worse than a red one.
    ///
    /// ✅️ What it shares with the shell lane, deliberately and by call rather than by copy:
    /// the owner-qualified `ManifestActionInvocation` address, `admit_addressed_action_view`'s
    /// window projection, the mode/window-kind ownership checks `handle_action_invocation` runs,
    /// the `windowId` argument it injects, `validate_ui_dispatch_classification`'s interactive-job
    /// discipline, `A::command_from_action`, the exact retained-wire admission `dispatch_action`
    /// begins (`admit_command_wire` + `require_complete_tool_operation_pipeline`) and the app's own
    /// job, built and dispatched through the same factory entry ([`Self::preview_typed_command_job`]).
    /// The one thing it does NOT share is the applying tail: the job runs to the completion it would
    /// publish, and nothing is published. It fails closed ([`agent_lane_carriage`]): a completion that publishes
    /// anything an agent transaction cannot carry is refused by name, never settled without it, and one that carries
    /// nothing but a notice or a selection is refused `app.command.no-effect`, its notices as structured causes.
    ///
    /// 🎟️ The admission it begins is consumed by the previewed job within this same call and retired
    /// through that job's own close protocol, so a prepared handle the caller never invokes leaves
    /// nothing behind.
    ///
    /// 🪟️ `"*"` is the catalog's own marker for an APP-scope verb — one that belongs to no
    /// particular window kind (`🗒️note` declares all 48 of its verbs this way). A concrete
    /// kind must own the verb; `"*"` addresses the app registry directly, exactly as
    /// `dispatch_action` does for every verb the shell sends.
    pub async fn preview_addressed_action(&mut self, invocation: &ManifestActionInvocation, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        if self.host_events.resumes.in_flight(){return Ok(None)}
        let address=&invocation.address;
        let owner_app_id = self.app.instance_id().await;
        if address.app_id != owner_app_id {
            return Err((plugin_sdk_fault(format!("action app owner {} does not match {owner_app_id}", address.app_id))).into());
        }
        if address.window_instance_id.trim().is_empty() {
            return Err((plugin_sdk_fault("action window instance id must be non-empty")).into());
        }
        let reserved_kind = framework_reserved_action_kind(&address.action_id);
        if reserved_kind.is_none() && !self.tools.registry.has_mode(&address.mode_id).await {
            return Err((plugin_sdk_fault(format!("unknown action mode owner {}", address.mode_id))).into());
        }
        if address.window_kind_id != "*" && reserved_kind.is_none() && self.tools.registry.window_action(&address.window_kind_id, &address.action_id).await.is_none() {
            return Err((plugin_sdk_fault(format!("window kind {} does not own action {}", address.window_kind_id, address.action_id))).into());
        }
        let context = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault("addressed action requires the caller's explicit language and terminology context"))?;
        let view = semio_framework_plugin_reactor_app::admit_addressed_action_view(context, &invocation.address)?;
        let definition = self
            .tools.registry
            .get(&address.action_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-key"), format!("UI dispatch rejected unknown action key '{}' before command construction", address.action_id)))?;
        let kind = definition.kind;
        validate_ui_dispatch_classification("action", &address.action_id, definition.semantics.execution.interactive_job)?;
        if self.tools.agent_lane_preview.as_ref().is_none_or(|state|state.starting){
            if !self.admit_agent_lane_preview_key(meta,cx)?{return Ok(None)}
            if !self.ingress.action_source_capture_ready(&address.action_id, cx){return Ok(None)}
            let mut original_args=if self.ingress.action_source_pending(){None}else{let mut arguments=invocation.arguments.clone();arguments.insert("windowId".into(),DslValue::String(address.window_instance_id.clone()));self.fill_agent_revisions(address,&mut arguments).await?;
            Some(DslValue::Object(arguments.into_iter().collect()))};
            let mut original_fault=None;
            let Some(slot)=self.ingress.stage_action_source(&address.action_id,&mut original_args,&mut original_fault,cx)?else{return Ok(None)};
            let Some(mut source)=self.ingress.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
            let prepared=self.prepare_original_action_source(&address.action_id,&mut source,cx).await;
            let terminal=!matches!(&prepared,Ok(false));
            self.ingress.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
            if terminal{self.ingress.pending_action_ingress_identity=None;}
            if !prepared?{return Ok(None)}
            let (Some(command),Some(admission))=(self.original_typed_command.command.take(),self.original_typed_command.admission.take())else{return Err(plugin_sdk_fault("original preview source lost its produced command or admission").into())};
            self.media.require_complete_tool_operation_pipeline(&admission)?;
            self.begin_agent_lane_preview(command,admission,&ActionMeta{view_state:Some(view),..meta.clone()},cx).await?;
            return Ok(None)
        }
        if self.original_typed_command.pending()&&self.original_typed_command.metadata.is_none()&&self.original_typed_command.operation.is_none(){cx.with_original_retirement(|cx|self.original_typed_command.close_step(cx))?;return Ok(None)}
        let maximum_bytes=self.tools.agent_lane_preview.as_ref().unwrap().maximum_bytes;
        let Some((emit,(presence,transient,window_transient),job_steps))=self.tools.advance_agent_lane_preview(&address.action_id, cx)?else{return Ok(None)};
        let carriage = agent_lane_carriage(&AgentLanePublication {
            carried_ops: emit.artifact_mutations.len() + emit.config_mutations.len() + emit.draft_mutations.len() + emit.child_emits.len(),
            effects: &emit.effects,
            window_config_ops: emit.window_config_mutations.len(),
            extension_calls: emit.extension_invocations.len(),
            events: emit.events.len(),
            selection_writes: emit.interaction_writes.len(),
            tasks: emit.tasks.len(),
            presence,
            transient,
            window_transient,
        });
        match carriage {
            AgentLaneCarriage::Carried => {}
            AgentLaneCarriage::NoEffect => return Err((agent_lane_no_effect_fault(&address.action_id, &emit.effects)).into()),
            AgentLaneCarriage::Uncarried(lanes) => return Err((agent_lane_uncarried_fault(&address.action_id, &lanes.join(", "))).into()),
        }
        if A::ROLE == AppRole::Viewer && (!emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty()) {
            return Err((viewer_read_only_fault(&address.action_id)).into());
        }
        let mut artifact_op_bytes = Vec::with_capacity(emit.artifact_mutations.len());
        for op in emit.artifact_mutations.iter() {
            artifact_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        for child in emit.child_emits.iter() {
            if !child.owner.is_empty() || self.child_composition.children.get(&(child.slot.clone(), child.child_id.clone())).is_none() {
                return Err((Fault::new(
                    FaultOrigin::Framework,
                    FaultCode::new("interactive-job.agent-lane-child-missing"),
                    format!("action '{}' edits owned child {:?} in slot {:?}, which this instance does not hold", address.action_id, child.child_id, child.slot),
                )).into());
            }
        }
        let child_op_bytes = ChildEmit::encode_groups(&emit.child_emits);
        let mut config_op_bytes = Vec::with_capacity(emit.config_mutations.len());
        for op in emit.config_mutations.iter() {
            config_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        let mut draft_op_bytes = Vec::with_capacity(emit.draft_mutations.len());
        for op in emit.draft_mutations.iter() {
            draft_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        let priced = artifact_op_bytes.iter().chain(config_op_bytes.iter()).chain(draft_op_bytes.iter()).map(Vec::len).sum::<usize>();
        if priced > maximum_bytes {
            return Err((Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.preview-output"),
                format!("previewed action '{}' produced {priced} op byte(s); its exact output cap is {}", address.action_id, maximum_bytes),
            )).into());
        }
        self.pending.last_emit_wire = Some(EmitWire { document: protocol::encode_ops_vec(&artifact_op_bytes), config: protocol::encode_ops_vec(&config_op_bytes), draft: protocol::encode_ops_vec(&draft_op_bytes), children: child_op_bytes });
        let mut result = ToolOperations::<A>::empty_result(&address.action_id, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
        result.output = DslValue::Object(vec![
            ("previewedAction".into(), DslValue::String(address.action_id.clone())),
            ("actionKind".into(), DslValue::String(format!("{kind:?}"))),
            ("documentOps".into(), DslValue::String(artifact_op_bytes.len().to_string())),
            ("configOps".into(), DslValue::String(config_op_bytes.len().to_string())),
            ("draftOps".into(), DslValue::String(draft_op_bytes.len().to_string())),
            ("childGroups".into(), DslValue::String(emit.child_emits.len().to_string())),
            ("opBytes".into(), DslValue::String(priced.to_string())),
            ("jobSteps".into(), DslValue::String(job_steps.to_string())),
        ]);
        Ok(Some(result))
    }

    /// 👁️ Builds the SAME app-owned job the shell lane builds for this admitted command (`A::build_tool_job` over
    /// [`Self::capture_typed_command_roots`]), dispatches it through the SAME retained-wire factory entry
    /// `start_typed_command_operation` uses, and drives it through its own steps on this turn
    /// ([`drive_agent_lane_preview`]) to the completion it would publish — publishing nothing. Every job shape an
    /// app owns previews this way: the SDK's retained command, a plugin's own tool-command job (puzzle, writer, …)
    /// and a download job, whose file an agent transaction cannot carry and is refused by name. The run holds a
    /// keyed cancellation lease, so cancelling the document cancels the preview. Answers the emit, how many presence,
    /// transient and window-transient mutations the job's completion would publish (lanes an agent
    /// transaction cannot carry, see [`agent_lane_uncarried_lanes`]) and how many of the job's own steps ran.
    fn admit_agent_lane_preview_key(&mut self,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.tools.agent_lane_preview.is_none(){
            let grant=cx.retained_grant();let bytes=size_of::<semio_framework_plugin_job::AgentLanePreview<A>>();if cx.should_yield()||grant.maximum_items<2||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
            let Some(operation_id)=self.media.admit_typed_operation_slot(&*self.pending, &*self.tools)else{return Ok(false)};
            let revision=self.stores.store.content_revision_now();let key=ToolOperationKey{app_instance_id:meta.instance_id,document:ArtifactDocumentAuthority(meta.instance_id),operation_id,base_revision:semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().expect("revision lane width"))),generation:semio_framework_job::Generation(self.stores.store.generation_now())};
            let Some(caller)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(false)};
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
            let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:grant,site:"plugin_agent_lane_preview",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:0,step_budget_us:0};
            self.tools.agent_lane_preview=Some(semio_framework_plugin_job::AgentLanePreview{key:Some(key),caller:std::mem::ManuallyDrop::new(Some(caller)),starting:true,producer_request:None,producer_request_retirement:None,producer:None,children:None,completion:None,completion_close:None,lease:None,lease_close:None,config,maximum_bytes:0,steps:0,publication:None,complete:false,closing:false,publication_close:None});
        }
        let state=self.tools.agent_lane_preview.as_mut().unwrap();
        if state.closing{return Ok(false)}
        state.caller.as_ref().ok_or_else(||plugin_sdk_fault("original preview caller source is absent"))?.validate_context(cx)?;
        let key=state.key.as_ref().ok_or_else(||plugin_sdk_fault("original preview operation key is absent"))?;
        if key.app_instance_id!=meta.instance_id{return Err(plugin_sdk_fault("original preview operation belongs to another app instance").into())}
        if state.lease.is_none(){state.lease=self.tools.tool_cancellations.begin_keyed(key,cx)?;}
        Ok(state.lease.is_some())
    }

    async fn begin_agent_lane_preview(&mut self, command: Box<A::Command>, admission: AdmittedToolCommand, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure> {
        let verb = admission.verb.clone();
        let roots = self.as_dispatch().capture_typed_command_roots(command.as_ref(), meta).await?;
        let state=self.tools.agent_lane_preview.as_ref().ok_or_else(||plugin_sdk_fault("original preview admission source is absent"))?;
        let key=state.key.as_ref().ok_or_else(||plugin_sdk_fault("original preview operation key is absent"))?;
        if !state.starting||state.lease.is_none()||key.base_revision!=roots.base_revision||key.generation!=roots.generation{return Err(plugin_sdk_fault("original preview roots no longer match their admitted operation key").into())}
        let operation_id=key.operation_id;
        let seed_handle = artifact_handle_of(&format!("{}/{}/{verb}/{}", meta.instance_id, self.tools.tool_controller_id(), roots.base_revision.0)).await;
        let operation = semio_framework_job::Operation::new(operation_id, roots.base_revision, roots.generation, (seed_handle.0 as u64) ^ ((seed_handle.0 >> 64) as u64));
        let completion = ArtifactToolCompletion::<A>::new();
        let presence_local = self.stores.presence_store.local_read().map_err(Fault::from)?;
        let (context, _context_birth) = ArtifactOwnedContextHandle::admit(
            ArtifactOwnedToolJobContext::new(
                meta.instance_id,
                meta.view_state.clone(),
                roots.canonical_base_revision,
                roots.draft_generation,
                roots.transient_generation,
                ArtifactOwnedToolJobSnapshots {
                    children: std::sync::Arc::clone(&roots.children),
                    draft: std::sync::Arc::clone(&roots.draft_snapshot),
                    transient: std::sync::Arc::clone(&roots.transient),
                    window_config: roots.window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                    window_transient: roots.window_transient_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                },
            )
            .with_source_issuers(&self.stores.draft_store, &self.stores.transient_store)
            .with_presence(presence_local, std::sync::Arc::clone(&roots.presence_peers), self.stores.presence_store.peer_retirement_factory())
            .with_original_tool_run(self.tool_runs.capture_original_view_for(meta.view_state.as_ref().and_then(|view|view.window_id.as_deref()),cx)?)
            .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation()),
            artifact_owned_tool_birth_grant(),
        );
        let key = admission.proof.key();
        let contract = admission.proof.contract();
        let request = ArtifactOwnedToolJobRequest {
            source_binding: semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx),
                retained:self.mounted_policy.maintenance,
            command: semio_framework_plugin_decl_app::tool_contract::ArtifactOriginalToolCommand::from_admitted_op_binary(command,&admission.raw_wire,semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx)),
            input: Some(admission.raw_wire),
            checkpoint: None,
            operation,
            controller_id: key.controller_id,
            tool_id: key.tool_id,
            payload_schema_id: admission.proof.schema_id(),
            contract,
            decoded_items: admission.decoded_items,
            app_instance_id: meta.instance_id,
            parent_document_id: self.stores.store.envelope().id.clone(),
            canonical_base_revision: roots.canonical_base_revision,
            authoring_seed: self.stores.authoring_seed(&meta.actor),
            snapshot: roots.snapshot,
            snapshot_read: self.stores.store.snapshot_read().map_err(FaultFrom::into_fault)?,
            config: roots.config,
            window_config: roots.window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
            history: roots.history,
            interaction_state: roots.interaction_state,
            interaction_hover: std::sync::Arc::new(roots.interaction_hover),
            context,
            instance_operation_owner: self.tools.instance_operation_owner.clone(),
            output_chunks: ArtifactOutputChunks::new(contract.max_output_bytes),
            completion: completion.clone(),
        };
        let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:cx.retained_grant(),site:"plugin_agent_lane_preview",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:contract.max_work_units_per_step,step_budget_us:u64::from(contract.max_step_micros)};
        let state=self.tools.agent_lane_preview.as_mut().expect("original preview lease remains received");state.producer_request=Some(request);state.completion=Some(completion);state.config=config;state.maximum_bytes=contract.max_output_bytes;state.starting=false;
        Ok(())
    }

    /// 🧩️ A typed frame IS an owner-qualified invocation — the exact wire `AppCommand::Command`
    /// carries. Decoding it here is what gives the agent lane the same dispatch the shell's UI
    /// lane has; running it through [`Self::preview_addressed_action`] rather than
    /// `handle_action_invocation` is what makes it a PREPARE (see that method's own doc).
    pub async fn dispatch_command_frame(&mut self, command_bytes: &[u8], meta: &ActionMeta, _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let invocation: ManifestActionInvocation = semio_framework_plugin_plugin_program::decode_wire_serialized(command_bytes).await.map_err(|_| {
            Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.missing-exact-key"),
                "a typed command frame carries one owner-qualified ManifestActionInvocation — the same wire AppCommand::Command carries; this frame decoded as neither, so no command-specific pre-serde envelope can be selected",
            )
        })?;
        self.preview_addressed_action(&invocation, meta, cx).await
    }
}

//! 🎞️ Media import, export and fingerprint turns of the mounted instance.

use super::*;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    pub(super) async fn prepare_media_import(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let source=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?);
        if let Some((_,original))=self.media.pending_reserved_producer_permit.as_ref(){
            if *original!=source{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media import requires its retained original receiving source").into())}
            return Ok(!cx.should_yield()&&cx.retained_grant().maximum_items>0&&cx.retained_grant().maximum_depth>0)
        }
        let meta=ActionMeta{ expected_document_revision:None,actor:self.stores.store.local_actor_id().0.clone(),instance_id:self.tools.live_runtime_instance_id.ok_or_else(||plugin_sdk_fault("media import requires its mounted live instance"))?,view_state:None};
        let Some(permit)=self.dispatch_ctx().admit_reserved_emit_producer("import-media",&meta,cx)?else{return Ok(false)};
        *self.media.pending_reserved_producer_permit=Some((permit,source));Ok(false)
    }

    pub(super) async fn advance_original_media_import(&mut self, port: &str, media: &mut Option<Media>, meta: &ActionMeta, progress: &mut semio_plugin_reactor_app::OriginalMediaImport, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        use semio_plugin_decl_app::tool_contract::{ArtifactBoundedToolFault, TypedOperationResultLane, decode_typed_operation_fault_page};
        use semio_framework_value::{RetainedCloneProgress, RetirementDemand};
        if progress.complete {
            return Ok(true);
        }
        let Some(operation) = progress.operation else {
            self.dispatch_ctx().advance_original_reserved_media(port, media, meta, &mut progress.operation, cx).await?;
            return Ok(false);
        };

        if let Some(mounted) = self.tools.tool_operations.get(operation.operation.0) {
            let awaiting_page = mounted.result_page.as_ref().filter(|_| mounted.stage == MountedTypedCommandFullOperationStage::AwaitingAck);
            if progress.terminal.is_none() {
                let terminal = mounted.terminal_fault.map(Err).or_else(|| {
                    let page = mounted.result_page.as_ref()?;
                    match page.lane {
                        TypedOperationResultLane::Fault => Some(Err(ArtifactBoundedToolFault::from_fault(&decode_typed_operation_fault_page(page.bytes())))),
                        TypedOperationResultLane::Terminal if mounted.user_cancel_requested => Some(Err(ArtifactBoundedToolFault::from_fault(&Fault::from("original media import was cancelled")))),
                        TypedOperationResultLane::Terminal | TypedOperationResultLane::Download => Some(Ok(())),
                        _ => None,
                    }
                });
                if let Some(terminal) = terminal {
                    let copy_bytes = size_of_val(&progress.terminal);
                    let demand = RetirementDemand { copy_bytes, depth: 1, ..Default::default() };
                    if cx.retirement_should_yield() || !cx.retained_work_can_enter(demand) {
                        return Ok(false);
                    }
                    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() })?;
                    cx.consume_fuel(1);
                    progress.terminal = Some(terminal);
                }
            }
            if let Some(token) = awaiting_page.map(|page| page.token) {
                let copy_bytes = size_of_val(&token);
                let demand = RetirementDemand { copy_bytes, depth: 1, ..Default::default() };
                if cx.retirement_should_yield() || !cx.retained_work_can_enter(demand) {
                    return Ok(false);
                }
                if !self.members_ctx().acknowledge_typed_operation_result(token)? {
                    return Err(ValueError::invariant("original media import lost its exact publication acknowledgement").into());
                }
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() })?;
                cx.consume_fuel(1);
            }
            return Ok(false);
        }
        let terminal = progress.terminal.ok_or_else(|| ValueError::invariant("original media import retired without a terminal publication"))?;
        terminal.map_err(|fault| fault.into_fault())?;
        progress.complete = true;
        Ok(true)
    }

    pub(super) async fn submit_media_export(&mut self,port:&str,receiver:&mut Option<ArtifactMediaExportHandle>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.media.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        let Some(app_instance_id)=self.tools.live_runtime_instance_id else{return self.media.retain_media_submission_refusal::<()>(MediaError::Payload(port.to_string(),"resumable media export requires a bound live app instance".into()), cx).map(|_|false)};
        self.media.receive_owned_media_export(&mut self.stores, &mut self.tools, &self.child_composition.child_content_root, port, None, app_instance_id, Some(receiver), cx).await
    }

    pub(super) async fn submit_media_export_checked(&mut self,port:&str,expected_parent_document_id:&str,expected_base_revision:u64,checkpoint_id:Option<&str>,receiver:&mut Option<ArtifactMediaExportHandle>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.media.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        let parent_document_id=self.stores.store.envelope().id.as_str();let content_revision=self.stores.store.content_revision();let base_revision=u64::from_be_bytes(content_revision[..8].try_into().expect("revision lane width"));
        if parent_document_id!=expected_parent_document_id||base_revision!=expected_base_revision{return self.media.retain_media_submission_refusal::<()>(MediaError::Payload(port.to_string(),"media export resource authority is stale".into()), cx).map(|_|false)}
        let Some(app_instance_id)=self.tools.live_runtime_instance_id else{return self.media.retain_media_submission_refusal::<()>(MediaError::Payload(port.to_string(),"resumable media export requires a bound live app instance".into()), cx).map(|_|false)};
        self.media.receive_owned_media_export(&mut self.stores, &mut self.tools, &self.child_composition.child_content_root, port, checkpoint_id, app_instance_id, Some(receiver), cx).await
    }

    pub(super) async fn poll_media_export(&mut self,handle:semio_plugin_decl_media::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<ArtifactMediaExportPoll>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.media.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        self.media.snapshot_retirements.get(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media polling lost its original source"))?.validate(cx)?;
        if receiver.is_some(){return Ok(true)}
        if self.media.media_exports.get(handle.operation_id.0).is_none_or(|active|active.handle.original_loan()!=handle){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media polling requires its exact original live handle").into())}
        let grant=cx.retained_grant();let bytes=size_of::<ArtifactMediaExportPoll>()+size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        let Some(poll)=self.media.receive_owned_media_poll(&mut self.stores, handle.operation_id.0, cx).await?else{return Ok(false)};
        if matches!(poll,ArtifactMediaExportPoll::Complete(_)){self.media.media_closures.get_mut(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"terminal media poll lost its original close owner"))?.download_pending=true}
        *receiver=Some(poll);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }

    pub(super) async fn import_media(&mut self,port:&str,media:&mut Option<Media>,meta:&ActionMeta,cx:&mut StepContext<'_>)->Result<Option<InvocationResult>,OriginalTurnFailure>{
        let log_generation_before=self.history.log_generation;
        let Some(result)=self.dispatch_ctx().dispatch_import_media(port,media,meta,cx).await? else{return Ok(None)};
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before,&format!("import-media:{port}"),result).await))
    }

    pub(super) async fn media_fingerprint(&mut self, port: &str) -> Result<MediaFingerprint, MediaError> {
        self.render_ctx().refresh_cache().await.map_err(|error| MediaError::Payload(port.to_string(), error.message))?;
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let VcsArtifactApp { app: _, history: command_history, child_composition, tools, .. } = self;
        let (_, snapshot, config, history) = command_history.cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(&child_composition.child_content_root), render_operation, &parent_document_id, None).await.with_instance(tools.instance_operation_owner.clone());
        let _cfg = ConfigView { snapshot: config.as_ref(), window: None };
        A::media_fingerprint(port, &doc).await
    }
}

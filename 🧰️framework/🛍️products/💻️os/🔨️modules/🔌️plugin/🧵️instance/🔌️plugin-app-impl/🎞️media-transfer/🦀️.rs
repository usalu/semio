//! 🎞️ Media transfer of the mounted artifact instance: the production and consumption of a media artifact and the retained export result. See [`super`].

use super::*;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    /// 📤️ ABI-level media delivery for ONE port — the route `AppCommand::MediaOut` reaches
    /// through `plugin_produce_media`.
    ///
    /// 🎞️ Produces one document or declared media output while preserving the initiating pending turn.
    pub(super) async fn produce_media(&mut self, port: &str, owner: &mut semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner<'_,'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<MediaArtifact>,semio_framework_plugin_decl_app::MediaArtifactOperationFailure> {
        if port == NATURAL_FILE_PORT {
            let codec = A::natural_file_codec().await.ok_or_else(|| MediaArtifactError::Payload("natural file exporter is not mounted".into()))?;
            self.render_ctx().refresh_cache().await?;
            let snapshot = self.history.cache.as_ref().expect("cache refreshed above").1.as_ref();
            let data = A::encode_natural_file(snapshot).await.map_err(MediaArtifactError::Export)?;
            return Ok(Some(MediaArtifact {
                descriptor: MediaArtifactDescriptor {
                    edge_id: None,
                    port_id: Some(port.to_string()),
                    kind_id: Some(codec.format_kind.to_string()),
                    media_type: None,
                    wire: MediaWireFormat::Binary { format_kind: codec.format_kind.to_string() },
                    blob_hash: None,
                },
                data,
            }));
        }
        if port != "artifact:out" {
            let exported = owner.receive_nested::<Media, Result<Option<MediaArtifact>,semio_framework_plugin_decl_app::MediaArtifactOperationFailure>>(|slot, original| {
                let media = match ::semio_framework_async::poll::resolve_ready(self.export_media(port,cx)) {
                    Ok(Some(media))=>media,
                    Ok(None)=>return Ok(Ok(None)),
                    Err(error)=>return Ok(Err(error.into())),
                };
                *slot = Some(media);
                let media = slot.as_ref().unwrap();
                let artifact = match &media.payload {
                    MediaPayload::Intrinsic { schema, value } => MediaArtifact::encode_intrinsic(port, media.media_type, schema, value, original).map_err(|error| match error { MediaArtifactError::Value(error) => error, _ => media_projection_error() })?,
                    MediaPayload::Structured { schema, json } => {
                        let (port, schema, text) = copy_media_text_parts(port, schema, json, original)?;
                        original.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<String>(), ..Default::default() })?;
                        MediaArtifact { descriptor: MediaArtifactDescriptor { edge_id: None, port_id: Some(port), kind_id: None, media_type: Some(media.media_type), wire: MediaWireFormat::Binary { format_kind: schema }, blob_hash: None }, data: text.into_bytes() }
                    }
                    MediaPayload::Binary { format_kind, blob_hash } => {
                        let (port, format_kind, blob_hash) = copy_media_text_parts(port, format_kind, blob_hash, original)?;
                        MediaArtifact { descriptor: MediaArtifactDescriptor { edge_id: None, port_id: Some(port), kind_id: None, media_type: Some(media.media_type), wire: MediaWireFormat::Binary { format_kind }, blob_hash: Some(blob_hash) }, data: Vec::new() }
                    }
                };
                Ok(Ok(Some(artifact)))
            })??;
            return Ok(exported);
        }
        let archive = PluginApp::document_archive(self).await?;
        whole_document_media(port, self.artifact_schema().await.to_string(), &archive).await.map(Some).map_err(Into::into)
    }

    pub(super) async fn consume_media(&mut self, port: &str, artifact: MediaArtifact, cx:&mut semio_framework_job::StepContext<'_>) -> Result<MediaConsumption, MediaArtifactError> {
        if port != NATURAL_FILE_PORT {
            let schema_now = self.artifact_schema().await.to_string();
            return match artifact.descriptor.wire {
                MediaWireFormat::Document { schema } if schema == schema_now => whole_document_archive(&artifact.data).await.map(MediaConsumption::DocumentLoad),
                MediaWireFormat::Document { schema } => Err(MediaArtifactError::SchemaMismatch { expected: schema_now, found: schema }),
                MediaWireFormat::Binary { format_kind } => Err(MediaArtifactError::NoImporter(format_kind)),
                MediaWireFormat::Intrinsic { .. } => {
                    let media = {
                        let mut accept = |_| true;
                        let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES, &mut accept);
                        artifact.decode_intrinsic(&mut control)?
                    };
                    let instance_id = self.tools.live_runtime_instance_id.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "intrinsic media import requires a bound live app instance"))?;
                    let actor = self.stores.store.local_actor_id().0.clone();
                    self.dispatch_ctx().dispatch_import_media(port, media, &ActionMeta { actor, instance_id, view_state: None }, cx).await.map(|_| MediaConsumption::Applied).map_err(MediaArtifactError::Import)
                }
            };
        }
        let codec = A::natural_file_codec().await.ok_or_else(|| MediaArtifactError::NoImporter(port.to_string()))?;
        let MediaWireFormat::Binary { format_kind } = artifact.descriptor.wire else {
            return Err(MediaArtifactError::NoImporter(codec.format_kind.to_string()));
        };
        if format_kind != codec.format_kind {
            return Err(MediaArtifactError::NoImporter(format_kind));
        }
        let instance_id = self.tools.live_runtime_instance_id.ok_or_else(|| MediaArtifactError::Payload("natural file import requires a bound live app instance".into()))?;
        let actor = self.stores.store.local_actor_id().0.clone();
        let media_type = A::io().await.map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
        let media = natural_file_input_media(codec, media_type, artifact.data);
        self.dispatch_ctx().dispatch_import_media(port, media, &ActionMeta { actor, instance_id, view_state: None }, cx)
            .await
            .map(|_| MediaConsumption::Applied)
            .map_err(|fault| MediaArtifactError::Payload(fault.message))
    }

    pub(super) async fn retain_media_export_result(&mut self,handle:semio_framework_plugin_decl_media::ArtifactMediaExportHandleLoan<'_>,source:&mut Option<ArtifactMediaExportPoll>,status:&mut Option<semio_framework_plugin_decl_media::ArtifactMediaExportPublishedStatus>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let operation_id=handle.operation_id.0;let identity=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?);
        if handle.parent_document_id.len()>semio_framework_plugin_decl_media::ARTIFACT_OUTPUT_METADATA_BYTES{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"download handle document metadata exceeds its original limit").into())}
        self.media.snapshot_retirements.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving lost its original source"))?.validate(cx)?;
        if self.media.media_download_preparation.is_none()&&status.is_some()&&source.is_none(){return if self.media.segmented_downloads.get(operation_id).is_some_and(|output|output.owns_media_export_loan(handle)){Ok(true)}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"published download lost its exact registry recipient").into())}}
        if self.media.media_download_preparation.is_none(){
            let active=self.media.media_closures.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving lost its original completed operation"))?;
            if active.handle.original_loan()!=handle{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving requires its exact completed business handle").into())}
            if let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref(){if !result.chunks.same_operation(&active.output_chunks){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving requires its original supplied chunks").into())}}
        }
        let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled(){return Ok(false)}
        if self.media.media_download_preparation.is_none(){
            if status.is_some()||!self.media.segmented_downloads.can_insert(operation_id){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving slot is occupied").into())}
            let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref()else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving requires its retained completion").into())};
            let bytes=size_of::<semio_framework_plugin_document_media::media_export::MediaDownloadPreparation>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
            *self.media.media_download_preparation=Some(semio_framework_plugin_document_media::media_export::MediaDownloadPreparation{operation_id,source_identity:identity,document_identity:(handle.parent_document_id.as_ptr()as usize,handle.parent_document_id.len()),mime_identity:(result.mime_type.as_ptr()as usize,result.mime_type.len()),chunks_identity:result.chunks.original_identity(),coordinates:(handle.app_instance_id,handle.base_revision,handle.generation),buffers:ManuallyDrop::new(Some(std::array::from_fn(|_|Vec::new()))),buffers_close:ManuallyDrop::new(None),stage:0,handle:ManuallyDrop::new(None),handle_close:ManuallyDrop::new(None),output:ManuallyDrop::new(None),output_close:ManuallyDrop::new(None),published:false});
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        let original=self.media.media_download_preparation.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download publication lost its retained preparation"))?;if original.operation_id!=operation_id||original.source_identity!=identity||original.coordinates!=(handle.app_instance_id,handle.base_revision,handle.generation)||original.document_identity!=(handle.parent_document_id.as_ptr()as usize,handle.parent_document_id.len()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download publication requires its unchanged original source and handle").into())}
        if let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref(){if original.chunks_identity!=result.chunks.original_identity(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving lost its same actual result root").into())}}
        if original.published{
            if source.is_some()||status.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"published download has inconsistent original receiving slots").into())}
            let bytes=size_of::<semio_framework_plugin_document_media::media_export::MediaDownloadPreparation>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}self.media.media_download_preparation.take();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(true)
        }
        if original.stage<2{
            let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref()else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"partial download metadata lost its retained result").into())};if original.mime_identity!=(result.mime_type.as_ptr()as usize,result.mime_type.len()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"partial download requires its unchanged original MIME source").into())}
            let input=if original.stage==0{handle.parent_document_id.as_bytes()}else{result.mime_type.as_bytes()};let output=&mut original.buffers.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download metadata lost its copy buffers"))?[original.stage];
            if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
            if output.capacity()==0&&!input.is_empty(){if grant.maximum_capacity_bytes<input.len(){return Ok(false)}output.try_reserve_exact(input.len()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"download metadata allocation refused"))?;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:output.capacity(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
            if output.len()<input.len(){let count=(input.len()-output.len()).min(grant.maximum_copy_bytes);if count==0{return Ok(false)}let start=output.len();output.extend_from_slice(&input[start..start+count]);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
            if grant.maximum_copy_bytes<size_of::<usize>(){return Ok(false)}original.stage+=1;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<usize>(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        if original.handle.is_none()&&status.is_none(){
            let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref()else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download metadata donation lost its original result").into())};let total_bytes=result.chunks.bytes()as u64;let bytes=size_of::<ArtifactMediaExportHandle>()+size_of::<semio_framework_plugin_decl_media::ArtifactMediaExportPublishedStatus>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
            let buffers=original.buffers.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download metadata lost its copy buffers"))?;let document=String::from_utf8(std::mem::take(&mut buffers[0])).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"copied original document lost its UTF-8 encoding"))?;let mime_type=String::from_utf8(std::mem::take(&mut buffers[1])).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"copied original MIME lost its UTF-8 encoding"))?;*original.handle=Some(ArtifactMediaExportHandle{app_instance_id:handle.app_instance_id,parent_document_id:document,operation_id:handle.operation_id,base_revision:handle.base_revision,generation:handle.generation});*status=Some(semio_framework_plugin_decl_media::ArtifactMediaExportPublishedStatus{mime_type,total_bytes});cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        if original.output.is_none(){let _=ArtifactDownloadOutput::receive_media_export_original(&mut original.handle,source,&mut original.output,cx)?;return Ok(false)}
        let bytes=size_of::<ArtifactDownloadOutput>()+2*size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}if !self.media.segmented_downloads.can_insert(operation_id){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download registry receiving slot changed before publication").into())}
        self.media.segmented_downloads.insert_admitted(operation_id,original.output.take().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download publication lost its donated output"))?);original.published=true;if let Some(active)=self.media.media_closures.get_mut(operation_id){active.download_pending=false}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(false)
    }
}

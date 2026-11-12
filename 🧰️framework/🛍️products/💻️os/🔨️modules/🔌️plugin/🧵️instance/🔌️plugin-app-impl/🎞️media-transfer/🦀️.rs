//! 🎞️ Media transfer of the mounted artifact instance: the production and consumption of a media artifact and the retained export result. See [`super`].

use super::*;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    /// 📤️ ABI-level media delivery for ONE port — the route `AppCommand::MediaOut` reaches
    /// through `plugin_produce_media`.
    ///
    /// 🎞️ Produces one document or declared media output while preserving the initiating pending turn.
    pub(super) async fn produce_media(&mut self, port: &str, owner: &mut semio_os_kernel::io::control::NativeSnapshotEncodeOwner<'_,'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<MediaArtifact>,semio_plugin_decl_app::MediaArtifactOperationFailure> {
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
            let mut produced: Option<Result<Option<MediaArtifact>,semio_plugin_decl_app::MediaArtifactOperationFailure>> = None;
            owner.receive_nested::<Media, Result<Option<MediaArtifact>,semio_plugin_decl_app::MediaArtifactOperationFailure>, semio_os_kernel::io::control::NativeSnapshotFailure>(&mut produced, &mut |slot, result, _marker, _unwind, original| semio_os_kernel::io::control::native_snapshot_turn(|work| { work.performed(); (|| -> Result<Result<Option<MediaArtifact>,semio_plugin_decl_app::MediaArtifactOperationFailure>, ValueError> {
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
            })().map(|value| { *result = Some(value); }).map_err(Into::into) })).result.map_err(|failure| ValueError::new(failure.kind(), failure.message().to_owned()).with_retained_progress(failure.retained_progress()))?;
            let exported = produced.ok_or_else(|| ValueError::invariant("media export produced no result"))??;
            return Ok(exported);
        }
        let archive = PluginApp::document_archive(self).await?;
        whole_document_media(port, self.artifact_schema().await.to_string(), &archive).await.map(Some).map_err(Into::into)
    }

    pub(super) async fn consume_media(&mut self,port:&str,artifact:&mut Option<MediaArtifact>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<MediaConsumption>,semio_plugin_decl_app::MediaArtifactOperationFailure>{
        let wire=artifact.as_ref().ok_or_else(||MediaArtifactError::Payload("media artifact was already consumed".into()))?.descriptor.wire.clone();
        if port!=NATURAL_FILE_PORT{
            let schema_now=self.artifact_schema().await.to_string();
            return match wire{
                MediaWireFormat::Document{schema} if schema==schema_now=>{let data=artifact.take().map(|consumed|consumed.data).unwrap_or_default();Ok(Some(MediaConsumption::DocumentLoad(whole_document_archive(&data).await?)))}
                MediaWireFormat::Document{schema}=>Err(MediaArtifactError::SchemaMismatch{expected:schema_now,found:schema}.into()),
                MediaWireFormat::Binary{format_kind}=>Err(MediaArtifactError::NoImporter(format_kind).into()),
                MediaWireFormat::Intrinsic{..}=>{
                    let mut media=None;
                    if self.render_ctx().reserved_media_due(cx){
                        let mut accept=|_|true;
                        let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES,&mut accept);
                        media=Some(artifact.as_ref().ok_or_else(||MediaArtifactError::Payload("media artifact was already consumed".into()))?.decode_intrinsic(&mut control)?);
                    }
                    Ok(self.import_artifact_media(port,&mut media,cx).await?)
                }
            };
        }
        let codec=A::natural_file_codec().await.ok_or_else(||MediaArtifactError::NoImporter(port.to_string()))?;
        let MediaWireFormat::Binary{format_kind}=wire else{return Err(MediaArtifactError::NoImporter(codec.format_kind.to_string()).into())};
        if format_kind!=codec.format_kind{return Err(MediaArtifactError::NoImporter(format_kind).into())}
        let mut media=None;
        if self.render_ctx().reserved_media_due(cx){
            let media_type=A::io().await.map_or(MediaType{class:MediaClass::Data,form:MediaForm::Value},|io|io.artifact_media_type);
            let data=artifact.as_mut().map(|source|std::mem::take(&mut source.data)).unwrap_or_default();
            media=Some(natural_file_input_media(codec,media_type,data));
        }
        let consumption=self.import_artifact_media(port,&mut media,cx).await?;
        Ok(consumption)
    }

    /// 🧲️ Imports one decoded artifact media through the stepped reserved producer under the instance's mounted identity.
    async fn import_artifact_media(&mut self,port:&str,media:&mut Option<Media>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<MediaConsumption>,semio_framework_job::OriginalTurnFailure>{
        let instance_id=self.tools.live_runtime_instance_id.ok_or_else(||plugin_sdk_fault("media import requires a bound live app instance"))?;
        let actor=self.stores.store.local_actor_id().0.clone();
        Ok(self.dispatch_ctx().dispatch_import_media(port,media,&ActionMeta{expected_document_revision:None,actor,instance_id,view_state:None},cx).await?.map(|_|MediaConsumption::Applied))
    }
}

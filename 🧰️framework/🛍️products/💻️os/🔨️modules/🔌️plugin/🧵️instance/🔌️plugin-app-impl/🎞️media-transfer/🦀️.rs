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
            let exported = owner.receive_nested::<Media, Result<Option<MediaArtifact>,semio_plugin_decl_app::MediaArtifactOperationFailure>, semio_os_kernel::io::control::NativeSnapshotFailure>(|slot, original| semio_os_kernel::io::control::native_snapshot_turn(|work| { work.performed(); (|| -> Result<Result<Option<MediaArtifact>,semio_plugin_decl_app::MediaArtifactOperationFailure>, ValueError> {
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
            })().map_err(Into::into) })).result.map_err(semio_os_kernel::io::control::NativeSnapshotFailure::into_value_error)??;
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
                MediaWireFormat::Intrinsic { .. } => Err(MediaArtifactError::Payload("intrinsic media import is driven by the original media import owner, never by a one-shot consume".into())),
            };
        }
        let codec = A::natural_file_codec().await.ok_or_else(|| MediaArtifactError::NoImporter(port.to_string()))?;
        let MediaWireFormat::Binary { format_kind } = artifact.descriptor.wire else {
            return Err(MediaArtifactError::NoImporter(codec.format_kind.to_string()));
        };
        if format_kind != codec.format_kind {
            return Err(MediaArtifactError::NoImporter(format_kind));
        }
        Err(MediaArtifactError::Payload("natural file import is driven by the original media import owner, never by a one-shot consume".into()))
    }
}

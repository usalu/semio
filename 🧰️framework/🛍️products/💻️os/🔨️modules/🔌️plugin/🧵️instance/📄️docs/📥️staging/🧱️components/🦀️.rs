//! 🩻️ The instance components of `instance-docs`: MediaTransfers, EnvelopeJobs (WP6, plan section 7.3).
use semio_framework_plugin_document_media::{ActiveArtifactEnvelopeDecode, ActiveArtifactEnvelopeIngress, ActiveArtifactStoreReplacement, ActiveDocumentArchiveLoad};
use semio_framework_plugin_document_media::media_export::ActiveMediaExport;
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_views::media_contract::ArtifactDownloadOutput;
use semio_framework_plugin_ownership::bounded_ownership::{ArtifactFixedRegistry, DOCUMENT_ARCHIVE_LOAD_SLOTS};
use semio_framework_plugin_ownership::store_lifecycle::{ArtifactOwnedDisposer, ArtifactSnapshotCloseRetention};
use pack::snapshot::ArtifactPack;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_plugin_job::reserved_jobs::FrameworkReservedCommitPermit;
use std::future::Future;
use std::mem::ManuallyDrop;
use crate::docs_instance_root::MediaExportPreparation;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_child::SnapshotReadReturnPump;
use semio_framework_ui_contract::*;

/// 🩺️ Media transfers: the media exports, downloads, closures and snapshot retirements of the instance.
pub struct MediaTransfers<A: ArtifactApp> {
    pub pending_media_import_permit:ManuallyDrop<Option<(FrameworkReservedCommitPermit,(semio_framework_job::OperationId,semio_framework_job::Generation,usize))>>,
    pub media_exports: ArtifactFixedRegistry<ActiveMediaExport>,
    pub media_export_preparation:ManuallyDrop<Option<MediaExportPreparation<A>>>,
    pub media_download_preparation:ManuallyDrop<Option<semio_framework_plugin_document_media::media_export::MediaDownloadPreparation>>,
    pub media_closures: ArtifactFixedRegistry<ActiveMediaExport>,
    pub snapshot_retirements: ArtifactFixedRegistry<ArtifactSnapshotCloseRetention>,
    pub current_media_export: Option<u64>,
    pub media_submission_refusal: std::mem::ManuallyDrop<Option<semio_framework_plugin_document_media::media_operation_refusal::MediaOperationRefusal>>,
    pub media_submission_refusal_identity: Option<(semio_framework_job::OperationId,semio_framework_job::Generation)>,
    pub media_submission_refusal_retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_plugin_document_media::media_operation_refusal::MediaOperationRefusal>>>,
    pub segmented_downloads: ArtifactFixedRegistry<ArtifactDownloadOutput>,
    pub segmented_closures: ArtifactFixedRegistry<semio_framework_plugin_document_media::media_export::MediaDownloadRetirement>,
}

impl<A: ArtifactApp> MediaTransfers<A> {
    /// 💉️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {
        Self {
            pending_media_import_permit: ManuallyDrop::new(None),
            media_exports: ArtifactFixedRegistry::new(),
            media_export_preparation: ManuallyDrop::new(None),
            media_download_preparation: ManuallyDrop::new(None),
            media_closures: ArtifactFixedRegistry::new(),
            snapshot_retirements: ArtifactFixedRegistry::new(),
            current_media_export: None,
            media_submission_refusal: std::mem::ManuallyDrop::new(None),
            media_submission_refusal_identity: None,
            media_submission_refusal_retirement: std::mem::ManuallyDrop::new(None),
            segmented_downloads: ArtifactFixedRegistry::new(),
            segmented_closures: ArtifactFixedRegistry::new(),
        }
    }
}

/// 🦠️ Envelope jobs: envelope ingress and decode, store replacement and document archive loads.
pub struct EnvelopeJobs<A: ArtifactApp, M: SpaceMember + MemberFactory> {
    pub envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>,
    pub envelope_decode_jobs: ArtifactFixedRegistry<ActiveArtifactEnvelopeDecode<A::Snapshot, A::Mutation>>,
    pub store_replacement_jobs: ArtifactFixedRegistry<ActiveArtifactStoreReplacement<A::Snapshot, A::Mutation, M>>,
    /// 📥️ This instance's open file-import runs: every chunked pick is reassembled here, so an app's import
    /// action only ever decodes a whole file (`semio_framework::kernel::ImportStaging`).
    pub import_staging: semio_framework::kernel::ImportStaging,
    pub document_archive_loads: ArtifactFixedRegistry<ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>>,
    pub envelope_field_decoders: std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoderRegistry<A::Snapshot, A::Mutation>>,
    pub envelope_field_decoder_retirements: ArtifactFixedRegistry<store::os_store::owned_schema_decode::ArtifactEnvelopeReturnedFieldDecoder<A::Snapshot, A::Mutation>>,
    pub envelope_completed_records: std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry<A::Snapshot, A::Mutation>>,
    pub envelope_completed_record_retirements: ArtifactFixedRegistry<Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecord<A::Snapshot, A::Mutation>>>,
    pub document_snapshot_read_returns: SnapshotReadReturnPump,
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> EnvelopeJobs<A, M> {
    /// 🪸️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {
        Self {
            envelope_ingress: ArtifactFixedRegistry::new(),
            envelope_decode_jobs: ArtifactFixedRegistry::new(),
            store_replacement_jobs: ArtifactFixedRegistry::new(),
            import_staging: semio_framework::kernel::ImportStaging::default(),
            document_archive_loads: ArtifactFixedRegistry::with_slots(DOCUMENT_ARCHIVE_LOAD_SLOTS),
            envelope_field_decoders: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoderRegistry::new(),
            envelope_field_decoder_retirements: ArtifactFixedRegistry::new(),
            envelope_completed_records: store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry::new(),
            envelope_completed_record_retirements: ArtifactFixedRegistry::new(),
            document_snapshot_read_returns: SnapshotReadReturnPump::new(),
        }
    }
}

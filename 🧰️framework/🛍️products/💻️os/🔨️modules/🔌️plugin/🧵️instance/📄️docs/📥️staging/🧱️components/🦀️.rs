//! 🩻️ The instance components of `instance-docs`: MediaTransfers, EnvelopeJobs (WP6, plan section 7.3).
use semio_plugin_document_media::{ActiveArtifactEnvelopeDecode, ActiveArtifactEnvelopeIngress, ActiveArtifactStoreReplacement, ActiveDocumentArchiveLoad};
use semio_plugin_document_media::media_export::ActiveMediaExport;
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_decl_media::ArtifactDownloadOutput;
use semio_plugin_ownership::bounded_ownership::{ArtifactFixedRegistry, DOCUMENT_ARCHIVE_LOAD_SLOTS};
use semio_plugin_store_lifecycle::store_lifecycle::{ArtifactOwnedDisposer, ArtifactSnapshotCloseRetention};
use pack::snapshot::ArtifactPack;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_plugin_job::reserved_jobs::FrameworkReservedCommitPermit;
use std::future::Future;
use std::mem::ManuallyDrop;
use crate::docs_instance_root::MediaExportPreparation;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_child::SnapshotReadReturnPump;
use semio_ui_contract::*;

/// 🩺️ Media transfers: the media exports, downloads, closures and snapshot retirements of the instance.
pub struct MediaTransfers<A: ArtifactApp> {
    pub original_picker: Option<Box<semio_plugin_instance_natural_publication::picker::PickerPublication>>,
    pub natural_source: Option<Box<semio_plugin_instance_natural_publication::NaturalFilePublication>>,
    pub pending_reserved_producer_permit:ManuallyDrop<Option<(FrameworkReservedCommitPermit,(semio_framework_job::OperationId,semio_framework_job::Generation,usize))>>,
    pub reserved_mount:ManuallyDrop<Option<Box<semio_plugin_retained_command::typed_operation::original_emission::OriginalEmissionAdmission<A>>>>,
    pub reserved_mounted_construction:semio_framework::action_bus::ResidentConstruction<Box<semio_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation<A>>>,
    pub reserved_receipt:Option<semio_plugin_job::original_emission::OriginalEmissionReceipt>,
    /// 🏁️ The terminal publication of the operation the reserved mount issued: registered with its operation id when the operation is mounted, filled by the first accepted acknowledgement of its terminal page whoever sends it, and taken exactly once by the custody that waits for it.
    pub reserved_terminal:Option<(u64,Option<Result<(),semio_plugin_decl_app::tool_contract::ArtifactBoundedToolFault>>)>,
    pub reserved_mount_construction:semio_framework_job::OriginalConstructionAdmission,
    pub media_exports: ArtifactFixedRegistry<ActiveMediaExport>,
    pub media_export_preparation:ManuallyDrop<Option<Box<MediaExportPreparation<A>>>>,
    pub media_download_preparation:ManuallyDrop<Option<semio_plugin_document_media::media_export::MediaDownloadPreparation>>,
    pub media_closures: ArtifactFixedRegistry<ActiveMediaExport>,
    pub snapshot_retirements: ArtifactFixedRegistry<ArtifactSnapshotCloseRetention>,
    pub current_media_export: Option<u64>,
    pub media_submission_refusal: std::mem::ManuallyDrop<Option<semio_plugin_document_media::media_operation_refusal::MediaOperationRefusal>>,
    pub media_submission_refusal_place: std::mem::ManuallyDrop<Option<semio_framework_job::OriginalDomainReservation<semio_plugin_document_media::media_operation_refusal::MediaOperationRefusal>>>,
    pub media_submission_refusal_identity: Option<(semio_framework_job::OperationId,semio_framework_job::Generation)>,
    pub media_submission_refusal_retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_plugin_document_media::media_operation_refusal::MediaOperationRefusal>>>,
    pub segmented_downloads: ArtifactFixedRegistry<ArtifactDownloadOutput>,
    pub segmented_closures: ArtifactFixedRegistry<semio_plugin_document_media::media_export::MediaDownloadRetirement>,
}

impl<A: ArtifactApp> MediaTransfers<A> {
    /// 📏️ Quotes this component frame and its fixed original backing; input roots remain separately admitted.
    pub fn birth_demands()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{semio_plugin_ownership::bounded_ownership::original_component_birth::<Self>(&[semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.media_exports,64)?,semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.media_closures,64)?,semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.snapshot_retirements,64)?,semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.segmented_downloads,64)?,semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.segmented_closures,64)?])}

    /// 💉️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {Self::with_backing(true)}
    /// 🪹️ Empty cleanup custody retains no unneeded transfer backing.
    pub fn with_backing(backing:bool)->Self{
        Self {
            pending_reserved_producer_permit: ManuallyDrop::new(None),
            original_picker:None,
            natural_source:None,
            reserved_mount:ManuallyDrop::new(None),
            reserved_mount_construction:Default::default(),
            reserved_mounted_construction:Default::default(),
            reserved_receipt:None,
            reserved_terminal:None,
            media_exports: if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(64)},
            media_export_preparation: ManuallyDrop::new(None),
            media_download_preparation: ManuallyDrop::new(None),
            media_closures: if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(64)},
            snapshot_retirements: if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(64)},
            current_media_export: None,
            media_submission_refusal: std::mem::ManuallyDrop::new(None),
            media_submission_refusal_place: std::mem::ManuallyDrop::new(None),
            media_submission_refusal_identity: None,
            media_submission_refusal_retirement: std::mem::ManuallyDrop::new(None),
            segmented_downloads: if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(64)},
            segmented_closures: if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(64)},
        }
    }

    semio_framework_value::initialization::original_empty_fields!(EMPTY_FIELDS, empty_field_demands, initialize_empty_field_at;
        pending_reserved_producer_permit = ManuallyDrop::new(None),
        reserved_mount = ManuallyDrop::new(None),
        reserved_mount_construction = Default::default(),
        reserved_mounted_construction = Default::default(),
        reserved_receipt = None,
        reserved_terminal = None,
        current_media_export = None,
        media_submission_refusal = ManuallyDrop::new(None),
        media_submission_refusal_place = ManuallyDrop::new(None),
        media_submission_refusal_identity = None,
        media_submission_refusal_retirement = ManuallyDrop::new(None),
    );

    /// 📋️ Counts absent transfers, fixed registry headers and other resident fields.
    pub const INITIAL_FIELDS:usize=7+Self::EMPTY_FIELDS;

    /// 📐️ Quotes one bounded media field before placing its original fixed backing.
    pub fn initial_field_demands(field:usize,offset:usize,body:usize,backing:bool)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        use semio_framework_value::{initialization::absent_chunk_bytes,RetirementDemand};
        use semio_plugin_ownership::bounded_ownership::original_registry_backing;
        fn extent<S,T>(_:fn(&S)->&T)->usize{size_of::<T>()}
        if field<2{
            let bytes=if field==0{extent::<Self,_>(|owner|&owner.media_export_preparation)}else{extent::<Self,_>(|owner|&owner.media_download_preparation)};
            return Ok(RetirementDemand{copy_bytes:absent_chunk_bytes(bytes,offset,body.max(1))?,depth:1,..Default::default()});
        }
        macro_rules! quote_registry{
            ($key:ident)=>{RetirementDemand{copy_bytes:2*extent::<Self,_>(|owner|&owner.$key),capacity_bytes:if backing{original_registry_backing::<Self,_>(|owner|&owner.$key,64)?}else{0},depth:1,..Default::default()}}
        }
        Ok(match field{
            2=>quote_registry!(media_exports),
            3=>quote_registry!(media_closures),
            4=>quote_registry!(snapshot_retirements),
            5=>quote_registry!(segmented_downloads),
            6=>quote_registry!(segmented_closures),
            _=>return Self::empty_field_demands(field-7),
        })
    }

    /// 🌱️ Places a selected field directly in the caller's original media storage.
    /// # Safety
    /// The selected field must be exclusively owned, uninitialized storage; offsets must advance monotonically.
    pub unsafe fn initialize_field_at(target:*mut Self,field:usize,offset:usize,count:usize,backing:bool)->Result<bool,semio_framework_value::ValueError>{
        use semio_framework_value::initialization::initialize_absent_field_at;
        use std::ptr::addr_of_mut;
        unsafe{
            match field{
                0=>return initialize_absent_field_at(addr_of_mut!((*target).media_export_preparation).cast::<Option<Box<MediaExportPreparation<A>>>>(),offset,count),
                1=>return initialize_absent_field_at(addr_of_mut!((*target).media_download_preparation).cast::<Option<semio_plugin_document_media::media_export::MediaDownloadPreparation>>(),offset,count),
                7.. => return Self::initialize_empty_field_at(target,field-7,offset,count),
                _=>{},
            }
        }
        if offset!=0||count!=Self::initial_field_demands(field,0,count,backing)?.copy_bytes{
            return Err(semio_framework_value::ValueError::invariant("media registry requires its exact copy extent"));
        }
        macro_rules! place_registry{
            ($key:ident)=>{unsafe{addr_of_mut!((*target).$key).write(if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(64)})}}
        }
        match field{
            2=>place_registry!(media_exports),
            3=>place_registry!(media_closures),
            4=>place_registry!(snapshot_retirements),
            5=>place_registry!(segmented_downloads),
            6=>place_registry!(segmented_closures),
            _=>unreachable!(),
        }
        Ok(true)
    }

    pub async fn take_media_export_chunk(&mut self,handle:semio_plugin_decl_media::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<Vec<u8>>,terminal:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        cx.original_source_parent()?;if receiver.is_some(){return Ok(true)}let operation_id=handle.operation_id.0;
        if *terminal&&self.segmented_downloads.get(operation_id).is_none(){return Ok(true)}
        self.snapshot_retirements.get(operation_id).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download chunk receiving lost its original source"))?.validate(cx)?;
        let output=self.segmented_downloads.get(operation_id).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download chunk receiving has no exact output"))?;
        if !output.owns_media_export_loan(handle){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download chunk receiving requires its exact handle").into())}
        if !*terminal{if !output.chunks.take_chunk_original(receiver,terminal,cx)?{return Ok(false)}return Ok(receiver.is_some())}
        if !output.terminal_is_empty(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download EOF retains unread original chunks").into())}
        let grant=cx.retained_grant();let bytes=size_of::<semio_plugin_decl_media::ArtifactDownloadOutput>()+size_of::<semio_plugin_document_media::media_export::MediaDownloadRetirement>();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        if !self.segmented_closures.can_insert(operation_id){return Ok(false)}let output=self.segmented_downloads.remove(operation_id).expect("checked exact original download");self.segmented_closures.insert_admitted(operation_id,semio_plugin_document_media::media_export::MediaDownloadRetirement::pending(output));cx.consume_retained(semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }

    pub async fn take_segmented_download_chunk(&mut self, operation_id: u64) -> Result<Option<Vec<u8>>, semio_framework::Fault> {
        let output = self.segmented_downloads.get(operation_id).ok_or_else(|| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("interactive-job.unknown-segmented-download"), format!("unknown segmented download operation {operation_id}")))?;
        let chunk = output.chunks.take_chunk()?;
        if chunk.is_none() {
            let output =
                self.segmented_downloads.remove(operation_id).ok_or_else(|| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("interactive-job.segmented-download-authority"), "terminal segmented download changed before exact removal"))?;
            if !output.terminal_is_empty() {
                self.segmented_downloads.insert_admitted(operation_id, output);
                return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("interactive-job.segmented-download-terminal-not-empty"), "terminal segmented download retained unread chunks"));
            }
            drop(output);
        }
        Ok(chunk)
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
    pub envelope_field_decoders: store::os_store::owned_schema_decode::ArtifactEnvelopeRegistryOwner<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoderRegistry<A::Snapshot, A::Mutation>>,
    pub envelope_field_decoder_retirements: ArtifactFixedRegistry<store::os_store::owned_schema_decode::ArtifactEnvelopeReturnedFieldDecoder<A::Snapshot, A::Mutation>>,
    pub envelope_completed_records: store::os_store::owned_schema_decode::ArtifactEnvelopeRegistryOwner<store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry<A::Snapshot, A::Mutation>>,
    pub envelope_completed_record_retirements: ArtifactFixedRegistry<Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecord<A::Snapshot, A::Mutation>>>,
    pub document_snapshot_read_returns: SnapshotReadReturnPump,
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> EnvelopeJobs<A, M> {
    /// 📏️ Quotes the component frame and fixed registries while preserving the separately admitted decoder issuers.
    pub fn birth_demands()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{semio_plugin_ownership::bounded_ownership::original_component_birth::<Self>(&[semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.envelope_ingress,64)?,semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.envelope_decode_jobs,64)?,semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.store_replacement_jobs,64)?,semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.envelope_field_decoder_retirements,64)?,semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.envelope_completed_record_retirements,64)?,semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.document_archive_loads,DOCUMENT_ARCHIVE_LOAD_SLOTS)?])}

    /// 🪸️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {Self::from_original(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoderRegistry::new(),store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry::new())}

    /// 🧷️ Retains the actual admitted decoder and completion issuers during native component construction.
    pub fn from_original(field_decoders:std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoderRegistry<A::Snapshot,A::Mutation>>,completed_records:std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry<A::Snapshot,A::Mutation>>)->Self{Self::from_original_with_backing(field_decoders,completed_records,true)}
    /// 🪹️ Cleanup keeps the original decode issuers without unused ingress backing.
    pub fn from_original_with_backing(field_decoders:std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoderRegistry<A::Snapshot,A::Mutation>>,completed_records:std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry<A::Snapshot,A::Mutation>>,backing:bool)->Self{
        Self {
            envelope_ingress: if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(64)},
            envelope_decode_jobs: if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(64)},
            store_replacement_jobs: if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(64)},
            import_staging: semio_framework::kernel::ImportStaging::default(),
            document_archive_loads: if backing{ArtifactFixedRegistry::with_slots(DOCUMENT_ARCHIVE_LOAD_SLOTS)}else{ArtifactFixedRegistry::unallocated(DOCUMENT_ARCHIVE_LOAD_SLOTS)},
            envelope_field_decoders: store::os_store::owned_schema_decode::ArtifactEnvelopeRegistryOwner::from_original(field_decoders),
            envelope_field_decoder_retirements: if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(64)},
            envelope_completed_records: store::os_store::owned_schema_decode::ArtifactEnvelopeRegistryOwner::from_original(completed_records),
            envelope_completed_record_retirements: if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(64)},
            document_snapshot_read_returns: SnapshotReadReturnPump::new(),
        }
    }

    pub fn acknowledge_document_archive_load(&mut self, operation: u64) -> Result<(), semio_framework::Fault> {
        let active = self.document_archive_loads.get(operation).ok_or_else(|| semio_plugin_decl_app::document_load_fault(semio_plugin_decl_app::DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "unknown recursive document archive operation"))?;
        if !active.terminal() {
            return Err(semio_plugin_decl_app::document_load_fault(semio_plugin_decl_app::DOCUMENT_LOAD_FAILED_CODE, "recursive document archive operation cannot be acknowledged before terminal status"));
        }
        if !active.inputs_are_empty() {
            return Err(semio_plugin_decl_app::document_load_fault(semio_plugin_decl_app::DOCUMENT_LOAD_FAILED_CODE, "terminal recursive document archive operation still owns retained input"));
        }
        let mut active = self.document_archive_loads.remove(operation).ok_or_else(|| semio_plugin_decl_app::document_load_fault(semio_plugin_decl_app::DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "terminal recursive document archive authority changed before acknowledgement"))?;
        drop(std::mem::take(&mut active.fault));
        drop(active);
        Ok(())
    }

    /// 📊️ The status of one load, counting the records its awaited store replacement decoded so far: the replacement's own steps are what run through the long phase after the fold, and the load itself only observes them between its turns.
    pub fn document_archive_status(&self, operation: u64) -> Option<protocol::DocumentArchiveLoadStatus> {
        let load = self.document_archive_loads.get(operation)?;
        let live = load.replacement.and_then(|handle| self.store_replacement_jobs.get(handle.operation.0).filter(|job| job.operation == handle.operation && job.generation == handle.generation)).map_or(0, |job| job.decoded);
        Some(load.status_with_decoded(live))
    }

    /// 🛑️ Cancels one running whole-document load together with the store replacement it awaits, in this one step, so no
    /// maintenance unit can commit the candidate between the request and the load's own next turn. `false` when the load is
    /// not running or its replacement already committed: the document is then the new one and the load ends ready.
    pub fn request_document_archive_cancel(&mut self, operation: u64) -> bool {
        let Some(active) = self.document_archive_loads.get_mut(operation) else { return false };
        if !matches!(active.state, semio_plugin_document_media::ActiveDocumentArchiveLoadState::Pending | semio_plugin_document_media::ActiveDocumentArchiveLoadState::Running) {
            return false;
        }
        if let Some(handle) = active.replacement {
            let replacement = self.store_replacement_jobs.get(handle.operation.0).filter(|job| job.operation == handle.operation && job.generation == handle.generation);
            if replacement.is_some_and(|job| job.committed) {
                return false;
            }
            if let Some(job) = replacement {
                job.request_cancel();
            }
        }
        active.request_terminal(semio_plugin_document_media::ActiveDocumentArchiveLoadState::Cancelled);
        true
    }

    pub fn cancel_document_archive_load(&mut self, operation: u64) -> Result<(), semio_framework::Fault> {
        let state = self.document_archive_loads.get(operation).ok_or_else(|| semio_plugin_decl_app::document_load_fault(semio_plugin_decl_app::DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "unknown recursive document archive operation"))?.state;
        match state {
            semio_plugin_document_media::ActiveDocumentArchiveLoadState::Pending | semio_plugin_document_media::ActiveDocumentArchiveLoadState::Running => {
                if self.request_document_archive_cancel(operation) { Ok(()) } else { Err(semio_plugin_decl_app::document_load_fault(semio_plugin_decl_app::DOCUMENT_LOAD_FAILED_CODE, "recursive document archive replacement already committed and cannot be cancelled")) }
            }
            semio_plugin_document_media::ActiveDocumentArchiveLoadState::Cancelled => Ok(()),
            semio_plugin_document_media::ActiveDocumentArchiveLoadState::Ready | semio_plugin_document_media::ActiveDocumentArchiveLoadState::Fault => Err(semio_plugin_decl_app::document_load_fault(semio_plugin_decl_app::DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "terminal recursive document archive operation cannot be cancelled")),
        }
    }
}

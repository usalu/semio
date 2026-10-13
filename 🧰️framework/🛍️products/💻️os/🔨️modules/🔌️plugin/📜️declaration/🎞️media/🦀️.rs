//! 🎞️ The media export contract an app producer writes into: output chunks, results, completions, credits and handles.
use semio_framework::Fault;
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use semio_framework::Media;
use semio_framework::MediaError;
use semio_framework::MediaPayload;
use semio_framework::MediaType;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use crate::app_contract::ArtifactApp;
use crate::bounded_ownership::ArtifactFixedQueue;
use crate::history_view::HistoryView;
use crate::sdk_fault::plugin_sdk_fault;
use crate::store_lifecycle::ArtifactSnapshotCloseLease;

/// 📤️ Per-chunk byte cap of the segmented-download lane. Mirror of the ONE schema-owned chunk
/// contract — `🎭️actor/📮️shard-client/📤️segmented-download/🧬️schema/🔣️.json` +
/// `🧫️fixtures/🔣️.json`'s `contract.chunkBytes`, mirrored on the host side by
/// `SEGMENTED_DOWNLOAD_CONTRACT` and interpolated into the generated shard worker as
/// `SEGMENTED_DOWNLOAD_CHUNK_BYTES` — held equal to that fixture by this crate's own law
/// [`segmented_download_constants_mirror_the_schema_owned_contract`], so a literal edited in one
/// hop alone fails closed instead of turning a correct producer into a runtime fault.
pub(crate) const ARTIFACT_OUTPUT_CHUNK_BYTES: usize = 4_096;

/// 📤️ Largest payload ONE segmented download may carry, end to end. Same contract, same law:
/// `contract.maximumTotalBytes`, which the host drain enforces as its own default total cap.
/// A producer that declares more than this is refused at construction
/// ([`ArtifactOutputChunks::admit_maximum`]) rather than mid-drain, so an over-cap export becomes
/// a notice in the app that asked for it and never a shard-worker fault.
pub(crate) const ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES: usize = 33_554_432;

const ARTIFACT_OUTPUT_METADATA_BYTES: usize = 4_096;

const ARTIFACT_SEGMENTED_HANDLE_ENCODING: &str = "semio-segmented-handle-v1";

#[derive(Debug)]
struct ArtifactOutputChunksState {
    chunks: ArtifactFixedQueue<Vec<u8>>,
}

struct ArtifactOutputChunksInner {
    state: semio_framework_value::retirement::controlled::RetainedOwnerGate<ArtifactOutputChunksState>,
    bytes: std::sync::atomic::AtomicUsize,
    bytes_remaining: std::sync::atomic::AtomicUsize,
    remaining: std::sync::atomic::AtomicUsize,
    slots: usize,
    maximum: usize,
    sealed: std::sync::atomic::AtomicBool,
}

/// 🧵️ Operation-owned segmented output. Every producer move is bounded to one 4 KiB chunk;
/// terminal completion transfers the queue without flattening, rescanning, or copying it.
#[derive(Clone)]
pub struct ArtifactOutputChunks {
    inner: std::sync::Arc<ArtifactOutputChunksInner>,
}

impl semio_framework_value::retirement::RetireOwned for ArtifactOutputChunksInner{
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{
        self.state.into_inner().chunks.items.retirement()
    }
    fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.state.try_lock().ok()?.chunks.items)}
    fn controlled_retirement_supported()->bool{<std::collections::VecDeque<Vec<u8>> as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
}

impl semio_framework_value::retirement::RetireOwned for ArtifactOutputChunks{
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(semio_framework_value::retirement::shared::SharedControlledRetirement::lease(self.inner))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<semio_framework_value::retirement::shared::SharedControlledRetirement<ArtifactOutputChunksInner>>())}
    fn controlled_retirement_supported()->bool{<ArtifactOutputChunksInner as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
}

impl std::fmt::Debug for ArtifactOutputChunks {
    fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{formatter.debug_struct("ArtifactOutputChunks").field("bytes",&self.bytes()).field("remaining",&self.chunks_remaining()).field("maximum",&self.inner.maximum).finish_non_exhaustive()}
}

impl ArtifactOutputChunks {
    /// 📏️ The exact byte cap [`Self::push`] enforces on ONE chunk — the wire constant an app must slice its
    /// payload by. Exported because every app that fills a segmented output re-declared it as its own
    /// literal, and a literal that drifts from `push`'s check turns a correct producer into a
    /// `segmented-output-limit` fault at runtime.
    pub const CHUNK_BYTES: usize = ARTIFACT_OUTPUT_CHUNK_BYTES;

    /// 📏️ Largest payload ONE segmented download may carry — the same
    /// `contract.maximumTotalBytes` the host drain defaults its total cap to. An app must refuse a
    /// payload above this with a NOTICE of its own; publishing it would be refused mid-drain, where
    /// the user sees a dead download instead of an answer.
    pub const MAXIMUM_TOTAL_BYTES: usize = ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES;

    /// 📏️ The declared output budget one segmented download may be built with, or the bound it
    /// violated. Checked HERE, at construction, so an over-cap producer is refused before it opens a
    /// download handle the host would then drain into a fault.
    pub fn admit_maximum(maximum: usize) -> Result<usize, Fault> {
        if maximum == 0 || maximum > ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-total-over-cap"), "segmented download declares an empty or over-cap total byte budget"));
        }
        Ok(maximum)
    }

    pub fn new(maximum: usize) -> Self {
        let slots = maximum / ARTIFACT_OUTPUT_CHUNK_BYTES + usize::from(!maximum.is_multiple_of(ARTIFACT_OUTPUT_CHUNK_BYTES));
        Self {
            inner: std::sync::Arc::new(ArtifactOutputChunksInner {
                state: semio_framework_value::retirement::controlled::RetainedOwnerGate::new(ArtifactOutputChunksState { chunks: ArtifactFixedQueue::new(slots) }),
                bytes: std::sync::atomic::AtomicUsize::new(0),
                bytes_remaining: std::sync::atomic::AtomicUsize::new(0),
                remaining: std::sync::atomic::AtomicUsize::new(0),
                slots,
                maximum,
                sealed: std::sync::atomic::AtomicBool::new(false),
            }),
        }
    }

    pub fn push(&self, chunk: Vec<u8>) -> Result<usize, Fault> {
        if self.inner.sealed.load(std::sync::atomic::Ordering::Acquire) || chunk.is_empty() || chunk.len() > ARTIFACT_OUTPUT_CHUNK_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-output-limit"), "segmented output is sealed, empty, oversized, or exceeds its exact byte cap"));
        }
        let mut state = self.inner.state.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-output-busy"), "segmented output slot authority is busy or poisoned"))?;
        let bytes = self.inner.bytes.load(std::sync::atomic::Ordering::Acquire);
        let next = bytes.checked_add(chunk.len()).filter(|next| *next <= self.inner.maximum);
        if self.inner.sealed.load(std::sync::atomic::Ordering::Acquire) || state.chunks.len() >= self.inner.slots || next.is_none() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-output-limit"), "segmented output is sealed, empty, oversized, has no pre-admitted slot, or exceeds its exact byte cap"));
        }
        state.chunks.push(chunk).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-output-capacity"), "segmented output fixed slot allocation was not admitted"))?;
        self.inner.remaining.store(state.chunks.len(), std::sync::atomic::Ordering::Release);
        let next = next.expect("validated segmented byte total");
        self.inner.bytes.store(next, std::sync::atomic::Ordering::Release);
        self.inner.bytes_remaining.store(next, std::sync::atomic::Ordering::Release);
        Ok(next)
    }

    pub fn seal(&self) -> Result<usize, Fault> {
        let _state = self.inner.state.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-output-busy"), "segmented output slot authority is busy or poisoned"))?;
        if self.inner.sealed.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-output-duplicate-seal"), "segmented output was sealed more than once"));
        }
        Ok(self.inner.bytes.load(std::sync::atomic::Ordering::Acquire))
    }

    pub fn bytes(&self) -> usize {
        self.inner.bytes.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn chunks_remaining(&self) -> usize {
        self.inner.remaining.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn bytes_remaining(&self) -> usize {
        self.inner.bytes_remaining.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn slot_capacity(&self) -> usize {
        self.inner.slots
    }

    pub fn take_chunk(&self) -> Result<Option<Vec<u8>>, Fault> {
        if !self.inner.sealed.load(std::sync::atomic::Ordering::Acquire) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-output-unsealed"), "segmented output cannot be consumed before it is sealed"));
        }
        let mut state = self.inner.state.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-output-busy"), "segmented output slot authority is busy or poisoned"))?;
        let chunk = state.chunks.pop();
        self.inner.remaining.store(state.chunks.len(), std::sync::atomic::Ordering::Release);
        if let Some(chunk) = chunk.as_ref() {
            let remaining = self.inner.bytes_remaining.load(std::sync::atomic::Ordering::Acquire);
            self.inner.bytes_remaining.store(remaining.checked_sub(chunk.len()).expect("queued output byte census"), std::sync::atomic::Ordering::Release);
        }
        Ok(chunk)
    }

    pub fn close_take_chunk(&self) -> Result<Option<Vec<u8>>, Fault> {
        let mut state = self.inner.state.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-output-busy"), "segmented output disposal authority is busy or poisoned"))?;
        let chunk = state.chunks.pop();
        self.inner.remaining.store(state.chunks.len(), std::sync::atomic::Ordering::Release);
        if let Some(chunk) = chunk.as_ref() {
            let remaining = self.inner.bytes_remaining.load(std::sync::atomic::Ordering::Acquire);
            self.inner.bytes_remaining.store(remaining.checked_sub(chunk.len()).expect("queued output byte census"), std::sync::atomic::Ordering::Release);
        }
        Ok(chunk)
    }

    fn is_sealed(&self) -> bool {
        self.inner.sealed.load(std::sync::atomic::Ordering::Acquire)
    }

    pub(crate) fn same_operation(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.inner, &other.inner)
    }
}

/// 🎞️ Segmented structured media returned by an interactive export without a terminal String.
#[derive(Clone, Debug)]
pub struct ArtifactMediaExportResult {
    pub media_type: MediaType,
    pub schema: String,
    pub mime_type: String,
    pub chunks: ArtifactOutputChunks,
}

impl ArtifactMediaExportResult {
    pub fn structured(media_type: MediaType, schema: impl Into<String>, mime_type: impl Into<String>, chunks: ArtifactOutputChunks) -> Result<Self, Fault> {
        let schema = schema.into();
        let mime_type = mime_type.into();
        if schema.len() > ARTIFACT_OUTPUT_METADATA_BYTES || mime_type.is_empty() || mime_type.len() > 128 || !chunks.is_sealed() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-media-metadata"), "segmented media schema or MIME type is invalid, or its chunks are unsealed"));
        }
        Ok(Self { media_type, schema, mime_type, chunks })
    }

    pub(crate) fn into_batch_media(self) -> Result<Media, MediaError> {
        let mut bytes = Vec::with_capacity(self.chunks.bytes());
        while let Some(chunk) = self.chunks.take_chunk().map_err(|error| MediaError::Payload(self.schema.clone(), error.message))? {
            bytes.extend_from_slice(&chunk);
        }
        let json = String::from_utf8(bytes).map_err(|error| MediaError::Payload(self.schema.clone(), error.to_string()))?;
        Ok(Media { media_type: self.media_type, payload: MediaPayload::Structured { schema: self.schema, json } })
    }
}

/// ⬇️ Segmented action output retained by the live app until the host takes every chunk.
#[derive(semio_framework_value::RetireOwned)]
pub struct ArtifactDownloadOutput {
    pub filename: String,
    pub mime_type: String,
    pub encoding: Option<String>,
    pub chunks: ArtifactOutputChunks,
    media_export_handle: Option<ArtifactMediaExportHandle>,
}

impl ArtifactDownloadOutput {
    pub fn new(filename: impl Into<String>, mime_type: impl Into<String>, encoding: Option<String>, chunks: ArtifactOutputChunks) -> Result<Self, Fault> {
        let filename = filename.into();
        let mime_type = mime_type.into();
        let encoding_bytes = encoding.as_ref().map_or(0, String::len);
        let marker_bytes = ARTIFACT_SEGMENTED_HANDLE_ENCODING.len().checked_add(1).and_then(|bytes| bytes.checked_add(encoding_bytes.max("identity".len())));
        if filename.len() > ARTIFACT_OUTPUT_METADATA_BYTES || mime_type.len() > ARTIFACT_OUTPUT_METADATA_BYTES || marker_bytes.is_none_or(|bytes| bytes > ARTIFACT_OUTPUT_METADATA_BYTES) || !chunks.is_sealed() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-metadata"), "segmented download metadata is oversized or its chunks are unsealed"));
        }
        Ok(Self { filename, mime_type, encoding, chunks, media_export_handle: None })
    }

    pub(crate) fn from_media_export(handle: ArtifactMediaExportHandle, mime_type: impl Into<String>, chunks: ArtifactOutputChunks) -> Result<Self, Fault> {
        let mut output = Self::new(String::new(), mime_type, None, chunks)?;
        output.media_export_handle = Some(handle);
        Ok(output)
    }

    pub(crate) fn owns_media_export(&self, handle: &ArtifactMediaExportHandle) -> bool {
        self.media_export_handle.as_ref() == Some(handle)
    }

    /// 🧪️ Test-only door to the media-export constructor the SDK keeps private.
    #[cfg(test)]
    fn test_from_media_export(handle: ArtifactMediaExportHandle, mime_type: impl Into<String>, chunks: ArtifactOutputChunks) -> Result<Self, Fault> {
        Self::from_media_export(handle, mime_type, chunks)
    }

    #[cfg(test)]
    fn handle_encoding(&self) -> String {
        format!("{ARTIFACT_SEGMENTED_HANDLE_ENCODING}:{}", self.encoding.as_deref().unwrap_or("identity"))
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.chunks.chunks_remaining() == 0 && self.chunks.bytes_remaining() == 0
    }
}

/// 📬️ Single-assignment result cell for one owned media-export operation.
#[derive(Clone)]
pub struct ArtifactMediaExportCompletion {
    inner: std::sync::Arc<std::sync::Mutex<ArtifactMediaExportCompletionSlot>>,
}

#[derive(Default)]
struct ArtifactMediaExportCompletionSlot {
    value: Option<Result<ArtifactMediaExportResult, MediaError>>,
    chunks: Option<ArtifactOutputChunks>,
    strings: [Option<String>; 2],
    active: Option<Box<dyn store::ErasedSnapshotRetirement>>,
}

impl ArtifactMediaExportCompletionSlot {
    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.chunks.is_none() && self.strings.iter().all(Option::is_none) && self.active.is_none()
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(active, body);
        }
        if self.value.is_some() {
            return Ok(RetirementDemand { copy_bytes: size_of::<Result<ArtifactMediaExportResult, MediaError>>(), depth: 1, ..Default::default() });
        }
        if self.chunks.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.chunks);
        }
        Ok(self.strings.iter().flatten().next().map_or(RetirementDemand::default(), |value| RetirementDemand { release_bytes: value.capacity(), depth: 1, ..Default::default() }))
    }
}

impl ArtifactMediaExportCompletion {
    pub(crate) fn new() -> Self {
        Self { inner: std::sync::Arc::new(std::sync::Mutex::new(ArtifactMediaExportCompletionSlot::default())) }
    }

    pub fn complete(&self, result: Result<ArtifactMediaExportResult, MediaError>) -> Result<(), Fault> {
        let error_bytes = match &result {
            Ok(_) | Err(MediaError::NotImplemented) => 0,
            Err(MediaError::UnknownPort(port)) => port.len(),
            Err(MediaError::Incompatible { port, .. }) => port.len(),
            Err(MediaError::Payload(port, message)) => port.len().saturating_add(message.len()),
        };
        if error_bytes > ARTIFACT_OUTPUT_METADATA_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-completion-metadata"), "media completion error metadata exceeds its exact byte cap"));
        }
        let mut slot = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if slot.value.is_some() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.duplicate-media-output"), "media export job attempted to complete more than once"));
        }
        slot.value = Some(result);
        Ok(())
    }

    pub(crate) fn take(&self) -> Result<Option<Result<ArtifactMediaExportResult, MediaError>>, Fault> {
        self.inner.try_lock().map(|mut slot| slot.value.take()).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-completion-busy"), "media completion authority is busy or poisoned"))
    }

    fn busy() -> Fault {
        Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-completion-busy"), "media completion disposal authority is busy or poisoned")
    }

    /// 📏️ Quotes the next completion close turn: handing the result apart, birthing the chunk retirement frame, stepping it, or releasing one metadata string.
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.inner.try_lock().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "media completion disposal authority is busy or poisoned"))?.retirement_demands(body)
    }

    /// 🧹 One granted close turn; `Complete` only when the result, its chunk retirement and every metadata string are gone.
    fn close_step(&self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let mut slot = self.inner.try_lock().map_err(|_| Self::busy())?;
        if slot.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let demand = slot.retirement_demands(grant.maximum_copy_bytes).map_err(|error| Fault::from(error.into_message()))?;
        if grant.maximum_depth < demand.depth {
            return Err(plugin_sdk_fault("media completion close exceeds its admitted depth"));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let slot = &mut *slot;
        let progress = if slot.active.is_some() {
            store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut slot.active, grant).map_err(|error| Fault::from(error.into_message()))?.progress()
        } else if let Some(value) = slot.value.take() {
            match value {
                Ok(ArtifactMediaExportResult { media_type: _, schema, mime_type, chunks }) => {
                    slot.chunks = Some(chunks);
                    slot.strings = [Some(schema), Some(mime_type)];
                }
                Err(MediaError::UnknownPort(port)) | Err(MediaError::Incompatible { port, .. }) => slot.strings[0] = Some(port),
                Err(MediaError::Payload(port, message)) => slot.strings = [Some(port), Some(message)],
                Err(MediaError::NotImplemented) => {}
            }
            RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }
        } else if slot.chunks.is_some() {
            store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut slot.chunks, &mut slot.active, grant).map_err(|error| Fault::from(error.into_message()))?.progress()
        } else {
            let value = slot.strings.iter_mut().find_map(Option::take);
            let released_bytes = value.as_ref().map_or(0, String::capacity);
            drop(value);
            RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }
        };
        Ok(if slot.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    fn terminal_is_empty(&self) -> Result<bool, Fault> {
        self.inner.try_lock().map(|slot| slot.terminal_is_empty()).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-completion-busy"), "media completion terminal authority is busy or poisoned"))
    }
}

struct ArtifactMediaExportCreditState {
    bytes: std::sync::atomic::AtomicUsize,
    exceeded: std::sync::atomic::AtomicBool,
}

/// 🧮️ Shared O(1) output credit. Producers add each encoded chunk while they already own it;
/// terminal polling reads the accumulated count without rescanning or reallocating the Media.
#[derive(Clone)]
pub struct ArtifactMediaExportCredit {
    maximum: usize,
    state: std::sync::Arc<ArtifactMediaExportCreditState>,
}

impl ArtifactMediaExportCredit {
    pub(crate) fn new(maximum: usize) -> Self {
        Self { maximum, state: std::sync::Arc::new(ArtifactMediaExportCreditState { bytes: std::sync::atomic::AtomicUsize::new(0), exceeded: std::sync::atomic::AtomicBool::new(false) }) }
    }

    pub fn credit(&self, bytes: usize) -> Result<usize, Fault> {
        let result = self.state.bytes.try_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |current| current.checked_add(bytes).filter(|next| *next <= self.maximum)).map(|previous| previous + bytes);
        match result {
            Ok(total) => Ok(total),
            Err(_) => {
                self.state.exceeded.store(true, std::sync::atomic::Ordering::SeqCst);
                Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-output-limit"), format!("media export output exceeds {} bytes", self.maximum)))
            }
        }
    }

    pub fn bytes(&self) -> usize {
        self.state.bytes.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn exceeded(&self) -> bool {
        self.state.exceeded.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub(crate) fn validate_terminal(&self) -> Result<usize, Fault> {
        let bytes = self.bytes();
        if self.exceeded() || bytes > self.maximum {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-output-limit"), format!("media export output exceeds {} bytes", self.maximum)));
        }
        Ok(bytes)
    }
}

/// 🎞️ Exact live-instance and operation authority for one resumable media export.
pub struct ArtifactMediaExportJobRequest<A: ArtifactApp> {
    pub retained:RetainedCloneGrant,
    pub operation: semio_framework_job::Operation,
    pub controller_id: String,
    pub tool_id: String,
    pub payload_schema_id: String,
    pub contract: semio_framework::ToolExecutionContract,
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub canonical_base_revision: [u8; 32],
    pub snapshot: std::sync::Arc<A::Snapshot>,
    pub snapshot_close: ArtifactSnapshotCloseLease<A::Snapshot>,
    pub config: std::sync::Arc<A::Config>,
    pub history: std::sync::Arc<HistoryView>,
    pub port: String,
    pub raw_wire: Vec<u8>,
    pub output_credit: ArtifactMediaExportCredit,
    pub output_chunks: ArtifactOutputChunks,
    pub completion: ArtifactMediaExportCompletion,
}

/// 🎫️ Externally addressable live media-export operation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArtifactMediaExportHandle {
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub operation_id: semio_framework_job::OperationId,
    pub base_revision: semio_framework_job::RevisionId,
    pub generation: semio_framework_job::Generation,
}

impl semio_framework_value::retirement::RetireOwned for ArtifactMediaExportHandle {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.parent_document_id)}
    fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.parent_document_id)}
    fn controlled_retirement_supported()->bool{<String as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
}

/// 📡️ Result of advancing one submitted media export by at most one worker slice.
#[derive(Clone, Debug)]
pub enum ArtifactMediaExportPoll {
    Running { applied_progress: u64, checkpoint_available: bool },
    Complete(ArtifactMediaExportResult),
    Cancelled,
    Failed(String),
}

pub(crate) fn validate_media_export_structure(result: &ArtifactMediaExportResult, operation_chunks: &ArtifactOutputChunks, credited_bytes: usize, maximum: usize) -> Result<(), Fault> {
    let actual_bytes = result.schema.len().checked_add(result.chunks.bytes()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-output-overflow"), "media export structural payload byte count overflowed"))?;
    if !result.chunks.same_operation(operation_chunks) || !result.chunks.is_sealed() || actual_bytes != credited_bytes || actual_bytes > maximum {
        return Err(Fault::new(
            FaultOrigin::Framework,
            FaultCode::new("interactive-job.media-output-structure"),
            format!("media export structural payload is {actual_bytes} bytes but producer sealed {credited_bytes} bytes against a {maximum}-byte cap"),
        ));
    }
    Ok(())
}

#[cfg(test)]
include!("../../🧪️tests/🔬️app-artifact-media-export-credit/🦀️.rs");

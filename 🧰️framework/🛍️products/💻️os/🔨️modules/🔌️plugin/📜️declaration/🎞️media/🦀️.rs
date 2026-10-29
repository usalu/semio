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
use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedQueue;
use semio_framework_plugin_decl_views::history_view::HistoryView;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactSnapshotCloseLease;

/// 📤️ Per-chunk byte cap of the segmented-download lane. Mirror of the ONE schema-owned chunk
/// contract — `🎭️actor/📮️shard-client/📤️segmented-download/🧬️schema/🔣️.json` +
/// `🧫️fixtures/🔣️.json`'s `contract.chunkBytes`, mirrored on the host side by
/// `SEGMENTED_DOWNLOAD_CONTRACT` and interpolated into the generated shard worker as
/// `SEGMENTED_DOWNLOAD_CHUNK_BYTES` — held equal to that fixture by this crate's own law
/// [`segmented_download_constants_mirror_the_schema_owned_contract`], so a literal edited in one
/// hop alone fails closed instead of turning a correct producer into a runtime fault.
pub const ARTIFACT_OUTPUT_CHUNK_BYTES: usize = 4_096;

/// 📤️ Largest payload ONE segmented download may carry, end to end. Same contract, same law:
/// `contract.maximumTotalBytes`, which the host drain enforces as its own default total cap.
/// A producer that declares more than this is refused at construction
/// ([`ArtifactOutputChunks::admit_maximum`]) rather than mid-drain, so an over-cap export becomes
/// a notice in the app that asked for it and never a shard-worker fault.
pub const ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES: usize = 33_554_432;

pub const ARTIFACT_OUTPUT_METADATA_BYTES: usize = 4_096;

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
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(semio_framework_value::retirement::arc_custody::SharedControlledRetirement::lease(self.inner))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<semio_framework_value::retirement::arc_custody::SharedControlledRetirement<ArtifactOutputChunksInner>>())}
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

    pub fn original_birth_admit(maximum:usize,receiving:&mut Option<Self>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let _=cx.original_source_parent()?;if let Some(owner)=receiving.as_ref(){if owner.inner.maximum!=maximum{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original output birth receiver changed its byte bound").into())}return Ok(true)}
        let demand=Self::original_birth_demands(maximum)?;
        let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes{return Ok(false)}
        let(owner,progress)=Self::construct_original(maximum)?;
        *receiving=Some(owner);cx.consume_retained(progress)?;cx.consume_fuel(1);Ok(true)
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

    pub fn take_chunk_original(&self,receiver:&mut Option<Vec<u8>>,terminal:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        cx.original_source_parent()?;
        if receiver.is_some()||*terminal{return Ok(true)}
        if !self.is_sealed(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original media chunk source is not sealed").into())}
        let bytes=std::mem::size_of::<Vec<u8>>()+4*std::mem::size_of::<usize>()+std::mem::size_of::<bool>();let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        let mut state=match self.inner.state.try_lock(){Ok(state)=>state,Err(_)=>return Ok(false)};
        *receiver=state.chunks.pop();*terminal=receiver.is_none();self.inner.remaining.store(state.chunks.len(),std::sync::atomic::Ordering::Release);
        if let Some(chunk)=receiver.as_ref(){let remaining=self.inner.bytes_remaining.load(std::sync::atomic::Ordering::Acquire);self.inner.bytes_remaining.store(remaining.checked_sub(chunk.len()).expect("original queued media extent"),std::sync::atomic::Ordering::Release)}
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
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

    pub fn is_sealed(&self) -> bool {
        self.inner.sealed.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn original_identity(&self)->usize{std::sync::Arc::as_ptr(&self.inner)as usize}

    pub fn same_operation(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.inner, &other.inner)
    }
}

/// 🎞️ Segmented structured media returned by an interactive export without a terminal String.
#[derive(Clone, Debug,semio_framework_value::RetireOwned)]
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

    pub fn into_batch_media(self) -> Result<Media, MediaError> {
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
    pub media_export_handle: Option<ArtifactMediaExportHandle>,
    pub media_export_metadata: Option<(MediaType,String)>,
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
        Ok(Self { filename, mime_type, encoding, chunks, media_export_handle: None,media_export_metadata:None })
    }

    pub fn receive_media_export_original(handle:&mut Option<ArtifactMediaExportHandle>,source:&mut Option<ArtifactMediaExportPoll>,receiver:&mut Option<Self>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        cx.original_source_parent()?;
        if receiver.is_some(){return if handle.is_none()&&source.is_none(){Ok(true)}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"download receiving cannot overwrite another original output").into())}}
        let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"download receiving requires its retained terminal media result").into())};
        if handle.is_none()||result.schema.len()>ARTIFACT_OUTPUT_METADATA_BYTES||result.mime_type.is_empty()||result.mime_type.len()>128||!result.chunks.is_sealed(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"download receiving requires complete original media metadata and sealed chunks").into())}
        let bytes=std::mem::size_of::<Self>()+std::mem::size_of::<ArtifactMediaExportPoll>()+std::mem::size_of::<ArtifactMediaExportHandle>();let grant=cx.retained_grant();
        if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        let Some(ArtifactMediaExportPoll::Complete(result))=source.take()else{unreachable!()};
        *receiver=Some(Self{filename:String::new(),mime_type:result.mime_type,encoding:None,chunks:result.chunks,media_export_handle:handle.take(),media_export_metadata:Some((result.media_type,result.schema))});
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }

    pub fn owns_media_export_loan(&self, handle: ArtifactMediaExportHandleLoan<'_>)->bool{self.media_export_handle.as_ref().is_some_and(|original|original.original_loan()==handle)}

    pub fn owns_media_export(&self, handle: &ArtifactMediaExportHandle) -> bool {
        self.media_export_handle.as_ref() == Some(handle)
    }

    #[doc(hidden)]
    pub fn handle_encoding(&self) -> String {
        format!("{ARTIFACT_SEGMENTED_HANDLE_ENCODING}:{}", self.encoding.as_deref().unwrap_or("identity"))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.chunks.chunks_remaining() == 0 && self.chunks.bytes_remaining() == 0
    }
}

/// 📬️ Single-assignment result cell for one owned media-export operation.
#[derive(Clone)]
pub struct ArtifactMediaExportCompletion {
    inner: std::sync::Arc<ArtifactMediaExportCompletionInner>,
}

#[derive(Default,semio_framework_value::RetireOwned)]
struct ArtifactMediaExportCompletionSlot{value:Option<Result<ArtifactMediaExportResult,MediaError>>}
struct ArtifactMediaExportCompletionInner{state:semio_framework_value::retirement::controlled::RetainedOwnerGate<ArtifactMediaExportCompletionSlot>}
impl semio_framework_value::retirement::RetireOwned for ArtifactMediaExportCompletionInner{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.state.into_inner())}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&*self.state.try_lock().ok()?)}
 fn controlled_retirement_supported()->bool{<ArtifactMediaExportCompletionSlot as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
}
impl semio_framework_value::retirement::RetireOwned for ArtifactMediaExportCompletion{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(semio_framework_value::retirement::arc_custody::SharedControlledRetirement::lease(self.inner))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<semio_framework_value::retirement::arc_custody::SharedControlledRetirement<ArtifactMediaExportCompletionInner>>())}
 fn controlled_retirement_supported()->bool{<ArtifactMediaExportCompletionInner as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
}

impl ArtifactMediaExportCompletion {
    pub fn original_birth_admit(receiving:&mut Option<Self>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let _=cx.original_source_parent()?;if receiving.is_some(){return Ok(true)}let capacity_bytes=semio_framework_value::shared_retirement_allocation_bytes::<ArtifactMediaExportCompletionInner>();let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<size_of::<Self>()||grant.maximum_capacity_bytes<capacity_bytes{return Ok(false)}
        let owner=Self::new();*receiving=Some(owner);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Self>(),retained_capacity_bytes:capacity_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }

    pub fn new() -> Self {
        Self { inner: std::sync::Arc::new(ArtifactMediaExportCompletionInner{state:semio_framework_value::retirement::controlled::RetainedOwnerGate::new(ArtifactMediaExportCompletionSlot::default())}) }
    }

    /// 📬️ Donates one original result only after the actual receiver and caller quote admit it.
    pub fn complete_original(&self,source:&mut Option<Result<ArtifactMediaExportResult,MediaError>>,published:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{ValueError,ValueRefusalKind};let _=cx.original_source_parent()?;
        if *published{if source.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"published completion cannot overwrite another original result").into())}return Ok(true)}
        let Some(result)=source.as_ref()else{return Ok(false)};
        let metadata=match result{Ok(_)|Err(MediaError::NotImplemented)=>0,Err(MediaError::UnknownPort(port))|Err(MediaError::Incompatible{port,..})=>port.len(),Err(MediaError::Payload(port,message))=>port.len().checked_add(message.len()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original completion metadata length overflow"))?};
        if metadata>ARTIFACT_OUTPUT_METADATA_BYTES{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original completion metadata exceeds its exact byte cap").into())}
        let grant=cx.retained_grant();let bytes=size_of::<Result<ArtifactMediaExportResult,MediaError>>()+size_of::<bool>();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        let Ok(mut slot)=self.inner.state.try_lock()else{return Ok(false)};if slot.value.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original completion receiving slot is already occupied").into())}
        slot.value=source.take();*published=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }

    pub fn take(&self) -> Result<Option<Result<ArtifactMediaExportResult, MediaError>>, Fault> {
        self.inner.state.try_lock().map(|mut slot| slot.value.take()).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-completion-busy"), "media completion authority is busy or poisoned"))
    }

}

struct ArtifactMediaExportCreditState {
    writer: semio_framework_value::retirement::controlled::RetainedOwnerGate<()>,
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

impl semio_framework_value::retirement::RetireOwned for ArtifactMediaExportCreditState{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(())}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&())}
 fn controlled_retirement_supported()->bool{<() as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
}
impl semio_framework_value::retirement::RetireOwned for ArtifactMediaExportCredit{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(semio_framework_value::retirement::arc_custody::SharedControlledRetirement::lease(self.state))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<semio_framework_value::retirement::arc_custody::SharedControlledRetirement<ArtifactMediaExportCreditState>>())}
 fn controlled_retirement_supported()->bool{<ArtifactMediaExportCreditState as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
}

impl ArtifactMediaExportCredit {
    pub fn original_birth_admit(maximum:usize,receiving:&mut Option<Self>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let _=cx.original_source_parent()?;if let Some(owner)=receiving.as_ref(){if owner.maximum!=maximum{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original credit birth receiver changed its byte bound").into())}return Ok(true)}let capacity_bytes=semio_framework_value::shared_retirement_allocation_bytes::<ArtifactMediaExportCreditState>();let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<size_of::<Self>()||grant.maximum_capacity_bytes<capacity_bytes{return Ok(false)}
        let owner=Self::new(maximum);*receiving=Some(owner);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Self>(),retained_capacity_bytes:capacity_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }

    pub fn new(maximum: usize) -> Self {
        Self { maximum, state: std::sync::Arc::new(ArtifactMediaExportCreditState { writer: semio_framework_value::retirement::controlled::RetainedOwnerGate::new(()), bytes: std::sync::atomic::AtomicUsize::new(0), exceeded: std::sync::atomic::AtomicBool::new(false) }) }
    }

    /// 🎟️ Charges the original output ceiling without allocating a cold diagnostic on refusal.
    pub fn credit_original(&self,bytes:usize)->Result<usize,ValueError>{
        self.state.bytes.try_update(std::sync::atomic::Ordering::SeqCst,std::sync::atomic::Ordering::SeqCst,|current|current.checked_add(bytes).filter(|next|*next<=self.maximum)).map(|previous|previous+bytes).map_err(|_|{self.state.exceeded.store(true,std::sync::atomic::Ordering::SeqCst);ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original media output exceeds its admitted byte credit")})
    }

    pub fn credit(&self, bytes: usize) -> Result<usize, Fault> {
        let _writer = self.state.writer.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-output-busy"), "media output credit authority is busy"))?;
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

    pub fn validate_terminal(&self) -> Result<usize, Fault> {
        let bytes = self.bytes();
        if self.exceeded() || bytes > self.maximum {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-output-limit"), format!("media export output exceeds {} bytes", self.maximum)));
        }
        Ok(bytes)
    }
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

/// 🎟️ Borrows the initiating decoded command's complete media authority without copying its document identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactMediaExportHandleLoan<'a> {
    pub app_instance_id:u32,
    pub parent_document_id:&'a str,
    pub operation_id:semio_framework_job::OperationId,
    pub base_revision:semio_framework_job::RevisionId,
    pub generation:semio_framework_job::Generation,
}
impl ArtifactMediaExportHandle {
    pub fn receive_original(source:&mut Option<Self>,receiver:&mut Option<Self>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        cx.original_source_parent()?;
        if receiver.is_some(){return if source.is_none(){Ok(true)}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"media handle receiving cannot overwrite a retained owner").into())}}
        if source.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"media handle receiving requires its actual donor").into())}
        let grant=cx.retained_grant();let bytes=std::mem::size_of::<Self>();
        if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        *receiver=source.take();
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }
    pub fn original_loan(&self)->ArtifactMediaExportHandleLoan<'_>{ArtifactMediaExportHandleLoan{app_instance_id:self.app_instance_id,parent_document_id:&self.parent_document_id,operation_id:self.operation_id,base_revision:self.base_revision,generation:self.generation}}
}

impl semio_framework_value::retirement::RetireOwned for ArtifactMediaExportHandle {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.parent_document_id)}
    fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.parent_document_id)}
    fn controlled_retirement_supported()->bool{<String as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
}

/// 📬️ A funded terminal publication owns its exact status metadata independently of the download.
#[derive(semio_framework_value::RetireOwned)]
pub struct ArtifactMediaExportPublishedStatus{pub mime_type:String,pub total_bytes:u64}

/// 📡️ Result of advancing one submitted media export by at most one worker slice.
#[derive(Clone, Debug,semio_framework_value::RetireOwned)]
pub enum ArtifactMediaExportPoll {
    Running { applied_progress: u64, checkpoint_available: bool },
    Complete(ArtifactMediaExportResult),
    Cancelled,
    Failed(String),
}

pub fn validate_media_export_structure(result: &ArtifactMediaExportResult, operation_chunks: &ArtifactOutputChunks, credited_bytes: usize, maximum: usize) -> Result<(), Fault> {
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



#[path="🏭️construction/🦀️.rs"]
mod original_output_construction;

#[path="📤️producer/🦀️.rs"]
mod original_output_producer;

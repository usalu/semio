//! 🎬️ Format-neutral lifecycle for a truly incremental Stdio media serializer.

use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, Operation, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::app::{ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportResult, ArtifactOutputChunks, ArtifactSnapshotCloseLease};
use semio_framework_plugin::{
    ArtifactApp, ArtifactMediaExportJobRequest, ArtifactReservedJob, ArtifactSnapshotDisposer, Fault, MediaClass, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, PluginCloseStep, PortMultiplicity, ToolExecutionContract,
};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress}};
use std::marker::PhantomData;
use std::sync::Arc;

pub const PLAYBACK_PORT_ID: &str = "playback:out";
pub const PLAYBACK_TOOL_ID: &str = "export-media:playback:out";
pub const PLAYBACK_MEDIA_TYPE: MediaType = MediaType { class: MediaClass::Presentation, form: MediaForm::Sequence };
pub const PLAYBACK_CONTRACT: ToolExecutionContract = ToolExecutionContract::resumable(4_096, 4_096, 1, ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES, 2_000, 64, 1);

#[derive(Debug, PartialEq, Eq)]
pub enum IncrementalMediaAdvance {
    Progress,
    Chunk(Vec<u8>),
    Complete,
}

pub trait IncrementalMediaExportSpec: Send + Sync + 'static {
    type Snapshot: Send + Sync + 'static;
    type Cursor: Send + 'static;
    const DOCUMENT_SCHEMA: &'static str;
    const MEDIA_SCHEMA: &'static str;
    const MIME_TYPE: &'static str;
    const PAYLOAD_SCHEMA: &'static str;
    const STAGE: &'static str;
    const KIND_ID: &'static str;
    const ARTIFACT_ID: &'static str;
    const ARTIFACT_NAME: &'static str;
    const COMPONENT_KIND: &'static str;
    fn cursor(snapshot: &Self::Snapshot) -> Result<Self::Cursor, Fault>;
    fn advance(cursor: &mut Self::Cursor, snapshot: &Self::Snapshot, maximum_bytes: usize) -> Result<IncrementalMediaAdvance, Fault>;
}

pub fn playback_app_io<S: IncrementalMediaExportSpec>() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        artifact_schema: S::DOCUMENT_SCHEMA.into(),
        artifact_media_type: PLAYBACK_MEDIA_TYPE,
        ports: vec![MediaPortSpec {
            id: PLAYBACK_PORT_ID.into(),
            label: "Playback".into(),
            direction: MediaPortDirection::Out,
            media_type: PLAYBACK_MEDIA_TYPE,
            kind_id: Some(S::KIND_ID.into()),
            required: false,
            multiplicity: PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: S::ARTIFACT_ID.into(), name: S::ARTIFACT_NAME.into(), dimension: "time".into(), component_kind: S::COMPONENT_KIND.into() },
    }
}

pub struct IncrementalMediaExportJob<S: IncrementalMediaExportSpec> {
    operation: Operation,
    snapshot: Option<Arc<S::Snapshot>>,
    snapshot_close: Option<ArtifactSnapshotCloseLease<S::Snapshot>>,
    cursor: Option<S::Cursor>,
    page: Vec<u8>,
    chunks: Option<ArtifactOutputChunks>,
    output_retirement:Option<Box<dyn ErasedSnapshotRetirement>>,
    credit: Option<ArtifactMediaExportCredit>,
    completion: Option<ArtifactMediaExportCompletion>,
    progress: u64,
    completed: bool,
    closing: bool,
}

impl<S: IncrementalMediaExportSpec> IncrementalMediaExportJob<S> {
    pub fn new<A>(request: ArtifactMediaExportJobRequest<A>) -> Result<Self, Fault>
    where
        A: ArtifactApp<Snapshot = S::Snapshot>,
    {
        let cursor = S::cursor(&request.snapshot)?;
        Ok(Self {
            operation: request.operation,
            snapshot: Some(request.snapshot),
            snapshot_close: Some(request.snapshot_close),
            cursor: Some(cursor),
            page: Vec::with_capacity(ArtifactOutputChunks::CHUNK_BYTES),
            chunks: Some(request.output_chunks),
            output_retirement:None,
            credit: Some(request.output_credit),
            completion: Some(request.completion),
            progress: 0,
            completed: false,
            closing: false,
        })
    }

    fn fault(context: &mut StepContext<'_>, message: &str) -> StepOutcome {
        let bytes = message.as_bytes();
        let bounded = &bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)];
        let detail = context.payload_from_bytes(JobPayloadStream::Fault, bounded).unwrap_or_else(|rejected| {
            drop(rejected.into_source());
            RetainedJobPayload::empty(JobPayloadStream::Fault)
        });
        StepOutcome::Fault(JobFault { detail })
    }

    fn advance(&mut self) -> Result<bool, Fault> {
        let snapshot = self.snapshot.as_deref().ok_or_else(|| Fault::from("media.export.snapshot-missing"))?;
        if self.page.len() == ArtifactOutputChunks::CHUNK_BYTES {
            let page = std::mem::replace(&mut self.page, Vec::with_capacity(ArtifactOutputChunks::CHUNK_BYTES));
            self.credit.as_ref().ok_or_else(|| Fault::from("media.export.credit-missing"))?.credit(page.len())?;
            self.chunks.as_ref().ok_or_else(|| Fault::from("media.export.chunks-missing"))?.push(page)?;
            self.progress = self.progress.checked_add(1).ok_or_else(|| Fault::from("media.export.progress-overflow"))?;
            return Ok(false);
        }
        let maximum_bytes = ArtifactOutputChunks::CHUNK_BYTES - self.page.len();
        let advance = S::advance(self.cursor.as_mut().ok_or_else(|| Fault::from("media.export.cursor-missing"))?, snapshot, maximum_bytes)?;
        self.progress = self.progress.checked_add(1).ok_or_else(|| Fault::from("media.export.progress-overflow"))?;
        match advance {
            IncrementalMediaAdvance::Progress => Ok(false),
            IncrementalMediaAdvance::Chunk(chunk) => {
                if chunk.is_empty() || chunk.len() > maximum_bytes {
                    return Err(Fault::from("media.export.cursor-chunk-invalid"));
                }
                self.page.extend_from_slice(&chunk);
                Ok(false)
            }
            IncrementalMediaAdvance::Complete => {
                if !self.page.is_empty() {
                    let page = std::mem::replace(&mut self.page, Vec::with_capacity(ArtifactOutputChunks::CHUNK_BYTES));
                    self.credit.as_ref().ok_or_else(|| Fault::from("media.export.credit-missing"))?.credit(page.len())?;
                    self.chunks.as_ref().ok_or_else(|| Fault::from("media.export.chunks-missing"))?.push(page)?;
                    return Ok(false);
                }
                let chunks = self.chunks.take().ok_or_else(|| Fault::from("media.export.chunks-missing"))?;
                chunks.seal()?;
                self.credit.as_ref().ok_or_else(|| Fault::from("media.export.credit-missing"))?.credit(S::MEDIA_SCHEMA.len())?;
                let result = ArtifactMediaExportResult::structured(PLAYBACK_MEDIA_TYPE, S::MEDIA_SCHEMA, S::MIME_TYPE, chunks)?;
                self.completion.as_ref().ok_or_else(|| Fault::from("media.export.completion-missing"))?.complete(Ok(result))?;
                self.completed = true;
                Ok(true)
            }
        }
    }
}

impl<S: IncrementalMediaExportSpec> InteractiveJob for IncrementalMediaExportJob<S> {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if self.closing || context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.should_yield() {
            return StepOutcome::Yield;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation || self.completed {
            return Self::fault(context, "media.export.operation-authority-invalid");
        }
        context.set_stage(S::STAGE);
        context.consume_fuel(1);
        match self.advance() {
            Err(error) => Self::fault(context, &format!("{}: {}", error.code.0, error.message)),
            Ok(true) => StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) }),
            Ok(false) => StepOutcome::CheckpointReady(Checkpoint { state: RetainedJobPayload::empty(JobPayloadStream::CheckpointState), applied_progress: self.progress }),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->InteractiveJobCloseStep{
        self.begin_close();let demand=match self.close_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return InteractiveJobCloseStep::Refused(error.kind)};
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return InteractiveJobCloseStep::Pending{progress:Default::default()};}
        if self.page.capacity()!=0{let original=std::mem::take(&mut self.page);let bytes=original.capacity();drop(original);return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}};}
        if self.output_retirement.is_some(){return match semio_framework_plugin::plugin_app_close_prelude::store::artifact_retirement_box_close_step(&mut self.output_retirement,grant){Ok(step)=>InteractiveJobCloseStep::Pending{progress:step.progress()},Err(error)=>InteractiveJobCloseStep::Refused(error.kind)};}
        if self.chunks.is_some(){return match semio_framework_plugin::plugin_app_close_prelude::store::artifact_retirement_admit_owned(&mut self.chunks,&mut self.output_retirement,grant){Ok(step)=>InteractiveJobCloseStep::Pending{progress:step.progress()},Err(error)=>InteractiveJobCloseStep::Refused(error.kind)};}
        InteractiveJobCloseStep::Complete{progress:Default::default()}
    }
    fn terminal_is_empty(&self)->bool{self.cursor.is_none()&&self.page.capacity()==0&&self.chunks.is_none()&&self.output_retirement.is_none()&&self.completion.is_none()&&self.credit.is_none()&&self.snapshot.is_none()&&self.snapshot_close.is_none()}
}
impl<S:IncrementalMediaExportSpec> IncrementalMediaExportJob<S>{
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.page.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.page.capacity(),depth:1,..Default::default()});}
        if let Some(owner)=self.output_retirement.as_ref(){return semio_framework_plugin::plugin_app_close_prelude::store::artifact_retirement_box_demands(owner,body);}
        if self.chunks.is_some(){return semio_framework_plugin::plugin_app_close_prelude::store::artifact_retirement_owned_birth_demands(&self.chunks);}
        if self.cursor.is_some()||self.completion.is_some()||self.credit.is_some()||self.snapshot.is_some()||self.snapshot_close.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original media cursor, completion, credit and raw snapshot require their exact issuing return facets"));}
        Ok(Default::default())
    }
}
impl<S:IncrementalMediaExportSpec> ArtifactReservedJob for IncrementalMediaExportJob<S>{}

pub struct RetireOwnedSnapshotDisposer<T: RetireOwned + Sync> {
    retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    marker: PhantomData<fn() -> T>,
}

impl<T: RetireOwned + Sync> Default for RetireOwnedSnapshotDisposer<T> {
    fn default() -> Self {
        Self { retirement: None, marker: PhantomData }
    }
}

impl<T:RetireOwned+Sync> ArtifactSnapshotDisposer<T> for RetireOwnedSnapshotDisposer<T>{
    fn retirement_demands(&self,snapshot:&Option<Arc<T>>,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.retirement.as_ref(){return semio_framework_plugin::plugin_app_close_prelude::store::artifact_retirement_box_demands(owner,body);}
        if snapshot.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original raw media snapshot has no sealed issuing registry return"));}
        Ok(Default::default())
    }
    fn close_step(&mut self,snapshot:&mut Option<Arc<T>>,grant:RetainedCloneGrant)->Result<semio_framework_plugin::PluginLifecycleStep,Fault>{
        self.retirement_demands(snapshot,grant.maximum_copy_bytes).map_err(|error|Fault::from(error.into_message()))?;
        if self.retirement.is_some(){return semio_framework_plugin::plugin_app_close_prelude::store::artifact_retirement_box_close_step(&mut self.retirement,grant).map(|step|match step{RetainedCloneStep::Progress(progress)=>semio_framework_plugin::PluginLifecycleStep::Progress(progress),RetainedCloneStep::Complete(progress)=>semio_framework_plugin::PluginLifecycleStep::Complete(progress)}).map_err(|error|Fault::from(error.into_message()));}
        Ok(semio_framework_plugin::PluginLifecycleStep::Complete(Default::default()))
    }
    fn terminal_is_empty(&self,snapshot:&Option<Arc<T>>)->bool{snapshot.is_none()&&self.retirement.is_none()}

}

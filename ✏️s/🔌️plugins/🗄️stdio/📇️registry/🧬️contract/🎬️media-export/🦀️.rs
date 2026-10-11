//! 🎬️ Format-neutral lifecycle for a truly incremental Stdio media serializer.

use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, Operation, RetainedFaultPublication, StepContext};
use semio_framework_plugin::app::{ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportResult, ArtifactOutputChunks, ArtifactSnapshotCloseLease};
use semio_framework_plugin::{
    ArtifactApp, ArtifactMediaExportJobRequest, ArtifactReservedJob, ArtifactSnapshotDisposer, Fault, MediaClass, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, PortMultiplicity, ToolExecutionContract,
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
    fault: Option<Fault>,
    fault_publication: RetainedFaultPublication,
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
            fault: None,
            fault_publication: RetainedFaultPublication::new(),
            credit: Some(request.output_credit),
            completion: Some(request.completion),
            progress: 0,
            completed: false,
            closing: false,
        })
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
    fn step<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if self.closing || context.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(context);
        }
        if context.should_yield() {
            return Ok(None);
        }
        if let Some(fault) = self.fault.as_ref() {
            return self.fault_publication.advance_from_fault(fault, context);
        }
        if self.completed {
            return JobOutcomeBorrow::admit_complete(context, None, None);
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            self.fault = Some(Fault::from("media.export.operation-authority-invalid"));
            return Ok(None);
        }
        let grant = context.retained_grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(None);
        }
        context.set_stage(S::STAGE);
        context.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        context.consume_fuel(1);
        if let Err(error) = self.advance() {
            self.fault = Some(error);
        }
        Ok(None)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete if self.completed && self.fault.is_none() => descriptor.complete(None, None),
            JobOutcomeKind::Fault if self.fault.is_some() => self.fault_publication.borrow_outcome(descriptor),
            _ => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "media export outcome descriptor differs from its original job state")),
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
        self.begin_close();
        match self.close_turn(grant){
            Ok(progress)=>if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress}}else{InteractiveJobCloseStep::Pending{progress}},
            Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()},
        }
    }
    fn terminal_is_empty(&self)->bool{self.cursor.is_none()&&self.page.capacity()==0&&self.chunks.is_none()&&self.output_retirement.is_none()&&self.fault.is_none()&&self.fault_publication.terminal_is_empty()&&self.completion.is_none()&&self.credit.is_none()&&self.snapshot.is_none()&&self.snapshot_close.is_none()}
}
impl<S:IncrementalMediaExportSpec> IncrementalMediaExportJob<S>{
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let item=RetirementDemand{depth:1,..Default::default()};
        if let Some(owner)=self.output_retirement.as_ref(){return semio_framework_plugin::plugin_app_close_prelude::store::artifact_retirement_box_demands(owner,body);}
        if !self.fault_publication.terminal_is_empty(){return self.fault_publication.retirement_demands();}
        if self.fault.is_some(){return semio_framework_plugin::plugin_app_close_prelude::store::artifact_retirement_owned_birth_demands(&self.fault);}
        if self.cursor.is_some(){return Ok(item);}
        if self.page.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.page.capacity(),depth:1,..Default::default()});}
        if self.chunks.is_some(){return semio_framework_plugin::plugin_app_close_prelude::store::artifact_retirement_owned_birth_demands(&self.chunks);}
        if self.completion.is_some()||self.credit.is_some()||self.snapshot.is_some()||self.snapshot_close.is_some(){return Ok(item);}
        Ok(Default::default())
    }

    fn close_turn(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        use semio_framework_plugin::plugin_app_close_prelude::store::{artifact_retirement_admit_owned,artifact_retirement_box_close_step};
        let demand=self.close_demands(grant.maximum_copy_bytes)?;
        if demand==RetirementDemand::default(){return Ok(Default::default());}
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"media export close exceeds its admitted depth"));}
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(Default::default());}
        let item=RetainedCloneProgress{copied_items:1,..Default::default()};
        if self.output_retirement.is_some(){return artifact_retirement_box_close_step(&mut self.output_retirement,grant).map(|step|step.progress());}
        if !self.fault_publication.terminal_is_empty(){return self.fault_publication.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|step.progress());}
        if self.fault.is_some(){return artifact_retirement_admit_owned(&mut self.fault,&mut self.output_retirement,grant).map(|step|step.progress());}
        if self.cursor.take().is_some(){return Ok(item);}
        if self.page.capacity()!=0{let released=std::mem::take(&mut self.page).capacity();return Ok(RetainedCloneProgress{released_bytes:released,..item});}
        if self.chunks.is_some(){return artifact_retirement_admit_owned(&mut self.chunks,&mut self.output_retirement,grant).map(|step|step.progress());}
        if self.completion.take().is_some()||self.credit.take().is_some(){return Ok(item);}
        if let Some(snapshot)=self.snapshot.as_ref(){
            if !self.snapshot_close.as_ref().is_some_and(|lease|lease.can_release(snapshot)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"media export snapshot has no live close witness"));}
            self.snapshot=None;
            return Ok(item);
        }
        self.snapshot_close=None;
        Ok(item)
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
        let Some(owner)=snapshot.as_ref()else{return Ok(Default::default())};
        if Arc::strong_count(owner)!=1{return Ok(RetirementDemand{depth:1,..Default::default()});}
        if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"raw media snapshot has no controlled owner authority"));}
        Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(),depth:2,..Default::default()})
    }
    fn close_step(&mut self,snapshot:&mut Option<Arc<T>>,grant:RetainedCloneGrant)->Result<semio_framework_plugin::PluginLifecycleStep,Fault>{
        use semio_framework_plugin::{plugin_app_close_prelude::store::{artifact_retirement_admit_owned,artifact_retirement_box_close_step},PluginLifecycleStep};
        let fault=|error:ValueError|Fault::from(error.into_message());
        let demand=self.retirement_demands(snapshot,grant.maximum_copy_bytes).map_err(fault)?;
        if demand==RetirementDemand::default(){return Ok(PluginLifecycleStep::Complete(Default::default()));}
        if grant.maximum_depth<demand.depth{return Err(Fault::from("media.export.snapshot-depth-limit"));}
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        if self.retirement.is_some(){return artifact_retirement_box_close_step(&mut self.retirement,grant).map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(fault);}
        let shared=snapshot.as_ref().is_some_and(|owner|Arc::strong_count(owner)!=1);
        if shared{snapshot.take();return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        let mut pending=snapshot.take().and_then(Arc::into_inner);
        let step=artifact_retirement_admit_owned(&mut pending,&mut self.retirement,grant);
        if let Some(original)=pending{*snapshot=Some(Arc::new(original));}
        step.map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(fault)
    }
    fn terminal_is_empty(&self,snapshot:&Option<Arc<T>>)->bool{snapshot.is_none()&&self.retirement.is_none()}

}

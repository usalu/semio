//! 🎬️ Format-neutral lifecycle for a truly incremental Stdio media serializer.

use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, Operation, StepContext};
use semio_framework_job_fault_publication::RetainedFaultPublication;
use semio_framework_plugin::app::{ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportResult, ArtifactOutputChunks, ArtifactSnapshotCloseLease};
use semio_framework_plugin::{
    ArtifactApp, ArtifactMediaExportJobRequest, ArtifactReservedJob, Fault, MediaClass, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, PortMultiplicity, ToolExecutionContract,
};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
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
    type Cursor: RetireOwned + Send + 'static;
    const DOCUMENT_SCHEMA: &'static str;
    const MEDIA_SCHEMA: &'static str;
    const MIME_TYPE: &'static str;
    const PAYLOAD_SCHEMA: &'static str;
    const STAGE: &'static str;
    const KIND_ID: &'static str;
    const ARTIFACT_ID: &'static str;
    const ARTIFACT_NAME: &'static str;
    const COMPONENT_KIND: &'static str;
    fn cursor(snapshot: &Self::Snapshot) -> Self::Cursor;
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
    source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),
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
    pending_result:std::mem::ManuallyDrop<Option<Result<ArtifactMediaExportResult,semio_framework::MediaError>>>,result_published:bool,
    completed: bool,
    closing: bool,
    request_metadata: std::mem::ManuallyDrop<Option<(String,String,String)>>,
    metadata_retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<(String,String,String)>>>,
}

impl<S: IncrementalMediaExportSpec> IncrementalMediaExportJob<S> {
    pub fn admit_original<A:ArtifactApp<Snapshot=S::Snapshot>>(request:&mut Option<ArtifactMediaExportJobRequest<A>>,body:&mut Option<Box<dyn ArtifactReservedJob+Send>>,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        cx.original_source_parent()?;if cx.should_yield()||cx.is_cancelled(){return Ok(false)}let grant=cx.retained_grant();
        if body.is_none(){
            let Some(original)=request.as_ref()else{return Ok(false)};let bytes=size_of::<Self>();
            if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes||grant.maximum_capacity_bytes<bytes{return Ok(false)}
            let layout=std::alloc::Layout::new::<Self>();let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<Self>()).ok_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,"Stdio media concrete body allocation refused"))?;
            let owner=Self{operation:original.operation,source_identity:original.source_identity,snapshot:None,snapshot_close:None,cursor:None,page:Vec::new(),chunks:None,output_retirement:None,fault:None,fault_publication:RetainedFaultPublication::new(),credit:None,completion:None,progress:0,pending_result:std::mem::ManuallyDrop::new(None),result_published:false,completed:false,closing:false,request_metadata:std::mem::ManuallyDrop::new(None),metadata_retirement:std::mem::ManuallyDrop::new(None)};
            *body=Some(unsafe{pointer.as_ptr().write(owner);Box::from_raw(pointer.as_ptr())});cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        let original:&mut dyn std::any::Any=&mut **body.as_mut().unwrap();let original=original.downcast_mut::<Self>().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Stdio media admission received another concrete body"))?;
        if original.snapshot.is_some(){if request.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Stdio media producer cannot overwrite its admitted source").into())}return Ok(true)}
        if original.page.capacity()==0{let bytes=ArtifactOutputChunks::CHUNK_BYTES;if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes{return Ok(false)}original.page.try_reserve_exact(bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"Stdio media page allocation refused"))?;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:original.page.capacity(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
        if original.cursor.is_none(){let bytes=size_of::<S::Cursor>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}let input=request.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Stdio cursor lost its retained snapshot source"))?;original.cursor=Some(S::cursor(&input.snapshot));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
        let bytes=size_of::<ArtifactMediaExportJobRequest<A>>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}let input=request.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Stdio producer lost its original receiving request"))?;
        original.snapshot=Some(input.snapshot);original.snapshot_close=Some(input.snapshot_close);original.chunks=Some(input.output_chunks);original.credit=Some(input.output_credit);original.completion=Some(input.completion);*original.request_metadata=Some((input.tool_id,input.parent_document_id,input.port));
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(false)
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
                *self.pending_result=Some(Ok(result));
                Ok(false)
            }
        }
    }
}

impl<S: IncrementalMediaExportSpec> InteractiveJob for IncrementalMediaExportJob<S> {
    fn step<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if self.closing || context.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(context).map_err(Into::into);
        }
        if context.should_yield() {
            return Ok(None);
        }
        if let Some(fault) = self.fault.as_ref() {
            return self.fault_publication.advance_from_fault(fault, context).map_err(Into::into);
        }
        if self.completed {
            return JobOutcomeBorrow::admit_complete(context, None, None).map_err(Into::into);
        }
        if (context.operation(),context.generation()) != self.source_identity {
            self.fault = Some(Fault::from("media.export.operation-authority-invalid"));
            return Ok(None);
        }
        if self.pending_result.is_some()||self.result_published{
            self.completed=self.completion.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Stdio media lost its original completion receiver"))?.complete_original(&mut self.pending_result,&mut self.result_published,context)?;return Ok(None)
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
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        let demand=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{self.begin_close();let progress=self.close_turn(grant)?;Ok(semio_framework_value::RetainedCloneStep::Progress(progress))})?;
        Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }
    fn terminal_is_empty(&self)->bool{self.cursor.is_none()&&self.page.capacity()==0&&self.chunks.is_none()&&self.output_retirement.is_none()&&self.fault.is_none()&&self.fault_publication.terminal_is_empty()&&self.completion.is_none()&&self.credit.is_none()&&self.snapshot.is_none()&&self.snapshot_close.is_none()&&self.request_metadata.is_none()&&self.metadata_retirement.is_none()&&self.pending_result.is_none()}
}
impl<S:IncrementalMediaExportSpec> IncrementalMediaExportJob<S>{
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let item=RetirementDemand{depth:1,..Default::default()};
        if let Some(owner)=self.output_retirement.as_ref(){return semio_framework_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_box_demands(owner,body);}
        if !self.fault_publication.terminal_is_empty(){return self.fault_publication.retirement_demands();}
        if self.fault.is_some(){return semio_framework_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.fault);}
        if self.cursor.is_some(){return semio_framework_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.cursor);}
        if self.page.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.page.capacity(),depth:1,..Default::default()});}
        if self.chunks.is_some(){return semio_framework_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.chunks);}
        if self.pending_result.is_some(){return semio_framework_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_result)}
        if self.completion.is_some(){return semio_framework_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion);}
        if self.credit.is_some(){return semio_framework_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.credit);}
        if self.snapshot.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Arc<S::Snapshot>>(),..item});}
        if let Some(lease)=self.snapshot_close.as_ref(){return Ok(if lease.terminal_is_empty(){item}else{lease.retirement_demands()});}
        if let Some(owner)=self.metadata_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(item)}else{owner.selected_demand(body)};}
        if self.request_metadata.is_some(){return Ok(item);}
        Ok(Default::default())
    }

    fn close_turn(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        use semio_framework_plugin::plugin_app_close_prelude::store::{os_store::snapshot_read::artifact_retirement_admit_owned,os_store::snapshot_read::artifact_retirement_box_close_step};
        let demand=self.close_demands(grant.maximum_copy_bytes)?;
        if demand==RetirementDemand::default(){return Ok(Default::default());}
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"media export close exceeds its admitted depth"));}
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(Default::default());}
        let item=RetainedCloneProgress{copied_items:1,..Default::default()};
        if self.output_retirement.is_some(){return artifact_retirement_box_close_step(&mut self.output_retirement,grant).map(|step|step.progress());}
        if !self.fault_publication.terminal_is_empty(){return self.fault_publication.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|step.progress());}
        if self.fault.is_some(){return artifact_retirement_admit_owned(&mut self.fault,&mut self.output_retirement,grant).map(|step|step.progress());}
        if self.cursor.is_some(){return artifact_retirement_admit_owned(&mut self.cursor,&mut self.output_retirement,grant).map(|step|step.progress());}
        if self.page.capacity()!=0{let released=std::mem::take(&mut self.page).capacity();return Ok(RetainedCloneProgress{released_bytes:released,..item});}
        if self.chunks.is_some(){return artifact_retirement_admit_owned(&mut self.chunks,&mut self.output_retirement,grant).map(|step|step.progress());}
        if self.pending_result.is_some(){return artifact_retirement_admit_owned(&mut self.pending_result,&mut self.output_retirement,grant).map(|step|step.progress())}
        if self.completion.is_some(){return artifact_retirement_admit_owned(&mut self.completion,&mut self.output_retirement,grant).map(|step|step.progress());}
        if self.credit.is_some(){return artifact_retirement_admit_owned(&mut self.credit,&mut self.output_retirement,grant).map(|step|step.progress());}
        if let Some(snapshot)=self.snapshot.as_ref(){
            if !self.snapshot_close.as_ref().is_some_and(|lease|lease.can_release(snapshot)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"media export snapshot has no live close witness"));}
            self.snapshot=None;
            return Ok(RetainedCloneProgress{copied_bytes:demand.copy_bytes,..item});
        }
        if let Some(lease)=self.snapshot_close.as_mut(){if lease.terminal_is_empty(){self.snapshot_close=None;return Ok(item)}return lease.close_step(grant).map(|step|step.progress());}
        if let Some(owner)=self.metadata_retirement.as_mut(){if owner.terminal_is_empty(){self.metadata_retirement.take();return Ok(item)}return owner.step(grant).map(|step|step.progress());}
        if let Some(metadata)=self.request_metadata.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new(metadata){Ok(owner)=>*self.metadata_retirement=Some(owner),Err((error,metadata))=>{*self.request_metadata=Some(metadata);return Err(error)}}return Ok(item);}
        Ok(item)
    }
}
impl<S:IncrementalMediaExportSpec> ArtifactReservedJob for IncrementalMediaExportJob<S>{}

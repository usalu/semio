//! 🎬️ Format-neutral lifecycle for a truly incremental Stdio media serializer.

use semio_framework_job::{OriginalConstructionDemand, InteractiveJobStep, OriginalJobReturnClaim, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, Operation, StepContext};
use semio_framework_job_fault_publication::RetainedFaultPublication;
use semio_plugin::app::{ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportResult, ArtifactOutputChunks, ArtifactSnapshotCloseLease};
use semio_plugin::{
    MediaConstruction, ArtifactApp, ArtifactMediaExportJobRequest, ArtifactReservedJob, Fault, MediaClass, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, PortMultiplicity, ToolExecutionContract,
};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::{RetainedWorkOutcome,ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
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

pub fn playback_app_io<S: IncrementalMediaExportSpec>() -> semio_plugin::AppIo {
    semio_plugin::AppIo {
        artifact_schema: S::DOCUMENT_SCHEMA.into(),
        artifact_media_type: PLAYBACK_MEDIA_TYPE,
        ports: vec![MediaPortSpec {
            id: PLAYBACK_PORT_ID.into(),
            label: semio_plugin::LocalizedLabel::native("Playback", "Wiedergabe"),
            direction: MediaPortDirection::Out,
            media_type: PLAYBACK_MEDIA_TYPE,
            kind_id: Some(S::KIND_ID.into()),
            required: false,
            multiplicity: PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_plugin::ArtifactPresentation { id: S::ARTIFACT_ID.into(), name: S::ARTIFACT_NAME.into(), dimension: "time".into(), component_kind: S::COMPONENT_KIND.into() },
    }
}

pub struct IncrementalMediaExportJob<S: IncrementalMediaExportSpec> {
    operation: Operation,
    source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),
    snapshot: Option<Arc<S::Snapshot>>,
    snapshot_close: Option<ArtifactSnapshotCloseLease<S::Snapshot>>,
    cursor: Option<S::Cursor>,
    page: Option<Vec<u8>>,
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
    pub fn admit_original<A:ArtifactApp<Snapshot=S::Snapshot>>(request:&mut Option<ArtifactMediaExportJobRequest<A>>,body:&mut Option<Box<dyn ArtifactReservedJob+Send>>,construction:&mut MediaConstruction,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        cx.original_source_parent()?;if cx.should_yield()||cx.is_cancelled(){return Ok(false)}let grant=cx.retained_grant();
        if body.is_none(){
            let Some(original)=request.as_ref()else{return Ok(false)};let bytes=size_of::<Self>();let(operation,source_identity)=(original.operation,original.source_identity);
            return construction.built(cx,body,OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:bytes,release_bytes:0,depth:1},||{
                let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(std::alloc::Layout::new::<Self>())}.cast::<Self>()).ok_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,"Stdio media concrete body allocation refused"))?;
                let owner=Self{operation,source_identity,snapshot:None,snapshot_close:None,cursor:None,page:None,chunks:None,output_retirement:None,fault:None,fault_publication:RetainedFaultPublication::new(),credit:None,completion:None,progress:0,pending_result:std::mem::ManuallyDrop::new(None),result_published:false,completed:false,closing:false,request_metadata:std::mem::ManuallyDrop::new(None),metadata_retirement:std::mem::ManuallyDrop::new(None)};
                let boxed:Box<dyn ArtifactReservedJob+Send>=unsafe{pointer.as_ptr().write(owner);Box::from_raw(pointer.as_ptr())};
                Ok((boxed,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,..Default::default()}))
            }).map(|_|false)
        }
        if construction.settling(){construction.settle(cx)?;return Ok(false)}
        let original:&mut dyn std::any::Any=&mut **body.as_mut().unwrap();let original=original.downcast_mut::<Self>().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Stdio media admission received another concrete body"))?;
        if original.snapshot.is_some(){if request.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Stdio media producer cannot overwrite its admitted source").into())}return Ok(true)}
        if original.page.is_none(){let bytes=ArtifactOutputChunks::CHUNK_BYTES;return construction.built(cx,&mut original.page,OriginalConstructionDemand{items:1,copy_bytes:size_of::<Vec<u8>>(),capacity_bytes:bytes,release_bytes:0,depth:1},||{let mut page=Vec::new();page.try_reserve_exact(bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"Stdio media page allocation refused"))?;let capacity=page.capacity();Ok((page,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<u8>>(),retained_capacity_bytes:capacity,..Default::default()}))}).map(|_|false)}
        if original.cursor.is_none(){let bytes=size_of::<S::Cursor>();let input=request.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Stdio cursor lost its retained snapshot source"))?;let snapshot=&input.snapshot;return construction.built(cx,&mut original.cursor,OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:0,release_bytes:0,depth:1},||Ok((S::cursor(snapshot),RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}))).map(|_|false)}
        let bytes=size_of::<ArtifactMediaExportJobRequest<A>>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}let input=request.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Stdio producer lost its original receiving request"))?;
        original.snapshot=Some(input.snapshot);original.snapshot_close=Some(input.snapshot_close);original.chunks=Some(input.output_chunks);original.credit=Some(input.output_credit);original.completion=Some(input.completion);*original.request_metadata=Some((input.tool_id,input.parent_document_id,input.port));
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(false)
    }

    fn advance(&mut self) -> Result<bool, Fault> {
        let snapshot = self.snapshot.as_deref().ok_or_else(|| Fault::from("media.export.snapshot-missing"))?;
        let page_len = self.page.as_ref().map_or(0, Vec::len);
        if page_len == ArtifactOutputChunks::CHUNK_BYTES {
            let page = self.page.replace(Vec::with_capacity(ArtifactOutputChunks::CHUNK_BYTES)).ok_or_else(|| Fault::from("media.export.page-missing"))?;
            self.credit.as_ref().ok_or_else(|| Fault::from("media.export.credit-missing"))?.credit(page.len())?;
            self.chunks.as_ref().ok_or_else(|| Fault::from("media.export.chunks-missing"))?.push(page)?;
            self.progress = self.progress.checked_add(1).ok_or_else(|| Fault::from("media.export.progress-overflow"))?;
            return Ok(false);
        }
        let maximum_bytes = ArtifactOutputChunks::CHUNK_BYTES - page_len;
        let advance = S::advance(self.cursor.as_mut().ok_or_else(|| Fault::from("media.export.cursor-missing"))?, snapshot, maximum_bytes)?;
        self.progress = self.progress.checked_add(1).ok_or_else(|| Fault::from("media.export.progress-overflow"))?;
        match advance {
            IncrementalMediaAdvance::Progress => Ok(false),
            IncrementalMediaAdvance::Chunk(chunk) => {
                if chunk.is_empty() || chunk.len() > maximum_bytes {
                    return Err(Fault::from("media.export.cursor-chunk-invalid"));
                }
                self.page.as_mut().ok_or_else(|| Fault::from("media.export.page-missing"))?.extend_from_slice(&chunk);
                Ok(false)
            }
            IncrementalMediaAdvance::Complete => {
                if page_len != 0 {
                    let page = self.page.replace(Vec::with_capacity(ArtifactOutputChunks::CHUNK_BYTES)).ok_or_else(|| Fault::from("media.export.page-missing"))?;
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
    fn step<'a>(&'a mut self, context: &mut StepContext<'_>, _return_claim: &mut OriginalJobReturnClaim<'_>) -> InteractiveJobStep<'a> {
        InteractiveJobStep::from_callback(self.step_owned(context))
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
        let step=semio_framework_job::close_original_effect_step(cx,demand,|effect|{self.begin_close();self.close_turn(effect)})?;
        Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }
    fn terminal_is_empty(&self)->bool{self.cursor.is_none()&&self.page.is_none()&&self.chunks.is_none()&&self.output_retirement.is_none()&&self.fault.is_none()&&self.fault_publication.terminal_is_empty()&&self.completion.is_none()&&self.credit.is_none()&&self.snapshot.is_none()&&self.snapshot_close.is_none()&&self.request_metadata.is_none()&&self.metadata_retirement.is_none()&&self.pending_result.is_none()}
}
impl<S:IncrementalMediaExportSpec> IncrementalMediaExportJob<S>{
    fn step_owned<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
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

    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let item=RetirementDemand{depth:1,..Default::default()};
        if let Some(owner)=self.output_retirement.as_ref(){return semio_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_box_demands(owner,body);}
        if !self.fault_publication.terminal_is_empty(){return self.fault_publication.retirement_demands();}
        if self.fault.is_some(){return semio_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.fault);}
        if self.cursor.is_some(){return semio_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.cursor);}
        if let Some(page)=self.page.as_ref(){return Ok(RetirementDemand{release_bytes:page.capacity(),depth:1,..Default::default()});}
        if self.chunks.is_some(){return semio_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.chunks);}
        if self.pending_result.is_some(){return semio_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_result)}
        if self.completion.is_some(){return semio_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion);}
        if self.credit.is_some(){return semio_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.credit);}
        if self.snapshot.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Arc<S::Snapshot>>(),..item});}
        if let Some(lease)=self.snapshot_close.as_ref(){return Ok(if lease.terminal_is_empty(){item}else{lease.retirement_demands()});}
        if let Some(owner)=self.metadata_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(item)}else{owner.selected_demand(body)};}
        if self.request_metadata.is_some(){return Ok(item);}
        Ok(Default::default())
    }

    fn close_turn(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
        use semio_framework_value::{OriginalRetirementFailure as Failure,RetainedWorkOutcome as Turn,RetainedWorkPoll as Poll};
        use semio_plugin::plugin_app_close_prelude::store::os_store::snapshot_read::{artifact_retirement_admit_owned,artifact_retirement_box_close_step};
        let grant=effect.grant();let body=grant.maximum_copy_bytes;
        let done=|result:Result<RetainedCloneProgress,Failure>,work:u128|Poll::Complete(Turn{result:result.map(semio_framework_value::RetainedCloneStep::Progress),work});
        let admit=|turn:Turn<semio_framework_value::RetainedCloneStep,ValueError>|Poll::Complete(Turn{result:turn.result.map_err(Failure::from),work:turn.work});
        let demand=match self.close_demands(body){Ok(demand)=>demand,Err(error)=>return done(Err(error.into()),0)};
        if demand==RetirementDemand::default(){return done(Ok(Default::default()),0);}
        if grant.maximum_depth<demand.depth{return done(Err(ValueError::literal(ValueRefusalKind::DepthLimit,"media export close exceeds its admitted depth").into()),0);}
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return done(Ok(Default::default()),0);}
        let item=RetainedCloneProgress{copied_items:1,..Default::default()};
        if self.output_retirement.is_some(){return artifact_retirement_box_close_step(&mut self.output_retirement,body,effect);}
        if !self.fault_publication.terminal_is_empty(){let turn=self.fault_publication.close_step(effect);return Poll::Complete(Turn{result:turn.result.map_err(Failure::from),work:turn.work});}
        if self.fault.is_some(){return admit(artifact_retirement_admit_owned(&mut self.fault,&mut self.output_retirement,effect));}
        if self.cursor.is_some(){return admit(artifact_retirement_admit_owned(&mut self.cursor,&mut self.output_retirement,effect));}
        if let Some(page)=self.page.take(){let released=page.capacity();return done(Ok(RetainedCloneProgress{released_bytes:released,..item}),1);}
        if self.chunks.is_some(){return admit(artifact_retirement_admit_owned(&mut self.chunks,&mut self.output_retirement,effect));}
        if self.pending_result.is_some(){return admit(artifact_retirement_admit_owned(&mut self.pending_result,&mut self.output_retirement,effect));}
        if self.completion.is_some(){return admit(artifact_retirement_admit_owned(&mut self.completion,&mut self.output_retirement,effect));}
        if self.credit.is_some(){return admit(artifact_retirement_admit_owned(&mut self.credit,&mut self.output_retirement,effect));}
        if let Some(snapshot)=self.snapshot.as_ref(){
            if !self.snapshot_close.as_ref().is_some_and(|lease|lease.can_release(snapshot)){return done(Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"media export snapshot has no live close witness").into()),0);}
            self.snapshot=None;
            return done(Ok(RetainedCloneProgress{copied_bytes:demand.copy_bytes,..item}),1);
        }
        if let Some(lease)=self.snapshot_close.as_mut(){if lease.terminal_is_empty(){self.snapshot_close=None;return done(Ok(item),1)}return done(lease.close_step(grant).map(|step|step.progress()).map_err(Failure::from),1);}
        if let Some(owner)=self.metadata_retirement.as_mut(){if owner.terminal_is_empty(){self.metadata_retirement.take();return done(Ok(item),1)}return owner.step(body,effect);}
        if let Some(metadata)=self.request_metadata.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new(metadata){Ok(owner)=>*self.metadata_retirement=Some(owner),Err((error,metadata))=>{*self.request_metadata=Some(metadata);return done(Err(error.into()),1)}}return done(Ok(item),1);}
        done(Ok(item),0)
    }
}
impl<S:IncrementalMediaExportSpec> ArtifactReservedJob for IncrementalMediaExportJob<S>{}

//! 📤️ Cancellable layer-to-PNG export with sealed, credited output pages.
use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
#[path="🧵️work/🦀️.rs"]
mod original_work;
pub use original_work::ImageExportWork;
use crate::{RasterPlayApp, RasterCommand};
use semio_framework_action_bus::RetainedToolWireInput;
use semio_framework_plugin::{ArtifactOwnedToolJobRequest,ArtifactToolCompletion,ArtifactDownloadOutput,EphemeralEmit};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory};

use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, StepContext};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_plugin::{ArtifactMediaExportJobRequest,ArtifactReservedJob,ArtifactReservedToolJob,ArtifactToolPublicationContract,ArtifactToolPublicationLane,EditorApp,Fault,MediaClass,MediaForm,MediaType};
use semio_framework_plugin::app::{ArtifactMediaExportCompletion,ArtifactMediaExportCredit,ArtifactMediaExportResult,ArtifactOutputChunks,ArtifactSnapshotCloseLease};
use std::sync::Arc;
use std::mem::size_of;

pub const TOOL_ID:&str="export-media:image:out";
pub const PAYLOAD_SCHEMA:&str="raster.raster.media-export.v1";
const MEDIA_SCHEMA:&str="2d.image";
pub const BASE64_INPUT_BYTES:usize=ArtifactOutputChunks::CHUNK_BYTES/4*3;

pub fn base64_output_page(bytes:&[u8])->Result<Vec<u8>,String> {
    if bytes.len()>BASE64_INPUT_BYTES {return Err("raster.export-page-limit".into());}
    const ALPHABET:&[u8;64]=b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output=Vec::with_capacity(bytes.len().div_ceil(3)*4);
    for chunk in bytes.chunks(3) {let a=chunk[0];let b=chunk.get(1).copied().unwrap_or(0);let c=chunk.get(2).copied().unwrap_or(0);output.extend_from_slice(&[ALPHABET[(a>>2) as usize],ALPHABET[(((a&3)<<4)|(b>>4)) as usize],if chunk.len()>1{ALPHABET[(((b&15)<<2)|(c>>6)) as usize]}else{b'='},if chunk.len()>2{ALPHABET[(c&63) as usize]}else{b'='}]);}
    Ok(output)
}

pub fn progress_payload(stage:&str,completed:u64)->Vec<u8> {
    let (en,de)=match stage {"prepare"=>("Preparing layers","Ebenen werden vorbereitet"),"composite"=>("Rendering layers","Ebenen werden gerendert"),"encode"=>("Encoding PNG","PNG wird kodiert"),_=>("Preparing download","Download wird vorbereitet")};
    format!(r#"{{"stage":"{stage}","en":"{en}","de":"{de}","completedUnits":{completed}}}"#).into_bytes()
}

pub struct RasterImageExportJob {
    source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),request_metadata:std::mem::ManuallyDrop<Option<(String,String,String)>>,metadata_retirement:semio_framework_value::retained_clone::RetainedCloneClose,
    operation:Operation,snapshot:Option<Arc<RasterSnapshot>>,snapshot_close:Option<ArtifactSnapshotCloseLease<RasterSnapshot>>,work:ImageExportWork,
    chunks:Option<ArtifactOutputChunks>,credit:Option<ArtifactMediaExportCredit>,completion:Option<ArtifactMediaExportCompletion>,
    publication:RetainedJobPublication,publishing:Option<JobPublicationKind>,source:Vec<u8>,delivered:bool,
    pending_result:std::mem::ManuallyDrop<Option<Result<ArtifactMediaExportResult,semio_framework::MediaError>>>,result_published:bool,
    completed:bool,closing:bool,units:u64,close_work:u64,progress_pending:Option<(&'static str,u64)>,
    rejected_download:Option<ArtifactDownloadOutput>,active:Option<Box<dyn store::ErasedSnapshotRetirement>>,pending_page:std::mem::ManuallyDrop<Option<Vec<u8>>>,page_published:bool,credited:usize,output_metadata:std::mem::ManuallyDrop<Option<(String,String,String)>>,metadata_phase:u8,
    download:Option<ArtifactToolCompletion<EditorApp<RasterPlayApp>>>,request_owner:std::mem::ManuallyDrop<Option<ArtifactOwnedToolJobRequest<EditorApp<RasterPlayApp>>>>,raw_input:Option<RetainedToolWireInput>,
}
impl RasterImageExportJob {
    pub fn admit_original(request:&mut Option<ArtifactMediaExportJobRequest<EditorApp<RasterPlayApp>>>,body:&mut Option<Box<dyn ArtifactReservedJob+Send>>,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let _=cx.original_source_parent()?;if cx.should_yield()||cx.is_cancelled(){return Ok(false)}let grant=cx.retained_grant();
        if body.is_none(){let original=request.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"raster media producer lost its original request"))?;let bytes=size_of::<Self>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes||grant.maximum_copy_bytes<bytes{return Ok(false)}
            let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(std::alloc::Layout::new::<Self>())}.cast::<Self>()).ok_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,"raster original body allocation refused"))?;
            let owner=Self{source_identity:original.source_identity,request_metadata:std::mem::ManuallyDrop::new(None),metadata_retirement:Default::default(),operation:original.operation,snapshot:None,snapshot_close:None,work:ImageExportWork::default(),chunks:None,credit:None,completion:None,publication:RetainedJobPublication::new(),publishing:None,source:Vec::new(),delivered:false,pending_result:std::mem::ManuallyDrop::new(None),result_published:false,completed:false,closing:false,units:0,close_work:0,progress_pending:None,rejected_download:None,active:None,pending_page:std::mem::ManuallyDrop::new(None),page_published:false,credited:0,output_metadata:std::mem::ManuallyDrop::new(None),metadata_phase:0,download:None,request_owner:std::mem::ManuallyDrop::new(None),raw_input:None};
            *body=Some(unsafe{pointer.as_ptr().write(owner);Box::from_raw(pointer.as_ptr())});cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        let owner=(&mut**body.as_mut().unwrap()as &mut dyn std::any::Any).downcast_mut::<Self>().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"raster producer body requires its exact concrete owner"))?;
        if owner.snapshot.is_some()&&request.is_none(){return Ok(true)}if owner.snapshot.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"raster original request cannot overwrite a prepared snapshot").into())}
        let bytes=size_of::<ArtifactMediaExportJobRequest<EditorApp<RasterPlayApp>>>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}let original=request.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"raster producer lost its retained request"))?;
        owner.snapshot=Some(original.snapshot);owner.snapshot_close=Some(original.snapshot_close);owner.chunks=Some(original.output_chunks);owner.credit=Some(original.output_credit);owner.completion=Some(original.completion);*owner.request_metadata=Some((original.tool_id,original.parent_document_id,original.port));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(false)
    }
    fn stage_source(&mut self,stage:Option<(&'static str,u64)>,detail:&'static str,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use std::io::Write;
        let grant=cx.retained_grant();let capacity=if self.source.capacity()==0{512}else{0};if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<512||grant.maximum_capacity_bytes<capacity{return Ok(false)}
        if capacity!=0{self.source.try_reserve_exact(capacity).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original export progress backing refused"))?;}
        self.source.clear();if let Some((stage,completed))=stage{let(en,de)=match stage{"prepare"=>("Preparing layers","Ebenen werden vorbereitet"),"composite"=>("Rendering layers","Ebenen werden gerendert"),"encode"=>("Encoding PNG","PNG wird kodiert"),_=>("Preparing download","Download wird vorbereitet")};write!(&mut self.source,r#"{{"stage":"{stage}","en":"{en}","de":"{de}","completedUnits":{completed}}}"#).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original progress writer refused admitted backing"))?;self.publishing=Some(JobPublicationKind::Checkpoint{applied_progress:completed});}else{self.source.extend_from_slice(detail.as_bytes());self.publishing=Some(JobPublicationKind::Fault);}
        let copied=self.source.len();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copied,retained_capacity_bytes:if capacity!=0{self.source.capacity()}else{0},..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }
    fn advance(&mut self,cx:&mut StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure> {
        if self.rejected_download.is_some(){self.download.as_ref().unwrap().complete_download_original(&mut self.rejected_download,&mut self.completed,cx)?;return Ok(())}
        if self.chunks.is_none(){ArtifactOutputChunks::original_birth_admit(ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES,&mut self.chunks,cx)?;return Ok(())}
        if self.pending_page.is_some()||self.page_published{if self.chunks.as_ref().unwrap().push_original(&mut self.pending_page,&mut self.page_published,cx)?{if let Some(credit)=self.credit.as_ref(){credit.credit_original(self.chunks.as_ref().unwrap().bytes()-self.credited)?;self.credited=self.chunks.as_ref().unwrap().bytes();}self.page_published=false;}return Ok(())}
        if self.work.page_ready(){let turn=self.work.take_page(cx.retained_grant());let work=turn.work;let result=turn.result.map(|output|if let Some((page,progress))=output{*self.pending_page=Some(page);RetainedCloneStep::Progress(progress)}else{RetainedCloneStep::Progress(Default::default())});semio_framework_job::receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result,work})?;return Ok(())}
        if !self.work.done{let document=self.snapshot.as_deref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"raster.export-snapshot-missing"))?;semio_framework_job::receive_original_retained_work(cx,self.work.advance(document,cx.retained_grant()))?;return Ok(())}
        if !self.chunks.as_ref().unwrap().is_sealed(){self.chunks.as_ref().unwrap().seal_original(cx)?;return Ok(())}
        if self.metadata_phase<3{
            let values=if self.download.is_some(){["image.png","image/png","base64"]}else{[MEDIA_SCHEMA,"image/png",""]};let text=values[self.metadata_phase as usize];let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<text.len()+size_of::<String>()||grant.maximum_capacity_bytes<text.len(){return Ok(())}
            if self.output_metadata.is_none(){*self.output_metadata=Some((String::new(),String::new(),String::new()));}let metadata=self.output_metadata.as_mut().unwrap();let field=match self.metadata_phase{0=>&mut metadata.0,1=>&mut metadata.1,_=>&mut metadata.2};let before=field.capacity();field.try_reserve_exact(text.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original export metadata backing refused"))?;field.push_str(text);self.metadata_phase+=1;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:text.len()+size_of::<String>(),retained_capacity_bytes:field.capacity()-before,..Default::default()})?;cx.consume_fuel(1);return Ok(())
        }
        let grant=cx.retained_grant();let bytes=if self.download.is_some(){size_of::<ArtifactDownloadOutput>()}else{size_of::<ArtifactMediaExportResult>()};if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(())}
        if self.download.is_none(){self.credit.as_ref().unwrap().credit_original(MEDIA_SCHEMA.len())?;}
        let(first,mime,third)=self.output_metadata.take().unwrap();let chunks=self.chunks.take().unwrap();if self.download.is_some(){self.rejected_download=Some(ArtifactDownloadOutput{filename:first,mime_type:mime,encoding:Some(third),chunks,media_export_handle:None,media_export_metadata:None});}else{*self.pending_result=Some(Ok(ArtifactMediaExportResult{media_type:MediaType{class:MediaClass::TwoD,form:MediaForm::Raster},schema:first,mime_type:mime,chunks}));}
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);
        Ok(())
    }
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError> {
        let item=RetirementDemand {depth:1,..Default::default()};
        if !self.publication.terminal_is_empty(){return self.publication.retirement_demands();}
        if self.delivered{return Ok(item);}
        if let Some(owner)=self.active.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(owner,body);}
        if self.rejected_download.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.rejected_download);}
        if !self.work.terminal_is_empty(){return self.work.close_demands(body);}
        if self.pending_page.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_page);}
        if self.output_metadata.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.output_metadata);}
        if let Some(input)=self.raw_input.as_ref(){return Ok(RetirementDemand {copy_bytes:input.next_close_copy_byte_demand()?,capacity_bytes:input.next_close_capacity_byte_demand(body)?,release_bytes:input.next_close_release_byte_demand()?,depth:input.next_close_depth_demand()?.max(1)});}
        if self.chunks.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.chunks);}
        if self.pending_result.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_result)}
        if self.completion.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion)}
        if self.download.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.download)}
        if self.credit.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.credit)}
        if self.snapshot.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Arc<RasterSnapshot>>(),..item})}
        if let Some(lease)=self.snapshot_close.as_ref(){return Ok(if lease.terminal_is_empty(){item}else{lease.retirement_demands()})}
        if self.request_owner.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.request_owner);}
        if self.source.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.source.capacity(),..item})}
        if !self.metadata_retirement.is_empty()||self.request_metadata.is_some(){return self.metadata_retirement.next_owner_demands::<(String,String,String)>(self.request_metadata.is_some(),&None,body)}
        Ok(Default::default())
    }
    fn close_turn(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        self.close_work=1;
        let demand=self.close_demands(grant.maximum_copy_bytes)?;
        if demand==RetirementDemand::default(){return Ok(Default::default());}
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"raster export close exceeds its admitted depth"));}
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(Default::default());}
        let item=RetainedCloneProgress {copied_items:1,..Default::default()};
        let child=RetainedCloneGrant {maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if !self.publication.terminal_is_empty(){return self.publication.close_step(child).map(|step|step.progress());}
        if self.delivered{self.delivered=false;return Ok(item);}
        if self.active.is_some(){let turn=store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active,grant);self.close_work=turn.work;return turn.result.map(|step|step.progress());}
        if self.rejected_download.is_some(){let turn=store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.rejected_download,&mut self.active,grant);self.close_work=turn.work;return turn.result.map(|step|step.progress());}
        if !self.work.terminal_is_empty(){self.work.cancel();let turn=self.work.close_step(grant);self.close_work=turn.work;return turn.result.map(|step|step.progress());}
        if self.pending_page.is_some(){let turn=store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_page,&mut self.active,grant);self.close_work=turn.work;return turn.result.map(|step|step.progress());}
        if self.output_metadata.is_some(){let turn=store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.output_metadata,&mut self.active,grant);self.close_work=turn.work;return turn.result.map(|step|step.progress());}
        if self.chunks.is_some(){let turn=store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.chunks,&mut self.active,grant);self.close_work=turn.work;return turn.result.map(|step|step.progress());}
        if self.pending_result.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_result,&mut self.active,grant).map(|step|step.progress())}
        if self.completion.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.completion,&mut self.active,grant).map(|step|step.progress())}
        if self.download.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.download,&mut self.active,grant).map(|step|step.progress())}
        if self.credit.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.credit,&mut self.active,grant).map(|step|step.progress())}
        if let Some(snapshot)=self.snapshot.as_ref(){
            if !self.snapshot_close.as_ref().map_or_else(||Arc::strong_count(snapshot)>1,|lease|lease.can_release(snapshot)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"raster export snapshot has no live close witness"));}
            self.snapshot=None;return Ok(RetainedCloneProgress{copied_bytes:demand.copy_bytes,..item});
        }
        if let Some(lease)=self.snapshot_close.as_mut(){if lease.terminal_is_empty(){self.snapshot_close=None;return Ok(item)}return lease.close_step(grant).map(|step|step.progress())}
        if self.request_owner.is_some(){let turn=store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.request_owner,&mut self.active,grant);self.close_work=turn.work;return turn.result.map(|step|step.progress());}
        if self.source.capacity()!=0{let released=std::mem::take(&mut self.source).capacity();return Ok(RetainedCloneProgress{released_bytes:released,..item})}
        if !self.metadata_retirement.is_empty(){let turn=self.metadata_retirement.step_granted(grant);self.close_work=turn.work;return turn.result.map(|step|step.progress())}
        if self.request_metadata.is_some(){let turn=self.metadata_retirement.begin_granted(&mut self.request_metadata,grant);self.close_work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default())).progress())}
        Ok(item)
    }
}
impl RasterImageExportJob {
    fn step_value<'a>(&'a mut self,context:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure> {
        if self.delivered {
            let step=self.publication.close_step(context.retained_grant())?;
            context.consume_retained(step.progress())?;
            if matches!(step,RetainedCloneStep::Complete(_)){self.delivered=false;}
            return Ok(None);
        }
        if self.closing||context.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(context).map_err(Into::into);}
        if context.should_yield(){return Ok(None);}
        if (context.operation(),context.generation())!=self.source_identity {self.stage_source(None,"raster.export-operation-authority-invalid",context)?;return Ok(None)}
        if let Some(kind)=self.publishing {
            let result=self.publication.advance_from_source(kind,&self.source,context)?;
            if result.is_some(){self.delivered=true;if matches!(kind,JobPublicationKind::Checkpoint {..}){self.publishing=None;}}
            return Ok(result);
        }
        if let Some(stage)=self.progress_pending{if self.stage_source(Some(stage),"",context)?{self.progress_pending=None;}return Ok(None)}
        if self.completed{return JobOutcomeBorrow::admit_complete(context,None,None).map_err(Into::into);}
        if self.pending_result.is_some()||self.result_published{self.completed=self.completion.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"raster media lost its original completion receiver"))?.complete_original(&mut self.pending_result,&mut self.result_published,context)?;return Ok(None)}
        let stage=self.work.stage();context.set_stage(stage);
        self.advance(context)?;self.units+=1;
        if context.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(context).map_err(Into::into);}
        if self.completed{return JobOutcomeBorrow::admit_complete(context,None,None).map_err(Into::into)}if self.units==1||self.units.is_multiple_of(64)||self.work.stage()!=stage{let next=self.work.stage();self.progress_pending=Some((next,self.units));}Ok(None)
    }
}
impl InteractiveJob for RasterImageExportJob {
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(self.step_value(cx))}
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield=>descriptor.yielded(),
            JobOutcomeKind::Cancelled=>descriptor.cancelled(),
            JobOutcomeKind::Complete if self.completed=>descriptor.complete(None,None),
            _=>self.publication.borrow_outcome(descriptor),
        }
    }
    fn begin_close(&mut self){self.closing=true;self.work.cancel();if let Some(input)=self.raw_input.as_mut(){input.begin_close();}}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError> {Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError> {Ok(self.close_demands(0)?.depth)}
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        self.begin_close();
        if let Some(input)=self.raw_input.as_mut(){let step=input.close_step(cx)?;if input.terminal_is_empty(){self.raw_input=None}return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}
        let demand=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=self.close_turn(grant).map(RetainedCloneStep::Progress);semio_framework_value::RetainedWorkOutcome{result,work:self.close_work}})?;
        Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }
    fn terminal_is_empty(&self)->bool {self.rejected_download.is_none()&&self.active.is_none()&&self.pending_page.is_none()&&self.output_metadata.is_none()&&self.raw_input.is_none()&&self.download.is_none()&&self.publication.terminal_is_empty()&&!self.delivered&&self.work.terminal_is_empty()&&self.chunks.is_none()&&self.completion.is_none()&&self.credit.is_none()&&self.snapshot.is_none()&&self.snapshot_close.is_none()&&self.source.capacity()==0&&self.request_owner.is_none()&&self.request_metadata.is_none()&&self.metadata_retirement.is_empty()&&self.pending_result.is_none()}
}
impl ArtifactReservedJob for RasterImageExportJob {}

pub struct RasterMediaExportJobFactory {keys:[ToolFactoryKey;1]}
impl RasterMediaExportJobFactory {pub fn new(controller:&str)->Self {Self {keys:[ToolFactoryKey::new(controller,TOOL_ID)]}}}
impl ToolJobFactory for RasterMediaExportJobFactory {
    type Payload=ArtifactReservedToolJob;type Job=ArtifactReservedToolJob;
    fn keys(&self)->&[ToolFactoryKey]{&self.keys}
    fn payload_schema_id(&self)->&str {PAYLOAD_SCHEMA}
    fn classification(&self)->InteractiveJobClassification {InteractiveJobClassification::Migrated}
    fn execution_contract(&self)->ToolExecutionContract {ToolExecutionContract::resumable(4096,4096,1,ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES,2000,64,1)}
    semio_framework_plugin::retained_command::reserved_tool_job_factory_admission!();
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for RasterMediaExportJobFactory {
    type Owner=EditorApp<RasterPlayApp>;
    const TOOL_IDS:&'static [&'static str]=&[TOOL_ID];
    const DOCUMENT_SCHEMA:&'static str=semio_s_artifact_raster_raster_model::RASTER_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS:&'static [ArtifactToolPublicationContract]=&[ArtifactToolPublicationContract {tool_id:TOOL_ID,lanes:&[ArtifactToolPublicationLane::HostOnly]}];
}

/// 🧾️ The command runtime retains its snapshot and completion while this private output queue is built.
pub fn build_download_job(request:&mut Option<ArtifactOwnedToolJobRequest<EditorApp<RasterPlayApp>>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_plugin::__job::StepContext<'_>)->Result<bool,semio_framework_plugin::__job::OriginalTurnFailure> {
    let original=request.as_ref().ok_or_else(||semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated,"original raster export request is absent"))?;
    if !original.source_binding.matches(cx)||original.checkpoint.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Raster download requires its initiating source and no checkpoint").into())}
    if original.tool_id!="exportPng"||!matches!(*original.command,RasterCommand::ExportPng(_)){return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated,"raster.export-command-mismatch").into());}
    let demand=semio_framework::action_bus::ToolJobAdmissionDemand{items:1,copy_bytes:std::mem::size_of::<RasterImageExportJob>()+std::mem::size_of::<ArtifactOwnedToolJobRequest<EditorApp<RasterPlayApp>>>(),capacity_bytes:std::mem::size_of::<RasterImageExportJob>(),release_bytes:0,depth:1};
    semio_framework::action_bus::admit_original_tool_construction(cx,output,demand,|_|{
        let mut retained=request.take().ok_or_else(||semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated,"original raster export request is absent"))?;
        let taken=&mut retained;
        let mut job=RasterImageExportJob {source_identity:(taken.operation.operation,taken.operation.generation),request_metadata:std::mem::ManuallyDrop::new(None),metadata_retirement:Default::default(),operation:taken.operation.clone(),snapshot:Some(taken.snapshot.clone()),snapshot_close:None,work:ImageExportWork::default(),chunks:None,credit:None,completion:None,publication:RetainedJobPublication::new(),publishing:None,source:Vec::new(),delivered:false,pending_result:std::mem::ManuallyDrop::new(None),result_published:false,completed:false,closing:false,units:0,close_work:0,progress_pending:None,download:Some(taken.completion.clone()),request_owner:std::mem::ManuallyDrop::new(None),raw_input:None,rejected_download:None,active:None,pending_page:std::mem::ManuallyDrop::new(None),page_published:false,credited:0,output_metadata:std::mem::ManuallyDrop::new(None),metadata_phase:0};
        job.raw_input=taken.input.take();
        *job.request_owner=Some(retained);
        Ok((semio_framework::ErasedToolJob::from_owned(Box::new(job)),semio_framework_plugin::__value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:std::mem::size_of::<RasterImageExportJob>(),..Default::default()}))
    })
}

pub struct RasterDownloadJobFactory {keys:[ToolFactoryKey;1]}
impl RasterDownloadJobFactory {pub fn new(controller:&str)->Self {Self {keys:[ToolFactoryKey::new(controller,"exportPng")]}}}
impl ToolJobFactory for RasterDownloadJobFactory {
    type Payload=RasterImageExportJob;type Job=RasterImageExportJob;
    fn keys(&self)->&[ToolFactoryKey]{&self.keys}
    fn payload_schema_id(&self)->&str {"raster.raster.download-png.v1"}
    fn classification(&self)->InteractiveJobClassification {InteractiveJobClassification::Migrated}
    fn execution_contract(&self)->ToolExecutionContract {ToolExecutionContract::resumable(4096,4096,1,ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES,2000,64,1)}
    fn admission_demands(&self,operation:&Operation,payload:&Self::Payload,input:Option<&RetainedToolWireInput>,checkpoint:Option<&RetainedToolWireInput>)->Result<semio_framework::action_bus::ToolJobAdmissionDemand,ValueError>{
        if payload.source_identity!=(operation.operation,operation.generation)||checkpoint.is_some()||input.is_some_and(|input|input.declared_bytes()>4096)||input.is_some()&&payload.raw_input.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Raster download factory requires its exact typed owner and nonduplicated bounded wire"))}
        Ok(semio_framework::action_bus::ToolJobAdmissionDemand{items:1,copy_bytes:2*size_of::<Box<RasterImageExportJob>>()+size_of::<Option<RetainedToolWireInput>>(),capacity_bytes:0,release_bytes:0,depth:1})
    }
    fn admit_job(&mut self,cx:&mut StepContext<'_>,operation:&Operation,payload:&mut Option<Box<Self::Payload>>,input:&mut Option<RetainedToolWireInput>,checkpoint:&mut Option<RetainedToolWireInput>,output:&mut Option<Box<Self::Job>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        cx.original_source_parent()?;if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Raster download candidate slot is occupied").into())}let owner=payload.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Raster download owner is absent"))?;let demand=self.admission_demands(operation,owner,input.as_ref(),checkpoint.as_ref())?;let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items<demand.items||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes{return Ok(false)}
        let mut owner=payload.take().unwrap();if owner.raw_input.is_none(){owner.raw_input=input.take();}*output=Some(owner);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for RasterDownloadJobFactory {
    type Owner=EditorApp<RasterPlayApp>;
    const TOOL_IDS:&'static [&'static str]=&["exportPng"];
    const DOCUMENT_SCHEMA:&'static str=semio_s_artifact_raster_raster_model::RASTER_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS:&'static [ArtifactToolPublicationContract]=&[ArtifactToolPublicationContract {tool_id:"exportPng",lanes:&[ArtifactToolPublicationLane::HostOnly]}];
}

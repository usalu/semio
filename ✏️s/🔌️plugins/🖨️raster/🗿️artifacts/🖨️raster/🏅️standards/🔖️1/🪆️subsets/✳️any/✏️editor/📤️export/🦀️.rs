//! 📤️ Cancellable layer-to-PNG export with sealed, credited output pages.
use crate::RasterSnapshot;
use crate::standards::v1::subsets::any::io::RasterStackPreparation;
use crate::editor::raster::{RasterPlayApp,RasterCommand};
use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework_plugin::{ArtifactOwnedToolJobRequest,ArtifactToolCompletion,ArtifactDownloadOutput,EphemeralEmit};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_pixels::{compositing::layers::RasterStackJob,png_encoding::PngEncodeJob};
use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeKind,JobOutcomeView,JobPublicationKind,Operation,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedJobPublication,StepContext};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_plugin::{ArtifactMediaExportJobRequest,ArtifactReservedJob,ArtifactReservedToolJob,ArtifactToolPublicationContract,ArtifactToolPublicationLane,EditorApp,Fault,MediaClass,MediaForm,MediaType};
use semio_framework_plugin::app::{ArtifactMediaExportCompletion,ArtifactMediaExportCredit,ArtifactMediaExportResult,ArtifactOutputChunks,ArtifactSnapshotCloseLease};
use std::sync::Arc;

pub const TOOL_ID:&str="export-media:image:out";
pub const PAYLOAD_SCHEMA:&str="raster.raster.media-export.v1";
const MEDIA_SCHEMA:&str="2d.image";
const BASE64_INPUT_BYTES:usize=ArtifactOutputChunks::CHUNK_BYTES/4*3;

fn base64_output_page(bytes:&[u8])->Result<Vec<u8>,String> {
    if bytes.len()>BASE64_INPUT_BYTES {return Err("raster.export-page-limit".into());}
    const ALPHABET:&[u8;64]=b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output=Vec::with_capacity(bytes.len().div_ceil(3)*4);
    for chunk in bytes.chunks(3) {let a=chunk[0];let b=chunk.get(1).copied().unwrap_or(0);let c=chunk.get(2).copied().unwrap_or(0);output.extend_from_slice(&[ALPHABET[(a>>2) as usize],ALPHABET[(((a&3)<<4)|(b>>4)) as usize],if chunk.len()>1{ALPHABET[(((b&15)<<2)|(c>>6)) as usize]}else{b'='},if chunk.len()>2{ALPHABET[(c&63) as usize]}else{b'='}]);}
    Ok(output)
}

#[derive(Default)]
struct ImageExportWork {
    preparing:Option<RasterStackPreparation>,compositing:Option<RasterStackJob>,encoding:Option<PngEncodeJob>,encoded:Vec<u8>,cursor:usize,done:bool,closing:bool,
}
impl ImageExportWork {
    fn stage(&self)->&'static str {if !self.encoded.is_empty(){"output"}else if self.encoding.is_some(){"encode"}else if self.compositing.is_some(){"composite"}else{"prepare"}}
    fn advance(&mut self,document:&RasterSnapshot,grant:usize)->Result<Option<Vec<u8>>,String> {
        if self.closing||self.done {return Err("raster.export-inactive".into());}
        if grant==0 {return Ok(None);}
        if !self.encoded.is_empty() {
            let end=(self.cursor+BASE64_INPUT_BYTES).min(self.encoded.len());let chunk=base64_output_page(&self.encoded[self.cursor..end])?;self.cursor=end;self.done=end==self.encoded.len();return Ok(Some(chunk));
        }
        if let Some(job)=self.encoding.as_mut() {
            if job.advance().map_err(|error|error.to_string())?.done {self.encoded=self.encoding.take().unwrap().into_result().map_err(|error|error.to_string())?.data;}
            return Ok(None);
        }
        if let Some(job)=self.compositing.as_mut() {
            if job.advance(4096).map_err(|error|error.to_string())?.done {let result=self.compositing.take().unwrap().into_result().map_err(|error|error.to_string())?;if result.empty{return Err("raster.export-no-visible-pixels".into());}self.encoding=Some(PngEncodeJob::new(result.image).map_err(|error|error.to_string())?);}
            return Ok(None);
        }
        if let Some(job)=self.preparing.as_mut() {
            if job.advance(document,32768)? {self.compositing=Some(self.preparing.take().unwrap().into_job()?);}
        } else {self.preparing=Some(RasterStackPreparation::new(document)?);}
        Ok(None)
    }
    fn terminal_is_empty(&self)->bool {self.preparing.is_none()&&self.compositing.is_none()&&self.encoding.is_none()&&self.encoded.is_empty()}
}

fn progress_payload(stage:&str,completed:u64)->Vec<u8> {
    let (en,de)=match stage {"prepare"=>("Preparing layers","Ebenen werden vorbereitet"),"composite"=>("Rendering layers","Ebenen werden gerendert"),"encode"=>("Encoding PNG","PNG wird kodiert"),_=>("Preparing download","Download wird vorbereitet")};
    format!(r#"{{"stage":"{stage}","en":"{en}","de":"{de}","completedUnits":{completed}}}"#).into_bytes()
}

pub struct RasterImageExportJob {
    operation:Operation,snapshot:Option<Arc<RasterSnapshot>>,snapshot_close:Option<ArtifactSnapshotCloseLease<RasterSnapshot>>,work:ImageExportWork,
    chunks:Option<ArtifactOutputChunks>,credit:Option<ArtifactMediaExportCredit>,completion:Option<ArtifactMediaExportCompletion>,
    publication:RetainedJobPublication,publishing:Option<JobPublicationKind>,source:Vec<u8>,delivered:bool,
    completed:bool,closing:bool,units:u64,
    rejected_download:Option<ArtifactDownloadOutput>,active:Option<Box<dyn store::ErasedSnapshotRetirement>>,encoded_retiring:Option<Vec<u8>>,
    download:Option<ArtifactToolCompletion<EditorApp<RasterPlayApp>>>,raw_input:Option<RetainedToolWireInput>,raw_bytes:Vec<u8>,raw_cursor:usize,raw_validated:bool,
}
impl RasterImageExportJob {
    pub fn new(request:ArtifactMediaExportJobRequest<EditorApp<RasterPlayApp>>)->Self {
        Self {operation:request.operation,snapshot:Some(request.snapshot),snapshot_close:Some(request.snapshot_close),work:ImageExportWork::default(),chunks:Some(request.output_chunks),credit:Some(request.output_credit),completion:Some(request.completion),publication:RetainedJobPublication::new(),publishing:None,source:Vec::new(),delivered:false,completed:false,closing:false,units:0,rejected_download:None,active:None,encoded_retiring:None,download:None,raw_input:None,raw_bytes:Vec::new(),raw_cursor:0,raw_validated:true}
    }
    fn stage_fault(&mut self,detail:&str) {
        self.source=detail.chars().take(256).collect::<String>().into_bytes();self.publishing=Some(JobPublicationKind::Fault);
    }
    fn stage_progress(&mut self,stage:&str,completed:u64) {
        self.source=progress_payload(stage,completed);self.publishing=Some(JobPublicationKind::Checkpoint {applied_progress:completed});
    }
    fn advance(&mut self)->Result<(),Fault> {
        let chunk=self.work.advance(self.snapshot.as_deref().ok_or_else(||Fault::from("raster.export-snapshot-missing"))?,1).map_err(Fault::from)?;
        if let Some(chunk)=chunk {if let Some(credit)=self.credit.as_ref(){credit.credit(chunk.len())?;}self.chunks.as_ref().unwrap().push(chunk)?;}
        if self.work.done {
            self.chunks.as_ref().unwrap().seal()?;
            if let Some(completion)=self.download.as_ref() {
                if let Err(rejected)=completion.complete_download(ArtifactDownloadOutput::new("image.png","image/png",Some("base64".into()),self.chunks.take().unwrap()),EphemeralEmit::default()){
                    if let Ok(download)=rejected.download{self.rejected_download=Some(download);}return Err(rejected.fault);
                }
            } else {
                self.credit.as_ref().unwrap().credit(MEDIA_SCHEMA.len())?;
                let result=ArtifactMediaExportResult::structured(MediaType {class:MediaClass::TwoD,form:MediaForm::Raster},MEDIA_SCHEMA,"image/png",self.chunks.take().unwrap())?;
                self.completion.as_ref().unwrap().complete(Ok(result))?;
            }
            self.completed=true;
        }
        Ok(())
    }
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError> {
        let item=RetirementDemand {depth:1,..Default::default()};
        let owned=|bytes:usize|RetirementDemand {capacity_bytes:bytes,depth:2,..Default::default()};
        if !self.publication.terminal_is_empty(){return self.publication.retirement_demands();}
        if self.delivered{return Ok(item);}
        if let Some(owner)=self.active.as_ref(){return store::artifact_retirement_box_demands(owner,body);}
        if self.rejected_download.is_some(){return store::artifact_retirement_owned_birth_demands(&self.rejected_download);}
        if self.work.preparing.is_some()||self.work.compositing.is_some()||self.work.encoding.is_some(){return Ok(item);}
        if self.encoded_retiring.is_some()||self.work.encoded.capacity()!=0{return Ok(owned(semio_framework_value::retirement::owned_retirement_birth_bytes::<Vec<u8>>()));}
        if let Some(input)=self.raw_input.as_ref(){return Ok(RetirementDemand {copy_bytes:input.next_close_copy_byte_demand()?,capacity_bytes:input.next_close_capacity_byte_demand(body)?,release_bytes:input.next_close_release_byte_demand()?,depth:input.next_close_depth_demand()?.max(1)});}
        if self.raw_bytes.capacity()!=0{return Ok(RetirementDemand {release_bytes:self.raw_bytes.capacity(),depth:1,..Default::default()});}
        if self.chunks.is_some(){return store::artifact_retirement_owned_birth_demands(&self.chunks);}
        if self.completion.is_some()||self.download.is_some()||self.credit.is_some()||self.snapshot.is_some()||self.snapshot_close.is_some(){return Ok(item);}
        Ok(Default::default())
    }
    fn close_turn(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        let demand=self.close_demands(grant.maximum_copy_bytes)?;
        if demand==RetirementDemand::default(){return Ok(Default::default());}
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"raster export close exceeds its admitted depth"));}
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(Default::default());}
        let item=RetainedCloneProgress {copied_items:1,..Default::default()};
        let child=RetainedCloneGrant {maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if !self.publication.terminal_is_empty(){return self.publication.close_step(child).map(|step|step.progress());}
        if self.delivered{self.delivered=false;return Ok(item);}
        if self.active.is_some(){return store::artifact_retirement_box_close_step(&mut self.active,grant).map(|step|step.progress());}
        if self.rejected_download.is_some(){return store::artifact_retirement_admit_owned(&mut self.rejected_download,&mut self.active,grant).map(|step|step.progress());}
        if let Some(mut job)=self.work.preparing.take(){job.cancel();return Ok(item);}
        if let Some(mut job)=self.work.compositing.take(){job.cancel();return Ok(item);}
        if let Some(mut job)=self.work.encoding.take(){job.cancel();return Ok(item);}
        if self.encoded_retiring.is_none()&&self.work.encoded.capacity()!=0{self.encoded_retiring=Some(std::mem::take(&mut self.work.encoded));}
        if self.encoded_retiring.is_some(){return store::artifact_retirement_admit_owned(&mut self.encoded_retiring,&mut self.active,grant).map(|step|step.progress());}
        if let Some(input)=self.raw_input.as_mut(){
            let step=input.close_step(grant);
            if input.terminal_is_empty(){self.raw_input=None;}
            return match step {InteractiveJobCloseStep::Pending {progress}|InteractiveJobCloseStep::Complete {progress}=>Ok(progress),InteractiveJobCloseStep::Refused {kind,progress}=>Err(ValueError::literal(kind,"raster export wire close refused").with_retained_progress(progress)),InteractiveJobCloseStep::Blocked=>Ok(Default::default())};
        }
        if self.raw_bytes.capacity()!=0{let released=std::mem::take(&mut self.raw_bytes).capacity();return Ok(RetainedCloneProgress {released_bytes:released,..item});}
        if self.chunks.is_some(){return store::artifact_retirement_admit_owned(&mut self.chunks,&mut self.active,grant).map(|step|step.progress());}
        if self.completion.take().is_some()||self.download.take().is_some()||self.credit.take().is_some(){return Ok(item);}
        if let Some(snapshot)=self.snapshot.as_ref(){
            if !self.snapshot_close.as_ref().map_or_else(||Arc::strong_count(snapshot)>1,|lease|lease.can_release(snapshot)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"raster export snapshot has no live close witness"));}
            self.snapshot=None;return Ok(item);
        }
        self.snapshot_close=None;
        Ok(item)
    }
}
impl InteractiveJob for RasterImageExportJob {
    fn step<'a>(&'a mut self,context:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,ValueError> {
        if self.delivered {
            let step=self.publication.close_step(context.retained_grant())?;
            context.consume_retained(step.progress())?;
            if matches!(step,RetainedCloneStep::Complete(_)){self.delivered=false;}
            return Ok(None);
        }
        if self.closing||context.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(context);}
        if context.should_yield(){return Ok(None);}
        if context.operation()!=self.operation.operation||context.generation()!=self.operation.generation {self.stage_fault("raster.export-operation-authority-invalid");}
        if let Some(kind)=self.publishing {
            let result=self.publication.advance_from_source(kind,&self.source,context)?;
            if result.is_some(){self.delivered=true;if matches!(kind,JobPublicationKind::Checkpoint {..}){self.publishing=None;}}
            return Ok(result);
        }
        if !self.raw_validated {
            context.set_stage("prepare");context.consume_fuel(1);
            if let Some(page)=self.raw_input.as_ref().and_then(|input|input.page(self.raw_cursor)) {self.raw_bytes.extend_from_slice(page);self.raw_cursor+=1;return Ok(None);}
            if !matches!(<RasterCommand as protocol::OpBinary>::decode_op(&self.raw_bytes),Ok(RasterCommand::ExportPng(_))) {self.stage_fault("raster.export-command-mismatch");return Ok(None);}
            self.raw_validated=true;return Ok(None);
        }
        if self.completed{return JobOutcomeBorrow::admit_complete(context,None,None);}
        let stage=self.work.stage();context.set_stage(stage);
        let result=self.advance();self.units+=1;context.consume_fuel(1);
        if context.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(context);}
        match result {
            Err(error)=>{self.stage_fault(&error.message);Ok(None)},
            Ok(()) if self.completed=>JobOutcomeBorrow::admit_complete(context,None,None),
            Ok(())=>{if self.units==1||self.units.is_multiple_of(64)||self.work.stage()!=stage {let next=self.work.stage();self.stage_progress(next,self.units);}Ok(None)},
        }
    }
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield=>descriptor.yielded(),
            JobOutcomeKind::Cancelled=>descriptor.cancelled(),
            JobOutcomeKind::Complete if self.completed=>descriptor.complete(None,None),
            _=>self.publication.borrow_outcome(descriptor),
        }
    }
    fn begin_close(&mut self){self.closing=true;self.work.closing=true;if let Some(input)=self.raw_input.as_mut(){input.begin_close();}}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError> {Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError> {Ok(self.close_demands(0)?.depth)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->InteractiveJobCloseStep {
        self.begin_close();
        match self.close_turn(grant) {
            Ok(progress)=>if self.terminal_is_empty(){InteractiveJobCloseStep::Complete {progress}}else{InteractiveJobCloseStep::Pending {progress}},
            Err(error)=>InteractiveJobCloseStep::Refused {kind:error.kind,progress:error.retained_progress()},
        }
    }
    fn terminal_is_empty(&self)->bool {self.rejected_download.is_none()&&self.active.is_none()&&self.encoded_retiring.is_none()&&self.raw_input.is_none()&&self.raw_bytes.capacity()==0&&self.download.is_none()&&self.publication.terminal_is_empty()&&!self.delivered&&self.work.terminal_is_empty()&&self.work.encoded.capacity()==0&&self.chunks.is_none()&&self.completion.is_none()&&self.credit.is_none()&&self.snapshot.is_none()&&self.snapshot_close.is_none()}
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
    fn create_job(&mut self,_operation:Operation,payload:Self::Payload)->Result<Self::Job,ToolJobFactoryError>{Ok(payload)}
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for RasterMediaExportJobFactory {
    type Owner=EditorApp<RasterPlayApp>;
    const TOOL_IDS:&'static [&'static str]=&[TOOL_ID];
    const DOCUMENT_SCHEMA:&'static str=crate::RASTER_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS:&'static [ArtifactToolPublicationContract]=&[ArtifactToolPublicationContract {tool_id:TOOL_ID,lanes:&[ArtifactToolPublicationLane::HostOnly]}];
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

/// 🧾️ The command runtime retains its snapshot and completion while this private output queue is built.
pub fn build_download_job(request:ArtifactOwnedToolJobRequest<EditorApp<RasterPlayApp>>)->Result<Option<semio_framework::ToolOperationSpec>,Fault> {
    if request.tool_id!="exportPng"||!matches!(*request.command,RasterCommand::ExportPng(_)){return Err(Fault::from("raster.export-command-mismatch"));}
    let job=RasterImageExportJob {operation:request.operation.clone(),snapshot:Some(request.snapshot),snapshot_close:None,work:ImageExportWork::default(),chunks:Some(ArtifactOutputChunks::new(ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES)),credit:None,completion:None,publication:RetainedJobPublication::new(),publishing:None,source:Vec::new(),delivered:false,completed:false,closing:false,units:0,download:Some(request.completion),raw_input:None,raw_bytes:Vec::new(),raw_cursor:0,raw_validated:true,rejected_download:None,active:None,encoded_retiring:None};
    Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id,request.tool_id,request.payload_schema_id,job,request.operation)))
}

pub struct RasterDownloadJobFactory {keys:[ToolFactoryKey;1]}
impl RasterDownloadJobFactory {pub fn new(controller:&str)->Self {Self {keys:[ToolFactoryKey::new(controller,"exportPng")]}}}
impl ToolJobFactory for RasterDownloadJobFactory {
    type Payload=RasterImageExportJob;type Job=RasterImageExportJob;
    fn keys(&self)->&[ToolFactoryKey]{&self.keys}
    fn payload_schema_id(&self)->&str {"raster.raster.download-png.v1"}
    fn classification(&self)->InteractiveJobClassification {InteractiveJobClassification::Migrated}
    fn execution_contract(&self)->ToolExecutionContract {ToolExecutionContract::resumable(4096,4096,1,ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES,2000,64,1)}
    fn create_job(&mut self,_operation:Operation,payload:Self::Payload)->Result<Self::Job,ToolJobFactoryError>{Ok(payload)}
    fn create_job_from_wire_pages_with_payload(&mut self,_operation:Operation,mut payload:Self::Payload,input:RetainedToolWireInput,checkpoint:Option<RetainedToolWireInput>)->Result<Self::Job,(ToolJobFactoryError,RetainedToolWireInput,Option<RetainedToolWireInput>)> {
        if input.declared_bytes()>4096||checkpoint.is_some(){return Err((ToolJobFactoryError::new("Raster export rejects oversized command or checkpoint"),input,checkpoint));}
        payload.raw_bytes=Vec::with_capacity(input.declared_bytes());payload.raw_input=Some(input);payload.raw_validated=false;Ok(payload)
    }
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for RasterDownloadJobFactory {
    type Owner=EditorApp<RasterPlayApp>;
    const TOOL_IDS:&'static [&'static str]=&["exportPng"];
    const DOCUMENT_SCHEMA:&'static str=crate::RASTER_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS:&'static [ArtifactToolPublicationContract]=&[ArtifactToolPublicationContract {tool_id:"exportPng",lanes:&[ArtifactToolPublicationLane::HostOnly]}];
}
/// 🧹️ Relinquishes a shared alias atomically and hands the final value to Raster's bounded retirement.
#[derive(Default)]
pub(super) struct RasterExportSnapshotDisposer {
    retirement:Option<Box<dyn store::ErasedSnapshotRetirement>>,
}

impl semio_framework_plugin::ArtifactSnapshotDisposer<crate::RasterSnapshot> for RasterExportSnapshotDisposer {
    fn retirement_demands(&self,snapshot:&Option<Arc<crate::RasterSnapshot>>,body:usize)->Result<RetirementDemand,ValueError> {
        if let Some(owner)=self.retirement.as_ref(){return store::artifact_retirement_box_demands(owner,body);}
        let Some(owner)=snapshot.as_ref() else{return Ok(Default::default())};
        if Arc::strong_count(owner)!=1{return Ok(RetirementDemand {depth:1,..Default::default()});}
        Ok(RetirementDemand {capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<crate::RasterSnapshot>(),depth:2,..Default::default()})
    }

    fn close_step(&mut self,snapshot:&mut Option<Arc<crate::RasterSnapshot>>,grant:RetainedCloneGrant)->Result<semio_framework_plugin::PluginLifecycleStep,Fault> {
        use semio_framework_plugin::PluginLifecycleStep;
        let fault=|error:ValueError|Fault::from(error.into_message());
        let demand=self.retirement_demands(snapshot,grant.maximum_copy_bytes).map_err(fault)?;
        if demand==RetirementDemand::default(){return Ok(PluginLifecycleStep::Complete(Default::default()));}
        if grant.maximum_depth<demand.depth{return Err(Fault::from("raster.export-snapshot-depth-limit"));}
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        if self.retirement.is_some(){return store::artifact_retirement_box_close_step(&mut self.retirement,grant).map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(fault);}
        if snapshot.as_ref().is_some_and(|owner|Arc::strong_count(owner)!=1){snapshot.take();return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));}
        let mut pending=snapshot.take().and_then(Arc::into_inner);
        let step=store::artifact_retirement_admit_owned(&mut pending,&mut self.retirement,grant);
        if let Some(original)=pending{*snapshot=Some(Arc::new(original));}
        step.map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(fault)
    }

    fn terminal_is_empty(&self,snapshot:&Option<Arc<crate::RasterSnapshot>>)->bool {snapshot.is_none()&&self.retirement.is_none()}
}

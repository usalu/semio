//! 📤️ Cancellable layer-to-PNG export with sealed, credited output pages.
use crate::RasterSnapshot;
use crate::standards::v1::subsets::any::io::RasterStackPreparation;
use crate::editor::raster::{RasterPlayApp,RasterCommand};
use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework_plugin::{ArtifactOwnedToolJobRequest,ArtifactToolCompletion,ArtifactDownloadOutput,EphemeralEmit};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_pixels::{compositing::layers::RasterStackJob,png_encoding::PngEncodeJob};
use semio_framework_job::{Checkpoint,CommitCandidate,InteractiveJob,InteractiveJobCloseStep,JobFault,JobPayloadCloseStep,JobPayloadStream,Operation,RetainedJobPayload,RetainedJobPayloadWriter,StepContext,StepOutcome};
use semio_framework_plugin::{ArtifactMediaExportJobRequest,ArtifactReservedJob,ArtifactReservedToolJob,ArtifactToolPublicationContract,ArtifactToolPublicationLane,EditorApp,Fault,MediaClass,MediaForm,MediaType,PluginCloseStep};
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
    fn close_step(&mut self,maximum_bytes:usize)->usize {
        self.closing=true;
        if let Some(mut job)=self.preparing.take(){job.cancel();return 0;}
        if let Some(mut job)=self.compositing.take(){job.cancel();return 0;}
        if let Some(mut job)=self.encoding.take(){job.cancel();return 0;}
        let released=self.encoded.len().min(maximum_bytes);self.encoded.truncate(self.encoded.len()-released);if self.encoded.is_empty(){self.encoded=Vec::new();}released
    }
    fn terminal_is_empty(&self)->bool {self.preparing.is_none()&&self.compositing.is_none()&&self.encoding.is_none()&&self.encoded.is_empty()}
}

struct Publication {writer:Option<RetainedJobPayloadWriter>,bytes:Vec<u8>,cursor:usize,progress:Option<u64>}
impl Publication {
    fn progress(stage:&str,completed:u64)->Self {
        let (en,de)=match stage {"prepare"=>("Preparing layers","Ebenen werden vorbereitet"),"composite"=>("Rendering layers","Ebenen werden gerendert"),"encode"=>("Encoding PNG","PNG wird kodiert"),_=>("Preparing download","Download wird vorbereitet")};
        let bytes=format!(r#"{{"stage":"{stage}","en":"{en}","de":"{de}","completedUnits":{completed}}}"#).into_bytes();
        Self {writer:Some(RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState)),bytes,cursor:0,progress:Some(completed)}
    }
    fn fault(error:&str)->Self {Self {writer:Some(RetainedJobPayloadWriter::new(JobPayloadStream::Fault)),bytes:error.chars().take(256).collect::<String>().into_bytes(),cursor:0,progress:None}}
    fn step(&mut self,context:&mut StepContext<'_>)->StepOutcome {
        let writer=self.writer.as_mut().unwrap();
        match writer.write_slice_page(context,&self.bytes,&mut self.cursor) {
            Ok(true)=>match self.writer.take().unwrap().finish(){Ok(payload)=>match self.progress{Some(applied_progress)=>StepOutcome::CheckpointReady(Checkpoint {state:payload,applied_progress}),None=>StepOutcome::Fault(JobFault {detail:payload})},Err(writer)=>{self.writer=Some(writer);StepOutcome::Yield}},
            _=>StepOutcome::Yield,
        }
    }
}

pub struct RasterImageExportJob {
    operation:Operation,snapshot:Option<Arc<RasterSnapshot>>,snapshot_close:Option<ArtifactSnapshotCloseLease<RasterSnapshot>>,work:ImageExportWork,
    chunks:Option<ArtifactOutputChunks>,credit:Option<ArtifactMediaExportCredit>,completion:Option<ArtifactMediaExportCompletion>,publication:Option<Publication>,completed:bool,closing:bool,units:u64,
    rejected_download:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    download:Option<ArtifactToolCompletion<EditorApp<RasterPlayApp>>>,private_chunks:bool,raw_input:Option<RetainedToolWireInput>,raw_bytes:Vec<u8>,raw_cursor:usize,raw_validated:bool,
}
impl RasterImageExportJob {
    pub fn new(request:ArtifactMediaExportJobRequest<EditorApp<RasterPlayApp>>)->Self {
        Self {operation:request.operation,snapshot:Some(request.snapshot),snapshot_close:Some(request.snapshot_close),work:ImageExportWork::default(),chunks:Some(request.output_chunks),credit:Some(request.output_credit),completion:Some(request.completion),publication:None,completed:false,closing:false,units:0,rejected_download:None,download:None,private_chunks:false,raw_input:None,raw_bytes:Vec::new(),raw_cursor:0,raw_validated:true}
    }
    fn advance(&mut self)->Result<(),Fault> {
        let chunk=self.work.advance(self.snapshot.as_deref().ok_or_else(||Fault::from("raster.export-snapshot-missing"))?,1).map_err(Fault::from)?;
        if let Some(chunk)=chunk {if let Some(credit)=self.credit.as_ref(){credit.credit(chunk.len())?;}self.chunks.as_ref().unwrap().push(chunk)?;}
        if self.work.done {
            self.chunks.as_ref().unwrap().seal()?;
            if let Some(completion)=self.download.as_ref() {
                if let Err(rejected)=completion.complete_download(ArtifactDownloadOutput::new("image.png","image/png",Some("base64".into()),self.chunks.take().unwrap()),EphemeralEmit::default()){
                    if let Ok(download)=rejected.download{self.rejected_download=Some(semio_framework_value::retirement::owned_retirement(download));}return Err(rejected.fault);
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
}
impl InteractiveJob for RasterImageExportJob {
    fn step(&mut self,context:&mut StepContext<'_>)->StepOutcome {
        if self.closing||context.is_cancelled(){return StepOutcome::Cancelled;}
        if context.should_yield(){return StepOutcome::Yield;}
        if context.operation()!=self.operation.operation||context.generation()!=self.operation.generation {return StepOutcome::Fault(JobFault {detail:RetainedJobPayload::empty(JobPayloadStream::Fault)});}
        if let Some(publication)=self.publication.as_mut(){let outcome=publication.step(context);if !matches!(outcome,StepOutcome::Yield){self.publication=None;}return outcome;}
        if !self.raw_validated {
            context.set_stage("prepare");context.consume_fuel(1);
            if let Some(page)=self.raw_input.as_ref().and_then(|input|input.page(self.raw_cursor)) {self.raw_bytes.extend_from_slice(page);self.raw_cursor+=1;return StepOutcome::Yield;}
            if !matches!(<RasterCommand as protocol::OpBinary>::decode_op(&self.raw_bytes),Ok(RasterCommand::ExportPng(_))) {self.publication=Some(Publication::fault("raster.export-command-mismatch"));return StepOutcome::Yield;}
            self.raw_validated=true;return StepOutcome::Yield;
        }
        if self.completed{return StepOutcome::Fault(JobFault {detail:RetainedJobPayload::empty(JobPayloadStream::Fault)});}
        let stage=self.work.stage();context.set_stage(stage);
        let result=self.advance();self.units+=1;context.consume_fuel(1);
        if context.is_cancelled(){return StepOutcome::Cancelled;}
        match result {
            Err(error)=>{self.publication=Some(Publication::fault(&error.message));StepOutcome::Yield},
            Ok(()) if self.completed=>StepOutcome::Complete(CommitCandidate {state:RetainedJobPayload::empty(JobPayloadStream::CommitState),output:RetainedJobPayload::empty(JobPayloadStream::CommitOutput)}),
            Ok(())=>{if self.units==1||self.units.is_multiple_of(64)||self.work.stage()!=stage {self.publication=Some(Publication::progress(self.work.stage(),self.units));}StepOutcome::Yield},
        }
    }
    fn begin_close(&mut self){self.closing=true;self.work.closing=true;if let Some(input)=self.raw_input.as_mut(){input.begin_close();}if let Some(writer)=self.publication.as_mut().and_then(|publication|publication.writer.as_mut()){writer.begin_close();}}
    fn close_step(&mut self,items:usize,bytes:usize)->InteractiveJobCloseStep {
        match ArtifactReservedJob::close_step(self,items,bytes){Ok(PluginCloseStep::Complete)=>InteractiveJobCloseStep::Complete,Ok(PluginCloseStep::Pending {released_items,released_bytes})=>InteractiveJobCloseStep::Pending {released_items,released_bytes},_=>InteractiveJobCloseStep::Blocked}
    }
    fn terminal_is_empty(&self)->bool {ArtifactReservedJob::terminal_is_empty(self)}
}
impl ArtifactReservedJob for RasterImageExportJob {
    /// ♻️ Framework operation ownership retains output queues and completion until its later drain stage.
    fn close_step(&mut self,items:usize,bytes:usize)->Result<PluginCloseStep,Fault> {
        self.begin_close();
        if let Some(retirement)=self.rejected_download.as_mut(){return match retirement.close_step(items,bytes){Ok(semio_framework_value::SnapshotRetirementStep::Complete)=>{self.rejected_download.take();Ok(PluginCloseStep::Pending{released_items:1,released_bytes:0})},Ok(semio_framework_value::SnapshotRetirementStep::Pending{released_items,released_bytes})=>Ok(PluginCloseStep::Pending{released_items,released_bytes}),_=>Err(Fault::from("raster.export-rejected-download-retirement-blocked"))};}
        if items==0{return Ok(PluginCloseStep::Pending {released_items:0,released_bytes:0});}
        if let Some(publication)=self.publication.as_mut() {
            if let Some(writer)=publication.writer.as_mut(){let step=writer.close_step(1,bytes);if writer.terminal_is_empty(){publication.writer=None;}return Ok(match step{JobPayloadCloseStep::Pending {released_items,released_bytes}=>PluginCloseStep::Pending {released_items,released_bytes},_=>PluginCloseStep::Pending {released_items:0,released_bytes:0}});}
            if publication.bytes.len()>bytes {return Ok(PluginCloseStep::Pending {released_items:0,released_bytes:0});}
            let released_bytes=publication.bytes.len();self.publication=None;return Ok(PluginCloseStep::Pending {released_items:1,released_bytes});
        }
        if !self.work.terminal_is_empty(){if bytes==0{return Ok(PluginCloseStep::Pending {released_items:0,released_bytes:0});}let released_bytes=self.work.close_step(bytes);return Ok(PluginCloseStep::Pending {released_items:1,released_bytes});}
        if let Some(input)=self.raw_input.as_mut() {
            let step=input.close_step(1,bytes);if input.terminal_is_empty(){self.raw_input=None;}
            return Ok(match step {InteractiveJobCloseStep::Pending {released_items,released_bytes}=>PluginCloseStep::Pending {released_items,released_bytes},InteractiveJobCloseStep::Complete=>PluginCloseStep::Pending {released_items:1,released_bytes:0},_=>return Err(Fault::from("raster.export-wire-close-blocked"))});
        }
        if !self.raw_bytes.is_empty(){let released_bytes=bytes.min(self.raw_bytes.len());self.raw_bytes.truncate(self.raw_bytes.len()-released_bytes);if self.raw_bytes.is_empty(){self.raw_bytes=Vec::new();}return Ok(PluginCloseStep::Pending {released_items:0,released_bytes});}
        if self.private_chunks {if let Some(chunks)=self.chunks.as_ref(){if chunks.chunks_remaining()>0 {if bytes<ArtifactOutputChunks::CHUNK_BYTES{return Ok(PluginCloseStep::Pending {released_items:0,released_bytes:0});}let chunk=chunks.close_take_chunk()?;return Ok(PluginCloseStep::Pending {released_items:1,released_bytes:chunk.map_or(0,|chunk|chunk.len())});}}}
        if self.chunks.take().is_some(){return Ok(PluginCloseStep::Pending {released_items:1,released_bytes:0});}
        if self.completion.take().is_some(){return Ok(PluginCloseStep::Pending {released_items:1,released_bytes:0});}
        if self.download.take().is_some(){return Ok(PluginCloseStep::Pending {released_items:1,released_bytes:0});}
        if self.credit.take().is_some(){return Ok(PluginCloseStep::Pending {released_items:1,released_bytes:0});}
        if let Some(snapshot)=self.snapshot.as_ref(){if !self.snapshot_close.as_ref().map_or_else(||Arc::strong_count(snapshot)>1,|lease|lease.can_release(snapshot)){return Err(Fault::from("raster.export-snapshot-unwitnessed"));}self.snapshot=None;return Ok(PluginCloseStep::Pending {released_items:1,released_bytes:0});}
        self.snapshot_close=None;Ok(PluginCloseStep::Complete)
    }
    fn terminal_is_empty(&self)->bool {self.rejected_download.is_none()&&self.raw_input.is_none()&&self.raw_bytes.is_empty()&&self.download.is_none()&&self.publication.is_none()&&self.work.terminal_is_empty()&&self.chunks.is_none()&&self.completion.is_none()&&self.credit.is_none()&&self.snapshot.is_none()&&self.snapshot_close.is_none()}
}

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
    let job=RasterImageExportJob {operation:request.operation.clone(),snapshot:Some(request.snapshot),snapshot_close:None,work:ImageExportWork::default(),chunks:Some(ArtifactOutputChunks::new(ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES)),credit:None,completion:None,publication:None,completed:false,closing:false,units:0,download:Some(request.completion),private_chunks:true,raw_input:None,raw_bytes:Vec::new(),raw_cursor:0,raw_validated:true,rejected_download:None};
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
    fn close_step(&mut self,snapshot:&mut Option<std::sync::Arc<crate::RasterSnapshot>>,maximum_items:usize,maximum_bytes:usize)->Result<semio_framework_plugin::PluginCloseStep,Fault> {
        use semio_framework_plugin::PluginCloseStep;
        if maximum_items==0{return Ok(PluginCloseStep::Pending {released_items:0,released_bytes:0});}
        if let Some(retirement)=self.retirement.as_mut(){
            return Ok(match retirement.close_step(1,maximum_bytes).map_err(|error|Fault::from(error.message))? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty()=>{drop(self.retirement.take());PluginCloseStep::Pending {released_items:1,released_bytes:0}},
                store::SnapshotRetirementStep::Complete=>return Err(Fault::from("raster export snapshot reported false terminal")),
                store::SnapshotRetirementStep::Pending {released_items,released_bytes}=>PluginCloseStep::Pending {released_items,released_bytes},
                store::SnapshotRetirementStep::Blocked=>PluginCloseStep::Blocked {reason:"raster export snapshot retirement awaits its exact owner"},
            });
        }
        let Some(owner)=snapshot.take() else{return Ok(PluginCloseStep::Complete)};
        if let Some(value)=std::sync::Arc::into_inner(owner){
            self.retirement=Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&crate::host::owned::RasterSnapshotRetirementFactory,value));
            return Ok(PluginCloseStep::Pending {released_items:0,released_bytes:0});
        }
        Ok(PluginCloseStep::Pending {released_items:1,released_bytes:0})
    }

    fn terminal_is_empty(&self,snapshot:&Option<std::sync::Arc<crate::RasterSnapshot>>)->bool {snapshot.is_none()&&self.retirement.is_none()}
}

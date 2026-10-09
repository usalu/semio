//! 🧵️ Imports one privately decoded image through cooperative work and explicit child retirement.
use super::*;
use crate::standards::v1::subsets::any::io::image::DrawingImageAdmissionJob;
use semio_framework_plugin::retained_command::{ArtifactCommandWork,ArtifactCommandInputs,ArtifactCommandWorkStep};
use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneStep},ValueError};
type App=semio_framework_plugin::EditorApp<DrawingPlayApp>;
#[derive(semio_framework_value::RetireOwned)]
struct Workspace {source:String,job:Option<DrawingImageAdmissionJob>}
struct Work {workspace:Option<ControlledRetirement<Workspace>>,identity:Option<usize>,complete:bool,closing:bool}
impl Work {fn new()->Self {Self {workspace:None,identity:None,complete:false,closing:false}}}
impl ArtifactCommandWork<App> for Work {
 fn tool_id(&self)->&'static str {"importImage"}
 fn extent(&self,command:&DrawingCommand,snapshot:&DrawingSnapshot,_interaction:&protocol::InteractionState,_context:Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<App>>)->Option<usize> {
  let DrawingCommand::ImportImage(payload)=command else{return None};
  (!self.complete&&import_image::validate(payload).is_ok()&&snapshot.layers.len()+snapshot.assets.len()<=4096).then_some(1)
 }
 fn step(&mut self,input:&ArtifactCommandInputs<'_,App>,cx:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactCommandWorkStep<App>,Fault> {
  if self.complete||self.closing {return Err(Fault::from("drawing-image-import-terminal"));}
  if cx.is_cancelled() {return Err(Fault::from("Image import cancelled"));}
  let DrawingCommand::ImportImage(payload)=input.command else{return Err(Fault::from("drawing-image-command-required"));};import_image::validate(payload)?;
  let identity=input.snapshot as*const DrawingSnapshot as usize;if self.identity.is_some_and(|old|old!=identity){return Err(Fault::from("Image import source changed"));}self.identity=Some(identity);
  let retained=self.workspace.get_or_insert_with(||ControlledRetirement::new(Workspace {source:String::new(),job:None}).unwrap_or_else(|_|unreachable!()));
  let workspace=retained.original_mut().ok_or_else(||Fault::from("drawing-image-retirement-started"))?;
  while !cx.should_yield() {
   if cx.is_cancelled(){return Err(Fault::from("Image import cancelled"));}
   if workspace.job.is_none() {
    cx.set_stage("image-source");
    if workspace.source.len()<payload.payload.len(){let byte=payload.payload.as_bytes()[workspace.source.len()];if !byte.is_ascii(){return Err(Fault::from("Image source must contain ASCII encoded bytes"));}workspace.source.try_reserve(1).map_err(|_|Fault::from("Image source allocation refused"))?;workspace.source.push(byte as char);cx.consume_fuel(1);continue;}
    workspace.job=Some(DrawingImageAdmissionJob::new(semio_framework_pixels::image_decoding::ImageDecodeInput {mime:"image/png".into(),data:std::sync::Arc::new(std::mem::take(&mut workspace.source)),max_source_bytes:import_image::MAX_SOURCE_BYTES,max_bytes:67108864,max_pixels:import_image::MAX_PIXELS,max_chunks:65536}).map_err(|error|Fault::from(error.to_string()))?);cx.consume_fuel(1);continue;
   }
   let job=workspace.job.as_mut().unwrap();let progress=job.advance(1).map_err(|error|Fault::from(error.to_string()))?;cx.set_stage(if progress.samples>0 {"image-samples"}else{progress.decoding.map_or("image-decode",|progress|progress.phase)});cx.consume_fuel(1);
   if progress.done {let asset=job.take_result().map_err(|error|Fault::from(error.to_string()))?;let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|!cx.is_cancelled();let mut control=semio_framework_value::NativeEncodeControl::new(1024*1024,&mut observer);let emit=import_image::publish(input.snapshot,input.operation,payload,asset,&mut control)?;self.complete=true;return Ok(ArtifactCommandWorkStep::Complete(emit));}
  }
  Ok(ArtifactCommandWorkStep::Progress {stage:"image-import",preview:b""})
 }
 fn begin_close(&mut self){self.closing=true;}
 fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep {use semio_framework_job::InteractiveJobCloseStep;if !self.closing{return InteractiveJobCloseStep::Blocked;}let Some(workspace)=self.workspace.as_mut() else{return InteractiveJobCloseStep::Complete {progress:Default::default()};};match workspace.step(grant){Ok(RetainedCloneStep::Complete(progress))=>InteractiveJobCloseStep::Complete {progress},Ok(RetainedCloneStep::Progress(progress))=>InteractiveJobCloseStep::Pending {progress},Err(error)=>InteractiveJobCloseStep::Refused(error.kind)}}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.workspace.as_ref().map_or(Ok(0),ControlledRetirement::next_copy_byte_demand)}
 fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.workspace.as_ref().map_or(Ok(0),|workspace|workspace.next_capacity_byte_demand(copy))}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.workspace.as_ref().map_or(Ok(0),ControlledRetirement::next_release_byte_demand)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.workspace.as_ref().map_or(Ok(0),ControlledRetirement::next_depth_demand)}
 fn terminal_frame_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
 fn terminal_is_empty(&self)->bool {self.closing&&self.workspace.as_ref().is_none_or(ControlledRetirement::terminal_is_empty)}
}
pub(super) struct DrawingImageCommandJobFactory {keys:Vec<semio_framework::ToolFactoryKey>}
impl DrawingImageCommandJobFactory {pub(super) fn new(controller:&str)->Self {Self {keys:vec![semio_framework::ToolFactoryKey::new(controller,"importImage")]}}}
impl semio_framework::ToolJobFactory for DrawingImageCommandJobFactory {
    type Payload=semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<App>;
    type Job=semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<App>;
    fn keys(&self)->&[semio_framework::ToolFactoryKey] {&self.keys}
    fn payload_schema_id(&self)->&str {DRAWING_BOUNDED_PAYLOAD_SCHEMA}
    fn classification(&self)->semio_framework::InteractiveJobClassification {semio_framework::InteractiveJobClassification::Migrated}
    fn execution_contract(&self)->semio_framework::ToolExecutionContract {semio_framework::ToolExecutionContract::resumable(DRAWING_IMAGE_RAW_BYTES,4096,1,262144,7500,1,1)}
    fn create_job(&mut self,_operation:semio_framework_job::Operation,payload:Self::Payload)->Result<Self::Job,semio_framework::ToolJobFactoryError> {Ok(Self::Job::new(payload))}
    fn create_job_from_wire_pages_with_payload(&mut self,_operation:semio_framework_job::Operation,payload:Self::Payload,input:semio_framework::action_bus::RetainedToolWireInput,checkpoint:Option<semio_framework::action_bus::RetainedToolWireInput>)->Result<Self::Job,(semio_framework::ToolJobFactoryError,semio_framework::action_bus::RetainedToolWireInput,Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some()||input.declared_bytes()>DRAWING_IMAGE_RAW_BYTES {return Err((semio_framework::ToolJobFactoryError::new("Image import ingress rejects oversized wire or checkpoint"),input,checkpoint));}
        Ok(Self::Job::from_wire(payload,input))
    }
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for DrawingImageCommandJobFactory {
    type Owner=App;
    const TOOL_IDS:&'static[&'static str]=DRAWING_IMAGE_TOOL_IDS;
    const DOCUMENT_SCHEMA:&'static str=DRAWING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS:&'static[semio_framework_plugin::ArtifactToolPublicationContract]=DRAWING_IMAGE_PUBLICATION_CONTRACTS;
}
pub(super) fn build(request:semio_framework_plugin::ArtifactOwnedToolJobRequest<App>)->Result<semio_framework::ToolOperationSpec,Fault> {
    let work=Work::new();
    if request.tool_id!="importImage"||work.extent(&request.command,&request.snapshot,&request.interaction_state,Some(&request.context))!=Some(1) {return Err(Fault::from("Image import exceeds its registered source capacity"));}
    let operation=semio_framework_plugin::AppOperationContext {app_instance_id:request.app_instance_id,parent_document_id:request.parent_document_id.clone(),operation_id:request.operation.operation.0,generation:request.operation.generation.0,canonical_base_revision:request.canonical_base_revision,authoring_seed:request.authoring_seed.clone()};
    let payload=semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {command:*request.command,snapshot:request.snapshot,config:request.config,history:request.history,interaction_state:request.interaction_state,interaction_hover:request.interaction_hover,context:Some(request.context),operation,completion:request.completion},DrawingCommand::command_id,DRAWING_IMAGE_RAW_BYTES,4096,Box::new(work));
    Ok(semio_framework::ToolOperationSpec::new(request.controller_id,request.tool_id,request.payload_schema_id,payload,request.operation))
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

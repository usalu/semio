//! 🧵️ Imports one privately decoded image through cooperative work and explicit child retirement.
use super::*;
use crate::DrawingImageAsset;
use crate::standards::v1::subsets::any::io::image::DrawingImageAdmissionJob;
use import_image::publication::DrawingImagePublicationJob;
use semio_framework_plugin::retained_command::{ArtifactCommandWork,ArtifactCommandInputs,ArtifactCommandWorkStep};
use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneStep},ValueError};
type App=semio_framework_plugin::EditorApp<DrawingPlayApp>;
/// 🪙️ Whether the step wallet still funds one whole work turn quoted as `demand`.
fn can_enter(grant:RetainedCloneGrant,demand:semio_framework_value::RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
#[derive(semio_framework_value::RetireOwned)]
struct Workspace {job:Option<DrawingImageAdmissionJob>,publication:Option<DrawingImagePublicationJob>}
struct Work {workspace:Option<ControlledRetirement<Workspace>>,identity:Option<usize>,complete:bool,closing:bool}
impl Work {fn new()->Self {Self {workspace:None,identity:None,complete:false,closing:false}}}
impl ArtifactCommandWork<App> for Work {
 fn tool_id(&self)->&'static str {"importImage"}
 fn extent(&self,command:&DrawingCommand,snapshot:&DrawingSnapshot,_interaction:&protocol::InteractionState,_context:Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<App>>)->Option<usize> {
  let DrawingCommand::ImportImage(payload)=command else{return None};
  (!self.complete&&import_image::validate(payload).is_ok()&&snapshot.layers.len()+snapshot.assets.len()<=4096).then_some(1)
 }
 fn work_demands(&self,input:&ArtifactCommandInputs<'_,App>,_maximum_copy_bytes:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
  use semio_framework_value::{RetirementDemand,ValueRefusalKind};
  let DrawingCommand::ImportImage(payload)=input.command else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"drawing image command required"));};
  let Some(workspace)=self.workspace.as_ref().and_then(ControlledRetirement::original)else{return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Workspace>()+std::mem::size_of::<DrawingImageAdmissionJob>(),depth:1,..Default::default()});};
  if let Some(publication)=&workspace.publication{let mut demand=publication.work_demands(payload,input.operation,_maximum_copy_bytes)?;if publication.ready(){demand.copy_bytes+=std::mem::size_of::<Emit<DrawingMutation,NoConfigMutation>>();}return Ok(demand);}
  let job=workspace.job.as_ref().unwrap();if job.progress().done{return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<DrawingImageAsset>()+std::mem::size_of::<DrawingImagePublicationJob>(),depth:1,..Default::default()});}
  Ok(RetirementDemand{copy_bytes:job.next_copy_byte_demand().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"drawing image copy quote refused"))?,capacity_bytes:job.next_capacity_byte_demand().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"drawing image capacity quote refused"))?,depth:1,..Default::default()})
 }
 fn step(&mut self,input:&ArtifactCommandInputs<'_,App>,cx:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactCommandWorkStep<App>,Fault> {
  if self.complete||self.closing{return Err(Fault::from("drawing-image-import-terminal"));}
  if cx.is_cancelled(){return Err(Fault::from("Image import cancelled"));}
  let DrawingCommand::ImportImage(payload)=input.command else{return Err(Fault::from("drawing-image-command-required"));};import_image::validate(payload)?;
  let identity=input.snapshot as*const DrawingSnapshot as usize;if self.identity.is_some_and(|old|old!=identity){return Err(Fault::from("Image import source changed"));}
  while !cx.should_yield(){
   let demand=self.work_demands(input,cx.retained_grant().maximum_copy_bytes).map_err(|error|Fault::from(error.into_message()))?;
   if !can_enter(cx.retained_grant(),demand){break;}
   if self.workspace.is_none(){
    let mut owner=ControlledRetirement::new(Workspace{job:None,publication:None}).map_err(|_|Fault::from("Image owner retirement admission refused"))?;
    let job=DrawingImageAdmissionJob::new(semio_framework_pixels::image_decoding::ImageDecodeInput{mime:import_image::source_mime(payload).unwrap(),data:payload.payload.as_str(),max_source_bytes:import_image::MAX_SOURCE_BYTES,max_bytes:67108864,max_pixels:import_image::MAX_PIXELS,max_chunks:65536}).map_err(|_|Fault::from("Image decoder admission refused"));
    match job{Ok(job)=>owner.original_mut().unwrap().job=Some(job),Err(error)=>{self.workspace=Some(owner);return Err(error);}}
    self.workspace=Some(owner);self.identity=Some(identity);cx.consume_retained(semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}).map_err(|error|Fault::from(error.into_message()))?;cx.consume_fuel(1);continue;
   }
   let workspace=self.workspace.as_mut().unwrap().original_mut().ok_or_else(||Fault::from("drawing-image-retirement-started"))?;
   let grant=RetainedCloneGrant{maximum_items:1,..cx.retained_grant()};
   if let Some(publication)=&mut workspace.publication{
    if publication.ready(){let extra=std::mem::size_of::<Emit<DrawingMutation,NoConfigMutation>>();let Some((output,mut receipt))=publication.take_result(payload,input.operation,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-extra,..grant}).map_err(|error|Fault::from(error.into_message()))?else{break;};let mut emit=Emit::default();emit.artifact_mutations=output.mutations;emit.interaction_writes=output.selection;receipt.copied_bytes+=extra;cx.consume_retained(receipt).map_err(|error|Fault::from(error.into_message()))?;cx.consume_fuel(1);self.complete=true;return Ok(ArtifactCommandWorkStep::Complete(emit));}
    let receipt=publication.advance(input.snapshot,payload,input.operation,grant).map_err(|error|Fault::from(error.into_message()))?;cx.consume_retained(receipt).map_err(|error|Fault::from(error.into_message()))?;cx.set_stage("image-publication");cx.consume_fuel(1);if receipt.copied_items==0{break;}continue;
   }
   if workspace.job.as_ref().unwrap().progress().done{let Some((asset,mut receipt))=workspace.job.as_mut().unwrap().take_result(grant).map_err(|_|Fault::from("Image asset handoff refused"))?else{break;};workspace.publication=Some(DrawingImagePublicationJob::new(asset,input.operation));receipt.copied_bytes+=std::mem::size_of::<DrawingImagePublicationJob>();cx.consume_retained(receipt).map_err(|error|Fault::from(error.into_message()))?;cx.consume_fuel(1);continue;}
   let(progress,receipt)=workspace.job.as_mut().unwrap().advance(payload.payload.as_str(),grant).map_err(|_|Fault::from("Image decoder work refused"))?;
   cx.consume_retained(receipt).map_err(|error|Fault::from(error.into_message()))?;cx.set_stage(if progress.samples>0{"image-samples"}else{progress.decoding.map_or("image-decode",|progress|progress.phase)});cx.consume_fuel(1);
   if receipt.copied_items==0{break;}
  }
  Ok(ArtifactCommandWorkStep::Progress{stage:"image-import",preview:b""})
 }
 fn begin_close(&mut self){self.closing=true;}
 fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep {use semio_framework_job::InteractiveJobCloseStep;if !self.closing{return InteractiveJobCloseStep::Blocked;}let Some(workspace)=self.workspace.as_mut() else{return InteractiveJobCloseStep::Complete {progress:Default::default()};};match workspace.step(grant){Ok(RetainedCloneStep::Complete(progress))=>InteractiveJobCloseStep::Complete {progress},Ok(RetainedCloneStep::Progress(progress))=>InteractiveJobCloseStep::Pending {progress},Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}}}
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
    fn execution_contract(&self)->semio_framework::ToolExecutionContract {semio_framework::ToolExecutionContract::resumable(DRAWING_IMAGE_RAW_BYTES,4096,1,262144,7500,1,1).with_retained_work(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:262144,maximum_capacity_bytes:536870912,maximum_release_bytes:0,maximum_depth:128})}
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
    let operation=semio_framework_plugin::AppOperationContext {app_instance_id:request.app_instance_id,parent_document_id:request.parent_document_id.clone(),operation_id:request.operation.operation.0,generation:request.operation.generation.0,canonical_base_revision:request.canonical_base_revision,retained: request.retained,authoring_seed:request.authoring_seed.clone()};
    let payload=semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {command:*request.command,snapshot:request.snapshot,config:request.config,history:request.history,interaction_state:request.interaction_state,interaction_hover:request.interaction_hover,context:Some(request.context),operation,completion:request.completion},DrawingCommand::command_id,DRAWING_IMAGE_RAW_BYTES,4096,Box::new(work));
    Ok(semio_framework::ToolOperationSpec::new(request.controller_id,request.tool_id,request.payload_schema_id,payload,request.operation))
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

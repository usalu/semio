//! 📤️ Owned document export retains immutable source and private segmented output across yielded stages.
use super::*;
use crate::schema::scene_preparation::{DocumentRasterJob,DocumentRasterRetirement,DocumentSceneViewport,DocumentVectorJob,DocumentVectorRetirement,DocumentScenePlan};
use semio_framework_plugin::retained_command::{ArtifactCommandWork,ArtifactCommandInputs,ArtifactCommandWorkStep};
use semio_framework_plugin::app::{ArtifactDownloadOutput,ArtifactOutputChunks};
use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress},ValueError};
use crate::standards::v1::subsets::any::io::export::svg::v1_1::any::write::{SvgWriteJob,SvgWriteRetirement,SvgWriteLimits};
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::pdf::v1_4::any::write::{PdfWriteJob,PdfWriteRetirement,PdfWriteLimits};
use crate::schema::scene_paint::scene::{ScenePaintJob,ScenePaintRetirement,PreparedScene,PaintedSceneLimits};
type App=semio_framework_plugin::EditorApp<DrawingPlayApp>;
const OUTPUT_LIMIT:usize=ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES;
const PAGE_INPUT:usize=ArtifactOutputChunks::CHUNK_BYTES/4*3;
#[derive(semio_framework_value::RetireOwned)]
struct OutputOwners {pdf:Option<PdfWriteJob>,pdf_close:Option<PdfWriteRetirement>,svg:Option<SvgWriteJob>,svg_close:Option<SvgWriteRetirement>,paint:Option<ScenePaintJob>,paint_close:Option<ScenePaintRetirement>,painted:Option<PreparedScene>,plan:Option<DocumentScenePlan>,pages:semio_framework_value::list::PagedList<Vec<u8>,{usize::MAX}>,image:Option<semio_framework_pixels::RasterImage>,encoder:Option<semio_framework_pixels::png_encoding::PngEncodeJob>,bytes:Vec<u8>,chunks:Option<ArtifactOutputChunks>,filename:String,mime:String,failure:Option<String>}
struct Work {vector:Option<DocumentVectorJob<'static>>,vector_close:Option<DocumentVectorRetirement<'static>>,view_box:Option<[f64;4]>,bounds:Option<[f64;4]>,bound_at:usize,page:usize,raster:Option<DocumentRasterJob<'static>>,raster_close:Option<DocumentRasterRetirement<'static>>,read:Option<store::SnapshotRead<DrawingSnapshot>>,output:ControlledRetirement<OutputOwners>,phase:&'static str,cursor:usize,pixel:usize,transparent:bool,complete:bool,closing:bool}
fn extent(document:&DrawingSnapshot,payload:&export_document::ExportDocument)->Result<(u32,u32,f64,f64),Fault>{
 let(world_width,world_height)=document.artboard.as_ref().map_or((1024.0,1024.0),|board|(board.width,board.height));
 if ![world_width,world_height].into_iter().all(|n|n.is_finite()&&n>0.0&&n<=1e9){return Err(Fault::from("Invalid PNG artboard extent"));}
 let width=payload.width.unwrap_or(world_width.ceil()as u32);let height=payload.height.unwrap_or(world_height.ceil()as u32);
 if !(1..=16384).contains(&width)||!(1..=16384).contains(&height)||u64::from(width)*u64::from(height)>67108864{return Err(Fault::from("PNG requested pixel extent exceeds capacity"));}
 Ok((width,height,f64::from(width)/world_width,f64::from(height)/world_height))
}
impl Work {
 fn new(read:store::SnapshotRead<DrawingSnapshot>,payload:&export_document::ExportDocument)->Self{
  let mut failure=(!["png","pdf","svg"].contains(&payload.format.as_str())).then(||"Drawing exports to png, pdf or svg".into());
  let filename=format!("{}.{}",export_document::export_stem(read.get()),payload.format),mime=match payload.format.as_str(){"png"=>"image/png","pdf"=>"application/pdf",_=>"image/svg+xml"}.into();
  let view_box=read.get().artboard.as_ref().map(|board|[0.0,0.0,board.width,board.height]);let(mut vector,mut raster,mut source)=(None,None,Some(read));if matches!(payload.format.as_str(),"svg"|"pdf"){vector=Some(DocumentVectorJob::from_snapshot_read(source.take().unwrap(),geometry_session::limits(),geometry_session::algorithms()));}
  if payload.format=="png"{match extent(source.as_ref().unwrap().get(),payload){Ok((width,height,x,y))=>{let mut producer=DocumentRasterJob::from_snapshot_read(source.take().unwrap(),geometry_session::limits(),DocumentSceneViewport{width,height,origin:[0.0;2],tolerance:0.01,max_pixels:67108864,max_source_bytes:268439552},geometry_session::algorithms());if let Err(error)=producer.set_pixel_scale(x,y){failure=Some(error.to_string());}raster=Some(producer);},Err(error)=>failure=Some(error.to_string())}}
  let output=ControlledRetirement::new(OutputOwners{pdf:None,pdf_close:None,svg:None,svg_close:None,paint:None,paint_close:None,painted:None,plan:None,pages:Default::default(),image:None,encoder:None,bytes:Vec::new(),chunks:Some(ArtifactOutputChunks::new(OUTPUT_LIMIT)),filename,mime,failure}).unwrap_or_else(|_|unreachable!());
  Self{vector,vector_close:None,view_box,bounds:None,bound_at:0,page:0,raster,raster_close:None,read:source,output,phase:"prepare",cursor:0,pixel:0,transparent:payload.transparent.unwrap_or(true),complete:false,closing:false}
 }
}
impl ArtifactCommandWork<App> for Work {
 fn tool_id(&self)->&'static str{"exportDocument"}
 fn extent(&self,command:&DrawingCommand,_snapshot:&DrawingSnapshot,_interaction:&protocol::InteractionState,_context:Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<App>>)->Option<usize>{(!self.complete&&matches!(command,DrawingCommand::ExportDocument(_))).then_some(1)}
 fn step(&mut self,input:&ArtifactCommandInputs<'_,App>,cx:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactCommandWorkStep<App>,Fault>{
  if self.complete||self.closing||cx.is_cancelled(){return Err(Fault::from("Drawing export cancelled or terminal"));}
  let generation=input.operation.generation;let revision=input.operation.canonical_base_revision;
  let current=if let Some(job)=&self.vector{job.snapshot_authority_matches(generation,revision)}else if let Some(close)=&self.vector_close{close.snapshot_authority_matches(generation,revision)}else if let Some(job)=&self.raster{job.snapshot_authority_matches(generation,revision)}else if let Some(close)=&self.raster_close{close.snapshot_authority_matches(generation,revision)}else{self.read.as_ref().is_some_and(|read|read.commit_authority_matches(generation,revision))};
  if !current{return Err(Fault::from("Drawing export source authority changed"));}
  let DrawingCommand::ExportDocument(payload)=input.command else{return Err(Fault::from("Drawing export command required"));};
  if self.phase=="prepare"{self.phase=if self.vector.is_some(){"vector-prepare"}else if self.raster.is_some(){"raster"}else{"invalid"};cx.consume_fuel(1);return Ok(ArtifactCommandWorkStep::Progress{stage:self.phase,preview:b""});}
  let owners=self.output.original_mut().ok_or_else(||Fault::from("Drawing export retirement started"))?;
  if let Some(error)=&owners.failure{return Err(Fault::from(error.as_str()));}
  while !cx.should_yield(){
   if cx.is_cancelled(){return Err(Fault::from("Drawing export cancelled"));}cx.set_stage(self.phase);
   match self.phase {
    "vector-prepare"=>{let progress=self.vector.as_mut().unwrap().advance(1).map_err(|error|Fault::from(error.to_string()))?;cx.consume_fuel(1);if !progress.done{continue;}let(close,plan)=self.vector.take().unwrap().into_retirement();self.vector_close=Some(close);let plan=plan.unwrap();if let Some(view_box)=self.view_box{if payload.format=="pdf"{owners.pdf=Some(PdfWriteJob::new(plan,PdfWriteLimits{view_box,max_output_bytes:OUTPUT_LIMIT/4*3,max_work:1000000000}));self.phase="pdf-write";}else{owners.svg=Some(SvgWriteJob::new(plan,SvgWriteLimits{view_box,max_output_bytes:OUTPUT_LIMIT/4*3,max_work:1000000000}));self.phase="svg-write";}}else{owners.paint=Some(ScenePaintJob::new(plan,0.005,PaintedSceneLimits{max_nodes:1024,max_segments:65536,max_points:262144,max_contours:65536,max_work:1000000000}));self.phase="vector-paint";}},
    "vector-paint"=>{let progress=owners.paint.as_mut().unwrap().advance(1).map_err(Fault::from)?;cx.consume_fuel(1);if progress.done{let(close,painted)=owners.paint.take().unwrap().into_retirement();owners.paint_close=Some(close);owners.painted=painted;self.phase="vector-bounds";}},
    "vector-bounds"=>{let painted=owners.painted.as_mut().unwrap();if self.bound_at<painted.geometry.len(){if painted.plan.nodes[self.bound_at].visible{if let Some(b)=painted.geometry[self.bound_at].bounds.or(painted.geometry[self.bound_at].geometry_bounds){self.bounds=Some(self.bounds.map_or(b,|a|[a[0].min(b[0]),a[1].min(b[1]),a[2].max(b[2]),a[3].max(b[3])]));}}self.bound_at+=1;cx.consume_fuel(1);continue;}let[x1,y1,x2,y2]=self.bounds.unwrap_or([0.0,0.0,1024.0,768.0]);let plan=std::mem::take(&mut painted.plan);let view_box=[x1,y1,(x2-x1).max(1.0),(y2-y1).max(1.0)];if payload.format=="pdf"{owners.pdf=Some(PdfWriteJob::new(plan,PdfWriteLimits{view_box,max_output_bytes:OUTPUT_LIMIT/4*3,max_work:1000000000}));self.phase="pdf-write";}else{owners.svg=Some(SvgWriteJob::new(plan,SvgWriteLimits{view_box,max_output_bytes:OUTPUT_LIMIT/4*3,max_work:1000000000}));self.phase="svg-write";}cx.consume_fuel(1);},
    "pdf-write"=>{let progress=owners.pdf.as_mut().unwrap().advance(1).map_err(Fault::from)?;cx.consume_fuel(1);if progress.done{let(close,pages)=owners.pdf.take().unwrap().into_retirement();owners.pdf_close=Some(close);owners.pages=pages.unwrap();self.phase="output";}},
    "svg-write"=>{let progress=owners.svg.as_mut().unwrap().advance(1).map_err(Fault::from)?;cx.consume_fuel(1);if progress.done{let(close,pages)=owners.svg.take().unwrap().into_retirement();owners.svg_close=Some(close);owners.pages=pages.unwrap();self.phase="output";}},
    "raster"=>{let progress=self.raster.as_mut().unwrap().advance(1).map_err(|error|Fault::from(error.to_string()))?;cx.consume_fuel(1);if !progress.done{continue;}let(close,image)=self.raster.take().unwrap().into_retirement();self.raster_close=Some(close);owners.image=image;self.phase="background";},
    "background"=>{if !self.transparent&&self.pixel<owners.image.as_ref().unwrap().pixels.len(){let image=owners.image.as_mut().unwrap();let at=self.pixel;let alpha=f64::from(image.pixels[at+3]);for channel in 0..3{image.pixels[at+channel]=(f64::from(image.pixels[at+channel])*alpha/255.0+255.0-alpha).round()as u8;}image.pixels[at+3]=255;self.pixel+=4;cx.consume_fuel(1);continue;}owners.encoder=Some(match semio_framework_pixels::png_encoding::PngEncodeJob::with_maximum_bytes(owners.image.take().unwrap(),OUTPUT_LIMIT/4*3){Ok(encoder)=>encoder,Err((error,image))=>{owners.image=Some(image);return Err(Fault::from(error.to_string()));}});self.phase="encode";cx.consume_fuel(1);},
    "encode"=>{let encoder=owners.encoder.as_mut().unwrap();let progress=encoder.advance().map_err(|error|Fault::from(error.to_string()))?;cx.consume_fuel(1);if progress.done{owners.bytes=encoder.take_result().map_err(|error|Fault::from(error.to_string()))?.data;self.phase="output";}},
    "output"=>{if self.page<owners.pages.len(){let page=base64_codec::base64_standard_encode(owners.pages.get(self.page).unwrap()).into_bytes();owners.chunks.as_ref().unwrap().push(page)?;self.page+=1;cx.consume_fuel(1);continue;}if self.cursor<owners.bytes.len(){let end=(self.cursor+PAGE_INPUT).min(owners.bytes.len());let page=base64_codec::base64_standard_encode(&owners.bytes[self.cursor..end]).into_bytes();owners.chunks.as_ref().unwrap().push(page)?;self.cursor=end;cx.consume_fuel(1);continue;}owners.chunks.as_ref().unwrap().seal()?;self.phase="publish";cx.consume_fuel(1);return Ok(ArtifactCommandWorkStep::Progress{stage:self.phase,preview:b""});},
    "publish"=>{if cx.is_cancelled(){return Err(Fault::from("Drawing export cancelled before publication"));}let download=ArtifactDownloadOutput::new(std::mem::take(&mut owners.filename),std::mem::take(&mut owners.mime),Some("base64".into()),owners.chunks.take().unwrap())?;self.complete=true;cx.consume_fuel(1);return Ok(ArtifactCommandWorkStep::CompleteDownload{download,ephemeral:Default::default()});},
    _=>return Err(Fault::from("Drawing export stage is invalid")),
   }
  }
  Ok(ArtifactCommandWorkStep::Progress{stage:self.phase,preview:b""})
 }
 fn begin_close(&mut self){self.closing=true;if let Some(job)=self.vector.take(){let(close,plan)=job.into_retirement();self.vector_close=Some(close);if let Some(plan)=plan{if let Some(output)=self.output.original_mut(){output.plan=Some(plan);}}}if let Some(job)=self.raster.take(){let(close,image)=job.into_retirement();self.raster_close=Some(close);if let Some(image)=image{if let Some(output)=self.output.original_mut(){output.image=Some(image);}}}}
 fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep{
  use semio_framework_job::InteractiveJobCloseStep;if !self.closing{return InteractiveJobCloseStep::Blocked;}
  if !self.output.terminal_is_empty(){return match self.output.step(grant){Ok(step)=>InteractiveJobCloseStep::Pending{progress:step.progress()},Err(error)=>InteractiveJobCloseStep::Refused(error.kind)};}
  if let Some(close)=&mut self.vector_close{if grant.maximum_items==0||grant.maximum_depth==0{return InteractiveJobCloseStep::Blocked;}if let Some(read)=close.take_snapshot_read(){if !read.return_to_registry(){return InteractiveJobCloseStep::Refused(semio_framework_value::ValueRefusalKind::InvariantViolated);}if close.terminal_is_empty(){self.vector_close=None;}return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}};}return match close.close_step(grant){Ok(step)=>{let progress=step.progress();if close.terminal_is_empty(){self.vector_close=None;}InteractiveJobCloseStep::Pending{progress}},Err(error)=>InteractiveJobCloseStep::Refused(error.kind)};}
  if let Some(close)=&mut self.raster_close{
   if grant.maximum_items==0||grant.maximum_depth==0{return InteractiveJobCloseStep::Blocked;}
   if let Some(read)=close.take_snapshot_read(){if !read.return_to_registry(){return InteractiveJobCloseStep::Refused(semio_framework_value::ValueRefusalKind::InvariantViolated);}if close.terminal_is_empty(){self.raster_close=None;}return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}};}
   return match close.close_step(grant){Ok(step)=>{let progress=step.progress();if close.terminal_is_empty(){self.raster_close=None;}InteractiveJobCloseStep::Pending{progress}},Err(error)=>InteractiveJobCloseStep::Refused(error.kind)};
  }
  if let Some(read)=self.read.take(){if grant.maximum_items==0||grant.maximum_depth==0{self.read=Some(read);return InteractiveJobCloseStep::Blocked;}if !read.return_to_registry(){return InteractiveJobCloseStep::Refused(semio_framework_value::ValueRefusalKind::InvariantViolated);}return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}};}
  InteractiveJobCloseStep::Complete{progress:Default::default()}
 }
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{if !self.output.terminal_is_empty(){self.output.next_copy_byte_demand()}else if let Some(close)=&self.vector_close{close.next_copy_byte_demand()}else{self.raster_close.as_ref().map_or(Ok(0),DocumentRasterRetirement::next_copy_byte_demand)}}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if !self.output.terminal_is_empty(){self.output.next_capacity_byte_demand(body)}else if let Some(close)=&self.vector_close{close.next_capacity_byte_demand(body)}else{self.raster_close.as_ref().map_or(Ok(0),|close|close.next_capacity_byte_demand(body))}}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{if !self.output.terminal_is_empty(){self.output.next_release_byte_demand()}else if let Some(close)=&self.vector_close{close.next_release_byte_demand()}else{self.raster_close.as_ref().map_or(Ok(0),DocumentRasterRetirement::next_release_byte_demand)}}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{if !self.output.terminal_is_empty(){self.output.next_depth_demand()}else if let Some(close)=&self.vector_close{close.next_depth_demand()}else if let Some(close)=&self.raster_close{close.next_depth_demand()}else{Ok(usize::from(self.read.is_some()))}}
 fn terminal_frame_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
 fn terminal_is_empty(&self)->bool{self.closing&&self.vector.is_none()&&self.vector_close.is_none()&&self.raster.is_none()&&self.raster_close.is_none()&&self.read.is_none()&&self.output.terminal_is_empty()}
}
pub(super) struct DrawingExportCommandJobFactory{keys:Vec<semio_framework::ToolFactoryKey>}
impl DrawingExportCommandJobFactory{pub(super) fn new(controller:&str)->Self{Self{keys:vec![semio_framework::ToolFactoryKey::new(controller,"exportDocument")]}}}
impl semio_framework::ToolJobFactory for DrawingExportCommandJobFactory {
 type Payload=semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<App>;
 type Job=semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<App>;
 fn keys(&self)->&[semio_framework::ToolFactoryKey]{&self.keys}
 fn payload_schema_id(&self)->&str{DRAWING_BOUNDED_PAYLOAD_SCHEMA}
 fn classification(&self)->semio_framework::InteractiveJobClassification{semio_framework::InteractiveJobClassification::Migrated}
 fn execution_contract(&self)->semio_framework::ToolExecutionContract{semio_framework::ToolExecutionContract::resumable(4096,4096,1,OUTPUT_LIMIT,7500,1,1)}
 fn create_job(&mut self,_operation:semio_framework_job::Operation,payload:Self::Payload)->Result<Self::Job,semio_framework::ToolJobFactoryError>{Ok(Self::Job::new(payload))}
 fn create_job_from_wire_pages_with_payload(&mut self,_operation:semio_framework_job::Operation,payload:Self::Payload,input:semio_framework::action_bus::RetainedToolWireInput,checkpoint:Option<semio_framework::action_bus::RetainedToolWireInput>)->Result<Self::Job,(semio_framework::ToolJobFactoryError,semio_framework::action_bus::RetainedToolWireInput,Option<semio_framework::action_bus::RetainedToolWireInput>)>{if checkpoint.is_some()||input.declared_bytes()>4096{return Err((semio_framework::ToolJobFactoryError::new("Drawing export rejects oversized command or checkpoint"),input,checkpoint));}Ok(Self::Job::from_wire(payload,input))}
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for DrawingExportCommandJobFactory{type Owner=App;const TOOL_IDS:&'static[&'static str]=DRAWING_EXPORT_TOOL_IDS;const DOCUMENT_SCHEMA:&'static str=DRAWING_DOCUMENT_SCHEMA;const PUBLICATION_CONTRACTS:&'static[semio_framework_plugin::ArtifactToolPublicationContract]=DRAWING_EXPORT_PUBLICATION_CONTRACTS;}
pub(super) fn build(request:semio_framework_plugin::ArtifactOwnedToolJobRequest<App>)->Result<semio_framework::ToolOperationSpec,Fault>{
 let DrawingCommand::ExportDocument(command)=request.command.as_ref()else{return Err(Fault::from("Drawing export command mismatch"));};if !request.snapshot_read.commit_authority_matches(request.operation.generation.0,request.canonical_base_revision){return Err(Fault::from("Drawing export source authority mismatch"));}let work=Work::new(request.snapshot_read,command);
 let operation=semio_framework_plugin::AppOperationContext{app_instance_id:request.app_instance_id,parent_document_id:request.parent_document_id.clone(),operation_id:request.operation.operation.0,generation:request.operation.generation.0,canonical_base_revision:request.canonical_base_revision,authoring_seed:request.authoring_seed.clone()};
 let payload=semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs{command:*request.command,snapshot:request.snapshot,config:request.config,history:request.history,interaction_state:request.interaction_state,interaction_hover:request.interaction_hover,context:Some(request.context),operation,completion:request.completion},DrawingCommand::command_id,4096,4096,Box::new(work));
 Ok(semio_framework::ToolOperationSpec::new(request.controller_id,request.tool_id,request.payload_schema_id,payload,request.operation))
}

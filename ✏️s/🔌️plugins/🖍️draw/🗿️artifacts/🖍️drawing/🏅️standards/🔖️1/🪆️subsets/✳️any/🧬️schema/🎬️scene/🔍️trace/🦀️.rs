//! 🔍️ Genuine intrinsic samples become private native trace paths under the same original ledger.
use crate::preparation::{DocumentScenePlan,DocumentSceneContent,DocumentSceneError};
use semio_s_artifact_draw_drawing_geometry::from_kernel_segment;
use semio_s_artifact_draw_drawing_model::PathSegment as DrawingSegment;
use semio_framework_2d::{PathSegment,trace::{BitmapTraceInput,BitmapTraceJob,BitmapTraceProgress,BitmapTraceRetirement}};
use semio_framework_2d::retirement::WorkRetirementProgress;
use semio_framework_value::{RetirementDemand,work::RetainedWorkLedger};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,arc_custody::SharedControlledRetirement};
use semio_framework_pixels::retirement::RasterLease;
use std::sync::Arc;
pub type DocumentTraceRetirementProgress=WorkRetirementProgress;
#[derive(Clone,Copy,Debug)]
pub struct DocumentTraceLimits{pub max_pixels:usize,pub max_admitted_pixels:usize,pub max_source_bytes:usize,pub max_edges:usize,pub max_segments:usize,pub max_retained_segments:usize,pub max_work:u64}
#[derive(Clone,Debug)]
pub struct DocumentTraceInput{pub plan:DocumentScenePlan,pub limits:DocumentTraceLimits}
#[derive(Clone,Copy,Debug)]
pub struct DocumentTraceProgress{pub phase:&'static str,pub assets:usize,pub nodes:usize,pub resolved:usize,pub admitted_images:usize,pub pixels:usize,pub source_bytes:usize,pub segments:usize,pub work:u64,pub trace:Option<BitmapTraceProgress>,pub done:bool}
fn invalid(message:&'static str)->DocumentSceneError{DocumentSceneError::literal(message)}
pub fn identity(v:&str)->Result<(),DocumentSceneError>{if v.is_empty()||v.len()>4096{Err(invalid("Invalid document trace identity"))}else{Ok(())}}
fn kernel_error(error:semio_framework_2d::trace::BitmapTraceError)->DocumentSceneError{use semio_framework_2d::trace::BitmapTraceError;invalid(match error{BitmapTraceError::Invalid(message)=>message,BitmapTraceError::Incomplete=>"Bitmap trace incomplete",BitmapTraceError::Cancelled=>"Bitmap trace cancelled"})}
fn pixel_error(error:semio_framework_pixels::editing::PixelEditError)->DocumentSceneError{use semio_framework_pixels::editing::PixelEditError;invalid(match error{PixelEditError::Invalid(message)=>message,PixelEditError::Incomplete=>"Pixel operation is incomplete",PixelEditError::Cancelled=>"Pixel operation cancelled"})}
struct TraceMask(Arc<Vec<u8>>);
impl AsRef<[u8]> for TraceMask{fn as_ref(&self)->&[u8]{self.0.as_slice()}}
impl RetireOwned for TraceMask{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(SharedControlledRetirement::lease(self.0))}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SharedControlledRetirement<Vec<u8>>>())}fn controlled_retirement_supported()->bool{true}}
fn slot<T>(values:&mut Vec<T>,work:&mut RetainedWorkLedger)->Result<bool,DocumentSceneError>{let target=if values.len()==values.capacity(){values.capacity().checked_mul(2).unwrap_or(usize::MAX).max(16)}else{values.len()+1};if !work.reserve_vec(values,target).map_err(invalid)?{return Ok(false);}work.vec_slot(values).map_err(invalid)}
fn fault_part(plan:&DocumentScenePlan,node:usize,kind:u8,part:usize)->Option<&str>{
 let n=&plan.nodes[node];match(kind,part){(1,0)=>Some("Missing document trace source: "),(1,1)|(2,1)=>Some(&n.id),(1,2)=>Some(" -> "),(1,3)=>if let DocumentSceneContent::Trace{source,..}=&n.content{Some(source)}else{None},(2,0)=>Some("Invalid document trace parameters: "),_=>None}
}
/// ⏱️ Original plan owners supply identity lookup while actual luma, glyph and child storage stay funded.
pub struct DocumentTraceJob{
 input:DocumentTraceInput,phase:&'static str,assets:usize,nodes:usize,resolved:usize,admitted_images:usize,pixels:usize,source_bytes:usize,segments:usize,work:u64,at:usize,pixel:usize,publish_at:usize,
 masks:Vec<(usize,u32,u32,TraceMask)>,source:usize,image:Option<RasterLease>,mask:Vec<u8>,tracer:Option<BitmapTraceJob<TraceMask>>,trace:Option<BitmapTraceProgress>,candidate:Vec<DrawingSegment>,
 output:Option<DocumentScenePlan>,failure:Option<DocumentSceneError>,cancelled:bool,pub closing_tracer:Option<BitmapTraceRetirement<TraceMask>>,closing_mask:Option<TraceMask>,retired_tracers:Vec<BitmapTraceRetirement<TraceMask>>,retired_sources:Vec<String>,retired_masks:Vec<TraceMask>,
 pending:RetirementDemand,last_turn_work:u64,last_turn_semantic_work:u64,step_child_work:Option<(u64,u64)>,step_control_work:u64,error_buffer:String,error_kind:u8,error_part:usize,
}
impl DocumentTraceJob{
 /// 📊️ Validates scalar trace limits before an owning plan is transferred.
 pub fn validate_limits(l:DocumentTraceLimits)->Result<(),DocumentSceneError>{
  for(value,min,max)in[(l.max_pixels,1,16777216),(l.max_admitted_pixels,1,67108864),(l.max_source_bytes,1,268439552),(l.max_edges,1,65536),(l.max_segments,1,65536),(l.max_retained_segments,1,65536)]{if !(min..=max).contains(&value){return Err(invalid("Invalid document trace limits"));}}
  if !(1..=1_000_000_000).contains(&l.max_work){return Err(invalid("Invalid document trace plan or work limit"));}Ok(())
 }
 /// 📥️ Invalid input remains inside the genuine owner until separately funded close.
 pub fn new(input:DocumentTraceInput)->Self{
  let failure=Self::validate_limits(input.limits).err().or_else(||(input.plan.nodes.len()>1024||input.plan.assets.len()>1024).then(||invalid("Invalid document trace plan or work limit")));
  Self{input,phase:"assets",assets:0,nodes:0,resolved:0,admitted_images:0,pixels:0,source_bytes:0,segments:0,work:0,at:0,pixel:0,publish_at:0,masks:Vec::new(),source:0,image:None,mask:Vec::new(),tracer:None,trace:None,candidate:Vec::new(),output:None,failure,cancelled:false,closing_tracer:None,closing_mask:None,retired_tracers:Vec::new(),retired_sources:Vec::new(),retired_masks:Vec::new(),pending:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0,step_child_work:None,step_control_work:0,error_buffer:String::new(),error_kind:0,error_part:0}
 }
 fn start_trace(&mut self,width:u32,height:u32,mask:Arc<Vec<u8>>)->Result<(),DocumentSceneError>{
  let DocumentSceneContent::Trace{threshold,simplify_epsilon,..}=&self.input.plan.nodes[self.at].content else{return Err(invalid("Document trace node lost its trace content"))};let l=self.input.limits;
  self.tracer=Some(BitmapTraceJob::new(BitmapTraceInput{width,height,mask:TraceMask(mask),threshold:*threshold,simplify_epsilon:*simplify_epsilon,max_pixels:l.max_pixels,max_edges:l.max_edges,max_segments:l.max_segments.min(l.max_retained_segments-self.segments).max(1),max_work:l.max_work}));self.phase="tracing";Ok(())
 }
 fn step(&mut self,work:&mut RetainedWorkLedger)->Result<bool,DocumentSceneError>{
  match self.phase{
   "assets"=>{
    let Some(asset)=self.input.plan.assets.get(self.assets)else{self.phase="indexing";return Ok(true);};identity(&asset.id)?;semio_framework_pixels::editing::validate_image(&asset.image).map_err(pixel_error)?;
    if self.input.plan.assets[..self.assets].iter().any(|other|other.id==asset.id){return Err(invalid("Invalid or duplicate document trace asset"));}self.assets+=1;
   },
   "indexing"=>{
    let Some(node)=self.input.plan.nodes.get(self.nodes)else{self.phase="nodes";return Ok(true);};crate::preparation::validate_scene_source_address(&node.source_path,node.locked_ancestors)?;identity(&node.id)?;
    if self.input.plan.nodes[..self.nodes].iter().any(|other|other.id==node.id)||!node.transform.into_iter().all(|value|value.is_finite()&&value.abs()<=1e9){return Err(invalid("Invalid or duplicate document trace node"));}
    if let DocumentSceneContent::Path{segments,..}=&node.content{if segments.len()>self.input.limits.max_retained_segments-self.segments{return Err(invalid("Document trace retained segment limit exceeded"));}self.segments+=segments.len();}
    if let DocumentSceneContent::Trace{source,threshold,simplify_epsilon,..}=&node.content{
     identity(source)?;let kind=if !self.input.plan.assets.iter().any(|asset|&asset.id==source){1}else if !threshold.is_finite()||!(0.0..=1.0).contains(threshold)||!simplify_epsilon.is_finite()||!(0.0..=8192.0).contains(simplify_epsilon){2}else{0};
     if kind!=0{self.error_kind=kind;self.error_part=0;self.phase="diagnostic";return Ok(true);}
    }self.nodes+=1;
   },
   "diagnostic"=>{
    let total=(0..4).filter_map(|part|fault_part(&self.input.plan,self.nodes,self.error_kind,part)).try_fold(0usize,|sum,part|sum.checked_add(part.len())).ok_or_else(||invalid("Trace diagnostic extent overflow"))?;
    if !work.reserve_string(&mut self.error_buffer,total).map_err(invalid)?{return Ok(false);}
    if let Some(part)=fault_part(&self.input.plan,self.nodes,self.error_kind,self.error_part){if !work.string_push(&mut self.error_buffer,part).map_err(invalid)?{return Ok(false);}self.error_part+=1;return Ok(true);}
    if !work.arc_ready::<String>().map_err(invalid)?{return Ok(false);}
    let message=work.arc_new(std::mem::take(&mut self.error_buffer)).map_err(|(error,_)|invalid(error))?;return Err(DocumentSceneError::owned(message));
   },
   "nodes"=>{
    let Some(node)=self.input.plan.nodes.get(self.at)else{self.step_control_work=1;self.output=Some(std::mem::take(&mut self.input.plan));self.phase="complete";return Ok(true);};
    let DocumentSceneContent::Trace{source,..}=&node.content else{self.at+=1;return Ok(true);};let index=self.input.plan.assets.iter().position(|asset|&asset.id==source).ok_or_else(||invalid("Captured trace source disappeared"))?;
    if let Some((_,width,height,mask))=self.masks.iter().find(|(key,_,_,_)|*key==index){if !work.can_step(){return Ok(false);}let(width,height,mask)=(*width,*height,Arc::clone(&mask.0));self.start_trace(width,height,mask)?;return Ok(true);}
    let asset=&self.input.plan.assets[index];let l=self.input.limits;let count=asset.image.width as usize*asset.image.height as usize;
    if asset.image.pixels.len()>l.max_source_bytes.saturating_sub(self.source_bytes)||count>l.max_pixels.min(l.max_admitted_pixels.saturating_sub(self.pixels)){return Err(invalid("Document trace intrinsic image budget exceeded"));}
    if !work.can_step(){return Ok(false);}
    self.source=index;self.source_bytes+=asset.image.pixels.len();self.pixels+=count;self.admitted_images+=1;self.pixel=0;self.image=Some(RasterLease(Arc::clone(&asset.image)));self.phase="luma";
   },
   "luma"=>{
    let image=self.image.as_ref().ok_or_else(DocumentSceneError::held)?;let count=image.width as usize*image.height as usize;
    if self.pixel==count{
     if !slot(&mut self.masks,work)?{return Ok(false);}let copy=0;let capacity=semio_framework_value::retirement::arc_custody::arc_bytes::<Vec<u8>>();let remaining=work.child_grant();let demand=RetirementDemand{copy_bytes:copy+size_of::<Vec<u8>>(),capacity_bytes:capacity,depth:1,release_bytes:0};
     if remaining.maximum_items==0||remaining.maximum_copy_bytes<demand.copy_bytes||remaining.maximum_capacity_bytes<capacity||remaining.maximum_depth<1{work.require_child(demand).map_err(invalid)?;return Ok(false);}
     if !work.copy_bytes(copy){return Ok(false);}let(width,height)=(image.width,image.height);let mask=work.arc_new(std::mem::take(&mut self.mask)).map_err(|(error,_)|invalid(error))?;
     work.vec_push(&mut self.masks,(self.source,width,height,TraceMask(Arc::clone(&mask)))).map_err(|(error,_)|invalid(error))?;self.image=None;self.start_trace(width,height,mask)?;return Ok(true);
    }
    if !slot(&mut self.mask,work)?{return Ok(false);}let p=&image.pixels;let i=self.pixel*4;let value=((299.0*p[i]as f64+587.0*p[i+1]as f64+114.0*p[i+2]as f64)*p[i+3]as f64/255000.0).round()as u8;
    work.vec_push(&mut self.mask,value).map_err(|(error,_)|invalid(error))?;self.pixel+=1;
   },
   "tracing"=>{
    let child=self.tracer.as_mut().ok_or_else(DocumentSceneError::held)?;let result=child.advance(1,work);self.step_child_work=Some((child.last_turn_work(),child.last_turn_semantic_work()));self.pending=child.next_work_demand();let p=result.map_err(kernel_error)?;self.trace=Some(p);if p.done{self.publish_at=0;self.phase="publishing";}
   },
   "publishing"=>{
    if self.closing_tracer.is_some(){
     if !slot(&mut self.retired_tracers,work)?{return Ok(false);}if self.closing_mask.is_some()&&!slot(&mut self.retired_masks,work)?{return Ok(false);}
     work.vec_push(&mut self.retired_tracers,self.closing_tracer.take().ok_or_else(DocumentSceneError::held)?).map_err(|(error,_)|invalid(error))?;if let Some(mask)=self.closing_mask.take(){work.vec_push(&mut self.retired_masks,mask).map_err(|(error,_)|invalid(error))?;}self.step_control_work=1;return Ok(true);
    }
    if self.tracer.as_ref().is_none_or(|job|job.result().is_ok_and(|trace|self.publish_at==trace.len())){
     if let Some(tracer)=self.tracer.take(){let(close,mask)=tracer.into_retirement();self.closing_tracer=Some(close);self.closing_mask=mask;self.step_control_work=1;return Ok(true);}
     if !slot(&mut self.retired_sources,work)?||!work.can_step(){return Ok(false);}
     let content=&mut self.input.plan.nodes[self.at].content;let old=std::mem::replace(content,DocumentSceneContent::Group{children:Vec::new(),isolation:false});let DocumentSceneContent::Trace{source,fill_rule,fill,stroke,..}=old else{unreachable!()};
     work.vec_push(&mut self.retired_sources,source).map_err(|(error,_)|invalid(error))?;*content=DocumentSceneContent::Path{segments:std::mem::take(&mut self.candidate),fill_rule,fill,stroke};self.at+=1;self.resolved+=1;self.phase="nodes";self.step_control_work=1;return Ok(true);
    }
    if self.segments>=self.input.limits.max_retained_segments{return Err(invalid("Document trace retained segment limit exceeded"));}
    let segment=&self.tracer.as_ref().ok_or_else(DocumentSceneError::held)?.result().map_err(kernel_error)?[self.publish_at];if !matches!(segment,PathSegment::Move{..}|PathSegment::Line{..}|PathSegment::Close){return Err(invalid("Unresolved document trace curve"));}
    if !slot(&mut self.candidate,work)?{return Ok(false);}work.vec_push(&mut self.candidate,from_kernel_segment(segment)).map_err(|(error,_)|invalid(error))?;self.publish_at+=1;self.segments+=1;self.step_control_work=1;
   },
   _=>{}
  }Ok(true)
 }
 pub fn settlement_pending(&self)->bool{self.phase=="complete"||self.phase=="publishing"||self.phase=="nodes"&&self.at==self.input.plan.nodes.len()||self.phase=="tracing"&&self.tracer.as_ref().is_some_and(BitmapTraceJob::settlement_pending)}
 pub fn failure_pending(&self)->bool{self.phase=="diagnostic"||self.failure.is_some()}
 pub fn next_work_demand(&self)->RetirementDemand{self.pending}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 fn progress(&self)->DocumentTraceProgress{DocumentTraceProgress{phase:self.phase,assets:self.assets,nodes:self.nodes,resolved:self.resolved,admitted_images:self.admitted_images,pixels:self.pixels,source_bytes:self.source_bytes,segments:self.segments,work:self.work,trace:self.trace,done:self.phase=="complete"}}
 pub fn advance(&mut self,budget:usize,work:&mut RetainedWorkLedger)->Result<DocumentTraceProgress,DocumentSceneError>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if budget==0||budget as u128>9_007_199_254_740_991{return Err(invalid("Invalid document trace work grant"));}if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}
  let d=self.pending;if !work.admits_absolute(d){work.require_absolute(d);return Ok(self.progress());}
  let Some(root)=work.enter()else{self.pending=work.next_demand();return Ok(self.progress());};
  let result=(||{for _ in 0..budget{if self.phase=="complete"||!work.can_step(){break;}if self.work>=self.input.limits.max_work&&!self.failure_pending()&&!self.settlement_pending(){let error=invalid("Document trace work limit exceeded");self.failure=Some(error.clone());return Err(error);}
   let before=work.receipt();let publishing_failure=self.failure_pending();self.step_child_work=None;self.step_control_work=0;let publishing_custody=self.phase=="publishing"||self.phase=="nodes"&&self.at==self.input.plan.nodes.len();let result=self.step(work);let own_semantic=!publishing_custody&&matches!(&result,Ok(true)|Err(_));let(cpu,semantic)=if let Some((cpu,semantic))=self.step_child_work{(cpu,semantic)}else{(self.step_control_work.max((own_semantic||work.receipt()!=before)as u64),own_semantic as u64)};if cpu>0{work.note_source(root);self.last_turn_work+=cpu;}if !publishing_failure{self.work+=semantic;self.last_turn_semantic_work+=semantic;}self.pending=work.next_demand();if let Err(error)=result{self.failure=Some(error.clone());return Err(error);}if work.blocked(){break;}
  }Ok(self.progress())})();work.exit();result
 }
 pub fn result(&self)->Result<&DocumentScenePlan,DocumentSceneError>{if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}self.output.as_ref().ok_or(DocumentSceneError::Incomplete)}
 /// 🧹️ No archive allocation occurs while genuine active child and mask custody move into scalar close slots.
 pub fn into_retirement(mut self)->(DocumentTraceRetirement,DocumentScenePlan,Option<DocumentScenePlan>){
  if let Some(tracer)=self.tracer.take(){assert!(self.closing_tracer.is_none());let(close,mask)=tracer.into_retirement();self.closing_tracer=Some(close);self.closing_mask=mask;}
  let input=std::mem::take(&mut self.input.plan);let output=if self.phase=="complete"&&!self.cancelled&&self.failure.is_none(){self.output.take()}else{None};self.cancelled=true;(DocumentTraceRetirement::new(self),input,output)
 }
 pub fn cancel(&mut self){self.cancelled=true;}
}
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct DocumentTraceOwners{plan:DocumentScenePlan,masks:Vec<(usize,u32,u32,TraceMask)>,image:Option<RasterLease>,mask:Vec<u8>,tracer:Option<BitmapTraceJob<TraceMask>>,candidate:Vec<DrawingSegment>,output:Option<DocumentScenePlan>,failure:Option<DocumentSceneError>,closing_tracer:Option<BitmapTraceRetirement<TraceMask>>,closing_mask:Option<TraceMask>,retired_tracers:Vec<BitmapTraceRetirement<TraceMask>>,retired_sources:Vec<String>,retired_masks:Vec<TraceMask>,error_buffer:String}
impl RetireOwned for DocumentTraceJob{
 fn retirement(self)->Box<dyn RetirementCursor>{DocumentTraceOwners{plan:self.input.plan,masks:self.masks,image:self.image,mask:self.mask,tracer:self.tracer,candidate:self.candidate,output:self.output,failure:self.failure,closing_tracer:self.closing_tracer,closing_mask:self.closing_mask,retired_tracers:self.retired_tracers,retired_sources:self.retired_sources,retired_masks:self.retired_masks,error_buffer:self.error_buffer}.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.input.plan),deferred_birth_bytes_for(&self.masks),deferred_birth_bytes_for(&self.image),deferred_birth_bytes_for(&self.mask),deferred_birth_bytes_for(&self.tracer),deferred_birth_bytes_for(&self.candidate),deferred_birth_bytes_for(&self.output),deferred_birth_bytes_for(&self.failure),deferred_birth_bytes_for(&self.closing_tracer),deferred_birth_bytes_for(&self.closing_mask),deferred_birth_bytes_for(&self.retired_tracers),deferred_birth_bytes_for(&self.retired_sources),deferred_birth_bytes_for(&self.retired_masks),deferred_birth_bytes_for(&self.error_buffer)])}
 fn controlled_retirement_supported()->bool{DocumentTraceOwners::controlled_retirement_supported()}
}
semio_framework_2d::physical_work_retirement!(DocumentTraceRetirement,DocumentTraceJob);

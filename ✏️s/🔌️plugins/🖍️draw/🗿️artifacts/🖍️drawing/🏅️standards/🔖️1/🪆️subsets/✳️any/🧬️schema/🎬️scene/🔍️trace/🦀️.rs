//! 🔍️ Admitted intrinsic samples become private native-coordinate trace paths under grants.
use crate::schema::scene_preparation::{DocumentScenePlan,DocumentSceneContent,DocumentSceneError};
use crate::standards::v1::subsets::any::schema::component::from_kernel_segment;
use crate::PathSegment as DrawingSegment;
use semio_framework_2d::{PathSegment,trace::{BitmapTraceInput,BitmapTraceJob,BitmapTraceProgress,BitmapTraceRetirement}};
use semio_framework_pixels::RasterImage;
use crate::schema::scene_retirement::ScenePlanCloseJob;
use semio_framework_2d::retirement::{WorkRetirementCounter,WorkRetirementProgress};
pub type DocumentTraceRetirementProgress=WorkRetirementProgress;
use std::{collections::{BTreeMap,BTreeSet},sync::Arc};
#[derive(Clone,Copy,Debug)]
pub struct DocumentTraceLimits{pub max_pixels:usize,pub max_admitted_pixels:usize,pub max_source_bytes:usize,pub max_edges:usize,pub max_segments:usize,pub max_retained_segments:usize,pub max_work:u64}
#[derive(Clone,Debug)]
pub struct DocumentTraceInput{pub plan:DocumentScenePlan,pub limits:DocumentTraceLimits}
#[derive(Clone,Copy,Debug)]
pub struct DocumentTraceProgress{pub phase:&'static str,pub assets:usize,pub nodes:usize,pub resolved:usize,pub admitted_images:usize,pub pixels:usize,pub source_bytes:usize,pub segments:usize,pub work:u64,pub trace:Option<BitmapTraceProgress>,pub done:bool}
fn invalid(message:impl Into<String>)->DocumentSceneError{DocumentSceneError::Invalid(message.into())}
fn identity(v:&str)->Result<(),DocumentSceneError>{if v.is_empty()||v.len()>4096{Err(invalid("Invalid document trace identity"))}else{Ok(())}}
struct TraceMask(Arc<Vec<u8>>);
impl AsRef<[u8]> for TraceMask{fn as_ref(&self)->&[u8]{self.0.as_slice()}}
/// ⏱️ Move shared luma owners into the real trace job and publish complete native pixel paths privately.
pub struct DocumentTraceJob{
 input:DocumentTraceInput,phase:&'static str,assets:usize,nodes:usize,resolved:usize,admitted_images:usize,pixels:usize,source_bytes:usize,segments:usize,work:u64,at:usize,pixel:usize,publish_at:usize,
 catalog:BTreeMap<String,usize>,ids:BTreeSet<String>,masks:BTreeMap<String,(u32,u32,Arc<Vec<u8>>)>,source:String,
 image:Option<Arc<RasterImage>>,mask:Vec<u8>,tracer:Option<BitmapTraceJob<TraceMask>>,trace:Option<BitmapTraceProgress>,raw:Vec<PathSegment>,candidate:Vec<DrawingSegment>,
 output:Option<DocumentScenePlan>,failure:Option<DocumentSceneError>,cancelled:bool,closing_tracer:Option<BitmapTraceRetirement<TraceMask>>,cleanup_slot:u8,
}
impl DocumentTraceJob{
 pub fn new(input:DocumentTraceInput)->Result<Self,DocumentSceneError>{
  let l=input.limits;for(value,min,max)in [(l.max_pixels,1,16777216),(l.max_admitted_pixels,1,67108864),(l.max_source_bytes,1,268439552),(l.max_edges,1,65536),(l.max_segments,1,65536),(l.max_retained_segments,1,65536)]{if !(min..=max).contains(&value){return Err(invalid("Invalid document trace limits"));}}
  if !(1..=1000000000).contains(&l.max_work)||input.plan.nodes.len()>1024||input.plan.assets.len()>1024{return Err(invalid("Invalid document trace plan or work limit"));}
  Ok(Self{input,phase:"assets",assets:0,nodes:0,resolved:0,admitted_images:0,pixels:0,source_bytes:0,segments:0,work:0,at:0,pixel:0,publish_at:0,catalog:BTreeMap::new(),ids:BTreeSet::new(),masks:BTreeMap::new(),source:String::new(),image:None,mask:Vec::new(),tracer:None,trace:None,raw:Vec::new(),candidate:Vec::new(),output:None,failure:None,cancelled:false,closing_tracer:None,cleanup_slot:0})
 }
 fn start_trace(&mut self,width:u32,height:u32,mask:Arc<Vec<u8>>)->Result<(),DocumentSceneError>{
  let DocumentSceneContent::Trace{threshold,simplify_epsilon,..}=&self.input.plan.nodes[self.at].content else{return Err(invalid("Expected document trace record"));};let l=self.input.limits;
  self.tracer=Some(BitmapTraceJob::new(BitmapTraceInput{width,height,mask:TraceMask(mask),threshold:*threshold,simplify_epsilon:*simplify_epsilon,max_pixels:l.max_pixels,max_edges:l.max_edges,max_segments:l.max_segments.min(l.max_retained_segments-self.segments).max(1),max_work:l.max_work}).map_err(|e|invalid(e.to_string()))?);self.phase="tracing";Ok(())
 }
 fn step(&mut self)->Result<(),DocumentSceneError>{
  match self.phase{
   "assets"=>{let Some(a)=self.input.plan.assets.get(self.assets)else{self.phase="indexing";return Ok(());};identity(&a.id)?;semio_framework_pixels::editing::validate_image(&a.image).map_err(|error|invalid(error.to_string()))?;if self.catalog.contains_key(&a.id){return Err(invalid("Invalid or duplicate document trace asset"));}self.catalog.insert(a.id.clone(),self.assets);self.assets+=1;},
   "indexing"=>{
    let Some(n)=self.input.plan.nodes.get(self.nodes)else{self.phase="nodes";return Ok(());};crate::schema::scene_preparation::validate_scene_source_address(&n.source_path,n.locked_ancestors)?;identity(&n.id)?;if !self.ids.insert(n.id.clone())||!n.transform.into_iter().all(|v|v.is_finite()&&v.abs()<=1e9){return Err(invalid("Invalid or duplicate document trace node"));}
    if let DocumentSceneContent::Path{segments,..}=&n.content{if segments.len()>self.input.limits.max_retained_segments-self.segments{return Err(invalid("Document trace retained segment limit exceeded"));}self.segments+=segments.len();}
    if let DocumentSceneContent::Trace{source,threshold,simplify_epsilon,..}=&n.content{identity(source)?;if !self.catalog.contains_key(source){return Err(invalid(format!("Missing document trace source: {} -> {source}",n.id)));}if !threshold.is_finite()||!(0.0..=1.0).contains(threshold)||!simplify_epsilon.is_finite()||!(0.0..=8192.0).contains(simplify_epsilon){return Err(invalid(format!("Invalid document trace parameters: {}",n.id)));}}self.nodes+=1;
   },
   "nodes"=>{
    let Some(n)=self.input.plan.nodes.get(self.at)else{if self.cleanup(){self.output=Some(std::mem::take(&mut self.input.plan));self.phase="complete";}return Ok(());};let DocumentSceneContent::Trace{source,..}=&n.content else{self.at+=1;return Ok(());};
    if let Some((w,h,mask))=self.masks.get(source).map(|(w,h,m)|(*w,*h,m.clone())){return self.start_trace(w,h,mask);}
    let a=&self.input.plan.assets[self.catalog[source]];let l=self.input.limits;let count=a.image.width as usize*a.image.height as usize;if a.image.pixels.len()>l.max_source_bytes.saturating_sub(self.source_bytes)||count>l.max_pixels.min(l.max_admitted_pixels.saturating_sub(self.pixels)){return Err(invalid("Document trace intrinsic image budget exceeded"));}
    self.source_bytes+=a.image.pixels.len();self.source=source.clone();self.pixels+=count;self.admitted_images+=1;self.mask=Vec::new();self.pixel=0;self.image=Some(Arc::clone(&a.image));self.phase="luma";
   },
   "luma"=>{
    let image=self.image.as_ref().unwrap();let count=image.width as usize*image.height as usize;if self.pixel==count{let(w,h)=(image.width,image.height);let mask=Arc::new(std::mem::take(&mut self.mask));self.masks.insert(self.source.clone(),(w,h,mask.clone()));self.image=None;return self.start_trace(w,h,mask);}
    let p=&image.pixels;let i=self.pixel*4;self.mask.push(((299.0*p[i]as f64+587.0*p[i+1]as f64+114.0*p[i+2]as f64)*p[i+3]as f64/255000.0).round()as u8);self.pixel+=1;
   },
   "tracing"=>{let p=self.tracer.as_mut().unwrap().advance(1).map_err(|e|invalid(e.to_string()))?;self.trace=Some(p);if p.done{self.candidate=Vec::new();self.publish_at=0;self.phase="publishing";}},
   "publishing"=>{
    if let Some(close)=&mut self.closing_tracer{if close.advance(1).map_err(|e|invalid(e.to_string()))?.done{self.closing_tracer=None;}return Ok(());}
    if self.tracer.as_ref().is_none_or(|job|self.publish_at==job.result().expect("completed trace remains private").len()){
     if let Some(tracer)=self.tracer.take(){self.closing_tracer=Some(tracer.into_retirement().0);return Ok(());}let content=&mut self.input.plan.nodes[self.at].content;let old=std::mem::replace(content,DocumentSceneContent::Group{children:Vec::new(),isolation:false});let DocumentSceneContent::Trace{fill_rule,fill,stroke,..}=old else{return Err(invalid("Expected document trace record"));};*content=DocumentSceneContent::Path{segments:std::mem::take(&mut self.candidate),fill_rule,fill,stroke};self.raw=Vec::new();self.at+=1;self.resolved+=1;self.phase="nodes";return Ok(());}
    if self.segments>=self.input.limits.max_retained_segments{return Err(invalid("Document trace retained segment limit exceeded"));}let s=&self.tracer.as_ref().unwrap().result().expect("completed trace remains private")[self.publish_at];if !matches!(s,PathSegment::Move{..}|PathSegment::Line{..}|PathSegment::Close){return Err(invalid("Unresolved document trace curve"));}self.candidate.push(from_kernel_segment(s));self.publish_at+=1;self.segments+=1;
   },
   _=>{}
  }Ok(())
 }
 fn cleanup(&mut self)->bool{let complete=match self.cleanup_slot{0=>self.catalog.pop_first().is_none(),1=>self.ids.pop_first().is_none(),2=>self.masks.pop_first().is_none(),3=>true,4=>{self.source=String::new();true},_=>unreachable!()};if complete{self.cleanup_slot+=1;}self.cleanup_slot==5}
 pub fn advance(&mut self,budget:usize)->Result<DocumentTraceProgress,DocumentSceneError>{
  if budget==0||budget as u128>9_007_199_254_740_991{return Err(invalid("Invalid document trace work grant"));}if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}
  for _ in 0..budget{if self.phase=="complete"{break;}let result=if self.work>=self.input.limits.max_work{Err(invalid("Document trace work limit exceeded"))}else{self.step()};if let Err(error)=result{self.failure=Some(error.clone());return Err(error);}self.work+=1;}
  Ok(DocumentTraceProgress{phase:self.phase,assets:self.assets,nodes:self.nodes,resolved:self.resolved,admitted_images:self.admitted_images,pixels:self.pixels,source_bytes:self.source_bytes,segments:self.segments,work:self.work,trace:self.trace,done:self.phase=="complete"})
 }
 pub fn result(&self)->Result<&DocumentScenePlan,DocumentSceneError>{if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}self.output.as_ref().ok_or(DocumentSceneError::Incomplete)}
 pub fn into_result(mut self)->Result<DocumentScenePlan,DocumentSceneError>{self.result()?;Ok(self.output.take().unwrap())}
 /// 🧹️ Return genuine source and complete output while retaining all private cleanup authority.
 pub fn into_retirement(mut self)->(DocumentTraceRetirement,DocumentScenePlan,Option<DocumentScenePlan>){
  if let Some(tracer)=self.tracer.take(){self.closing_tracer=Some(tracer.into_retirement().0);}let input=std::mem::take(&mut self.input.plan);let output=if self.phase=="complete"&&!self.cancelled&&self.failure.is_none(){self.output.take()}else{None};let draft=self.output.take().map(ScenePlanCloseJob::new);self.cancelled=true;(DocumentTraceRetirement{job:Some(self),draft,slot:0,counter:Default::default()},input,output)
 }
 fn clear(&mut self){if let Some(job)=&mut self.tracer{job.cancel();}self.tracer=None;self.closing_tracer=None;self.catalog.clear();self.ids.clear();self.masks.clear();self.image=None;self.mask=Vec::new();self.raw=Vec::new();self.candidate=Vec::new();self.source=String::new();self.output=None;self.input.plan=DocumentScenePlan::default();}
 pub fn cancel(&mut self){self.cancelled=true;self.clear();}
}
/// 🧹️ Drain tree entries and actual child cursors before dropping the empty wrapper.
pub struct DocumentTraceRetirement{job:Option<DocumentTraceJob>,draft:Option<ScenePlanCloseJob>,slot:u8,counter:WorkRetirementCounter}
impl DocumentTraceRetirement{
 pub fn terminal_is_empty(&self)->bool{self.job.is_none()&&self.draft.is_none()}
 fn step(&mut self){let job=self.job.as_mut().unwrap();let complete=match self.slot{
  0=>if let Some(close)=&mut job.closing_tracer{if close.advance(1).expect("positive trace cleanup grant").done{job.closing_tracer=None;true}else{false}}else{true},
  1=>{true},
  2=>if let Some(close)=&mut self.draft{if close.advance(1).expect("positive scene cleanup grant").done{self.draft=None;true}else{false}}else{true},
  3=>true,
  4=>job.catalog.pop_first().is_none(),5=>job.ids.pop_first().is_none(),6=>job.masks.pop_first().is_none(),7=>true,
  8=>{job.image=None;true},9=>{job.mask=Vec::new();true},10=>{job.raw=Vec::new();true},11=>{job.candidate=Vec::new();true},12=>{job.source=String::new();true},_=>unreachable!()
 };if complete{self.slot+=1;}if self.slot==13{self.job=None;}}
 pub fn advance(&mut self,grant:usize)->Result<DocumentTraceRetirementProgress,DocumentSceneError>{let mut counter=self.counter;let result=counter.advance(grant,||{self.step();self.terminal_is_empty()});self.counter=counter;result.map_err(invalid)}
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//! 🔀️ Complete prepared documents resolve referenced filled geometry under grants.
use crate::schema::scene_preparation::{DocumentScenePlan,DocumentSceneNode,DocumentSceneContent,DocumentSceneError};
use crate::PathSegment as DrawingSegment;
use semio_framework_2d::{PathSegment,Vec2};
use semio_framework_2d::booleans::{BooleanOperation,BooleanFillRule};
use semio_framework_2d::booleans::paths::{PathBooleanInput,PathBooleanOperand,PathBooleanJob,PathBooleanProgress,PathBooleanRetirement};
use semio_framework_2d::retirement::{WorkRetirementCounter,WorkRetirementProgress};
use crate::schema::scene_retirement::ScenePlanCloseJob;
pub type DocumentBooleanRetirementProgress=WorkRetirementProgress;
use std::collections::{BTreeMap,BTreeSet};
#[derive(Clone,Copy,Debug)]
pub struct DocumentBooleanLimits{pub tolerance:f64,pub epsilon:f64,pub max_depth:usize,pub max_references:usize,pub max_edges:usize,pub max_parameters:usize,pub max_atomic_edges:usize,pub max_segments:usize,pub max_retained_segments:usize,pub max_work:u64}
#[derive(Clone,Debug)]
pub struct DocumentBooleanInput{pub plan:DocumentScenePlan,pub limits:DocumentBooleanLimits}
#[derive(Clone,Copy,Debug)]
pub struct DocumentBooleanProgress{pub phase:&'static str,pub nodes:usize,pub resolved:usize,pub references:usize,pub segments:usize,pub work:u64,pub geometry:Option<PathBooleanProgress>,pub done:bool}
const IDENTITY:[f64;6]=[1.0,0.0,0.0,1.0,0.0,0.0];
fn invalid(message:impl Into<String>)->DocumentSceneError{DocumentSceneError::Invalid(message.into())}
fn matrix(m:[f64;6])->Result<[f64;6],DocumentSceneError>{if m.into_iter().all(|n|n.is_finite()&&n.abs()<=1e9){Ok(m)}else{Err(invalid("Invalid Boolean document matrix"))}}
fn inverse(m:[f64;6])->Result<[f64;6],DocumentSceneError>{let d=m[0]*m[3]-m[1]*m[2];if !d.is_finite()||d==0.0{return Err(invalid("Boolean reference space is singular"));}matrix([m[3]/d,-m[1]/d,-m[2]/d,m[0]/d,(m[2]*m[5]-m[3]*m[4])/d,(m[1]*m[4]-m[0]*m[5])/d])}
fn magnification(n:&DocumentSceneNode)->Result<f64,DocumentSceneError>{
 let DocumentSceneContent::Boolean{reference_transform:r,..}=&n.content else{return Ok(1.0)};let m=n.transform;let d=r[0]*r[3]-r[1]*r[2];if d==0.0{return Ok(1.0);}
 let a=(m[0]*r[3]-m[2]*r[1])/d;let b=(m[1]*r[3]-m[3]*r[1])/d;let c=(m[2]*r[0]-m[0]*r[2])/d;let e=(m[3]*r[0]-m[1]*r[2])/d;let scale=a.abs().max(b.abs()).max(c.abs()).max(e.abs());
 if !scale.is_finite(){return Err(invalid("Boolean document precision exceeds numeric limits"));}if scale==0.0{return Ok(1.0);}
 let(x,y,z,w)=(a/scale,b/scale,c/scale,e/scale);let norm=scale*((x*x+y*y+z*z+w*w+(x*x+y*y-z*z-w*w).hypot(2.0*(x*z+y*w)))/2.0).sqrt();
 if !norm.is_finite(){return Err(invalid("Boolean document precision exceeds numeric limits"));}Ok(norm.max(1.0))
}
fn point(p:Vec2,m:[f64;6])->Result<Vec2,DocumentSceneError>{let mut q=[m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]];for n in &mut q{if !n.is_finite()||n.abs()>1e9{return Err(invalid("Boolean document point exceeds coordinate limit"));}if *n==0.0{*n=0.0;}}Ok(q)}
fn refs(n:&DocumentSceneNode)->&[String]{match &n.content{DocumentSceneContent::Group{children,..}|DocumentSceneContent::Boolean{children,..}=>children,_=>&[]}}
fn config(l:DocumentBooleanLimits,operation:BooleanOperation,operands:Vec<PathBooleanOperand>)->PathBooleanInput{PathBooleanInput{operation,operands,epsilon:l.epsilon,max_edges:l.max_edges,max_parameters:l.max_parameters,max_atomic_edges:l.max_atomic_edges,max_segments:l.max_segments,max_work:l.max_work}}
#[derive(Clone,Copy)]
struct Frame{index:usize,at:usize}
/// ⏱️ Indexes shared references and remaps each filled result under explicit grants.
pub struct DocumentBooleanJob{
 input:DocumentBooleanInput,cleanup_slot:u8,retired_references:Vec<String>,retired_operation:Option<String>,phase:&'static str,work:u64,nodes:usize,resolved:usize,references:usize,retained:usize,source_segments:usize,validate_at:usize,ref_at:usize,root_at:usize,current:usize,publish_at:usize,
 ids:BTreeMap<String,usize>,visited:BTreeSet<usize>,visiting:BTreeSet<usize>,stack:Vec<Frame>,cache:BTreeMap<usize,Vec<PathSegment>>,local:BTreeMap<usize,Vec<DrawingSegment>>,
 order:Vec<usize>,impact:BTreeMap<usize,f64>,quality:BTreeMap<usize,f64>,requirement_current:Option<usize>,
 operands:Vec<PathBooleanOperand>,operand_at:usize,copy_at:usize,copied:usize,operand:Option<PathBooleanOperand>,child:Option<PathBooleanJob>,geometry:Option<PathBooleanProgress>,
 raw:Vec<PathSegment>,remap_at:usize,reference:[f64;6],local_result:Vec<DrawingSegment>,world_result:Vec<PathSegment>,output:Option<DocumentScenePlan>,failure:Option<DocumentSceneError>,cancelled:bool,
}
impl DocumentBooleanJob{
 pub fn new(input:DocumentBooleanInput)->Result<Self,DocumentSceneError>{
  let l=input.limits;if input.plan.nodes.len()>1024||input.plan.assets.len()>1024||!l.tolerance.is_finite()||!(1e-6..=16.0).contains(&l.tolerance)||!(1..=1024).contains(&l.max_depth)||!(1..=32768).contains(&l.max_references)||!(1..=262144).contains(&l.max_retained_segments){return Err(invalid("Invalid Boolean document contract or limits"));}
  let mut check=PathBooleanJob::new(config(l,BooleanOperation::Union,vec![PathBooleanOperand{segments:Vec::new(),transform:IDENTITY,tolerance:l.tolerance,fill_rule:BooleanFillRule::Nonzero}])).map_err(|e|invalid(e.to_string()))?;check.cancel();
  Ok(Self{input,cleanup_slot:0,retired_references:Vec::new(),retired_operation:None,phase:"indexing",work:0,nodes:0,resolved:0,references:0,retained:0,source_segments:0,validate_at:0,ref_at:0,root_at:0,current:0,publish_at:0,ids:BTreeMap::new(),visited:BTreeSet::new(),visiting:BTreeSet::new(),stack:Vec::new(),cache:BTreeMap::new(),local:BTreeMap::new(),order:Vec::new(),impact:BTreeMap::new(),quality:BTreeMap::new(),requirement_current:None,operands:Vec::new(),operand_at:0,copy_at:0,copied:0,operand:None,child:None,geometry:None,raw:Vec::new(),remap_at:0,reference:IDENTITY,local_result:Vec::new(),world_result:Vec::new(),output:None,failure:None,cancelled:false})
 }
 fn push(&mut self,index:usize)->Result<(),DocumentSceneError>{if self.stack.len()>=self.input.limits.max_depth{return Err(invalid("Boolean document dependency depth exceeded"));}if !self.visiting.insert(index){return Err(invalid("Cyclic Boolean document operands"));}self.stack.push(Frame{index,at:0});Ok(())}
 fn index(&mut self)->Result<(),DocumentSceneError>{
  let Some(n)=self.input.plan.nodes.get(self.nodes)else{self.phase="validation";return Ok(());};if n.id.is_empty()||n.id.len()>4096||self.ids.contains_key(&n.id){return Err(invalid("Invalid or duplicate Boolean document node"));}crate::schema::scene_preparation::validate_scene_source_address(&n.source_path,n.locked_ancestors)?;matrix(n.transform)?;
  if let DocumentSceneContent::Path{segments,..}=&n.content{if segments.len()>65536{return Err(invalid("Invalid Boolean document path"));}self.source_segments+=segments.len();if self.source_segments>65536{return Err(invalid("Boolean document source segment cap exceeded"));}}
  let children=refs(n);if children.len()>1024{return Err(invalid("Invalid Boolean document references"));}self.references+=children.len();if self.references>self.input.limits.max_references{return Err(invalid("Boolean document reference cap exceeded"));}
  if let DocumentSceneContent::Boolean{operation,reference_transform,..}=&n.content{BooleanOperation::parse(operation).map_err(|e|invalid(e.to_string()))?;matrix(*reference_transform)?;}
  self.ids.insert(n.id.clone(),self.nodes);self.nodes+=1;Ok(())
 }
 fn traversal(&mut self,cycles:bool)->Result<(),DocumentSceneError>{
  if self.stack.is_empty(){let Some(n)=self.input.plan.nodes.get(self.root_at)else{if cycles{if self.visited.pop_first().is_some(){return Ok(());}self.visited=BTreeSet::new();self.root_at=0;self.phase="requirements";}else{self.phase="publishing";}return Ok(());};let index=self.root_at;self.root_at+=1;if if cycles{self.visited.contains(&index)}else{!matches!(n.content,DocumentSceneContent::Boolean{..})||self.cache.contains_key(&index)}{return Ok(());}return self.push(index);}
  let top=*self.stack.last().unwrap();let n=&self.input.plan.nodes[top.index];let children=refs(n);
  if let Some(key)=children.get(top.at){let index=self.ids[key];self.stack.last_mut().unwrap().at+=1;if if cycles{self.visited.contains(&index)}else{self.cache.contains_key(&index)}{return Ok(());}return self.push(index);}
  if cycles{self.stack.pop();self.visiting.remove(&top.index);self.visited.insert(top.index);self.order.push(top.index);return Ok(());}
  if !matches!(n.content,DocumentSceneContent::Path{..}|DocumentSceneContent::Group{..}|DocumentSceneContent::Boolean{..}){return Err(invalid(format!("Unsupported Boolean operand: {}",n.id)));}
  self.current=top.index;self.operand_at=0;self.copy_at=0;self.copied=0;self.operand=None;self.operands=Vec::new();self.phase="operands";Ok(())
 }
 fn requirements(&mut self)->Result<(),DocumentSceneError>{
  let Some(index)=self.requirement_current else{
   let Some(index)=self.order.pop()else{if self.impact.pop_first().is_some(){return Ok(());}self.impact=BTreeMap::new();self.phase="traversal";return Ok(());};let factor=self.impact.get(&index).copied().unwrap_or(1.0)*magnification(&self.input.plan.nodes[index])?;
   if !factor.is_finite(){return Err(invalid("Boolean document precision exceeds numeric limits"));}self.quality.insert(index,factor);self.requirement_current=Some(index);self.ref_at=0;return Ok(());
  };
  let children=refs(&self.input.plan.nodes[index]);let Some(key)=children.get(self.ref_at)else{self.requirement_current=None;return Ok(());};let child=self.ids[key];let factor=self.quality[&index];let incoming=self.impact.get(&child).copied().unwrap_or(1.0);self.impact.insert(child,incoming.max(factor));self.ref_at+=1;Ok(())
 }
 fn prepare(&mut self)->Result<(),DocumentSceneError>{
  let n=&self.input.plan.nodes[self.current];let children=refs(n);let count=if matches!(n.content,DocumentSceneContent::Path{..}){1}else{children.len()};
  let factor=self.quality.get(&self.current).copied().unwrap_or(1.0);let mut limits=self.input.limits;limits.tolerance/=factor;limits.epsilon/=factor;if limits.tolerance<1e-6||limits.epsilon<1e-12{return Err(invalid("Boolean document precision exceeds supported tolerance"));}
  if self.operand.is_none(){
   if self.operand_at==count{if self.operands.is_empty(){self.operands.push(PathBooleanOperand{segments:Vec::new(),transform:IDENTITY,tolerance:limits.tolerance,fill_rule:BooleanFillRule::Nonzero});}let operation=if let DocumentSceneContent::Boolean{operation,..}=&n.content{BooleanOperation::parse(operation).map_err(|e|invalid(e.to_string()))?}else{BooleanOperation::Union};let operands=std::mem::take(&mut self.operands);self.child=Some(PathBooleanJob::new(config(limits,operation,operands)).map_err(|e|invalid(e.to_string()))?);self.phase="geometry";return Ok(());}
   let(transform,fill_rule)=if let DocumentSceneContent::Path{fill_rule,..}=&n.content{(n.transform,if *fill_rule==crate::FillRule::Evenodd{BooleanFillRule::Evenodd}else{BooleanFillRule::Nonzero})}else{(IDENTITY,BooleanFillRule::Nonzero)};
   self.operand=Some(PathBooleanOperand{segments:Vec::new(),transform,tolerance:limits.tolerance,fill_rule});self.copy_at=0;return Ok(());
  }
  let next=if let DocumentSceneContent::Path{segments,..}=&n.content{segments.get(self.copy_at).map(crate::standards::v1::subsets::any::schema::component::to_kernel_segment)}else{self.cache[&self.ids[&children[self.operand_at]]].get(self.copy_at).cloned()};
  if let Some(next)=next{if self.copied>=self.input.limits.max_edges{return Err(invalid("Boolean document operand segment budget exceeded"));}self.operand.as_mut().unwrap().segments.push(next);self.copied+=1;self.copy_at+=1;return Ok(());}
  self.operands.push(self.operand.take().unwrap());self.operand_at+=1;Ok(())
 }
 fn finish_node(&mut self){self.resolved+=1;let top=self.stack.pop().unwrap();self.visiting.remove(&top.index);self.phase="traversal";}
 fn complete_geometry(&mut self)->Result<(),DocumentSceneError>{
  let n=&self.input.plan.nodes[self.current];if let DocumentSceneContent::Boolean{reference_transform,..}=&n.content{self.reference=if self.raw.is_empty(){IDENTITY}else{inverse(*reference_transform)?};self.remap_at=0;self.local_result=Vec::new();self.world_result=Vec::new();self.phase="remapping";return Ok(());}
  if self.retained+self.raw.len()>self.input.limits.max_retained_segments{return Err(invalid("Boolean document retained geometry cap exceeded"));}self.retained+=self.raw.len();self.cache.insert(self.current,std::mem::take(&mut self.raw));self.finish_node();Ok(())
 }
 fn step(&mut self)->Result<(),DocumentSceneError>{
  match self.phase{
   "indexing"=>self.index()?,
   "validation"=>{let Some(n)=self.input.plan.nodes.get(self.validate_at)else{self.phase="cycles";return Ok(());};let children=refs(n);if let Some(key)=children.get(self.ref_at){if !self.ids.contains_key(key){return Err(invalid("Missing Boolean document operand"));}self.ref_at+=1;}else{self.validate_at+=1;self.ref_at=0;}},
   "cycles"|"traversal"=>self.traversal(self.phase=="cycles")?,
   "requirements"=>self.requirements()?,
   "operands"=>self.prepare()?,
   "geometry"=>{let p=self.child.as_mut().unwrap().advance(1).map_err(|e|invalid(e.to_string()))?;self.geometry=Some(p);if p.done{self.raw=self.child.take().unwrap().into_result().map_err(|e|invalid(e.to_string()))?;self.complete_geometry()?;}},
   "remapping"=>{
    let Some(s)=self.raw.get(self.remap_at)else{self.cache.insert(self.current,std::mem::take(&mut self.world_result));self.local.insert(self.current,std::mem::take(&mut self.local_result));self.raw=Vec::new();self.finish_node();return Ok(());};
    if self.retained+2>self.input.limits.max_retained_segments{return Err(invalid("Boolean document retained geometry cap exceeded"));}
    match s{PathSegment::Move{to}|PathSegment::Line{to}=>{let local=point(*to,self.reference)?;let world=point(local,self.input.plan.nodes[self.current].transform)?;if matches!(s,PathSegment::Move{..}){self.local_result.push(DrawingSegment::Move{to:local});self.world_result.push(PathSegment::Move{to:world});}else{self.local_result.push(DrawingSegment::Line{to:local});self.world_result.push(PathSegment::Line{to:world});}},PathSegment::Close=>{self.local_result.push(DrawingSegment::Close);self.world_result.push(PathSegment::Close);},_=>return Err(invalid("Unresolved Boolean document curve"))}
    self.retained+=2;self.remap_at+=1;
   }
   "publishing"=>{
    if self.retired_references.pop().is_some(){return Ok(());}if self.retired_references.capacity()!=0{self.retired_references=Vec::new();return Ok(());}if self.retired_operation.take().is_some(){return Ok(());}
    if self.publish_at==self.input.plan.nodes.len(){if self.cleanup(){self.output=Some(std::mem::take(&mut self.input.plan));self.phase="complete";}return Ok(());}
    let index=self.publish_at;self.publish_at+=1;let content=&mut self.input.plan.nodes[index].content;
    if matches!(content,DocumentSceneContent::Boolean{..}){let old=std::mem::replace(content,DocumentSceneContent::Group{children:Vec::new(),isolation:false});let DocumentSceneContent::Boolean{operation,children,fill_rule,fill,stroke,..}=old else{unreachable!()};self.retired_references=children;self.retired_operation=Some(operation);*content=DocumentSceneContent::Path{segments:self.local.remove(&index).unwrap(),fill_rule,fill,stroke};}
   }
   _=>{}
  }Ok(())
 }
 fn cleanup(&mut self)->bool{
  match self.cleanup_slot{
   0=>{if self.cache.pop_first().is_some(){return false;}self.cache=BTreeMap::new();},
   1=>{if self.local.pop_first().is_some(){return false;}self.local=BTreeMap::new();},
   2=>{if self.ids.pop_first().is_some(){return false;}self.ids=BTreeMap::new();},
   3=>{if self.quality.pop_first().is_some(){return false;}self.quality=BTreeMap::new();},
   4=>{if self.impact.pop_first().is_some(){return false;}self.impact=BTreeMap::new();},
   5=>{if self.visited.pop_first().is_some(){return false;}self.visited=BTreeSet::new();},
   6=>{if self.visiting.pop_first().is_some(){return false;}self.visiting=BTreeSet::new();},
   7=>self.stack=Vec::new(),8=>self.order=Vec::new(),_=>unreachable!(),
  }self.cleanup_slot+=1;self.cleanup_slot==9
 }
 pub fn advance(&mut self,budget:usize)->Result<DocumentBooleanProgress,DocumentSceneError>{
  if budget==0||budget as u128>9_007_199_254_740_991{return Err(invalid("Invalid Boolean document work grant"));}if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}
  for _ in 0..budget{if self.phase=="complete"{break;}let step=if self.work>=self.input.limits.max_work{Err(invalid("Boolean document work cap exceeded"))}else{self.step()};if let Err(error)=step{self.failure=Some(error.clone());return Err(error);}self.work+=1;}
  Ok(DocumentBooleanProgress{phase:self.phase,nodes:self.nodes,resolved:self.resolved,references:self.references,segments:self.retained,work:self.work,geometry:self.geometry,done:self.phase=="complete"})
 }
 pub fn result(&self)->Result<&DocumentScenePlan,DocumentSceneError>{if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}self.output.as_ref().ok_or(DocumentSceneError::Incomplete)}
 pub fn into_result(mut self)->Result<DocumentScenePlan,DocumentSceneError>{self.result()?;Ok(self.output.take().unwrap())}
 /// 🧹️ Move source and completed output; retain interrupted child and private candidate ownership.
 pub fn into_retirement(mut self)->(DocumentBooleanRetirement,DocumentScenePlan,Option<DocumentScenePlan>){
  let child=if let Some(job)=self.child.take(){let(close,output)=job.into_retirement();if let Some(output)=output{self.raw=output;}Some(close)}else{None};
  let output=if self.phase=="complete"&&!self.cancelled&&self.failure.is_none(){self.output.take()}else{None};let draft=self.output.take().map(ScenePlanCloseJob::new);let input=std::mem::take(&mut self.input.plan);self.cancelled=true;
  (DocumentBooleanRetirement{job:Some(self),child,draft,slot:0,counter:WorkRetirementCounter::default()},input,output)
 }
 fn clear(&mut self){if let Some(child)=&mut self.child{child.cancel();}self.child=None;self.retired_references=Vec::new();self.retired_operation=None;self.input.plan=DocumentScenePlan::default();self.ids.clear();self.visited.clear();self.visiting.clear();self.stack=Vec::new();self.order=Vec::new();self.impact.clear();self.quality.clear();self.requirement_current=None;self.cache.clear();self.local.clear();self.operands=Vec::new();self.operand=None;self.raw=Vec::new();self.local_result=Vec::new();self.world_result=Vec::new();self.output=None;}
 pub fn cancel(&mut self){self.cancelled=true;self.clear();}
}
/// 🧽️ Preserve private ownership until scene, path, tree and operand retirement reaches terminal state.
pub struct DocumentBooleanRetirement{job:Option<DocumentBooleanJob>,child:Option<PathBooleanRetirement>,draft:Option<ScenePlanCloseJob>,slot:u8,counter:WorkRetirementCounter}
impl DocumentBooleanRetirement{
 pub fn terminal_is_empty(&self)->bool{self.job.is_none()&&self.child.is_none()&&self.draft.is_none()}
 pub fn advance(&mut self,grant:usize)->Result<DocumentBooleanRetirementProgress,DocumentSceneError>{let mut counter=self.counter;let p=counter.advance(grant,||self.step()).map_err(invalid)?;self.counter=counter;Ok(p)}
 fn step(&mut self)->bool{
  let Some(job)=self.job.as_mut()else{return true;};
  match self.slot{
   0=>{if let Some(close)=self.child.as_mut(){if !close.advance(1).expect("positive internal grant").done{return false;}self.child=None;}},
   1=>{if let Some(close)=self.draft.as_mut(){if !close.advance(1).expect("positive internal grant").done{return false;}self.draft=None;}},
   2=>{},
   3=>{if job.ids.pop_first().is_some(){return false;}job.ids=BTreeMap::new();},
   4=>{if job.visited.pop_first().is_some(){return false;}job.visited=BTreeSet::new();},
   5=>{if job.visiting.pop_first().is_some(){return false;}job.visiting=BTreeSet::new();},
   6=>{if job.cache.pop_first().is_some(){return false;}job.cache=BTreeMap::new();},
   7=>{if job.local.pop_first().is_some(){return false;}job.local=BTreeMap::new();},
   8=>{if job.impact.pop_first().is_some(){return false;}job.impact=BTreeMap::new();},
   9=>{if job.quality.pop_first().is_some(){return false;}job.quality=BTreeMap::new();},
   10=>job.stack=Vec::new(),11=>job.order=Vec::new(),
   12=>{if job.operands.pop().is_some(){return false;}job.operands=Vec::new();},
   13=>job.operand=None,14=>job.raw=Vec::new(),15=>job.local_result=Vec::new(),16=>job.world_result=Vec::new(),
   17=>{if job.retired_references.pop().is_some(){return false;}job.retired_references=Vec::new();},
   18=>job.retired_operation=None,19=>job.input.plan=DocumentScenePlan::default(),20=>job.output=None,_=>unreachable!(),
  }self.slot+=1;if self.slot==21{self.job=None;return true;}false
 }
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

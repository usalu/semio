//! 🛤️ Curved transformed fill operands enter work-granted planar booleans.
use super::{BooleanError,BooleanFillRule,BooleanInput,BooleanJob,BooleanOperand,BooleanOperation,BooleanProgress,BooleanRetirement};
use crate::{PathSegment,Vec2};
use crate::flatten::{FlatContour,PathFlattenError,PathFlattenInput,PathFlattenJob,PathFlattenProgress,PathFlattenRetirement};
use crate::retirement::WorkRetirementProgress;
use std::collections::VecDeque;
pub type PathBooleanRetirementProgress=WorkRetirementProgress;
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct PathBooleanOperand{pub segments:Vec<PathSegment>,pub transform:[f64;6],pub tolerance:f64,pub fill_rule:BooleanFillRule}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct PathBooleanInput{pub operation:BooleanOperation,pub operands:Vec<PathBooleanOperand>,pub epsilon:f64,pub max_edges:usize,pub max_parameters:usize,pub max_atomic_edges:usize,pub max_segments:usize,pub max_work:u64}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PathBooleanPhase{Admitting,Flattening,Transforming,Boolean,Complete}
impl PathBooleanPhase{pub fn as_str(self)->&'static str{match self{Self::Admitting=>"admitting",Self::Flattening=>"flattening",Self::Transforming=>"transforming",Self::Boolean=>"boolean",Self::Complete=>"complete"}}}
#[derive(Clone,Copy,Debug)]
pub struct PathBooleanProgress{pub phase:PathBooleanPhase,pub operands:usize,pub source_segments:usize,pub points:usize,pub work:u64,pub flatten:Option<PathFlattenProgress>,pub boolean:Option<BooleanProgress>,pub done:bool}
#[derive(semio_framework_value::RetireOwned)]
struct Current{source:VecDeque<PathSegment>,transform:[f64;6],tolerance:f64,fill_rule:BooleanFillRule}
fn flatten_error(error:PathFlattenError)->BooleanError{match error{PathFlattenError::Invalid(message)=>BooleanError::Invalid(message),PathFlattenError::Incomplete=>BooleanError::Incomplete,PathFlattenError::Cancelled=>BooleanError::Cancelled}}
fn config(input:&PathBooleanInput,operands:Vec<BooleanOperand>)->BooleanInput{BooleanInput{operation:input.operation,operands,epsilon:input.epsilon,max_edges:input.max_edges,max_parameters:input.max_parameters,max_atomic_edges:input.max_atomic_edges,max_segments:input.max_segments,max_work:input.max_work}}
/// ⏱️ Admits paths, flattens in world tolerance and transforms one point per grant.
#[derive(semio_framework_value::RetireOwned)]
pub struct PathBooleanJob{
 input:PathBooleanInput,source:VecDeque<PathBooleanOperand>,phase:PathBooleanPhase,work:u64,operands:usize,source_segments:usize,points:usize,
 current:Option<Current>,admitted:Vec<PathSegment>,local:VecDeque<FlatContour>,contour:Option<VecDeque<Vec2>>,world:Vec<Vec<Vec2>>,prepared:Vec<BooleanOperand>,
 retired_sources:Vec<VecDeque<PathSegment>>,retired_locals:Vec<VecDeque<FlatContour>>,retired_contours:Vec<VecDeque<Vec2>>,flat_retirements:Vec<PathFlattenRetirement>,flat_retirement:Option<PathFlattenRetirement>,boolean_retirement:Option<BooleanRetirement>,retained:Vec<BooleanOperand>,
 flatten:Option<PathFlattenJob>,flat_progress:Option<PathFlattenProgress>,boolean:Option<BooleanJob>,boolean_progress:Option<BooleanProgress>,output:Vec<PathSegment>,cancelled:bool,failure:Option<BooleanError>,
}
impl PathBooleanJob{
 pub fn new(mut input:PathBooleanInput)->Result<Self,BooleanError>{
  if input.operands.is_empty()||input.operands.len()>1024{return Err(BooleanError::Invalid("Invalid boolean path operands"));}
  let mut check=BooleanJob::new(config(&input,vec![BooleanOperand{contours:Vec::new(),fill_rule:BooleanFillRule::Nonzero}]))?;check.cancel();let source=std::mem::take(&mut input.operands).into();
  Ok(Self{input,source,phase:PathBooleanPhase::Admitting,work:0,operands:0,source_segments:0,points:0,current:None,admitted:Vec::new(),local:Vec::new().into(),contour:None,world:Vec::new(),prepared:Vec::new(),retired_sources:Vec::new(),retired_locals:Vec::new(),retired_contours:Vec::new(),flat_retirements:Vec::new(),flat_retirement:None,boolean_retirement:None,retained:Vec::new(),flatten:None,flat_progress:None,boolean:None,boolean_progress:None,output:Vec::new(),cancelled:false,failure:None})
 }
 fn step(&mut self)->Result<(),BooleanError>{
  match self.phase{
   PathBooleanPhase::Admitting=>{
    if self.current.is_none(){
     if let Some(operand)=self.source.pop_front(){
      self.current=Some(Current{source:operand.segments.into(),transform:operand.transform,tolerance:operand.tolerance,fill_rule:operand.fill_rule});let current=self.current.as_ref().unwrap();
      if current.source.len()>65536||!current.transform.into_iter().all(|n|n.is_finite()&&n.abs()<=1e9)||!current.tolerance.is_finite()||!(1e-6..=16.0).contains(&current.tolerance){return Err(BooleanError::Invalid("Invalid boolean path operand"));}return Ok(());
     }
     self.boolean=Some(BooleanJob::new(config(&self.input,std::mem::take(&mut self.prepared)))?);self.phase=PathBooleanPhase::Boolean;return Ok(());
    }
    let current=self.current.as_mut().unwrap();if let Some(segment)=current.source.pop_front(){if self.source_segments>=self.input.max_edges{return Err(BooleanError::Invalid("Boolean paths exceed source segment budget"));}self.admitted.push(segment);self.source_segments+=1;return Ok(());}
    self.flatten=Some(PathFlattenJob::new(PathFlattenInput{segments:std::mem::take(&mut self.admitted),transform:current.transform,tolerance:current.tolerance}).map_err(flatten_error)?);self.phase=PathBooleanPhase::Flattening;
   }
   PathBooleanPhase::Flattening=>{

    let p=self.flatten.as_mut().unwrap().advance(1).map_err(flatten_error)?;self.flat_progress=Some(p);if self.points+p.points>self.input.max_edges{return Err(BooleanError::Invalid("Boolean paths exceed flattened point budget"));}
    if p.done{let(close,output)=self.flatten.take().unwrap().into_retirement();self.retired_locals.push(std::mem::replace(&mut self.local,output.unwrap().into()));if let Some(previous)=self.flat_retirement.replace(close){self.flat_retirements.push(previous);}self.phase=PathBooleanPhase::Transforming;}
   }
   PathBooleanPhase::Transforming=>{
    if let Some(points)=self.contour.as_mut(){
     if let Some(p)=points.pop_front(){let m=self.current.as_ref().unwrap().transform;let q=[m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]];if !q.into_iter().all(|n|n.is_finite()&&n.abs()<=1e9){return Err(BooleanError::Invalid("Boolean world point exceeds coordinate budget"));}if self.points>=self.input.max_edges{return Err(BooleanError::Invalid("Boolean paths exceed flattened point budget"));}self.world.last_mut().unwrap().push(q);self.points+=1;return Ok(());}self.retired_contours.push(self.contour.take().unwrap());return Ok(());
    }
    if let Some(contour)=self.local.pop_front(){self.world.push(Vec::new());self.contour=Some(contour.points.into());return Ok(());}
    let current=self.current.take().unwrap();self.retired_sources.push(current.source);self.prepared.push(BooleanOperand{contours:std::mem::take(&mut self.world),fill_rule:current.fill_rule});self.operands+=1;self.phase=PathBooleanPhase::Admitting;
   }
   PathBooleanPhase::Boolean=>{


    let p=self.boolean.as_mut().unwrap().advance(1)?;self.boolean_progress=Some(p);if p.done{let(close,operands,output)=self.boolean.take().unwrap().into_retirement();self.output=output.unwrap();self.retained=operands;self.boolean_retirement=Some(close);self.phase=PathBooleanPhase::Complete;}
   }
   PathBooleanPhase::Complete=>{}
  }Ok(())
 }
 pub fn advance(&mut self,budget:usize)->Result<PathBooleanProgress,BooleanError>{
  if budget==0||budget as u128>9_007_199_254_740_991{return Err(BooleanError::Invalid("Invalid boolean path work grant"));}if self.cancelled{return Err(BooleanError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}
  for _ in 0..budget{if self.phase==PathBooleanPhase::Complete{break;}let result=if self.work>=self.input.max_work{Err(BooleanError::Invalid("Boolean paths exceed work budget"))}else{self.step()};if let Err(error)=result{self.failure=Some(error.clone());return Err(error);}self.work+=1;}
  Ok(PathBooleanProgress{phase:self.phase,operands:self.operands,source_segments:self.source_segments,points:self.points,work:self.work,flatten:self.flat_progress,boolean:self.boolean_progress,done:self.phase==PathBooleanPhase::Complete})
 }
 pub fn result(&self)->Result<&[PathSegment],BooleanError>{if self.cancelled{return Err(BooleanError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}if self.phase!=PathBooleanPhase::Complete{return Err(BooleanError::Incomplete);}Ok(&self.output)}
 pub fn into_result(mut self)->Result<Vec<PathSegment>,BooleanError>{let status=self.result().map(|_|());let output=if status.is_ok(){std::mem::take(&mut self.output)}else{Vec::new()};let(mut close,_)=self.into_retirement();while !close.advance(4096)?.done{}status.map(|_|output)}
 /// 🧹️ Move valid output and compose interrupted children without eager destruction.
 pub fn into_retirement(mut self)->(PathBooleanRetirement,Option<Vec<PathSegment>>){
  if let Some(job)=self.flatten.take(){let(close,output)=job.into_retirement();if let Some(previous)=self.flat_retirement.replace(close){self.flat_retirements.push(previous);}if let Some(output)=output{self.retired_locals.push(std::mem::replace(&mut self.local,output.into()));}}
  if let Some(job)=self.boolean.take(){let(close,operands,output)=job.into_retirement();self.boolean_retirement=Some(close);self.retained=operands;if let Some(output)=output{self.output=output;}}
  let output=if self.phase==PathBooleanPhase::Complete&&!self.cancelled&&self.failure.is_none(){Some(std::mem::take(&mut self.output))}else{None};self.cancelled=true;
  (PathBooleanRetirement::new(self),output)
 }
 pub fn cancel(&mut self){self.cancelled=true;}
}
crate::physical_work_retirement!(PathBooleanRetirement,PathBooleanJob,BooleanError,|_:&str|BooleanError::Invalid("Boolean path physical close refused"));
semio_framework_value::artifact_retire_leaf!(PathBooleanPhase,PathBooleanProgress);
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

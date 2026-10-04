//! 🛤️ Curved transformed fill operands enter work-granted planar booleans.
use super::{BooleanError,BooleanFillRule,BooleanInput,BooleanJob,BooleanOperand,BooleanOperation,BooleanProgress,BooleanRetirement};
use crate::{PathSegment,Vec2};
use crate::flatten::{FlatContour,PathFlattenError,PathFlattenInput,PathFlattenJob,PathFlattenProgress,PathFlattenRetirement};
use crate::retirement::{WorkRetirementCounter,WorkRetirementProgress};
pub type PathBooleanRetirementProgress=WorkRetirementProgress;
#[derive(Clone,Debug)]
pub struct PathBooleanOperand{pub segments:Vec<PathSegment>,pub transform:[f64;6],pub tolerance:f64,pub fill_rule:BooleanFillRule}
#[derive(Clone,Debug)]
pub struct PathBooleanInput{pub operation:BooleanOperation,pub operands:Vec<PathBooleanOperand>,pub epsilon:f64,pub max_edges:usize,pub max_parameters:usize,pub max_atomic_edges:usize,pub max_segments:usize,pub max_work:u64}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PathBooleanPhase{Admitting,Flattening,Transforming,Boolean,Complete}
impl PathBooleanPhase{pub fn as_str(self)->&'static str{match self{Self::Admitting=>"admitting",Self::Flattening=>"flattening",Self::Transforming=>"transforming",Self::Boolean=>"boolean",Self::Complete=>"complete"}}}
#[derive(Clone,Copy,Debug)]
pub struct PathBooleanProgress{pub phase:PathBooleanPhase,pub operands:usize,pub source_segments:usize,pub points:usize,pub work:u64,pub flatten:Option<PathFlattenProgress>,pub boolean:Option<BooleanProgress>,pub done:bool}
struct Current{source:std::vec::IntoIter<PathSegment>,transform:[f64;6],tolerance:f64,fill_rule:BooleanFillRule}
fn flatten_error(error:PathFlattenError)->BooleanError{match error{PathFlattenError::Invalid(message)=>BooleanError::Invalid(message),PathFlattenError::Incomplete=>BooleanError::Incomplete,PathFlattenError::Cancelled=>BooleanError::Cancelled}}
fn config(input:&PathBooleanInput,operands:Vec<BooleanOperand>)->BooleanInput{BooleanInput{operation:input.operation,operands,epsilon:input.epsilon,max_edges:input.max_edges,max_parameters:input.max_parameters,max_atomic_edges:input.max_atomic_edges,max_segments:input.max_segments,max_work:input.max_work}}
/// ⏱️ Admits paths, flattens in world tolerance and transforms one point per grant.
pub struct PathBooleanJob{
 input:PathBooleanInput,source:std::vec::IntoIter<PathBooleanOperand>,phase:PathBooleanPhase,work:u64,operands:usize,source_segments:usize,points:usize,
 current:Option<Current>,admitted:Vec<PathSegment>,local:std::vec::IntoIter<FlatContour>,contour:Option<std::vec::IntoIter<Vec2>>,world:Vec<Vec<Vec2>>,prepared:Vec<BooleanOperand>,
 flat_retirement:Option<PathFlattenRetirement>,boolean_retirement:Option<BooleanRetirement>,retained:Vec<BooleanOperand>,retained_operand:Option<BooleanOperand>,
 flatten:Option<PathFlattenJob>,flat_progress:Option<PathFlattenProgress>,boolean:Option<BooleanJob>,boolean_progress:Option<BooleanProgress>,output:Vec<PathSegment>,cancelled:bool,failure:Option<BooleanError>,
}
impl PathBooleanJob{
 pub fn new(mut input:PathBooleanInput)->Result<Self,BooleanError>{
  if input.operands.is_empty()||input.operands.len()>1024{return Err(BooleanError::Invalid("Invalid boolean path operands"));}
  let mut check=BooleanJob::new(config(&input,vec![BooleanOperand{contours:Vec::new(),fill_rule:BooleanFillRule::Nonzero}]))?;check.cancel();let source=std::mem::take(&mut input.operands).into_iter();
  Ok(Self{input,source,phase:PathBooleanPhase::Admitting,work:0,operands:0,source_segments:0,points:0,current:None,admitted:Vec::new(),local:Vec::new().into_iter(),contour:None,world:Vec::new(),prepared:Vec::new(),flat_retirement:None,boolean_retirement:None,retained:Vec::new(),retained_operand:None,flatten:None,flat_progress:None,boolean:None,boolean_progress:None,output:Vec::new(),cancelled:false,failure:None})
 }
 fn step(&mut self)->Result<(),BooleanError>{
  match self.phase{
   PathBooleanPhase::Admitting=>{
    if self.current.is_none(){
     if let Some(operand)=self.source.next(){
      if operand.segments.len()>65536||!operand.transform.into_iter().all(|n|n.is_finite()&&n.abs()<=1e9)||!operand.tolerance.is_finite()||!(1e-6..=16.0).contains(&operand.tolerance){return Err(BooleanError::Invalid("Invalid boolean path operand"));}
      self.current=Some(Current{source:operand.segments.into_iter(),transform:operand.transform,tolerance:operand.tolerance,fill_rule:operand.fill_rule});return Ok(());
     }
     self.boolean=Some(BooleanJob::new(config(&self.input,std::mem::take(&mut self.prepared)))?);self.phase=PathBooleanPhase::Boolean;return Ok(());
    }
    let current=self.current.as_mut().unwrap();if let Some(segment)=current.source.next(){if self.source_segments>=self.input.max_edges{return Err(BooleanError::Invalid("Boolean paths exceed source segment budget"));}self.admitted.push(segment);self.source_segments+=1;return Ok(());}
    self.flatten=Some(PathFlattenJob::new(PathFlattenInput{segments:std::mem::take(&mut self.admitted),transform:current.transform,tolerance:current.tolerance}).map_err(flatten_error)?);self.phase=PathBooleanPhase::Flattening;
   }
   PathBooleanPhase::Flattening=>{
    if let Some(close)=self.flat_retirement.as_mut(){if close.advance(1).map_err(flatten_error)?.done{self.flat_retirement=None;self.phase=PathBooleanPhase::Transforming;}return Ok(());}
    let p=self.flatten.as_mut().unwrap().advance(1).map_err(flatten_error)?;self.flat_progress=Some(p);if self.points+p.points>self.input.max_edges{return Err(BooleanError::Invalid("Boolean paths exceed flattened point budget"));}
    if p.done{let(close,output)=self.flatten.take().unwrap().into_retirement();self.local=output.unwrap().into_iter();self.flat_retirement=Some(close);}
   }
   PathBooleanPhase::Transforming=>{
    if let Some(points)=self.contour.as_mut(){
     if let Some(p)=points.next(){let m=self.current.as_ref().unwrap().transform;let q=[m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]];if !q.into_iter().all(|n|n.is_finite()&&n.abs()<=1e9){return Err(BooleanError::Invalid("Boolean world point exceeds coordinate budget"));}if self.points>=self.input.max_edges{return Err(BooleanError::Invalid("Boolean paths exceed flattened point budget"));}self.world.last_mut().unwrap().push(q);self.points+=1;return Ok(());}self.contour=None;return Ok(());
    }
    if let Some(contour)=self.local.next(){self.world.push(Vec::new());self.contour=Some(contour.points.into_iter());return Ok(());}
    self.prepared.push(BooleanOperand{contours:std::mem::take(&mut self.world),fill_rule:self.current.take().unwrap().fill_rule});self.operands+=1;self.phase=PathBooleanPhase::Admitting;
   }
   PathBooleanPhase::Boolean=>{
    if let Some(close)=self.boolean_retirement.as_mut(){if close.advance(1)?.done{self.boolean_retirement=None;}return Ok(());}
    if self.boolean.is_none(){if self.retire_operands(false){self.phase=PathBooleanPhase::Complete;}return Ok(());}
    let p=self.boolean.as_mut().unwrap().advance(1)?;self.boolean_progress=Some(p);if p.done{let(close,operands,output)=self.boolean.take().unwrap().into_retirement();self.output=output.unwrap();self.retained=operands;self.boolean_retirement=Some(close);}
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
 pub fn into_result(self)->Result<Vec<PathSegment>,BooleanError>{self.result()?;Ok(self.output)}
 fn retire_operands(&mut self,prepared:bool)->bool{
  if let Some(operand)=self.retained_operand.as_mut(){if operand.contours.pop().is_some(){return false;}self.retained_operand=None;return false;}
  let items=if prepared{&mut self.prepared}else{&mut self.retained};self.retained_operand=items.pop();if self.retained_operand.is_some(){return false;}*items=Vec::new();true
 }
 /// 🧹️ Move valid output and compose interrupted children without eager destruction.
 pub fn into_retirement(mut self)->(PathBooleanRetirement,Option<Vec<PathSegment>>){
  if let Some(job)=self.flatten.take(){let(close,output)=job.into_retirement();self.flat_retirement=Some(close);if let Some(output)=output{self.local=output.into_iter();}}
  if let Some(job)=self.boolean.take(){let(close,operands,output)=job.into_retirement();self.boolean_retirement=Some(close);self.retained=operands;if let Some(output)=output{self.output=output;}}
  let output=if self.phase==PathBooleanPhase::Complete&&!self.cancelled&&self.failure.is_none(){Some(std::mem::take(&mut self.output))}else{None};self.cancelled=true;
  (PathBooleanRetirement{job:Some(self),slot:0,counter:WorkRetirementCounter::default()},output)
 }
 fn clear(&mut self){if let Some(job)=&mut self.flatten{job.cancel();}if let Some(job)=&mut self.boolean{job.cancel();}self.source=Vec::new().into_iter();self.current=None;self.flatten=None;self.boolean=None;self.flat_retirement=None;self.boolean_retirement=None;self.retained=Vec::new();self.retained_operand=None;self.admitted=Vec::new();self.local=Vec::new().into_iter();self.contour=None;self.world=Vec::new();self.prepared=Vec::new();self.output=Vec::new();}
 pub fn cancel(&mut self){self.cancelled=true;self.clear();}
}
/// 🧽️ Retire child arrangements, unconsumed operands and private contours under one work budget.
pub struct PathBooleanRetirement{job:Option<PathBooleanJob>,slot:u8,counter:WorkRetirementCounter}
impl PathBooleanRetirement{
 pub fn terminal_is_empty(&self)->bool{self.job.is_none()}
 pub fn advance(&mut self,grant:usize)->Result<PathBooleanRetirementProgress,BooleanError>{let mut counter=self.counter;let p=counter.advance(grant,||self.step()).map_err(BooleanError::Invalid)?;self.counter=counter;Ok(p)}
 fn step(&mut self)->bool{
  let Some(job)=self.job.as_mut()else{return true;};
  match self.slot{
   0=>{if let Some(close)=job.flat_retirement.as_mut(){if !close.advance(1).expect("positive internal grant").done{return false;}job.flat_retirement=None;}},
   1=>{if let Some(close)=job.boolean_retirement.as_mut(){if !close.advance(1).expect("positive internal grant").done{return false;}job.boolean_retirement=None;}},
   2=>{if !job.retire_operands(false){return false;}},
   3=>{if job.source.next().is_some(){return false;}job.source=Vec::new().into_iter();},
   4=>job.current=None,
   5=>job.admitted=Vec::new(),
   6=>{if job.local.next().is_some(){return false;}job.local=Vec::new().into_iter();},
   7=>job.contour=None,
   8=>{if job.world.pop().is_some(){return false;}job.world=Vec::new();},
   9=>{if !job.retire_operands(true){return false;}},
   10=>job.output=Vec::new(),
   11=>job.input.operands=Vec::new(),
   _=>unreachable!(),
  }
  self.slot+=1;if self.slot==12{self.job=None;return true;}false
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

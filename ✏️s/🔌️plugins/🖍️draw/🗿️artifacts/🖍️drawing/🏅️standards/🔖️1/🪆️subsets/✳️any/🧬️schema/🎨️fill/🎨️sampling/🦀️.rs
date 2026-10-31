//! 🎨️ Stable local-space gradient preparation and allocation-free bounded paint queries.
use crate::{FillStyle,GradientStop};
use semio_framework_2d::physical_work_retirement;
use semio_framework_value::{work::RetainedWorkLedger,RetirementDemand};
use semio_framework_value_persistent::list::PagedList;
pub const MAX_PAINT_STOPS:usize=4096;
fn unit(value:f64)->bool{value.is_finite()&&(0.0..=1.0).contains(&value)}
fn valid_color(color:&[f64;4])->bool{color.iter().all(|value|unit(*value))}
fn valid_stop(stop:&GradientStop)->bool{unit(stop.offset)&&valid_color(&stop.color)}
fn source_stops(fill:&FillStyle)->Option<&PagedList<GradientStop,{usize::MAX}>>{match fill{FillStyle::Solid{..}=>None,FillStyle::LinearGradient{stops,..}|FillStyle::RadialGradient{stops,..}=>Some(stops)}}
#[derive(Clone,Copy,semio_framework_value::RetireOwned)]
enum Geometry{Solid([f64;4]),Linear{origin:[f64;2],unit:[f64;2],length:f64},Radial{center:[f64;2],radius:f64}}
fn geometry(fill:&FillStyle)->Result<Geometry,&'static str>{match fill{
 FillStyle::Solid{color}=>{if !valid_color(color){return Err("Invalid fill color")}Ok(Geometry::Solid(*color))}
 FillStyle::LinearGradient{x1,y1,x2,y2,..}=>{let dx=x2-x1;let dy=y2-y1;let length=dx.hypot(dy);if ![x1,y1,x2,y2,&length].iter().all(|value|value.is_finite()){return Err("Invalid linear gradient geometry")}Ok(Geometry::Linear{origin:[*x1,*y1],unit:if length==0.{[0.;2]}else{[dx/length,dy/length]},length})}
 FillStyle::RadialGradient{cx,cy,r,..}=>{if ![cx,cy,r].iter().all(|value|value.is_finite())||*r<0.{return Err("Invalid radial gradient geometry")}Ok(Geometry::Radial{center:[*cx,*cy],radius:*r})}
}}
fn offset(geometry:Geometry,point:[f64;2])->Result<f64,&'static str>{if !point.into_iter().all(f64::is_finite){return Err("Fill sample position must be finite")}Ok(match geometry{Geometry::Solid(_)=>0.,Geometry::Linear{origin,unit,length}=>if length==0.{1.}else{((point[0]-origin[0])*unit[0]+(point[1]-origin[1])*unit[1])/length},Geometry::Radial{center,radius}=>if radius==0.{1.}else{(point[0]-center[0]).hypot(point[1]-center[1])/radius}})}
fn interpolate(left:&GradientStop,right:&GradientStop,position:f64)->[f64;4]{let t=(position-left.offset)/(right.offset-left.offset);std::array::from_fn(|index|left.color[index]+(right.color[index]-left.color[index])*t)}
/// 🔍️ Scans bounded authored stops without creating retained storage, preserving stable duplicate offsets.
pub fn sample_gradient_stops(stops:&PagedList<GradientStop,{usize::MAX}>,position:f64)->Result<[f64;4],&'static str>{
 if position.is_nan(){return Err("Invalid gradient position")}if stops.len()>MAX_PAINT_STOPS{return Err("Invalid gradient stops")}
 let mut first:Option<&GradientStop>=None;let mut left:Option<&GradientStop>=None;let mut right:Option<&GradientStop>=None;
 for stop in stops.iter(){if !valid_stop(stop){return Err("Invalid gradient stops")}if first.is_none_or(|value|stop.offset<value.offset){first=Some(stop)}if stop.offset<=position{if left.is_none_or(|value|stop.offset>=value.offset){left=Some(stop)}}else if right.is_none_or(|value|stop.offset<value.offset){right=Some(stop)}}
 let Some(first)=first else{return Ok([0.;4])};let Some(left)=left else{return Ok(first.color)};Ok(right.map_or(left.color,|right|interpolate(left,right,position)))
}
/// 🖌️ Evaluates one bounded authored sample without constructing a retained preparation job.
pub fn sample_fill(fill:&FillStyle,point:[f64;2])->Result<[f64;4],&'static str>{let shape=geometry(fill)?;let position=offset(shape,point)?;match shape{Geometry::Solid(color)=>Ok(color),_=>sample_gradient_stops(source_stops(fill).ok_or("Invalid gradient stops")?,position)}}
/// 🎯️ Resolves constant authored paint without allocating sorted stop storage.
pub fn constant_fill_color(fill:&FillStyle)->Result<Option<[f64;4]>,&'static str>{
 let shape=geometry(fill)?;if let Geometry::Solid(color)=shape{return Ok(Some(color))}let stops=source_stops(fill).ok_or("Invalid gradient stops")?;let sample=sample_gradient_stops(stops,1.)?;
 Ok(match shape{Geometry::Linear{length:0.,..}|Geometry::Radial{radius:0.,..}=>Some(sample),_ if stops.len()<=1=>Some(sample),_=>None})
}
#[derive(semio_framework_value::RetireOwned)]
pub struct GradientRamp{stops:Vec<GradientStop>,scratch:Vec<GradientStop>}
impl GradientRamp{
 pub fn constant_color(&self)->Option<[f64;4]>{(self.stops.len()<=1).then(||self.stops.first().map_or([0.;4],|stop|stop.color))}
 pub fn sample(&self,position:f64)->Result<[f64;4],&'static str>{if position.is_nan(){return Err("Invalid gradient position")}let Some(first)=self.stops.first()else{return Ok([0.;4])};let right=self.stops.partition_point(|stop|stop.offset<=position);if right==0{return Ok(first.color)}let left=&self.stops[right-1];Ok(self.stops.get(right).map_or(left.color,|right|interpolate(left,right,position)))}
 pub fn into_retirement(self)->GradientRampRetirement{GradientRampRetirement::new(self)}
}
physical_work_retirement!(GradientRampRetirement,GradientRamp);
#[derive(Clone,Copy,PartialEq,Eq)]
enum Stage{Geometry,Stops,Scratch,Sort,Complete}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct PreparedFillProgress{pub completed:usize,pub total:usize,pub work:u64,pub done:bool}
#[derive(semio_framework_value::RetireOwned)]
pub struct PreparedFill{input:FillStyle,geometry:Option<Geometry>,ramp:GradientRamp,stage:Stage,at:usize,width:usize,left:usize,i:usize,j:usize,k:usize,mid:usize,end:usize,work:u64,last_turn_work:u64,last_turn_semantic_work:u64,step_semantic_error:u64,demand:RetirementDemand,failed:Option<&'static str>,cancelled:bool}
impl PreparedFill{
 pub fn new(input:FillStyle)->Self{Self{input,geometry:None,ramp:GradientRamp{stops:Vec::new(),scratch:Vec::new()},stage:Stage::Geometry,at:0,width:1,left:0,i:0,j:0,k:0,mid:0,end:0,work:0,last_turn_work:0,last_turn_semantic_work:0,step_semantic_error:0,demand:RetirementDemand{depth:1,..Default::default()},failed:None,cancelled:false}}
 fn setup(&mut self){let n=self.ramp.stops.len();self.i=self.left;self.mid=(self.left+self.width).min(n);self.j=self.mid;self.k=self.left;self.end=(self.left+2*self.width).min(n);}
 fn step(&mut self,ledger:&mut RetainedWorkLedger)->Result<bool,&'static str>{match self.stage{
  Stage::Geometry=>{self.step_semantic_error=1;let shape=geometry(&self.input)?;if source_stops(&self.input).is_some_and(|stops|stops.len()>MAX_PAINT_STOPS){return Err("Invalid gradient stops")}self.geometry=Some(shape);self.stage=Stage::Stops;Ok(true)}
  Stage::Stops=>{let Some(stops)=source_stops(&self.input)else{self.stage=Stage::Complete;return Ok(false)};let Some(stop)=stops.get(self.at)else{self.stage=Stage::Scratch;return Ok(false)};if !valid_stop(stop){self.step_semantic_error=1;return Err("Invalid gradient stops")}if !ledger.reserve_vec(&mut self.ramp.stops,stops.len())?||!ledger.copy_bytes(std::mem::size_of::<GradientStop>()){return Ok(false)}self.ramp.stops.push(stop.clone());self.at+=1;Ok(true)}
  Stage::Scratch=>{let n=self.ramp.stops.len();if n<2{self.stage=Stage::Complete;return Ok(false)}if !ledger.reserve_vec(&mut self.ramp.scratch,n)?{return Ok(false)}if self.ramp.scratch.len()<n{if !ledger.copy_bytes(std::mem::size_of::<GradientStop>()){return Ok(false)}self.ramp.scratch.push(GradientStop{offset:0.,color:[0.;4]});return Ok(false)}self.setup();self.stage=Stage::Sort;Ok(false)}
  Stage::Sort=>{if self.width>=self.ramp.stops.len(){self.stage=Stage::Complete;return Ok(false)}if !ledger.copy_bytes(std::mem::size_of::<GradientStop>()){return Ok(false)}let source=if self.i<self.mid&&(self.j>=self.end||self.ramp.stops[self.i].offset<=self.ramp.stops[self.j].offset){let source=self.i;self.i+=1;source}else{let source=self.j;self.j+=1;source};self.ramp.scratch[self.k]=self.ramp.stops[source].clone();self.k+=1;if self.k==self.end{self.left+=2*self.width;if self.left>=self.ramp.stops.len(){std::mem::swap(&mut self.ramp.stops,&mut self.ramp.scratch);self.width*=2;self.left=0;}self.setup();}Ok(true)}
  Stage::Complete=>Ok(false)
 }}
 pub fn progress(&self)->PreparedFillProgress{PreparedFillProgress{completed:self.at,total:source_stops(&self.input).map_or(0,|stops|stops.len()),work:self.work,done:self.stage==Stage::Complete}}
 pub fn next_work_demand(&self)->RetirementDemand{self.demand}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 pub fn failure_pending(&self)->bool{self.failed.is_some()}
 pub fn settlement_pending(&self)->bool{!self.cancelled&&self.failed.is_none()&&(self.stage==Stage::Complete||(self.stage==Stage::Sort&&self.width>=self.ramp.stops.len())||(self.stage==Stage::Stops&&source_stops(&self.input).is_none_or(|stops|stops.len()<2&&self.at==stops.len()))||(self.stage==Stage::Scratch&&self.ramp.stops.len()<2))}
 pub fn advance(&mut self,items:usize,ledger:&mut RetainedWorkLedger)->Result<PreparedFillProgress,&'static str>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if items==0||items as u128>9_007_199_254_740_991{return Err("Fill work grant must be a positive integer")}if self.cancelled{return Err("Fill preparation cancelled")}if let Some(error)=self.failed{return Err(error)}
  for _ in 0..items{if self.stage==Stage::Complete{break}if !ledger.admits_absolute(self.demand){ledger.require_absolute(self.demand);break}let Some(root)=ledger.enter()else{self.demand=ledger.next_demand();break};let before=ledger.receipt();self.step_semantic_error=0;let result=self.step(ledger);let semantic=match &result{Ok(true)=>1,Err(_)=>self.step_semantic_error,_=>0};let actual=ledger.receipt()!=before||!ledger.blocked();if actual{ledger.note_source(root);self.last_turn_work+=1;self.work+=semantic;self.last_turn_semantic_work+=semantic;}ledger.exit();self.demand=ledger.next_demand();if let Err(error)=result{self.failed=Some(error);return Err(error)}if ledger.blocked()||!actual{break}}
  Ok(self.progress())
 }
 pub fn cancel(&mut self){self.cancelled=true;}
 fn ready(&self)->Result<Geometry,&'static str>{if self.cancelled{return Err("Fill preparation cancelled")}if let Some(error)=self.failed{return Err(error)}if self.stage!=Stage::Complete{return Err("Fill preparation is incomplete")}self.geometry.ok_or("Fill preparation is incomplete")}
 pub fn constant_color(&self)->Option<[f64;4]>{match self.ready().ok()?{Geometry::Solid(color)=>Some(color),Geometry::Linear{length:0.,..}|Geometry::Radial{radius:0.,..}=>self.ramp.sample(1.).ok(),_=>self.ramp.constant_color()}}
 pub fn sample(&self,point:[f64;2])->Result<[f64;4],&'static str>{let shape=self.ready()?;let position=offset(shape,point)?;match shape{Geometry::Solid(color)=>Ok(color),_=>self.ramp.sample(position)}}
 pub fn into_retirement(self)->PreparedFillRetirement{PreparedFillRetirement::new(self)}
}
semio_framework_value::artifact_retire_leaf!(Stage);
physical_work_retirement!(PreparedFillRetirement,PreparedFill);
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

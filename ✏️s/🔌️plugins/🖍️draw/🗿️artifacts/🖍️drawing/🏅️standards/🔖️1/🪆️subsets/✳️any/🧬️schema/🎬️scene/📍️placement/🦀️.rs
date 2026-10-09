//! 📍️ Actual borrowed scene owners yield finite bounds and geometric moments.
use crate::schema::scene_preparation::{DrawingSceneSource,DocumentVectorPreparationJob,DocumentSceneLimits,DocumentAlgorithmLimits,DocumentSceneContent};
use crate::schema::scene_paint::scene::{ScenePaintJob,PaintedSceneLimits,PreparedPaint};
use semio_framework_2d::{booleans::{BooleanJob,BooleanInput,BooleanOperand,BooleanOperation,BooleanFillRule},flatten::{PathFlattenJob,PathFlattenInput},PathSegment};
type Point=[f64;2];type Bounds=[f64;4];
#[derive(Clone,Copy,Debug,PartialEq,serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase")]
pub enum PlacementBasis{Area,Length,Bounds,Empty}
#[derive(Clone,Copy,Debug,PartialEq,serde::Serialize,serde::Deserialize)]
pub struct DrawingScenePlacement{pub bounds:Option<Bounds>,pub centroid:Point,pub area:f64,pub basis:PlacementBasis}
#[derive(Clone,Copy,Debug,serde::Serialize)]
pub struct DrawingScenePlacementProgress{pub phase:&'static str,pub work:u64,pub done:bool}
#[derive(Clone,Copy,Default)]
struct Sum{value:f64,error:f64}
impl Sum{fn add(&mut self,value:f64){let adjusted=value-self.error;let total=self.value+adjusted;self.error=(total-self.value)-adjusted;self.value=total;}}
semio_framework_value::artifact_retire_leaf!(PlacementBasis,DrawingScenePlacement,DrawingScenePlacementProgress,Sum);
/// 🧱️ Completed vector, cache, union and length children remain owned until caller close.
#[derive(semio_framework_value::RetireOwned)]
pub struct DrawingScenePlacementJob{
 vector:DocumentVectorPreparationJob,paint:Option<ScenePaintJob>,union:Option<BooleanJob>,flatten:Option<PathFlattenJob>,retired_flats:Vec<PathFlattenJob>,length_segments:Vec<PathSegment>,
 phase:&'static str,work:u64,node:usize,part:usize,contour:usize,point:usize,segment:usize,flatten_contour:usize,flatten_point:usize,operands:Vec<BooleanOperand>,operand:BooleanOperand,bounds:Option<Bounds>,origin:Point,ring:Point,pen:Point,anchored:bool,
 area:Sum,mx:Sum,my:Sum,length:Sum,lx:Sum,ly:Sum,output:Option<DrawingScenePlacement>,failure:Option<String>,cancelled:bool,
 flatness:f64,epsilon:f64,max_work:u64,max_edges:usize,max_parameters:usize,max_atomic_edges:usize,max_segments:usize,
}
impl DrawingScenePlacementJob{
 pub fn new(source:&impl DrawingSceneSource,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits,flatness:f64)->Result<Self,String>{
  if !flatness.is_finite()||!(1e-6..=16.0).contains(&flatness){return Err("Invalid placement flatness".into());}let vector=DocumentVectorPreparationJob::new(source,limits,algorithms).map_err(|error|error.to_string())?;let boolean=algorithms.booleans;
  Ok(Self{vector,paint:None,union:None,flatten:None,retired_flats:Vec::new(),length_segments:Vec::new(),phase:"vector",work:0,node:0,part:0,contour:0,point:0,segment:0,flatten_contour:0,flatten_point:0,operands:Vec::new(),operand:BooleanOperand{contours:Vec::new(),fill_rule:BooleanFillRule::Nonzero},bounds:None,origin:[0.0;2],ring:[0.0;2],pen:[0.0;2],anchored:false,area:Sum::default(),mx:Sum::default(),my:Sum::default(),length:Sum::default(),lx:Sum::default(),ly:Sum::default(),output:None,failure:None,cancelled:false,flatness,epsilon:boolean.epsilon,max_work:algorithms.max_work,max_edges:boolean.max_edges,max_parameters:boolean.max_parameters,max_atomic_edges:boolean.max_atomic_edges,max_segments:boolean.max_segments})
 }
 fn edge(&mut self,to:Point){let a=[self.pen[0]-self.origin[0],self.pen[1]-self.origin[1]];let b=[to[0]-self.origin[0],to[1]-self.origin[1]];let cross=a[0]*b[1]-b[0]*a[1];self.area.add(cross/2.0);self.mx.add((a[0]+b[0])*cross/6.0);self.my.add((a[1]+b[1])*cross/6.0);self.pen=to;}
 fn finish(&mut self)->Result<(),String>{let area=self.area.value.abs();let(area,basis,centroid)=if area>self.epsilon*self.epsilon{(area,PlacementBasis::Area,[self.origin[0]+self.mx.value/self.area.value,self.origin[1]+self.my.value/self.area.value])}else if self.length.value>0.0{(0.0,PlacementBasis::Length,[self.lx.value/self.length.value,self.ly.value/self.length.value])}else if let Some(bounds)=self.bounds{(0.0,PlacementBasis::Bounds,[(bounds[0]+bounds[2])/2.0,(bounds[1]+bounds[3])/2.0])}else{(0.0,PlacementBasis::Empty,[0.0;2])};if !area.is_finite()||!centroid.into_iter().all(f64::is_finite){return Err("Placement moments exceed finite coordinate limits".into());}self.output=Some(DrawingScenePlacement{bounds:self.bounds,centroid,area,basis});self.phase="complete";Ok(())}
 fn step(&mut self,source:&impl DrawingSceneSource)->Result<(),String>{match self.phase{
  "vector"=>{if self.vector.advance(source,1).map_err(|error|error.to_string())?.done{let plan=self.vector.result().map_err(|error|error.to_string())?;self.paint=Some(ScenePaintJob::new(plan,self.flatness,PaintedSceneLimits{max_nodes:1024,max_segments:65536,max_points:262144,max_contours:65536,max_work:self.max_work}));self.phase="paint";}},
  "paint"=>{if self.paint.as_mut().unwrap().advance(1)?.done{self.phase="contours";}},
  "contours"=>{
   let scene=self.paint.as_ref().unwrap().result()?;let Some(geometry)=scene.geometry.get(self.node)else{if self.operands.is_empty(){self.phase="length";self.node=0;return Ok(());}self.union=Some(BooleanJob::admit(BooleanInput{operation:BooleanOperation::Union,operands:std::mem::take(&mut self.operands),epsilon:self.epsilon,max_edges:self.max_edges,max_parameters:self.max_parameters,max_atomic_edges:self.max_atomic_edges,max_segments:self.max_segments,max_work:self.max_work}));self.phase="union";return Ok(());};
   if self.part==0&&self.contour==0&&self.point==0&&scene.plan.nodes[self.node].visible{if let Some(bounds)=geometry.bounds.or(geometry.geometry_bounds){self.bounds=Some(if let Some(current)=self.bounds{[current[0].min(bounds[0]),current[1].min(bounds[1]),current[2].max(bounds[2]),current[3].max(bounds[3])]}else{bounds});}}
   let contour:Option<&[Point]>=match &geometry.paint{PreparedPaint::Path(regions)=>if self.part==0{regions.fill.get(self.contour)}else{regions.stroke.get(self.contour)}.map(Vec::as_slice),PreparedPaint::Image(corners)|PreparedPaint::TextFallback(corners)=>if self.part==0&&self.contour==0{Some(corners.as_slice())}else{None},PreparedPaint::Empty=>None};
   if let Some(contour)=contour{if self.point==0{self.operand.contours.push(Vec::new());self.point+=1;}else if self.point<=contour.len(){self.operand.contours.last_mut().unwrap().push(contour[self.point-1]);self.point+=1;}else{self.contour+=1;self.point=0;}return Ok(());}
   if self.part==0{self.part=1;self.contour=0;return Ok(());}if !self.operand.contours.is_empty(){self.operands.push(std::mem::replace(&mut self.operand,BooleanOperand{contours:Vec::new(),fill_rule:BooleanFillRule::Nonzero}));}self.node+=1;self.part=0;self.contour=0;self.point=0;
  },
  "union"=>{if self.union.as_mut().unwrap().advance_work(1).map_err(|error|error.to_string())?{self.phase="moments";self.segment=0;}},
  "moments"=>{let segment=self.union.as_ref().unwrap().result().map_err(|error|error.to_string())?.get(self.segment).cloned();self.segment+=1;match segment{None=>{if self.area.value.abs()>self.epsilon*self.epsilon{self.finish()?;}else{self.phase="length";self.node=0;}},Some(PathSegment::Move{to})=>{if !self.anchored{self.origin=to;self.anchored=true;}self.ring=to;self.pen=to;},Some(PathSegment::Line{to})=>self.edge(to),Some(PathSegment::Close)=>self.edge(self.ring),_=>return Err("Canonical placement union must contain line segments".into())}},
  "length"=>{
   if let Some(flatten)=&mut self.flatten{if !flatten.advance(1).map_err(|error|error.to_string())?.done{return Ok(());}let contours=flatten.result().map_err(|error|error.to_string())?;if let Some(contour)=contours.get(self.flatten_contour){let count=if contour.closed{contour.points.len()}else{contour.points.len().saturating_sub(1)};if self.flatten_point<count{let a=contour.points[self.flatten_point];let b=contour.points[(self.flatten_point+1)%contour.points.len()];let length=(b[0]-a[0]).hypot(b[1]-a[1]);self.length.add(length);self.lx.add((a[0]/2.0+b[0]/2.0)*length);self.ly.add((a[1]/2.0+b[1]/2.0)*length);self.flatten_point+=1;}else{self.flatten_contour+=1;self.flatten_point=0;}return Ok(());}self.retired_flats.push(self.flatten.take().unwrap());self.node+=1;self.segment=0;return Ok(());}
   let scene=self.paint.as_ref().unwrap().result()?;let Some(node)=scene.plan.nodes.get(self.node)else{return self.finish();};if !node.visible{self.node+=1;return Ok(());}let DocumentSceneContent::Path{segments,..}=&node.content else{self.node+=1;return Ok(());};if let Some(segment)=segments.get(self.segment){self.length_segments.push(crate::schema::to_kernel_segment(segment));self.segment+=1;return Ok(());}self.flatten=Some(PathFlattenJob::admit(PathFlattenInput{segments:std::mem::take(&mut self.length_segments),transform:node.transform,tolerance:self.flatness}));self.flatten_contour=0;self.flatten_point=0;
  },"complete"=>{},_=>unreachable!(),}Ok(())}
 pub fn advance(&mut self,source:&impl DrawingSceneSource,grant:usize)->Result<DrawingScenePlacementProgress,String>{if grant==0||grant as u128>9_007_199_254_740_991{return Err("Invalid placement work grant".into());}if self.cancelled{return Err("Scene placement cancelled".into());}if let Some(error)=&self.failure{return Err(error.clone());}for _ in 0..grant{if self.phase=="complete"{break;}let result=if self.work>=self.max_work{Err("Placement work limit exceeded".into())}else{self.step(source)};if let Err(error)=result{self.failure=Some(error.clone());return Err(error);}self.work+=1;}Ok(DrawingScenePlacementProgress{phase:self.phase,work:self.work,done:self.phase=="complete"})}
 pub fn result(&self)->Result<DrawingScenePlacement,String>{if self.cancelled{return Err("Scene placement cancelled".into());}if let Some(error)=&self.failure{return Err(error.clone());}self.output.ok_or_else(||"Scene placement incomplete".into())}
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn into_retirement(mut self)->(DrawingScenePlacementRetirement,Option<DrawingScenePlacement>){let output=self.result().ok();self.output=None;self.cancelled=true;(DrawingScenePlacementRetirement::new(self),output)}
}
semio_framework_2d::physical_work_retirement!(DrawingScenePlacementRetirement,DrawingScenePlacementJob,String,|error:&str|error.to_string());
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
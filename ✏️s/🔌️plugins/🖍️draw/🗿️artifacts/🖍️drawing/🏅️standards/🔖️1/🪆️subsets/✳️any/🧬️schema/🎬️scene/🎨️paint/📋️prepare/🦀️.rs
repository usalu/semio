//! 🎬️ Owned vector plans and indexed painted caches publish as one completed scene.
use super::{PaintedPathPrepareJob,PaintedPathRegions,PaintedRegionLimits,PaintedPreparationRetirement};
use crate::schema::{scene_preparation::{DocumentScenePlan,DocumentSceneContent}};

type Point=[f64;2];type Bounds=[f64;4];
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct PaintedSceneLimits{pub max_nodes:usize,pub max_segments:usize,pub max_points:usize,pub max_contours:usize,pub max_work:u64}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct PreparedSceneCounts{pub nodes:usize,pub segments:usize,pub points:usize,pub contours:usize}
#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub enum PreparedPaint{Empty,Path(PaintedPathRegions),Image([Point;4])}
#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct PreparedGeometry{pub geometry_bounds:Option<Bounds>,pub bounds:Option<Bounds>,pub paint:PreparedPaint}
#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct PreparedScene{pub plan:DocumentScenePlan,pub geometry:Vec<PreparedGeometry>,pub flatness:f64,pub counts:PreparedSceneCounts}
#[derive(Clone,Copy,Debug)]
pub struct ScenePaintProgress{pub phase:&'static str,pub work:u64,pub nodes:usize,pub segments:usize,pub points:usize,pub contours:usize,pub done:bool}
fn valid(v:f64)->bool{v.is_finite()&&v.abs()<=1e9}
fn union(a:Option<Bounds>,b:Bounds)->Bounds{a.map_or(b,|a|[a[0].min(b[0]),a[1].min(b[1]),a[2].max(b[2]),a[3].max(b[3])])}
fn rectangle(x:f64,y:f64,w:f64,h:f64,m:[f64;6])->Result<PreparedGeometry,String>{let mut corners=[[x,y],[x+w,y],[x+w,y+h],[x,y+h]];let mut bounds=None;for p in &mut corners{*p=[m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]];if !p.iter().copied().all(valid){return Err("Painted scene coordinate limit exceeded".into());}bounds=Some(union(bounds,[p[0],p[1],p[0],p[1]]));}Ok(PreparedGeometry{geometry_bounds:bounds,bounds,paint:PreparedPaint::Image(corners)})}
semio_framework_2d::physical_work_retirement!(PreparedSceneCloseJob,PreparedScene,&'static str,|_:&str|"Prepared scene physical close refused");
/// 🧱️ Constructor adoption preserves refused owned input for work-granted retirement.
#[derive(semio_framework_value::RetireOwned)]
pub struct ScenePaintJob{
 plan:Option<DocumentScenePlan>,limits:PaintedSceneLimits,flatness:f64,geometry:Vec<PreparedGeometry>,counts:PreparedSceneCounts,phase:&'static str,work:u64,validated:bool,index:usize,segment:usize,pen:Point,origin:Point,bounds:Option<Bounds>,
 path:Option<PaintedPathPrepareJob>,retired_paths:Vec<PaintedPreparationRetirement>,path_retirement:Option<PaintedPreparationRetirement>,pending:Option<PreparedGeometry>,output:Option<PreparedScene>,cancelled:bool,failure:Option<String>,
}
impl ScenePaintJob{
 pub fn new(plan:DocumentScenePlan,flatness:f64,limits:PaintedSceneLimits)->Self{Self{plan:Some(plan),limits,flatness,geometry:Vec::new(),counts:Default::default(),phase:"nodes",work:0,validated:false,index:0,segment:0,pen:[0.0;2],origin:[0.0;2],bounds:None,path:None,retired_paths:Vec::new(),path_retirement:None,pending:None,output:None,cancelled:false,failure:None}}
 fn validate(&mut self)->Result<(),String>{let l=&self.limits;if !self.flatness.is_finite()||!(1e-6..=16.0).contains(&self.flatness)||l.max_nodes==0||l.max_nodes>1024||l.max_segments==0||l.max_segments>65536||l.max_points==0||l.max_points>262144||l.max_contours==0||l.max_contours>65536||l.max_work==0||l.max_work>1000000000{return Err("Invalid painted scene contract".into());}if self.plan.as_ref().unwrap().nodes.len()>l.max_nodes{return Err("Painted scene node limit exceeded".into());}self.validated=true;Ok(())}
 fn admit(&mut self,g:PreparedGeometry)->Result<(),String>{let(points,contours)=match &g.paint{PreparedPaint::Empty=>(0,0),PreparedPaint::Path(r)=>(r.points,r.contours),_=>(4,1)};if self.counts.points+points>self.limits.max_points||self.counts.contours+contours>self.limits.max_contours{self.pending=Some(g);return Err("Painted scene cache count limit exceeded".into());}self.geometry.push(g);self.counts.points+=points;self.counts.contours+=contours;self.counts.nodes+=1;self.index+=1;self.phase="nodes";Ok(())}
 fn step(&mut self)->Result<(),String>{if !self.validated{return self.validate();}let node=self.plan.as_ref().unwrap().nodes.get(self.index);match self.phase{
  "nodes"=>{let Some(node)=node else{self.output=Some(PreparedScene{plan:self.plan.take().unwrap(),geometry:std::mem::take(&mut self.geometry),flatness:self.flatness,counts:self.counts});self.phase="complete";return Ok(());};if !node.transform.into_iter().all(valid){return Err("Painted scene transform limit exceeded".into());}if matches!(node.content,DocumentSceneContent::Boolean{..}|DocumentSceneContent::Trace{..}){return Err("Unresolved algorithm in painted scene".into());}if let DocumentSceneContent::Path{segments,..}|DocumentSceneContent::Glyphs{segments,..}=&node.content{if self.counts.segments+segments.len()>self.limits.max_segments{return Err("Painted scene segment limit exceeded".into());}self.counts.segments+=segments.len();}self.bounds=None;self.segment=0;self.pen=[0.0;2];self.origin=self.pen;if !node.visible||matches!(node.content,DocumentSceneContent::Group{..}){self.admit(PreparedGeometry{geometry_bounds:None,bounds:None,paint:PreparedPaint::Empty})?;}else{match &node.content{DocumentSceneContent::Path{stroke,..}|DocumentSceneContent::Glyphs{stroke,..}=>{if stroke.as_ref().and_then(|s|s.dash.as_ref()).is_some_and(|d|d.len()>1024){return Err("Painted scene stroke limit exceeded".into());}self.phase="bounds";},DocumentSceneContent::Image{width,height,..}=>{let g=rectangle(0.0,0.0,*width,*height,node.transform)?;self.admit(g)?;},DocumentSceneContent::Text{..}=>return Err("Unresolved semantic text in painted scene".into()),_=>unreachable!()}}},
  "bounds"=>{let node=node.unwrap();let (DocumentSceneContent::Path{segments,..}|DocumentSceneContent::Glyphs{segments,..})=&node.content else{unreachable!()};if let Some(s)=segments.get(self.segment){if !matches!(s,crate::PathSegment::Close)||self.bounds.is_some(){let b=crate::schema::geometry::segment_bounds(s,self.pen,self.origin,node.transform);let world=[b[0],b[1],b[0]+b[2],b[1]+b[3]];if !world.into_iter().all(valid){return Err("Painted scene bounds limit exceeded".into());}self.bounds=Some(union(self.bounds,world));}match s{crate::PathSegment::Move{to}=>{self.pen=*to;self.origin=*to},crate::PathSegment::Close=>self.pen=self.origin,crate::PathSegment::Line{to}|crate::PathSegment::Quad{to,..}|crate::PathSegment::Cubic{to,..}|crate::PathSegment::Arc{to,..}=>self.pen=*to}self.segment+=1;}else{self.path=Some(PaintedPathPrepareJob::from_prepared(node,self.flatness,PaintedRegionLimits{max_segments:65536,max_points:262144,max_contours:65536,max_work:self.limits.max_work})?);self.phase="path";}},
  "path"=>{let node=node.unwrap();let (DocumentSceneContent::Path{segments,..}|DocumentSceneContent::Glyphs{segments,..})=&node.content else{unreachable!()};if self.path.as_mut().unwrap().advance_work(1,|index|segments.get(index).cloned())?{let(c,output)=self.path.take().unwrap().into_retirement();if let Some(previous)=self.path_retirement.replace(c){self.retired_paths.push(previous);}let regions=output.unwrap();self.pending=Some(PreparedGeometry{geometry_bounds:self.bounds,bounds:regions.bounds,paint:PreparedPaint::Path(regions)});self.phase="pathCleanup";}},
  "pathCleanup"=>{let g=self.pending.take().unwrap();self.admit(g)?;},"complete"=>{},_=>unreachable!(),}Ok(())}
 pub fn advance(&mut self,grant:usize)->Result<ScenePaintProgress,String>{if grant==0||grant as u128>9007199254740991{return Err("Invalid painted scene work grant".into());}if self.cancelled{return Err("Painted scene cancelled".into());}if let Some(error)=&self.failure{return Err(error.clone());}for _ in 0..grant{if self.phase=="complete"{break;}let step=if self.validated&&self.work>=self.limits.max_work{Err("Painted scene work limit exceeded".into())}else{self.step()};if let Err(error)=step{self.failure=Some(error.clone());return Err(error);}self.work+=1;}Ok(ScenePaintProgress{phase:self.phase,work:self.work,nodes:self.counts.nodes,segments:self.counts.segments,points:self.counts.points,contours:self.counts.contours,done:self.phase=="complete"})}
 pub fn result(&self)->Result<&PreparedScene,String>{if self.cancelled{return Err("Painted scene cancelled".into());}if let Some(error)=&self.failure{return Err(error.clone());}if self.phase!="complete"{return Err("Painted scene incomplete".into());}Ok(self.output.as_ref().unwrap())}
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn into_retirement(mut self)->(ScenePaintRetirement,Option<PreparedScene>){let output=if self.result().is_ok(){self.output.take()}else{None};self.cancelled=true;if let Some(mut path)=self.path.take(){path.cancel();let retired=path.into_retirement().0;if let Some(previous)=self.path_retirement.replace(retired){self.retired_paths.push(previous);}}(ScenePaintRetirement::new(self),output)}
}
semio_framework_2d::physical_work_retirement!(ScenePaintRetirement,ScenePaintJob,&'static str,|_:&str|"Painted scene physical close refused");
semio_framework_value::artifact_retire_leaf!(PreparedSceneCounts,ScenePaintProgress);
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path="🎯️query/🦀️.rs"]
pub mod query;

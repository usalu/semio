//! 🎨️ Borrowed path admission, painted winding and real geometry retirement under work grants.
use crate::{PathSegment,FillRule,StrokeCap,StrokeJoin};
use semio_framework_2d::{flatten::{PathFlattenJob,PathFlattenInput,PathFlattenRetirement,FlatContour},stroke::{StrokeOutlineJob,StrokeOutlineInput,StrokeOutlineRetirement,StrokeGeometryStyle,StrokeGeometryCap,StrokeGeometryJoin,StrokeContour},retirement::{WorkRetirementCounter,WorkRetirementProgress}};
type Point=[f64;2];
#[derive(Clone,Debug)]
pub struct PaintedPathStroke{pub width:f64,pub cap:StrokeCap,pub join:StrokeJoin,pub dash:Vec<f64>}
#[derive(Clone,Debug)]
pub struct PaintedPathQuery{pub point:Point,pub transform:[f64;6],pub tolerance:f64,pub flatness:f64,pub fill:bool,pub fill_rule:FillRule,pub stroke:Option<PaintedPathStroke>}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct PaintedPathHit{pub contains:bool,pub bounds:Option<[f64;4]>}
#[derive(Clone,Copy,Debug)]
pub struct PaintedPathProgress{pub phase:&'static str,pub work:u64,pub done:bool}
fn valid(v:f64)->bool{v.is_finite()&&v.abs()<=1e9}
fn map(p:Point,m:[f64;6])->Point{[m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]]}
fn retire_flat(flat:&mut Vec<FlatContour>)->bool{if flat.pop().is_some(){false}else{*flat=Vec::new();true}}
fn retire_polygons(polygons:&mut Vec<Vec<Point>>)->bool{if polygons.pop().is_some(){false}else{*polygons=Vec::new();true}}
/// 🧱️ Stores only admitted fixed-shape segments; completed results contain no source references.
pub struct PaintedPathHitJob{
 query:PaintedPathQuery,style:Option<StrokeGeometryStyle>,segments:Vec<semio_framework_2d::PathSegment>,flat:Vec<FlatContour>,contours:Vec<StrokeContour>,polygons:Vec<Vec<Point>>,
 flatten:Option<PathFlattenJob>,flatten_retirement:Option<PathFlattenRetirement>,outline:Option<StrokeOutlineJob>,outline_retirement:Option<StrokeOutlineRetirement>,
 phase:&'static str,next:usize,at:usize,edge:usize,work:u64,winding:i64,boundary:bool,stroke_hit:bool,bounds:[f64;4],output:Option<PaintedPathHit>,cancelled:bool,failure:Option<String>,cleanup:u8,
}
impl PaintedPathHitJob{
 /// 🎬️ Uses the resolved leaf's actual geometry transform and renderer stroke semantics.
 pub fn from_prepared(node:&crate::schema::scene_preparation::DocumentSceneNode,point:Point,tolerance:f64,flatness:f64)->Result<Self,String>{
  let crate::schema::scene_preparation::DocumentSceneContent::Path{fill,fill_rule,stroke,..}=&node.content else{return Err("Painted query requires a resolved path leaf".into());};
  Self::new(PaintedPathQuery{point,transform:node.transform,tolerance,flatness,fill:fill.is_some(),fill_rule:fill_rule.clone(),stroke:stroke.as_ref().map(|s|PaintedPathStroke{width:s.width,cap:s.cap.clone(),join:s.join.clone(),dash:s.dash.clone().unwrap_or_default()})})
 }
 pub fn new(query:PaintedPathQuery)->Result<Self,String>{
  if !query.point.into_iter().chain(query.transform).all(valid)||!valid(query.tolerance)||query.tolerance<0.0||!query.flatness.is_finite()||!(1e-6..=16.0).contains(&query.flatness){return Err("Invalid painted query contract".into());}
  let style=query.stroke.as_ref().map(|s|{
   if !valid(s.width)||s.width<0.0||s.dash.len()>1024||!s.dash.iter().all(|v|valid(*v)&&*v>=0.0){return Err("Invalid painted query stroke".to_string());}
   Ok(StrokeGeometryStyle{width:s.width,cap:match s.cap{StrokeCap::Butt=>StrokeGeometryCap::Butt,StrokeCap::Round=>StrokeGeometryCap::Round,StrokeCap::Square=>StrokeGeometryCap::Square},join:match s.join{StrokeJoin::Miter=>StrokeGeometryJoin::Miter,StrokeJoin::Round=>StrokeGeometryJoin::Round,StrokeJoin::Bevel=>StrokeGeometryJoin::Bevel},miter_limit:4.0,dash:s.dash.clone(),dash_offset:0.0})
  }).transpose()?;
  Ok(Self{query,style,segments:Vec::new(),flat:Vec::new(),contours:Vec::new(),polygons:Vec::new(),flatten:None,flatten_retirement:None,outline:None,outline_retirement:None,phase:"admitting",next:0,at:0,edge:0,work:0,winding:0,boundary:false,stroke_hit:false,bounds:[f64::INFINITY,f64::INFINITY,f64::NEG_INFINITY,f64::NEG_INFINITY],output:None,cancelled:false,failure:None,cleanup:0})
 }
 fn line(&mut self,a:Point,b:Point)->Result<(),String>{
  if !a.into_iter().chain(b).all(valid){return Err("Painted query exceeds coordinate limit".into());}
  for p in [a,b]{self.bounds[0]=self.bounds[0].min(p[0]);self.bounds[1]=self.bounds[1].min(p[1]);self.bounds[2]=self.bounds[2].max(p[0]);self.bounds[3]=self.bounds[3].max(p[1]);}
  let rounding=a.into_iter().chain(b).chain(self.query.point).fold(1.0_f64,|s,v|s.max(v.abs()))*f64::EPSILON*8.0;
  self.boundary|=super::distance(self.query.point,a,b)<=self.query.tolerance+rounding;
  let[x,y]=self.query.point;if (a[1]<=y&&b[1]>y)||(b[1]<=y&&a[1]>y){let t=(y-a[1])/(b[1]-a[1]);if a[0]*(1.0-t)+b[0]*t>x{self.winding+=if b[1]>a[1]{1}else{-1};}}
  Ok(())
 }
 fn retire_one(&mut self)->bool{
  if self.cleanup==8{return true;}
  let complete=match self.cleanup{
   0=>{if let Some(child)=&mut self.flatten_retirement{if !child.terminal_is_empty(){child.advance(1).expect("positive query cleanup");return false;}self.flatten_retirement=None;}true},
   1=>{if let Some(child)=&mut self.outline_retirement{if !child.terminal_is_empty(){child.advance(1).expect("positive query cleanup");return false;}self.outline_retirement=None;}true},
   2=>retire_flat(&mut self.flat),3=>{if self.contours.pop().is_some(){false}else{self.contours=Vec::new();true}},4=>retire_polygons(&mut self.polygons),5=>{self.segments=Vec::new();true},
   6=>{self.style=None;self.query.stroke=None;true},7=>{self.failure=None;true},_=>unreachable!(),
  };if complete{self.cleanup+=1;}self.cleanup==8
 }
 fn step(&mut self,source:&mut impl FnMut(usize)->Option<PathSegment>)->Result<(),String>{
  match self.phase{
   "admitting"=>{if let Some(segment)=source(self.next){if self.next>=65536{return Err("Painted query exceeds segment limit".into());}self.segments.push(crate::schema::to_kernel_segment(&segment));self.next+=1;}else{self.flatten=Some(PathFlattenJob::new(PathFlattenInput{segments:std::mem::take(&mut self.segments),transform:self.query.transform,tolerance:self.query.flatness}).map_err(|e|e.to_string())?);self.phase="flattening";}},
   "flattening"=>{if self.flatten.as_mut().unwrap().advance(1).map_err(|e|e.to_string())?.done{let(child,output)=self.flatten.take().unwrap().into_retirement();self.flatten_retirement=Some(child);self.flat=output.unwrap();self.phase="flattenCleanup";}},
   "flattenCleanup"=>{let child=self.flatten_retirement.as_mut().unwrap();if !child.terminal_is_empty(){child.advance(1).map_err(|e|e.to_string())?;}else{self.flatten_retirement=None;self.phase="fill";}},
   "fill"=>{if let Some(contour)=self.flat.get(self.at){if !self.query.fill||contour.points.len()<2{self.at+=1;self.edge=0;}else if self.edge<contour.points.len(){let a=map(contour.points[self.edge],self.query.transform);let b=map(contour.points[(self.edge+1)%contour.points.len()],self.query.transform);self.edge+=1;self.line(a,b)?;}else{self.at+=1;self.edge=0;}}else{self.phase="contours";}},
   "contours"=>{if let Some(c)=self.flat.pop(){if self.style.is_some(){self.contours.push(StrokeContour{points:c.points,closed:c.closed});}}else{self.flat=Vec::new();if let Some(style)=self.style.take(){self.outline=Some(StrokeOutlineJob::new(StrokeOutlineInput{contours:std::mem::take(&mut self.contours),transform:self.query.transform,tolerance:self.query.flatness,style}).map_err(|e|e.to_string())?);self.phase="stroke";}else{self.publish();self.phase="cleanup";}}},
   "stroke"=>{if self.outline.as_mut().unwrap().advance(1).map_err(|e|e.to_string())?.done{let(child,output)=self.outline.take().unwrap().into_retirement();self.outline_retirement=Some(child);self.polygons=output.unwrap();self.phase="strokeCleanup";}},
   "strokeCleanup"=>{let child=self.outline_retirement.as_mut().unwrap();if !child.terminal_is_empty(){child.advance(1).map_err(|e|e.to_string())?;}else{self.outline_retirement=None;self.publish();self.winding=0;self.boundary=false;self.at=0;self.edge=0;self.phase="polygons";}},
   "polygons"=>{if let Some(polygon)=self.polygons.get(self.at){if self.edge<polygon.len(){let a=map(polygon[self.edge],self.query.transform);let b=map(polygon[(self.edge+1)%polygon.len()],self.query.transform);self.edge+=1;self.line(a,b)?;}else{self.stroke_hit|=self.boundary||self.winding!=0;self.winding=0;self.boundary=false;self.at+=1;self.edge=0;}}else{let fill=self.output.unwrap().contains;self.output=Some(PaintedPathHit{contains:fill||self.stroke_hit,bounds:self.bounds[0].is_finite().then_some(self.bounds)});self.phase="cleanup";}},
   "cleanup"=>{if self.retire_one(){self.phase="complete";}},"complete"=>{},_=>unreachable!(),
  }Ok(())
 }
 fn publish(&mut self){self.output=Some(PaintedPathHit{contains:self.query.fill&&(self.boundary||if self.query.fill_rule==FillRule::Evenodd{self.winding%2!=0}else{self.winding!=0}),bounds:self.bounds[0].is_finite().then_some(self.bounds)});}
 pub fn advance(&mut self,grant:usize,mut source:impl FnMut(usize)->Option<PathSegment>)->Result<PaintedPathProgress,String>{
  if grant==0||grant as u128>9_007_199_254_740_991{return Err("Invalid painted query work grant".into());}if self.cancelled{return Err("Painted query cancelled".into());}if let Some(error)=&self.failure{return Err(error.clone());}
  for _ in 0..grant{if self.phase=="complete"{break;}if let Err(error)=self.step(&mut source){self.failure=Some(error.clone());return Err(error);}self.work+=1;}
  Ok(PaintedPathProgress{phase:self.phase,work:self.work,done:self.phase=="complete"})
 }
 pub fn result(&self)->Result<PaintedPathHit,String>{if self.cancelled{return Err("Painted query cancelled".into());}if let Some(error)=&self.failure{return Err(error.clone());}if self.phase!="complete"{return Err("Painted query incomplete".into());}Ok(self.output.unwrap())}
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn into_retirement(mut self)->(PaintedPathRetirement,Option<PaintedPathHit>){
  let output=self.result().ok();self.cancelled=true;
  if let Some(mut child)=self.flatten.take(){child.cancel();self.flatten_retirement=Some(child.into_retirement().0);}if let Some(mut child)=self.outline.take(){child.cancel();self.outline_retirement=Some(child.into_retirement().0);}
  (PaintedPathRetirement{job:Some(self),counter:Default::default()},output)
 }
}
/// 🧹️ Actual child owners reach empty retirement before private query state is released.
pub struct PaintedPathRetirement{job:Option<PaintedPathHitJob>,counter:WorkRetirementCounter}
impl PaintedPathRetirement{
 pub fn terminal_is_empty(&self)->bool{self.job.is_none()}
 pub fn advance(&mut self,grant:usize)->Result<WorkRetirementProgress,&'static str>{let mut counter=std::mem::take(&mut self.counter);let result=counter.advance(grant,||{if self.job.as_mut().unwrap().retire_one(){self.job=None;true}else{false}});self.counter=counter;result}
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

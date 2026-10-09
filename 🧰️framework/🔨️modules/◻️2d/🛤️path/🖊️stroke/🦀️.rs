//! 🖊️ Stroke regions for https://www.w3.org/TR/SVG11/painting.html#StrokeProperties.
use crate::Vec2;
use std::collections::VecDeque;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum StrokeGeometryCap {Butt,Round,Square}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum StrokeGeometryJoin {Miter,Round,Bevel}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct StrokeGeometryStyle {pub width:f64,pub cap:StrokeGeometryCap,pub join:StrokeGeometryJoin,pub miter_limit:f64,pub dash:Vec<f64>,pub dash_offset:f64}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct StrokeContour {pub points:Vec<Vec2>,pub closed:bool}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct StrokeOutlineInput {pub contours:Vec<StrokeContour>,pub transform:[f64;6],pub tolerance:f64,pub style:StrokeGeometryStyle}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum StrokeOutlinePhase {Preparing,Dashing,Outlining,Complete}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct StrokeOutlineProgress {pub phase:StrokeOutlinePhase,pub completed:usize,pub total:usize,pub points:usize,pub work:u64,pub done:bool}
#[derive(Clone,Debug,PartialEq,Eq)]
#[derive(semio_framework_value::RetireOwned)]
pub enum StrokeOutlineError {Invalid(&'static str),Incomplete,Cancelled}
impl std::fmt::Display for StrokeOutlineError {
 fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {match self {Self::Invalid(message)=>f.write_str(message),Self::Incomplete=>f.write_str("Stroke preparation is incomplete"),Self::Cancelled=>f.write_str("Stroke preparation cancelled")}}
}
impl std::error::Error for StrokeOutlineError {}
#[derive(semio_framework_value::RetireOwned)]
struct Run {points:Vec<Vec2>,closed:bool,cap_start:bool,cap_end:bool,painted:bool,tangent:Vec2}
#[derive(semio_framework_value::RetireOwned)]
enum Primitive {Polygon(Vec<Vec2>),Round {center:Vec2,a:f64,b:f64}}
#[derive(Clone,Copy)]
struct ArcNode {a:f64,b:f64,depth:u8}
#[derive(semio_framework_value::RetireOwned)]
struct Round {center:Vec2,polygon:usize,stack:Vec<ArcNode>}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Stage {Prepare,Source,Offset,Walk,Finish,Outline,Done}
const MAX_POINTS:usize=65536;
const MAX_CONTOURS:usize=65536;
fn valid(v:f64)->bool {v.is_finite()&&v.abs()<=1e9}
fn unit(a:Vec2,b:Vec2)->Vec2 {let x=b[0]-a[0];let y=b[1]-a[1];let length=x.hypot(y);if length>0.0 {[x/length,y/length]} else {[1.0,0.0]}}
fn normal(t:Vec2)->Vec2 {[-t[1],t[0]]}
fn shifted(p:Vec2,t:Vec2,d:f64)->Vec2 {[p[0]+t[0]*d,p[1]+t[1]*d]}

/// 🧱️ Work-granted dash runs and positive polygon unions preserve overlapping stroke regions.
#[derive(semio_framework_value::RetireOwned)]
pub struct StrokeOutlineJob {
 input:StrokeOutlineInput,sources:Vec<StrokeContour>,prepared:Vec<Vec2>,runs:Vec<Run>,seams:Vec<[Vec2;3]>,polygons:Vec<Vec<Vec2>>,queue:VecDeque<Primitive>,round:Option<Round>,
 stage:Stage,contour:usize,point:usize,source:usize,edge:usize,position:f64,pattern:Vec<f64>,pattern_total:f64,dash:usize,remaining:f64,offset:f64,active:Option<usize>,first:Option<usize>,last:Option<usize>,
 run:usize,geom_edge:usize,cap:u8,seam:usize,count:usize,run_points:usize,work:u64,cancelled:bool,failed:Option<StrokeOutlineError>,radius:f64,round_bound:f64,
}
impl StrokeOutlineJob {
 pub fn new(input:StrokeOutlineInput)->Result<Self,StrokeOutlineError> {
  let s=&input.style;let count=input.contours.iter().try_fold(0usize,|n,c|n.checked_add(c.points.len())).ok_or(StrokeOutlineError::Invalid("Invalid stroke geometry contract"))?;
  if !input.transform.into_iter().all(valid)||!input.tolerance.is_finite()||!(1e-6..=16.0).contains(&input.tolerance)||input.contours.len()>4096||count>MAX_POINTS||!valid(s.width)||s.width<0.0||!s.miter_limit.is_finite()||!(1.0..=1000.0).contains(&s.miter_limit)||s.dash.len()>1024||!s.dash.iter().all(|v|valid(*v)&&*v>=0.0)||!valid(s.dash_offset) {return Err(StrokeOutlineError::Invalid("Invalid stroke geometry contract"));}
  let mut pattern=s.dash.clone();if pattern.len()%2==1 {pattern.extend_from_slice(&s.dash);}let pattern_total=pattern.iter().sum::<f64>();if pattern_total==0.0||pattern.iter().enumerate().all(|(at,v)|at%2==0||*v==0.0) {pattern.clear();}
  let radius=s.width/2.0;let m=input.transform;let round_bound=radius*(m[0].hypot(m[1])+m[2].hypot(m[3]));
  Ok(Self {input,sources:Vec::new(),prepared:Vec::new(),runs:Vec::new(),seams:Vec::new(),polygons:Vec::new(),queue:VecDeque::new(),round:None,stage:Stage::Prepare,contour:0,point:0,source:0,edge:0,position:0.0,pattern,pattern_total,dash:0,remaining:0.0,offset:0.0,active:None,first:None,last:None,run:0,geom_edge:0,cap:0,seam:0,count:0,run_points:0,work:0,cancelled:false,failed:None,radius,round_bound})
 }
 fn check(&self,p:Vec2)->Result<(),StrokeOutlineError> {
  let m=self.input.transform;if !p.into_iter().all(valid)||!valid(m[0]*p[0]+m[2]*p[1]+m[4])||!valid(m[1]*p[0]+m[3]*p[1]+m[5]) {return Err(StrokeOutlineError::Invalid("Invalid stroke geometry point"));}Ok(())
 }
 fn run_point(&mut self,at:usize,p:Vec2)->Result<(),StrokeOutlineError> {if self.run_points>=MAX_POINTS {return Err(StrokeOutlineError::Invalid("Stroke dashing exceeds point budget"));}self.runs[at].points.push(p);self.run_points+=1;Ok(())}
 fn create_run(&mut self,a:Vec2,b:Vec2,positive:bool)->Result<(),StrokeOutlineError> {
  let src=&self.sources[self.source];let n=src.points.len();let tangent=if n>=2 {let edge=self.edge.min(if src.closed {n-1} else {n-2});unit(src.points[edge],src.points[(edge+1)%n])} else {[1.0,0.0]};
  let at=self.runs.len();self.runs.push(Run {points:Vec::new(),closed:false,cap_start:true,cap_end:true,painted:positive,tangent});self.run_point(at,a)?;self.run_point(at,b)?;self.active=Some(at);
  if positive {if self.first.is_none() {self.first=Some(at);}self.last=Some(at);}Ok(())
 }
 fn location(&self)->Vec2 {
  let src=&self.sources[self.source];let count=if src.closed {src.points.len()} else {src.points.len()-1};if self.edge>=count {return if src.closed {src.points[0]} else {*src.points.last().unwrap()};}
  let a=src.points[self.edge];let b=src.points[(self.edge+1)%src.points.len()];let length=(b[0]-a[0]).hypot(b[1]-a[1]);if self.position==0.0 {a} else {[a[0]+(b[0]-a[0])*self.position/length,a[1]+(b[1]-a[1])*self.position/length]}
 }
 fn polygon(&mut self,mut points:Vec<Vec2>)->Result<(),StrokeOutlineError> {
  let mut area=0.0;let origin=points[0];for at in 0..points.len() {self.check(points[at])?;let a=points[at];let b=points[(at+1)%points.len()];area+=(a[0]-origin[0])*(b[1]-origin[1])-(a[1]-origin[1])*(b[0]-origin[0]);}
  if area==0.0 {return Ok(());}if self.count+points.len()>MAX_POINTS||self.polygons.len()>=MAX_CONTOURS {return Err(StrokeOutlineError::Invalid("Stroke outline exceeds geometry budget"));}
  self.count+=points.len();if area<0.0 {points.reverse();}self.polygons.push(points);Ok(())
 }
 fn circle_point(&self,center:Vec2,angle:f64)->Vec2 {let (s,c)=angle.sin_cos();[center[0]+self.radius*c,center[1]+self.radius*s]}
 fn round_point(&mut self,p:Vec2)->Result<(),StrokeOutlineError> {self.check(p)?;if self.count>=MAX_POINTS {return Err(StrokeOutlineError::Invalid("Stroke outline exceeds point budget"));}self.polygons[self.round.as_ref().unwrap().polygon].push(p);self.count+=1;Ok(())}
 fn join(&mut self,[before,p,after]:[Vec2;3]) {
  let a=unit(before,p);let b=unit(p,after);let cross=a[0]*b[1]-a[1]*b[0];let dot=(a[0]*b[0]+a[1]*b[1]).clamp(-1.0,1.0);if cross==0.0&&dot>0.0 {return;}
  let side=if cross<0.0 {1.0} else {-1.0};let n=normal(a);let z=normal(b);let from=shifted(p,n,side*self.radius);let to=shifted(p,z,side*self.radius);
  if self.input.style.join==StrokeGeometryJoin::Round {let start=if cross<0.0 {to} else {from};let theta=(start[1]-p[1]).atan2(start[0]-p[0]);let span=cross.abs().atan2(dot);self.queue.push_back(Primitive::Round {center:p,a:theta,b:theta+span});return;}
  let denominator=1.0+dot;let ratio=if denominator>0.0 {(2.0/denominator).sqrt()} else {f64::INFINITY};
  if self.input.style.join==StrokeGeometryJoin::Miter&&ratio<=self.input.style.miter_limit {self.queue.push_back(Primitive::Polygon(vec![p,from,[p[0]+side*self.radius*(n[0]+z[0])/denominator,p[1]+side*self.radius*(n[1]+z[1])/denominator],to]));}
  else {self.queue.push_back(Primitive::Polygon(vec![p,from,to]));}
 }
 fn endpoint(&mut self,p:Vec2,t:Vec2,start:bool) {
  if self.input.style.cap==StrokeGeometryCap::Butt {return;}let n=normal(t);let a=shifted(p,n,self.radius);let b=shifted(p,n,-self.radius);
  if self.input.style.cap==StrokeGeometryCap::Square {let extension=shifted([0.0,0.0],t,(if start {-1.0} else {1.0})*self.radius);self.queue.push_back(Primitive::Polygon(vec![a,b,[b[0]+extension[0],b[1]+extension[1]],[a[0]+extension[0],a[1]+extension[1]]]));}
  else {let theta=if start {n[1].atan2(n[0])} else {(-n[1]).atan2(-n[0])};self.queue.push_back(Primitive::Round {center:p,a:theta,b:theta+std::f64::consts::PI});}
 }
 fn outline(&mut self)->Result<(),StrokeOutlineError> {
  if let Some(round)=self.round.as_mut() {
   let Some(node)=round.stack.pop() else {self.round=None;return Ok(());};let center=round.center;let span=node.b-node.a;let error=2.0*self.round_bound*(span/4.0).sin().powi(2);
   if span<=std::f64::consts::PI&&error<=self.input.tolerance {self.round_point(self.circle_point(center,node.b))?;return Ok(());}
   if node.depth>=32 {return Err(StrokeOutlineError::Invalid("Stroke outline exceeds subdivision budget"));}let middle=(node.a+node.b)/2.0;
   self.round.as_mut().unwrap().stack.extend([ArcNode {a:middle,b:node.b,depth:node.depth+1},ArcNode {a:node.a,b:middle,depth:node.depth+1}]);return Ok(());
  }
  if let Some(primitive)=self.queue.pop_front() {
   match primitive {Primitive::Polygon(points)=>self.polygon(points)?,Primitive::Round {center,a,b}=>{
    if self.polygons.len()>=MAX_CONTOURS {return Err(StrokeOutlineError::Invalid("Stroke outline exceeds contour budget"));}
    self.round=Some(Round {center,polygon:self.polygons.len(),stack:vec![ArcNode {a,b,depth:0}]});self.polygons.push(Vec::new());self.round_point(center)?;self.round_point(self.circle_point(center,a))?;
   }}return Ok(());
  }
  let Some(run)=self.runs.get(self.run) else {if let Some(seam)=self.seams.get(self.seam).copied() {self.seam+=1;self.join(seam);} else {self.stage=Stage::Done;}return Ok(());};
  let n=run.points.len();let count=if run.closed {n} else {n-1};
  if n<2||n==2&&run.points[0]==run.points[1] {
   let p=run.points[0];if self.input.style.cap==StrokeGeometryCap::Round {self.queue.push_back(Primitive::Round {center:p,a:0.0,b:std::f64::consts::TAU});}
   else if self.input.style.cap==StrokeGeometryCap::Square {let t=run.tangent;let normal=normal(t);let a=shifted(p,t,-self.radius);let b=shifted(p,t,self.radius);self.queue.push_back(Primitive::Polygon(vec![shifted(a,normal,self.radius),shifted(b,normal,self.radius),shifted(b,normal,-self.radius),shifted(a,normal,-self.radius)]));}
   self.run+=1;return Ok(());
  }
  if self.geom_edge<count {
   let at=self.geom_edge;self.geom_edge+=1;let a=run.points[at];let b=run.points[(at+1)%n];let normal=normal(unit(a,b));let join=(at>0||run.closed).then(||[run.points[(at+n-1)%n],a,b]);
   self.queue.push_back(Primitive::Polygon(vec![shifted(a,normal,self.radius),shifted(b,normal,self.radius),shifted(b,normal,-self.radius),shifted(a,normal,-self.radius)]));if let Some(join)=join {self.join(join);}return Ok(());
  }
  if self.cap==0 {self.cap+=1;if !run.closed&&run.cap_start {self.endpoint(run.points[0],unit(run.points[0],run.points[1]),true);}return Ok(());}
  if self.cap==1 {self.cap+=1;if !run.closed&&run.cap_end {self.endpoint(run.points[n-1],unit(run.points[n-2],run.points[n-1]),false);}return Ok(());}
  self.run+=1;self.geom_edge=0;self.cap=0;Ok(())
 }
 fn step(&mut self)->Result<(),StrokeOutlineError> {
  match self.stage {
   Stage::Prepare=>{
    let Some(contour)=self.input.contours.get(self.contour) else {self.stage=if self.radius==0.0 {Stage::Done} else {Stage::Source};return Ok(());};
    if self.point<contour.points.len() {let p=contour.points[self.point];self.point+=1;self.check(p)?;if self.prepared.last().is_none_or(|last|*last!=p) {self.prepared.push(p);}}
    else {let closed=contour.closed;let raw=contour.points.len();if closed&&self.prepared.len()>1&&self.prepared.first()==self.prepared.last() {self.prepared.pop();}if self.prepared.len()>=2||self.prepared.len()==1&&(raw>=2||closed) {self.sources.push(StrokeContour {points:std::mem::take(&mut self.prepared),closed});} else {self.prepared.clear();}self.contour+=1;self.point=0;}
   }
   Stage::Source=>{
    let Some(src)=self.sources.get_mut(self.source) else {self.stage=Stage::Outline;return Ok(());};
    if self.pattern.is_empty() {self.runs.push(Run {points:std::mem::take(&mut src.points),closed:src.closed,cap_start:true,cap_end:true,painted:true,tangent:[1.0,0.0]});self.source+=1;return Ok(());}
    self.edge=0;self.position=0.0;self.dash=0;self.offset=(self.input.style.dash_offset%self.pattern_total+self.pattern_total)%self.pattern_total;self.active=None;self.first=None;self.last=None;self.stage=Stage::Offset;
   }
   Stage::Offset=>{let length=self.pattern[self.dash];if self.offset>0.0&&self.offset>=length {self.offset-=length;self.dash=(self.dash+1)%self.pattern.len();} else {self.remaining=length-self.offset;self.offset=0.0;self.stage=Stage::Walk;}}
   Stage::Walk=>{
    let src=&self.sources[self.source];let n=src.points.len();let count=if src.closed {n} else {n-1};
    if self.remaining==0.0 {if self.dash%2==0&&self.active.is_none() {let p=self.location();self.create_run(p,p,false)?;}self.dash=(self.dash+1)%self.pattern.len();self.remaining=self.pattern[self.dash];return Ok(());}
    if self.dash%2==1 {self.active=None;}
    if self.edge>=count {if n==1&&self.dash%2==0&&self.active.is_none() {let p=src.points[0];self.create_run(p,p,false)?;}self.stage=Stage::Finish;return Ok(());}
    let a=src.points[self.edge];let b=src.points[(self.edge+1)%n];let length=(b[0]-a[0]).hypot(b[1]-a[1]);let left=length-self.position;let take=left.min(self.remaining);let from=self.location();let end_edge=take==left;let end_dash=take==self.remaining;
    if take>0.0&&self.position+take==self.position {return Err(StrokeOutlineError::Invalid("Stroke dashing exceeds numeric limits"));}self.position+=take;let to=if end_edge {b} else {self.location()};
    if self.dash%2==0 {if let Some(active)=self.active {if !self.runs[active].painted {self.runs[active].points.pop();self.runs[active].painted=true;if self.first.is_none() {self.first=Some(active);}self.last=Some(active);}self.run_point(active,to)?;} else {self.create_run(from,to,true)?;}}
    self.remaining-=take;if end_dash {self.dash=(self.dash+1)%self.pattern.len();self.remaining=self.pattern[self.dash];}if end_edge {self.edge+=1;self.position=0.0;}
   }
   Stage::Finish=>{
    let src=&self.sources[self.source];if src.closed {if let (Some(first),Some(last))=(self.first,self.last) {if self.runs[first].points[0]==src.points[0]&&self.runs[last].points.last()==Some(&src.points[0]) {
     if first==last {self.runs[first].closed=true;self.runs[first].points.pop();} else {self.runs[first].cap_start=false;self.runs[last].cap_end=false;let end=self.runs[last].points.len();self.seams.push([self.runs[last].points[end-2],src.points[0],self.runs[first].points[1]]);}
    }}}
    self.active=None;self.source+=1;self.stage=Stage::Source;
   }
   Stage::Outline=>self.outline()?,Stage::Done=>{}
  }Ok(())
 }
 pub fn advance(&mut self,budget:usize)->Result<StrokeOutlineProgress,StrokeOutlineError> {
  if budget==0||budget as u128>9_007_199_254_740_991 {return Err(StrokeOutlineError::Invalid("Stroke work grant must be a positive integer"));}if self.cancelled {return Err(StrokeOutlineError::Cancelled);}if let Some(error)=&self.failed {return Err(error.clone());}
  for _ in 0..budget {if self.stage==Stage::Done {break;}if let Err(error)=self.step() {self.failed=Some(error.clone());return Err(error);}self.work+=1;}
  let phase=match self.stage {Stage::Prepare=>StrokeOutlinePhase::Preparing,Stage::Outline=>StrokeOutlinePhase::Outlining,Stage::Done=>StrokeOutlinePhase::Complete,_=>StrokeOutlinePhase::Dashing};
  let (completed,total)=match phase {StrokeOutlinePhase::Preparing=>(self.contour,self.input.contours.len()),StrokeOutlinePhase::Dashing=>(self.source,self.sources.len()),_=>(self.run,self.runs.len())};
  Ok(StrokeOutlineProgress {phase,completed,total,points:self.count,work:self.work,done:self.stage==Stage::Done})
 }
 pub fn cancel(&mut self) {self.cancelled=true;self.active=None;}
 pub fn result(&self)->Result<&[Vec<Vec2>],StrokeOutlineError> {if self.cancelled {return Err(StrokeOutlineError::Cancelled);}if let Some(error)=&self.failed {return Err(error.clone());}if self.stage!=Stage::Done {return Err(StrokeOutlineError::Incomplete);}Ok(&self.polygons)}
 pub fn into_result(self)->Result<Vec<Vec<Vec2>>,StrokeOutlineError> {self.result()?;Ok(self.polygons)}
}
semio_framework_value::artifact_retire_leaf!(StrokeGeometryCap,StrokeGeometryJoin,StrokeOutlinePhase,ArcNode,Stage);
crate::physical_work_retirement!(StrokeOutlineRetirement,StrokeOutlineJob,StrokeOutlineError,|_:&str|StrokeOutlineError::Invalid("Stroke retirement refused its physical grant"));
impl StrokeOutlineJob{
 /// 🧹️ Moves complete output unchanged and retains interrupted owners for work-granted cleanup.
 pub fn into_retirement(mut self)->(StrokeOutlineRetirement,Option<Vec<Vec<Vec2>>>){let output=if self.stage==Stage::Done&&!self.cancelled&&self.failed.is_none(){Some(std::mem::take(&mut self.polygons))}else{None};self.cancelled=true;self.active=None;(StrokeOutlineRetirement::new(self),output)}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

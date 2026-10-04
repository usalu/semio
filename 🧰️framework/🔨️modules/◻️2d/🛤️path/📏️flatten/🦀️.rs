//! 📏️ Device-tolerance path preparation for https://www.w3.org/TR/SVG11/implnote.html#ArcImplementationNotes.
use crate::{PathSegment,Vec2};
#[derive(Clone,Debug)]
pub struct PathFlattenInput {pub segments:Vec<PathSegment>,pub transform:[f64;6],pub tolerance:f64}
#[derive(Clone,Debug,PartialEq)]
pub struct FlatContour {pub points:Vec<Vec2>,pub closed:bool}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PathFlattenPhase {Preparing,Subdividing,Complete}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct PathFlattenProgress {pub phase:PathFlattenPhase,pub completed:usize,pub total:usize,pub points:usize,pub work:u64,pub done:bool}
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum PathFlattenError {Invalid(&'static str),Incomplete,Cancelled}
impl std::fmt::Display for PathFlattenError {
 fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {match self {Self::Invalid(message)=>f.write_str(message),Self::Incomplete=>f.write_str("Path preparation is incomplete"),Self::Cancelled=>f.write_str("Path preparation cancelled")}}
}
impl std::error::Error for PathFlattenError {}
#[derive(Clone,Copy)]
struct Ellipse {center:Vec2,u:Vec2,v:Vec2,bound:f64}
#[derive(Clone,Copy)]
enum Curve {Bezier {points:[Vec2;4],count:usize,depth:u8},Arc {ellipse:Ellipse,a:f64,b:f64,to:Vec2,depth:u8}}
const MAX_COORDINATE:f64=1e9;
const MAX_POINTS:usize=65536;
const MAX_CONTOURS:usize=4096;
const MAX_DEPTH:u8=32;
fn valid(value:f64)->bool {value.is_finite()&&value.abs()<=MAX_COORDINATE}
fn point(value:Vec2)->Result<Vec2,PathFlattenError> {if value.into_iter().all(valid) {Ok(value)} else {Err(PathFlattenError::Invalid("Invalid path preparation point"))}}
fn midpoint(a:Vec2,b:Vec2)->Vec2 {[(a[0]+b[0])/2.0,(a[1]+b[1])/2.0]}
fn transformed(p:Vec2,m:[f64;6])->Result<Vec2,PathFlattenError> {point([m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]])}
fn distance_to_chord(p:Vec2,a:Vec2,b:Vec2)->f64 {
 let dx=b[0]-a[0];let dy=b[1]-a[1];let length=dx*dx+dy*dy;let t=if length==0.0 {0.0} else {(((p[0]-a[0])*dx+(p[1]-a[1])*dy)/length).clamp(0.0,1.0)};
 (p[0]-a[0]-t*dx).hypot(p[1]-a[1]-t*dy)
}
fn split(points:[Vec2;4],count:usize)->([Vec2;4],[Vec2;4]) {
 let mut row=points;let mut left=[[0.0;2];4];let mut right=left;left[0]=row[0];right[count-1]=row[count-1];
 for level in 1..count {for at in 0..count-level {row[at]=midpoint(row[at],row[at+1]);}left[level]=row[0];right[count-1-level]=row[count-1-level];}
 (left,right)
}
fn ellipse_point(ellipse:Ellipse,angle:f64)->Result<Vec2,PathFlattenError> {
 let (s,c)=angle.sin_cos();point([ellipse.center[0]+ellipse.u[0]*c+ellipse.v[0]*s,ellipse.center[1]+ellipse.u[1]*c+ellipse.v[1]*s])
}
fn arc(from:Vec2,segment:&PathSegment,m:[f64;6])->Result<Option<Curve>,PathFlattenError> {
 let PathSegment::Arc {rx,ry,rotation,large_arc,sweep,to}=segment else {return Err(PathFlattenError::Invalid("Invalid path preparation arc"));};
 let to=point(*to)?;if ![*rx,*ry,*rotation].into_iter().all(valid) {return Err(PathFlattenError::Invalid("Invalid path preparation arc"));}
 if from==to {return Ok(None);}
 let (mut rx,mut ry)=(rx.abs(),ry.abs());if rx==0.0||ry==0.0 {return Ok(Some(Curve::Bezier {points:[from,to,[0.0;2],[0.0;2]],count:2,depth:0}));}
 let phi=(rotation%360.0).to_radians();let (s,c)=phi.sin_cos();let dx=(from[0]-to[0])/2.0;let dy=(from[1]-to[1])/2.0;let x=c*dx+s*dy;let y=-s*dx+c*dy;
 let mut norm=(x/rx).hypot(y/ry);
 if !norm.is_finite() {let corrected_rx=x.hypot(y*(rx/ry));let corrected_ry=(x*(ry/rx)).hypot(y);rx=corrected_rx;ry=corrected_ry;norm=(x/rx).hypot(y/ry);}
 else if norm>1.0 {rx*=norm;ry*=norm;norm=(x/rx).hypot(y/ry);}
 if !rx.is_finite()||!ry.is_finite()||!norm.is_finite()||norm==0.0 {return Err(PathFlattenError::Invalid("Arc preparation exceeds numeric limits"));}
 let sign=if large_arc==sweep {-1.0} else {1.0};let root=sign*(1.0-norm*norm).max(0.0).sqrt();let cx=rx*(y/ry/norm)*root;let cy=-ry*(x/rx/norm)*root;
 let center=[c*cx-s*cy+(from[0]+to[0])/2.0,s*cx+c*cy+(from[1]+to[1])/2.0];let u=[rx*c,rx*s];let v=[-ry*s,ry*c];
 let ux=(x-cx)/rx;let uy=(y-cy)/ry;let vx=(-x-cx)/rx;let vy=(-y-cy)/ry;let a=uy.atan2(ux);let mut delta=(ux*vy-uy*vx).atan2(ux*vx+uy*vy);
 if !sweep&&delta>0.0 {delta-=std::f64::consts::TAU;} else if *sweep&&delta<0.0 {delta+=std::f64::consts::TAU;}
 let bound=(m[0]*u[0]+m[2]*u[1]).hypot(m[1]*u[0]+m[3]*u[1])+(m[0]*v[0]+m[2]*v[1]).hypot(m[1]*v[0]+m[3]*v[1]);
 if !bound.is_finite()||!center.into_iter().all(f64::is_finite) {return Err(PathFlattenError::Invalid("Arc preparation exceeds numeric limits"));}
 Ok(Some(Curve::Arc {ellipse:Ellipse {center,u,v,bound},a,b:a+delta,to,depth:0}))
}

/// 🕰️ Owned contours with adaptive device-space flatness and bounded source/subdivision steps.
pub struct PathFlattenJob {
 input:PathFlattenInput,contours:Vec<FlatContour>,current:Option<usize>,stack:Vec<Curve>,pen:Vec2,start:Vec2,index:usize,count:usize,work:u64,done:bool,cancelled:bool,failed:Option<PathFlattenError>,
}
impl PathFlattenJob {
 pub fn new(input:PathFlattenInput)->Result<Self,PathFlattenError> {
  if !input.transform.into_iter().all(valid)||!input.tolerance.is_finite()||!(1e-6..=16.0).contains(&input.tolerance)||input.segments.len()>MAX_POINTS {return Err(PathFlattenError::Invalid("Invalid path preparation contract"));}
  Ok(Self {input,contours:Vec::new(),current:None,stack:Vec::new(),pen:[0.0;2],start:[0.0;2],index:0,count:0,work:0,done:false,cancelled:false,failed:None})
 }
 fn append(&mut self,p:Vec2)->Result<(),PathFlattenError> {
  point(p)?;transformed(p,self.input.transform)?;if self.count>=MAX_POINTS {return Err(PathFlattenError::Invalid("Path preparation exceeds point budget"));}
  self.contours[self.current.unwrap()].points.push(p);self.count+=1;Ok(())
 }
 fn begin(&mut self,p:Vec2)->Result<(),PathFlattenError> {
  if self.contours.len()>=MAX_CONTOURS {return Err(PathFlattenError::Invalid("Path preparation exceeds contour budget"));}
  self.current=Some(self.contours.len());self.contours.push(FlatContour {points:Vec::new(),closed:false});self.append(p)?;self.pen=p;self.start=p;Ok(())
 }
 fn step(&mut self)->Result<(),PathFlattenError> {
  if let Some(curve)=self.stack.pop() {
   match curve {
    Curve::Bezier {points,count,depth}=>{
     let mut p=[[0.0;2];4];for at in 0..count {p[at]=transformed(points[at],self.input.transform)?;}
     let mut error=0.0f64;for at in 1..count-1 {error=error.max(distance_to_chord(p[at],p[0],p[count-1]));}
     if error<=self.input.tolerance {self.append(points[count-1])?;return Ok(());}
     if depth>=MAX_DEPTH {return Err(PathFlattenError::Invalid("Path preparation exceeds subdivision budget"));}
     let (left,right)=split(points,count);self.stack.extend([Curve::Bezier {points:right,count,depth:depth+1},Curve::Bezier {points:left,count,depth:depth+1}]);
    }
    Curve::Arc {ellipse,a,b,to,depth}=>{
     let span=(b-a).abs();let error=2.0*ellipse.bound*(span/4.0).sin().powi(2);
     if span<=std::f64::consts::PI&&error<=self.input.tolerance {self.append(to)?;return Ok(());}
     if depth>=MAX_DEPTH {return Err(PathFlattenError::Invalid("Path preparation exceeds subdivision budget"));}
     let middle=(a+b)/2.0;let mid=ellipse_point(ellipse,middle)?;transformed(mid,self.input.transform)?;
     self.stack.extend([Curve::Arc {ellipse,a:middle,b,to,depth:depth+1},Curve::Arc {ellipse,a,b:middle,to:mid,depth:depth+1}]);
    }
   }
   return Ok(());
  }
  let Some(segment)=self.input.segments.get(self.index).cloned() else {self.done=true;return Ok(());};self.index+=1;
  if let PathSegment::Move {to}=segment {self.begin(point(to)?)?;return Ok(());}
  if matches!(segment,PathSegment::Close) {if let Some(current)=self.current {self.contours[current].closed=true;self.pen=self.start;self.current=None;}return Ok(());}
  if self.current.is_none() {self.begin(self.pen)?;}
  self.pen=match segment {
   PathSegment::Line {to}=>{self.append(point(to)?)?;to}
   PathSegment::Quad {ctrl,to}=>{self.stack.push(Curve::Bezier {points:[self.pen,point(ctrl)?,point(to)?,[0.0;2]],count:3,depth:0});to}
   PathSegment::Cubic {ctrl1,ctrl2,to}=>{self.stack.push(Curve::Bezier {points:[self.pen,point(ctrl1)?,point(ctrl2)?,point(to)?],count:4,depth:0});to}
   PathSegment::Arc {to,..}=>{if let Some(curve)=arc(self.pen,&segment,self.input.transform)? {self.stack.push(curve);}to}
   _=>return Err(PathFlattenError::Invalid("Invalid path preparation segment")),
  };
  Ok(())
 }
 pub fn advance(&mut self,budget:usize)->Result<PathFlattenProgress,PathFlattenError> {
  if budget==0||budget as u128>9_007_199_254_740_991 {return Err(PathFlattenError::Invalid("Path preparation work grant must be a positive integer"));}
  if self.cancelled {return Err(PathFlattenError::Cancelled);}if let Some(error)=&self.failed {return Err(error.clone());}
  for _ in 0..budget {if self.done {break;}if let Err(error)=self.step() {self.failed=Some(error.clone());return Err(error);}self.work+=1;}
  Ok(PathFlattenProgress {phase:if self.done {PathFlattenPhase::Complete} else if self.stack.is_empty() {PathFlattenPhase::Preparing} else {PathFlattenPhase::Subdividing},completed:self.index,total:self.input.segments.len(),points:self.count,work:self.work,done:self.done})
 }
 pub fn cancel(&mut self) {self.cancelled=true;self.input.segments=Vec::new();self.contours=Vec::new();self.stack=Vec::new();self.current=None;}
 pub fn result(&self)->Result<&[FlatContour],PathFlattenError> {
  if self.cancelled {return Err(PathFlattenError::Cancelled);}if let Some(error)=&self.failed {return Err(error.clone());}if !self.done {return Err(PathFlattenError::Incomplete);}Ok(&self.contours)
 }
 pub fn into_result(self)->Result<Vec<FlatContour>,PathFlattenError> {self.result()?;Ok(self.contours)}
}
pub type PathFlattenRetirementProgress=crate::retirement::WorkRetirementProgress;
/// 🧹️ Retains incomplete contours until each owned point buffer has retired.
pub struct PathFlattenRetirement{job:Option<PathFlattenJob>,slot:u8,counter:crate::retirement::WorkRetirementCounter}
impl PathFlattenRetirement{
 pub fn terminal_is_empty(&self)->bool{self.job.is_none()}
 fn step(&mut self){let job=self.job.as_mut().unwrap();let complete=match self.slot{
  0=>{job.input.segments=Vec::new();true},1=>{job.stack=Vec::new();true},
  2=>if job.contours.pop().is_some(){false}else{job.contours=Vec::new();true},_=>unreachable!(),
 };if complete{self.slot+=1;}if self.slot==3{self.job=None;}}
 pub fn advance(&mut self,grant:usize)->Result<PathFlattenRetirementProgress,PathFlattenError>{let mut counter=self.counter;let result=counter.advance(grant,||{self.step();self.terminal_is_empty()});self.counter=counter;result.map_err(PathFlattenError::Invalid)}
}
impl PathFlattenJob{
 /// 🧹️ Moves complete output to its caller and keeps unpublished contours private during cleanup.
 pub fn into_retirement(mut self)->(PathFlattenRetirement,Option<Vec<FlatContour>>){let output=if self.done&&!self.cancelled&&self.failed.is_none(){Some(std::mem::take(&mut self.contours))}else{None};self.cancelled=true;self.current=None;(PathFlattenRetirement{job:Some(self),slot:0,counter:Default::default()},output)}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

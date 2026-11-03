//! 📏️ Device-tolerance path preparation for https://www.w3.org/TR/SVG11/implnote.html#ArcImplementationNotes.
use semio_framework_2d_engine::{PathSegment,Vec2};
use semio_framework_value::RetirementDemand;
use semio_framework_value_persistent::work::RetainedWorkLedger;
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct PathFlattenInput {pub segments:Vec<PathSegment>,pub transform:[f64;6],pub tolerance:f64}
#[derive(Clone,Debug,PartialEq)]
#[derive(semio_framework_value::RetireOwned)]
pub struct FlatContour {pub points:Vec<Vec2>,pub closed:bool}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PathFlattenPhase {Preparing,Subdividing,Complete}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct PathFlattenProgress {pub phase:PathFlattenPhase,pub completed:usize,pub total:usize,pub points:usize,pub work:u64,pub done:bool}
#[derive(Clone,Debug,PartialEq,Eq)]
#[derive(semio_framework_value::RetireOwned)]
pub enum PathFlattenError {Invalid(&'static str),Incomplete,Cancelled}
impl PathFlattenError{
 /// 🗨️ Borrows the exact original cause without formatting or copying its payload.
 pub fn message(&self)->&str{match self{Self::Invalid(message)=>message,Self::Incomplete=>"Path preparation is incomplete",Self::Cancelled=>"Path preparation cancelled"}}
}
impl std::fmt::Display for PathFlattenError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(self.message())}}
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
#[derive(semio_framework_value::RetireOwned)]
pub struct PathFlattenJob {
 input:PathFlattenInput,contours:Vec<FlatContour>,pending_points:Vec<Vec2>,pending:RetirementDemand,last_turn_work:u64,last_turn_semantic_work:u64,step_prepared:bool,current:Option<usize>,stack:Vec<Curve>,pen:Vec2,start:Vec2,index:usize,count:usize,work:u64,done:bool,cancelled:bool,failed:Option<PathFlattenError>,
}
impl PathFlattenJob {
 fn validate(input:&PathFlattenInput)->Result<(),PathFlattenError> {
  if !input.transform.into_iter().all(valid)||!input.tolerance.is_finite()||!(1e-6..=16.0).contains(&input.tolerance)||input.segments.len()>MAX_POINTS {return Err(PathFlattenError::Invalid("Invalid path preparation contract"));}
  Ok(())
 }
 /// 📥️ Refused source contracts remain owned until their physical close.
 pub fn new(input:PathFlattenInput)->Self {let failed=Self::validate(&input).err();Self {input,contours:Vec::new(),pending_points:Vec::new(),pending:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0,step_prepared:false,current:None,stack:Vec::new(),pen:[0.0;2],start:[0.0;2],index:0,count:0,work:0,done:false,cancelled:false,failed}}
 fn slot<T>(values:&mut Vec<T>,limit:usize,work:&mut RetainedWorkLedger)->Result<bool,PathFlattenError>{
  if values.len()>=limit{return Err(PathFlattenError::Invalid("Path preparation exceeds output budget"));}
  if values.len()==values.capacity(){let target=values.capacity().max(1).saturating_mul(2).min(limit);if !work.reserve_vec(values,target).map_err(PathFlattenError::Invalid)?{return Ok(false);}}
  work.vec_slot(values).map_err(PathFlattenError::Invalid)
 }
 fn append(&mut self,p:Vec2,work:&mut RetainedWorkLedger)->Result<bool,PathFlattenError>{
  point(p)?;transformed(p,self.input.transform)?;if self.count>=MAX_POINTS{return Err(PathFlattenError::Invalid("Path preparation exceeds point budget"));}
  let points=&mut self.contours[self.current.ok_or(PathFlattenError::Invalid("path flatten has no current contour"))?].points;if !Self::slot(points,MAX_POINTS,work)?{return Ok(false);}work.vec_push(points,p).map_err(|_|PathFlattenError::Invalid("Path point copy authority lost"))?;self.count+=1;Ok(true)
 }
 fn begin(&mut self,p:Vec2,work:&mut RetainedWorkLedger)->Result<bool,PathFlattenError>{
  point(p)?;transformed(p,self.input.transform)?;if self.count>=MAX_POINTS{return Err(PathFlattenError::Invalid("Path preparation exceeds point budget"));}
  if !Self::slot(&mut self.contours,MAX_CONTOURS,work)?||!Self::slot(&mut self.pending_points,MAX_POINTS,work)?{return Ok(false);}
  work.vec_push(&mut self.pending_points,p).map_err(|_|PathFlattenError::Invalid("Path first point copy authority lost"))?;let index=self.contours.len();work.vec_push(&mut self.contours,FlatContour{points:std::mem::take(&mut self.pending_points),closed:false}).map_err(|_|PathFlattenError::Invalid("Path contour copy authority lost"))?;self.current=Some(index);self.count+=1;self.pen=p;self.start=p;Ok(true)
 }
 fn stack_room(&mut self,count:usize,work:&mut RetainedWorkLedger)->Result<bool,PathFlattenError>{
  let target=self.stack.len().checked_add(count).and_then(|value|value.checked_sub(usize::from(!self.stack.is_empty()))).ok_or(PathFlattenError::Invalid("Path subdivision extent overflow"))?;
  if target>MAX_DEPTH as usize+1{return Err(PathFlattenError::Invalid("Path preparation exceeds subdivision budget"));}
  if self.stack.capacity()<target&&!work.reserve_vec(&mut self.stack,target).map_err(PathFlattenError::Invalid)?{return Ok(false);}Ok(work.copy_bytes(count*std::mem::size_of::<Curve>()))
 }
 fn step(&mut self,work:&mut RetainedWorkLedger)->Result<bool,PathFlattenError>{
  if let Some(curve)=self.stack.last().copied(){
   match curve{
    Curve::Bezier{points,count,depth}=>{
     let mut p=[[0.0;2];4];for at in 0..count{p[at]=transformed(points[at],self.input.transform)?;}let mut error=0.0f64;for at in 1..count-1{error=error.max(distance_to_chord(p[at],p[0],p[count-1]));}
     if error<=self.input.tolerance{let completed=self.append(points[count-1],work)?;if completed{self.stack.pop();}return Ok(completed);}
     if depth>=MAX_DEPTH{return Err(PathFlattenError::Invalid("Path preparation exceeds subdivision budget"));}if !self.stack_room(2,work)?{return Ok(false);}let(left,right)=split(points,count);self.stack.pop();self.stack.extend([Curve::Bezier{points:right,count,depth:depth+1},Curve::Bezier{points:left,count,depth:depth+1}]);
    }
    Curve::Arc{ellipse,a,b,to,depth}=>{
     let span=(b-a).abs();let error=2.0*ellipse.bound*(span/4.0).sin().powi(2);if span<=std::f64::consts::PI&&error<=self.input.tolerance{let completed=self.append(to,work)?;if completed{self.stack.pop();}return Ok(completed);}
     if depth>=MAX_DEPTH{return Err(PathFlattenError::Invalid("Path preparation exceeds subdivision budget"));}let middle=(a+b)/2.0;let mid=ellipse_point(ellipse,middle)?;transformed(mid,self.input.transform)?;if !self.stack_room(2,work)?{return Ok(false);}self.stack.pop();self.stack.extend([Curve::Arc{ellipse,a:middle,b,to,depth:depth+1},Curve::Arc{ellipse,a,b:middle,to:mid,depth:depth+1}]);
    }
   }return Ok(true);
  }
  let Some(segment)=self.input.segments.get(self.index).cloned()else{self.done=true;self.step_prepared=true;return Ok(false);};
  if let PathSegment::Move{to}=segment{let completed=self.begin(point(to)?,work)?;if completed{self.index+=1;}return Ok(completed);}
  if matches!(segment,PathSegment::Close){if !work.can_step(){return Ok(false);}if let Some(current)=self.current{self.contours[current].closed=true;self.pen=self.start;self.current=None;}self.index+=1;return Ok(true);}
  if self.current.is_none()&&!self.begin(self.pen,work)?{return Ok(false);}
  let pen=match segment{
   PathSegment::Line{to}=>{let to=point(to)?;if !self.append(to,work)?{return Ok(false);}to}
   PathSegment::Quad{ctrl,to}=>{let curve=Curve::Bezier{points:[self.pen,point(ctrl)?,point(to)?,[0.0;2]],count:3,depth:0};if !self.stack_room(1,work)?{return Ok(false);}self.stack.push(curve);to}
   PathSegment::Cubic{ctrl1,ctrl2,to}=>{let curve=Curve::Bezier{points:[self.pen,point(ctrl1)?,point(ctrl2)?,point(to)?],count:4,depth:0};if !self.stack_room(1,work)?{return Ok(false);}self.stack.push(curve);to}
   PathSegment::Arc{to,..}=>{if let Some(curve)=arc(self.pen,&segment,self.input.transform)?{if !self.stack_room(1,work)?{return Ok(false);}self.stack.push(curve);}to}
   _=>return Err(PathFlattenError::Invalid("Invalid path preparation segment")),
  };self.pen=pen;self.index+=1;Ok(true)
 }
 pub fn next_work_demand(&self)->RetirementDemand{self.pending}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 pub fn failure_pending(&self)->bool{self.failed.is_some()}
 pub fn settlement_pending(&self)->bool{self.done||self.stack.is_empty()&&self.index>=self.input.segments.len()}
 pub fn advance(&mut self,budget:usize,work:&mut RetainedWorkLedger)->Result<PathFlattenProgress,PathFlattenError>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if budget==0||budget as u128>9_007_199_254_740_991{return Err(PathFlattenError::Invalid("Path preparation work grant must be a positive integer"));}if self.cancelled{return Err(PathFlattenError::Cancelled);}if let Some(error)=&self.failed{return Err(error.clone());}
  let d=self.pending;if !work.admits_absolute(d){work.require_absolute(d);return Ok(self.progress());}
  if let Some(root)=work.enter(){let result=(||{for _ in 0..budget{if self.done{break;}if !work.can_step(){self.pending=work.next_demand();break;}let before=work.receipt();self.step_prepared=false;let result=self.step(work);let completed=result.as_ref().copied().unwrap_or(true);if work.receipt()!=before||completed||self.step_prepared{work.note_source(root);self.last_turn_work+=1;}if completed{self.work+=1;self.last_turn_semantic_work+=1;}self.pending=work.next_demand();if let Err(error)=result{self.failed=Some(error.clone());return Err(error);}if work.blocked(){break;}}Ok(self.progress())})();work.exit();result}else{self.pending=work.next_demand();Ok(self.progress())}
 }
 fn progress(&self)->PathFlattenProgress{PathFlattenProgress{phase:if self.done{PathFlattenPhase::Complete}else if self.stack.is_empty(){PathFlattenPhase::Preparing}else{PathFlattenPhase::Subdividing},completed:self.index,total:self.input.segments.len(),points:self.count,work:self.work,done:self.done}}
 pub fn cancel(&mut self) {self.cancelled=true;self.current=None;}
 pub fn result(&self)->Result<&[FlatContour],PathFlattenError> {
  if self.cancelled {return Err(PathFlattenError::Cancelled);}if let Some(error)=&self.failed {return Err(error.clone());}if !self.done {return Err(PathFlattenError::Incomplete);}Ok(&self.contours)
 }
}
pub type PathFlattenRetirementProgress=semio_framework_2d_retire::WorkRetirementProgress;
semio_framework_value::artifact_retire_leaf!(PathFlattenPhase,PathFlattenProgress,Ellipse,Curve);
semio_framework_2d_retire::physical_work_retirement!(PathFlattenRetirement,PathFlattenJob);
impl PathFlattenJob{
 /// 🧹️ Moves complete output to its caller and keeps unpublished contours private during cleanup.
 pub fn into_retirement(mut self)->(PathFlattenRetirement,Option<Vec<FlatContour>>){let output=if self.done&&!self.cancelled&&self.failed.is_none(){Some(std::mem::take(&mut self.contours))}else{None};self.cancelled=true;self.current=None;(PathFlattenRetirement::new(self),output)}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

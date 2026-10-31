//! 🖊️ Stroke regions for https://www.w3.org/TR/SVG11/painting.html#StrokeProperties.
use semio_framework_2d_engine::Vec2;
use semio_framework_value::RetirementDemand;
use semio_framework_value_persistent::work::RetainedWorkLedger;
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
impl StrokeOutlineError{
 /// 🗨️ Borrows the exact original cause without formatting or copying its payload.
 pub fn message(&self)->&str{match self{Self::Invalid(message)=>message,Self::Incomplete=>"Stroke preparation is incomplete",Self::Cancelled=>"Stroke preparation cancelled"}}
}
impl std::fmt::Display for StrokeOutlineError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(self.message())}}
impl std::error::Error for StrokeOutlineError {}
#[derive(semio_framework_value::RetireOwned)]
struct Run {points:Vec<Vec2>,closed:bool,cap_start:bool,cap_end:bool,painted:bool,tangent:Vec2}
#[derive(Clone,Copy,semio_framework_value::RetireOwned)]
enum Primitive{Polygon{points:[Vec2;4],length:usize},Round{center:Vec2,a:f64,b:f64}}
#[derive(Clone,Copy)]
struct ArcNode {a:f64,b:f64,depth:u8}
#[derive(semio_framework_value::RetireOwned)]
struct PrimitiveQueue{values:[Option<Primitive>;2],head:usize,length:usize}
impl PrimitiveQueue{
 fn new()->Self{Self{values:[None;2],head:0,length:0}}
 fn len(&self)->usize{self.length}
 fn is_empty(&self)->bool{self.length==0}
 fn push_back(&mut self,value:Primitive){assert!(self.length<2);self.values[(self.head+self.length)%2]=Some(value);self.length+=1;}
 fn front(&self)->Option<Primitive>{self.values[self.head]}
 fn pop_front(&mut self)->Option<Primitive>{if self.length==0{return None;}let value=self.values[self.head].take();self.head=(self.head+1)%2;self.length-=1;value}
}
#[derive(semio_framework_value::RetireOwned)]
struct ArcStack{values:[Option<ArcNode>;33],length:usize}
impl ArcStack{
 fn new(node:ArcNode)->Self{let mut values=[None;33];values[0]=Some(node);Self{values,length:1}}
 fn last(&self)->Option<ArcNode>{self.length.checked_sub(1).and_then(|at|self.values[at])}
 fn pop(&mut self){if self.length>0{self.length-=1;self.values[self.length]=None;}}
 fn extend(&mut self,nodes:[ArcNode;2]){for node in nodes{assert!(self.length<33);self.values[self.length]=Some(node);self.length+=1;}}
}
#[derive(semio_framework_value::RetireOwned)]
struct Round{center:Vec2,polygon:usize,stack:ArcStack}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Stage {Pattern,Prepare,Source,Offset,Walk,Finish,Outline,Done}
const MAX_POINTS:usize=65536;
const MAX_CONTOURS:usize=65536;
fn valid(v:f64)->bool {v.is_finite()&&v.abs()<=1e9}
fn unit(a:Vec2,b:Vec2)->Vec2 {let x=b[0]-a[0];let y=b[1]-a[1];let length=x.hypot(y);if length>0.0 {[x/length,y/length]} else {[1.0,0.0]}}
fn normal(t:Vec2)->Vec2 {[-t[1],t[0]]}
fn shifted(p:Vec2,t:Vec2,d:f64)->Vec2 {[p[0]+t[0]*d,p[1]+t[1]*d]}

/// 🧱️ Work-granted dash runs and positive polygon unions preserve overlapping stroke regions.
#[derive(semio_framework_value::RetireOwned)]
pub struct StrokeOutlineJob {
 input:StrokeOutlineInput,sources:Vec<StrokeContour>,prepared:Vec<Vec2>,runs:Vec<Run>,seams:Vec<[Vec2;3]>,polygons:Vec<Vec<Vec2>>,queue:PrimitiveQueue,round:Option<Round>,new_run_points:Vec<Vec2>,new_polygon:Vec<Vec2>,pending:RetirementDemand,last_turn_work:u64,last_turn_semantic_work:u64,step_prepared:bool,
 stage:Stage,contour:usize,point:usize,source:usize,edge:usize,position:f64,pattern:Vec<f64>,pattern_total:f64,pattern_has_gap:bool,dash:usize,remaining:f64,offset:f64,active:Option<usize>,first:Option<usize>,last:Option<usize>,
 run:usize,geom_edge:usize,cap:u8,seam:usize,count:usize,run_points:usize,work:u64,cancelled:bool,failed:Option<StrokeOutlineError>,radius:f64,round_bound:f64,
}
impl StrokeOutlineJob {
 fn validate(input:&StrokeOutlineInput)->Result<(),StrokeOutlineError>{
  let s=&input.style;let count=input.contours.iter().try_fold(0usize,|n,c|n.checked_add(c.points.len())).ok_or(StrokeOutlineError::Invalid("Invalid stroke geometry contract"))?;
  if !input.transform.into_iter().all(valid)||!input.tolerance.is_finite()||!(1e-6..=16.0).contains(&input.tolerance)||input.contours.len()>4096||count>MAX_POINTS||!valid(s.width)||s.width<0.0||!s.miter_limit.is_finite()||!(1.0..=1000.0).contains(&s.miter_limit)||s.dash.len()>1024||!s.dash.iter().all(|v|valid(*v)&&*v>=0.0)||!valid(s.dash_offset) {return Err(StrokeOutlineError::Invalid("Invalid stroke geometry contract"));}
  Ok(())
 }
 /// 📥️ Invalid owned input stays resident until its caller funds retirement.
 pub fn new(input:StrokeOutlineInput)->Self{let failed=Self::validate(&input).err();let s=&input.style;
  let pattern=Vec::new();let pattern_total=0.0;
  let radius=s.width/2.0;let m=input.transform;let round_bound=radius*(m[0].hypot(m[1])+m[2].hypot(m[3]));
  Self {input,sources:Vec::new(),prepared:Vec::new(),runs:Vec::new(),seams:Vec::new(),polygons:Vec::new(),queue:PrimitiveQueue::new(),round:None,new_run_points:Vec::new(),new_polygon:Vec::new(),pending:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0,step_prepared:false,stage:Stage::Pattern,contour:0,point:0,source:0,edge:0,position:0.0,pattern,pattern_total,pattern_has_gap:false,dash:0,remaining:0.0,offset:0.0,active:None,first:None,last:None,run:0,geom_edge:0,cap:0,seam:0,count:0,run_points:0,work:0,cancelled:false,failed,radius,round_bound}
 }
 fn check(&self,p:Vec2)->Result<(),StrokeOutlineError> {
  let m=self.input.transform;if !p.into_iter().all(valid)||!valid(m[0]*p[0]+m[2]*p[1]+m[4])||!valid(m[1]*p[0]+m[3]*p[1]+m[5]) {return Err(StrokeOutlineError::Invalid("Invalid stroke geometry point"));}Ok(())
 }
 fn reserve<T>(values:&mut Vec<T>,required:usize,limit:usize,work:&mut RetainedWorkLedger)->Result<bool,StrokeOutlineError>{
  if required>limit{return Err(StrokeOutlineError::Invalid("Stroke preparation exceeds retained output budget"));}
  if values.capacity()<required{let target=values.capacity().max(1).saturating_mul(2).max(required).min(limit);if !work.reserve_vec(values,target).map_err(StrokeOutlineError::Invalid)?{return Ok(false);}}Ok(true)
 }
 fn append(values:&mut Vec<Vec2>,points:&[Vec2],work:&mut RetainedWorkLedger)->Result<bool,StrokeOutlineError>{
  let target=values.len()+points.len();if !Self::reserve(values,target,MAX_POINTS,work)?||!work.copy_bytes(std::mem::size_of_val(points)){return Ok(false);}values.extend_from_slice(points);Ok(true)
 }
 fn run_point(&mut self,at:usize,p:Vec2,work:&mut RetainedWorkLedger)->Result<bool,StrokeOutlineError>{
  if self.run_points>=MAX_POINTS{return Err(StrokeOutlineError::Invalid("Stroke dashing exceeds point budget"));}if !Self::append(&mut self.runs[at].points,&[p],work)?{return Ok(false);}self.run_points+=1;Ok(true)
 }
 fn create_run(&mut self,a:Vec2,b:Vec2,positive:bool,work:&mut RetainedWorkLedger)->Result<bool,StrokeOutlineError>{
  if self.run_points.checked_add(2).is_none_or(|n|n>MAX_POINTS){return Err(StrokeOutlineError::Invalid("Stroke dashing exceeds point budget"));}
  let src=&self.sources[self.source];let n=src.points.len();let tangent=if n>=2{let edge=self.edge.min(if src.closed{n-1}else{n-2});unit(src.points[edge],src.points[(edge+1)%n])}else{[1.0,0.0]};
  let target=self.runs.len()+1;if !Self::reserve(&mut self.runs,target,MAX_POINTS,work)?||!Self::reserve(&mut self.new_run_points,2,MAX_POINTS,work)?||!work.copy_bytes(2*std::mem::size_of::<Vec2>()+std::mem::size_of::<Run>()){return Ok(false);}
  self.new_run_points.extend([a,b]);let at=self.runs.len();self.runs.push(Run{points:std::mem::take(&mut self.new_run_points),closed:false,cap_start:true,cap_end:true,painted:positive,tangent});self.run_points+=2;self.active=Some(at);if positive{if self.first.is_none(){self.first=Some(at);}self.last=Some(at);}Ok(true)
 }
 fn location(&self)->Vec2 {
  let src=&self.sources[self.source];let count=if src.closed {src.points.len()} else {src.points.len()-1};if self.edge>=count {return if src.closed {src.points[0]} else {*src.points.last().unwrap()};}
  let a=src.points[self.edge];let b=src.points[(self.edge+1)%src.points.len()];let length=(b[0]-a[0]).hypot(b[1]-a[1]);if self.position==0.0 {a} else {[a[0]+(b[0]-a[0])*self.position/length,a[1]+(b[1]-a[1])*self.position/length]}
 }
 fn polygon(&mut self,points:[Vec2;4],length:usize,work:&mut RetainedWorkLedger)->Result<bool,StrokeOutlineError>{
  let points=&points[..length];let mut area=0.0;let origin=points[0];for at in 0..length{self.check(points[at])?;let a=points[at];let b=points[(at+1)%length];area+=(a[0]-origin[0])*(b[1]-origin[1])-(a[1]-origin[1])*(b[0]-origin[0]);}
  if area==0.0{return Ok(true);}if self.count+length>MAX_POINTS||self.polygons.len()>=MAX_CONTOURS{return Err(StrokeOutlineError::Invalid("Stroke outline exceeds geometry budget"));}
  let target=self.polygons.len()+1;if !Self::reserve(&mut self.polygons,target,MAX_CONTOURS,work)?||!Self::reserve(&mut self.new_polygon,length,MAX_POINTS,work)?||!work.copy_bytes(std::mem::size_of_val(points)+std::mem::size_of::<Vec<Vec2>>()){return Ok(false);}
  self.new_polygon.extend_from_slice(points);if area<0.0{self.new_polygon.reverse();}self.polygons.push(std::mem::take(&mut self.new_polygon));self.count+=length;Ok(true)
 }
 fn circle_point(&self,center:Vec2,angle:f64)->Vec2 {let (s,c)=angle.sin_cos();[center[0]+self.radius*c,center[1]+self.radius*s]}
 fn round_point(&mut self,p:Vec2,work:&mut RetainedWorkLedger)->Result<bool,StrokeOutlineError>{self.check(p)?;if self.count>=MAX_POINTS{return Err(StrokeOutlineError::Invalid("Stroke outline exceeds point budget"));}let index=self.round.as_ref().unwrap().polygon;if !Self::append(&mut self.polygons[index],&[p],work)?{return Ok(false);}self.count+=1;Ok(true)}
 fn join(&self,[before,p,after]:[Vec2;3])->Option<Primitive>{
  let a=unit(before,p);let b=unit(p,after);let cross=a[0]*b[1]-a[1]*b[0];let dot=(a[0]*b[0]+a[1]*b[1]).clamp(-1.0,1.0);if cross==0.0&&dot>0.0{return None;}let side=if cross<0.0{1.0}else{-1.0};let n=normal(a);let z=normal(b);let from=shifted(p,n,side*self.radius);let to=shifted(p,z,side*self.radius);
  if self.input.style.join==StrokeGeometryJoin::Round{let start=if cross<0.0{to}else{from};let theta=(start[1]-p[1]).atan2(start[0]-p[0]);return Some(Primitive::Round{center:p,a:theta,b:theta+cross.abs().atan2(dot)});}
  let denominator=1.0+dot;let ratio=if denominator>0.0{(2.0/denominator).sqrt()}else{f64::INFINITY};Some(if self.input.style.join==StrokeGeometryJoin::Miter&&ratio<=self.input.style.miter_limit{Primitive::Polygon{points:[p,from,[p[0]+side*self.radius*(n[0]+z[0])/denominator,p[1]+side*self.radius*(n[1]+z[1])/denominator],to],length:4}}else{Primitive::Polygon{points:[p,from,to,[0.0;2]],length:3}})
 }
 fn endpoint(&self,p:Vec2,t:Vec2,start:bool)->Option<Primitive>{
  if self.input.style.cap==StrokeGeometryCap::Butt{return None;}let n=normal(t);let a=shifted(p,n,self.radius);let b=shifted(p,n,-self.radius);Some(if self.input.style.cap==StrokeGeometryCap::Square{let extension=shifted([0.0,0.0],t,(if start{-1.0}else{1.0})*self.radius);Primitive::Polygon{points:[a,b,[b[0]+extension[0],b[1]+extension[1]],[a[0]+extension[0],a[1]+extension[1]]],length:4}}else{let theta=if start{n[1].atan2(n[0])}else{(-n[1]).atan2(-n[0])};Primitive::Round{center:p,a:theta,b:theta+std::f64::consts::PI}})
 }
 fn enqueue(&mut self,first:Option<Primitive>,second:Option<Primitive>,work:&mut RetainedWorkLedger)->bool{
  if !work.can_step(){return false;}if let Some(p)=first{self.queue.push_back(p);}if let Some(p)=second{self.queue.push_back(p);}true
 }
 fn outline(&mut self,work:&mut RetainedWorkLedger)->Result<bool,StrokeOutlineError>{
  if let Some(round)=&self.round{
   let Some(node)=round.stack.last()else{self.round=None;return Ok(true);};let center=round.center;let span=node.b-node.a;let error=2.0*self.round_bound*(span/4.0).sin().powi(2);
   if span<=std::f64::consts::PI&&error<=self.input.tolerance{let completed=self.round_point(self.circle_point(center,node.b),work)?;if completed{self.round.as_mut().unwrap().stack.pop();}return Ok(completed);}
   if node.depth>=32{return Err(StrokeOutlineError::Invalid("Stroke outline exceeds subdivision budget"));}if !work.can_step(){return Ok(false);}let middle=(node.a+node.b)/2.0;let stack=&mut self.round.as_mut().unwrap().stack;stack.pop();stack.extend([ArcNode{a:middle,b:node.b,depth:node.depth+1},ArcNode{a:node.a,b:middle,depth:node.depth+1}]);return Ok(true);
  }
  if let Some(primitive)=self.queue.front(){
   match primitive{
    Primitive::Polygon{points,length}=>{if !self.polygon(points,length,work)?{return Ok(false);}}
    Primitive::Round{center,a,b}=>{
     let start=self.circle_point(center,a);self.check(center)?;self.check(start)?;if self.polygons.len()>=MAX_CONTOURS||self.count+2>MAX_POINTS{return Err(StrokeOutlineError::Invalid("Stroke outline exceeds geometry budget"));}
     let target=self.polygons.len()+1;if !Self::reserve(&mut self.polygons,target,MAX_CONTOURS,work)?||!Self::reserve(&mut self.new_polygon,2,MAX_POINTS,work)?||!work.copy_bytes(2*std::mem::size_of::<Vec2>()+std::mem::size_of::<Vec<Vec2>>()){return Ok(false);}
     let polygon=self.polygons.len();self.new_polygon.extend([center,start]);self.polygons.push(std::mem::take(&mut self.new_polygon));self.round=Some(Round{center,polygon,stack:ArcStack::new(ArcNode{a,b,depth:0})});self.count+=2;
    }
   }self.queue.pop_front();return Ok(true);
  }
  let Some(run)=self.runs.get(self.run)else{if let Some(seam)=self.seams.get(self.seam).copied(){let primitive=self.join(seam);let completed=self.enqueue(primitive,None,work);if completed{self.seam+=1;}return Ok(completed);}self.stage=Stage::Done;self.step_prepared=true;return Ok(false);};
  let n=run.points.len();let count=if run.closed{n}else{n-1};
  if n<2||n==2&&run.points[0]==run.points[1]{
   let p=run.points[0];let primitive=if self.input.style.cap==StrokeGeometryCap::Round{Some(Primitive::Round{center:p,a:0.0,b:std::f64::consts::TAU})}else if self.input.style.cap==StrokeGeometryCap::Square{let tangent=run.tangent;let normal=normal(tangent);let a=shifted(p,tangent,-self.radius);let b=shifted(p,tangent,self.radius);Some(Primitive::Polygon{points:[shifted(a,normal,self.radius),shifted(b,normal,self.radius),shifted(b,normal,-self.radius),shifted(a,normal,-self.radius)],length:4})}else{None};let completed=self.enqueue(primitive,None,work);if completed{self.run+=1;}return Ok(completed);
  }
  if self.geom_edge<count{
   let at=self.geom_edge;let a=run.points[at];let b=run.points[(at+1)%n];let normal=normal(unit(a,b));let join=(at>0||run.closed).then(||self.join([run.points[(at+n-1)%n],a,b])).flatten();let rectangle=Primitive::Polygon{points:[shifted(a,normal,self.radius),shifted(b,normal,self.radius),shifted(b,normal,-self.radius),shifted(a,normal,-self.radius)],length:4};let completed=self.enqueue(Some(rectangle),join,work);if completed{self.geom_edge+=1;}return Ok(completed);
  }
  if self.cap==0{let endpoint=if !run.closed&&run.cap_start{self.endpoint(run.points[0],unit(run.points[0],run.points[1]),true)}else{None};let completed=self.enqueue(endpoint,None,work);if completed{self.cap+=1;}return Ok(completed);}
  if self.cap==1{let endpoint=if !run.closed&&run.cap_end{self.endpoint(run.points[n-1],unit(run.points[n-2],run.points[n-1]),false)}else{None};let completed=self.enqueue(endpoint,None,work);if completed{self.cap+=1;}return Ok(completed);}
  self.run+=1;self.geom_edge=0;self.cap=0;Ok(true)
 }
 fn step(&mut self,work:&mut RetainedWorkLedger)->Result<bool,StrokeOutlineError>{
  match self.stage{
   Stage::Pattern=>{
    let source=&self.input.style.dash;let target=source.len()*(1+usize::from(source.len()%2==1));if self.pattern.len()<target{if !Self::reserve(&mut self.pattern,target,2048,work)?||!work.copy_bytes(std::mem::size_of::<f64>()){return Ok(false);}let index=self.pattern.len();let value=source[index%source.len()];self.pattern.push(value);self.pattern_total+=value;self.pattern_has_gap|=index%2==1&&value>0.0;}else{if self.pattern_total==0.0||!self.pattern_has_gap{self.pattern.clear();}self.stage=Stage::Prepare;}self.step_prepared=true;return Ok(false);
   }
   Stage::Prepare=>{
    let Some(contour)=self.input.contours.get(self.contour)else{self.stage=if self.radius==0.0{Stage::Done}else{Stage::Source};self.step_prepared=true;return Ok(false);};
    if self.point<contour.points.len(){let p=contour.points[self.point];self.check(p)?;if self.prepared.last().is_none_or(|last|*last!=p)&&!Self::append(&mut self.prepared,&[p],work)?{return Ok(false);}self.point+=1;}
    else{
     let closed=contour.closed;let raw=contour.points.len();let repeated=closed&&self.prepared.len()>1&&self.prepared.first()==self.prepared.last();let length=self.prepared.len()-usize::from(repeated);let retained=length>=2||length==1&&(raw>=2||closed);
     if retained{let target=self.sources.len()+1;if !Self::reserve(&mut self.sources,target,4096,work)?||!work.copy_bytes(std::mem::size_of::<StrokeContour>()){return Ok(false);}}
     if repeated{self.prepared.pop();}if retained{self.sources.push(StrokeContour{points:std::mem::take(&mut self.prepared),closed});}else{self.prepared.clear();}self.contour+=1;self.point=0;
    }
   }
   Stage::Source=>{
    let Some(src)=self.sources.get_mut(self.source)else{self.stage=Stage::Outline;self.step_prepared=true;return Ok(false);};
    if self.pattern.is_empty(){let target=self.runs.len()+1;if !Self::reserve(&mut self.runs,target,MAX_POINTS,work)?||!work.copy_bytes(std::mem::size_of::<Run>()){return Ok(false);}self.runs.push(Run{points:std::mem::take(&mut src.points),closed:src.closed,cap_start:true,cap_end:true,painted:true,tangent:[1.0,0.0]});self.source+=1;return Ok(true);}
    self.edge=0;self.position=0.0;self.dash=0;self.offset=(self.input.style.dash_offset%self.pattern_total+self.pattern_total)%self.pattern_total;self.active=None;self.first=None;self.last=None;self.stage=Stage::Offset;
   }
   Stage::Offset=>{let length=self.pattern[self.dash];if self.offset>0.0&&self.offset>=length{self.offset-=length;self.dash=(self.dash+1)%self.pattern.len();}else{self.remaining=length-self.offset;self.offset=0.0;self.stage=Stage::Walk;}}
   Stage::Walk=>{
    let src=&self.sources[self.source];let n=src.points.len();let count=if src.closed{n}else{n-1};
    if self.remaining==0.0{if self.dash%2==0&&self.active.is_none(){let p=self.location();if !self.create_run(p,p,false,work)?{return Ok(false);}}self.dash=(self.dash+1)%self.pattern.len();self.remaining=self.pattern[self.dash];return Ok(true);}
    if self.edge>=count{if n==1&&self.dash%2==0&&self.active.is_none(){let p=src.points[0];if !self.create_run(p,p,false,work)?{return Ok(false);}}self.stage=Stage::Finish;return Ok(true);}
    let a=src.points[self.edge];let b=src.points[(self.edge+1)%n];let length=(b[0]-a[0]).hypot(b[1]-a[1]);let left=length-self.position;let take=left.min(self.remaining);let from=self.location();let end_edge=take==left;let end_dash=take==self.remaining;let position=self.position+take;
    if take>0.0&&position==self.position{return Err(StrokeOutlineError::Invalid("Stroke dashing exceeds numeric limits"));}let to=if end_edge{b}else{[a[0]+(b[0]-a[0])*position/length,a[1]+(b[1]-a[1])*position/length]};
    if self.dash%2==0{
     if let Some(active)=self.active{if !self.runs[active].painted{if !work.copy_bytes(std::mem::size_of::<Vec2>()){return Ok(false);}*self.runs[active].points.last_mut().unwrap()=to;self.runs[active].painted=true;if self.first.is_none(){self.first=Some(active);}self.last=Some(active);}else if !self.run_point(active,to,work)?{return Ok(false);}}
     else if !self.create_run(from,to,true,work)?{return Ok(false);}
    }else{self.active=None;}
    self.position=position;self.remaining-=take;if end_dash{self.dash=(self.dash+1)%self.pattern.len();self.remaining=self.pattern[self.dash];}if end_edge{self.edge+=1;self.position=0.0;}
   }
   Stage::Finish=>{
    let src=&self.sources[self.source];let seam=if src.closed{if let(Some(first),Some(last))=(self.first,self.last){if self.runs[first].points[0]==src.points[0]&&self.runs[last].points.last()==Some(&src.points[0])&&first!=last{let end=self.runs[last].points.len();Some([self.runs[last].points[end-2],src.points[0],self.runs[first].points[1]])}else{None}}else{None}}else{None};
    if seam.is_some(){let target=self.seams.len()+1;if !Self::reserve(&mut self.seams,target,MAX_POINTS,work)?||!work.copy_bytes(std::mem::size_of::<[Vec2;3]>()){return Ok(false);}}
    if src.closed{if let(Some(first),Some(last))=(self.first,self.last){if self.runs[first].points[0]==src.points[0]&&self.runs[last].points.last()==Some(&src.points[0]){if first==last{self.runs[first].closed=true;self.runs[first].points.pop();}else{self.runs[first].cap_start=false;self.runs[last].cap_end=false;self.seams.push(seam.unwrap());}}}}
    self.active=None;self.source+=1;self.stage=Stage::Source;
   }
   Stage::Outline=>return self.outline(work),Stage::Done=>{}
  }Ok(true)
 }
 pub fn next_work_demand(&self)->RetirementDemand{self.pending}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 pub fn failure_pending(&self)->bool{self.failed.is_some()}
 pub fn settlement_pending(&self)->bool{self.stage==Stage::Done||self.stage==Stage::Prepare&&self.contour>=self.input.contours.len()&&self.radius==0.0||self.stage==Stage::Source&&self.source>=self.sources.len()&&self.runs.is_empty()&&self.seams.is_empty()||self.stage==Stage::Outline&&self.queue.is_empty()&&self.round.is_none()&&self.run>=self.runs.len()&&self.seam>=self.seams.len()}
 pub fn advance(&mut self,budget:usize,work:&mut RetainedWorkLedger)->Result<StrokeOutlineProgress,StrokeOutlineError>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if budget==0||budget as u128>9_007_199_254_740_991{return Err(StrokeOutlineError::Invalid("Stroke work grant must be a positive integer"));}if self.cancelled{return Err(StrokeOutlineError::Cancelled);}if let Some(error)=&self.failed{return Err(error.clone());}
  let d=self.pending;if !work.admits_absolute(d){work.require_absolute(d);return Ok(self.progress());}
  if let Some(root)=work.enter(){let result=(||{for _ in 0..budget{if self.stage==Stage::Done{break;}if !work.can_step(){self.pending=work.next_demand();break;}let before=work.receipt();self.step_prepared=false;let result=self.step(work);let completed=result.as_ref().copied().unwrap_or(true);if work.receipt()!=before||completed||self.step_prepared{work.note_source(root);self.last_turn_work+=1;}if completed{self.work+=1;self.last_turn_semantic_work+=1;}self.pending=work.next_demand();if let Err(error)=result{self.failed=Some(error.clone());return Err(error);}if work.blocked(){break;}}Ok(self.progress())})();work.exit();result}else{self.pending=work.next_demand();Ok(self.progress())}
 }
 fn progress(&self)->StrokeOutlineProgress{
  let phase=match self.stage{Stage::Pattern|Stage::Prepare=>StrokeOutlinePhase::Preparing,Stage::Outline=>StrokeOutlinePhase::Outlining,Stage::Done=>StrokeOutlinePhase::Complete,_=>StrokeOutlinePhase::Dashing};let(completed,total)=match phase{StrokeOutlinePhase::Preparing=>(self.contour,self.input.contours.len()),StrokeOutlinePhase::Dashing=>(self.source,self.sources.len()),_=>(self.run,self.runs.len())};StrokeOutlineProgress{phase,completed,total,points:self.count,work:self.work,done:self.stage==Stage::Done}
 }
 pub fn cancel(&mut self) {self.cancelled=true;self.active=None;}
 pub fn result(&self)->Result<&[Vec<Vec2>],StrokeOutlineError> {if self.cancelled {return Err(StrokeOutlineError::Cancelled);}if let Some(error)=&self.failed {return Err(error.clone());}if self.stage!=Stage::Done {return Err(StrokeOutlineError::Incomplete);}Ok(&self.polygons)}
}
semio_framework_value::artifact_retire_leaf!(StrokeGeometryCap,StrokeGeometryJoin,StrokeOutlinePhase,ArcNode,Stage);
semio_framework_2d_retire::physical_work_retirement!(StrokeOutlineRetirement,StrokeOutlineJob);
impl StrokeOutlineJob{
 /// 🧹️ Moves complete output unchanged and retains interrupted owners for work-granted cleanup.
 pub fn into_retirement(mut self)->(StrokeOutlineRetirement,Option<Vec<Vec<Vec2>>>){let output=if self.stage==Stage::Done&&!self.cancelled&&self.failed.is_none(){Some(std::mem::take(&mut self.polygons))}else{None};self.cancelled=true;self.active=None;(StrokeOutlineRetirement::new(self),output)}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

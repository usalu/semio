//! 🖊️ Exact transformed polygon coverage for https://www.w3.org/TR/SVG11/painting.html#FillProperties.
use crate::editing::{validate_extent,PixelEditError};
use std::cmp::Ordering;
use semio_framework_value::{work::RetainedWorkLedger,RetirementDemand};
use semio_framework_2d::physical_work_retirement;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CoverageRule {NonZero,EvenOdd}
#[derive(Clone,Debug,PartialEq)]
#[derive(semio_framework_value::RetireOwned)]
pub struct CoverageInput {pub width:u32,pub height:u32,pub transform:[f64;6],pub rule:CoverageRule,pub contours:Vec<Vec<[f64;2]>>}
#[derive(Clone,Debug,PartialEq,Eq)]
#[derive(semio_framework_value::RetireOwned)]
pub struct CoverageMask {pub width:u32,pub height:u32,pub coverage:Vec<u8>}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CoveragePhase {Preparing,Sorting,Rasterizing,Complete}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct CoverageProgress {pub phase:CoveragePhase,pub completed:usize,pub total:usize,pub work:u64,pub done:bool}
#[derive(Clone,Copy)]
struct Edge {x:f64,y:f64,dx:f64,winding:i32}
#[derive(Clone,Copy)]
struct SweepEvent {y:f64,edge:usize,add:bool}
#[derive(Clone,Copy,PartialEq,Eq)]
#[doc(hidden)] pub enum Stage {Prepare,Events,Apply,Copy,Sort,Cross,Wind,Span,Finish,Write,Done}
const EPS:f64=1e-10;
const MAX_POINTS:usize=65536;
const MAX_CONTOURS:usize=65536;
const MAX_COORDINATE:f64=1e9;
const NONE:usize=usize::MAX;
fn coordinate(value:f64)->bool {value.is_finite()&&value.abs()<=MAX_COORDINATE}
fn primitive(value:f64)->f64 {if value<=0.0 {0.0} else if value<1.0 {value*value/2.0} else {value-0.5}}
fn integral(a:f64,b:f64,height:f64)->f64 {
    if a.max(b)<=0.0 {0.0} else if a.min(b)>=1.0 {height} else if (b-a).abs()<EPS {((a+b)/2.0).clamp(0.0,1.0)*height} else {(primitive(b)-primitive(a))/(b-a)*height}
}
fn x_at(edge:Edge,y:f64)->f64 {edge.x+(y-edge.y)*edge.dx}

fn grow<T>(values:&mut Vec<T>,extra:usize,limit:usize,work:&mut RetainedWorkLedger)->Result<bool,PixelEditError>{let target=values.len().checked_add(extra).ok_or(PixelEditError::Invalid("Coverage vector extent overflow"))?;if target>limit{return Err(PixelEditError::Invalid("Coverage geometry exceeds point budget"));}if values.capacity()>=target{return Ok(true)}let target=target.max(values.capacity().saturating_mul(2).min(limit));work.reserve_vec(values,target).map_err(PixelEditError::Invalid)}
#[derive(semio_framework_value::RetireOwned)]
struct MergeSort{source:Vec<usize>,target:Vec<usize>,width:usize,left:usize,i:usize,j:usize,k:usize,mid:usize,end:usize,writes:usize}
impl MergeSort{
 fn new(source:Vec<usize>,mut target:Vec<usize>)->Self{target.truncate(source.len());let mut sorter=Self{source,target,width:1,left:0,i:0,j:0,k:0,mid:0,end:0,writes:0};sorter.setup();sorter}
 fn done(&self)->bool{self.width>=self.source.len()}
 fn setup(&mut self){self.i=self.left;self.mid=(self.left+self.width).min(self.source.len());self.j=self.mid;self.k=self.left;self.end=(self.left+2*self.width).min(self.source.len());}
 fn step(&mut self,work:&mut RetainedWorkLedger,compare:impl Fn(usize,usize)->Ordering)->Result<bool,PixelEditError>{
  if self.done(){return Ok(true)}if !work.reserve_vec(&mut self.target,self.source.len()).map_err(PixelEditError::Invalid)?{return Ok(false)}
  if self.target.len()<self.source.len(){let next=(self.target.len()+512).min(self.source.len());let bytes=(next-self.target.len())*std::mem::size_of::<usize>();if !work.copy_bytes(bytes){return Ok(false)}self.target.resize(next,0);return Ok(false)}
  if !work.copy_bytes(std::mem::size_of::<usize>()){return Ok(false)}
  self.target[self.k]=if self.i<self.mid&&(self.j>=self.end||compare(self.source[self.i],self.source[self.j])!=Ordering::Greater){let value=self.source[self.i];self.i+=1;value}else{let value=self.source[self.j];self.j+=1;value};
  self.k+=1;self.writes+=1;if self.k==self.end{self.left+=2*self.width;if self.left>=self.source.len(){std::mem::swap(&mut self.source,&mut self.target);self.width*=2;self.left=0;}self.setup();}Ok(true)
 }
}
/// ⏱️ Actual coverage storage and sort scratch remain owned through funded work and controlled close.
#[derive(semio_framework_value::RetireOwned)]
pub struct CoverageJob{
 #[doc(hidden)]pub input:CoverageInput,#[doc(hidden)]pub mask:CoverageMask,areas:Vec<f64>,edges:Vec<Edge>,events:Vec<SweepEvent>,positions:Vec<usize>,active:Vec<usize>,event_ids:Vec<usize>,sorted:Vec<usize>,copied:Vec<usize>,scratch:Vec<usize>,sorter:Option<MergeSort>,
 #[doc(hidden)]pub stage:Stage,cancelled:bool,#[doc(hidden)]pub failed:Option<PixelEditError>,#[doc(hidden)]pub work:u64,count:usize,validation_started:bool,validated:bool,initialized:bool,checked_contour:usize,checked_points:usize,demand:RetirementDemand,last_turn_work:u64,last_turn_semantic_work:u64,step_semantic_error:u64,
 contour:usize,point:usize,prepared:usize,preparation_total:usize,first:Option<[f64;2]>,previous:Option<[f64;2]>,event:usize,copy:usize,scan:usize,row:usize,y:f64,end:f64,winding:i32,left:usize,span_left:usize,span_right:usize,pixel:usize,pixel_end:usize,dirty_start:usize,dirty_end:usize,
}
impl CoverageJob{
 pub fn new(input:CoverageInput)->Self{let width=input.width as usize;let mask=CoverageMask{width:input.width,height:input.height,coverage:Vec::new()};Self{input,mask,areas:Vec::new(),edges:Vec::new(),events:Vec::new(),positions:Vec::new(),active:Vec::new(),event_ids:Vec::new(),sorted:Vec::new(),copied:Vec::new(),scratch:Vec::new(),sorter:None,stage:Stage::Prepare,cancelled:false,failed:None,work:0,count:0,validation_started:false,validated:false,initialized:false,checked_contour:0,checked_points:0,demand:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0,step_semantic_error:0,contour:0,point:0,prepared:0,preparation_total:0,first:None,previous:None,event:0,copy:0,scan:0,row:0,y:0.,end:0.,winding:0,left:NONE,span_left:NONE,span_right:NONE,pixel:0,pixel_end:0,dirty_start:width,dirty_end:0}}
 fn initialize(&mut self,work:&mut RetainedWorkLedger)->Result<bool,PixelEditError>{
  if !self.validation_started{self.step_semantic_error=1;self.count=validate_extent(self.input.width,self.input.height)?;if !self.input.transform.into_iter().all(coordinate)||self.input.contours.len()>MAX_CONTOURS{return Err(PixelEditError::Invalid("Invalid vector coverage contract"));}self.validation_started=true;return Ok(true)}
  if !self.validated{self.step_semantic_error=1;if let Some(contour)=self.input.contours.get(self.checked_contour){self.checked_points=self.checked_points.checked_add(contour.len()).ok_or(PixelEditError::Invalid("Coverage geometry exceeds point budget"))?;if self.checked_points>MAX_POINTS{return Err(PixelEditError::Invalid("Coverage geometry exceeds point budget"));}self.checked_contour+=1;}else{self.preparation_total=self.checked_points+self.input.contours.len();self.validated=true;}return Ok(true)}
  if !work.reserve_vec(&mut self.mask.coverage,self.count).map_err(PixelEditError::Invalid)?{return Ok(false)}
  if self.mask.coverage.len()<self.count{let next=(self.mask.coverage.len()+4096).min(self.count);if !work.copy_bytes(next-self.mask.coverage.len()){return Ok(false)}self.mask.coverage.resize(next,0);return Ok(false)}
  let width=self.input.width as usize;if !work.reserve_vec(&mut self.areas,width).map_err(PixelEditError::Invalid)?{return Ok(false)}
  if self.areas.len()<width{let next=(self.areas.len()+512).min(width);if !work.copy_bytes((next-self.areas.len())*std::mem::size_of::<f64>()){return Ok(false)}self.areas.resize(next,0.);return Ok(false)}
  self.initialized=true;Ok(false)
 }
 fn edge(&mut self,from:[f64;2],to:[f64;2],work:&mut RetainedWorkLedger)->Result<bool,PixelEditError>{
  if from[1]==to[1]{return Ok(true)}let(low,high)=if from[1]<to[1]{(from,to)}else{(to,from)};let start=low[1].max(0.);let end=high[1].min(f64::from(self.input.height));if start>=end{return Ok(true)}
  let dx=(high[0]-low[0])/(high[1]-low[1]);if !dx.is_finite(){self.step_semantic_error=1;return Err(PixelEditError::Invalid("Coverage edge exceeds numeric limits"));}
  if !grow(&mut self.edges,1,MAX_POINTS,work)?||!grow(&mut self.positions,1,MAX_POINTS,work)?||!grow(&mut self.events,2,2*MAX_POINTS,work)?||!grow(&mut self.event_ids,2,2*MAX_POINTS,work)?{return Ok(false)}
  let bytes=std::mem::size_of::<Edge>()+3*std::mem::size_of::<usize>()+2*std::mem::size_of::<SweepEvent>();if !work.copy_bytes(bytes){return Ok(false)}
  let id=self.edges.len();self.edges.push(Edge{x:low[0],y:low[1],dx,winding:if from[1]<to[1]{1}else{-1}});self.positions.push(NONE);self.event_ids.extend([self.events.len(),self.events.len()+1]);self.events.extend([SweepEvent{y:start,edge:id,add:true},SweepEvent{y:end,edge:id,add:false}]);Ok(true)
 }
 fn inside(&self,winding:i32)->bool{match self.input.rule{CoverageRule::NonZero=>winding!=0,CoverageRule::EvenOdd=>winding.abs()%2==1}}
 fn step(&mut self,work:&mut RetainedWorkLedger)->Result<bool,PixelEditError>{
  if !self.initialized{return self.initialize(work)}
  match self.stage{
   Stage::Prepare=>{
    let Some(points)=self.input.contours.get(self.contour)else{self.sorter=Some(MergeSort::new(std::mem::take(&mut self.event_ids),std::mem::take(&mut self.scratch)));self.stage=Stage::Events;return Ok(true)};
    if self.point<points.len(){let value=points[self.point];if !value.into_iter().all(coordinate){self.step_semantic_error=1;return Err(PixelEditError::Invalid("Invalid coverage point"));}let m=self.input.transform;let point=[m[0]*value[0]+m[2]*value[1]+m[4],m[1]*value[0]+m[3]*value[1]+m[5]];if !point.into_iter().all(coordinate){self.step_semantic_error=1;return Err(PixelEditError::Invalid("Transformed coverage exceeds coordinate budget"));}if let Some(previous)=self.previous{if !self.edge(previous,point,work)?{return Ok(false)}}else{self.first=Some(point);}self.previous=Some(point);self.point+=1;
    }else{if let(Some(previous),Some(first))=(self.previous,self.first){if !self.edge(previous,first,work)?{return Ok(false)}}self.contour+=1;self.point=0;self.first=None;self.previous=None;}self.prepared+=1;
   }
   Stage::Events=>{
    if self.sorter.as_ref().unwrap().done(){let mut sorter=self.sorter.take().unwrap();self.sorted=std::mem::take(&mut sorter.source);self.scratch=std::mem::take(&mut sorter.target);self.stage=Stage::Apply;}
    else{let events=&self.events;return self.sorter.as_mut().unwrap().step(work,|a,b|events[a].y.total_cmp(&events[b].y).then_with(||events[a].add.cmp(&events[b].add)))}
   }
   Stage::Apply=>{
    if self.y>=f64::from(self.input.height){self.stage=Stage::Done;return Ok(false)}let event=self.sorted.get(self.event).map(|id|self.events[*id]);
    if let Some(event)=event.filter(|event|event.y<=self.y){
     if event.add{if !grow(&mut self.active,1,MAX_POINTS,work)?{return Ok(false)}if !work.copy_bytes(2*std::mem::size_of::<usize>()){return Ok(false)}self.positions[event.edge]=self.active.len();self.active.push(event.edge);}
     else{let at=self.positions[event.edge];let bytes=(if at+1<self.active.len(){3}else{1})*std::mem::size_of::<usize>();if !work.copy_bytes(bytes){return Ok(false)}let last=self.active.pop().unwrap();if at<self.active.len(){self.active[at]=last;self.positions[last]=at;}self.positions[event.edge]=NONE;}self.event+=1;
    }else{self.end=((self.row+1)as f64).min(event.map_or(f64::from(self.input.height),|event|event.y)).min(f64::from(self.input.height));self.copy=0;self.copied.clear();self.stage=Stage::Copy;}
   }
   Stage::Copy=>{
    if self.copy<self.active.len(){if !grow(&mut self.copied,1,MAX_POINTS,work)?||!work.copy_bytes(std::mem::size_of::<usize>()){return Ok(false)}self.copied.push(self.active[self.copy]);self.copy+=1;}
    else{self.sorter=Some(MergeSort::new(std::mem::take(&mut self.copied),std::mem::take(&mut self.scratch)));self.stage=Stage::Sort;}
   }
   Stage::Sort=>{
    if self.sorter.as_ref().unwrap().done(){let mut sorter=self.sorter.take().unwrap();self.copied=std::mem::take(&mut sorter.source);self.scratch=std::mem::take(&mut sorter.target);self.scan=0;self.stage=Stage::Cross;}
    else{let edges=&self.edges;let y=self.y;return self.sorter.as_mut().unwrap().step(work,|a,b|{let x=edges[a];let z=edges[b];let delta=x_at(x,y)-x_at(z,y);if delta.abs()>EPS{delta.total_cmp(&0.)}else{x.dx.total_cmp(&z.dx).then_with(||a.cmp(&b))}})}
   }
   Stage::Cross=>{
    if self.scan+1<self.copied.len(){let left=self.edges[self.copied[self.scan]];self.scan+=1;let right=self.edges[self.copied[self.scan]];let slope=left.dx-right.dx;if slope>0.{let cross=self.y+(x_at(right,self.y)-x_at(left,self.y))/slope;if cross>self.y+EPS&&cross<self.end{self.end=cross;}}}
    else{self.scan=0;self.winding=0;self.left=NONE;self.stage=Stage::Wind;}
   }
   Stage::Wind=>{
    if self.scan>=self.copied.len(){self.stage=Stage::Finish;return Ok(true)}let id=self.copied[self.scan];self.scan+=1;let before=self.inside(self.winding);self.winding+=self.edges[id].winding;let after=self.inside(self.winding);if !before&&after{self.left=id;}if before&&!after{self.span_left=self.left;self.span_right=id;let left=self.edges[self.left];let right=self.edges[id];self.pixel=x_at(left,self.y).min(x_at(left,self.end)).floor().max(0.)as usize;self.pixel_end=x_at(right,self.y).max(x_at(right,self.end)).ceil().clamp(0.,f64::from(self.input.width))as usize;self.stage=Stage::Span;}
   }
   Stage::Span=>{
    if self.pixel>=self.pixel_end{self.stage=Stage::Wind;return Ok(true)}if !work.copy_bytes(std::mem::size_of::<f64>()){return Ok(false)}let x=self.pixel;self.pixel+=1;let left=self.edges[self.span_left];let right=self.edges[self.span_right];let height=self.end-self.y;self.areas[x]+=integral(x_at(right,self.y)-x as f64,x_at(right,self.end)-x as f64,height)-integral(x_at(left,self.y)-x as f64,x_at(left,self.end)-x as f64,height);self.dirty_start=self.dirty_start.min(x);self.dirty_end=self.dirty_end.max(x+1);
   }
   Stage::Finish=>{self.y=self.end;if self.y>=(self.row+1)as f64{self.pixel=self.dirty_start;self.stage=Stage::Write;}else{self.stage=Stage::Apply;}}
   Stage::Write=>{
    if self.pixel<self.dirty_end{if !work.copy_bytes(1+std::mem::size_of::<f64>()){return Ok(false)}let x=self.pixel;self.pixel+=1;self.mask.coverage[self.row*self.input.width as usize+x]=(self.areas[x].clamp(0.,1.)*255.).round()as u8;self.areas[x]=0.;}
    else{self.row+=1;self.dirty_start=self.input.width as usize;self.dirty_end=0;self.stage=Stage::Apply;}
   }
   Stage::Done=>{}
  }Ok(true)
 }
 fn progress(&self)->CoverageProgress{let phase=match self.stage{Stage::Prepare=>CoveragePhase::Preparing,Stage::Events=>CoveragePhase::Sorting,Stage::Done=>CoveragePhase::Complete,_=>CoveragePhase::Rasterizing};let(completed,total)=match phase{CoveragePhase::Preparing=>(self.prepared,self.preparation_total),CoveragePhase::Sorting=>{let sort=self.sorter.as_ref().unwrap();let count=sort.source.len();let passes=if count<2{0}else{usize::BITS-(count-1).leading_zeros()};(sort.writes,count*passes as usize)},_=>(self.row*self.input.width as usize,self.mask.coverage.len())};CoverageProgress{phase,completed,total,work:self.work,done:self.stage==Stage::Done}}
 pub fn next_work_demand(&self)->RetirementDemand{self.demand}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 pub fn failure_pending(&self)->bool{self.failed.is_some()}
 pub fn settlement_pending(&self)->bool{!self.cancelled&&self.failed.is_none()&&(self.stage==Stage::Done||(self.initialized&&self.stage==Stage::Apply&&self.y>=f64::from(self.input.height)))}
 pub fn advance(&mut self,budget:usize,work:&mut RetainedWorkLedger)->Result<CoverageProgress,PixelEditError>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if budget==0||budget as u128>9_007_199_254_740_991{return Err(PixelEditError::Invalid("Coverage work grant must be a positive integer"));}if self.cancelled{return Err(PixelEditError::Cancelled)}if let Some(error)=&self.failed{return Err(error.clone())}
  for _ in 0..budget{if self.stage==Stage::Done{break}if !work.admits_absolute(self.demand){work.require_absolute(self.demand);break}let Some(root)=work.enter()else{self.demand=work.next_demand();break};let before=work.receipt();self.step_semantic_error=0;let result=self.step(work);let semantic=match &result{Ok(true)=>1,Err(_)=>self.step_semantic_error,_=>0};let actual=work.receipt()!=before||!work.blocked();if actual{work.note_source(root);self.last_turn_work+=1;self.work+=semantic;self.last_turn_semantic_work+=semantic;}work.exit();self.demand=work.next_demand();match result{Ok(_)=>{},Err(error)=>{self.failed=Some(error.clone());return Err(error)}}if work.blocked()||!actual{break}}
  Ok(self.progress())
 }
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn result(&self)->Result<&CoverageMask,PixelEditError>{if self.cancelled{return Err(PixelEditError::Cancelled)}if let Some(error)=&self.failed{return Err(error.clone())}if self.stage!=Stage::Done{return Err(PixelEditError::Incomplete)}Ok(&self.mask)}
 /// 🧹️ Output custody moves without allocation; every private input and scratch owner remains in controlled retirement.
 pub fn into_retirement(mut self)->(CoverageRetirement,Option<CoverageMask>){let output=if self.stage==Stage::Done&&!self.cancelled&&self.failed.is_none(){Some(CoverageMask{width:self.mask.width,height:self.mask.height,coverage:std::mem::take(&mut self.mask.coverage)})}else{None};self.cancelled=true;(CoverageRetirement::new(self),output)}
}
semio_framework_value::artifact_retire_leaf!(CoverageRule,CoveragePhase,Edge,SweepEvent,Stage);
physical_work_retirement!(CoverageRetirement,CoverageJob);

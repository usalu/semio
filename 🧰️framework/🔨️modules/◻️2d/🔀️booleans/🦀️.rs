//! 🔀️ Filled planar arrangements with spatial queries, bounded grants and private outputs.
use semio_framework_2d_engine::{PathSegment,Vec2};
use semio_framework_2d_retire::WorkRetirementProgress;
pub type BooleanRetirementProgress=WorkRetirementProgress;
use std::cmp::Ordering;
use semio_framework_value::numeric_scratch::{NumericIndex,NumericWorkInsertCursor,NumericWorkRemoveCursor};
use semio_framework_value::{work::RetainedWorkLedger,RetirementDemand};
#[path="🛤️paths/🦀️.rs"]
pub mod paths;
type Box2=[f64;4];
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BooleanOperation {Union,Difference,Intersection,Xor}
impl BooleanOperation {
 pub fn parse(value:&str)->Result<Self,BooleanError> {match value {"union"=>Ok(Self::Union),"difference"=>Ok(Self::Difference),"intersection"=>Ok(Self::Intersection),"xor"=>Ok(Self::Xor),_=>Err(BooleanError::Invalid("Unknown boolean operation"))}}
 fn apply(self,a:bool,b:bool)->bool {match self {Self::Union=>a||b,Self::Difference=>a&&!b,Self::Intersection=>a&&b,Self::Xor=>a!=b}}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BooleanFillRule {Nonzero,Evenodd}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct BooleanOperand {pub contours:Vec<Vec<Vec2>>,pub fill_rule:BooleanFillRule}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct BooleanInput {
 pub operation:BooleanOperation,pub operands:Vec<BooleanOperand>,pub epsilon:f64,pub max_edges:usize,pub max_parameters:usize,pub max_atomic_edges:usize,pub max_segments:usize,pub max_work:u64,
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BooleanPhase {Preparing,Indexing,Intersections,Splitting,Classifying,Contours,Compacting,Emitting,Complete}
impl BooleanPhase {
 pub fn as_str(self)->&'static str {match self {Self::Preparing=>"preparing",Self::Indexing=>"indexing",Self::Intersections=>"intersections",Self::Splitting=>"splitting",Self::Classifying=>"classifying",Self::Contours=>"contours",Self::Compacting=>"compacting",Self::Emitting=>"emitting",Self::Complete=>"complete"}}
}
#[derive(Clone,Copy,Debug)]
pub struct BooleanProgress {
 pub phase:BooleanPhase,pub operands:usize,pub vertices:usize,pub edges:usize,pub parameters:usize,pub pairs:u64,pub atomic_edges:usize,pub boundary_edges:usize,pub contours:usize,pub segments:usize,pub work:u64,pub done:bool,
}
#[derive(Clone,Debug,PartialEq,Eq)]
#[derive(semio_framework_value::RetireOwned)]
pub enum BooleanError {Invalid(&'static str),Refused(semio_framework_value::ValueError),Incomplete,Cancelled}
impl std::fmt::Display for BooleanError {
 fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {f.write_str(match self {Self::Invalid(message)=>message,Self::Refused(error)=>error.message.as_ref(),Self::Incomplete=>"Boolean incomplete",Self::Cancelled=>"Boolean cancelled"})}
}
impl std::error::Error for BooleanError {}
#[derive(semio_framework_value::RetireOwned)]
struct Edge {a:Vec2,b:Vec2,operand:usize,bounds:Box2,parameters:NumericIndex<u64,()>}
#[derive(Clone,Copy)]
struct Node {bounds:Box2,left:usize,right:usize,edge:Option<usize>,maximum:usize}
#[derive(Clone,Copy)]
struct Boundary {from:usize,to:usize,used:bool}
#[derive(semio_framework_value::RetireOwned)]
struct Ring {points:Vec<Vec2>,anchor:usize}
#[derive(Clone,Copy)]
struct IndexItem {key:u32,edge:usize}
#[derive(Clone,Copy)]
struct RingItem {key:[f64;4],ring:usize}
fn index_order(a:&IndexItem,b:&IndexItem)->Ordering {a.key.cmp(&b.key).then(a.edge.cmp(&b.edge))}
fn ring_order(a:&RingItem,b:&RingItem)->Ordering {for at in 0..4 {let order=a.key[at].total_cmp(&b.key[at]);if order!=Ordering::Equal {return order;}}Ordering::Equal}
struct Heap<T> {values:Vec<T>,compare:fn(&T,&T)->Ordering}
impl<T> Heap<T> {
 fn new(compare:fn(&T,&T)->Ordering)->Self {Self {values:Vec::new(),compare}}
 fn pop(&mut self)->Option<T> {if self.values.is_empty() {return None;}let value=self.values.swap_remove(0);let mut at=0;while at*2+1<self.values.len() {let mut child=at*2+1;if child+1<self.values.len()&&(self.compare)(&self.values[child+1],&self.values[child])==Ordering::Less {child+=1;}if (self.compare)(&self.values[at],&self.values[child])!=Ordering::Greater {break;}self.values.swap(at,child);at=child;}Some(value)}
}
enum HeapWorkPop<T>{Pending,Empty,Ready(T)}
impl<T:Copy> Heap<T>{
 fn push_work(&mut self,value:T,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,&'static str>{
  let Some(root)=work.enter()else{return Ok(false);};let before=work.receipt();let result=(||{let needed=self.values.len().checked_add(1).ok_or("boolean heap extent overflow")?;if self.values.capacity()<needed{let target=self.values.capacity().checked_mul(2).ok_or("boolean heap capacity overflow")?.max(16).max(needed);if !work.reserve_vec(&mut self.values,target,alloc)?{return Ok(false);}}if !work.vec_slot(&mut self.values)?{return Ok(false);}work.vec_push(&mut self.values,value).map_err(|(error,_)|error)?;let mut at=self.values.len()-1;while at>0{let parent=(at-1)/2;if(self.compare)(&self.values[parent],&self.values[at])!=Ordering::Greater{break;}self.values.swap(parent,at);at=parent;}Ok(true)})();if matches!(&result,Ok(true))||work.receipt()!=before{work.note_source(root);}work.exit();result
 }
 fn pop_work(&mut self,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<HeapWorkPop<T>,&'static str>{
  if self.values.is_empty(){return Ok(HeapWorkPop::Empty);}let Some(root)=work.enter()else{return Ok(HeapWorkPop::Pending);};if !work.can_step(){work.exit();return Ok(HeapWorkPop::Pending);}let value=self.pop().ok_or("pop target absent")?;work.note_source(root);work.exit();Ok(HeapWorkPop::Ready(value))
 }

}
#[derive(Clone,Copy,PartialEq,Eq)]
enum IndexMode {Push,Leaves,Levels}
#[derive(Clone,Copy,PartialEq,Eq)]
enum ClassifyMode {Start,Nearest,Ray,Fold}
#[derive(Clone,Copy,PartialEq,Eq)]
enum CompactMode {Scan,Measure}
fn point(p:Vec2)->Result<Vec2,BooleanError> {if !p.into_iter().all(|n|n.is_finite()&&n.abs()<=1e12) {return Err(BooleanError::Invalid("Invalid boolean point"));}Ok(p.map(|n|if n==0.0 {0.0} else {n}))}
fn cross(a:Vec2,b:Vec2,c:Vec2)->f64 {(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])}
fn length(a:Vec2,b:Vec2)->f64 {(b[0]-a[0]).hypot(b[1]-a[1])}
fn bounds(a:Vec2,b:Vec2)->Box2 {[a[0].min(b[0]),a[1].min(b[1]),a[0].max(b[0]),a[1].max(b[1])]}
fn merge(a:Box2,b:Box2)->Box2 {[a[0].min(b[0]),a[1].min(b[1]),a[2].max(b[2]),a[3].max(b[3])]}
fn intersects(a:Box2,b:Box2,e:f64)->bool {a[0]<=b[2]+e&&a[2]+e>=b[0]&&a[1]<=b[3]+e&&a[3]+e>=b[1]}
fn box_distance(b:Box2,p:Vec2)->f64 {(b[0]-p[0]).max(0.0).max(p[0]-b[2]).hypot((b[1]-p[1]).max(0.0).max(p[1]-b[3]))}
fn ray(b:Box2,p:Vec2,axis:usize,direction:f64)->bool {(if direction>0.0{b[axis+2]>=p[axis]}else{b[axis]<=p[axis]})&&b[1-axis]<=p[1-axis]&&b[3-axis]>p[1-axis]}
fn interpolate(e:&Edge,t:f64)->Result<Vec2,BooleanError> {if t==0.0 {return Ok(e.a);}if t==1.0 {return Ok(e.b);}point([e.a[0]+(e.b[0]-e.a[0])*t,e.a[1]+(e.b[1]-e.a[1])*t])}
fn parameter(e:&Edge,p:Vec2)->f64 {let (dx,dy)=(e.b[0]-e.a[0],e.b[1]-e.a[1]);if dx.abs()>=dy.abs() {(p[0]-e.a[0])/dx} else {(p[1]-e.a[1])/dy}}
fn endpoint(e:&Edge,p:Vec2,epsilon:f64,len:f64)->Option<f64>{let t=parameter(e,p);let tolerance=epsilon/len;(t>=-tolerance&&t<=1.0+tolerance&&cross(e.a,e.b,p).abs()<=epsilon*len).then(||t.clamp(0.0,1.0))}
fn distance(e:&Edge,p:Vec2)->Result<f64,BooleanError> {let (dx,dy)=(e.b[0]-e.a[0],e.b[1]-e.a[1]);let len=dx.hypot(dy);let t=(((p[0]-e.a[0])*(dx/len)+(p[1]-e.a[1])*(dy/len))/len).clamp(0.0,1.0);Ok(length(p,interpolate(e,t)?))}
fn winding(e:&Edge,p:Vec2,axis:usize,direction:f64)->i32 {let a=[e.a[axis]*direction,e.a[1-axis]];let b=[e.b[axis]*direction,e.b[1-axis]];let p=[p[axis]*direction,p[1-axis]];let c=cross(a,b,p);if a[1]<=p[1]&&b[1]>p[1]&&c>0.0{1}else if a[1]>p[1]&&b[1]<=p[1]&&c<0.0{-1}else{0}}
fn spread(mut n:u32)->u32 {n=(n|(n<<8))&0x00ff00ff;n=(n|(n<<4))&0x0f0f0f0f;n=(n|(n<<2))&0x33333333;(n|(n<<1))&0x55555555}
fn morton(e:&Edge,b:Box2)->u32 {
 let x=if b[2]==b[0] {0} else {(((e.a[0]+e.b[0])/2.0-b[0])/(b[2]-b[0])*65535.0).floor().clamp(0.0,65535.0) as u32};
 let y=if b[3]==b[1] {0} else {(((e.a[1]+e.b[1])/2.0-b[1])/(b[3]-b[1])*65535.0).floor().clamp(0.0,65535.0) as u32};
 spread(x)|(spread(y)<<1)
}
/// ⏱️ Prepare, intersect, classify and emit owned regions under explicit work grants.
fn reserve_for<T>(values:&mut Vec<T>,needed:usize,minimum:usize,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{if values.capacity()>=needed{return Ok(true);}let target=values.capacity().checked_mul(2).ok_or(BooleanError::Invalid("Boolean capacity overflow"))?.max(needed).max(minimum);work.reserve_vec(values,target,alloc).map_err(BooleanError::Invalid)}
fn slot<T>(values:&mut Vec<T>,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{let needed=values.len().checked_add(1).ok_or(BooleanError::Invalid("Boolean extent overflow"))?;if !reserve_for(values,needed,16,work,alloc)?{return Ok(false);}work.vec_slot(values).map_err(BooleanError::Invalid)}
fn move_from<T:Default>(target:&mut Vec<T>,source:&mut T,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<(),BooleanError>{match work.vec_push(target,std::mem::take(source)){Ok(())=>Ok(()),Err((error,value))=>{*source=value;Err(BooleanError::Invalid(error))}}}
fn move_option<T>(target:&mut Vec<T>,source:&mut Option<T>,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<(),BooleanError>{let value=source.take().ok_or(BooleanError::Invalid("Boolean pending owner absent"))?;match work.vec_push(target,value){Ok(())=>Ok(()),Err((error,value))=>{*source=Some(value);Err(BooleanError::Invalid(error))}}}
fn insert_work<K:Copy+Ord,V>(cursor:&mut Option<NumericWorkInsertCursor<K,V>>,index:&mut NumericIndex<K,V>,key:K,value:impl FnOnce()->V,work:&mut RetainedWorkLedger,actual:&mut u64,alloc:&mut Option<usize>)->Result<bool,BooleanError>{if cursor.is_none(){if !work.can_step(){return Ok(false);}*cursor=Some(NumericWorkInsertCursor::new(key,value()));*actual=1;return Ok(false);}let child=cursor.as_mut().ok_or(BooleanError::Invalid("cursor absent"))?;if child.is_complete(){return Ok(true);}let result=child.advance(index,1,work,alloc);*actual=child.last_turn_work();result.map_err(BooleanError::Refused)?;Ok(false)}
fn remove_work<K:Copy+Ord,V>(cursor:&mut Option<NumericWorkRemoveCursor<K,V>>,index:&mut NumericIndex<K,V>,key:K,work:&mut RetainedWorkLedger,actual:&mut u64)->Result<bool,BooleanError>{if cursor.is_none(){if !work.can_step(){return Ok(false);}*cursor=Some(NumericWorkRemoveCursor::new(key));*actual=1;return Ok(false);}let child=cursor.as_mut().ok_or(BooleanError::Invalid("cursor absent"))?;if child.is_complete(){return Ok(true);}let result=child.advance(index,1,work);*actual=child.last_turn_work();result.map_err(BooleanError::Refused)?;Ok(false)}
#[derive(Clone,Copy)]struct AtomicPending{p:Vec2,q:Vec2,from:usize,to:usize,stage:u8}
semio_framework_value::artifact_retire_leaf!(AtomicPending);
#[derive(semio_framework_value::RetireOwned)]
pub struct BooleanJob {
 input:BooleanInput,phase:BooleanPhase,work:u64,pending:RetirementDemand,last_turn_work:u64,last_turn_semantic_work:u64,step_work:u64,cancelled:bool,failure:Option<BooleanError>,prepared:usize,vertices:usize,parameters:usize,pairs:u64,
 operand:usize,contour:usize,at:usize,entering:bool,first:Option<Vec2>,previous:Option<Vec2>,bounds:Box2,source:Vec<Edge>,
 retired_levels:Vec<Vec<usize>>,index_mode:IndexMode,index_at:usize,index_heap:Heap<IndexItem>,tree:Vec<Node>,level:Vec<usize>,next_level:Vec<usize>,root:usize,query:Vec<usize>,pivot:usize,
 split_at:usize,split_started:bool,split_heap:Heap<f64>,split_build:bool,split_previous:Option<f64>,nodes:Vec<Vec2>,grid:NumericIndex<(i64,i64),Vec<usize>>,atomic:Vec<(usize,usize)>,atomic_ids:NumericIndex<(usize,usize),()>,
 classify_at:usize,classify_mode:ClassifyMode,classify_axis:usize,classify_direction:f64,midpoint:Vec2,left_point:Vec2,right_point:Vec2,nearest:f64,left_winding:Vec<i32>,right_winding:Vec<i32>,fold_at:usize,left_filled:bool,right_filled:bool,
 boundary:Vec<Boundary>,outgoing:NumericIndex<usize,Vec<usize>>,boundary_at:usize,current:Option<usize>,start:usize,raw:Vec<Vec<usize>>,ring:Vec<usize>,positions:NumericIndex<usize,usize>,selecting:bool,choice_at:usize,choice_node:usize,reverse_angle:f64,best:Option<usize>,best_angle:f64,
 split_ring:Option<Vec<usize>>,ring_split_at:usize,ring_split_stop:usize,ring_split_copy:bool,
 compact_at:usize,compact_vertex:usize,compact_mode:CompactMode,compact_points:Vec<Vec2>,area:f64,lower:usize,upper:usize,rings:Vec<Ring>,ring_heap:Heap<RingItem>,emitting:Option<usize>,emit_at:usize,output:Vec<PathSegment>,
 intersection_active:bool,intersection_actions:[Option<(usize,f64)>;4],intersection_at:usize,intersection_len:usize,parameter_insert:Option<NumericWorkInsertCursor<u64,()>>,parameter_remove:Option<NumericWorkRemoveCursor<u64,()>>,split_seed:u8,split_parameter:Option<u64>,split_removed:bool,pending_atomic:Option<AtomicPending>,intern_current:Option<(Vec2,(i64,i64),usize)>,intern_phase:u8,grid_insert:Option<NumericWorkInsertCursor<(i64,i64),Vec<usize>>>,atomic_insert:Option<NumericWorkInsertCursor<(usize,usize),()>>,outgoing_insert:Option<NumericWorkInsertCursor<usize,Vec<usize>>>,position_insert:Option<NumericWorkInsertCursor<usize,usize>>,position_remove:Option<NumericWorkRemoveCursor<usize,usize>>,
}
impl BooleanJob {
 fn validate(input:&BooleanInput)->Result<(),BooleanError>{if input.operands.is_empty()||input.operands.len()>1024{return Err(BooleanError::Invalid("Invalid boolean contract"));}Self::validate_limits(input.epsilon,input.max_edges,input.max_parameters,input.max_atomic_edges,input.max_segments,input.max_work)}
 /// 📏️ Validates the defining scalar contract without constructing or discarding operand owners.
 pub fn validate_limits(epsilon:f64,max_edges:usize,max_parameters:usize,max_atomic_edges:usize,max_segments:usize,max_work:u64)->Result<(),BooleanError>{
  if !epsilon.is_finite()||!(1e-12..=16.0).contains(&epsilon)||!(1..=262144).contains(&max_edges)||!(1..=1048576).contains(&max_parameters)||!(1..=262144).contains(&max_atomic_edges)||!(1..=327680).contains(&max_segments)||!(1..=1000000000).contains(&max_work){return Err(BooleanError::Invalid("Invalid boolean contract"));}Ok(())
 }
 /// 📥️ Invalid contracts retain their original owned operands until caller-funded close.
 pub fn failure_pending(&self)->bool{self.failure.is_some()}
 pub fn new(input:BooleanInput)->Self{let failure=Self::validate(&input).err();
  Self {intersection_active:false,intersection_actions:[None;4],intersection_at:0,intersection_len:0,parameter_insert:None,parameter_remove:None,split_seed:0,split_parameter:None,split_removed:false,pending_atomic:None,intern_current:None,intern_phase:0,grid_insert:None,atomic_insert:None,outgoing_insert:None,position_insert:None,position_remove:None,input,phase:BooleanPhase::Preparing,work:0,pending:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0,step_work:0,cancelled:false,failure,prepared:0,vertices:0,parameters:0,pairs:0,operand:0,contour:0,at:0,entering:true,first:None,previous:None,bounds:[f64::INFINITY,f64::INFINITY,f64::NEG_INFINITY,f64::NEG_INFINITY],source:Vec::new(),retired_levels:Vec::new(),index_mode:IndexMode::Push,index_at:0,index_heap:Heap::new(index_order),tree:Vec::new(),level:Vec::new(),next_level:Vec::new(),root:usize::MAX,query:Vec::new(),pivot:0,split_at:0,split_started:false,split_heap:Heap::new(f64::total_cmp),split_build:true,split_previous:None,nodes:Vec::new(),grid:NumericIndex::new(),atomic:Vec::new(),atomic_ids:NumericIndex::new(),classify_at:0,classify_mode:ClassifyMode::Start,classify_axis:0,classify_direction:1.0,midpoint:[0.0;2],left_point:[0.0;2],right_point:[0.0;2],nearest:f64::INFINITY,left_winding:Vec::new(),right_winding:Vec::new(),fold_at:0,left_filled:false,right_filled:false,boundary:Vec::new(),outgoing:NumericIndex::new(),boundary_at:0,current:None,start:0,raw:Vec::new(),ring:Vec::new(),positions:NumericIndex::new(),selecting:false,choice_at:0,choice_node:0,reverse_angle:0.0,best:None,best_angle:f64::INFINITY,split_ring:None,ring_split_at:0,ring_split_stop:0,ring_split_copy:true,compact_at:0,compact_vertex:0,compact_mode:CompactMode::Scan,compact_points:Vec::new(),area:0.0,lower:0,upper:0,rings:Vec::new(),ring_heap:Heap::new(ring_order),emitting:None,emit_at:0,output:Vec::new()}
 }
 fn add_edge(&mut self,a:Vec2,b:Vec2,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{if length(a,b)==0.0{return Ok(true);}if self.source.len()>=self.input.max_edges{return Err(BooleanError::Invalid("Boolean exceeds edge budget"));}if self.parameters+2>self.input.max_parameters{return Err(BooleanError::Invalid("Boolean exceeds parameter budget"));}if !slot(&mut self.source,work,alloc)?{return Ok(false);}work.vec_push(&mut self.source,Edge{a,b,operand:self.operand,bounds:bounds(a,b),parameters:NumericIndex::new()}).map_err(|(e,_)|BooleanError::Invalid(e))?;self.parameters+=2;Ok(true)}
 fn add_parameter(&mut self,index:usize,value:f64,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{let v=value.clamp(0.0,1.0);if v==0.0||v==1.0{return Ok(true);}let bits=v.to_bits();if self.parameter_insert.is_none()&&self.source[index].parameters.contains_key(&bits){return Ok(true);}if self.parameter_insert.is_none()&&self.parameters>=self.input.max_parameters{return Err(BooleanError::Invalid("Boolean exceeds parameter budget"));}if !insert_work(&mut self.parameter_insert,&mut self.source[index].parameters,bits,||(),work,&mut self.step_work,alloc)?{return Ok(false);}self.parameter_insert=None;self.parameters+=1;self.step_work=1;Ok(true)}
 fn queue_parameter(&mut self,index:usize,t:f64){let t=t.clamp(0.0,1.0);if t!=0.0&&t!=1.0{self.intersection_actions[self.intersection_len]=Some((index,t));self.intersection_len+=1;}}
 fn intersect(&mut self,left:usize,right:usize){
  self.intersection_actions=[None;4];self.intersection_at=0;self.intersection_len=0;let(a,b)=(&self.source[left],&self.source[right]);let(rx,ry,sx,sy,ox,oy)=(a.b[0]-a.a[0],a.b[1]-a.a[1],b.b[0]-b.a[0],b.b[1]-b.a[1],b.a[0]-a.a[0],b.a[1]-a.a[1]);let denominator=rx*sy-ry*sx;let(la,lb)=(length(a.a,a.b),length(b.a,b.b));let(ta,tb)=(self.input.epsilon/la,self.input.epsilon/lb);
  if denominator.abs()>f64::EPSILON*16.0*la*lb{let snap=if let Some(u)=endpoint(b,a.a,self.input.epsilon,lb){Some((0.0,u))}else if let Some(u)=endpoint(b,a.b,self.input.epsilon,lb){Some((1.0,u))}else if let Some(t)=endpoint(a,b.a,self.input.epsilon,la){Some((t,0.0))}else{endpoint(a,b.b,self.input.epsilon,la).map(|t|(t,1.0))};let(t,u)=snap.unwrap_or_else(||((ox*sy-oy*sx)/denominator,(ox*ry-oy*rx)/denominator));if t>=-ta&&t<=1.0+ta&&u>=-tb&&u<=1.0+tb{self.queue_parameter(left,t);self.queue_parameter(right,u);}return;}
  if(ox*ry-oy*rx).abs()>self.input.epsilon*la{return;}let params=[(left,parameter(a,b.a),ta),(left,parameter(a,b.b),ta),(right,parameter(b,a.a),tb),(right,parameter(b,a.b),tb)];for(index,t,tolerance)in params{if t>=-tolerance&&t<=1.0+tolerance{self.queue_parameter(index,t);}}
 }
 fn intern(&mut self,p:Vec2,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<Option<usize>,BooleanError>{
  if self.intern_current.is_none(){self.step_work=1;let epsilon=self.input.epsilon;let(x,y)=((p[0]/epsilon).floor()as i64,(p[1]/epsilon).floor()as i64);let mut found=None;for dy in -1..=1{for dx in -1..=1{if let Some(indices)=self.grid.get(&(x+dx,y+dy)){for index in indices{if length(self.nodes[*index],p)<=epsilon&&found.is_none_or(|old|*index<old){found=Some(*index);}}}}}if let Some(index)=found{return Ok(Some(index));}if self.nodes.len()>=self.input.max_atomic_edges*2{return Err(BooleanError::Invalid("Boolean exceeds vertex budget"));}self.intern_current=Some((p,(x,y),self.nodes.len()));self.intern_phase=0;return Ok(None);}
  let(p,key,index)=self.intern_current.ok_or(BooleanError::Invalid("intern_current absent"))?;if self.intern_phase==0{if(self.grid_insert.is_some()||!self.grid.contains_key(&key))&&!insert_work(&mut self.grid_insert,&mut self.grid,key,Vec::new,work,&mut self.step_work,alloc)?{return Ok(None);}self.grid_insert=None;self.intern_phase=1;self.step_work=1;return Ok(None);}let cell=self.grid.get_mut(&key).ok_or(BooleanError::Invalid("get_mut target absent"))?;let needed=cell.len()+1;if cell.len()>=16{return Err(BooleanError::Invalid("Boolean spatial precision exceeded"));}if !slot(&mut self.nodes,work,alloc)?||!reserve_for(cell,needed,1,work,alloc)?||!work.vec_slot(cell).map_err(BooleanError::Invalid)?{return Ok(None);}work.vec_push(&mut self.nodes,p).map_err(|(e,_)|BooleanError::Invalid(e))?;work.vec_push(cell,index).map_err(|(e,_)|BooleanError::Invalid(e))?;self.intern_current=None;self.intern_phase=0;self.step_work=1;Ok(Some(index))
 }
 fn prepare(&mut self,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{
  if self.operand==self.input.operands.len(){let magnitude=self.bounds.into_iter().map(f64::abs).fold(0.0f64,f64::max);if !self.source.is_empty()&&self.input.epsilon<magnitude*f64::EPSILON*16.0{return Err(BooleanError::Invalid("Boolean epsilon is below coordinate precision"));}self.phase=BooleanPhase::Indexing;return Ok(true);}let operand=&self.input.operands[self.operand];if self.entering{if operand.contours.len()>65536{return Err(BooleanError::Invalid("Invalid boolean operand"));}self.prepared+=1;self.entering=false;return Ok(true);}if self.contour==operand.contours.len(){self.operand+=1;self.contour=0;self.entering=true;return Ok(true);}let points=&operand.contours[self.contour];if points.len()>262144{return Err(BooleanError::Invalid("Invalid boolean contour"));}
  if self.at<points.len(){self.step_work=1;if self.vertices>=self.input.max_edges{return Err(BooleanError::Invalid("Boolean exceeds input vertex budget"));}let p=point(points[self.at])?;let result=if let Some(previous)=self.previous{self.add_edge(previous,p,work,alloc)}else{Ok(true)};if matches!(&result,Ok(false)){return Ok(false);}self.at+=1;self.vertices+=1;self.bounds=merge(self.bounds,[p[0],p[1],p[0],p[1]]);result?;if self.previous.is_none(){self.first=Some(p);}self.previous=Some(p);return Ok(true);}if let(Some(previous),Some(first))=(self.previous,self.first){if !self.add_edge(previous,first,work,alloc)?{return Ok(false);}}self.contour+=1;self.at=0;self.previous=None;self.first=None;Ok(true)
 }
 fn query_replace(&mut self,children:[Option<usize>;2],work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{let count=children.iter().flatten().count();let needed=self.query.len()-1+count;if !reserve_for(&mut self.query,needed,16,work,alloc)?{return Ok(false);}if count>0&&!work.copy_bytes(count*std::mem::size_of::<usize>()){return Ok(false);}self.query.pop();for child in children.into_iter().flatten(){self.query.push(child);}Ok(true)}
 fn indexing(&mut self,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{
  if self.source.is_empty(){self.phase=BooleanPhase::Complete;self.step_work=1;return Ok(false);}match self.index_mode{
   IndexMode::Push=>{if self.index_at<self.source.len(){self.step_work=1;let item=IndexItem{key:morton(&self.source[self.index_at],self.bounds),edge:self.index_at};if !self.index_heap.push_work(item,work,alloc).map_err(BooleanError::Invalid)?{return Ok(false);}self.index_at+=1;}else{self.index_mode=IndexMode::Leaves;}},
   IndexMode::Leaves=>{if self.index_heap.values.is_empty(){self.index_mode=IndexMode::Levels;self.index_at=0;}else{if !slot(&mut self.level,work,alloc)?||!slot(&mut self.tree,work,alloc)?{return Ok(false);}let HeapWorkPop::Ready(entry)=self.index_heap.pop_work(work,alloc).map_err(BooleanError::Invalid)?else{return Ok(false);};work.vec_push(&mut self.level,self.tree.len()).map_err(|(e,_)|BooleanError::Invalid(e))?;work.vec_push(&mut self.tree,Node{bounds:self.source[entry.edge].bounds,left:usize::MAX,right:usize::MAX,edge:Some(entry.edge),maximum:entry.edge}).map_err(|(e,_)|BooleanError::Invalid(e))?;}},
   IndexMode::Levels=>{if self.level.len()==1{if !slot(&mut self.query,work,alloc)?{return Ok(false);}self.root=self.level[0];work.vec_push(&mut self.query,self.root).map_err(|(e,_)|BooleanError::Invalid(e))?;self.phase=BooleanPhase::Intersections;return Ok(true);}if self.index_at==self.level.len(){if !slot(&mut self.retired_levels,work,alloc)?{return Ok(false);}let old=std::mem::replace(&mut self.level,std::mem::take(&mut self.next_level));if let Err((error,old))=work.vec_push(&mut self.retired_levels,old){self.next_level=std::mem::take(&mut self.level);self.level=old;return Err(BooleanError::Invalid(error));}self.index_at=0;return Ok(true);}let left=self.level[self.index_at];let right=self.level.get(self.index_at+1).copied();if !slot(&mut self.next_level,work,alloc)?{return Ok(false);}if let Some(right)=right{if !slot(&mut self.tree,work,alloc)?{return Ok(false);}let(a,b)=(self.tree[left],self.tree[right]);work.vec_push(&mut self.next_level,self.tree.len()).map_err(|(e,_)|BooleanError::Invalid(e))?;work.vec_push(&mut self.tree,Node{bounds:merge(a.bounds,b.bounds),left,right,edge:None,maximum:a.maximum.max(b.maximum)}).map_err(|(e,_)|BooleanError::Invalid(e))?;self.index_at+=2;}else{work.vec_push(&mut self.next_level,left).map_err(|(e,_)|BooleanError::Invalid(e))?;self.index_at+=1;}},
  }Ok(true)
 }
 fn intersections(&mut self,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{
  if self.intersection_active{if self.intersection_at<self.intersection_len{let(index,t)=self.intersection_actions[self.intersection_at].ok_or(BooleanError::Invalid("intersection_at] absent"))?;if !self.add_parameter(index,t,work,alloc)?{return Ok(false);}self.intersection_at+=1;self.step_work=1;return Ok(false);}self.intersection_active=false;return Ok(true);}if self.pivot==self.source.len(){self.phase=BooleanPhase::Splitting;return Ok(true);}let Some(index)=self.query.last().copied()else{if self.pivot+1<self.source.len(){if !slot(&mut self.query,work,alloc)?{return Ok(false);}work.vec_push(&mut self.query,self.root).map_err(|(e,_)|BooleanError::Invalid(e))?;}self.pivot+=1;return Ok(true);};self.step_work=1;let node=self.tree[index];if node.maximum<=self.pivot||!intersects(node.bounds,self.source[self.pivot].bounds,self.input.epsilon){self.query.pop();return Ok(true);}if let Some(edge)=node.edge{self.query.pop();self.pairs+=1;self.intersect(self.pivot,edge);if self.intersection_len>0{self.intersection_active=true;return Ok(false);}return Ok(true);}let children=[node.right,node.left].map(|child|{let n=self.tree[child];(n.maximum>self.pivot&&intersects(n.bounds,self.source[self.pivot].bounds,self.input.epsilon)).then_some(child)});self.query_replace(children,work,alloc)
 }
 fn splitting(&mut self,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{
  if let Some(mut pending)=self.pending_atomic{match pending.stage{
   0=>{if let Some(index)=self.intern(pending.p,work,alloc)?{pending.from=index;pending.stage=1;self.pending_atomic=Some(pending);}return Ok(false);},
   1=>{if let Some(index)=self.intern(pending.q,work,alloc)?{pending.to=index;pending.stage=2;self.pending_atomic=Some(pending);}return Ok(false);},
   2=>{let pair=(pending.from.min(pending.to),pending.from.max(pending.to));if pending.from==pending.to||self.atomic_ids.contains_key(&pair){self.pending_atomic=None;return Ok(true);}if self.atomic.len()>=self.input.max_atomic_edges{return Err(BooleanError::Invalid("Boolean exceeds atomic edge budget"));}pending.stage=3;self.pending_atomic=Some(pending);self.step_work=1;return Ok(false);},
   3=>{let pair=(pending.from.min(pending.to),pending.from.max(pending.to));if !insert_work(&mut self.atomic_insert,&mut self.atomic_ids,pair,||(),work,&mut self.step_work,alloc)?{return Ok(false);}self.atomic_insert=None;pending.stage=4;self.pending_atomic=Some(pending);self.step_work=1;return Ok(false);},
   _=>{if !slot(&mut self.atomic,work,alloc)?{return Ok(false);}work.vec_push(&mut self.atomic,(pending.from.min(pending.to),pending.from.max(pending.to))).map_err(|(e,_)|BooleanError::Invalid(e))?;self.pending_atomic=None;return Ok(true);},
  }}
  if self.split_at==self.source.len(){self.phase=BooleanPhase::Classifying;return Ok(true);}if !self.split_started{self.split_started=true;self.split_build=true;self.split_previous=None;self.split_seed=0;return Ok(true);}if self.split_build{
   if self.split_seed<2{if !self.split_heap.push_work(self.split_seed as f64,work,alloc).map_err(BooleanError::Invalid)?{return Ok(false);}self.split_seed+=1;return Ok(true);}
   if let Some(bits)=self.split_parameter{if !self.split_removed{if !remove_work(&mut self.parameter_remove,&mut self.source[self.split_at].parameters,bits,work,&mut self.step_work)?{return Ok(false);}self.parameter_remove=None;self.split_removed=true;self.step_work=1;return Ok(false);}if !self.split_heap.push_work(f64::from_bits(bits),work,alloc).map_err(BooleanError::Invalid)?{return Ok(false);}self.split_parameter=None;self.split_removed=false;return Ok(true);}
   if let Some((bits,_))=self.source[self.split_at].parameters.entry_at_rank(0){self.split_parameter=Some(*bits);self.split_removed=false;self.step_work=1;return Ok(false);}self.split_build=false;return Ok(true);
  }
  let popped=self.split_heap.pop_work(work,alloc).map_err(BooleanError::Invalid)?;let t=match popped{HeapWorkPop::Pending=>return Ok(false),HeapWorkPop::Empty=>{self.split_started=false;self.split_at+=1;return Ok(true);},HeapWorkPop::Ready(t)=>t};let previous=self.split_previous.replace(t);let Some(previous)=previous else{return Ok(true);};if previous==t{return Ok(true);}let(p,q)=(interpolate(&self.source[self.split_at],previous)?,interpolate(&self.source[self.split_at],t)?);if length(p,q)<=self.input.epsilon{return Ok(true);}self.pending_atomic=Some(AtomicPending{p,q,from:0,to:0,stage:0});self.step_work=1;Ok(false)
 }
 fn classify(&mut self,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{
  if self.classify_at==self.atomic.len(){self.phase=BooleanPhase::Contours;return Ok(true);}let(from,to)=self.atomic[self.classify_at];let(a,b)=(self.nodes[from],self.nodes[to]);match self.classify_mode{
   ClassifyMode::Start=>{if !slot(&mut self.query,work,alloc)?{return Ok(false);}self.midpoint=[(a[0]+b[0])/2.0,(a[1]+b[1])/2.0];let mut nearest=f64::INFINITY;for axis in 0..2{for direction in [-1.0,1.0]{let d=if direction>0.0{self.bounds[axis+2]-self.midpoint[axis]}else{self.midpoint[axis]-self.bounds[axis]};if d<nearest{nearest=d;self.classify_axis=axis;self.classify_direction=direction;}}}let len=length(a,b);self.nearest=4.0*(len*0.25).min((self.input.epsilon*2.0).max(len*1e-7));work.vec_push(&mut self.query,self.root).map_err(|(e,_)|BooleanError::Invalid(e))?;self.classify_mode=ClassifyMode::Nearest;},
   ClassifyMode::Nearest=>{if let Some(index)=self.query.last().copied(){self.step_work=1;let n=self.tree[index];if box_distance(n.bounds,self.midpoint)>self.nearest{self.query.pop();return Ok(true);}if let Some(edge)=n.edge{self.query.pop();let d=distance(&self.source[edge],self.midpoint)?;if d>self.input.epsilon{self.nearest=self.nearest.min(d);}return Ok(true);}let(l,r)=(box_distance(self.tree[n.left].bounds,self.midpoint),box_distance(self.tree[n.right].bounds,self.midpoint));let order=if l<=r{[n.right,n.left]}else{[n.left,n.right]};let children=order.map(|child|((if child==n.left{l}else{r})<=self.nearest).then_some(child));return self.query_replace(children,work,alloc);}
    self.step_work=1;let len=length(a,b);let offset=(len*0.25).min(self.nearest*0.25).min((self.input.epsilon*2.0).max(len*1e-7));let(nx,ny)=(-(b[1]-a[1])/len,(b[0]-a[0])/len);let left=[self.midpoint[0]+nx*offset,self.midpoint[1]+ny*offset];let right=[self.midpoint[0]-nx*offset,self.midpoint[1]-ny*offset];if length(left,right)==0.0{return Err(BooleanError::Invalid("Boolean probes exceed coordinate precision"));}let count=self.input.operands.len();if !slot(&mut self.query,work,alloc)?||!reserve_for(&mut self.left_winding,count,16,work,alloc)?||!reserve_for(&mut self.right_winding,count,16,work,alloc)?{return Ok(false);}let new_rows=count.saturating_sub(self.left_winding.len())+count.saturating_sub(self.right_winding.len());if new_rows>0&&!work.copy_bytes(new_rows*std::mem::size_of::<i32>()){return Ok(false);}self.left_winding.resize(count,0);self.right_winding.resize(count,0);self.left_winding.fill(0);self.right_winding.fill(0);self.left_point=left;self.right_point=right;work.vec_push(&mut self.query,self.root).map_err(|(e,_)|BooleanError::Invalid(e))?;self.classify_mode=ClassifyMode::Ray;
   },
   ClassifyMode::Ray=>{if let Some(index)=self.query.last().copied(){self.step_work=1;let n=self.tree[index];if !ray(n.bounds,self.left_point,self.classify_axis,self.classify_direction)&&!ray(n.bounds,self.right_point,self.classify_axis,self.classify_direction){self.query.pop();return Ok(true);}if let Some(edge)=n.edge{self.query.pop();let e=&self.source[edge];self.left_winding[e.operand]+=winding(e,self.left_point,self.classify_axis,self.classify_direction);self.right_winding[e.operand]+=winding(e,self.right_point,self.classify_axis,self.classify_direction);return Ok(true);}let children=[n.right,n.left].map(|child|{let b=self.tree[child].bounds;(ray(b,self.left_point,self.classify_axis,self.classify_direction)||ray(b,self.right_point,self.classify_axis,self.classify_direction)).then_some(child)});return self.query_replace(children,work,alloc);}self.fold_at=0;self.classify_mode=ClassifyMode::Fold;},
   ClassifyMode::Fold=>{if self.fold_at<self.input.operands.len(){let rule=self.input.operands[self.fold_at].fill_rule;let inside=|n:i32|if rule==BooleanFillRule::Evenodd{n%2!=0}else{n!=0};let(l,r)=(inside(self.left_winding[self.fold_at]),inside(self.right_winding[self.fold_at]));self.left_filled=if self.fold_at==0{l}else{self.input.operation.apply(self.left_filled,l)};self.right_filled=if self.fold_at==0{r}else{self.input.operation.apply(self.right_filled,r)};self.fold_at+=1;return Ok(true);}if self.left_filled!=self.right_filled{let edge=if self.left_filled{Boundary{from,to,used:false}}else{Boundary{from:to,to:from,used:false}};if(self.outgoing_insert.is_some()||!self.outgoing.contains_key(&edge.from))&&!insert_work(&mut self.outgoing_insert,&mut self.outgoing,edge.from,Vec::new,work,&mut self.step_work,alloc)?{return Ok(false);}let cell=self.outgoing.get_mut(&edge.from).ok_or(BooleanError::Invalid("get_mut target absent"))?;let needed=cell.len()+1;if !slot(&mut self.boundary,work,alloc)?||!reserve_for(cell,needed,1,work,alloc)?||!work.vec_slot(cell).map_err(BooleanError::Invalid)?{return Ok(false);}let index=self.boundary.len();work.vec_push(&mut self.boundary,edge).map_err(|(e,_)|BooleanError::Invalid(e))?;work.vec_push(cell,index).map_err(|(e,_)|BooleanError::Invalid(e))?;self.outgoing_insert=None;}self.classify_at+=1;self.classify_mode=ClassifyMode::Start;},
  }Ok(true)
 }
 fn contours(&mut self,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{
  if self.split_ring.is_some(){if self.ring_split_copy{if self.ring_split_at<self.ring.len(){let split=self.split_ring.as_mut().ok_or(BooleanError::Invalid("split_ring absent"))?;if !slot(split,work,alloc)?{return Ok(false);}work.vec_push(split,self.ring[self.ring_split_at]).map_err(|(e,_)|BooleanError::Invalid(e))?;self.ring_split_at+=1;return Ok(true);}self.ring_split_copy=false;return Ok(true);}if self.ring.len()>self.ring_split_stop{let key=*self.ring.last().ok_or(BooleanError::Invalid("last target absent"))?;if !remove_work(&mut self.position_remove,&mut self.positions,key,work,&mut self.step_work)?{return Ok(false);}self.position_remove=None;self.ring.pop();return Ok(true);}if self.split_ring.as_ref().ok_or(BooleanError::Invalid("split_ring absent"))?.len()<3{return Err(BooleanError::Invalid("Boolean split contour is degenerate"));}if !slot(&mut self.raw,work,alloc)?{return Ok(false);}move_option(&mut self.raw,&mut self.split_ring,work,alloc)?;return Ok(true);}
  if self.selecting{let choices=self.outgoing.get(&self.choice_node).map(Vec::as_slice).unwrap_or(&[]);if let Some(index)=choices.get(self.choice_at).copied(){self.choice_at+=1;let e=self.boundary[index];if !e.used{let(a,b)=(self.nodes[e.from],self.nodes[e.to]);let angle=(self.reverse_angle-(b[1]-a[1]).atan2(b[0]-a[0])+std::f64::consts::TAU)%std::f64::consts::TAU;if angle<self.best_angle{self.best=Some(index);self.best_angle=angle;}}return Ok(true);}self.current=Some(self.best.ok_or(BooleanError::Invalid("Boolean boundary is open"))?);self.selecting=false;return Ok(true);}
  let Some(current)=self.current else{if self.boundary_at==self.boundary.len(){self.phase=BooleanPhase::Compacting;return Ok(true);}let index=self.boundary_at;self.boundary_at+=1;if self.boundary[index].used{return Ok(true);}self.current=Some(index);self.start=self.boundary[index].from;assert!(self.ring.is_empty());self.positions.reset();return Ok(true);};let e=self.boundary[current];if e.used{return Err(BooleanError::Invalid("Boolean boundary is not manifold"));}if self.position_insert.is_none(){if let Some(repeated)=self.positions.get(&e.from).copied(){self.split_ring=Some(Vec::new());self.ring_split_at=repeated;self.ring_split_stop=repeated;self.ring_split_copy=true;return Ok(true);}}
  let length=self.ring.len();if !insert_work(&mut self.position_insert,&mut self.positions,e.from,||length,work,&mut self.step_work,alloc)?{return Ok(false);}if !slot(&mut self.ring,work,alloc)?||e.to==self.start&&!slot(&mut self.raw,work,alloc)?{return Ok(false);}work.vec_push(&mut self.ring,e.from).map_err(|(e,_)|BooleanError::Invalid(e))?;self.position_insert=None;self.boundary[current].used=true;if e.to==self.start{move_from(&mut self.raw,&mut self.ring,work,alloc)?;self.current=None;return Ok(true);}self.choice_node=e.to;self.choice_at=0;self.best=None;self.best_angle=f64::INFINITY;let(a,b)=(self.nodes[e.from],self.nodes[e.to]);self.reverse_angle=(a[1]-b[1]).atan2(a[0]-b[0]);self.selecting=true;Ok(true)
 }
 fn compact(&mut self,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{
  if self.compact_at==self.raw.len(){self.phase=BooleanPhase::Emitting;return Ok(true);}let raw=&self.raw[self.compact_at];if self.compact_mode==CompactMode::Scan{if self.compact_vertex<raw.len(){self.step_work=1;let at=self.compact_vertex;let(p,a,b)=(self.nodes[raw[at]],self.nodes[raw[(at+raw.len()-1)%raw.len()]],self.nodes[raw[(at+1)%raw.len()]]);if cross(a,p,b).abs()>self.input.epsilon*(length(a,p)+length(p,b)){if !slot(&mut self.compact_points,work,alloc)?{return Ok(false);}work.vec_push(&mut self.compact_points,p).map_err(|(e,_)|BooleanError::Invalid(e))?;}self.compact_vertex+=1;return Ok(true);}self.compact_mode=CompactMode::Measure;self.compact_vertex=0;self.area=0.0;self.lower=0;self.upper=0;return Ok(true);}
  let points=&self.compact_points;let at=self.compact_vertex;if at<points.len(){let(p,l,u)=(points[at],points[self.lower],points[self.upper]);if p[0]<l[0]||p[0]==l[0]&&p[1]<l[1]{self.lower=at;}if p[0]<u[0]||p[0]==u[0]&&p[1]>u[1]{self.upper=at;}if at>0&&at<points.len()-1{self.area+=cross(points[0],p,points[at+1])/2.0;}self.compact_vertex+=1;return Ok(true);}if points.len()>=3&&self.area.abs()>self.input.epsilon*self.input.epsilon{let anchor=if self.area>0.0{self.lower}else{self.upper};let(a,b)=(points[anchor],points[(anchor+1)%points.len()]);let angle=(b[1]-a[1]).atan2(b[0]-a[0]);let ring=self.rings.len();if !slot(&mut self.rings,work,alloc)?{return Ok(false);}if !self.ring_heap.push_work(RingItem{key:[a[0],a[1],angle,self.area],ring},work,alloc).map_err(BooleanError::Invalid)?{return Ok(false);}if let Err((error,ring))=work.vec_push(&mut self.rings,Ring{points:std::mem::take(&mut self.compact_points),anchor}){self.compact_points=ring.points;return Err(BooleanError::Invalid(error));}}else{self.compact_points.clear();}self.compact_at+=1;self.compact_vertex=0;self.compact_mode=CompactMode::Scan;Ok(true)
 }
 fn emit(&mut self,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{
  let Some(index)=self.emitting else{match self.ring_heap.pop_work(work,alloc).map_err(BooleanError::Invalid)?{HeapWorkPop::Pending=>return Ok(false),HeapWorkPop::Ready(item)=>{self.emitting=Some(item.ring);self.emit_at=0;return Ok(true);},HeapWorkPop::Empty=>{self.phase=BooleanPhase::Complete;self.step_work=1;return Ok(false);}}};if self.output.len()>=self.input.max_segments{return Err(BooleanError::Invalid("Boolean exceeds output segment budget"));}if !slot(&mut self.output,work,alloc)?{return Ok(false);}let ring=&self.rings[index];if self.emit_at==ring.points.len(){work.vec_push(&mut self.output,PathSegment::Close).map_err(|(e,_)|BooleanError::Invalid(e))?;self.emitting=None;return Ok(true);}let p=ring.points[(ring.anchor+self.emit_at)%ring.points.len()];work.vec_push(&mut self.output,if self.emit_at==0{PathSegment::Move{to:p}}else{PathSegment::Line{to:p}}).map_err(|(e,_)|BooleanError::Invalid(e))?;self.emit_at+=1;Ok(true)
 }
 fn step(&mut self,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,BooleanError>{match self.phase{BooleanPhase::Preparing=>self.prepare(work,alloc),BooleanPhase::Indexing=>self.indexing(work,alloc),BooleanPhase::Intersections=>self.intersections(work,alloc),BooleanPhase::Splitting=>self.splitting(work,alloc),BooleanPhase::Classifying=>self.classify(work,alloc),BooleanPhase::Contours=>self.contours(work,alloc),BooleanPhase::Compacting=>self.compact(work,alloc),BooleanPhase::Emitting=>self.emit(work,alloc),BooleanPhase::Complete=>Ok(false)}}
 /// 📊️ Reads the actual arrangement frontier without advancing any owner.
 pub fn progress(&self)->BooleanProgress{BooleanProgress{phase:self.phase,operands:self.prepared,vertices:self.vertices,edges:self.source.len(),parameters:self.parameters,pairs:self.pairs,atomic_edges:self.atomic.len(),boundary_edges:self.boundary.len(),contours:self.rings.len(),segments:self.output.len(),work:self.work,done:self.phase==BooleanPhase::Complete}}
 pub fn next_work_demand(&self)->RetirementDemand{self.pending}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 pub fn settlement_pending(&self)->bool{self.phase==BooleanPhase::Complete||self.phase==BooleanPhase::Indexing&&self.source.is_empty()||self.phase==BooleanPhase::Emitting&&self.emitting.is_none()&&self.ring_heap.values.is_empty()}
 /// 🎟️ Real retained preparation and original semantic steps spend the same finite caller ledger.
 pub fn advance(&mut self,grant:usize,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<BooleanProgress,BooleanError>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if grant==0||grant as u128>9_007_199_254_740_991{return Err(BooleanError::Invalid("Boolean work grant must be a positive integer"));}if self.cancelled{return Err(BooleanError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}if !work.admits_absolute(self.pending){work.require_absolute(self.pending);return Ok(self.progress());}let Some(root)=work.enter()else{self.pending=work.next_demand();return Ok(self.progress());};let result=(||{for _ in 0..grant{if self.phase==BooleanPhase::Complete||!work.can_step(){break;}if self.work>=self.input.max_work&&!self.settlement_pending(){let error=BooleanError::Invalid("Boolean exceeds work budget");self.failure=Some(error.clone());work.note_source(root);self.last_turn_work+=1;return Err(error);}self.step_work=0;let before=work.receipt();let result=self.step(work,alloc);let semantic=matches!(&result,Ok(true)|Err(_));if semantic||self.step_work>0||work.receipt()!=before{work.note_source(root);self.last_turn_work+=self.step_work.max(1);}if semantic{self.work+=1;self.last_turn_semantic_work+=1;}self.pending=work.next_demand();if let Err(error)=result{self.failure=Some(error.clone());return Err(error);}if work.blocked(){break;}}Ok(self.progress())})();work.exit();result
 }
 pub fn result(&self)->Result<&[PathSegment],BooleanError> {if self.cancelled {return Err(BooleanError::Cancelled);}if let Some(error)=&self.failure {return Err(error.clone());}if self.phase!=BooleanPhase::Complete {return Err(BooleanError::Incomplete);}Ok(&self.output)}
 /// 🧹️ Move source operands and valid completed paths without destroying private arrangement owners.
 pub fn into_retirement(mut self)->(BooleanRetirement,Vec<BooleanOperand>,Option<Vec<PathSegment>>){
  let operands=std::mem::take(&mut self.input.operands);
  let output=if self.phase==BooleanPhase::Complete&&!self.cancelled&&self.failure.is_none(){Some(std::mem::take(&mut self.output))}else{None};
  self.cancelled=true;self.emitting=None;
  (BooleanRetirement::new(self),operands,output)
 }
 pub fn cancel(&mut self) {self.cancelled=true;}
}
semio_framework_2d_retire::physical_work_retirement!(BooleanRetirement,BooleanJob);
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod bounded_tests;

semio_framework_value::artifact_retire_leaf!(BooleanOperation,BooleanFillRule,BooleanPhase,BooleanProgress,Node,Boundary,IndexItem,RingItem,IndexMode,ClassifyMode,CompactMode);
impl<T:semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetireOwned for Heap<T>{type RetirementCursor=<Vec<T> as semio_framework_value::retirement::RetireOwned>::RetirementCursor;fn retirement_seed(self)->Self::RetirementCursor{semio_framework_value::retirement::RetireOwned::retirement_seed(self.values)}fn retirement_birth_bytes(&self)->Option<usize>{self.values.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}}

#[cfg(test)]
use semio_framework_value_testing_allocator as _;
#[cfg(test)]
#[path="🎟️work/🧪️tests/🦀️.rs"]
mod work_tests;

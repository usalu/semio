//! 🔀️ Filled planar arrangements with spatial queries, bounded grants and private outputs.
use crate::engine::{DrawingError,PathSegment,Vec2};
use crate::retirement::{WorkRetirementCounter,WorkRetirementProgress};
pub type BooleanRetirementProgress=WorkRetirementProgress;
use std::cmp::Ordering;
use std::collections::{BTreeMap,BTreeSet};
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
pub struct BooleanOperand {pub contours:Vec<Vec<Vec2>>,pub fill_rule:BooleanFillRule}
#[derive(Clone,Debug)]
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
pub enum BooleanError {Invalid(&'static str),Incomplete,Cancelled}
impl std::fmt::Display for BooleanError {
 fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {f.write_str(match self {Self::Invalid(message)=>message,Self::Incomplete=>"Boolean incomplete",Self::Cancelled=>"Boolean cancelled"})}
}
impl std::error::Error for BooleanError {}
#[derive(Clone)]
struct Edge {a:Vec2,b:Vec2,operand:usize,bounds:Box2,parameters:BTreeSet<u64>}
#[derive(Clone,Copy)]
struct Node {bounds:Box2,left:usize,right:usize,edge:Option<usize>,maximum:usize}
#[derive(Clone,Copy)]
struct Boundary {from:usize,to:usize,used:bool}
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
 fn push(&mut self,value:T) {self.values.push(value);let mut at=self.values.len()-1;while at>0 {let parent=(at-1)/2;if (self.compare)(&self.values[parent],&self.values[at])!=Ordering::Greater {break;}self.values.swap(parent,at);at=parent;}}
 fn pop(&mut self)->Option<T> {if self.values.is_empty() {return None;}let value=self.values.swap_remove(0);let mut at=0;while at*2+1<self.values.len() {let mut child=at*2+1;if child+1<self.values.len()&&(self.compare)(&self.values[child+1],&self.values[child])==Ordering::Less {child+=1;}if (self.compare)(&self.values[at],&self.values[child])!=Ordering::Greater {break;}self.values.swap(at,child);at=child;}Some(value)}
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
fn ray(b:Box2,p:Vec2)->bool {b[2]>=p[0]&&b[1]<=p[1]&&b[3]>p[1]}
fn interpolate(e:&Edge,t:f64)->Result<Vec2,BooleanError> {if t==0.0 {return Ok(e.a);}if t==1.0 {return Ok(e.b);}point([e.a[0]+(e.b[0]-e.a[0])*t,e.a[1]+(e.b[1]-e.a[1])*t])}
fn parameter(e:&Edge,p:Vec2)->f64 {let (dx,dy)=(e.b[0]-e.a[0],e.b[1]-e.a[1]);if dx.abs()>=dy.abs() {(p[0]-e.a[0])/dx} else {(p[1]-e.a[1])/dy}}
fn distance(e:&Edge,p:Vec2)->Result<f64,BooleanError> {let (dx,dy)=(e.b[0]-e.a[0],e.b[1]-e.a[1]);let len=dx.hypot(dy);let t=(((p[0]-e.a[0])*(dx/len)+(p[1]-e.a[1])*(dy/len))/len).clamp(0.0,1.0);Ok(length(p,interpolate(e,t)?))}
fn winding(e:&Edge,p:Vec2)->i32 {let c=cross(e.a,e.b,p);if e.a[1]<=p[1]&&e.b[1]>p[1]&&c>0.0 {1} else if e.a[1]>p[1]&&e.b[1]<=p[1]&&c<0.0 {-1} else {0}}
fn spread(mut n:u32)->u32 {n=(n|(n<<8))&0x00ff00ff;n=(n|(n<<4))&0x0f0f0f0f;n=(n|(n<<2))&0x33333333;(n|(n<<1))&0x55555555}
fn morton(e:&Edge,b:Box2)->u32 {
 let x=if b[2]==b[0] {0} else {(((e.a[0]+e.b[0])/2.0-b[0])/(b[2]-b[0])*65535.0).floor().clamp(0.0,65535.0) as u32};
 let y=if b[3]==b[1] {0} else {(((e.a[1]+e.b[1])/2.0-b[1])/(b[3]-b[1])*65535.0).floor().clamp(0.0,65535.0) as u32};
 spread(x)|(spread(y)<<1)
}
/// ⏱️ Prepare, intersect, classify and emit owned regions under explicit work grants.
pub struct BooleanJob {
 input:BooleanInput,phase:BooleanPhase,work:u64,cancelled:bool,failure:Option<BooleanError>,prepared:usize,vertices:usize,parameters:usize,pairs:u64,
 operand:usize,contour:usize,at:usize,entering:bool,first:Option<Vec2>,previous:Option<Vec2>,bounds:Box2,rules:Vec<BooleanFillRule>,source:Vec<Edge>,
 index_mode:IndexMode,index_at:usize,index_heap:Heap<IndexItem>,tree:Vec<Node>,level:Vec<usize>,next_level:Vec<usize>,root:usize,query:Vec<usize>,pivot:usize,
 split_at:usize,split_started:bool,split_heap:Heap<f64>,split_build:bool,split_previous:Option<f64>,nodes:Vec<Vec2>,grid:BTreeMap<(i64,i64),Vec<usize>>,atomic:Vec<(usize,usize)>,atomic_ids:BTreeSet<(usize,usize)>,
 classify_at:usize,classify_mode:ClassifyMode,midpoint:Vec2,left_point:Vec2,right_point:Vec2,nearest:f64,left_winding:Vec<i32>,right_winding:Vec<i32>,fold_at:usize,left_filled:bool,right_filled:bool,
 boundary:Vec<Boundary>,outgoing:BTreeMap<usize,Vec<usize>>,boundary_at:usize,current:Option<usize>,start:usize,raw:Vec<Vec<usize>>,ring:Vec<usize>,positions:BTreeMap<usize,usize>,selecting:bool,choice_at:usize,choice_node:usize,reverse_angle:f64,best:Option<usize>,best_angle:f64,
 split_ring:Option<Vec<usize>>,ring_split_at:usize,ring_split_stop:usize,ring_split_copy:bool,
 compact_at:usize,compact_vertex:usize,compact_mode:CompactMode,compact_points:Vec<Vec2>,area:f64,lower:usize,upper:usize,rings:Vec<Ring>,ring_heap:Heap<RingItem>,emitting:Option<usize>,emit_at:usize,output:Vec<PathSegment>,
}
impl BooleanJob {
 pub fn new(input:BooleanInput)->Result<Self,BooleanError> {
  if input.operands.is_empty()||input.operands.len()>1024||!input.epsilon.is_finite()||!(1e-12..=16.0).contains(&input.epsilon)||!(1..=65536).contains(&input.max_edges)||!(1..=262144).contains(&input.max_parameters)||!(1..=65536).contains(&input.max_atomic_edges)||!(1..=65536).contains(&input.max_segments)||!(1..=1000000000).contains(&input.max_work) {return Err(BooleanError::Invalid("Invalid boolean contract"));}
  Ok(Self {input,phase:BooleanPhase::Preparing,work:0,cancelled:false,failure:None,prepared:0,vertices:0,parameters:0,pairs:0,operand:0,contour:0,at:0,entering:true,first:None,previous:None,bounds:[f64::INFINITY,f64::INFINITY,f64::NEG_INFINITY,f64::NEG_INFINITY],rules:Vec::new(),source:Vec::new(),index_mode:IndexMode::Push,index_at:0,index_heap:Heap::new(index_order),tree:Vec::new(),level:Vec::new(),next_level:Vec::new(),root:usize::MAX,query:Vec::new(),pivot:0,split_at:0,split_started:false,split_heap:Heap::new(f64::total_cmp),split_build:true,split_previous:None,nodes:Vec::new(),grid:BTreeMap::new(),atomic:Vec::new(),atomic_ids:BTreeSet::new(),classify_at:0,classify_mode:ClassifyMode::Start,midpoint:[0.0;2],left_point:[0.0;2],right_point:[0.0;2],nearest:f64::INFINITY,left_winding:Vec::new(),right_winding:Vec::new(),fold_at:0,left_filled:false,right_filled:false,boundary:Vec::new(),outgoing:BTreeMap::new(),boundary_at:0,current:None,start:0,raw:Vec::new(),ring:Vec::new(),positions:BTreeMap::new(),selecting:false,choice_at:0,choice_node:0,reverse_angle:0.0,best:None,best_angle:f64::INFINITY,split_ring:None,ring_split_at:0,ring_split_stop:0,ring_split_copy:true,compact_at:0,compact_vertex:0,compact_mode:CompactMode::Scan,compact_points:Vec::new(),area:0.0,lower:0,upper:0,rings:Vec::new(),ring_heap:Heap::new(ring_order),emitting:None,emit_at:0,output:Vec::new()})
 }
 fn add_edge(&mut self,a:Vec2,b:Vec2)->Result<(),BooleanError> {
  if length(a,b)==0.0 {return Ok(());}if self.source.len()>=self.input.max_edges {return Err(BooleanError::Invalid("Boolean exceeds edge budget"));}if self.parameters+2>self.input.max_parameters {return Err(BooleanError::Invalid("Boolean exceeds parameter budget"));}
  self.source.push(Edge {a,b,operand:self.operand,bounds:bounds(a,b),parameters:BTreeSet::from([0.0f64.to_bits(),1.0f64.to_bits()])});self.parameters+=2;Ok(())
 }
 fn add_parameter(&mut self,index:usize,value:f64)->Result<(),BooleanError> {
  let v=value.clamp(0.0,1.0);let bits=(if v==0.0 {0.0} else {v}).to_bits();
  if !self.source[index].parameters.contains(&bits) {if self.parameters>=self.input.max_parameters {return Err(BooleanError::Invalid("Boolean exceeds parameter budget"));}self.source[index].parameters.insert(bits);self.parameters+=1;}Ok(())
 }
 fn intersect(&mut self,left:usize,right:usize)->Result<(),BooleanError> {
  let (a,b)=(&self.source[left],&self.source[right]);let (rx,ry,sx,sy,ox,oy)=(a.b[0]-a.a[0],a.b[1]-a.a[1],b.b[0]-b.a[0],b.b[1]-b.a[1],b.a[0]-a.a[0],b.a[1]-a.a[1]);let denominator=rx*sy-ry*sx;let (la,lb)=(length(a.a,a.b),length(b.a,b.b));let (ta,tb)=(self.input.epsilon/la,self.input.epsilon/lb);
  if denominator.abs()>f64::EPSILON*16.0*la*lb {let (t,u)=((ox*sy-oy*sx)/denominator,(ox*ry-oy*rx)/denominator);if t>=-ta&&t<=1.0+ta&&u>=-tb&&u<=1.0+tb {self.add_parameter(left,t)?;self.add_parameter(right,u)?;}return Ok(());}
  if (ox*ry-oy*rx).abs()>self.input.epsilon*la {return Ok(());}let params=[(left,parameter(a,b.a),ta),(left,parameter(a,b.b),ta),(right,parameter(b,a.a),tb),(right,parameter(b,a.b),tb)];
  for (index,t,tolerance) in params {if t>=-tolerance&&t<=1.0+tolerance {self.add_parameter(index,t)?;}}Ok(())
 }
 fn intern(&mut self,p:Vec2)->Result<usize,BooleanError> {
  let epsilon=self.input.epsilon;let (x,y)=((p[0]/epsilon).floor() as i64,(p[1]/epsilon).floor() as i64);let mut found=None;
  for dy in -1..=1 {for dx in -1..=1 {if let Some(indices)=self.grid.get(&(x+dx,y+dy)) {for index in indices {if length(self.nodes[*index],p)<=epsilon&&found.is_none_or(|existing|*index<existing) {found=Some(*index);}}}}}
  if let Some(index)=found {return Ok(index);}if self.nodes.len()>=self.input.max_atomic_edges*2 {return Err(BooleanError::Invalid("Boolean exceeds vertex budget"));}
  let index=self.nodes.len();self.nodes.push(p);let cell=self.grid.entry((x,y)).or_default();if cell.len()>=16 {return Err(BooleanError::Invalid("Boolean spatial precision exceeded"));}cell.push(index);Ok(index)
 }
 fn prepare(&mut self)->Result<(),BooleanError> {
  if self.operand==self.input.operands.len() {let magnitude=self.bounds.into_iter().map(f64::abs).fold(0.0f64,f64::max);if !self.source.is_empty()&&self.input.epsilon<magnitude*f64::EPSILON*16.0 {return Err(BooleanError::Invalid("Boolean epsilon is below coordinate precision"));}self.phase=BooleanPhase::Indexing;return Ok(());}
  let operand=&self.input.operands[self.operand];
  if self.entering {if operand.contours.len()>4096 {return Err(BooleanError::Invalid("Invalid boolean operand"));}self.rules.push(operand.fill_rule);self.prepared+=1;self.entering=false;return Ok(());}
  if self.contour==operand.contours.len() {self.operand+=1;self.contour=0;self.entering=true;return Ok(());}let points=&operand.contours[self.contour];
  if points.len()>65536 {return Err(BooleanError::Invalid("Invalid boolean contour"));}if self.at<points.len() {if self.vertices>=self.input.max_edges {return Err(BooleanError::Invalid("Boolean exceeds input vertex budget"));}let p=point(points[self.at])?;self.at+=1;self.vertices+=1;self.bounds=merge(self.bounds,[p[0],p[1],p[0],p[1]]);if let Some(previous)=self.previous {self.add_edge(previous,p)?;}else {self.first=Some(p);}self.previous=Some(p);return Ok(());}
  if let (Some(previous),Some(first))=(self.previous,self.first) {self.add_edge(previous,first)?;}self.contour+=1;self.at=0;self.previous=None;self.first=None;Ok(())
 }
 fn indexing(&mut self)->Result<(),BooleanError> {
  if self.source.is_empty() {self.phase=BooleanPhase::Complete;return Ok(());}
  match self.index_mode {
   IndexMode::Push=>{if self.index_at<self.source.len() {self.index_heap.push(IndexItem {key:morton(&self.source[self.index_at],self.bounds),edge:self.index_at});self.index_at+=1;}else {self.index_mode=IndexMode::Leaves;}}
   IndexMode::Leaves=>{if let Some(entry)=self.index_heap.pop() {self.level.push(self.tree.len());self.tree.push(Node {bounds:self.source[entry.edge].bounds,left:usize::MAX,right:usize::MAX,edge:Some(entry.edge),maximum:entry.edge});}else {self.index_mode=IndexMode::Levels;self.index_at=0;}}
   IndexMode::Levels=>{
    if self.level.len()==1 {self.root=self.level[0];self.query.push(self.root);self.phase=BooleanPhase::Intersections;return Ok(());}if self.index_at==self.level.len() {self.level=std::mem::take(&mut self.next_level);self.index_at=0;return Ok(());}
    let left=self.level[self.index_at];self.index_at+=1;let Some(right)=self.level.get(self.index_at).copied() else {self.next_level.push(left);return Ok(());};self.index_at+=1;
    let (a,b)=(self.tree[left],self.tree[right]);self.next_level.push(self.tree.len());self.tree.push(Node {bounds:merge(a.bounds,b.bounds),left,right,edge:None,maximum:a.maximum.max(b.maximum)});
   }
  }Ok(())
 }
 fn intersections(&mut self)->Result<(),BooleanError> {
  if self.pivot==self.source.len() {self.phase=BooleanPhase::Splitting;return Ok(());}let Some(index)=self.query.pop() else {self.pivot+=1;if self.pivot<self.source.len() {self.query.push(self.root);}return Ok(());};let node=self.tree[index];
  if node.maximum<=self.pivot||!intersects(node.bounds,self.source[self.pivot].bounds,self.input.epsilon) {return Ok(());}if let Some(edge)=node.edge {self.pairs+=1;self.intersect(self.pivot,edge)?;}else {self.query.extend([node.right,node.left]);}Ok(())
 }
 fn splitting(&mut self)->Result<(),BooleanError> {
  if self.split_at==self.source.len() {self.phase=BooleanPhase::Classifying;return Ok(());}if !self.split_started {self.split_started=true;self.split_build=true;self.split_previous=None;return Ok(());}
  if self.split_build {if let Some(bits)=self.source[self.split_at].parameters.pop_first() {self.split_heap.push(f64::from_bits(bits));}else {self.split_build=false;}return Ok(());}
  let Some(t)=self.split_heap.pop() else {self.split_started=false;self.split_at+=1;return Ok(());};let previous=self.split_previous.replace(t);let Some(previous)=previous else {return Ok(());};if previous==t {return Ok(());}
  let (p,q)=(interpolate(&self.source[self.split_at],previous)?,interpolate(&self.source[self.split_at],t)?);if length(p,q)<=self.input.epsilon {return Ok(());}let (from,to)=(self.intern(p)?,self.intern(q)?);if from==to {return Ok(());}let pair=(from.min(to),from.max(to));
  if !self.atomic_ids.contains(&pair) {if self.atomic.len()>=self.input.max_atomic_edges {return Err(BooleanError::Invalid("Boolean exceeds atomic edge budget"));}self.atomic_ids.insert(pair);self.atomic.push(pair);}Ok(())
 }
 fn classify(&mut self)->Result<(),BooleanError> {
  if self.classify_at==self.atomic.len() {self.query=Vec::new();self.phase=BooleanPhase::Contours;return Ok(());}let (from,to)=self.atomic[self.classify_at];let (a,b)=(self.nodes[from],self.nodes[to]);
  match self.classify_mode {
   ClassifyMode::Start=>{self.midpoint=[(a[0]+b[0])/2.0,(a[1]+b[1])/2.0];self.nearest=f64::INFINITY;self.query.push(self.root);self.classify_mode=ClassifyMode::Nearest;}
   ClassifyMode::Nearest=>{
    if let Some(index)=self.query.pop() {let n=self.tree[index];if box_distance(n.bounds,self.midpoint)>self.nearest {return Ok(());}if let Some(edge)=n.edge {let d=distance(&self.source[edge],self.midpoint)?;if d>self.input.epsilon {self.nearest=self.nearest.min(d);}}else {let (l,r)=(self.tree[n.left],self.tree[n.right]);if box_distance(l.bounds,self.midpoint)<=box_distance(r.bounds,self.midpoint) {self.query.extend([n.right,n.left]);}else {self.query.extend([n.left,n.right]);}}return Ok(());}
    let len=length(a,b);let offset=(len*0.25).min(self.nearest*0.25).min((self.input.epsilon*2.0).max(len*1e-7));let (nx,ny)=(-(b[1]-a[1])/len,(b[0]-a[0])/len);
    self.left_point=[self.midpoint[0]+nx*offset,self.midpoint[1]+ny*offset];self.right_point=[self.midpoint[0]-nx*offset,self.midpoint[1]-ny*offset];if length(self.left_point,self.right_point)==0.0 {return Err(BooleanError::Invalid("Boolean probes exceed coordinate precision"));}
    self.left_winding=vec![0;self.rules.len()];self.right_winding=vec![0;self.rules.len()];self.query.push(self.root);self.classify_mode=ClassifyMode::Ray;
   }
   ClassifyMode::Ray=>{
    if let Some(index)=self.query.pop() {let n=self.tree[index];if !ray(n.bounds,self.left_point)&&!ray(n.bounds,self.right_point) {return Ok(());}if let Some(edge)=n.edge {let e=&self.source[edge];self.left_winding[e.operand]+=winding(e,self.left_point);self.right_winding[e.operand]+=winding(e,self.right_point);}else {self.query.extend([n.right,n.left]);}return Ok(());}self.fold_at=0;self.classify_mode=ClassifyMode::Fold;
   }
   ClassifyMode::Fold=>{
    if self.fold_at<self.rules.len() {let rule=self.rules[self.fold_at];let inside=|n:i32|if rule==BooleanFillRule::Evenodd {n%2!=0} else {n!=0};let (l,r)=(inside(self.left_winding[self.fold_at]),inside(self.right_winding[self.fold_at]));
     self.left_filled=if self.fold_at==0 {l} else {self.input.operation.apply(self.left_filled,l)};self.right_filled=if self.fold_at==0 {r} else {self.input.operation.apply(self.right_filled,r)};self.fold_at+=1;return Ok(());
    }
    if self.left_filled!=self.right_filled {let edge=if self.left_filled {Boundary {from,to,used:false}} else {Boundary {from:to,to:from,used:false}};let index=self.boundary.len();self.boundary.push(edge);self.outgoing.entry(edge.from).or_default().push(index);}
    self.classify_at+=1;self.classify_mode=ClassifyMode::Start;
   }
  }Ok(())
 }
 fn contours(&mut self)->Result<(),BooleanError> {
  if let Some(split)=self.split_ring.as_mut() {if self.ring_split_copy {if self.ring_split_at<self.ring.len() {split.push(self.ring[self.ring_split_at]);self.ring_split_at+=1;return Ok(());}self.ring_split_copy=false;return Ok(());}if self.ring.len()>self.ring_split_stop {self.positions.remove(&self.ring.pop().unwrap());return Ok(());}if split.len()<3 {return Err(BooleanError::Invalid("Boolean split contour is degenerate"));}self.raw.push(self.split_ring.take().unwrap());return Ok(());}
  if self.selecting {let choices=self.outgoing.get(&self.choice_node).map(Vec::as_slice).unwrap_or(&[]);if let Some(index)=choices.get(self.choice_at).copied() {self.choice_at+=1;let e=self.boundary[index];if !e.used {let (a,b)=(self.nodes[e.from],self.nodes[e.to]);let angle=(self.reverse_angle-(b[1]-a[1]).atan2(b[0]-a[0])+std::f64::consts::TAU)%std::f64::consts::TAU;if angle<self.best_angle {self.best=Some(index);self.best_angle=angle;}}return Ok(());}self.current=Some(self.best.ok_or(BooleanError::Invalid("Boolean boundary is open"))?);self.selecting=false;return Ok(());}
  let Some(current)=self.current else {if self.boundary_at==self.boundary.len() {self.phase=BooleanPhase::Compacting;return Ok(());}let index=self.boundary_at;self.boundary_at+=1;if self.boundary[index].used {return Ok(());}self.current=Some(index);self.start=self.boundary[index].from;self.ring=Vec::new();self.positions=BTreeMap::new();return Ok(());};
  let e=self.boundary[current];if e.used {return Err(BooleanError::Invalid("Boolean boundary is not manifold"));}if let Some(repeated)=self.positions.get(&e.from).copied() {self.split_ring=Some(Vec::new());self.ring_split_at=repeated;self.ring_split_stop=repeated;self.ring_split_copy=true;return Ok(());}
  self.positions.insert(e.from,self.ring.len());self.ring.push(e.from);self.boundary[current].used=true;if e.to==self.start {self.raw.push(std::mem::take(&mut self.ring));self.current=None;return Ok(());}
  self.choice_node=e.to;self.choice_at=0;self.best=None;self.best_angle=f64::INFINITY;let (a,b)=(self.nodes[e.from],self.nodes[e.to]);self.reverse_angle=(a[1]-b[1]).atan2(a[0]-b[0]);self.selecting=true;Ok(())
 }
 fn compact(&mut self)->Result<(),BooleanError> {
  if self.compact_at==self.raw.len() {self.phase=BooleanPhase::Emitting;return Ok(());}let raw=&self.raw[self.compact_at];
  if self.compact_mode==CompactMode::Scan {if self.compact_vertex<raw.len() {let at=self.compact_vertex;self.compact_vertex+=1;let (p,a,b)=(self.nodes[raw[at]],self.nodes[raw[(at+raw.len()-1)%raw.len()]],self.nodes[raw[(at+1)%raw.len()]]);
    if cross(a,p,b).abs()>self.input.epsilon*(length(a,p)+length(p,b)) {self.compact_points.push(p);}return Ok(());}self.compact_mode=CompactMode::Measure;self.compact_vertex=0;self.area=0.0;self.lower=0;self.upper=0;return Ok(());
  }
  let points=&self.compact_points;let at=self.compact_vertex;self.compact_vertex+=1;if at<points.len() {let (p,l,u)=(points[at],points[self.lower],points[self.upper]);if p[0]<l[0]||p[0]==l[0]&&p[1]<l[1] {self.lower=at;}if p[0]<u[0]||p[0]==u[0]&&p[1]>u[1] {self.upper=at;}if at>0&&at<points.len()-1 {self.area+=cross(points[0],p,points[at+1])/2.0;}return Ok(());}
  if points.len()>=3&&self.area.abs()>self.input.epsilon*self.input.epsilon {let anchor=if self.area>0.0 {self.lower} else {self.upper};let (a,b)=(points[anchor],points[(anchor+1)%points.len()]);let angle=(b[1]-a[1]).atan2(b[0]-a[0]);let ring=self.rings.len();self.rings.push(Ring {points:std::mem::take(&mut self.compact_points),anchor});self.ring_heap.push(RingItem {key:[a[0],a[1],angle,self.area],ring});}
  self.compact_at+=1;self.compact_vertex=0;self.compact_points=Vec::new();self.compact_mode=CompactMode::Scan;Ok(())
 }
 fn emit(&mut self)->Result<(),BooleanError> {
  let Some(index)=self.emitting else {if let Some(item)=self.ring_heap.pop() {self.emitting=Some(item.ring);self.emit_at=0;}else {self.phase=BooleanPhase::Complete;}return Ok(());};
  if self.output.len()>=self.input.max_segments {return Err(BooleanError::Invalid("Boolean exceeds output segment budget"));}let ring=&self.rings[index];
  if self.emit_at==ring.points.len() {self.output.push(PathSegment::Close);self.emitting=None;return Ok(());}let p=ring.points[(ring.anchor+self.emit_at)%ring.points.len()];self.output.push(if self.emit_at==0 {PathSegment::Move {to:p}} else {PathSegment::Line {to:p}});self.emit_at+=1;Ok(())
 }
 fn step(&mut self)->Result<(),BooleanError> {match self.phase {BooleanPhase::Preparing=>self.prepare(),BooleanPhase::Indexing=>self.indexing(),BooleanPhase::Intersections=>self.intersections(),BooleanPhase::Splitting=>self.splitting(),BooleanPhase::Classifying=>self.classify(),BooleanPhase::Contours=>self.contours(),BooleanPhase::Compacting=>self.compact(),BooleanPhase::Emitting=>self.emit(),BooleanPhase::Complete=>Ok(())}}
 pub fn advance(&mut self,grant:usize)->Result<BooleanProgress,BooleanError> {
  if grant==0||grant as u128>9_007_199_254_740_991 {return Err(BooleanError::Invalid("Boolean work grant must be a positive integer"));}if self.cancelled {return Err(BooleanError::Cancelled);}if let Some(error)=&self.failure {return Err(error.clone());}
  for _ in 0..grant {if self.phase==BooleanPhase::Complete {break;}let result=if self.work>=self.input.max_work {Err(BooleanError::Invalid("Boolean exceeds work budget"))} else {self.step()};if let Err(error)=result {self.failure=Some(error.clone());return Err(error);}self.work+=1;}
  Ok(BooleanProgress {phase:self.phase,operands:self.prepared,vertices:self.vertices,edges:self.source.len(),parameters:self.parameters,pairs:self.pairs,atomic_edges:self.atomic.len(),boundary_edges:self.boundary.len(),contours:self.rings.len(),segments:self.output.len(),work:self.work,done:self.phase==BooleanPhase::Complete})
 }
 pub fn result(&self)->Result<&[PathSegment],BooleanError> {if self.cancelled {return Err(BooleanError::Cancelled);}if let Some(error)=&self.failure {return Err(error.clone());}if self.phase!=BooleanPhase::Complete {return Err(BooleanError::Incomplete);}Ok(&self.output)}
 pub fn into_result(self)->Result<Vec<PathSegment>,BooleanError> {self.result()?;Ok(self.output)}
 /// 🧹️ Move source operands and valid completed paths without destroying private arrangement owners.
 pub fn into_retirement(mut self)->(BooleanRetirement,Vec<BooleanOperand>,Option<Vec<PathSegment>>){
  let operands=std::mem::take(&mut self.input.operands);
  let output=if self.phase==BooleanPhase::Complete&&!self.cancelled&&self.failure.is_none(){Some(std::mem::take(&mut self.output))}else{None};
  self.cancelled=true;self.emitting=None;
  (BooleanRetirement{job:Some(self),edge:None,slot:0,counter:WorkRetirementCounter::default()},operands,output)
 }
 pub fn cancel(&mut self) {self.cancelled=true;self.input.operands=Vec::new();self.source=Vec::new();self.tree=Vec::new();self.query=Vec::new();self.level=Vec::new();self.next_level=Vec::new();self.index_heap=Heap::new(index_order);self.split_heap=Heap::new(f64::total_cmp);self.nodes=Vec::new();self.grid=BTreeMap::new();self.atomic=Vec::new();self.atomic_ids=BTreeSet::new();self.boundary=Vec::new();self.outgoing=BTreeMap::new();self.raw=Vec::new();self.ring=Vec::new();self.positions=BTreeMap::new();self.split_ring=None;self.compact_points=Vec::new();self.rings=Vec::new();self.ring_heap=Heap::new(ring_order);self.emitting=None;self.output=Vec::new();self.left_winding=Vec::new();self.right_winding=Vec::new();}
}
/// 🧽️ Retain interrupted arrangement owners until every nested collection is empty.
pub struct BooleanRetirement{job:Option<BooleanJob>,edge:Option<Edge>,slot:u8,counter:WorkRetirementCounter}
impl BooleanRetirement{
 pub fn terminal_is_empty(&self)->bool{self.job.is_none()&&self.edge.is_none()}
 pub fn advance(&mut self,grant:usize)->Result<BooleanRetirementProgress,BooleanError>{
  let mut counter=self.counter;let p=counter.advance(grant,||self.step()).map_err(BooleanError::Invalid)?;self.counter=counter;Ok(p)
 }
 fn step(&mut self)->bool{
  let Some(job)=self.job.as_mut()else{return true;};
  match self.slot{
   0=>job.input.operands=Vec::new(),
   1=>job.rules=Vec::new(),
   2=>{if let Some(edge)=self.edge.as_mut(){if edge.parameters.pop_first().is_some(){return false;}self.edge=None;return false;}self.edge=job.source.pop();if self.edge.is_some(){return false;}job.source=Vec::new();},
   3=>job.index_heap.values=Vec::new(),
   4=>job.tree=Vec::new(),
   5=>job.level=Vec::new(),
   6=>job.next_level=Vec::new(),
   7=>job.query=Vec::new(),
   8=>job.split_heap.values=Vec::new(),
   9=>job.nodes=Vec::new(),
   10=>{if job.grid.pop_first().is_some(){return false;}job.grid=BTreeMap::new();},
   11=>job.atomic=Vec::new(),
   12=>{if job.atomic_ids.pop_first().is_some(){return false;}job.atomic_ids=BTreeSet::new();},
   13=>job.left_winding=Vec::new(),
   14=>job.right_winding=Vec::new(),
   15=>job.boundary=Vec::new(),
   16=>{if job.outgoing.pop_first().is_some(){return false;}job.outgoing=BTreeMap::new();},
   17=>{if job.raw.pop().is_some(){return false;}job.raw=Vec::new();},
   18=>job.ring=Vec::new(),
   19=>{if job.positions.pop_first().is_some(){return false;}job.positions=BTreeMap::new();},
   20=>job.split_ring=None,
   21=>job.compact_points=Vec::new(),
   22=>{if job.rings.pop().is_some(){return false;}job.rings=Vec::new();},
   23=>job.ring_heap.values=Vec::new(),
   24=>job.output=Vec::new(),
   _=>unreachable!(),
  }
  self.slot+=1;if self.slot==25{self.job=None;return true;}false
 }
}
fn path_operand(segments:&[PathSegment])->Result<BooleanOperand,DrawingError> {
 if segments.len()>65536 {return Err(DrawingError::InvalidInput("Boolean source segment budget exceeded".into()));}let mut contours=Vec::new();let mut points=Vec::new();
 for segment in segments {match segment {PathSegment::Move {to}=>{if !points.is_empty() {return Err(DrawingError::InvalidInput("Boolean input requires closed contours".into()));}points.push(*to);},PathSegment::Line {to}=>{if points.is_empty() {return Err(DrawingError::InvalidInput("Boolean line requires a contour origin".into()));}points.push(*to);},PathSegment::Close=>{if !points.is_empty() {contours.push(std::mem::take(&mut points));}},_=>return Err(DrawingError::InvalidInput("Boolean curves require explicit path preparation".into()))}}
 if !points.is_empty() {return Err(DrawingError::InvalidInput("Boolean input requires closed contours".into()));}Ok(BooleanOperand {contours,fill_rule:BooleanFillRule::Nonzero})
}
fn run_paths(inputs:&[&[PathSegment]],operation:&str)->Result<Vec<PathSegment>,DrawingError> {
 let operation=BooleanOperation::parse(operation).map_err(|error|DrawingError::InvalidInput(error.to_string()))?;
 if inputs.is_empty()||inputs.len()>1024 {return Err(DrawingError::InvalidInput("Boolean needs one to 1024 operands".into()));}let mut count=0usize;for input in inputs {count=count.checked_add(input.len()).ok_or_else(||DrawingError::InvalidInput("Boolean source segment budget exceeded".into()))?;if count>65536{return Err(DrawingError::InvalidInput("Boolean source segment budget exceeded".into()));}}let mut operands=Vec::new();let mut magnitude=0.0f64;
 for input in inputs {let operand=path_operand(input)?;for ring in &operand.contours {for p in ring {magnitude=magnitude.max(p[0].abs()).max(p[1].abs());}}operands.push(operand);}
 let mut job=BooleanJob::new(BooleanInput {operation,operands,epsilon:1e-8f64.max(magnitude*f64::EPSILON*32.0),max_edges:65536,max_parameters:262144,max_atomic_edges:65536,max_segments:65536,max_work:100000000}).map_err(|error|DrawingError::InvalidInput(error.to_string()))?;
 while !job.advance(4096).map_err(|error|DrawingError::Operation(error.to_string()))?.done {}job.into_result().map_err(|error|DrawingError::Operation(error.to_string()))
}
/// 🎯️ Regularize two closed linear paths with nonzero fill through the bounded region job.
pub fn boolean_paths(a:&[PathSegment],b:&[PathSegment],operation:&str)->Result<Vec<PathSegment>,DrawingError> {run_paths(&[a,b],operation)}
/// 🧩️ Regularize one or more closed linear paths, including legitimate empty operands/results.
pub fn boolean_paths_many(inputs:&[Vec<PathSegment>],operation:&str)->Result<Vec<PathSegment>,DrawingError> {if inputs.is_empty()||inputs.len()>1024{return Err(DrawingError::InvalidInput("Boolean needs one to 1024 operands".into()));}let inputs=inputs.iter().map(Vec::as_slice).collect::<Vec<_>>();run_paths(&inputs,operation)}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod bounded_tests;

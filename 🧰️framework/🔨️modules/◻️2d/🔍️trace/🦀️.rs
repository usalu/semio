//! 🔍️ Pixel-cell contours with bounded grants, topology checks and private publication.
use crate::engine::{DrawingError,PathSegment,Vec2};
use semio_framework_value::numeric_scratch::NumericIndex;

#[derive(Clone,Copy,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct BitmapTraceInput<M> {
 pub width:u32,pub height:u32,pub mask:M,pub threshold:f64,pub simplify_epsilon:f64,pub max_pixels:usize,pub max_edges:usize,pub max_segments:usize,pub max_work:u64,
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BitmapTracePhase {Scan,Contours,Compact,Simplify,Topology,Coverage,Emit,Complete}
impl BitmapTracePhase {
 pub fn as_str(self)->&'static str {match self {Self::Scan=>"scan",Self::Contours=>"contours",Self::Compact=>"compact",Self::Simplify=>"simplify",Self::Topology=>"topology",Self::Coverage=>"coverage",Self::Emit=>"emit",Self::Complete=>"complete"}}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct BitmapTraceProgress {
 pub phase:BitmapTracePhase,pub scanned:usize,pub pixels:usize,pub edges:usize,pub contours:usize,pub segments:usize,pub simplified:bool,pub work:u64,pub done:bool,
}
#[derive(Clone,Debug,PartialEq,Eq)]
#[derive(semio_framework_value::RetireOwned)]
pub enum BitmapTraceError {Invalid(&'static str),Incomplete,Cancelled}
impl std::fmt::Display for BitmapTraceError {
 fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {f.write_str(match self {Self::Invalid(message)=>message,Self::Incomplete=>"Bitmap trace incomplete",Self::Cancelled=>"Bitmap trace cancelled"})}
}
impl std::error::Error for BitmapTraceError {}
#[derive(Clone,Copy)]
struct Edge {a:Vec2,b:Vec2,direction:u8,used:bool}
#[derive(Clone,Copy)]
struct Range {a:usize,b:usize,at:usize,distance:f64,index:usize}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Mode {Anchor,Ranges,Build,Edges}
fn cross(a:Vec2,b:Vec2,c:Vec2)->f64 {(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])}
fn between(a:Vec2,b:Vec2,p:Vec2)->bool {cross(a,b,p)==0.0&&p[0]>=a[0].min(b[0])&&p[0]<=a[0].max(b[0])&&p[1]>=a[1].min(b[1])&&p[1]<=a[1].max(b[1])}
fn overlap(a:Vec2,b:Vec2,c:Vec2,d:Vec2)->bool {
 let (p,q,r,s)=(cross(a,b,c),cross(a,b,d),cross(c,d,a),cross(c,d,b));
 if p*q<0.0&&r*s<0.0 {return true;}
 if p==0.0&&q==0.0 {let axis=usize::from((b[0]-a[0]).abs()<(b[1]-a[1]).abs());return a[axis].max(b[axis]).min(c[axis].max(d[axis]))>a[axis].min(b[axis]).max(c[axis].min(d[axis]));}
 (p==0.0&&between(a,b,c)&&c!=a&&c!=b)||(q==0.0&&between(a,b,d)&&d!=a&&d!=b)||(r==0.0&&between(c,d,a)&&a!=c&&a!=d)||(s==0.0&&between(c,d,b)&&b!=c&&b!=d)
}
fn distance(p:Vec2,a:Vec2,b:Vec2)->f64 {
 let (dx,dy)=(b[0]-a[0],b[1]-a[1]);let length=dx*dx+dy*dy;let t=if length==0.0 {0.0} else {(((p[0]-a[0])*dx+(p[1]-a[1])*dy)/length).clamp(0.0,1.0)};
 (p[0]-a[0]-t*dx).hypot(p[1]-a[1]-t*dy)
}
/// ⏱️ Retain borrowed or moved byte storage and visit one pixel, edge, vertex or validation pair per grant.
#[derive(semio_framework_value::RetireOwned)]
pub struct BitmapTraceJob<M:AsRef<[u8]>> {
 input:BitmapTraceInput<Option<M>>,phase:BitmapTracePhase,scanned:usize,work:u64,cancelled:bool,failed:Option<BitmapTraceError>,simplified:bool,
 edges:Vec<Edge>,outgoing:NumericIndex<usize,Vec<usize>>,raw:Vec<Vec<Vec2>>,base:Vec<Vec<Vec2>>,candidate:Vec<Vec<Vec2>>,flat:Vec<(Vec2,Vec2)>,
 first:usize,current:Option<usize>,ring:Vec<Vec2>,positions:NumericIndex<usize,usize>,split:Option<Vec<Vec2>>,split_at:usize,split_stop:usize,split_copy:bool,start:Vec2,contour:usize,at:usize,anchor:usize,farthest:f64,
 mode:Mode,kept:NumericIndex<usize,()>,ranges:Vec<Range>,base_area:f64,candidate_area:f64,previous:Option<Vec2>,changed:bool,
 left:usize,right:usize,pixel:usize,edge:usize,winding:i32,changes:NumericIndex<usize,i32>,covering:bool,output:Vec<PathSegment>,pixels:usize,threshold:u8,edge_count:usize,
}
impl<M:AsRef<[u8]>> BitmapTraceJob<M> {
 pub fn new(input:BitmapTraceInput<M>)->Result<Self,BitmapTraceError> {
  if !(1..=8192).contains(&input.width)||!(1..=8192).contains(&input.height)||!(1..=16777216).contains(&input.max_pixels)||!(1..=65536).contains(&input.max_edges)||!(1..=65536).contains(&input.max_segments)||!(1..=1000000000).contains(&input.max_work)||!input.threshold.is_finite()||!(0.0..=1.0).contains(&input.threshold)||!input.simplify_epsilon.is_finite()||!(0.0..=8192.0).contains(&input.simplify_epsilon) {return Err(BitmapTraceError::Invalid("Invalid bitmap trace contract"));}
  let pixels=input.width as usize*input.height as usize;let threshold=(input.threshold*255.0).round() as u8;
  if pixels>input.max_pixels||input.mask.as_ref().len()!=pixels {return Err(BitmapTraceError::Invalid("Bitmap trace expects exact dimensions within the pixel budget"));}
  let BitmapTraceInput{width,height,mask,threshold:cutoff,simplify_epsilon,max_pixels,max_edges,max_segments,max_work}=input;
  let input=BitmapTraceInput{width,height,mask:Some(mask),threshold:cutoff,simplify_epsilon,max_pixels,max_edges,max_segments,max_work};
  Ok(Self {input,phase:BitmapTracePhase::Scan,scanned:0,work:0,cancelled:false,failed:None,simplified:false,edges:Vec::new(),outgoing:NumericIndex::new(),raw:Vec::new(),base:Vec::new(),candidate:Vec::new(),flat:Vec::new(),first:0,current:None,ring:Vec::new(),positions:NumericIndex::new(),split:None,split_at:0,split_stop:0,split_copy:true,start:[0.0;2],contour:0,at:0,anchor:0,farthest:0.0,mode:Mode::Anchor,kept:NumericIndex::new(),ranges:Vec::new(),base_area:0.0,candidate_area:0.0,previous:None,changed:false,left:0,right:1,pixel:0,edge:0,winding:0,changes:NumericIndex::new(),covering:false,output:Vec::new(),pixels,threshold,edge_count:0})
 }
 fn on(&self,x:i32,y:i32)->bool {x>=0&&y>=0&&x<(self.input.width as i32)&&y<(self.input.height as i32)&&self.input.mask.as_ref().unwrap().as_ref()[y as usize*self.input.width as usize+x as usize]>=self.threshold}
 fn key(&self,p:Vec2)->usize {p[1] as usize*(self.input.width as usize+1)+p[0] as usize}
 fn append_edge(&mut self,a:Vec2,b:Vec2,direction:u8)->Result<(),BitmapTraceError> {
  if self.edges.len()>=self.input.max_edges {return Err(BitmapTraceError::Invalid("Bitmap trace exceeds edge budget"));}
  let index=self.edges.len();self.edges.push(Edge {a,b,direction,used:false});self.edge_count+=1;let key=self.key(a);self.outgoing.get_or_insert_default(key).push(index);Ok(())
 }
 fn fallback(&mut self) {self.simplified=false;self.phase=BitmapTracePhase::Emit;self.contour=0;self.at=0;}
 fn step(&mut self)->Result<(),BitmapTraceError> {
  match self.phase {
   BitmapTracePhase::Scan=>{
    if self.scanned==self.pixels {self.phase=BitmapTracePhase::Contours;return Ok(());}
    let x=(self.scanned%self.input.width as usize) as i32;let y=(self.scanned/self.input.width as usize) as i32;self.scanned+=1;
    if !self.on(x,y) {return Ok(());}let (px,py)=(x as f64,y as f64);
    if !self.on(x,y-1) {self.append_edge([px,py],[px+1.0,py],0)?;}if !self.on(x+1,y) {self.append_edge([px+1.0,py],[px+1.0,py+1.0],1)?;}if !self.on(x,y+1) {self.append_edge([px+1.0,py+1.0],[px,py+1.0],2)?;}if !self.on(x-1,y) {self.append_edge([px,py+1.0],[px,py],3)?;}
   }
   BitmapTracePhase::Contours=>{
    if let Some(split)=self.split.as_mut() {
     if self.split_copy {if self.split_at<self.ring.len() {split.push(self.ring[self.split_at]);self.split_at+=1;return Ok(());}self.split_copy=false;return Ok(());}
     if self.ring.len()>self.split_stop {let p=self.ring.pop().unwrap();self.positions.remove(&self.key(p));return Ok(());}
     if split.len()<3 {return Err(BitmapTraceError::Invalid("Bitmap trace split contour is degenerate"));}self.raw.push(self.split.take().unwrap());return Ok(());
    }
    let Some(current)=self.current else {
     if self.first==self.edges.len() {self.phase=BitmapTracePhase::Compact;self.contour=0;self.at=0;return Ok(());}
     let index=self.first;self.first+=1;if self.edges[index].used {return Ok(());}self.current=Some(index);self.start=self.edges[index].a;self.ring=Vec::new();self.positions.reset();return Ok(());
    };
    let edge=self.edges[current];if edge.used {return Err(BitmapTraceError::Invalid("Bitmap trace contour is not manifold"));}let key=self.key(edge.a);
    if let Some(repeated)=self.positions.get(&key).copied() {self.split=Some(Vec::new());self.split_at=repeated;self.split_stop=repeated;self.split_copy=true;return Ok(());}
    self.positions.insert(key,self.ring.len());self.edges[current].used=true;self.ring.push(edge.a);
    if edge.b==self.start {self.raw.push(std::mem::take(&mut self.ring));self.current=None;return Ok(());}
    let mut next=None;let mut priority=5;
    if let Some(indices)=self.outgoing.get(&self.key(edge.b)) {for index in indices {let candidate=self.edges[*index];if candidate.used {continue;}let turn=(candidate.direction+4-edge.direction)%4;let rank=match turn {1=>0,0=>1,3=>2,_=>3};if rank<priority {next=Some(*index);priority=rank;}}}
    self.current=Some(next.ok_or(BitmapTraceError::Invalid("Bitmap trace contour is open"))?);
   }
   BitmapTracePhase::Compact=>{
    if self.contour==self.raw.len() {self.contour=0;self.at=0;self.phase=if self.input.simplify_epsilon>0.0 {BitmapTracePhase::Simplify} else {BitmapTracePhase::Emit};return Ok(());}
    let raw=&self.raw[self.contour];if self.at==0 {self.base.push(Vec::new());}let p=raw[self.at];let previous=raw[(self.at+raw.len()-1)%raw.len()];let next=raw[(self.at+1)%raw.len()];
    if cross(previous,p,next)!=0.0 {self.base[self.contour].push(p);}self.at+=1;
    if self.at==raw.len() {if self.base[self.contour].len()<3 {return Err(BitmapTraceError::Invalid("Bitmap trace contour is degenerate"));}self.contour+=1;self.at=0;}
   }
   BitmapTracePhase::Simplify=>{
    if self.mode==Mode::Edges {
     if self.contour==self.candidate.len() {if !self.changed {self.fallback();return Ok(());}self.simplified=true;self.phase=BitmapTracePhase::Topology;return Ok(());}
     let ring=&self.candidate[self.contour];self.flat.push((ring[self.at],ring[(self.at+1)%ring.len()]));self.at+=1;if self.at==ring.len() {self.contour+=1;self.at=0;}return Ok(());
    }
    if self.contour==self.base.len() {self.contour=0;self.at=0;self.mode=Mode::Edges;return Ok(());}let ring=&self.base[self.contour];let n=ring.len();
    match self.mode {
     Mode::Anchor=>{
      let p=ring[self.at];let d=(p[0]-ring[0][0]).powi(2)+(p[1]-ring[0][1]).powi(2);if d>self.farthest {self.farthest=d;self.anchor=self.at;}
      self.base_area+=p[0]*ring[(self.at+1)%n][1]-ring[(self.at+1)%n][0]*p[1];self.at+=1;
      if self.at==n {self.kept.reset();self.kept.insert(0,());self.kept.insert(self.anchor,());self.ranges.clear();self.ranges.extend([Range {a:self.anchor,b:n,at:self.anchor+1,distance:0.0,index:0},Range {a:0,b:self.anchor,at:1,distance:0.0,index:0}]);self.mode=Mode::Ranges;}
     }
     Mode::Ranges=>{
      let Some(range)=self.ranges.last_mut() else {self.mode=Mode::Build;self.at=0;self.candidate.push(Vec::new());return Ok(());};
      if range.at<range.b {let d=distance(ring[range.at%n],ring[range.a%n],ring[range.b%n]);if d>range.distance {range.distance=d;range.index=range.at;}range.at+=1;return Ok(());}
      let range=self.ranges.pop().unwrap();if range.distance>self.input.simplify_epsilon {self.kept.insert(range.index%n,());self.ranges.extend([Range {a:range.index,b:range.b,at:range.index+1,distance:0.0,index:0},Range {a:range.a,b:range.index,at:range.a+1,distance:0.0,index:0}]);}
     }
     Mode::Build=>{
      if self.at<n {if self.kept.contains_key(&self.at) {let p=ring[self.at];if let Some(previous)=self.previous {self.candidate_area+=previous[0]*p[1]-p[0]*previous[1];}self.candidate[self.contour].push(p);self.previous=Some(p);}self.at+=1;return Ok(());}
      let candidate=&self.candidate[self.contour];if let Some(previous)=self.previous {if !candidate.is_empty() {self.candidate_area+=previous[0]*candidate[0][1]-candidate[0][0]*previous[1];}}
      if candidate.len()<3||self.candidate_area.signum()!=self.base_area.signum() {self.fallback();return Ok(());}
      self.changed|=candidate.len()!=n;self.contour+=1;self.at=0;self.anchor=0;self.farthest=0.0;self.base_area=0.0;self.candidate_area=0.0;self.previous=None;self.mode=Mode::Anchor;
     }
     Mode::Edges=>unreachable!(),
    }
   }
   BitmapTracePhase::Topology=>{
    if self.left>=self.flat.len().saturating_sub(1) {self.phase=BitmapTracePhase::Coverage;return Ok(());}if self.right==self.flat.len() {self.left+=1;self.right=self.left+1;return Ok(());}
    let (a,b)=self.flat[self.left];let (c,d)=self.flat[self.right];self.right+=1;if overlap(a,b,c,d) {self.fallback();}
   }
   BitmapTracePhase::Coverage=>{
    if self.pixel==self.pixels {self.phase=BitmapTracePhase::Emit;self.contour=0;self.at=0;return Ok(());}
    let width=self.input.width as usize;let x=self.pixel%width;let y=(self.pixel/width) as f64+0.5;
    if !self.covering {
     if self.edge==self.flat.len() {self.covering=true;return Ok(());}let (a,b)=self.flat[self.edge];self.edge+=1;if (a[1]>y)==(b[1]>y) {return Ok(());}
     let crossing=a[0]+(y-a[1])*(b[0]-a[0])/(b[1]-a[1]);let end=(crossing-0.5).ceil();let sign=if b[1]>a[1] {1} else {-1};
     if (crossing-0.5).fract()==0.0&&crossing>=0.5&&crossing<width as f64 {self.fallback();return Ok(());}
     if end>0.0 {self.winding+=sign;if end<width as f64 {*self.changes.get_or_insert_default(end as usize)-=sign;}}return Ok(());
    }
    self.winding+=self.changes.remove(&x).unwrap_or(0);if (self.winding!=0)!=self.on(x as i32,y.floor() as i32) {self.fallback();return Ok(());}
    self.pixel+=1;if self.pixel%width==0 {self.edge=0;self.winding=0;self.covering=false;}
   }
   BitmapTracePhase::Emit=>{
    let rings=if self.simplified {&self.candidate} else {&self.base};if self.contour==rings.len() {self.phase=BitmapTracePhase::Complete;return Ok(());}
    if self.output.len()>=self.input.max_segments {return Err(BitmapTraceError::Invalid("Bitmap trace exceeds segment budget"));}let ring=&rings[self.contour];
    if self.at==ring.len() {self.output.push(PathSegment::Close);self.contour+=1;self.at=0;}else {self.output.push(if self.at==0 {PathSegment::Move {to:ring[self.at]}} else {PathSegment::Line {to:ring[self.at]}});self.at+=1;}
   }
   BitmapTracePhase::Complete=>{},
  }
  Ok(())
 }
 pub fn advance(&mut self,budget:usize)->Result<BitmapTraceProgress,BitmapTraceError> {
  if budget==0||budget as u128>9_007_199_254_740_991 {return Err(BitmapTraceError::Invalid("Bitmap trace grant must be a positive integer"));}
  if self.cancelled {return Err(BitmapTraceError::Cancelled);}if let Some(error)=&self.failed {return Err(error.clone());}
  for _ in 0..budget {if self.phase==BitmapTracePhase::Complete {break;}
   let result=if self.work>=self.input.max_work {Err(BitmapTraceError::Invalid("Bitmap trace exceeds work budget"))} else {self.step()};
   if let Err(error)=result {self.failed=Some(error.clone());return Err(error);}self.work+=1;
  }
  Ok(BitmapTraceProgress {phase:self.phase,scanned:self.scanned,pixels:self.pixels,edges:self.edge_count,contours:self.base.len(),segments:self.output.len(),simplified:self.simplified,work:self.work,done:self.phase==BitmapTracePhase::Complete})
 }
 pub fn result(&self)->Result<&[PathSegment],BitmapTraceError> {if self.cancelled {return Err(BitmapTraceError::Cancelled);}if let Some(error)=&self.failed {return Err(error.clone());}if self.phase!=BitmapTracePhase::Complete {return Err(BitmapTraceError::Incomplete);}Ok(&self.output)}
 pub fn into_result(self)->Result<Vec<PathSegment>,BitmapTraceError> {self.result()?;Ok(self.output)}
 pub fn cancel(&mut self) {self.cancelled=true;}
}
pub type BitmapTraceRetirementProgress=crate::retirement::WorkRetirementProgress;
/// 🧹️ Owns the actual typed trace storage until separately funded physical close.
pub struct BitmapTraceRetirement<M:AsRef<[u8]>+semio_framework_value::retirement::RetireOwned>{owner:semio_framework_value::retirement::controlled::ControlledRetirement<BitmapTraceJob<M>>,work:u64}
impl<M:AsRef<[u8]>+semio_framework_value::retirement::RetireOwned> BitmapTraceRetirement<M>{
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 pub fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{self.owner.step(grant)}
 pub fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_copy_byte_demand()}
 pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{self.owner.next_capacity_byte_demand(copy)}
 pub fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_release_byte_demand()}
 pub fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_depth_demand()}
 pub fn advance(&mut self,items:usize)->Result<BitmapTraceRetirementProgress,BitmapTraceError>{crate::retirement::advance_physical_work(&mut self.owner,&mut self.work,items).map_err(|_|BitmapTraceError::Invalid("Bitmap trace physical close refused"))}
}
impl<M:AsRef<[u8]>+semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetireOwned for BitmapTraceRetirement<M>{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.owner),semio_framework_value::retirement::deferred(self.work)])}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.owner),semio_framework_value::retirement::deferred_birth_bytes_for(&self.work)])}
 fn controlled_retirement_supported()->bool{M::controlled_retirement_supported()}
}
impl<M:AsRef<[u8]>+semio_framework_value::retirement::RetireOwned> BitmapTraceJob<M>{
 /// 🧹️ Transfers genuine mask ownership to the caller and retains every private scratch page.
 pub fn into_retirement(mut self)->(BitmapTraceRetirement<M>,Option<M>){self.cancelled=true;let mask=self.input.mask.take();(BitmapTraceRetirement{owner:semio_framework_value::retirement::controlled::ControlledRetirement::new(self).unwrap_or_else(|(error,_)|panic!("trace physical owner refused: {error}")),work:0},mask)}
}
semio_framework_value::artifact_retire_leaf!(BitmapTracePhase,BitmapTraceProgress,Edge,Range,Mode);
/// 🎯️ Synchronous trace entry point backed by the same bounded job and exact byte contract.
pub fn trace_bitmap_paths(width:u32,height:u32,mask_or_luma:&[u8],threshold:f64,simplify_epsilon:f64)->Result<Vec<PathSegment>,DrawingError> {
 let mut job=BitmapTraceJob::new(BitmapTraceInput {width,height,mask:mask_or_luma,threshold,simplify_epsilon,max_pixels:16777216,max_edges:65536,max_segments:65536,max_work:100000000}).map_err(|error|DrawingError::InvalidInput(error.to_string()))?;
 while !job.advance(4096).map_err(|error|DrawingError::Operation(error.to_string()))?.done {}
 job.into_result().map_err(|error|DrawingError::Operation(error.to_string()))
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod bounded_tests;

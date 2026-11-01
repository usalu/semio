//! 🔍️ Pixel-cell contours with bounded grants, topology checks and private publication.
use semio_framework_2d_engine::{PathSegment,Vec2};
use semio_framework_value::numeric_scratch::{NumericIndex,NumericWorkInsertCursor,NumericWorkRemoveCursor};
use semio_framework_value::{work::RetainedWorkLedger,RetirementDemand};

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
enum Mode {Anchor,SeedZero,SeedAnchor,Ranges,Build,Edges}
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
 input:BitmapTraceInput<Option<M>>,phase:BitmapTracePhase,scanned:usize,work:u64,cancelled:bool,taken:bool,failed:Option<BitmapTraceError>,simplified:bool,
 edges:Vec<Edge>,outgoing:NumericIndex<usize,Vec<usize>>,raw:Vec<Vec<Vec2>>,base:Vec<Vec<Vec2>>,candidate:Vec<Vec<Vec2>>,flat:Vec<(Vec2,Vec2)>,
 first:usize,current:Option<usize>,ring:Vec<Vec2>,positions:NumericIndex<usize,usize>,split:Option<Vec<Vec2>>,split_at:usize,split_stop:usize,split_copy:bool,start:Vec2,contour:usize,at:usize,anchor:usize,farthest:f64,
 mode:Mode,kept:NumericIndex<usize,()>,ranges:Vec<Range>,base_area:f64,candidate_area:f64,previous:Option<Vec2>,changed:bool,
 left:usize,right:usize,pixel:usize,edge:usize,winding:i32,changes:NumericIndex<usize,i32>,covering:bool,output:Vec<PathSegment>,pixels:usize,threshold:u8,edge_count:usize,scan_side:u8,pending:RetirementDemand,last_turn_work:u64,last_turn_semantic_work:u64,step_work:u64,
 outgoing_insert:Option<NumericWorkInsertCursor<usize,Vec<usize>>>,position_insert:Option<NumericWorkInsertCursor<usize,usize>>,kept_insert:Option<NumericWorkInsertCursor<usize,()>>,change_insert:Option<NumericWorkInsertCursor<usize,i32>>,position_remove:Option<NumericWorkRemoveCursor<usize,usize>>,change_remove:Option<NumericWorkRemoveCursor<usize,i32>>,
}
fn trace_slot<T>(values:&mut Vec<T>,work:&mut RetainedWorkLedger)->Result<bool,BitmapTraceError>{let target=if values.len()==values.capacity(){values.capacity().checked_mul(2).unwrap_or(usize::MAX).max(16)}else{values.len()+1};if !work.reserve_vec(values,target).map_err(BitmapTraceError::Invalid)?{return Ok(false);}work.vec_slot(values).map_err(BitmapTraceError::Invalid)}
fn trace_insert<K:Copy+Ord,V>(cursor:&mut Option<NumericWorkInsertCursor<K,V>>,index:&mut NumericIndex<K,V>,key:K,value:impl FnOnce()->V,work:&mut RetainedWorkLedger,actual:&mut u64)->Result<bool,BitmapTraceError>{
 if cursor.is_none(){if !work.can_step(){return Ok(false);}*cursor=Some(NumericWorkInsertCursor::new(key,value()));*actual=1;return Ok(false);}
 let child=cursor.as_mut().ok_or(BitmapTraceError::Invalid("cursor absent"))?;if child.is_complete(){return Ok(true);}let result=child.advance(index,1,work);*actual=child.last_turn_work();result.map_err(BitmapTraceError::Invalid)?;Ok(false)
}
fn trace_remove<K:Copy+Ord,V>(cursor:&mut Option<NumericWorkRemoveCursor<K,V>>,index:&mut NumericIndex<K,V>,key:K,work:&mut RetainedWorkLedger,actual:&mut u64)->Result<bool,BitmapTraceError>{
 if cursor.is_none(){if !work.can_step(){return Ok(false);}*cursor=Some(NumericWorkRemoveCursor::new(key));*actual=1;return Ok(false);}
 let child=cursor.as_mut().ok_or(BitmapTraceError::Invalid("cursor absent"))?;if child.is_complete(){return Ok(true);}let result=child.advance(index,1,work);*actual=child.last_turn_work();result.map_err(BitmapTraceError::Invalid)?;Ok(false)
}
impl<M:AsRef<[u8]>> BitmapTraceJob<M> {
 pub fn validate_input(input:&BitmapTraceInput<M>)->Result<(),BitmapTraceError>{
  if !(1..=8192).contains(&input.width)||!(1..=8192).contains(&input.height)||!(1..=16777216).contains(&input.max_pixels)||!(1..=65536).contains(&input.max_edges)||!(1..=65536).contains(&input.max_segments)||!(1..=1000000000).contains(&input.max_work)||!input.threshold.is_finite()||!(0.0..=1.0).contains(&input.threshold)||!input.simplify_epsilon.is_finite()||!(0.0..=8192.0).contains(&input.simplify_epsilon) {return Err(BitmapTraceError::Invalid("Invalid bitmap trace contract"));}
  let pixels=(input.width as usize).checked_mul(input.height as usize).ok_or(BitmapTraceError::Invalid("Bitmap trace expects exact dimensions within the pixel budget"))?;
  if pixels>input.max_pixels||input.mask.as_ref().len()!=pixels {return Err(BitmapTraceError::Invalid("Bitmap trace expects exact dimensions within the pixel budget"));}
  Ok(())
 }
 /// 📥️ Invalid contracts retain their genuine mask and private owner until funded retirement.
 pub fn new(input:BitmapTraceInput<M>)->Self{
  let failed=Self::validate_input(&input).err();let pixels=(input.width as usize).checked_mul(input.height as usize).unwrap_or(0);let threshold=(input.threshold*255.0).round()as u8;
  let BitmapTraceInput{width,height,mask,threshold:cutoff,simplify_epsilon,max_pixels,max_edges,max_segments,max_work}=input;
  let input=BitmapTraceInput{width,height,mask:Some(mask),threshold:cutoff,simplify_epsilon,max_pixels,max_edges,max_segments,max_work};
  Self {input,phase:BitmapTracePhase::Scan,scanned:0,work:0,cancelled:false,taken:false,failed,simplified:false,edges:Vec::new(),outgoing:NumericIndex::new(),raw:Vec::new(),base:Vec::new(),candidate:Vec::new(),flat:Vec::new(),first:0,current:None,ring:Vec::new(),positions:NumericIndex::new(),split:None,split_at:0,split_stop:0,split_copy:true,start:[0.0;2],contour:0,at:0,anchor:0,farthest:0.0,mode:Mode::Anchor,kept:NumericIndex::new(),ranges:Vec::new(),base_area:0.0,candidate_area:0.0,previous:None,changed:false,left:0,right:1,pixel:0,edge:0,winding:0,changes:NumericIndex::new(),covering:false,output:Vec::new(),pixels,threshold,edge_count:0,scan_side:0,pending:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0,step_work:0,outgoing_insert:None,position_insert:None,kept_insert:None,change_insert:None,position_remove:None,change_remove:None}
 }
 fn on(&self,x:i32,y:i32)->bool {x>=0&&y>=0&&x<(self.input.width as i32)&&y<(self.input.height as i32)&&self.input.mask.as_ref().unwrap().as_ref()[y as usize*self.input.width as usize+x as usize]>=self.threshold}
 fn key(&self,p:Vec2)->usize {p[1] as usize*(self.input.width as usize+1)+p[0] as usize}
 fn fallback(&mut self) {self.simplified=false;self.phase=BitmapTracePhase::Emit;self.contour=0;self.at=0;}
 fn step(&mut self,work:&mut RetainedWorkLedger)->Result<bool,BitmapTraceError>{
  match self.phase{
   BitmapTracePhase::Scan=>{
    if self.scanned==self.pixels{self.phase=BitmapTracePhase::Contours;return Ok(true);}
    let x=(self.scanned%self.input.width as usize)as i32;let y=(self.scanned/self.input.width as usize)as i32;
    if !self.on(x,y)||self.scan_side==4{self.scanned+=1;self.scan_side=0;return Ok(true);}
    let(px,py)=(x as f64,y as f64);let side=self.scan_side;
    let boundary=match side{0=>!self.on(x,y-1),1=>!self.on(x+1,y),2=>!self.on(x,y+1),_=>!self.on(x-1,y)};
    if !boundary{self.scan_side+=1;return Ok(true);}
    if self.edges.len()>=self.input.max_edges{return Err(BitmapTraceError::Invalid("Bitmap trace exceeds edge budget"));}
    let(a,b)=match side{0=>([px,py],[px+1.0,py]),1=>([px+1.0,py],[px+1.0,py+1.0]),2=>([px+1.0,py+1.0],[px,py+1.0]),_=>([px,py+1.0],[px,py])};let key=self.key(a);
    if (self.outgoing_insert.is_some()||!self.outgoing.contains_key(&key))&&!trace_insert(&mut self.outgoing_insert,&mut self.outgoing,key,Vec::new,work,&mut self.step_work)?{return Ok(false);}
    let target=self.outgoing.get_mut(&key).ok_or(BitmapTraceError::Invalid("get_mut target absent"))?;if !trace_slot(target,work)?||!trace_slot(&mut self.edges,work)?{return Ok(false);}
    work.vec_push(target,self.edges.len()).map_err(|(error,_)|BitmapTraceError::Invalid(error))?;
    work.vec_push(&mut self.edges,Edge{a,b,direction:side,used:false}).map_err(|(error,_)|BitmapTraceError::Invalid(error))?;self.edge_count+=1;self.scan_side+=1;self.outgoing_insert=None;
   },
   BitmapTracePhase::Contours=>{
    if self.split.is_some(){
     if self.split_copy{if self.split_at<self.ring.len(){let target=self.split.as_mut().ok_or(BitmapTraceError::Invalid("split absent"))?;if !trace_slot(target,work)?{return Ok(false);}work.vec_push(target,self.ring[self.split_at]).map_err(|(error,_)|BitmapTraceError::Invalid(error))?;self.split_at+=1;return Ok(true);}self.split_copy=false;return Ok(true);}
     if self.ring.len()>self.split_stop{let p=*self.ring.last().ok_or(BitmapTraceError::Invalid("last target absent"))?;let key=self.key(p);if !trace_remove(&mut self.position_remove,&mut self.positions,key,work,&mut self.step_work)?{return Ok(false);}if !work.can_step(){return Ok(false);}self.ring.pop();self.position_remove=None;return Ok(true);}
     if self.split.as_ref().ok_or(BitmapTraceError::Invalid("split absent"))?.len()<3{return Err(BitmapTraceError::Invalid("Bitmap trace split contour is degenerate"));}
     if !trace_slot(&mut self.raw,work)?{return Ok(false);}let split=self.split.take().ok_or(BitmapTraceError::Invalid("split absent"))?;if let Err((error,split))=work.vec_push(&mut self.raw,split){self.split=Some(split);return Err(BitmapTraceError::Invalid(error));}return Ok(true);
    }
    let Some(current)=self.current else{
     if self.first==self.edges.len(){self.phase=BitmapTracePhase::Compact;self.contour=0;self.at=0;return Ok(true);}
     let index=self.first;self.first+=1;if self.edges[index].used{return Ok(true);}self.current=Some(index);self.start=self.edges[index].a;assert!(self.ring.is_empty());self.positions.reset();return Ok(true);
    };
    let edge=self.edges[current];if edge.used{return Err(BitmapTraceError::Invalid("Bitmap trace contour is not manifold"));}let key=self.key(edge.a);
    if self.position_insert.is_none(){if let Some(repeated)=self.positions.get(&key).copied(){self.split=Some(Vec::new());self.split_at=repeated;self.split_stop=repeated;self.split_copy=true;return Ok(true);}}
    let ring_len=self.ring.len();if !trace_insert(&mut self.position_insert,&mut self.positions,key,||ring_len,work,&mut self.step_work)?{return Ok(false);}
    if !trace_slot(&mut self.ring,work)?{return Ok(false);}if edge.b==self.start&&!trace_slot(&mut self.raw,work)?{return Ok(false);}
    work.vec_push(&mut self.ring,edge.a).map_err(|(error,_)|BitmapTraceError::Invalid(error))?;self.edges[current].used=true;self.position_insert=None;
    if edge.b==self.start{let ring=std::mem::take(&mut self.ring);if let Err((error,ring))=work.vec_push(&mut self.raw,ring){self.ring=ring;return Err(BitmapTraceError::Invalid(error));}self.current=None;return Ok(true);}
    let mut next=None;let mut priority=5;
    if let Some(indices)=self.outgoing.get(&self.key(edge.b)){for index in indices{let candidate=self.edges[*index];if candidate.used{continue;}let turn=(candidate.direction+4-edge.direction)%4;let rank=match turn{1=>0,0=>1,3=>2,_=>3};if rank<priority{next=Some(*index);priority=rank;}}}
    self.current=Some(next.ok_or(BitmapTraceError::Invalid("Bitmap trace contour is open"))?);
   },
   BitmapTracePhase::Compact=>{
    if self.contour==self.raw.len(){self.contour=0;self.at=0;self.phase=if self.input.simplify_epsilon>0.0{BitmapTracePhase::Simplify}else{BitmapTracePhase::Emit};return Ok(true);}
    if self.base.len()==self.contour{if !trace_slot(&mut self.base,work)?{return Ok(false);}work.vec_push(&mut self.base,Vec::new()).map_err(|(error,_)|BitmapTraceError::Invalid(error))?;return Ok(true);}
    let raw=&self.raw[self.contour];let p=raw[self.at];let previous=raw[(self.at+raw.len()-1)%raw.len()];let next=raw[(self.at+1)%raw.len()];
    if cross(previous,p,next)!=0.0{let target=&mut self.base[self.contour];if !trace_slot(target,work)?{return Ok(false);}work.vec_push(target,p).map_err(|(error,_)|BitmapTraceError::Invalid(error))?;}
    self.at+=1;if self.at==raw.len(){if self.base[self.contour].len()<3{return Err(BitmapTraceError::Invalid("Bitmap trace contour is degenerate"));}self.contour+=1;self.at=0;}
   },
   BitmapTracePhase::Simplify=>{
    if self.mode==Mode::Edges{
     if self.contour==self.candidate.len(){if !self.changed{self.fallback();return Ok(true);}self.simplified=true;self.phase=BitmapTracePhase::Topology;return Ok(true);}
     let ring=&self.candidate[self.contour];if !trace_slot(&mut self.flat,work)?{return Ok(false);}work.vec_push(&mut self.flat,(ring[self.at],ring[(self.at+1)%ring.len()])).map_err(|(error,_)|BitmapTraceError::Invalid(error))?;self.at+=1;if self.at==ring.len(){self.contour+=1;self.at=0;}return Ok(true);
    }
    if self.contour==self.base.len(){self.contour=0;self.at=0;self.mode=Mode::Edges;return Ok(true);}let ring=&self.base[self.contour];let n=ring.len();
    match self.mode{
     Mode::Anchor=>{
      let p=ring[self.at];let d=(p[0]-ring[0][0]).powi(2)+(p[1]-ring[0][1]).powi(2);if d>self.farthest{self.farthest=d;self.anchor=self.at;}
      self.base_area+=p[0]*ring[(self.at+1)%n][1]-ring[(self.at+1)%n][0]*p[1];self.at+=1;
      if self.at==n{self.kept.reset();self.ranges.clear();self.mode=Mode::SeedZero;}
     },
     Mode::SeedZero=>{if !trace_insert(&mut self.kept_insert,&mut self.kept,0,||(),work,&mut self.step_work)?{return Ok(false);}self.kept_insert=None;self.mode=Mode::SeedAnchor;},
     Mode::SeedAnchor=>{
      if !trace_insert(&mut self.kept_insert,&mut self.kept,self.anchor,||(),work,&mut self.step_work)?{return Ok(false);}
      if !work.reserve_vec(&mut self.ranges,2).map_err(BitmapTraceError::Invalid)?{return Ok(false);}
      if !work.copy_bytes(2*size_of::<Range>()){return Ok(false);}
      self.ranges.extend([Range{a:self.anchor,b:n,at:self.anchor+1,distance:0.0,index:0},Range{a:0,b:self.anchor,at:1,distance:0.0,index:0}]);self.kept_insert=None;self.mode=Mode::Ranges;
     },
     Mode::Ranges=>{
      let Some(range)=self.ranges.last().copied()else{if !trace_slot(&mut self.candidate,work)?{return Ok(false);}work.vec_push(&mut self.candidate,Vec::new()).map_err(|(error,_)|BitmapTraceError::Invalid(error))?;self.mode=Mode::Build;self.at=0;return Ok(true);};
      if range.at<range.b{let d=distance(ring[range.at%n],ring[range.a%n],ring[range.b%n]);let target=self.ranges.last_mut().ok_or(BitmapTraceError::Invalid("last_mut target absent"))?;if d>target.distance{target.distance=d;target.index=target.at;}target.at+=1;return Ok(true);}
      if range.distance>self.input.simplify_epsilon{
       if !trace_insert(&mut self.kept_insert,&mut self.kept,range.index%n,||(),work,&mut self.step_work)?{return Ok(false);}
       let target=self.ranges.len()+1;if !work.reserve_vec(&mut self.ranges,target).map_err(BitmapTraceError::Invalid)?{return Ok(false);}
       if !work.copy_bytes(2*size_of::<Range>()){return Ok(false);}self.ranges.pop();self.ranges.extend([Range{a:range.index,b:range.b,at:range.index+1,distance:0.0,index:0},Range{a:range.a,b:range.index,at:range.a+1,distance:0.0,index:0}]);self.kept_insert=None;
      }else if work.can_step(){self.ranges.pop();}else{return Ok(false);}
     },
     Mode::Build=>{
      if self.at<n{if self.kept.contains_key(&self.at){let p=ring[self.at];let target=&mut self.candidate[self.contour];if !trace_slot(target,work)?{return Ok(false);}work.vec_push(target,p).map_err(|(error,_)|BitmapTraceError::Invalid(error))?;if let Some(previous)=self.previous{self.candidate_area+=previous[0]*p[1]-p[0]*previous[1];}self.previous=Some(p);}self.at+=1;return Ok(true);}
      let candidate=&self.candidate[self.contour];if let Some(previous)=self.previous{if !candidate.is_empty(){self.candidate_area+=previous[0]*candidate[0][1]-candidate[0][0]*previous[1];}}
      if candidate.len()<3||self.candidate_area.signum()!=self.base_area.signum(){self.fallback();return Ok(true);}
      self.changed|=candidate.len()!=n;self.contour+=1;self.at=0;self.anchor=0;self.farthest=0.0;self.base_area=0.0;self.candidate_area=0.0;self.previous=None;self.mode=Mode::Anchor;
     },
     Mode::Edges=>unreachable!(),
    }
   },
   BitmapTracePhase::Topology=>{
    if self.left>=self.flat.len().saturating_sub(1){self.phase=BitmapTracePhase::Coverage;return Ok(true);}if self.right==self.flat.len(){self.left+=1;self.right=self.left+1;return Ok(true);}
    let(a,b)=self.flat[self.left];let(c,d)=self.flat[self.right];self.right+=1;if overlap(a,b,c,d){self.fallback();}
   },
   BitmapTracePhase::Coverage=>{
    if self.pixel==self.pixels{self.phase=BitmapTracePhase::Emit;self.contour=0;self.at=0;return Ok(true);}
    let width=self.input.width as usize;let x=self.pixel%width;let y=(self.pixel/width)as f64+0.5;
    if !self.covering{
     if self.edge==self.flat.len(){self.covering=true;return Ok(true);}let(a,b)=self.flat[self.edge];if(a[1]>y)==(b[1]>y){self.edge+=1;return Ok(true);}
     let crossing=a[0]+(y-a[1])*(b[0]-a[0])/(b[1]-a[1]);let end=(crossing-0.5).ceil();let sign=if b[1]>a[1]{1}else{-1};
     if(crossing-0.5).fract()==0.0&&crossing>=0.5&&crossing<width as f64{self.fallback();return Ok(true);}
     if end>0.0&&end<width as f64{
      let key=end as usize;if self.change_insert.is_some()||!self.changes.contains_key(&key){if !trace_insert(&mut self.change_insert,&mut self.changes,key,||-sign,work,&mut self.step_work)?{return Ok(false);}self.change_insert=None;}else{if !work.can_step(){return Ok(false);}*self.changes.get_mut(&key).ok_or(BitmapTraceError::Invalid("get_mut target absent"))?-=sign;}
     }
     if end>0.0{self.winding+=sign;}self.edge+=1;return Ok(true);
    }
    if !trace_remove(&mut self.change_remove,&mut self.changes,x,work,&mut self.step_work)?{return Ok(false);}
    self.winding+=self.change_remove.as_mut().ok_or(BitmapTraceError::Invalid("change_remove absent"))?.take_value().unwrap_or(0);self.change_remove=None;
    if(self.winding!=0)!=self.on(x as i32,y.floor()as i32){self.fallback();return Ok(true);}
    self.pixel+=1;if self.pixel%width==0{self.edge=0;self.winding=0;self.covering=false;}
   },
   BitmapTracePhase::Emit=>{
    let rings=if self.simplified{&self.candidate}else{&self.base};if self.contour==rings.len(){self.phase=BitmapTracePhase::Complete;self.step_work=1;return Ok(false);}
    if self.output.len()>=self.input.max_segments{return Err(BitmapTraceError::Invalid("Bitmap trace exceeds segment budget"));}let ring=&rings[self.contour];if !trace_slot(&mut self.output,work)?{return Ok(false);}
    if self.at==ring.len(){work.vec_push(&mut self.output,PathSegment::Close).map_err(|(error,_)|BitmapTraceError::Invalid(error))?;self.contour+=1;self.at=0;}else{work.vec_push(&mut self.output,if self.at==0{PathSegment::Move{to:ring[self.at]}}else{PathSegment::Line{to:ring[self.at]}}).map_err(|(error,_)|BitmapTraceError::Invalid(error))?;self.at+=1;}
   },
   BitmapTracePhase::Complete=>{}
  }Ok(true)
 }
 pub fn settlement_pending(&self)->bool{self.phase==BitmapTracePhase::Complete||self.phase==BitmapTracePhase::Emit&&self.contour==if self.simplified{self.candidate.len()}else{self.base.len()}}
 pub fn failure_pending(&self)->bool{self.failed.is_some()}
 pub fn next_work_demand(&self)->RetirementDemand{self.pending}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 fn progress(&self)->BitmapTraceProgress{BitmapTraceProgress{phase:self.phase,scanned:self.scanned,pixels:self.pixels,edges:self.edge_count,contours:self.base.len(),segments:self.output.len(),simplified:self.simplified,work:self.work,done:self.phase==BitmapTracePhase::Complete}}
 pub fn advance(&mut self,budget:usize,work:&mut RetainedWorkLedger)->Result<BitmapTraceProgress,BitmapTraceError>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if budget==0||budget as u128>9_007_199_254_740_991{return Err(BitmapTraceError::Invalid("Bitmap trace grant must be a positive integer"));}
  if self.cancelled{return Err(BitmapTraceError::Cancelled);}if let Some(error)=&self.failed{return Err(error.clone());}
  let d=self.pending;if !work.admits_absolute(d){work.require_absolute(d);return Ok(self.progress());}
  let Some(root)=work.enter()else{self.pending=work.next_demand();return Ok(self.progress());};
  let result=(||{for _ in 0..budget{if self.phase==BitmapTracePhase::Complete||!work.can_step(){break;}if self.work>=self.input.max_work&&!self.settlement_pending(){let error=BitmapTraceError::Invalid("Bitmap trace exceeds work budget");self.failed=Some(error.clone());return Err(error);}
   let before=work.receipt();self.step_work=0;let result=self.step(work);let semantic=matches!(&result,Ok(true)|Err(_));let actual=semantic||self.step_work>0||work.receipt()!=before;if actual{work.note_source(root);self.last_turn_work+=self.step_work.max(1);}if semantic{self.work+=1;self.last_turn_semantic_work+=1;}self.pending=work.next_demand();if let Err(error)=result{self.failed=Some(error.clone());return Err(error);}if work.blocked(){break;}
  }Ok(self.progress())})();work.exit();result
 }
 pub fn result(&self)->Result<&[PathSegment],BitmapTraceError> {if self.cancelled {return Err(BitmapTraceError::Cancelled);}if let Some(error)=&self.failed {return Err(error.clone());}if self.phase!=BitmapTracePhase::Complete||self.taken{return Err(BitmapTraceError::Incomplete);}Ok(&self.output)}
 pub fn take_result(&mut self)->Result<Vec<PathSegment>,BitmapTraceError>{self.result()?;self.taken=true;Ok(std::mem::take(&mut self.output))}
 pub fn cancel(&mut self) {self.cancelled=true;}
}
/// 🧹️ Owns the actual typed trace storage until separately funded physical close.
pub struct BitmapTraceRetirement<M:AsRef<[u8]>+semio_framework_value::retirement::RetireOwned>{owner:semio_framework_value::retirement::controlled::ControlledRetirement<BitmapTraceJob<M>>}
impl<M:AsRef<[u8]>+semio_framework_value::retirement::RetireOwned> BitmapTraceRetirement<M>{
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 pub fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{self.owner.step(grant)}
 pub fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_copy_byte_demand()}
 pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{self.owner.next_capacity_byte_demand(copy)}
 pub fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_release_byte_demand()}
 pub fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_depth_demand()}
}
impl<M:AsRef<[u8]>+semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetireOwned for BitmapTraceRetirement<M>{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.owner)}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.owner)}
 fn controlled_retirement_supported()->bool{M::controlled_retirement_supported()}
}
impl<M:AsRef<[u8]>+semio_framework_value::retirement::RetireOwned> BitmapTraceJob<M>{
 /// 🎭️ Transfers genuine mask ownership to the caller and retains every private scratch page.
 pub fn into_retirement(mut self)->(BitmapTraceRetirement<M>,Option<M>){self.cancelled=true;let mask=self.input.mask.take();(BitmapTraceRetirement{owner:semio_framework_value::retirement::controlled::ControlledRetirement::retain(self)},mask)}
}
semio_framework_value::artifact_retire_leaf!(BitmapTracePhase,BitmapTraceProgress,Edge,Range,Mode);
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod bounded_tests;

#[cfg(test)]
use semio_framework_value_testing_allocator as _;
#[cfg(test)]
#[path="🎟️work/🧪️tests/🦀️.rs"]
mod work_tests;

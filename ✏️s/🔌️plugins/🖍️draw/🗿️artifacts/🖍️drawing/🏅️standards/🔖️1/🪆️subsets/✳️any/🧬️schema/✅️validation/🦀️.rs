//! ✅️ Canonical Drawing admission retains the exact original typed source and every partial numeric owner.
use crate::snapshot::DrawingSnapshot;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_io_ports::{ChildAdmissionSource,ChildAdmissionAdvance,EmbeddedChildAdmission,EmbeddedChildAdmissionCursorV1,EmbeddedChildAdmitterV1,VectorCanvas};
use std::{future::Future,pin::Pin,task::{Context,Poll,Waker},mem::size_of_val};
type Text=semio_framework_value::paged::PagedUtf8<{usize::MAX}>;
mod source {
//! 🧵️ Borrowed ancestry and paged text cursors never copy a Drawing key, node or path.
use super::*;
use semio_framework_value::paged::Utf8Text;
use semio_framework_value_persistent::list::PagedList;
use std::mem::size_of;
pub(super) type Layers=PagedList<crate::DrawingLayerNode,{usize::MAX}>;
#[derive(Clone,Copy)]struct Frame{layers:&'static Layers,at:usize,matrix:[f64;6]}
pub(super) struct Walk{frames:PagedList<Frame,{usize::MAX}>,pending:Option<Frame>}
pub(super) enum WalkTurn{Progress,Node(&'static crate::DrawingLayerNode),Done}
impl Default for Walk{fn default()->Self{Self{frames:Default::default(),pending:None}}}
fn page_error(_:impl std::fmt::Display)->ValueError{fault("Drawing ancestry backing is invalid")}
impl Walk{
 pub fn start(&mut self,source:&'static DrawingSnapshot){self.pending=Some(Frame{layers:&source.layers,at:0,matrix:[1.0,0.0,0.0,1.0,0.0,0.0]})}
 pub fn demand(&self)->Result<RetirementDemand,ValueError>{if self.pending.is_some()&&!self.frames.has_reserved_slot(){return Ok(RetirementDemand{copy_bytes:self.frames.next_reserve_copy_byte_demand().map_err(page_error)?,capacity_bytes:self.frames.next_allocation_bytes().map_err(page_error)?,depth:self.frames.next_reserve_depth_demand().map_err(page_error)?.max(self.frames.len()+1),..Default::default()})}Ok(RetirementDemand{copy_bytes:size_of::<Frame>()*2+512,depth:(self.frames.len()+usize::from(self.pending.is_some())).max(1),..Default::default()})}
 pub fn advance(&mut self,g:RetainedCloneGrant)->Result<(WalkTurn,RetainedCloneProgress,Option<&'static str>),ValueError>{let d=self.demand()?;if !retained_clone_grant_admits(g,d){return Ok((WalkTurn::Progress,Default::default(),None))}if self.pending.is_some()&&!self.frames.has_reserved_slot(){let p=self.frames.reserve_one_funded(g).map_err(|e|fault("Drawing ancestry allocation failed").with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:e.allocated_bytes,..Default::default()}))?;return Ok((WalkTurn::Progress,p,None))}let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};if let Some(frame)=self.pending.take(){self.frames.push_reserved(frame).map_err(|_|fault("Drawing ancestry reserved slot vanished"))?;return Ok((WalkTurn::Progress,p,None))}let Some(last)=self.frames.len().checked_sub(1)else{return Ok((WalkTurn::Done,p,None))};let frame=self.frames[last];let Some(node)=frame.layers.get(frame.at)else{self.frames.pop();return Ok((WalkTurn::Progress,p,None))};self.frames[last].at+=1;let own=crate::drawing_transform_to_matrix(&crate::layer_base(node).transform);let a=frame.matrix;let b=own;let matrix=[a[0]*b[0]+a[2]*b[1],a[1]*b[0]+a[3]*b[1],a[0]*b[2]+a[2]*b[3],a[1]*b[2]+a[3]*b[3],a[0]*b[4]+a[2]*b[5]+a[4],a[1]*b[4]+a[3]*b[5]+a[5]];if matrix.iter().any(|v|!v.is_finite()){return Ok((WalkTurn::Progress,p,Some("Drawing transforms and composed matrices must be finite.")))}if let crate::DrawingLayerNode::Group(group)=node{self.pending=Some(Frame{layers:&group.children,at:0,matrix})}Ok((WalkTurn::Node(node),p,None))}
 pub fn close_demand(&self)->Result<RetirementDemand,ValueError>{if self.pending.is_some()||!self.frames.is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Frame>(),depth:self.frames.len().max(1),..Default::default()})}Ok(RetirementDemand{release_bytes:self.frames.next_release_allocation_bytes().map_err(page_error)?,depth:self.frames.next_release_depth_demand().map_err(page_error)?.max(1),..Default::default()})}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=self.close_demand()?;if !retained_clone_grant_admits(g,d){return Ok(Default::default())}if self.pending.take().is_some()||self.frames.pop().is_some(){return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}let p=self.frames.release_empty_page(g.maximum_release_bytes).map_err(page_error)?;Ok(RetainedCloneProgress{copied_items:usize::from(p.progressed),released_bytes:p.released_allocation_bytes,..Default::default()})}
 pub fn terminal_is_empty(&self)->bool{self.pending.is_none()&&self.frames.terminal_is_empty()}
}
#[derive(Default)]pub(super) struct PagedHash{chunk:usize,at:usize,pub hash:u64,started:bool}
impl PagedHash{pub fn step<T:Utf8Text+?Sized>(&mut self,text:&T)->bool{if !self.started{self.hash=14695981039346656037;self.started=true}if self.chunk==text.text_chunk_count(){return true}let chunk=text.text_chunk(self.chunk).expect("immutable paged text chunk");let end=chunk.len().min(self.at.saturating_add(256));for byte in &chunk.as_bytes()[self.at..end]{self.hash=(self.hash^u64::from(*byte)).wrapping_mul(1099511628211)}self.at=end;if end==chunk.len(){self.chunk+=1;self.at=0}self.chunk==text.text_chunk_count()}}
#[derive(Default)]pub(super) struct PagedCompare{a:usize,b:usize,ap:usize,bp:usize,done:usize}
impl PagedCompare{pub fn step<A:Utf8Text+?Sized,B:Utf8Text+?Sized>(&mut self,a:&A,b:&B)->Option<bool>{if a.text_bytes()!=b.text_bytes(){return Some(false)}if self.done==a.text_bytes(){return Some(true)}let aa=a.text_chunk(self.a).expect("original first text chunk");let bb=b.text_chunk(self.b).expect("original second text chunk");let n=(aa.len()-self.ap).min(bb.len()-self.bp).min(256);if aa.as_bytes()[self.ap..self.ap+n]!=bb.as_bytes()[self.bp..self.bp+n]{return Some(false)}self.done+=n;self.ap+=n;self.bp+=n;if self.ap==aa.len(){self.a+=1;self.ap=0}if self.bp==bb.len(){self.b+=1;self.bp=0}if self.done==a.text_bytes(){Some(true)}else{None}}}

}
mod index {
//! 📇️ The Draw index stores only original paged keys, original node/asset loans and fixed graph marks.
use super::*;
use semio_framework_value::scratch_hash::{ScratchHashMap,ScratchHashSlots};
use std::sync::atomic::{AtomicU8,AtomicUsize};
use super::source::{PagedHash,PagedCompare};
#[derive(Clone,Copy)]pub(super) enum Target{Layer(&'static crate::DrawingLayerNode),Asset(&'static crate::DrawingImageAsset)}
pub(super) struct Entry{pub key:&'static Text,pub target:Target,pub mark:AtomicU8,pub parent:AtomicUsize,pub edge:AtomicUsize}
impl Entry{fn new(key:&'static Text,target:Target)->Self{Self{key,target,mark:AtomicU8::new(0),parent:AtomicUsize::new(usize::MAX),edge:AtomicUsize::new(0)}}}
pub(super) type Slots<'a>=ScratchHashSlots<'a,(u8,u64),Entry>;
pub(super) fn start(kind:u8,hash:u64,length:usize)->usize{(hash^(kind as u64).wrapping_mul(0x9e3779b97f4a7c15))as usize&(length-1)}
pub(super) struct Index{map:ScratchHashMap<(u8,u64),Entry>,pending:Option<(u8,&'static Text,Target)>,hash:PagedHash,comparison:PagedCompare,probe:Option<usize>}
impl Default for Index{fn default()->Self{Self{map:Default::default(),pending:None,hash:Default::default(),comparison:Default::default(),probe:None}}}
impl Index{
 pub fn reserve_demand(count:usize)->Result<RetirementDemand,ValueError>{ScratchHashMap::<(u8,u64),Entry>::reservation_demands(count)}
 pub fn reserve(&mut self,count:usize,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=Self::reserve_demand(count)?;if !retained_clone_grant_admits(g,d){return Ok(Default::default())}self.map=ScratchHashMap::try_reserve_uninitialized(count)?;let p=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:self.map.backing_bytes(),..Default::default()};if p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault("Draw index reservation exceeded original grant").with_retained_progress(p))}Ok(p)}
 pub fn initialized(&self)->bool{self.map.initialized_slots()==self.map.required_slots().unwrap_or(usize::MAX)}
 pub fn initialize_demand()->RetirementDemand{RetirementDemand{copy_bytes:ScratchHashMap::<(u8,u64),Entry>::slot_bytes(),depth:1,..Default::default()}}
 pub fn initialize(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=Self::initialize_demand();if !retained_clone_grant_admits(g,d){return Ok(Default::default())}let changed=self.map.initialize_one()?;Ok(RetainedCloneProgress{copied_items:usize::from(changed),copied_bytes:if changed{d.copy_bytes}else{0},..Default::default()})}
 pub fn begin_key(&mut self,kind:u8,key:&'static Text,target:Target){self.pending=Some((kind,key,target));self.hash=Default::default();self.comparison=Default::default();self.probe=None}
 pub fn pending(&self)->bool{self.pending.is_some()}
 pub fn step(&mut self)->Option<&'static str>{let(kind,key,target)=self.pending.expect("selected original Draw key");if key.is_empty(){return Some("Drawing identities must be non-empty and unique within their namespace.")}if self.probe.is_none(){if self.hash.step(key){self.probe=Some(start(kind,self.hash.hash,self.map.initialized_slots()))}return None}let at=self.probe.unwrap();let slots=self.map.slots();if slots.is_vacant(at){self.map.insert_vacant(at,(kind,self.hash.hash),Entry::new(key,target)).unwrap_or_else(|_|unreachable!("admitted Draw index slot"));self.pending=None;return None}let(held,entry)=slots.get(at).expect("original occupied Draw slot");if *held==(kind,self.hash.hash){match self.comparison.step(key,entry.key){Some(true)=>return Some("Drawing identities must be non-empty and unique within their namespace."),None=>return None,Some(false)=>{}}}self.probe=Some((at+1)&(slots.len()-1));self.comparison=Default::default();None}
 pub fn slots(&self)->Slots<'_>{self.map.slots()}
 pub fn close_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:ScratchHashMap::<(u8,u64),Entry>::slot_bytes(),release_bytes:if self.pending.is_none()&&self.map.initialized_slots()==0{self.map.backing_bytes()}else{0},depth:1,..Default::default()}}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=self.close_demand();if !retained_clone_grant_admits(g,d){return Ok(Default::default())}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};if self.pending.take().is_some(){return Ok(p)}if self.map.pop_last_slot().is_none(){p.released_bytes=self.map.release_empty_backing()?}Ok(p)}
 pub fn terminal_is_empty(&self)->bool{self.pending.is_none()&&self.map.initialized_slots()==0&&self.map.backing_bytes()==0}
}

}
mod program {
//! 📋️ Canonical Drawing graph admission borrows every typed authored value without planning or rendering it.
use super::*;
use super::index::{Slots,Target,start};
use super::source::{PagedHash,PagedCompare};
use semio_framework_value::paged::Utf8Text;
use std::{future::Future,pin::Pin,task::{Context,Poll},sync::atomic::Ordering};
struct Tick(bool);
impl Future for Tick{type Output=();fn poll(mut self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<()>{if self.0{Poll::Ready(())}else{self.0=true;Poll::Pending}}}
async fn tick(){Tick(false).await}
fn require(valid:bool,message:&'static str)->Result<(),&'static str>{if valid{Ok(())}else{Err(message)}}
fn positive(v:f64)->bool{v.is_finite()&&v>0.0}
fn nonnegative(v:f64)->bool{v.is_finite()&&v>=0.0}
fn unit(v:f64)->bool{v.is_finite()&&(0.0..=1.0).contains(&v)}
fn color(v:&[f64;4])->bool{v.iter().copied().all(unit)}
async fn same<A:Utf8Text+Sync+?Sized,B:Utf8Text+Sync+?Sized>(a:&A,b:&B)->bool{let mut cursor=PagedCompare::default();loop{tick().await;if let Some(equal)=cursor.step(a,b){return equal}}}
async fn member(text:&Text,values:&[&str])->bool{for value in values{if same(text,*value).await{return true}}false}
async fn lookup<'a>(slots:Slots<'a>,kind:u8,key:&Text)->Option<(usize,&'a index::Entry)>{if slots.len()==0{tick().await;return None}let mut hash=PagedHash::default();loop{tick().await;if hash.step(key){break}}let mut at=start(kind,hash.hash,slots.len());for _ in 0..slots.len(){tick().await;let Some((held,entry))=slots.get(at)else{return None};if *held==(kind,hash.hash)&&same(key,entry.key).await{return Some((at,entry))}at=(at+1)&(slots.len()-1);}None}
async fn attributes(attributes:&crate::DrawingAttributes)->Result<(),&'static str>{
 if let Some(fill)=&attributes.fill{tick().await;let stops=match fill{crate::FillStyle::Solid{color:c}=>{require(color(c),"Drawing paint channels must be finite unit values.")?;None},crate::FillStyle::LinearGradient{x1,y1,x2,y2,stops}=>{require([x1,y1,x2,y2].iter().all(|v|v.is_finite())&&!stops.is_empty(),"Drawing gradients need finite geometry and non-empty stops.")?;Some(stops)},crate::FillStyle::RadialGradient{cx,cy,r,stops}=>{require(cx.is_finite()&&cy.is_finite()&&positive(*r)&&!stops.is_empty(),"Drawing radial gradients need finite geometry, positive radius and non-empty stops.")?;Some(stops)}};if let Some(stops)=stops{for stop in stops.iter(){tick().await;require(unit(stop.offset)&&color(&stop.color),"Drawing gradient stops must have unit offsets and paint channels.")?}}}
 if let Some(stroke)=&attributes.stroke{tick().await;require(nonnegative(stroke.width)&&color(&stroke.color),"Drawing strokes need non-negative finite width and unit paint channels.")?;if let Some(dash)=&stroke.dash{for value in dash.iter(){tick().await;require(nonnegative(*value),"Drawing dash lengths must be finite and non-negative.")?}}}Ok(())
}
async fn shape(shape:&crate::DrawingShapeBody)->Result<(),&'static str>{tick().await;let count=usize::from(shape.rect.is_some())+usize::from(shape.ellipse.is_some())+usize::from(shape.circle.is_some())+usize::from(shape.line.is_some())+usize::from(shape.polygon.is_some());require(count==1,"Drawing shape kind must match exactly one geometry.")?;
 if same(&shape.shape_kind,"rect").await{let r=shape.rect.as_ref().ok_or("Drawing shape kind must match exactly one geometry.")?;require(r.x.is_finite()&&r.y.is_finite()&&nonnegative(r.width)&&nonnegative(r.height),"Drawing rectangles need finite coordinates and non-negative extents.")?}
 else if same(&shape.shape_kind,"ellipse").await{let r=shape.ellipse.as_ref().ok_or("Drawing shape kind must match exactly one geometry.")?;require(r.cx.is_finite()&&r.cy.is_finite()&&nonnegative(r.rx)&&nonnegative(r.ry),"Drawing ellipses need finite coordinates and non-negative radii.")?}
 else if same(&shape.shape_kind,"circle").await{let r=shape.circle.as_ref().ok_or("Drawing shape kind must match exactly one geometry.")?;require(r.cx.is_finite()&&r.cy.is_finite()&&nonnegative(r.r),"Drawing circles need finite coordinates and non-negative radius.")?}
 else if same(&shape.shape_kind,"line").await{let r=shape.line.as_ref().ok_or("Drawing shape kind must match exactly one geometry.")?;require([r.x1,r.y1,r.x2,r.y2].iter().all(|v|v.is_finite()),"Drawing line coordinates must be finite.")?}
 else if same(&shape.shape_kind,"polygon").await{let p=shape.polygon.as_ref().ok_or("Drawing shape kind must match exactly one geometry.")?;for point in p.points.iter(){tick().await;require(point.iter().all(|v|v.is_finite()),"Drawing polygon coordinates must be finite.")?}}
 else{return Err("Drawing shape kind must name an authored primitive.")}Ok(())
}
pub(super) async fn check(source:&DrawingSnapshot,slots:Slots<'_>)->Result<(),&'static str>{
 tick().await;require(same(&source.schema,crate::DRAWING_DOCUMENT_SCHEMA).await&&!source.id.is_empty(),"Drawing documents need their exact schema and non-empty identity.")?;
 for at in 0..slots.len(){tick().await;let Some((_,entry))=slots.get(at)else{continue};match entry.target{
 Target::Asset(asset)=>{let count=u64::from(asset.width)*u64::from(asset.height);require(count>0&&usize::try_from(count).is_ok_and(|count|count==asset.samples.len()),"Drawing asset dimensions and intrinsic RGBA sample count must agree.")?},
 Target::Layer(node)=>{let base=crate::layer_base(node);let t=&base.transform;require([t.x,t.y,t.scale_x,t.scale_y,t.shear,t.rotation].iter().all(|v|v.is_finite())&&unit(base.opacity)&&member(&base.blend_mode,crate::DRAWING_BLEND_MODES).await,"Drawing layers need finite transforms, unit opacity and an authored blend mode.")?;attributes(&base.attributes).await?;match node{
 crate::DrawingLayerNode::Path(path)=>for segment in path.segments.iter(){tick().await;require(crate::valid_path_segment(segment),"Drawing path geometry must be finite with non-negative arc radii.")?},
 crate::DrawingLayerNode::Shape(value)=>shape(value).await?,
 crate::DrawingLayerNode::Text(value)=>require(value.x.is_finite()&&value.y.is_finite()&&positive(value.size),"Drawing text needs finite coordinates and positive size.")?,
 crate::DrawingLayerNode::Image(value)=>{require(positive(value.width)&&positive(value.height),"Drawing images need positive finite extents.")?;require(lookup(slots,1,&value.image_key).await.is_some(),"Drawing image layers must reference an existing asset.")?},
 crate::DrawingLayerNode::Boolean(value)=>require(member(&value.operation,crate::DRAWING_BOOLEAN_OPERATIONS).await,"Drawing booleans must use an authored operation.")?,
 crate::DrawingLayerNode::Trace(value)=>{require(unit(value.params.threshold)&&nonnegative(value.params.simplify_epsilon),"Drawing trace settings need unit threshold and non-negative finite epsilon.")?;require(lookup(slots,1,&value.source_key).await.is_some(),"Drawing trace layers must reference an existing asset.")?},
 crate::DrawingLayerNode::Group(_)=>{}}}}
 }
 for root in 0..slots.len(){tick().await;let Some((_,entry))=slots.get(root)else{continue};if !matches!(entry.target,Target::Layer(_))||entry.mark.load(Ordering::Relaxed)==2{continue}entry.mark.store(1,Ordering::Relaxed);let mut at=root;while at!=usize::MAX{tick().await;let(_,current)=slots.get(at).expect("original active Draw layer slot");let Target::Layer(node)=current.target else{unreachable!()};let edge=current.edge.fetch_add(1,Ordering::Relaxed);let key=match node{crate::DrawingLayerNode::Group(group)=>group.children.get(edge).map(|node|&crate::layer_base(node).id),crate::DrawingLayerNode::Boolean(value)=>value.children.get(edge),_=>None};if let Some(key)=key{let(next,target)=lookup(slots,0,key).await.ok_or("Drawing operands must reference existing layers.")?;match target.mark.load(Ordering::Relaxed){1=>return Err("Drawing group and boolean dependencies must not contain cycles."),0=>{target.parent.store(at,Ordering::Relaxed);target.mark.store(1,Ordering::Relaxed);at=next},_=>{}}}else{current.mark.store(2,Ordering::Relaxed);at=current.parent.load(Ordering::Relaxed);}}}
 Ok(())
}

}
fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
type Program=Pin<Box<dyn Future<Output=Result<(),&'static str>>+Send>>;
/// 🏷️ The real Drawing editor/member capability binds this producer.
pub struct DrawingAdmitter;
pub static DRAWING_ADMITTER:DrawingAdmitter=DrawingAdmitter;
impl EmbeddedChildAdmitterV1 for DrawingAdmitter{
 fn birth_demand(&self)->RetirementDemand{EmbeddedChildAdmission::constructor_demand::<DrawingAdmissionCursor>()}
 fn begin(&self,g:RetainedCloneGrant)->Result<(EmbeddedChildAdmission,RetainedCloneProgress),ValueError>{EmbeddedChildAdmission::admit(DrawingAdmissionCursor::new(),g).map_err(|(error,_)|error)}
}
/// 🧳️ Original source loans and index views remain alive through continuation closure before ancestry/index/alias retirement.
pub struct DrawingAdmissionCursor{binding:Option<RetainedCloneBinding>,source:Option<&'static DrawingSnapshot>,walk:source::Walk,index:index::Index,count:usize,asset:usize,phase:u8,program:Option<Program>,program_bytes:usize,done:bool,closing:bool,error:Option<&'static str>}
impl Default for DrawingAdmissionCursor{fn default()->Self{Self::new()}}
impl DrawingAdmissionCursor{
 const OVERHEAD:usize=1024;
 pub fn new()->Self{Self{binding:None,source:None,walk:Default::default(),index:Default::default(),count:0,asset:0,phase:0,program:None,program_bytes:0,done:false,closing:false,error:None}}
 fn snapshot(&self)->&'static DrawingSnapshot{self.source.expect("original Drawing snapshot binding")}
 fn nested(d:RetirementDemand)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:d.copy_bytes.checked_add(Self::OVERHEAD).ok_or_else(||fault("Drawing admission nested copy overflow"))?,depth:d.depth.checked_add(1).ok_or_else(||fault("Drawing admission nested depth overflow"))?,..d})}
 fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(Self::OVERHEAD),maximum_depth:g.maximum_depth.saturating_sub(1),..g}}
 fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.saturating_add(Self::OVERHEAD);p}
 fn failure(error:ValueError)->ValueError{let p=Self::receipt(error.retained_progress());error.with_retained_progress(p)}
 fn future_bytes(&self)->usize{size_of_val(&program::check(self.snapshot(),self.index.slots()))}
 fn outcome(&self,progress:RetainedCloneProgress,canvas:Option<VectorCanvas>)->ChildAdmissionAdvance{ChildAdmissionAdvance{progress,done:self.done,error:self.error,canvas}}
 fn refuse(&mut self,error:&'static str){self.error=Some(error);self.done=true}
}
impl EmbeddedChildAdmissionCursorV1 for DrawingAdmissionCursor{
 fn next_demand(&self,s:ChildAdmissionSource<'_>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing{return Err(fault("Drawing admission is closing"))}if !s.get().is::<DrawingSnapshot>(){return Err(fault("Drawing admission requires its original typed DrawingSnapshot"))}let binding=s.binding_demand(&self.binding)?;if self.binding.is_none(){return Self::nested(binding.unwrap_or_default())}if self.done{return Ok(Default::default())}match self.phase{2=>Self::nested(self.walk.demand()?),3=>Self::nested(index::Index::reserve_demand(self.count)?),4=>Self::nested(index::Index::initialize_demand()),5 if !self.index.pending()=>Self::nested(self.walk.demand()?),7=>Ok(RetirementDemand{copy_bytes:self.future_bytes(),capacity_bytes:self.future_bytes(),depth:1,..Default::default()}),8=>Ok(RetirementDemand{copy_bytes:self.program_bytes.checked_mul(2).and_then(|n|n.checked_add(2048)).ok_or_else(||fault("Drawing continuation work overflow"))?,depth:1,..Default::default()}),_=>{let _=body;Ok(RetirementDemand{copy_bytes:Self::OVERHEAD,depth:1,..Default::default()})}}}
 /// 🪢️ The admitted original binding and fixed index allocation keep both immutable continuation loans live until their funded drop.
 fn advance(&mut self,s:ChildAdmissionSource<'_>,g:RetainedCloneGrant)->Result<ChildAdmissionAdvance,ValueError>{let d=self.next_demand(s,g.maximum_copy_bytes)?;if self.done||!retained_clone_grant_admits(g,d){return Ok(self.outcome(Default::default(),None))}if self.binding.is_none(){let p=s.bind(&mut self.binding,Self::child(g)).map_err(Self::failure)?.unwrap_or_default();if self.binding.is_some(){let root=s.get().downcast_ref::<DrawingSnapshot>().expect("checked Drawing owner");self.source=Some(unsafe{&*(root as*const DrawingSnapshot)});self.phase=1;}return Ok(self.outcome(Self::receipt(p),None))}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};let mut canvas=None;match self.phase{
 1=>{let root=self.snapshot();if !root.schema.eq_str(crate::DRAWING_DOCUMENT_SCHEMA)||root.id.is_empty(){self.refuse("Drawing documents need their exact schema and non-empty identity.")}else if let Some(board)=&root.artboard{if !board.width.is_finite()||!board.height.is_finite()||board.width<=0.0||board.height<=0.0{self.refuse("Drawing authored canvas dimensions must be positive and finite.")}else{canvas=Some(VectorCanvas{width:board.width,height:board.height,background:None})}}if !self.done{self.walk.start(root);self.phase=2}},
 2=>{let(turn,used,error)=self.walk.advance(Self::child(g)).map_err(Self::failure)?;p=Self::receipt(used);if let Some(error)=error{self.refuse(error)}else{match turn{source::WalkTurn::Node(_)=>self.count=self.count.checked_add(1).ok_or_else(||fault("Drawing layer index extent overflow").with_retained_progress(p))?,source::WalkTurn::Done=>{self.count=self.count.checked_add(self.snapshot().assets.len()).ok_or_else(||fault("Drawing asset index extent overflow").with_retained_progress(p))?;self.phase=3},source::WalkTurn::Progress=>{}}}},
 3=>{p=Self::receipt(self.index.reserve(self.count,Self::child(g)).map_err(Self::failure)?);self.phase=4},
 4=>{p=Self::receipt(self.index.initialize(Self::child(g)).map_err(Self::failure)?);if self.index.initialized(){let root=self.snapshot();self.walk.start(root);self.phase=5}},
 5=>if self.index.pending(){if let Some(error)=self.index.step(){self.refuse(error)}}else{let(turn,used,error)=self.walk.advance(Self::child(g)).map_err(Self::failure)?;p=Self::receipt(used);if let Some(error)=error{self.refuse(error)}else{match turn{source::WalkTurn::Node(node)=>self.index.begin_key(0,&crate::layer_base(node).id,index::Target::Layer(node)),source::WalkTurn::Done=>self.phase=6,source::WalkTurn::Progress=>{}}}},
 6=>if self.index.pending(){if let Some(error)=self.index.step(){self.refuse(error)}}else if let Some((key,asset))=self.snapshot().assets.entry_at(self.asset){self.index.begin_key(1,key,index::Target::Asset(asset));self.asset+=1}else{self.phase=7},
 7=>{let root=self.snapshot();let slots:index::Slots<'static>=unsafe{std::mem::transmute(self.index.slots())};self.program_bytes=self.future_bytes();self.program=Some(Box::pin(program::check(root,slots)));p.retained_capacity_bytes=self.program_bytes;self.phase=8},
 8=>{let mut context=Context::from_waker(Waker::noop());if let Poll::Ready(result)=self.program.as_mut().ok_or_else(||fault("Drawing continuation is absent"))?.as_mut().poll(&mut context){self.done=true;self.error=result.err()}},_=>return Err(fault("Drawing admission phase is invalid").with_retained_progress(p))}Ok(self.outcome(p,canvas))}
 fn begin_close(&mut self){self.closing=true}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.program.is_some(){return Ok(RetirementDemand{copy_bytes:self.program_bytes.checked_mul(2).ok_or_else(||fault("Drawing continuation close overflow"))?,release_bytes:self.program_bytes,depth:1,..Default::default()})}if !self.index.terminal_is_empty(){return Self::nested(self.index.close_demand())}if !self.walk.terminal_is_empty(){return Self::nested(self.walk.close_demand()?)}Self::nested(RetainedCloneBinding::close_demand(&self.binding,body.saturating_sub(Self::OVERHEAD))?)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(fault("Drawing admission close has not begun"))}let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.program.is_some(){self.program=None;let bytes=std::mem::take(&mut self.program_bytes);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:bytes,..Default::default()}))}if !self.index.terminal_is_empty(){return self.index.close_step(Self::child(g)).map(|p|RetainedCloneStep::Progress(Self::receipt(p))).map_err(Self::failure)}if !self.walk.terminal_is_empty(){return self.walk.close_step(Self::child(g)).map(|p|RetainedCloneStep::Progress(Self::receipt(p))).map_err(Self::failure)}self.source=None;let step=RetainedCloneBinding::close_one(&mut self.binding,Self::child(g)).map_err(Self::failure)?;let p=Self::receipt(step.progress());Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.source.is_none()&&self.program.is_none()&&self.program_bytes==0&&self.index.terminal_is_empty()&&self.walk.terminal_is_empty()}
}
impl Drop for DrawingAdmissionCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Drawing admission retains original source or partial backing until funded close")}}
#[cfg(test)]#[path="🧪️tests.rs"]mod tests;

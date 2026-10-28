//! ✅️ Canonical Drawing admission retains the exact original typed source and every partial numeric owner.
use crate::snapshot::DrawingSnapshot;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_io_ports::{ChildAdmissionSource,ChildAdmissionAdvance,EmbeddedChildAdmission,EmbeddedChildAdmissionCursorV1,EmbeddedChildAdmitterV1,VectorCanvas};
use std::{future::Future,pin::Pin,task::{Context,Poll,Waker},mem::size_of_val};
type Text=semio_framework_value::paged::PagedUtf8<{usize::MAX}>;
#[path="🧵️source.rs"]mod source;
#[path="📇️index.rs"]mod index;
#[path="📋️program.rs"]mod program;
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

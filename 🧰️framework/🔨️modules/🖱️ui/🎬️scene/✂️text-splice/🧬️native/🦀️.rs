//! ✂️ Original text-splice composition retains bounded scalar frontiers under the actual caller.
/// 📖️ The provider retains this exact immutable scalar-text pair until the original composer is physically closed.
#[derive(Clone, Copy)]
pub struct TextSpliceView<'a>{pub start:u32,pub deleted:&'a str,pub insert:&'a str,pub before:&'a str,pub after:&'a str}
#[derive(Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub struct OriginalTextSplice{pub start:u32,pub deleted:String,pub insert:String,pub before:String,pub after:String}
/// 🚦️ Names what one composer turn did, so a caller never mistakes an unfunded turn for progress.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalTextSpliceAdvance{Unfunded,Progress,Ready}
#[derive(Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub enum OriginalTextSpliceComposition{Composed(OriginalTextSplice),Cancelled,Disjoint}

use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{RetainedWorkOutcome,ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::size_of;
#[path="📐️math.rs"]mod math;
#[path="🧵️advance.rs"]mod advance;
const HASH:u64=0xcbf29ce484222325;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original text splice requires its exact immutable pair and original receiver")}
fn extent()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"original text splice extent exceeds its native representation")}
fn construct<T>(build:impl FnOnce()->Result<(T,RetainedCloneProgress),ValueError>)->RetainedWorkOutcome<(T,RetainedCloneProgress)>{RetainedWorkOutcome{result:build(),work:1}}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Fence>()+2*size_of::<Cursor>()+size_of::<OriginalSourceBinding>()+16,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
#[derive(semio_framework_value::RetireOwned)]
struct Scalars{a:Vec<char>,b:Vec<char>,c:Vec<char>,d:Vec<char>,e:Vec<char>,f:Vec<char>,g:Vec<char>,h:Vec<char>}
impl Scalars{
 fn new()->Self{Self{a:Vec::new(),b:Vec::new(),c:Vec::new(),d:Vec::new(),e:Vec::new(),f:Vec::new(),g:Vec::new(),h:Vec::new()}}
 fn get(&self,i:usize)->Result<&Vec<char>,ValueError>{match i{0=>Ok(&self.a),1=>Ok(&self.b),2=>Ok(&self.c),3=>Ok(&self.d),4=>Ok(&self.e),5=>Ok(&self.f),6=>Ok(&self.g),7=>Ok(&self.h),_=>Err(invalid())}}
 fn get_mut(&mut self,i:usize)->Result<&mut Vec<char>,ValueError>{match i{0=>Ok(&mut self.a),1=>Ok(&mut self.b),2=>Ok(&mut self.c),3=>Ok(&mut self.d),4=>Ok(&mut self.e),5=>Ok(&mut self.f),6=>Ok(&mut self.g),7=>Ok(&mut self.h),_=>Err(invalid())}}
}
#[derive(semio_framework_value::RetireOwned)]
struct Owners{scalars:Scalars,result:Option<OriginalTextSpliceComposition>}
#[derive(Clone,Copy)]
struct Fence{parts:[(usize,usize);8],starts:[u32;2],context:usize}
impl Fence{
 fn new(a:TextSpliceView<'_>,b:TextSpliceView<'_>,context:usize)->Self{Self{parts:parts(a,b).map(|s|(s.as_ptr()as usize,s.len())),starts:[a.start,b.start],context}}
 fn matches(&self,a:TextSpliceView<'_>,b:TextSpliceView<'_>)->bool{self.starts==[a.start,b.start]&&self.parts==parts(a,b).map(|s|(s.as_ptr()as usize,s.len()))}
}
fn parts<'a>(a:TextSpliceView<'a>,b:TextSpliceView<'a>)->[&'a str;8]{[a.before,a.deleted,a.insert,a.after,b.before,b.deleted,b.insert,b.after]}
#[derive(Clone,Copy)]
struct Cursor{phase:u8,field:usize,byte:usize,index:usize,lengths:[usize;8],output_bytes:[usize;4],hash:u64,fingerprint:u64,at:usize,seen_before:usize,seen_after:usize,run:usize,next:usize,limits:[usize;4],lo:usize,hi:usize,original_hi:usize,typed_hi:usize,scan:usize}
impl Cursor{fn new()->Self{Self{phase:0,field:0,byte:0,index:0,lengths:[0;8],output_bytes:[0;4],hash:HASH,fingerprint:0,at:0,seen_before:0,seen_after:0,run:0,next:0,limits:[0;4],lo:0,hi:0,original_hi:0,typed_hi:0,scan:0}}}
/// 🧶️ One original composition keeps only scalar buffers and virtual text frontiers until its result and physical owners leave.
pub struct OriginalTextSpliceComposer{binding:Option<OriginalSourceBinding>,decoder:(u64,u64),fence:Fence,cursor:Cursor,owners:ControlledRetirement<Owners>,closing:bool}
impl OriginalTextSpliceComposer{
 pub fn birth_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>(),capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}
 /// 🛡️ SAFETY: the allocation uses the exact `Layout` of `Self`, is checked non-null, and is fully initialised by `write` before `Box::from_raw` takes sole ownership, so the box deallocates with the matching layout.
 pub fn admit(previous:TextSpliceView<'_>,next:TextSpliceView<'_>,context:usize,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  let _=cx.original_source_parent()?;if output.is_some(){return Err(invalid().into())}let d=Self::birth_demands();let binding=OriginalSourceBinding::borrow(cx);let fence=Fence::new(previous,next,context);
  admit_original_construction(cx,output,construction(d),|native|construct(||{let decoder=(native.ledger_identity(),native.retirement_recipient_identity().ok_or_else(invalid)?);let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(std::alloc::Layout::new::<Self>())}.cast::<Self>()).ok_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,"original text splice allocation refused"))?;Ok((unsafe{pointer.as_ptr().write(Self{binding:Some(binding),decoder,fence,cursor:Cursor::new(),owners:ControlledRetirement::retain(Owners{scalars:Scalars::new(),result:None}),closing:false});Box::from_raw(pointer.as_ptr())},receipt(d)))}))
 }
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.is_some_and(|b|b.matches(cx))||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid().into())}Ok(())}
 pub fn demands(&self,previous:TextSpliceView<'_>,next:TextSpliceView<'_>,_body:usize)->Result<RetirementDemand,ValueError>{
  if !self.fence.matches(previous,next)||self.closing{return Err(invalid())}let mut d=fixed();
  match self.cursor.phase{1=>d.capacity_bytes=self.cursor.lengths[self.cursor.field].checked_mul(size_of::<char>()).ok_or_else(extent)?,10=>d.capacity_bytes=self.cursor.output_bytes[self.cursor.field],_=>{}}Ok(d)
 }
 pub fn advance(&mut self,previous:TextSpliceView<'_>,next:TextSpliceView<'_>,cx:&mut StepContext<'_>)->Result<OriginalTextSpliceAdvance,OriginalTurnFailure>{
  self.validate(cx)?;if !self.fence.matches(previous,next)||self.closing{return Err(invalid().into())}if self.ready(){return Ok(OriginalTextSpliceAdvance::Ready)}if self.cursor.phase==15{return Err(invalid().into())}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original text splice was cancelled").into())}
  let d=self.demands(previous,next,cx.retained_grant().maximum_copy_bytes)?;let mut done=None;let ran=admit_original_construction(cx,&mut done,construction(d),|_|construct(||{self.step(previous,next)?;Ok(((),receipt(d)))}))?;Ok(if !ran{OriginalTextSpliceAdvance::Unfunded}else if self.ready(){OriginalTextSpliceAdvance::Ready}else{OriginalTextSpliceAdvance::Progress})
 }
 pub fn ready(&self)->bool{self.cursor.phase==14&&!self.closing}
 pub fn take_demands(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}let mut d=fixed();d.copy_bytes+=2*size_of::<OriginalTextSpliceComposition>();Ok(d)}
 pub fn take_into(&mut self,output:&mut Option<OriginalTextSpliceComposition>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if output.is_some(){return Err(invalid().into())}let d=self.take_demands()?;admit_original_construction(cx,output,construction(d),|_|construct(||{let result=self.owners.original_mut().and_then(|o|o.result.take()).ok_or_else(invalid)?;self.cursor.phase=15;Ok((result,receipt(d)))}))}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.owners.terminal_is_empty(){self.owners.next_demand(body)}else{Ok(fixed())}}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(true)}self.validate(cx)?;if !self.owners.terminal_is_empty(){let d=self.owners.next_demand(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|{let result=self.owners.step(g);RetainedWorkOutcome{result,work:self.owners.last_turn_work()}})?;if step.progress()!=RetainedCloneProgress::default(){self.closing=true;}return Ok(false)}
  let d=fixed();close_original_retained_step(cx,d,|_|{self.binding=None;self.closing=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}})?;Ok(self.terminal_is_empty())
 }
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.owners.terminal_is_empty()}
 pub fn release_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<Self>>>(),release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl Drop for OriginalTextSpliceComposer{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original text splice requires source-funded physical closure")}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

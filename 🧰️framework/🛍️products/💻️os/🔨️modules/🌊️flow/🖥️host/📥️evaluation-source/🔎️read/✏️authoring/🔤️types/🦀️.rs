//! 🔤️ Borrowed type tokens are compared and copied through bounded original byte events.
use super::{ports::PortTypes,compare::TextCompare};
use semio_framework_job::{OriginalConstructionDemand,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::{RetainedAllocationEffect,OriginalRetirementPoll,RetirementPollExit};
use semio_os_flow_host_source::{close_with_effect,close_with_poll,poll_outcome};
use super::storage::Construction;
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original port type text changed its retained frontier")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let d=fixed(bytes);if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(receipt(d))?;cx.consume_fuel(1);Ok(true)}
#[derive(Clone,Copy)]
pub(super) enum TextInput<'a>{Endpoint(&'a str,&'a str),Types(PortTypes<'a>)}
impl<'a> TextInput<'a>{
 fn raw(self,index:usize)->Option<&'a str>{match self{Self::Endpoint(node,port)=>match index{0=>Some(node),1=>Some("@"),2=>Some(port),_=>None},Self::Types(types)=>types.raw(index)}}
 fn split(self)->bool{matches!(self,Self::Types(_))}
 fn binding(self)->[usize;6]{fn pair(s:&str)->(usize,usize){(s.as_ptr()as usize,s.len())}match self{Self::Endpoint(a,b)=>{let(a,n)=pair(a);let(b,m)=pair(b);[1,a,n,b,m,0]},Self::Types(PortTypes::Empty)=>[2,0,0,0,0,0],Self::Types(PortTypes::Single(s))=>{let(p,n)=pair(s);[3,p,n,0,0,0]},Self::Types(PortTypes::Channels(s))=>[4,s.as_ptr()as usize,s.len(),0,0,0]}}
}
#[derive(Clone,Copy)]
struct Address{raw:usize,start:usize,end:usize}
impl Address{fn text<'a>(self,input:TextInput<'a>)->Result<&'a str,ValueError>{input.raw(self.raw).and_then(|s|s.get(self.start..self.end)).ok_or_else(invalid)}}
enum Segment{Working,Part(Address),End}
struct Segments{raw:usize,offset:usize,start:usize}
impl Segments{
 fn new()->Self{Self{raw:0,offset:0,start:0}}
 fn step(&mut self,input:TextInput<'_>,cx:&mut StepContext<'_>)->Result<Segment,OriginalTurnFailure>{
  let Some(source)=input.raw(self.raw)else{if paid(cx,size_of::<Self>())?{return Ok(Segment::End)}return Ok(Segment::Working)};
  let count=64.min(source.len().saturating_sub(self.offset));if !paid(cx,size_of::<Self>()+count)?{return Ok(Segment::Working)}let end=self.offset+count;
  while self.offset<end{let byte=source.as_bytes()[self.offset];self.offset+=1;if input.split()&&byte==b','{let result=Address{raw:self.raw,start:self.start,end:self.offset-1};self.start=self.offset;if result.start!=result.end{return Ok(Segment::Part(result))}}}
  if self.offset==source.len(){let result=Address{raw:self.raw,start:self.start,end:self.offset};self.raw+=1;self.offset=0;self.start=0;if !input.split()||result.start!=result.end{return Ok(Segment::Part(result))}}Ok(Segment::Working)
 }
 fn reset(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !paid(cx,size_of::<Self>())?{return Ok(false)}*self=Self::new();Ok(true)}
}
pub(super) struct TypeCompatibility{left:Segments,right:Segments,a:Option<Address>,b:Option<Address>,compare:TextCompare,matched:Option<bool>,saw_left:bool,saw_right:bool,reset_right:bool,done:Option<bool>}
impl TypeCompatibility{
 pub(super) fn new()->Self{Self{left:Segments::new(),right:Segments::new(),a:None,b:None,compare:TextCompare::new(),matched:None,saw_left:false,saw_right:false,reset_right:false,done:None}}
 pub(super) fn step(&mut self,left:PortTypes<'_>,right:PortTypes<'_>,cx:&mut StepContext<'_>)->Result<Option<bool>,OriginalTurnFailure>{
  if self.done.is_some(){return Ok(self.done)}let left=TextInput::Types(left);let right=TextInput::Types(right);
  if self.reset_right{if self.right.reset(cx)?{self.a=None;self.reset_right=false}return Ok(None)}
  if self.a.is_none(){match self.left.step(left,cx)?{Segment::Working=>{},Segment::Part(a)=>{self.a=Some(a);self.saw_left=true},Segment::End=>{self.done=Some(!self.saw_left)}}return Ok(self.done)}
  if self.b.is_none(){match self.right.step(right,cx)?{Segment::Working=>{},Segment::Part(b)=>{self.b=Some(b);self.saw_right=true},Segment::End=>{if !self.saw_right{self.done=Some(true)}else{self.reset_right=true}}}return Ok(self.done)}
  if self.matched.is_none(){self.matched=self.compare.step(self.a.unwrap().text(left)?,self.b.unwrap().text(right)?,cx)?;return Ok(None)}
  if self.compare.reset(cx)?{if self.matched.take().unwrap(){self.done=Some(true)}else{self.b=None}}Ok(self.done)
 }
}
pub(super) struct TypeTextWork{construction:Construction,binding:Option<[usize;6]>,cursor:Segments,part:Option<Address>,output:ManuallyDrop<Option<String>>,retirement:RetainedCloneClose,total:usize,parts:usize,offset:usize,phase:u8,closing:bool}
impl TypeTextWork{
 pub(super) fn new()->Self{Self{construction:Default::default(),binding:None,cursor:Segments::new(),part:None,output:ManuallyDrop::new(None),retirement:Default::default(),total:0,parts:0,offset:0,phase:0,closing:false}}
 pub(super) fn next_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{let _=copy;Ok(RetirementDemand{capacity_bytes:if self.phase==2{self.total}else{0},..fixed(256)})}
 pub(super) fn step(&mut self,input:TextInput<'_>,cx:&mut StepContext<'_>)->Result<Option<String>,OriginalTurnFailure>{
  if self.closing||self.binding.is_some_and(|b|b!=input.binding()){return Err(invalid().into())}match self.phase{
   0=>if paid(cx,size_of::<Self>())?{self.binding=Some(input.binding());self.phase=1},
   1=>match self.cursor.step(input,cx)?{Segment::Working=>{},Segment::Part(a)=>{let separator=usize::from(input.split()&&self.parts!=0);self.total=self.total.checked_add(a.end-a.start).and_then(|n|n.checked_add(separator)).ok_or_else(invalid)?;self.parts+=1},Segment::End=>{self.phase=2}},
   2=>{let d=self.next_demands(cx.retained_grant().maximum_copy_bytes)?;self.construction.run(cx,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:0,depth:1},||(||{let mut output=String::new();output.try_reserve_exact(self.total).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original port diagnostic text allocation refused"))?;let actual=output.capacity();*self.output=Some(output);self.phase=3;Ok(((),RetainedCloneProgress{retained_capacity_bytes:actual,..receipt(d)}))})())?;},
   3=>if self.cursor.reset(cx)?{self.parts=0;self.phase=4},
   4=>{if self.part.is_none(){match self.cursor.step(input,cx)?{Segment::Working=>{},Segment::Part(a)=>{self.part=Some(a);self.offset=0;self.phase=5},Segment::End=>self.phase=6}return Ok(None)}return Err(invalid().into())},
   5=>{let part=self.part.ok_or_else(invalid)?.text(input)?;let mut end=(self.offset+64).min(part.len());while !part.is_char_boundary(end){end-=1}let count=end-self.offset;let separator=usize::from(input.split()&&self.parts!=0&&self.offset==0);if paid(cx,count+separator+2*size_of::<usize>())?{let output=self.output.as_mut().ok_or_else(invalid)?;if separator!=0{output.push(',')}output.push_str(&part[self.offset..end]);self.offset=end;if end==part.len(){self.part=None;self.parts+=1;self.phase=4}}},
   6=>if paid(cx,size_of::<Option<String>>())?{self.phase=7;return Ok(self.output.take())},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub(super) fn begin_close(&mut self){self.closing=true}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(copy)}self.retirement.next_owner_demands::<String>(self.output.is_some(),&None,copy)}
 pub(super) fn close_step(&mut self,g:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}if !self.retirement.is_empty(){return poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect))}let turn=self.retirement.begin_granted(&mut self.output,effect);RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Complete(Default::default()))),work:turn.work}}
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.retirement.is_empty()}
}
impl Drop for TypeTextWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original port diagnostic text requires terminal closure")}}

//! 🌲️ Dynamic original trees advance through bounded probes, actual backing births and retained row publication.
use super::{NativeValueSource,NativeTreeKind,fixed,invalid,receipt};
use crate::{DslValue,Number,NativeDecodeControl,RetainedWorkOutcome,OriginalReceivingPrimary,RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits}};
use crate::{NativeReceiver,OriginalEffectReply,OriginalEffectReplyComplete};
use semio_framework_value_native_receiver_protocol::NativeAdvanceSelection;
use std::mem::{ManuallyDrop,size_of};
pub const NATIVE_TREE_MAXIMUM_DEPTH:usize=64;
const TEXT_FRAGMENT:usize=64;
fn allocate_vec<E>(count:usize,actual:&mut Option<usize>)->Result<Vec<E>,ValueError>{if actual.is_some(){return Err(invalid())}if count==0||size_of::<E>()==0{return Ok(Vec::new())}let layout=std::alloc::Layout::array::<E>(count).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"native vector layout overflow"))?;let pointer=unsafe{std::alloc::alloc(layout)}.cast::<E>();*actual=Some(if pointer.is_null(){0}else{layout.size()});if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"native vector backing allocation refused"))}Ok(unsafe{Vec::from_raw_parts(pointer,0,count)})}
fn allocate_string(bytes:usize,actual:&mut Option<usize>)->Result<String,ValueError>{allocate_vec::<u8>(bytes,actual).map(|vector|unsafe{String::from_utf8_unchecked(vector)})}
fn admit_in<V>(effect:&mut crate::RetainedAllocationEffect<'_>,demand:RetirementDemand,allocate:impl FnOnce(&mut Option<usize>)->Result<V,ValueError>)->Result<V,OriginalReceivingPrimary>{if !retained_clone_grant_admits(effect.grant(),demand){return Err(invalid().into())}allocate(effect.actual()).map_err(Into::into)}
pub trait NativeTreeValue:RetireOwned+Sized{
 fn null()->Self;
 fn boolean(value:bool)->Self;
 fn number(value:Number)->Result<Self,ValueError>;
 fn text(value:String)->Self;
 fn array(value:Vec<Self>)->Self;
 fn object(value:Vec<(String,Self)>)->Self;
 fn sorted_unique_objects()->bool;
 fn scalar_copy_bytes(kind:NativeTreeKind)->usize;
 fn depth_message()->&'static str;
 fn duplicate_message()->&'static str;
}
struct Frame<T:NativeTreeValue>{
 node:*const dyn NativeValueSource,kind:Option<NativeTreeKind>,phase:u8,count:usize,index:usize,bytes:usize,offset:usize,
 source_text:Option<*const str>,key_text:Option<*const str>,key_length:usize,key_offset:usize,
 text:Option<String>,key:Option<String>,array:Option<Vec<T>>,object:Option<Vec<(String,T)>>,pending:Option<T>,value:Option<T>,
 heap:usize,heap_start:usize,root:usize,candidate:usize,building:bool,compare_left:usize,compare_right:usize,compare_offset:usize,compare_next:u8,order:i8,
}
impl<T:NativeTreeValue> Frame<T>{
 fn new(node:*const dyn NativeValueSource)->Self{Self{node,kind:None,phase:0,count:0,index:0,bytes:0,offset:0,source_text:None,key_text:None,key_length:0,key_offset:0,text:None,key:None,array:None,object:None,pending:None,value:None,heap:0,heap_start:0,root:0,candidate:0,building:false,compare_left:0,compare_right:0,compare_offset:0,compare_next:0,order:0}}
 fn empty(&self)->bool{self.text.is_none()&&self.key.is_none()&&self.array.is_none()&&self.object.is_none()&&self.pending.is_none()&&self.value.is_none()}
 fn compare(&mut self,left:usize,right:usize,next:u8){self.compare_left=left;self.compare_right=right;self.compare_offset=0;self.compare_next=next;self.phase=50;}
}
fn fragment(text:&str,offset:usize)->usize{let mut end=offset.saturating_add(TEXT_FRAGMENT).min(text.len());while !text.is_char_boundary(end){end-=1;}end-offset}
pub struct NativeTreeReceiver<I:NativeValueSource+?Sized,T:NativeTreeValue>{binding:Option<RetainedCloneBinding>,frames:ManuallyDrop<[Option<Frame<T>>;NATIVE_TREE_MAXIMUM_DEPTH+1]>,depth:usize,output:ManuallyDrop<Option<T>>,retirement:RetainedCloneClose,closing:bool,taken:bool,complete:bool,failure:Option<(ValueRefusalKind,&'static str)>,input:std::marker::PhantomData<fn(&I)>}
unsafe impl<I:NativeValueSource+?Sized,T:NativeTreeValue> Send for NativeTreeReceiver<I,T>{}
impl<I:NativeValueSource+?Sized,T:NativeTreeValue> NativeTreeReceiver<I,T>{
 pub fn new()->Self{Self{binding:None,frames:ManuallyDrop::new(std::array::from_fn(|_|None)),depth:0,output:ManuallyDrop::new(None),retirement:Default::default(),closing:false,taken:false,complete:false,failure:None,input:std::marker::PhantomData}}
 fn top(&self)->Option<&Frame<T>>{self.frames[self.depth].as_ref()}
 fn failed(&self)->Option<ValueError>{self.failure.map(|(kind,message)|ValueError::literal(kind,message))}
 fn frame_close(&self,body:usize)->Result<RetirementDemand,ValueError>{let Some(f)=self.top()else{return Ok(fixed(0))};if f.value.is_some()||f.pending.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}if f.key.is_some()||f.text.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,body)}if f.array.is_some(){return self.retirement.next_owner_demands::<Vec<T>>(true,&None,body)}if f.object.is_some(){return self.retirement.next_owner_demands::<Vec<(String,T)>>(true,&None,body)}Ok(fixed(0))}
}
impl<I:NativeValueSource+?Sized,T:NativeTreeValue> Default for NativeTreeReceiver<I,T>{fn default()->Self{Self::new()}}
impl<I:NativeValueSource+?Sized,T:NativeTreeValue> NativeReceiver<I,T> for NativeTreeReceiver<I,T>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,_:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.taken||self.complete{return Err(invalid())}if self.failure.is_some(){return Ok(Default::default())}if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}
  let Some(f)=self.top()else{return Ok(fixed(0))};let mut d=fixed(0);
  match f.phase{
   1=>d.copy_bytes=T::scalar_copy_bytes(f.kind.ok_or_else(invalid)?),
   3 if f.text.is_none()=>d.capacity_bytes=f.count,
   4=>{let text=unsafe{&*f.source_text.ok_or_else(invalid)?};d.copy_bytes=fragment(text,f.offset);},
   6 if f.array.is_none()=>d.capacity_bytes=f.bytes,
   10 if f.object.is_none()=>d.capacity_bytes=f.bytes,
   12 if f.key.is_none()=>d.capacity_bytes=f.key_length,
   13=>{let text=unsafe{&*f.key_text.ok_or_else(invalid)?};d.copy_bytes=fragment(text,f.key_offset);},
   17=>d.copy_bytes=if f.kind==Some(NativeTreeKind::Array){size_of::<T>()}else{size_of::<(String,T)>()},
   _=>{}
  }Ok(d)
 }
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,_body:usize,maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{
  if self.closing||self.taken||self.complete{return Err(invalid())}if let Some(error)=self.failed(){return Err(error)}
  let work=if input.check_binding(&self.binding)?{1}else{input.binding_work_demand(&self.binding)?};if work>maximum_work_units{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native tree action exceeds its original work ceiling"))}Ok(NativeAdvanceSelection::Local{work})
 }
 fn advance_nested(&mut self,_:RetainedCloneRef<'_,I>,_:&mut NativeDecodeControl<'_>,_:usize,_:u64,_:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let g=effect.grant();let maximum_work_units=effect.maximum_work_units();
  let mut own=RetainedCloneProgress::default();let result:Result<RetainedCloneStep,OriginalReceivingPrimary>=(||{
   let d=self.demands(input,body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())).map_err(Into::into)}
   if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native tree action exceeded its original work").into())}
   if self.top().is_none(){self.frames[0]=Some(Frame::new(input.get().as_source()as*const dyn NativeValueSource));return Ok(RetainedCloneStep::Progress(receipt(fixed(0))))}
   if self.top().unwrap().phase==20{
    let frame=self.frames[self.depth].as_ref().unwrap();if frame.value.is_none()||frame.text.is_some()||frame.key.is_some()||frame.array.is_some()||frame.object.is_some()||frame.pending.is_some()||self.depth>0&&self.frames[self.depth-1].as_ref().unwrap().pending.is_some(){return Err(invalid().into())}own=receipt(fixed(0));if self.depth==0{let value=self.frames[0].as_mut().unwrap().value.take().ok_or_else(invalid)?;*self.output=Some(value);self.complete=true;return Ok(RetainedCloneStep::Complete(own))}
    let value=self.frames[self.depth].as_mut().unwrap().value.take().ok_or_else(invalid)?;self.frames[self.depth]=None;self.depth-=1;let parent=self.frames[self.depth].as_mut().unwrap();parent.pending=Some(value);parent.phase=17;return Ok(RetainedCloneStep::Progress(own))
   }
   let f=self.frames[self.depth].as_mut().unwrap();let node=unsafe{&*f.node};
   if matches!(f.phase,3|6|10|12){
    match f.phase{3 if f.text.is_none()=>{let d=RetirementDemand{copy_bytes:size_of::<String>(),capacity_bytes:f.count,depth:1,..Default::default()};f.text=Some(admit_in(effect,d,|actual|allocate_string(f.count,actual))?);return Ok(RetainedCloneStep::Progress(receipt(d)))},6 if f.array.is_none()=>{let d=RetirementDemand{copy_bytes:size_of::<Vec<T>>(),capacity_bytes:f.bytes,depth:1,..Default::default()};f.array=Some(admit_in(effect,d,|actual|allocate_vec::<T>(f.count,actual))?);return Ok(RetainedCloneStep::Progress(receipt(d)))},10 if f.object.is_none()=>{let d=RetirementDemand{copy_bytes:size_of::<Vec<(String,T)>>(),capacity_bytes:f.bytes,depth:1,..Default::default()};f.object=Some(admit_in(effect,d,|actual|allocate_vec::<(String,T)>(f.count,actual))?);return Ok(RetainedCloneStep::Progress(receipt(d)))},12 if f.key.is_none()=>{let d=RetirementDemand{copy_bytes:size_of::<String>(),capacity_bytes:f.key_length,depth:1,..Default::default()};f.key=Some(admit_in(effect,d,|actual|allocate_string(f.key_length,actual))?);return Ok(RetainedCloneStep::Progress(receipt(d)))},_=>{}}
   }
   match f.phase{
    0=>{let Some(kind)=node.tree_kind()else{self.failure=Some((ValueRefusalKind::InvalidValue,"native receiving original type, field or phase is invalid"));own=receipt(fixed(0));return Err(invalid().into())};f.kind=Some(kind);f.phase=match kind{NativeTreeKind::Null|NativeTreeKind::Bool|NativeTreeKind::Number=>1,NativeTreeKind::Text=>2,NativeTreeKind::Array=>5,NativeTreeKind::Object=>9};},
    1=>{let value=match f.kind.unwrap(){NativeTreeKind::Null=>T::null(),NativeTreeKind::Bool=>T::boolean(node.boolean()?),NativeTreeKind::Number=>T::number(node.number()?)?,_=>return Err(invalid().into())};f.value=Some(value);f.phase=20;own.copied_bytes=d.copy_bytes;},
    2=>{let text=node.text()?;f.count=text.len();f.source_text=Some(text.as_str()as*const str);f.phase=3;},
    3=>f.phase=4,
    4=>{let text=unsafe{&*f.source_text.ok_or_else(invalid)?};if f.offset==f.count{f.value=Some(T::text(f.text.take().ok_or_else(invalid)?));f.phase=20;}else{let len=fragment(text,f.offset);let output=f.text.as_mut().ok_or_else(invalid)?;if output.capacity()-output.len()<len{return Err(invalid().into())}output.push_str(&text[f.offset..f.offset+len]);f.offset+=len;own.copied_bytes=len;}},
    5=>{let count=node.sequence_len()?;let bytes=count.checked_mul(size_of::<T>()).filter(|n|*n<=isize::MAX as usize).ok_or_else(invalid)?;f.count=count;f.bytes=bytes;f.phase=6;},
    6=>f.phase=7,
    7|14=>{if f.index==f.count{if f.kind==Some(NativeTreeKind::Array){f.value=Some(T::array(f.array.take().ok_or_else(invalid)?));f.phase=20;}else{f.phase=16;}}else{let child=if f.kind==Some(NativeTreeKind::Array){node.sequence_child(f.index)?}else{node.record_child(f.index)?};if self.depth==NATIVE_TREE_MAXIMUM_DEPTH{self.failure=Some((ValueRefusalKind::DepthLimit,T::depth_message()));own=receipt(fixed(0));return Err(ValueError::literal(ValueRefusalKind::DepthLimit,T::depth_message()).into())}f.phase=18;self.depth+=1;self.frames[self.depth]=Some(Frame::new(child as*const dyn NativeValueSource));}},
    9=>{let count=node.record_len()?;let bytes=count.checked_mul(size_of::<(String,T)>()).filter(|n|*n<=isize::MAX as usize).ok_or_else(invalid)?;f.count=count;f.bytes=bytes;f.phase=10;},
    10=>f.phase=11,
    11=>{if f.index==f.count{f.phase=16;}else{let text=node.record_name(f.index)?;f.key_length=text.len();f.key_offset=0;f.key_text=Some(text as*const str);f.phase=12;}},
    12=>f.phase=13,
    13=>{let text=unsafe{&*f.key_text.ok_or_else(invalid)?};if f.key_offset==f.key_length{f.phase=14;}else{let len=fragment(text,f.key_offset);let output=f.key.as_mut().ok_or_else(invalid)?;if output.capacity()-output.len()<len{return Err(invalid().into())}output.push_str(&text[f.key_offset..f.key_offset+len]);f.key_offset+=len;own.copied_bytes=len;}},
    16=>{if T::sorted_unique_objects()&&f.count>1{f.heap=f.count;f.heap_start=f.count/2;f.building=true;f.phase=30;}else{f.phase=41;}},
    17=>{if f.kind==Some(NativeTreeKind::Array){let rows=f.array.as_mut().ok_or_else(invalid)?;if size_of::<T>()!=0&&rows.len()==rows.capacity(){return Err(invalid().into())}rows.push(f.pending.take().ok_or_else(invalid)?);f.phase=7;}else{let rows=f.object.as_mut().ok_or_else(invalid)?;if rows.len()==rows.capacity(){return Err(invalid().into())}if f.key.is_none()||f.pending.is_none(){return Err(invalid().into())}rows.push((f.key.take().ok_or_else(||ValueError::invariant("key absent"))?,f.pending.take().ok_or_else(||ValueError::invariant("pending absent"))?));f.key_text=None;f.phase=11;}f.index+=1;own.copied_bytes=d.copy_bytes;},
    30=>{if f.heap_start==0{f.building=false;f.phase=35;}else{f.heap_start-=1;f.root=f.heap_start;f.phase=31;}},
    31=>{let left=f.root.checked_mul(2).and_then(|n|n.checked_add(1)).ok_or_else(invalid)?;if left>=f.heap{f.phase=if f.building{30}else{35};}else{f.candidate=left;if left+1<f.heap{f.compare(left,left+1,32);}else{f.phase=33;}}},
    32=>{if f.order<0{f.candidate+=1;}f.phase=33;},
    33=>f.compare(f.root,f.candidate,34),
    34=>{if f.order<0{f.object.as_mut().ok_or_else(||ValueError::invariant("object absent"))?.swap(f.root,f.candidate);f.root=f.candidate;f.phase=31;}else{f.phase=if f.building{30}else{35};}},
    35=>{if f.heap<=1{f.index=1;f.phase=40;}else{f.heap-=1;f.object.as_mut().ok_or_else(||ValueError::invariant("object absent"))?.swap(0,f.heap);f.root=0;f.phase=31;}},
    40=>{if f.index>=f.count{f.phase=41;}else{f.compare(f.index-1,f.index,42);}},
    41=>{f.value=Some(T::object(f.object.take().ok_or_else(invalid)?));f.phase=20;},
    42=>{if f.order==0{self.failure=Some((ValueRefusalKind::InvalidValue,T::duplicate_message()));own=receipt(fixed(0));return Err(ValueError::literal(ValueRefusalKind::InvalidValue,T::duplicate_message()).into())}f.index+=1;f.phase=40;},
    50=>{let rows=f.object.as_ref().ok_or_else(invalid)?;let left=rows[f.compare_left].0.as_bytes();let right=rows[f.compare_right].0.as_bytes();let mut offset=f.compare_offset;let end=offset.saturating_add(TEXT_FRAGMENT).min(left.len().min(right.len()));let mut order=std::cmp::Ordering::Equal;while offset<end{order=left[offset].cmp(&right[offset]);if order!=std::cmp::Ordering::Equal{break}offset+=1;}if order==std::cmp::Ordering::Equal&&offset==left.len().min(right.len()){order=left.len().cmp(&right.len());}if order!=std::cmp::Ordering::Equal||offset==left.len().min(right.len()){f.order=match order{std::cmp::Ordering::Less=>-1,std::cmp::Ordering::Equal=>0,std::cmp::Ordering::Greater=>1};f.phase=f.compare_next;}else{f.compare_offset=offset;}},
    _=>return Err(invalid().into()),
   }
   own.copied_items=1;let step=RetainedCloneStep::Progress(own);own=Default::default();Ok(step)
  })();let result=result.map_err(|mut error|{if own!=RetainedCloneProgress::default(){error.progress=error.progress.saturating_add(own);}error});Ok(reply.complete(result))
 }
 fn output_ready(&self)->bool{!self.closing&&self.complete&&self.output.is_some()}
 semio_framework_value_native_receiver_protocol::native_owned_output_state!(T,invalid,taken,true);
 fn begin_close(&mut self){self.closing=true;}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.top().is_some(){return self.frame_close(body)}if self.output.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}if self.failure.is_some(){return Ok(fixed(0))}RetainedCloneBinding::close_demand(&self.binding,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<()>(false,&None,body)}if let Some(f)=self.top(){if f.value.is_some()||f.pending.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if f.key.is_some()||f.text.is_some(){return self.retirement.next_owner_work::<String>(true,&None,body)}if f.array.is_some(){return self.retirement.next_owner_work::<Vec<T>>(true,&None,body)}if f.object.is_some(){return self.retirement.next_owner_work::<Vec<(String,T)>>(true,&None,body)}return Ok(1)}if self.output.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.failure.is_some(){return Ok(1)}RetainedCloneBinding::close_work_demand(&self.binding,body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{let g=effect.grant();
  let mut work=0;let result:Result<RetainedCloneStep,crate::OriginalRetirementFailure>=(||{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if !self.retirement.is_empty(){let turn=self.retirement.step_granted(effect);work=turn.work();return match turn{crate::RetainedWorkPoll::Complete(done)=>done.result.map(|s|RetainedCloneStep::Progress(s.progress())),crate::RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default()))}}
   if let Some(f)=self.frames[self.depth].as_mut(){let turn=if f.value.is_some(){Some(self.retirement.begin_granted(&mut f.value,effect))}else if f.pending.is_some(){Some(self.retirement.begin_granted(&mut f.pending,effect))}else if f.key.is_some(){Some(self.retirement.begin_granted(&mut f.key,effect))}else if f.text.is_some(){Some(self.retirement.begin_granted(&mut f.text,effect))}else if f.array.is_some(){Some(self.retirement.begin_granted(&mut f.array,effect))}else if f.object.is_some(){Some(self.retirement.begin_granted(&mut f.object,effect))}else{None};if let Some(turn)=turn{work=turn.work;return turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)}self.frames[self.depth]=None;if self.depth>0{self.depth-=1;}work=1;return Ok(RetainedCloneStep::Progress(receipt(fixed(0))))}
   if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);work=turn.work;return turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)}
   if self.failure.take().is_some(){work=1;return Ok(RetainedCloneStep::Progress(receipt(fixed(0))))}
   let turn=RetainedCloneBinding::close_one(&mut self.binding,effect);work+=turn.work();match turn{crate::RetainedWorkPoll::Complete(done)=>done.result,crate::RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default()))}
  })();crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})
 }
 fn requires_original_birth(&self)->bool{self.retirement.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementPoll{self.retirement.step_original_birth(original,grant)}
 fn requires_original_control(&self)->bool{self.retirement.requires_original_control()}
 fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{if self.requires_original_birth(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"settle original birth before opening a close effect").into()),work:0})}if self.retirement.requires_original_control(){return self.retirement.step_original(original,effect)}self.close_step(effect)}
 fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.frames.iter().all(Option::is_none)&&self.output.is_none()&&self.retirement.is_empty()&&self.failure.is_none()}
}
impl<I:NativeValueSource+?Sized,T:NativeTreeValue> Drop for NativeTreeReceiver<I,T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"native tree receiving requires funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.frames);ManuallyDrop::drop(&mut self.output);}}}}
impl NativeTreeValue for DslValue{
 fn null()->Self{Self::Null}fn boolean(v:bool)->Self{Self::Bool(v)}fn number(v:Number)->Result<Self,ValueError>{Ok(Self::Number(v))}fn text(v:String)->Self{Self::String(v)}fn array(v:Vec<Self>)->Self{Self::Array(v)}fn object(v:Vec<(String,Self)>)->Self{Self::Object(v)}fn sorted_unique_objects()->bool{false}fn scalar_copy_bytes(kind:NativeTreeKind)->usize{match kind{NativeTreeKind::Bool=>size_of::<bool>(),NativeTreeKind::Number=>size_of::<Number>(),_=>0}}fn depth_message()->&'static str{"native dynamic subtree exceeds depth limit"}fn duplicate_message()->&'static str{"native dynamic subtree repeats a key"}
}


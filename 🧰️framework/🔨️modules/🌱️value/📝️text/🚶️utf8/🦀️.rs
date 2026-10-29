//! 🚶️ Original immutable UTF8 pages retain scalar positions and funded output allocation custody.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn page(source:&str,start:usize,body:usize)->Result<usize,ValueError>{if start>source.len()||!source.is_char_boundary(start){return Err(invalid("UTF8 original source boundary changed"));}let mut end=start+body.min(source.len()-start);while end>start&&!source.is_char_boundary(end){end-=1;}Ok(end)}

/// 🧭️ The caller retains immutable source/revision custody; this zero-heap cursor lends funded ranges.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct RetainedUtf8Cursor{byte:usize,scalar:usize,ready:bool}
/// 🧾️ A byte/scalar range is available only after its actual bounded inspection was funded.
#[derive(Clone,Copy,Debug)]
pub struct RetainedUtf8CursorStep{pub start_byte:usize,pub end_byte:usize,pub start_scalar:usize,pub end_scalar:usize,pub progress:RetainedCloneProgress}
impl RetainedUtf8Cursor{
 pub fn new()->Self{Self::default()}
 pub fn ready(&self)->bool{self.ready}
 pub fn byte_offset(&self)->usize{self.byte}
 pub fn scalar_offset(&self)->usize{self.scalar}
 pub fn fixed_copy_bytes()->usize{size_of::<Self>()*2+size_of::<usize>()*2}
 pub fn demand(&self,source:&str)->Result<RetirementDemand,ValueError>{if self.ready{return Ok(Default::default());}page(source,self.byte,0)?;Ok(RetirementDemand{copy_bytes:Self::fixed_copy_bytes()+source[self.byte..].chars().next().map_or(0,char::len_utf8),depth:1,..Default::default()})}
 pub fn advance(&mut self,source:&str,grant:RetainedCloneGrant)->Result<RetainedUtf8CursorStep,ValueError>{
  let mut step=RetainedUtf8CursorStep{start_byte:self.byte,end_byte:self.byte,start_scalar:self.scalar,end_scalar:self.scalar,progress:Default::default()};if self.ready||!retained_clone_grant_admits(grant,self.demand(source)?){return Ok(step);}let end=page(source,self.byte,grant.maximum_copy_bytes-Self::fixed_copy_bytes())?;let count=source[self.byte..end].chars().count();let scalars=self.scalar.checked_add(count).ok_or_else(||invalid("UTF8 scalar coordinate overflow"))?;self.byte=end;self.scalar=scalars;self.ready=end==source.len();step.end_byte=end;step.end_scalar=scalars;step.progress=RetainedCloneProgress{copied_items:1,copied_bytes:Self::fixed_copy_bytes()+end-step.start_byte,..Default::default()};Ok(step)
 }
}

/// 🧳️ One declared String allocation appends only funded UTF8 pages and closes through original retirement.
pub struct RetainedUtf8Buffer{output:ManuallyDrop<Option<String>>,capacity:usize,reserved:bool,spent:bool,closing:bool,retirement:RetainedCloneClose}
/// 🪡️ An append returns the next original byte coordinate beside its actual receipt.
#[derive(Clone,Copy,Debug)]
pub struct RetainedUtf8Append{pub end_byte:usize,pub progress:RetainedCloneProgress}
impl RetainedUtf8Buffer{
 pub fn new(capacity:usize)->Self{Self{output:ManuallyDrop::new(None),capacity,reserved:false,spent:false,closing:false,retirement:Default::default()}}
 pub fn capacity(&self)->usize{self.capacity}
 pub fn len(&self)->usize{self.output.as_ref().map_or(0,String::len)}
 pub fn original(&self)->Option<&str>{self.output.as_deref()}
 pub fn reserve_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<String>()*2+size_of::<usize>()*2,capacity_bytes:self.capacity,depth:1,..Default::default()}}
 pub fn reserve(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.closing||self.spent{return Err(invalid("UTF8 buffer is closing or spent"));}if self.reserved{return Ok(Default::default());}let demand=self.reserve_demand();if !retained_clone_grant_admits(grant,demand){return Ok(Default::default());}let mut output=String::new();output.try_reserve_exact(self.capacity).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"UTF8 original buffer reservation failed"))?;let bytes=output.capacity();*self.output=Some(output);self.reserved=true;let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:bytes,..Default::default()};if bytes>grant.maximum_capacity_bytes{return Err(invalid("UTF8 allocator exceeded admitted capacity").with_retained_progress(progress));}Ok(progress)
 }
 pub fn append_demand(&self,source:&str,start:usize)->Result<RetirementDemand,ValueError>{if !self.reserved||self.spent||self.closing{return Err(invalid("UTF8 buffer lacks original output custody"));}page(source,start,0)?;Ok(RetirementDemand{copy_bytes:size_of::<String>()*2+size_of::<usize>()*2+source[start..].chars().next().map_or(0,char::len_utf8),depth:1,..Default::default()})}
 pub fn append(&mut self,source:&str,start:usize,grant:RetainedCloneGrant)->Result<RetainedUtf8Append,ValueError>{
  let demand=self.append_demand(source,start)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedUtf8Append{end_byte:start,progress:Default::default()});}let fixed=size_of::<String>()*2+size_of::<usize>()*2;let end=page(source,start,grant.maximum_copy_bytes-fixed)?;let output=self.output.as_mut().ok_or_else(||invalid("UTF8 original output disappeared"))?;let length=output.len().checked_add(end-start).ok_or_else(||invalid("UTF8 output extent overflow"))?;if length>self.capacity{return Err(invalid("UTF8 output exceeds declared original capacity"));}output.push_str(&source[start..end]);Ok(RetainedUtf8Append{end_byte:end,progress:RetainedCloneProgress{copied_items:1,copied_bytes:fixed+end-start,..Default::default()}})
 }
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.reserved||self.spent||self.closing{return Err(invalid("UTF8 transfer lacks original output custody"));}Ok(RetirementDemand{copy_bytes:size_of::<String>()*2+size_of::<bool>()*2,depth:1,..Default::default()})}
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(String,RetainedCloneProgress)>,ValueError>{let demand=self.take_demand()?;if !retained_clone_grant_admits(grant,demand){return Ok(None);}let value=self.output.take().ok_or_else(||invalid("UTF8 original transfer disappeared"))?;self.spent=true;Ok(Some((value,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.retirement.next_owner_demands::<String>(self.output.is_some(),&None,body)}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(invalid("UTF8 original close has not begun"));}let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.retirement.is_empty(){return self.retirement.step_granted(grant);}if let Some(step)=self.retirement.begin_granted(&mut self.output,grant)?{return Ok(step);}Ok(RetainedCloneStep::Complete(Default::default()))}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.retirement.is_empty()}
}
impl Drop for RetainedUtf8Buffer{fn drop(&mut self){let empty=self.output.is_none()&&self.retirement.is_empty();assert!(std::thread::panicking()||empty,"UTF8 original buffer requires funded child closure");if empty{unsafe{ManuallyDrop::drop(&mut self.output);}}}}



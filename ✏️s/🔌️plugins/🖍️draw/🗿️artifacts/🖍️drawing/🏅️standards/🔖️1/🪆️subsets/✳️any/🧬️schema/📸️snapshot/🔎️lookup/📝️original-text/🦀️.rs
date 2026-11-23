//! 📝️ Sparse Drawing diff strings receive original paged UTF8 directly under their declared physical capacity and copy grants.
pub type DrawingNativeText=semio_framework_value::paged::PagedUtf8<{usize::MAX}>;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose,retained_clone_grant_admits,close_retained_binding}};
use semio_framework_value::RetainedAllocationEffect;
use super::original::close_result;
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Drawing original text receiver custody changed")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
/// 🧳️ One original native text binding retains every chunk through exact output transfer or bounded retirement.
pub struct DrawingOriginalText{binding:Option<RetainedCloneBinding>,output:ManuallyDrop<Option<String>>,retirement:RetainedCloneClose,chunk:usize,offset:usize,phase:u8,closing:bool}
impl Default for DrawingOriginalText{fn default()->Self{Self{binding:None,output:ManuallyDrop::new(None),retirement:Default::default(),chunk:0,offset:0,phase:0,closing:false}}}
impl DrawingOriginalText{
 fn metadata()->usize{2*size_of::<Option<String>>()+2*size_of::<usize>()+size_of::<u8>()}
 fn span(&self,original:RetainedCloneRef<'_,DrawingNativeText>,body:usize)->Result<usize,ValueError>{let Some(chunk)=original.get().retained_chunks().get(self.chunk)else{return Ok(0)};if self.offset>chunk.len()||!chunk.is_char_boundary(self.offset){return Err(invalid())}let remaining=&chunk[self.offset..];let Some(first)=remaining.chars().next()else{return Ok(0)};let mut bytes=remaining.len().min(body.saturating_sub(Self::metadata()).max(first.len_utf8()));while !remaining.is_char_boundary(bytes){bytes-=1}Ok(bytes)}
 pub fn demand(&self,original:RetainedCloneRef<'_,DrawingNativeText>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invalid())}if !original.check_binding(&self.binding)?{return Ok(fixed(original.binding_copy_bytes()))}Ok(match self.phase{0=>RetirementDemand{capacity_bytes:original.get().len(),..fixed(Self::metadata())},1=>fixed(Self::metadata()+self.span(original,body)?),2=>fixed(2*size_of::<Option<String>>()+size_of::<u8>()),_=>return Err(invalid())})}
 pub fn advance(&mut self,original:RetainedCloneRef<'_,DrawingNativeText>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{let g=effect.grant();let d=self.demand(original,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(p)=original.bind(&mut self.binding,effect).result?{return Ok(RetainedCloneStep::Progress(p))}if self.phase==2{return Ok(RetainedCloneStep::Complete(Default::default()))}if effect.remaining_work_units()?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Drawing lookup exceeds admitted work"))}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};match self.phase{
  0=>{let length=original.get().len();if length!=0&&effect.actual().is_some(){return Err(invalid())}self.phase=254;*self.output=Some(String::new());let output=self.output.as_mut().ok_or_else(invalid)?;if length!=0{*effect.actual()=Some(0)}let result=output.try_reserve_exact(length);p.retained_capacity_bytes=output.capacity();if length!=0{*effect.actual()=Some(p.retained_capacity_bytes)}if result.is_err()||!p.fits(g){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"Drawing original diff text allocation refused").with_retained_progress(p))}self.phase=1},
  1=>{let Some(chunk)=original.get().retained_chunks().get(self.chunk)else{self.phase=2;return Ok(RetainedCloneStep::Complete(p))};let bytes=self.span(original,body)?;let end=self.offset+bytes;let output=self.output.as_mut().ok_or_else(invalid)?;if output.len()+bytes>output.capacity(){return Err(invalid().with_retained_progress(p))}output.push_str(&chunk[self.offset..end]);self.offset=end;if self.offset==chunk.len(){self.chunk+=1;self.offset=0}},_=>return Err(invalid()),
 }Ok(RetainedCloneStep::Progress(p))}
 pub fn ready(&self)->bool{!self.closing&&self.phase==2&&self.output.is_some()}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}Ok(fixed(2*size_of::<Option<String>>()+size_of::<u8>()))}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<(Option<String>,RetainedCloneProgress),ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok((None,Default::default()))}self.phase=3;Ok((self.output.take(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn advance_work_demand(&self,original:RetainedCloneRef<'_,DrawingNativeText>)->Result<u64,ValueError>{
    if self.closing{return Err(invalid())}
    if !original.check_binding(&self.binding)?{return original.binding_work_demand(&self.binding)}
    Ok(u64::from(self.phase<2))
 }
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
    if !self.retirement.is_empty()||self.output.is_some(){
        return self.retirement.next_owner_work::<String>(self.output.is_some(),&None,body)
    }
    RetainedCloneBinding::close_work_demand(&self.binding,body)
 }
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<String>(false,&None,body)}if self.output.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,body)}RetainedCloneBinding::close_demand(&self.binding,body)}
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{let g=effect.grant();if !self.closing{return Err(invalid())}let d=self.close_demand(body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.retirement.is_empty(){return close_result(self.retirement.step_granted(body,effect)).map(|s|RetainedCloneStep::Progress(s.progress()))}if let Some(step)=self.retirement.begin_granted(&mut self.output,effect).result?{return Ok(RetainedCloneStep::Progress(step.progress()))}close_result(close_retained_binding(&mut self.binding,body,effect))}
 pub fn terminal_is_empty(&self)->bool{self.output.is_none()&&self.binding.is_none()&&self.retirement.is_empty()}
}
impl Drop for DrawingOriginalText{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Drawing original text retains its source or partial output");}}

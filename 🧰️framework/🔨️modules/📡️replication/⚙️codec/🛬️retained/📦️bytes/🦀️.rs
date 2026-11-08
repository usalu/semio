//! 📦️ Original binary fields receive and return canonical pages under their unchanged source custody.
use super::{OriginalWireSourceIdentity,RetainedWireByteSource,RetainedWireFieldStep};
use semio_framework_value::{paged::PagedBytes,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use std::mem::ManuallyDrop;
type Bytes=PagedBytes<{isize::MAX as usize}>;
/// 🧵️ One immutable original range retains every admitted prefix until a paid recipient or cancellation closes it.
pub struct RetainedWireBytesDecode{last_work:u64,source:OriginalWireSourceIdentity,start:usize,length:usize,output:ManuallyDrop<Option<Bytes>>,retiring:ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<Bytes>>>,receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,closing:bool,closed:bool}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn narrow(work:u128)->Result<u64,ValueError>{u64::try_from(work).map_err(|_|invalid("original wire field work exceeds its counter"))}
fn pay(effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<u64,ValueError>{match semio_framework_value::retirement::record_retirement_work(effect,1){Ok(work)=>narrow(work),Err((error,_))=>Err(error)}}
fn funded(g:RetainedCloneGrant,d:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(g,d)}
impl RetainedWireBytesDecode{
 /// 📐️ Quotes the exact inline source and destination headers before their enclosing original capture.
 pub fn birth_demand()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 /// 📍️ Binds one checked range without manufacturing a source, decoder, grant or destination alias.
 pub fn new_range<S:RetainedWireByteSource+?Sized>(source:&S,range:(usize,usize))->Result<Self,ValueError>{let identity=source.original_source_identity();let length=range.1.checked_sub(range.0).filter(|_|range.1<=identity.extent).ok_or_else(||invalid("original paged byte range escaped its source"))?;Ok(Self{last_work:0,source:identity,start:range.0,length,output:ManuallyDrop::new(Some(Bytes::default())),retiring:ManuallyDrop::new(None),receipt:None,closing:false,closed:false})}
 fn validate<S:RetainedWireByteSource+?Sized>(&self,source:&S)->Result<(),ValueError>{if self.source!=source.original_source_identity(){return Err(invalid("original paged byte source identity changed"))}Ok(())}
 pub fn last_turn_work(&self)->u64{self.last_work}
 /// 📏️ Quotes a physical page admission or one bounded prefix from that same original range.
 pub fn demands<S:RetainedWireByteSource+?Sized>(&self,source:&S,body:usize)->Result<RetirementDemand,ValueError>{self.validate(source)?;if self.receipt.is_some()||self.closed{return Ok(Default::default())}if self.closing{return self.close_demands(body)}let bytes=self.output.as_ref().ok_or_else(||invalid("original paged byte recipient disappeared"))?;if bytes.len()==self.length{return Ok(RetirementDemand{depth:1,..Default::default()})}let mut d=bytes.byte_backing_demand()?;if d.capacity_bytes==0{let count=self.length.saturating_sub(bytes.len()).min(bytes.retained_bytes().capacity()-bytes.len()).min(body).min(4096);d.copy_bytes=count;d.depth=bytes.retained_bytes().next_push_depth_demand().map_err(ValueError::from)?;}Ok(d)}
 /// ⏱️ Advances one page reservation or bounded original fragment with an exact consumable receipt.
 pub fn advance<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,g:RetainedCloneGrant)->Result<RetainedWireFieldStep,ValueError>{self.last_work=0;self.validate(source)?;if self.receipt.is_some()||self.closed||self.closing{return Ok(RetainedWireFieldStep::Blocked)}if self.is_ready(){return Ok(RetainedWireFieldStep::Ready)}let d=self.demands(source,g.maximum_copy_bytes)?;if !funded(g,d){return Ok(RetainedWireFieldStep::Blocked)}let bytes=self.output.as_mut().ok_or_else(||ValueError::invariant("output absent"))?;let backing=bytes.byte_backing_demand()?;let p=if backing.capacity_bytes!=0{self.last_work=1;bytes.reserve_byte_backing(g)?}else{let count=d.copy_bytes;if count==0{return Ok(RetainedWireFieldStep::Blocked)}self.last_work=1;let mut fragment=[0;4096];for(index,byte)in fragment[..count].iter_mut().enumerate(){*byte=source.byte_at(self.start+bytes.len()+index).ok_or_else(||invalid("original paged byte range changed"))?;}let start=bytes.len();let result=(||{for byte in &fragment[..count]{bytes.push_reserved_byte(*byte)?;}Ok::<_,ValueError>(())})();let copied_bytes=bytes.len()-start;let p=RetainedCloneProgress{copied_items:usize::from(copied_bytes!=0),copied_bytes,..Default::default()};if let Err(error)=result{self.receipt=Some((g,p));return Err(error.with_retained_progress(p))}p};self.receipt=Some((g,p));if !p.fits(g){return Err(invalid("original paged byte receipt exceeds its grant").with_retained_progress(p))}Ok(RetainedWireFieldStep::Progress)}
 pub fn is_ready(&self)->bool{!self.closing&&!self.closed&&self.receipt.is_none()&&self.output.as_ref().is_some_and(|bytes|bytes.len()==self.length)}
 pub fn take_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt.take()}
 /// 🫴️ Moves the exact admitted root into an empty actual recipient after its final source prefix.
 pub fn take_into(&mut self,recipient:&mut Option<Bytes>,g:RetainedCloneGrant)->Result<RetainedWireFieldStep,ValueError>{self.last_work=0;let d=RetirementDemand{depth:1,..Default::default()};if recipient.is_some()||!self.is_ready()||!funded(g,d){return Ok(RetainedWireFieldStep::Blocked)}self.last_work=1;*recipient=self.output.take();self.closed=true;self.receipt=Some((g,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));Ok(RetainedWireFieldStep::Closed)}
 pub fn cancel(&mut self){self.closing=true;}
 /// ♻️ Quotes one retained byte or original empty page release without a replacement decoder.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.closed||self.receipt.is_some(){return Ok(Default::default())}if let Some(retiring)=self.retiring.as_ref(){if !retiring.terminal_is_empty(){return retiring.next_demand(body)}}Ok(RetirementDemand{depth:1,..Default::default()})}
 /// 🧹️ Returns the original paged owner through its defining typed retirement and preserves every denied remainder.
 pub fn close_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedWireFieldStep,semio_framework_value::OriginalRetirementFailure>{
  use semio_framework_value::RetainedWorkPoll;
  let g=effect.grant();self.last_work=0;
  if self.closed{return Ok(RetainedWireFieldStep::Closed)}if !self.closing||self.receipt.is_some(){return Ok(RetainedWireFieldStep::Blocked)}
  let d=self.close_demands(g.maximum_copy_bytes)?;if !funded(g,d)||effect.maximum_work_units()==0{return Ok(RetainedWireFieldStep::Blocked)}
  let p=if let Some(retiring)=self.retiring.as_mut(){
   if !retiring.terminal_is_empty(){
    match retiring.step(effect){
     RetainedWorkPoll::Pending{work}=>{self.last_work=narrow(work)?;return Ok(RetainedWireFieldStep::Blocked)},
     RetainedWorkPoll::Complete(turn)=>{self.last_work=narrow(turn.work)?;match turn.result{Ok(step)=>step.progress(),Err(error)=>{let p=error.retained_progress();self.receipt=Some((g,p));return Err(error)}}},
    }
   }else{self.last_work=pay(effect)?;self.retiring.take();self.closed=true;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
  }else if self.output.is_some(){
   self.last_work=pay(effect)?;
   let Some(output)=self.output.take()else{return Err(ValueError::invariant("output absent").into())};
   match semio_framework_value::retirement::controlled::ControlledRetirement::new(output){Ok(retiring)=>*self.retiring=Some(retiring),Err((error,original))=>{*self.output=Some(original);return Err(error.into())}}
   RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}
  }else{self.last_work=pay(effect)?;self.closed=true;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}};
  self.receipt=Some((g,p));Ok(if self.closed{RetainedWireFieldStep::Closed}else{RetainedWireFieldStep::Progress})
 }

 pub fn terminal_is_empty(&self)->bool{self.closed&&self.output.is_none()&&self.retiring.is_none()&&self.receipt.is_none()}
}
impl Drop for RetainedWireBytesDecode{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original paged byte field abandoned its retained owners");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.retiring)}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

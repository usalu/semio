//! 📦️ Original payload storage initializes and fills in place before a semantic borrow is sealed.
use super::*;
use semio_framework_value::RetirementDemand;

/// 🪪️ Compares a live original payload source without lending its ledger or authority.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalPayloadSourceIdentity(pub usize);

/// 🎟️ Owns one paid original operation ledger; genuine caller turns borrow it until separate closure.
pub struct JobPayloadAuthority{original:ManuallyDrop<Option<Arc<JobPayloadOperationLedger>>>}
impl JobPayloadAuthority{
 pub fn admission_demand()->RetirementDemand{RetirementDemand{capacity_bytes:semio_framework_value::shared_retirement_allocation_bytes::<JobPayloadOperationLedger>(),depth:1,..Default::default()}}
 fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
 pub fn admit(operation:OperationId,generation:Generation,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{let demand=Self::admission_demand();if !Self::permits(grant,demand){return Ok(None)}let original=Some(Arc::new(JobPayloadOperationLedger::new(operation,generation)));Ok(Some((Self{original:ManuallyDrop::new(original)},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:0})))}
 pub fn original(&self)->Option<&Arc<JobPayloadOperationLedger>>{self.original.as_ref()}
 pub fn identity(&self)->Option<OriginalPayloadSourceIdentity>{self.original().map(|ledger|OriginalPayloadSourceIdentity(Arc::as_ptr(ledger)as usize))}
 /// 🧮️ Reads only this original ledger's accounted live backing; custody remains private.
 pub fn accounted_capacity_bytes(&self)->usize{self.original().map_or(0,|ledger|ledger.process_share_bytes())}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()}
 pub fn retirement_demands(&self)->RetirementDemand{if self.terminal_is_empty(){return Default::default()}RetirementDemand{copy_bytes:Self::admission_demand().copy_bytes,release_bytes:semio_framework_value::shared_retirement_allocation_bytes::<JobPayloadOperationLedger>(),depth:1,..Default::default()}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.retirement_demands();if !Self::permits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}let original=self.original.take().unwrap();let (copied_bytes,released_bytes)=if let Some(original)=Arc::into_inner(original){drop(original);(demand.copy_bytes,demand.release_bytes)}else{(0,0)};Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes:0,released_bytes}))}
}
impl Drop for JobPayloadAuthority{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original payload authority reached Drop before separate paid ledger closure");}}

/// 🪪️ Compares the current live in-place header storage without lending payload or ledger custody.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalPayloadBuilderIdentity(usize);
/// 🧱️ Enclosing job admission owns this storage; bounded turns initialize its original fields.
pub struct RetainedPayloadBuilder{original:MaybeUninit<RetainedJobPayload>,stream:JobPayloadStream,initialized:usize,ready:bool,sealed:bool,staged:Option<(Arc<JobPayloadOperationLedger>,JobPayloadPageSource,usize)>}
impl RetainedPayloadBuilder{
 /// 🏗️ Quotes only initialized empty-carrier metadata; its large payload storage remains uninitialized.
 pub fn empty_carrier_copy_bytes()->usize{size_of::<JobPayloadStream>()+size_of::<usize>()+2*size_of::<bool>()+size_of::<Option<(Arc<JobPayloadOperationLedger>,JobPayloadPageSource,usize)>>() }
 /// 🖋️ Initializes an enclosing caller-paid destination without moving or zeroing payload bindings.
 /// # Safety
 /// The destination must be exclusively owned uninitialized storage and the enclosing original initializer must pay the quoted metadata before entry.
 pub unsafe fn initialize_empty_at(target:*mut Self,stream:JobPayloadStream){unsafe{std::ptr::addr_of_mut!((*target).stream).write(stream);std::ptr::addr_of_mut!((*target).initialized).write(0);std::ptr::addr_of_mut!((*target).ready).write(false);std::ptr::addr_of_mut!((*target).sealed).write(false);std::ptr::addr_of_mut!((*target).staged).write(None);}}
 pub fn new(stream:JobPayloadStream)->Self{Self{original:MaybeUninit::uninit(),stream,initialized:0,ready:false,sealed:false,staged:None}}
 pub fn original(&self)->Option<&RetainedJobPayload>{self.ready.then(||unsafe{self.original.assume_init_ref()})}
 pub fn published(&self)->Option<&RetainedJobPayload>{if self.sealed{self.original()}else{None}}
 pub fn is_initialized(&self)->bool{self.ready}
 /// 🪪️ Observes this live storage location; moving its enclosing owner changes the observation.
 pub fn native_header_identity(&self)->OriginalPayloadBuilderIdentity{OriginalPayloadBuilderIdentity(self.original.as_ptr()as usize)}
 /// 🏗️ Reads the exact initialized native binding prefix without borrowing uninitialized fields.
 pub fn initialized_bindings(&self)->usize{self.initialized}
 /// 🏷️ Rebinds only an empty original carrier on one caller-funded metadata turn.
 pub fn select_stream(&mut self,stream:JobPayloadStream,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.stream==stream{return Ok(RetainedCloneStep::Complete(Default::default()))}if !self.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original payload stream rebind requires terminal empty custody"))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}self.stream=stream;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}
 pub fn terminal_is_empty(&self)->bool{!self.ready&&self.initialized==0&&self.staged.is_none()}
 fn permits(cx:&impl JobPayloadStep,demand:RetirementDemand)->bool{let grant=cx.retained_grant();grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
 pub fn initialization_demand(&self)->RetirementDemand{if self.ready{return Default::default()}let copy_bytes=if self.initialized<JOB_PAYLOAD_OPERATION_PAGES{size_of::<Option<JobPayloadPage>>()+size_of::<usize>()}else{size_of::<JobPayloadStream>()+2*size_of::<usize>()+size_of::<Option<Arc<JobPayloadOperationLedger>>>()+2*size_of::<bool>()};RetirementDemand{copy_bytes,depth:1,..Default::default()}}
 /// 🎛️ Initializes one original empty binding or the final scalar header under separate paid turns.
 pub fn advance_initialization(&mut self,cx:&mut impl JobPayloadStep)->Result<bool,ValueError>{
  if self.ready{return Ok(true)}
  let demand=self.initialization_demand();if cx.is_cancelled()||cx.should_yield()||!Self::permits(cx,demand){return Ok(false)}
  let target=self.original.as_mut_ptr();
  if self.initialized<JOB_PAYLOAD_OPERATION_PAGES{unsafe{std::ptr::addr_of_mut!((*target).pages).cast::<Option<JobPayloadPage>>().add(self.initialized).write(None);}self.initialized+=1;}else{
   unsafe{std::ptr::addr_of_mut!((*target).stream).write(self.stream);std::ptr::addr_of_mut!((*target).page_count).write(0);std::ptr::addr_of_mut!((*target).length).write(0);std::ptr::addr_of_mut!((*target).ledger).write(ManuallyDrop::new(None));}
   self.ready=true;self.sealed=false;
  }
  cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;Ok(self.ready)
 }
 fn append_metadata(&self)->Result<(bool,usize),ValueError>{let Some(payload)=self.original()else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload bindings are not initialized"))};if self.sealed{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original published payload cannot append"))}let new=payload.page_count==0||payload.pages[payload.page_count-1].as_ref().is_some_and(|page|page.length==page.source.allocated_capacity_bytes());Ok((new,0))}
 pub fn append_demand(&self,grant:RetainedCloneGrant)->Result<RetirementDemand,ValueError>{let(new,_)=self.append_metadata()?;Ok(RetirementDemand{copy_bytes:usize::from(!new),capacity_bytes:if new{JOB_PAYLOAD_PAGE_BYTES.min(grant.maximum_capacity_bytes).max(1)}else{0},depth:1,..Default::default()})}
 /// ✍️ Admits one actual native page extent or copies one original source slice on distinct turns.
 pub fn append_original(&mut self,cx:&mut impl JobPayloadStep,source:&[u8],cursor:&mut usize)->Result<bool,ValueError>{
  if *cursor>source.len(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload source cursor exceeds its borrowed extent"))}if *cursor==source.len(){return Ok(true)}let grant=cx.retained_grant();let demand=self.append_demand(grant)?;if cx.is_cancelled()||cx.should_yield()||!Self::permits(cx,demand)||cx.payload_page_granted(){return Ok(false)}let(new,_)=self.append_metadata()?;let payload=unsafe{self.original.assume_init_mut()};if payload.ledger.as_ref().is_some_and(|ledger|!Arc::ptr_eq(ledger,cx.payload_ledger())){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload append requires its same operation ledger"))}
  if new{if payload.page_count>=JOB_PAYLOAD_OPERATION_PAGES{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original payload has no remaining admitted page binding"))}let extent=demand.capacity_bytes;match cx.payload_ledger().reserve_original(payload.stream,extent){Ok(_)=>{},Err((fault,copy_bytes))=>{let progress=RetainedCloneProgress{copied_items:1,copied_bytes:copy_bytes,..Default::default()};cx.consume_retained(progress)?;return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,match fault{JobPayloadAdmissionFault::OperationItems=>"original operation page count refused",JobPayloadAdmissionFault::OperationBytes=>"original operation page extent refused",JobPayloadAdmissionFault::StreamItems=>"original stream page count refused",JobPayloadAdmissionFault::StreamBytes=>"original stream page extent refused",_=>"original process page extent refused"}).with_retained_progress(progress))}}payload.pages[payload.page_count]=Some(JobPayloadPage{source:JobPayloadPageSource::with_extent(extent),length:0});payload.page_count+=1;if payload.ledger.is_none(){*payload.ledger=Some(Arc::clone(cx.payload_ledger()));}cx.grant_payload_page();cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:extent,..Default::default()})?;return Ok(false)}
  let page=payload.pages[payload.page_count-1].as_mut().unwrap();let count=grant.maximum_copy_bytes.min(page.source.allocated_capacity_bytes()-page.length).min(source.len()-*cursor);unsafe{std::ptr::copy_nonoverlapping(source.as_ptr().add(*cursor),page.source.storage.as_mut_ptr().cast::<u8>().add(page.length),count);}page.length+=count;payload.length+=count;*cursor+=count;cx.grant_payload_page();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;Ok(*cursor==source.len())
 }
 /// 🔏️ Seals this same original header for borrowed semantic consumption without a finish transfer.
 pub fn seal(&mut self,cx:&mut impl JobPayloadStep)->Result<bool,ValueError>{if !self.ready{return Ok(false)}if self.sealed{return Ok(true)}let demand=RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()};if cx.is_cancelled()||!Self::permits(cx,demand){return Ok(false)}self.sealed=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;Ok(true)}
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{if self.staged.is_some(){return self.staged_retirement_demands()}if let Some(payload)=self.original(){if !payload.terminal_is_empty(){let mut demand=payload.retirement_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original payload builder depth overflow"))?;return Ok(demand)}}Ok(RetirementDemand{copy_bytes:if self.terminal_is_empty(){0}else{2*size_of::<bool>()+size_of::<usize>()},depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
 /// ♻️ Original pages and ledger close before a separate in-place metadata removal turn.
 pub fn close_step_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.retirement_demands()?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}if self.staged.is_some(){return self.close_staged_granted(grant)}if self.original().is_some_and(|payload|!payload.terminal_is_empty()){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let original=unsafe{self.original.assume_init_mut()};let step=original.close_step(child)?;semio_framework_value::retained_clone::admit_retained_clone_close(child,step,original.terminal_is_empty(),"original retained payload builder")?;return Ok(RetainedCloneStep::Progress(step.progress()))}self.ready=false;self.sealed=false;self.initialized=0;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
 /// ♻️ The original caller context records each exact granted close receipt before returning its original error.
 pub fn close_step(&mut self,cx:&mut impl JobPayloadStep)->Result<bool,ValueError>{let result=self.close_step_granted(cx.retained_grant());match result{Ok(step)=>{cx.consume_retained(step.progress())?;Ok(self.terminal_is_empty())},Err(original)=>{let _=cx.consume_retained(original.retained_progress());Err(original)}}}

}
impl Drop for RetainedPayloadBuilder{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original payload builder reached Drop before physical closure and in-place metadata removal");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="⛽️staged/🦀️.rs"]
mod staged;
impl RetainedPayloadBuilder{
 /// 📦️ Lends the sealed in-place header once its last page is ready to supply.
 pub fn sealed_original_mut(&mut self)->Option<&mut RetainedJobPayload>{if !self.sealed||!self.ready||self.staged.is_some(){return None}Some(unsafe{self.original.assume_init_mut()})}
}

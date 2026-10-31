//! 📦️ Original payload storage initializes and fills in place before a semantic borrow is sealed.
use super::*;
use crate::{RetirementDemand,RetainedWorkOutcome};

/// 🪪️ Compares a live original payload source without lending its ledger or authority.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalPayloadSourceIdentity(pub usize);

/// 🎟️ Owns one paid original operation ledger; genuine caller turns borrow it until separate closure.
pub struct JobPayloadAuthority{original:ManuallyDrop<Option<Arc<JobPayloadOperationLedger>>>}
impl JobPayloadAuthority{
 pub fn admission_demand()->RetirementDemand{RetirementDemand{capacity_bytes:semio_framework_value_retirement::retirement::allocation_return::shared_allocation_bytes::<JobPayloadOperationLedger>(),depth:1,..Default::default()}}
 fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
 pub fn admit(operation:OperationId,generation:Generation,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{
  if !Self::permits(grant,Self::admission_demand()){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}
  let turn=semio_framework_value_retirement::retirement::allocation_return::admit_shared_allocation(JobPayloadOperationLedger::new(operation,generation),grant);
  let result=match turn.result{Ok((original,p))=>Ok(Some((Self{original:ManuallyDrop::new(Some(original))},p))),Err((error,original))=>{drop(original);Err(error)}};
  crate::RetainedWorkOutcome{result,work:turn.work}
 }
 pub fn original(&self)->Option<&Arc<JobPayloadOperationLedger>>{self.original.as_ref()}
 pub fn identity(&self)->Option<OriginalPayloadSourceIdentity>{self.original().map(|ledger|OriginalPayloadSourceIdentity(Arc::as_ptr(ledger)as usize))}
 /// 🧮️ Reads only this original ledger's accounted live backing; custody remains private.
 pub fn accounted_capacity_bytes(&self)->usize{self.original().map_or(0,|ledger|ledger.process_share_bytes())}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()}
 pub fn retirement_demands(&self)->RetirementDemand{if self.terminal_is_empty(){return Default::default()}RetirementDemand{copy_bytes:Self::admission_demand().copy_bytes,release_bytes:semio_framework_value_retirement::retirement::allocation_return::shared_allocation_bytes::<JobPayloadOperationLedger>(),depth:1,..Default::default()}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{close_ledger(&mut self.original,grant)}
}
impl Drop for JobPayloadAuthority{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original payload authority reached Drop before separate paid ledger closure");}}

/// 🔨️ Compares the current live in-place header storage without lending payload or ledger custody.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalPayloadBuilderIdentity(usize);
/// 🧱️ Enclosing job admission owns this storage; bounded turns initialize its original fields.
struct OriginalPageReservation{ledger:Option<Arc<JobPayloadOperationLedger>>,extent:usize,held:bool}
pub struct RetainedPayloadBuilder{original:MaybeUninit<RetainedJobPayload>,stream:JobPayloadStream,ready:bool,sealed:bool,staged:Option<(Arc<JobPayloadOperationLedger>,JobPayloadPageSource,usize)>,reservation:Option<OriginalPageReservation>,incoming:Option<JobPayloadPageSource>}
impl RetainedPayloadBuilder{
 /// 🏗️ Empty inline fields do not copy retained payload; the actual native storage remains separately quoted.
 pub fn empty_carrier_copy_bytes()->usize{0}
 /// 🖋️ Initializes an enclosing caller-paid destination without moving or zeroing payload bindings.
 /// # Safety
 /// The destination must be exclusively owned uninitialized storage and the enclosing original initializer must pay the quoted metadata before entry.
 pub unsafe fn initialize_empty_at(target:*mut Self,stream:JobPayloadStream){unsafe{std::ptr::addr_of_mut!((*target).stream).write(stream);std::ptr::addr_of_mut!((*target).ready).write(false);std::ptr::addr_of_mut!((*target).sealed).write(false);std::ptr::addr_of_mut!((*target).staged).write(None);std::ptr::addr_of_mut!((*target).reservation).write(None);std::ptr::addr_of_mut!((*target).incoming).write(None);}}
 pub fn new(stream:JobPayloadStream)->Self{Self{original:MaybeUninit::uninit(),stream,ready:false,sealed:false,staged:None,reservation:None,incoming:None}}
 pub fn original(&self)->Option<&RetainedJobPayload>{self.ready.then(||unsafe{self.original.assume_init_ref()})}
 pub fn published(&self)->Option<&RetainedJobPayload>{if self.sealed{self.original()}else{None}}
 pub fn is_initialized(&self)->bool{self.ready}
 /// 🧲️ Observes this live storage location; moving its enclosing owner changes the observation.
 pub fn native_header_identity(&self)->OriginalPayloadBuilderIdentity{OriginalPayloadBuilderIdentity(self.original.as_ptr()as usize)}
 /// 🏷️ Rebinds only an empty original carrier on one caller-funded metadata turn.
 pub fn select_stream(&mut self,stream:JobPayloadStream,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if self.stream==stream{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  if !self.terminal_is_empty(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original payload stream rebind requires terminal empty custody")),work:0}}
  if grant.maximum_items==0||grant.maximum_depth==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  self.stream=stream;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
 pub fn terminal_is_empty(&self)->bool{!self.ready&&self.staged.is_none()&&self.reservation.is_none()&&self.incoming.is_none()}
 fn permits(cx:&impl JobPayloadStep,demand:RetirementDemand)->bool{let grant=cx.retained_grant();grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
 pub fn initialization_demand(&self)->RetirementDemand{RetirementDemand{depth:usize::from(!self.ready),..Default::default()}}
 /// 🎛️ Initializes the scalar header and empty live prefix on one paid turn without touching unused page rows.
 pub fn advance_initialization(&mut self,cx:&impl JobPayloadStep)->RetainedWorkOutcome{
  if self.ready{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  let demand=self.initialization_demand();if cx.is_cancelled()||cx.should_yield()||!Self::permits(cx,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let target=self.original.as_mut_ptr();
  unsafe{std::ptr::addr_of_mut!((*target).stream).write(self.stream);std::ptr::addr_of_mut!((*target).pages).write([const {MaybeUninit::uninit()};JOB_PAYLOAD_OPERATION_PAGES]);std::ptr::addr_of_mut!((*target).page_count).write(0);std::ptr::addr_of_mut!((*target).length).write(0);std::ptr::addr_of_mut!((*target).ledger).write(ManuallyDrop::new(None));}self.ready=true;self.sealed=false;
  let p=RetainedCloneProgress{copied_items:1,..Default::default()};RetainedWorkOutcome{result:Ok(if self.ready{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)}),work:1}
 }
 fn append_metadata(&self)->Result<(bool,usize),ValueError>{let Some(payload)=self.original()else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload bindings are not initialized"))};if self.sealed{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original published payload cannot append"))}let new=payload.page_count==0||payload.live_pages().last().is_some_and(|page|page.length==page.source.allocated_capacity_bytes());Ok((new,0))}
 pub fn append_demand(&self,grant:RetainedCloneGrant)->Result<RetirementDemand,ValueError>{
  if self.staged.is_some(){return self.staged_commit_demands()}
  if self.incoming.is_some(){return self.original_page_admission_demand(&None)}
  if let Some(reservation)=self.reservation.as_ref(){return Ok(RetirementDemand{capacity_bytes:if reservation.held{reservation.extent}else{0},depth:1,..Default::default()})}
  let(new,_)=self.append_metadata()?;Ok(if new{RetirementDemand{capacity_bytes:JOB_PAYLOAD_PAGE_BYTES.min(grant.maximum_capacity_bytes).max(1),depth:2,..Default::default()}}else{RetirementDemand{copy_bytes:1,depth:1,..Default::default()}})
 }
 fn reserve_page(&mut self,extent:usize,cx:&mut impl JobPayloadStep)->RetainedWorkOutcome{
  let demand=RetirementDemand{depth:1,..Default::default()};
  if extent==0||extent>JOB_PAYLOAD_PAGE_BYTES{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original page reservation extent is invalid")),work:0}}
  if cx.is_cancelled()||cx.should_yield()||cx.payload_page_granted()||!Self::permits(cx,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let payload=match self.original(){Some(payload)=>payload,None=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload bindings are not initialized")),work:0}};
  if self.staged.is_some()||self.sealed||payload.page_count>=JOB_PAYLOAD_OPERATION_PAGES{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original payload cannot reserve another page")),work:0}}
  if payload.ledger.as_ref().is_some_and(|ledger|!Arc::ptr_eq(ledger,cx.payload_ledger())){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload append requires its same operation ledger")),work:0}}
  if let Some(reservation)=self.reservation.as_mut(){
   if reservation.extent!=extent||reservation.ledger.as_ref().is_none_or(|ledger|!Arc::ptr_eq(ledger,cx.payload_ledger())){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original pending reservation changed its ledger or extent")),work:0}}
   if reservation.held{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
   let turn=cx.payload_ledger().reserve_original(self.stream,extent);let result=match turn.result{Ok(_)=>{reservation.held=true;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))},Err((fault,_))=>Err(ValueError::from(fault))};if turn.work>0{cx.grant_payload_page();}return RetainedWorkOutcome{result,work:turn.work}
  }
  let ledger=Arc::clone(cx.payload_ledger());self.reservation=Some(OriginalPageReservation{ledger:Some(ledger),extent,held:false});cx.grant_payload_page();
  RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
 fn allocate_reserved_page(&mut self,cx:&mut impl JobPayloadStep)->RetainedWorkOutcome{
  if self.incoming.is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original incoming backing must be resolved before allocation")),work:0}}
  let Some(reservation)=self.reservation.as_ref()else{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original page backing has no retained reservation")),work:0}};
  if !reservation.held||reservation.extent==0||reservation.ledger.as_ref().is_none_or(|ledger|!Arc::ptr_eq(ledger,cx.payload_ledger())){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original page backing changed its retained reservation")),work:0}}
  if cx.is_cancelled()||cx.should_yield()||cx.payload_page_granted(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let turn=JobPayloadPageSource::admit(reservation.extent,cx.retained_grant());let result=match turn.result{Ok(Some((source,p)))=>{let mut reservation=self.reservation.take().unwrap();self.staged=Some((reservation.ledger.take().unwrap(),source,0));cx.grant_payload_page();Ok(RetainedCloneStep::Progress(p))},Ok(None)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>Err(error)};
  RetainedWorkOutcome{result,work:turn.work}
 }
 /// ✍️ Reserves the original counter, admits backing, publishes ownership and copies bytes on separate funded turns.
 pub fn append_original(&mut self,cx:&mut impl JobPayloadStep,source:&[u8],cursor:&mut usize)->RetainedWorkOutcome{
  if *cursor>source.len(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload source cursor exceeds its borrowed extent")),work:0}}
  if *cursor==source.len(){return RetainedWorkOutcome{result:if self.reservation.is_some()||self.staged.is_some(){Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original completed source still has a pending page"))}else{Ok(RetainedCloneStep::Complete(Default::default()))},work:0}}
  let grant=cx.retained_grant();let demand=match self.append_demand(grant){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if cx.is_cancelled()||cx.should_yield()||cx.payload_page_granted()||!Self::permits(cx,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if self.staged.is_some(){let turn=self.commit_staged_page_funded(cx);return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}}
  if self.incoming.is_some(){let turn=self.accept_original_page(&mut None,cx);return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}}
  if let Some(reservation)=self.reservation.as_ref(){if !reservation.held{return self.reserve_page(reservation.extent,cx)}return self.allocate_reserved_page(cx)}
  let(new,_)=match self.append_metadata(){Ok(value)=>value,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if new{return self.reserve_page(demand.capacity_bytes,cx)}
  let payload=unsafe{self.original.assume_init_mut()};if payload.ledger.as_ref().is_some_and(|ledger|!Arc::ptr_eq(ledger,cx.payload_ledger())){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload append requires its same operation ledger")),work:0}}
  let page=payload.last_page_mut().unwrap();let count=grant.maximum_copy_bytes.min(page.source.allocated_capacity_bytes()-page.length).min(source.len()-*cursor);unsafe{std::ptr::copy_nonoverlapping(source.as_ptr().add(*cursor),page.source.storage.as_mut_ptr().cast::<u8>().add(page.length),count);}page.length+=count;payload.length+=count;*cursor+=count;cx.grant_payload_page();let p=RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()};
  RetainedWorkOutcome{result:Ok(if *cursor==source.len(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)}),work:1}
 }
 /// 📏️ Quotes the scalar header publication without charging a payload copy.
 pub fn seal_demand()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 /// 🔏️ Seals this same original header and returns actual work without posting a caller receipt.
 pub fn seal(&mut self,cx:&impl JobPayloadStep)->RetainedWorkOutcome{
  if self.sealed{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  if !self.ready||cx.is_cancelled()||cx.should_yield()||!Self::permits(cx,Self::seal_demand()){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  self.sealed=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{if let Some(source)=self.incoming.as_ref(){return Ok(RetirementDemand{release_bytes:source.allocated_capacity_bytes(),depth:1,..Default::default()})}if let Some(reservation)=self.reservation.as_ref(){return Ok(if reservation.held{RetirementDemand{depth:1,..Default::default()}}else if reservation.ledger.is_some(){ledger_demands(&reservation.ledger)}else{RetirementDemand{depth:1,..Default::default()}})}if self.staged.is_some(){return self.staged_retirement_demands()}if let Some(payload)=self.original(){if !payload.terminal_is_empty(){let mut demand=payload.retirement_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original payload builder depth overflow"))?;return Ok(demand)}}Ok(RetirementDemand{depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
 /// ♻️ Original pages and ledger close before a separate in-place metadata removal turn.
 /// ```compile_fail,E0599
 /// use semio_framework_job_payload::{RetainedPayloadBuilder,JobPayloadStep};
 /// fn cannot_hide_work(builder:&mut RetainedPayloadBuilder,cx:&mut impl JobPayloadStep){let _=builder.close_step(cx);}
 /// ```
 pub fn close_step_granted(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{
  if self.terminal_is_empty(){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}
  let demand=match self.retirement_demands(){Ok(demand)=>demand,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};
  if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
  if let Some(source)=self.incoming.as_ref(){let released=source.allocated_capacity_bytes();drop(self.incoming.take());return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()})),work:1}}
  if let Some(reservation)=self.reservation.as_mut(){
   if reservation.held{reservation.ledger.as_ref().unwrap().release(self.stream,reservation.extent);reservation.held=false;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
   if reservation.ledger.is_some(){return close_ledger(&mut reservation.ledger,grant)}
   self.reservation.take();return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
  }
  if self.staged.is_some(){return self.close_staged_granted(grant);}
  if self.original().is_some_and(|payload|!payload.terminal_is_empty()){
   let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let original=unsafe{self.original.assume_init_mut()};let turn=original.close_step(child);
   let result=turn.result.and_then(|step|crate::retained_clone::admit_retained_clone_close(child,step,original.terminal_is_empty(),"original retained payload builder")).map(|step|RetainedCloneStep::Progress(step.progress()));
   return crate::RetainedWorkOutcome{result,work:turn.work};
  }
  self.ready=false;self.sealed=false;
  crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }


}
impl Drop for RetainedPayloadBuilder{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original payload builder reached Drop before physical closure and in-place metadata removal");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="⛽️staged/🦀️.rs"]
mod staged;
impl RetainedPayloadBuilder{
 /// 🔐️ Lends the sealed in-place header once its last page is ready to supply.
 pub fn sealed_original_mut(&mut self)->Option<&mut RetainedJobPayload>{if !self.sealed||!self.ready||self.staged.is_some(){return None}Some(unsafe{self.original.assume_init_mut()})}
}



/// 📤️ Transfers the same sealed payload header into caller-owned output on a separate paid turn.
impl RetainedPayloadBuilder{
 pub fn take_finished_demand()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 pub fn take_finished(&mut self,output:&mut Option<RetainedJobPayload>,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if output.is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original payload output is occupied")),work:0}}
  if !self.ready||!self.sealed||self.staged.is_some()||self.incoming.is_some()||self.reservation.is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original payload is not sealed for ownership transfer")),work:0}}
  if !JobPayloadAuthority::permits(grant,Self::take_finished_demand()){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  *output=Some(unsafe{self.original.as_ptr().read()});self.ready=false;self.sealed=false;
  RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
}

impl RetainedPayloadBuilder{
 /// 🧲️ Observes the actually retained incoming backing while another staged page may still be live.
 pub fn incoming_backing_identity(&self)->Option<*const MaybeUninit<u8>>{self.incoming.as_ref().map(JobPayloadPageSource::backing_identity)}
 /// 📏️ Quotes ownership of the existing page without issuing replacement allocation authority.
 pub fn original_page_admission_demand(&self,source:&Option<JobPayloadPageSource>)->Result<RetirementDemand,ValueError>{
  if source.is_some()&&self.incoming.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original incoming page slot is occupied"))}
  if let Some(source)=source.as_ref(){let extent=source.allocated_capacity_bytes();if extent==0||extent>JOB_PAYLOAD_PAGE_BYTES{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original incoming page extent is invalid"))}return Ok(RetirementDemand{depth:1,..Default::default()})}
  if self.incoming.is_none()||self.staged.is_some()||!self.ready{return Ok(Default::default())}Ok(RetirementDemand{depth:1,..Default::default()})
 }
 /// 📥️ The real incoming owner survives every refusal before alias, reservation and zero-copy publication.
 pub fn accept_original_page(&mut self,source:&mut Option<JobPayloadPageSource>,cx:&mut impl JobPayloadStep)->RetainedWorkOutcome{
  if source.is_none()&&self.incoming.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  let demand=match self.original_page_admission_demand(source){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if cx.is_cancelled()||cx.should_yield()||!Self::permits(cx,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if cx.payload_page_granted(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if source.is_some(){self.incoming=source.take();cx.grant_payload_page();return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
  let Some(incoming)=self.incoming.as_ref()else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
  if self.staged.is_some()||!self.ready{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let extent=incoming.allocated_capacity_bytes();
  if let Some(reservation)=self.reservation.as_ref(){
   if reservation.extent!=extent{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original incoming page differs from its retained reservation extent")),work:0}}
   if reservation.held{
    if reservation.ledger.as_ref().is_none_or(|ledger|!Arc::ptr_eq(ledger,cx.payload_ledger())){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original incoming page changed its reservation ledger")),work:0}}
    let mut reservation=self.reservation.take().unwrap();self.staged=Some((reservation.ledger.take().unwrap(),self.incoming.take().unwrap(),0));cx.grant_payload_page();
    return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
   }
  }
  self.reserve_page(extent,cx)
 }
}

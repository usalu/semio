//! 👷️ Core-owned prepaid payload custody crosses the worker boundary without exposing its ledger.
use super::*;

/// 🎟️ Reserves the same bounded process page until its original worker payload consumes it.
pub struct OriginalWorkerPageReservation{reserved:bool}
impl OriginalWorkerPageReservation{
 pub fn available()->bool{JOB_PAYLOAD_PROCESS_OWNED_BYTES.load(Ordering::Acquire)<=JOB_PAYLOAD_PROCESS_BYTES-JOB_PAYLOAD_PAGE_BYTES}
 pub fn try_reserve()->Option<Self>{JOB_PAYLOAD_PROCESS_OWNED_BYTES.try_update(Ordering::AcqRel,Ordering::Acquire,|value|value.checked_add(JOB_PAYLOAD_PAGE_BYTES).filter(|value|*value<=JOB_PAYLOAD_PROCESS_BYTES)).ok().map(|_|Self{reserved:true})}
}
impl Drop for OriginalWorkerPageReservation{fn drop(&mut self){if self.reserved{JOB_PAYLOAD_PROCESS_OWNED_BYTES.fetch_sub(JOB_PAYLOAD_PAGE_BYTES,Ordering::AcqRel);}}}

/// 📦️ Retains the original prepaid page and private ledger throughout worker admission and closure.
pub struct OriginalWorkerPayload{ledger:Option<Arc<JobPayloadOperationLedger>>,fault:JobPayloadSlot,source:Option<JobPayloadPageSource>,initialized:usize}
impl OriginalWorkerPayload{
 pub fn allocation_bytes()->usize{semio_framework_value::shared_retirement_allocation_bytes::<JobPayloadOperationLedger>()+JOB_PAYLOAD_PAGE_BYTES}
 pub fn ledger_release_bytes()->usize{semio_framework_value::shared_retirement_allocation_bytes::<JobPayloadOperationLedger>()}
 pub fn initialization_copy_bytes()->usize{JOB_PAYLOAD_OPERATION_PAGES*size_of::<Option<JobPayloadPage>>()+size_of::<usize>()}
 /// 🖋️ Initializes only the already-admitted worker payload fields in their final native storage.
 /// # Safety
 /// The target must be exclusive uninitialized storage paid by the enclosing original worker admission.
 pub unsafe fn initialize_reserved_at(target:*mut Self,operation:OperationId,generation:Generation,mut reservation:OriginalWorkerPageReservation){
  let ledger=Arc::new(JobPayloadOperationLedger::new(operation,generation));ledger.pages.store(1,Ordering::Relaxed);ledger.bytes.store(JOB_PAYLOAD_PAGE_BYTES,Ordering::Relaxed);ledger.stream_pages[JobPayloadStream::Fault as usize].store(1,Ordering::Relaxed);ledger.stream_bytes[JobPayloadStream::Fault as usize].store(JOB_PAYLOAD_PAGE_BYTES,Ordering::Relaxed);let mut source=JobPayloadPageSource::new();const FAULT:&[u8]=b"job-session.terminal-fault";
  unsafe{std::ptr::copy_nonoverlapping(FAULT.as_ptr(),source.storage.as_mut_ptr().cast::<u8>(),FAULT.len());std::ptr::addr_of_mut!((*target).ledger).write(Some(ledger));std::ptr::addr_of_mut!((*target).fault.present).write(false);std::ptr::addr_of_mut!((*target).source).write(Some(source));std::ptr::addr_of_mut!((*target).initialized).write(0);}reservation.reserved=false;
 }
 pub fn fault(&self)->Option<&RetainedJobPayload>{self.fault.original()}
 pub fn fault_is_empty(&self)->bool{self.fault.is_empty()}
 pub fn has_unpublished_page(&self)->bool{self.source.is_some()}
 pub fn ledger_is_present(&self)->bool{self.ledger.is_some()}
 pub fn ledger_is_empty(&self)->bool{self.ledger.as_ref().is_none_or(|ledger|ledger.terminal_is_empty())}
 pub fn ledger_identity(&self)->Option<usize>{self.ledger.as_ref().map(|ledger|Arc::as_ptr(ledger)as usize)}
 pub fn page_identity(&self)->Option<*const MaybeUninit<u8>>{self.source.as_ref().map(JobPayloadPageSource::backing_identity).or_else(||self.fault().and_then(|payload|payload.pages[0].as_ref().map(|page|page.source.backing_identity())))}
 pub fn initialization_demands(&self)->RetirementDemand{RetirementDemand{depth:usize::from(self.fault.is_empty()),..Default::default()}}
 pub fn advance_initialization(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress{
  if !self.fault.is_empty()||grant.maximum_items==0||grant.maximum_depth==0{return Default::default()}if self.initialized<JOB_PAYLOAD_OPERATION_PAGES{let original=self.fault.original.as_mut_ptr();unsafe{let pages=std::ptr::addr_of_mut!((*original).pages).cast::<Option<JobPayloadPage>>();for index in self.initialized..JOB_PAYLOAD_OPERATION_PAGES{pages.add(index).write(None);}}self.initialized=JOB_PAYLOAD_OPERATION_PAGES;}else{self.publish();}RetainedCloneProgress{copied_items:1,..Default::default()}
 }
 pub fn initialize_page_bindings(&mut self){assert!(self.fault.is_empty()&&self.source.is_some()&&self.initialized==0,"original prepaid bindings initialize exactly once");unsafe{let pages=std::ptr::addr_of_mut!((*self.fault.original.as_mut_ptr()).pages).cast::<Option<JobPayloadPage>>();for index in 0..JOB_PAYLOAD_OPERATION_PAGES{pages.add(index).write(None);}}self.initialized=JOB_PAYLOAD_OPERATION_PAGES;self.publish();}
 fn publish(&mut self){assert_eq!(self.initialized,JOB_PAYLOAD_OPERATION_PAGES);let source=self.source.take().expect("original prepaid page remains owned");let ledger=Arc::clone(self.ledger.as_ref().expect("original prepaid page retains its ledger"));let original=self.fault.original.as_mut_ptr();unsafe{std::ptr::addr_of_mut!((*original).stream).write(JobPayloadStream::Fault);std::ptr::addr_of_mut!((*original).pages).cast::<Option<JobPayloadPage>>().write(Some(JobPayloadPage{source,length:b"job-session.terminal-fault".len()}));std::ptr::addr_of_mut!((*original).page_count).write(1);std::ptr::addr_of_mut!((*original).length).write(b"job-session.terminal-fault".len());std::ptr::addr_of_mut!((*original).ledger).write(ManuallyDrop::new(Some(ledger)));}self.fault.present=true;}
 pub fn unpublished_demands(&self)->RetirementDemand{self.source.as_ref().map_or(Default::default(),|source|RetirementDemand{release_bytes:source.allocated_capacity_bytes(),depth:1,..Default::default()})}
 pub fn close_unpublished(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{let demand=self.unpublished_demands();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return RetainedCloneStep::Progress(Default::default())}let Some(source)=self.source.take()else{return RetainedCloneStep::Complete(Default::default())};let extent=source.allocated_capacity_bytes();self.ledger.as_ref().expect("original page retains ledger").release(JobPayloadStream::Fault,extent);drop(source);RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:extent,..Default::default()})}
 pub fn fault_demands(&self)->Result<RetirementDemand,ValueError>{self.fault.retirement_demands()}
 pub fn close_fault(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.fault.close_step(grant)}
 pub fn close_ledger(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.ledger_is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original worker payload ledger retains its original pages"))}payload_retirement::close_ledger(&mut self.ledger,grant)}
 pub fn context<'a>(&'a self,operation:OperationId,generation:Generation,budget:StepBudget,cancel:&'a CancelToken,now_us:fn()->Option<u64>,clock:ClockStride,preview_sequence:&'a mut u64,recipient:&'a mut RetainedCloneProgress)->Result<StepContext<'a>,ValueError>{StepContext::with_original_worker_authority(operation,generation,budget,cancel,now_us,clock,preview_sequence,recipient,self.ledger.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original worker payload ledger is closed"))?)}
}

impl Drop for OriginalWorkerPayload{fn drop(&mut self){assert!(std::thread::panicking()||self.ledger.is_none()&&self.source.is_none()&&self.fault.is_empty(),"original worker payload requires paid page and ledger closure");}}

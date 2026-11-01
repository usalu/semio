//! ⛽️ Original in-place payload projection admits frame, canonical page and scalar source separately.
use super::*;
impl RetainedPayloadBuilder{
 /// 🪺️ Quotes the actual allocation while original payload slots remain uninitialized.
 pub fn boxed_birth_demands()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}
 /// 🎟️ Retains stable native storage in the real caller output before its receipt can be refused.
 pub fn admit_boxed(stream:JobPayloadStream,output:&mut Option<Box<Self>>,cx:&impl JobPayloadStep)->RetainedWorkOutcome{
  if output.is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original boxed payload requires an empty caller output")),work:0}}
  let demand=Self::boxed_birth_demands();if cx.is_cancelled()||cx.should_yield()||!Self::permits(cx,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let layout=std::alloc::Layout::new::<Self>();let pointer=unsafe{std::alloc::alloc(layout)}.cast::<Self>();
  if pointer.is_null(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original boxed payload allocation refused")),work:1}}
  unsafe{Self::initialize_empty_at(pointer,stream);*output=Some(Box::from_raw(pointer));}
  RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()})),work:1}
 }
 pub fn staged_page_len(&self)->Option<usize>{self.staged.as_ref().map(|(_,_,length)|*length)}
 pub fn staged_page_remaining(&self)->usize{self.staged.as_ref().map_or(0,|(_,source,length)|source.allocated_capacity_bytes()-*length)}
 pub fn staged_backing_identity(&self)->Option<*const MaybeUninit<u8>>{self.staged.as_ref().map(|(_,source,_)|source.backing_identity())}
 pub fn staged_page_birth_demands(&self)->Result<RetirementDemand,ValueError>{
  if self.staged.is_some(){return Ok(Default::default())}let payload=self.original().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original payload bindings are not initialized"))?;
  if self.sealed||payload.page_count>=JOB_PAYLOAD_OPERATION_PAGES{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original payload cannot admit another page"))}
  Ok(if let Some(reservation)=self.reservation.as_ref(){RetirementDemand{capacity_bytes:if reservation.held{reservation.extent}else{0},depth:1,..Default::default()}}else{RetirementDemand{depth:1,..Default::default()}})
 }
 /// 🧱️ Retains the real counter reservation before a separately funded canonical page allocation.
 pub fn begin_staged_page_funded(&mut self,cx:&mut impl JobPayloadStep)->RetainedWorkOutcome{
  if self.staged.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  if self.incoming.is_some(){return self.accept_original_page(&mut None,cx)}
  if self.reservation.as_ref().is_none_or(|reservation|!reservation.held){return self.reserve_page(JOB_PAYLOAD_PAGE_BYTES,cx)}
  if match self.reservation.as_ref(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("reservation absent").into()),work:0}}.extent!=JOB_PAYLOAD_PAGE_BYTES{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"canonical staged page cannot replace a different retained reservation")),work:0}}
  let turn=self.allocate_reserved_page(cx);let ready=self.staged.is_some();RetainedWorkOutcome{result:turn.result.map(|step|if ready{RetainedCloneStep::Complete(step.progress())}else{step}),work:turn.work}
 }
 pub fn staged_source_demands(&self,source:&(impl JobPayloadStagedSource+?Sized))->Result<RetirementDemand,ValueError>{
  if self.staged.is_none()||source.byte_len()>self.staged_page_remaining(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original staged source exceeds its paid page"))}
  let copy_bytes=source.copy_byte_demand().map_err(staged_fault)?;let depth=source.depth_demand().map_err(staged_fault)?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original staged source depth overflow"))?;Ok(RetirementDemand{copy_bytes,depth,..Default::default()})
 }
 /// ✍️ Preserves actual partial source output and forwards its work before a refused result.
 pub fn write_staged_source_funded(&mut self,cx:&mut impl JobPayloadStep,source:&(impl JobPayloadStagedSource+?Sized))->RetainedWorkOutcome{
  let demand=match self.staged_source_demands(source){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if cx.is_cancelled()||cx.should_yield()||cx.payload_page_granted()||!Self::permits(cx,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let(ledger,page,length)=match self.staged.as_mut(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("staged absent").into()),work:0}};if !Arc::ptr_eq(ledger,cx.payload_ledger()){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original staged source changed its ledger")),work:0}}
  let end=*length+source.byte_len();let turn=source.write_bytes(&mut page.storage[*length..end]);let(written,p,fault)=match turn.result{Ok(value)=>(value.initialized_bytes,value.progress,None),Err(error)=>(error.initialized_bytes,error.progress,Some(error.fault))};
  if written<=source.byte_len(){if written>0{*length+=written;}}
  if turn.work>0||p!=RetainedCloneProgress::default(){cx.grant_payload_page();}
  let result=if let Some(fault)=fault{Err(staged_fault(fault).with_retained_progress(p))}else if written!=source.byte_len()||p.copied_bytes>demand.copy_bytes||!p.fits(cx.retained_grant()){Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original staged source changed its admitted output or receipt").with_retained_progress(p))}else{Ok(RetainedCloneStep::Complete(p))};
  RetainedWorkOutcome{result,work:turn.work}
 }
 pub fn staged_commit_demands(&self)->Result<RetirementDemand,ValueError>{if self.staged.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original staged page is absent"))}Ok(RetirementDemand{depth:1,..Default::default()})}
 /// 📤️ Installs this same original page and ledger as a separate admitted ownership transition.
 pub fn commit_staged_page_funded(&mut self,cx:&mut impl JobPayloadStep)->RetainedWorkOutcome{
  let demand=match self.staged_commit_demands(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if cx.is_cancelled()||cx.should_yield()||cx.payload_page_granted()||!Self::permits(cx,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let payload=match self.original(){Some(payload)=>payload,None=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload bindings are not initialized")),work:0}};let index=payload.page_count;
  if self.sealed||index>=JOB_PAYLOAD_OPERATION_PAGES{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original payload slot is occupied")),work:0}}
  if !Arc::ptr_eq(&match self.staged.as_ref(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("staged absent").into()),work:0}}.0,cx.payload_ledger())||payload.ledger.as_ref().is_some_and(|ledger|!Arc::ptr_eq(ledger,cx.payload_ledger())){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original staged publication changed its ledger")),work:0}}
  let(ledger,source,length)=match self.staged.take(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("staged absent").into()),work:0}};let payload=unsafe{self.original.assume_init_mut()};payload.push_page(JobPayloadPage{source,length});if payload.ledger.is_none(){*payload.ledger=Some(ledger)}cx.grant_payload_page();
  RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
 pub fn staged_retirement_demands(&self)->Result<RetirementDemand,ValueError>{let Some((_,source,_))=self.staged.as_ref()else{return Ok(Default::default())};Ok(RetirementDemand{release_bytes:source.allocated_capacity_bytes(),depth:3,..Default::default()})}
 pub fn close_staged_granted(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{
  if self.staged.is_none(){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}
  let demand=match self.staged_retirement_demands(){Ok(demand)=>demand,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};
  if !JobPayloadAuthority::permits(grant,demand){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
  let(ledger,source,_)=match self.staged.take(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("staged absent").into()),work:0}};ledger.release(self.stream,source.allocated_capacity_bytes());let payload=unsafe{self.original.assume_init_mut()};if payload.ledger.is_none(){*payload.ledger=Some(ledger)}drop(source);
  crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})),work:1}
 }
 /// ♻️ Quotes the separate physical return of an already terminal boxed owner.
 pub fn boxed_release_demands()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
fn staged_fault(_:JobPayloadAdmissionFault)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"original staged payload source refused")}

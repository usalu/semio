//! 🎟️ History coroutines retain original runners and decoder lineage through construction and closure.
use super::super::*;
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeDecodeCloseControl, native_decoding::OriginalReceivingPrimary, native_decoding::NativeConstructionStepOutcome, native_decoding::NativeConstructionStepResult};

fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, message) }

fn shared_bytes<T>() -> usize { std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<T>()).unwrap().0.pad_to_align().size() }

impl<'a, T: RetireOwned> HistoryFoldJob<'a, T> {
    /// 📐️ Quotes the concrete future, fixed retirement directory and actual shared control headers.
    pub fn original_birth_demand<R, F>(_: &R) -> Result<RetirementDemand, ValueError>
    where R: FnOnce(HistoryFoldControl) -> F, F: Future<Output = Result<T, semio_framework_replication_error::ProtocolError>> + Send + 'a {
        let capacity_bytes = shared_bytes::<FoldControlState>().checked_add(128 * size_of::<Box<dyn ErasedSnapshotRetirement>>()).and_then(|bytes| bytes.checked_add(size_of::<F>())).and_then(|bytes| bytes.checked_add(shared_bytes::<FoldWake>())).ok_or_else(|| invalid("history fold original construction size overflow"))?;
        let copy_bytes = size_of::<Self>().checked_mul(2).and_then(|bytes| bytes.checked_add(size_of::<R>())).ok_or_else(|| invalid("history fold original construction copy overflow"))?;
        Ok(RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() })
    }

    /// 🫴️ The runner remains original on refusal; the admitted job precedes every receipt response.
    pub fn admit_original<R, F>(runner: &mut Option<R>, output: &mut Option<Self>, native: &mut NativeDecodeControl<'_>) -> NativeConstructionStepOutcome<bool, OriginalReceivingPrimary>
    where R: FnOnce(HistoryFoldControl) -> F, F: Future<Output = Result<T, semio_framework_replication_error::ProtocolError>> + Send + 'a {
        let demand = match runner.as_ref().ok_or_else(|| invalid("history fold requires its original runner")).and_then(Self::original_birth_demand) {
            Ok(demand) if output.is_none() => demand,
            Ok(_) => return NativeConstructionStepOutcome { work: 0, result: Some(Err(invalid("history fold destination is occupied").into())), receipt_refusal: None, progress: Default::default() },
            Err(error) => return NativeConstructionStepOutcome { work: 0, result: Some(Err(error.into())), receipt_refusal: None, progress: Default::default() },
        };
        native.receive_construction_step_typed(demand, |native, _| {
            if native.owned_bytes().checked_add(demand.capacity_bytes).is_none_or(|bytes| bytes > native.maximum_bytes()) { return NativeConstructionStepResult { work: 0, result: Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "history fold birth exceeds its original decoder ceiling").into()), progress: Default::default() }; }
            let mut job = Self::new(runner.take().unwrap());
            job.native_binding = Some((native.ledger_identity(), native.retirement_recipient_identity()));
            *output = Some(job);
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, released_bytes: 0 };
            let allocation_refusal = native.record_constructed_allocation(progress.retained_capacity_bytes).err();
            let result = if allocation_refusal.is_none() { Ok(true) } else { Err(OriginalReceivingPrimary {retirement_failure:None,work_refusal:None,work_overrun:None, primary: None, quote_refusal: None, allocation_refusal, allocation_failure: None, native_allocation: None, receipt_refusal: None, progress }) };
            NativeConstructionStepResult { work: 1, result, progress }
        })
    }

    fn validate_original(&self, native: &dyn NativeDecodeCloseControl) -> Result<(), ValueError> {
        if self.native_binding != Some((native.ledger_identity(), native.retirement_recipient_identity())) { return Err(invalid("history fold changed its original receiving source")); }
        Ok(())
    }

    /// 🪜️ Runs one original transition; completed output remains resident through every refused receipt.
    pub fn step_original(&mut self, native: &mut NativeDecodeControl<'_>, body: usize) -> NativeConstructionStepOutcome<HistoryFoldJobStep<()>, OriginalReceivingPrimary> {
        self.last_work=0;
        if let Err(error) = self.validate_original(native) { return NativeConstructionStepOutcome { work: 0, result: Some(Err(error.into())), receipt_refusal: None, progress: Default::default() }; }
        if self.original_output.is_some()&&!self.cancelling { return NativeConstructionStepOutcome { work: 0, result: Some(Ok(HistoryFoldJobStep::Ready { value: (), progress: Default::default() })), receipt_refusal: None, progress: Default::default() }; }
        let ready_copy = if self.control_closed && matches!(self.result, Some(Ok(_))) { 2 * size_of::<ControlledRetirement<T>>() } else { 0 };
        let demand = match self.original_step_demand(body) {
            Ok(demand) => demand,
            Err(error) => return NativeConstructionStepOutcome { work: 0, result: Some(Err(error.into())), receipt_refusal: None, progress: Default::default() },
        };
        native.receive_construction_step_typed(demand, |native, grant| {
            let result = self.step_granted(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes - ready_copy, maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth }, body, &mut || false);
            let mut progress = match &result {
                Ok(HistoryFoldJobStep::Pending { progress, .. } | HistoryFoldJobStep::Ready { progress, .. } | HistoryFoldJobStep::Rejected { progress }) => *progress,
                Err(error) => error.retained_progress(),
            };
            let result = result.map(|step| match step {
                HistoryFoldJobStep::Ready { value, .. } => {
                    self.original_output = Some(ControlledRetirement::retain(value));
                    self.terminal = false;
                    progress.copied_bytes += ready_copy;
                    HistoryFoldJobStep::Ready { value: (), progress }
                }
                HistoryFoldJobStep::Pending { completed, released_bytes, progress } => HistoryFoldJobStep::Pending { completed, released_bytes, progress },
                HistoryFoldJobStep::Rejected { progress } => HistoryFoldJobStep::Rejected { progress },
            });
            let quote_refusal = if progress.fits(grant) && progress.copied_bytes <= demand.copy_bytes && progress.retained_capacity_bytes <= demand.capacity_bytes && progress.released_bytes <= demand.release_bytes { None } else { Some(invalid("history fold transition exceeded its declared physical effects").with_retained_progress(progress)) };
            let allocation_refusal = if progress.retained_capacity_bytes == 0 { None } else { native.record_constructed_allocation(progress.retained_capacity_bytes).err() };
            let result = match result {
                Ok(step) if quote_refusal.is_none() && allocation_refusal.is_none() => Ok(step),
                result => Err(OriginalReceivingPrimary {retirement_failure:None,work_refusal:None,work_overrun:None, primary: result.err(), quote_refusal, allocation_refusal, allocation_failure: None, native_allocation: None, receipt_refusal: None, progress }),
            };
            NativeConstructionStepResult { work: self.last_work, result, progress }
        })
    }

    /// 📏️ Borrows the next original coroutine action including its final retained output handoff.
    pub fn original_step_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let ready_copy=if self.control_closed&&matches!(self.result,Some(Ok(_))){2*size_of::<ControlledRetirement<T>>()}else{0};
        self.next_step_grant(1,body).and_then(|grant|Ok(RetirementDemand{copy_bytes:grant.maximum_copy_bytes.checked_add(ready_copy).ok_or_else(||invalid("original history output copy overflow"))?,capacity_bytes:grant.maximum_capacity_bytes,release_bytes:grant.maximum_release_bytes,depth:grant.maximum_depth}))
    }

    /// 👁️ Observes whether the same original native output is ready for its funded handoff.
    pub fn original_output_ready(&self)->bool{self.original_output.is_some()}
    /// 🧯️ Observes an original refusal only after all producer cleanup has completed.
    pub fn original_rejection_ready(&self)->bool{self.control_closed&&matches!(self.result,Some(Err(_)))}

    /// 📬️ A separate funded header handoff installs the actual output before receipt acceptance.
    pub fn take_original_output(&mut self, output: &mut Option<T>, native: &mut NativeDecodeControl<'_>) -> NativeConstructionStepOutcome<bool, OriginalReceivingPrimary> {
        let valid = self.validate_original(native).and_then(|()| if output.is_none() && self.original_output.as_ref().is_some_and(ControlledRetirement::original_is_untouched) { Ok(()) } else { Err(invalid("original history fold output is absent, retiring or its destination is occupied")) });
        if let Err(error) = valid { return NativeConstructionStepOutcome { work: 0, result: Some(Err(error.into())), receipt_refusal: None, progress: Default::default() }; }
        let demand = Self::original_output_demand();
        native.receive_construction_step_typed(demand, |_, _| {
            *output = self.original_output.as_mut().unwrap().take_original();
            self.original_output = None;
            self.terminal = self.control_closed && self.future.is_none() && self.result.is_none() && self.waker.is_none();
            NativeConstructionStepResult { work: 1, result: Ok(true), progress: RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() } }
        })
    }

    /// 🧯️ Hands back the complete actual producer cause after its coroutine and control have closed.
    pub fn take_original_rejection(&mut self,output:&mut Option<semio_framework_replication_error::ProtocolError>,native:&mut NativeDecodeControl<'_>)->NativeConstructionStepOutcome<bool,OriginalReceivingPrimary>{
        if let Err(error)=self.validate_original(native){return NativeConstructionStepOutcome{work:0,result:Some(Err(error.into())),receipt_refusal:None,progress:Default::default()}}
        if output.is_some(){return NativeConstructionStepOutcome{work:0,result:Some(Err(invalid("original fold rejection destination is occupied").into())),receipt_refusal:None,progress:Default::default()}}
        if !self.original_rejection_ready(){return NativeConstructionStepOutcome{work:0,result:Some(Ok(false)),receipt_refusal:None,progress:Default::default()}}
        let demand=Self::original_rejection_demand();
        native.receive_construction_step_typed(demand,|_,_|{
            let Some(Err(original))=self.result.take()else{unreachable!("qualified original fold rejection remains retained")};
            *output=Some(original);
            self.terminal=self.control_closed&&self.future.is_none()&&self.result.is_none()&&self.waker.is_none()&&self.original_output.is_none();
            NativeConstructionStepResult{work:1,result:Ok(true),progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
        })
    }
    /// ♻️ A real cleanup receiver takes the same closed producer cause under its remaining retirement stock.
    pub fn take_original_rejection_retiring(&mut self,output:&mut Option<semio_framework_replication_error::ProtocolError>,native:&dyn NativeDecodeCloseControl,grant:RetainedCloneGrant,maximum_work:u64)->semio_framework_value::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
        let result=(||->Result<RetainedCloneProgress,ValueError>{
            self.validate_original(native)?;
            if output.is_some(){return Err(invalid("original retiring fold rejection destination is occupied"))}
            let demand=Self::original_rejection_demand();
            if !self.original_rejection_ready()||maximum_work==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(Default::default())}
            let Some(Err(original))=self.result.take()else{unreachable!("qualified retiring fold rejection remains retained")};
            *output=Some(original);self.terminal=self.control_closed&&self.future.is_none()&&self.result.is_none()&&self.waker.is_none()&&self.original_output.is_none();
            Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
        })();
        let work=u64::from(result.as_ref().is_ok_and(|progress|progress.copied_items!=0));
        semio_framework_value::RetainedWorkOutcome{result:result.map_err(Into::into),work}
    }

    /// 📐️ Prices the producer error slot, recipient slot and terminal metadata before any owner transfers.
    pub fn original_rejection_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<Result<T,semio_framework_replication_error::ProtocolError>>>()+2*size_of::<Option<semio_framework_replication_error::ProtocolError>>()+size_of::<bool>(),depth:1,..Default::default()}}

    /// 📦️ Quotes the unchanged output header before the original decoder is borrowed for its transfer.
    pub fn original_output_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>()+2*size_of::<Option<T>>(),depth:1,..Default::default()}}

    /// 📏️ Completion and post-effect refusal keep actual output inside the original job until handoff or closure.
    pub fn next_original_close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty(){return Ok(Default::default())}
        if self.control_closed&&matches!(self.result,Some(Ok(_))){let mut demand=Self::output_handoff_demand();demand.copy_bytes+=2*size_of::<ControlledRetirement<T>>();return Ok(demand)}
        if let Some(owner)=&self.original_output{if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:2*size_of::<Option<ControlledRetirement<T>>>()+size_of::<bool>(),depth:1,..Default::default()})}let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.max(1);return Ok(demand)}
        if self.control_closed&&self.future.is_none()&&self.result.is_none()&&self.waker.is_none(){return Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}
        let mut demand=self.next_demand(body)?;demand.depth=demand.depth.max(1);Ok(demand)
    }

    /// ♻️ Closing borrows the original decoder even after cancellation, without creating fresh authority.
    pub fn close_original(&mut self, native: &mut dyn NativeDecodeCloseControl, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome {
        self.last_work=0;
        let result=(||{
            self.validate_original(native)?;
            if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
            self.request_cancel();
            let demand=self.next_original_close_demand(grant.maximum_copy_bytes)?;
            if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
            native.charge(demand.capacity_bytes)?;
            if let Some(owner)=self.original_output.as_mut(){
                if owner.terminal_is_empty(){self.last_work=1;self.original_output=None;self.terminal=self.control_closed&&self.future.is_none()&&self.result.is_none()&&self.waker.is_none();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
                let result=owner.step(grant);self.last_work=owner.last_turn_work();return result.map(|step|RetainedCloneStep::Progress(step.progress()));
            }
            if self.control_closed&&self.future.is_none()&&self.result.is_none()&&self.waker.is_none(){self.last_work=1;self.terminal=true;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
            let turn=self.close_step(grant);self.last_work=turn.work;turn.result
        })();
        semio_framework_value::RetainedWorkOutcome{work:self.last_work,result}
    }
}

#[cfg(test)]
mod rejection_laws {
 use super::*;
 #[global_allocator]
 static ORIGINAL_REJECTION_ALLOCATOR:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;
 use semio_framework_value_native_owned::native_decoding::{NativeDecodeConstructionRecipient,NativeDecodeWorkRecipient};
 struct Physical{left:RetainedCloneGrant,submitted:RetainedCloneProgress}
 fn debit(g:&mut RetainedCloneGrant,p:RetainedCloneProgress){assert!(p.fits(*g));g.maximum_items-=p.copied_items;g.maximum_copy_bytes-=p.copied_bytes;g.maximum_capacity_bytes-=p.retained_capacity_bytes;g.maximum_release_bytes-=p.released_bytes;}
 impl NativeDecodeConstructionRecipient for Physical{
  fn submitted(&self)->RetainedCloneProgress{self.submitted}
  fn grant(&mut self,_:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{Ok(self.left)}
  fn accept(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{assert!(p.fits(g));self.submitted=self.submitted.checked_add(p)?;debit(&mut self.left,p);Ok(())}
 }
 struct Work(u64);
 impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{self.0.checked_sub(observed).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original finite work was already exceeded"))}fn admits(&mut self,observed:u64,extra:u64)->Result<bool,ValueError>{Ok(observed.checked_add(extra).is_some_and(|n|n<=self.0))}}
 fn receiving<T>(native:&mut NativeDecodeControl<'_>,physical:&mut Physical,work:&mut Work,call:impl FnOnce(&mut NativeDecodeControl<'_>)->T)->T{
  native.with_work_recipient(work,|native|native.with_construction_recipient(physical,|native|Ok::<_,ValueError>(call(native)))).unwrap()
 }
 fn policy(value:&serde_json::Value)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:value["items"].as_u64().unwrap()as usize,maximum_copy_bytes:value["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:value["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:value["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:value["depth"].as_u64().unwrap()as usize}}
 fn close_value<T:RetireOwned>(value:T,left:&mut RetainedCloneGrant,work:&mut u64){
  let mut owner=ControlledRetirement::new(value).ok().unwrap();
  for _ in 0..100000{if owner.terminal_is_empty(){return}let step=owner.step(*left).unwrap();let used=owner.last_turn_work();assert!(used<=*work);*work-=used;debit(left,step.progress());}
  panic!("original rejection value did not retire")
 }
 #[test]
 fn actual_rejection_takes_the_same_cause_only_after_closed_control_and_qualified_receiving(){
  let law:serde_json::Value=serde_json::from_str(include_str!("rejection.json")).unwrap();
  for case in law["cases"].as_array().unwrap(){
   let mut physical=Physical{left:policy(&law["stock"]),submitted:Default::default()};let mut fuel=law["stock"]["work"].as_u64().unwrap();let mut cleanup=policy(&law["cleanup"]);let mut cleanup_work=law["cleanup"]["work"].as_u64().unwrap();
   let failing=case["result"]!="value";let cause=failing.then(||semio_framework_replication_error::ProtocolError::Malformed{what:"original-fold",offset:7,detail:"complete original cause".into()});let pointer=cause.as_ref().map(|cause|match cause{semio_framework_replication_error::ProtocolError::Malformed{detail,..}=>detail.as_ptr(),_=>unreachable!()});
   let mut runner=Some(move|_control|async move{match cause{Some(cause)=>Err::<(),_>(cause),None=>Ok(())}});let mut job=None;let mut accept=|_|true;let mut native=NativeDecodeControl::new(physical.left.maximum_capacity_bytes,&mut accept);
   let(turn,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||receiving(&mut native,&mut physical,&mut Work(fuel),|native|HistoryFoldJob::admit_original(&mut runner,&mut job,native)));assert_eq!((allocated,released),(turn.progress.retained_capacity_bytes,turn.progress.released_bytes));assert!(turn.receipt_refusal.is_none());assert_eq!(turn.result.unwrap().unwrap(),true);fuel-=turn.work;
   let job=job.as_mut().unwrap();
   if case["controlClosed"].as_bool().unwrap(){
    for _ in 0..100000{
     if job.original_rejection_ready()||job.original_output_ready(){break}
     let(turn,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||receiving(&mut native,&mut physical,&mut Work(fuel),|native|job.step_original(native,4096)));assert_eq!((allocated,released),(turn.progress.retained_capacity_bytes,turn.progress.released_bytes));assert!(turn.receipt_refusal.is_none());assert!(turn.result.unwrap().is_ok());fuel=fuel.checked_sub(turn.work).unwrap();
    }
    assert!(job.original_rejection_ready()||job.original_output_ready());
   }
   let mut output=if case["destinationEmpty"].as_bool().unwrap(){None}else{Some(semio_framework_replication_error::ProtocolError::Malformed{what:"receiver",offset:3,detail:"existing owner".into()})};
   let before=physical.submitted;
   let turn=receiving(&mut native,&mut physical,&mut Work(fuel),|native|job.take_original_rejection(&mut output,native));assert!(turn.receipt_refusal.is_none());let taken=match turn.result.unwrap(){Ok(taken)=>taken,Err(error)=>{close_value(error,&mut cleanup,&mut cleanup_work);false}};fuel=fuel.checked_sub(turn.work).unwrap();
   assert_eq!(taken,case["taken"].as_bool().unwrap());if !taken{assert_eq!(physical.submitted,before)}
   if taken{let semio_framework_replication_error::ProtocolError::Malformed{what,offset,detail}=output.as_ref().unwrap()else{panic!("whole original cause changed")};assert_eq!((*what,*offset),("original-fold",7));assert_eq!(Some(detail.as_ptr()),pointer)}
   if let Some(cause)=output.take(){close_value(cause,&mut cleanup,&mut cleanup_work)}
   if !case["destinationEmpty"].as_bool().unwrap(){
    let denied=job.take_original_rejection_retiring(&mut output,&native,cleanup,0);assert_eq!(denied.result.unwrap(),RetainedCloneProgress::default());assert_eq!(denied.work,0);assert!(output.is_none()&&job.original_rejection_ready());
    let turn=job.take_original_rejection_retiring(&mut output,&native,cleanup,cleanup_work);let progress=turn.result.unwrap();cleanup_work-=turn.work;debit(&mut cleanup,progress);
    let semio_framework_replication_error::ProtocolError::Malformed{detail,..}=output.as_ref().unwrap()else{panic!("actual retiring cause changed")};assert_eq!(Some(detail.as_ptr()),pointer);
    close_value(output.take().unwrap(),&mut cleanup,&mut cleanup_work);
   }
   for _ in 0..100000{if job.terminal_is_empty(){break}let(turn,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||job.close_original(&mut native,cleanup));let step=turn.result.unwrap();assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));cleanup_work=cleanup_work.checked_sub(turn.work).unwrap();debit(&mut cleanup,step.progress());}assert!(job.terminal_is_empty());
   let _=fuel;
  }
 }
}

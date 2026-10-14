//! 🎟️ Every fixture supplies its original issuer, source admission, clock and work before a once-paid caller is born.
use super::{CancelToken,JobPayloadAuthority,OperationId,Generation,RetainedCloneGrant,RetainedCloneProgress,StepContext,StepBudget,OriginalTurnFailure,OriginalCallerSourceParent};
use std::mem::ManuallyDrop;
use super::{InteractiveJob,WorkerJobSession,MountedWorkerJobSession,BatchJobSession,WorkerJobCloseStep,WorkerJobDemandError,RetainedCloneStep,ValueError,ValueRefusalKind};
pub trait OriginalFixtureSession{
 fn witnessed_alias(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>;
 fn return_alias(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>;
 fn close_original(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>;
}
macro_rules! original_session{($owner:ident)=>{impl<J:InteractiveJob+'static> OriginalFixtureSession for $owner<J>{
 fn witnessed_alias(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{self.has_original_cancel_alias_witness(witness)}
 fn return_alias(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{self.return_original_cancel_alias_step(witness,grant)}
 fn close_original(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{self.close_step(cx)}
}}}
original_session!(WorkerJobSession);original_session!(MountedWorkerJobSession);original_session!(BatchJobSession);
pub struct OriginalJobFixtureCaller{operation:OperationId,generation:Generation,parent:ManuallyDrop<Option<OriginalCallerSourceParent>>,sequence:u64,clock:fn()->Option<u64>,deadline:u64,fuel:u64,clock_stride:super::ClockStride,stock:RetainedCloneGrant,spent:RetainedCloneProgress,born:usize,roots_released:usize,last:RetainedCloneProgress}
impl OriginalJobFixtureCaller{
    pub fn admit(operation:OperationId,generation:Generation,source:RetainedCloneGrant,clock:fn()->Option<u64>,deadline:u64,mut fuel:u64)->Self{
        let mut spent=RetainedCloneProgress::default();let mut born=0;let mut clock_stride=super::ClockStride::new();
        assert!(fixture_work_open(clock,deadline,fuel,&mut clock_stride));let grant=fixture_remaining(source,spent);
        let((cancel,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||CancelToken::admit_root(grant).unwrap().unwrap());assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));spent=spent.checked_add(receipt).unwrap();born+=heap.requested_bytes;fuel=fuel.checked_sub(receipt.copied_items as u64).expect("original source root consumes initiating work");
        assert!(fixture_work_open(clock,deadline,fuel,&mut clock_stride));let grant=fixture_remaining(source,spent);
        let((authority,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||JobPayloadAuthority::admit(operation,generation,grant).unwrap().unwrap());assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));spent=spent.checked_add(receipt).unwrap();born+=heap.requested_bytes;fuel=fuel.checked_sub(receipt.copied_items as u64).expect("original payload root consumes initiating work");
        assert!(fixture_work_open(clock,deadline,fuel,&mut clock_stride));let grant=fixture_remaining(source,spent);
        let((parent,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||OriginalCallerSourceParent::admit(authority,cancel,grant).ok().unwrap());assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));spent=spent.checked_add(receipt).unwrap();born+=heap.requested_bytes;fuel=fuel.checked_sub(receipt.copied_items as u64).expect("original source parent consumes initiating work");
        Self{operation,generation,parent:ManuallyDrop::new(Some(parent)),sequence:0,clock,deadline,fuel,clock_stride,stock:source,spent,born,roots_released:0,last:RetainedCloneProgress::default()}
    }
    pub fn born_bytes(&self)->usize{self.born}
    pub fn cancel(&self)->&CancelToken{self.parent.as_ref().expect("original fixture caller retains its parent").cancellation().expect("original fixture caller retains its root")}
    pub fn admit_alias(&mut self,source:RetainedCloneGrant)->CancelToken{let alias=self.turn(source,|cx|{let alias=cx.admit_original_cancel_alias()?;if alias.is_some(){cx.consume_fuel(1);}Ok(alias)}).expect("original fixture alias turn").expect("original fixture alias admitted");assert!(alias.is_same_node(self.cancel()));assert_eq!(self.last,RetainedCloneProgress{copied_items:1,..Default::default()});alias}
    pub fn turn<R>(&mut self,grant:RetainedCloneGrant,advance:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        let before=self.spent;let issued_fuel=self.fuel;let mut value=None;self.clock_stride.begin((self.clock)());
        let mut cx=self.parent.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original fixture parent has already closed"))?.borrow_context(StepBudget::new(self.fuel,self.deadline,self.stock),self.clock,&mut self.sequence,&mut self.spent)?;
        cx.clock.set(self.clock_stride);assert_eq!(cx.operation(),self.operation);assert_eq!(cx.generation(),self.generation);
        let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||cx.with_retained_loan(grant,|cx|{value=Some(advance(cx)?);Ok(())})));
        self.fuel=cx.fuel_remaining().min(issued_fuel);self.clock_stride=cx.clock.get();drop(cx);self.last=fixture_delta(before,self.spent);
        match result{Ok(result)=>{if result.is_ok(){assert!(self.last.fits(grant));assert!(self.spent.fits(self.stock));}result.map(|()|value.expect("original fixture callback returned its original value"))},Err(payload)=>std::panic::resume_unwind(payload)}
    }
    pub fn stock(&self)->RetainedCloneGrant{self.stock}
    pub fn cumulative_receipt(&self)->RetainedCloneProgress{self.spent}
    pub fn remaining_grant(&self)->RetainedCloneGrant{fixture_remaining(self.stock,self.spent)}
    pub fn fuel_remaining(&self)->u64{self.fuel}
    pub fn terminal_is_empty(&self)->bool{self.parent.is_none()}
    pub fn close_owner(&mut self,grant:RetainedCloneGrant,owner:&mut impl OriginalFixtureSession)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
        let step=self.turn(grant,|cx|{let remaining=cx.retained_grant();if cx.should_yield()||remaining.maximum_items==0||remaining.maximum_depth==0{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
            let witnessed=match owner.witnessed_alias(cx.original_cancel_token()){Ok(value)=>value,Err(WorkerJobDemandError::Contention(_))=>return Ok(WorkerJobCloseStep::Blocked),Err(WorkerJobDemandError::Refused(error))=>return Err(error.into())};
            if !witnessed{return owner.close_original(cx)}
            match owner.return_alias(cx.original_cancel_token(),remaining){Ok(Some(step))=>{cx.consume_retained(step.progress())?;cx.consume_fuel(1);Ok(WorkerJobCloseStep::Pending{progress:step.progress()})},Ok(None)|Err(WorkerJobDemandError::Contention(_))=>Ok(WorkerJobCloseStep::Blocked),Err(WorkerJobDemandError::Refused(error))=>Err(OriginalTurnFailure::debit_original(error,cx))}
        })?;assert_eq!(self.last,step.progress());Ok(step)
    }
    pub fn receipt(&self)->RetainedCloneProgress{self.last}
    pub fn return_alias(&mut self,token:CancelToken,grant:RetainedCloneGrant){
        assert!(token.is_same_node(self.cancel()));let mut owner=semio_framework_async::CancelTokenRetirement::from_token(token);self.turn(grant,|cx|{let step=owner.return_alias_step(cx.original_cancel_token(),cx.retained_grant()).map_err(|error|match error{semio_framework_async::CancelTokenRetirementError::Refused(error)=>OriginalTurnFailure::from(error),semio_framework_async::CancelTokenRetirementError::Blocked(_)=>OriginalTurnFailure::from(super::ValueError::literal(super::ValueRefusalKind::UnsupportedOwner,"original fixture alias remains blocked"))})?;cx.consume_retained(step.progress())?;cx.consume_fuel(1);Ok(step)}).unwrap();assert!(owner.terminal_is_empty());
    }


    pub fn retire_step(&mut self,requested:RetainedCloneGrant)->Result<RetainedCloneStep,OriginalTurnFailure>{
        self.last=Default::default();if self.parent.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !fixture_work_open(self.clock,self.deadline,self.fuel,&mut self.clock_stride){return Ok(RetainedCloneStep::Progress(Default::default()))}
        let available=self.remaining_grant();let grant=RetainedCloneGrant{maximum_items:requested.maximum_items.min(available.maximum_items).min(usize::try_from(self.fuel).unwrap_or(usize::MAX)),maximum_copy_bytes:requested.maximum_copy_bytes.min(available.maximum_copy_bytes),maximum_capacity_bytes:requested.maximum_capacity_bytes.min(available.maximum_capacity_bytes),maximum_release_bytes:requested.maximum_release_bytes.min(available.maximum_release_bytes),maximum_depth:requested.maximum_depth.min(available.maximum_depth)};
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{if self.parent.as_ref().unwrap().terminal_is_empty(){drop(self.parent.take());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}else{self.parent.as_mut().unwrap().close_step(grant)}});
        let receipt=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));self.last=receipt;self.fuel=self.fuel.saturating_sub(receipt.copied_items as u64);self.roots_released=self.roots_released.checked_add(receipt.released_bytes).expect("original root release accounting");
        let debit=self.spent.checked_add(receipt).and_then(|total|{self.spent=total;if total.fits(self.stock)&&receipt.fits(grant){Ok(())}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original fixture source close exceeds initiating stock").with_retained_progress(receipt))}});
        match(result,debit){(Ok(step),Ok(()))=>Ok(if self.parent.is_none(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())}),(Err(original),Ok(()))=>Err(original.into()),(Ok(_),Err(original))=>Err(original.into()),(Err(original),Err(refusal))=>{let mut failure=OriginalTurnFailure::from(original);failure.receipt_refusal=Some(refusal);Err(failure)}}
    }
    pub fn retire(mut self,source:RetainedCloneGrant)->usize{
        let mut turns=0;while !self.terminal_is_empty(){let step=self.retire_step(source).expect("original fixture source-root close");assert!(step.progress().fits(source));turns+=1;assert!(turns<=64,"original fixture root closure exhausted its initiating authority");}assert_eq!(self.roots_released,self.born);self.roots_released
    }

}
impl Drop for OriginalJobFixtureCaller{fn drop(&mut self){assert!(std::thread::panicking()||self.parent.is_none(),"original fixed fixture caller requires paid terminal closure")}}

fn fixture_work_open(clock:fn()->Option<u64>,deadline:u64,fuel:u64,stride:&mut super::ClockStride)->bool{if fuel==0{return false}stride.begin(clock());stride.read(clock).is_some_and(|now|now<deadline)}
fn fixture_remaining(stock:RetainedCloneGrant,spent:RetainedCloneProgress)->RetainedCloneGrant{assert!(spent.fits(stock),"original fixture recipient exceeds its one issued stock");RetainedCloneGrant{maximum_items:stock.maximum_items-spent.copied_items,maximum_copy_bytes:stock.maximum_copy_bytes-spent.copied_bytes,maximum_capacity_bytes:stock.maximum_capacity_bytes-spent.retained_capacity_bytes,maximum_release_bytes:stock.maximum_release_bytes-spent.released_bytes,..stock}}
fn fixture_delta(before:RetainedCloneProgress,after:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items.checked_sub(before.copied_items).expect("original fixture metadata recipient is monotone"),copied_bytes:after.copied_bytes.checked_sub(before.copied_bytes).expect("original fixture copy recipient is monotone"),retained_capacity_bytes:after.retained_capacity_bytes.checked_sub(before.retained_capacity_bytes).expect("original fixture capacity recipient is monotone"),released_bytes:after.released_bytes.checked_sub(before.released_bytes).expect("original fixture release recipient is monotone")}}

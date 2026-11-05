//! 🧹️ A resident turn result keeps its own typed descendants until explicit funded acknowledgement.
use semio_framework::kernel::TurnResult;
use semio_framework_job::{OriginalTurnFailure,StepContext,close_original_retained_step};
use semio_framework_value::{ControlledRetirement,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,RetireOwned,ValueError,ValueRefusalKind};
use std::mem::ManuallyDrop;

pub struct OriginalTurnOutput {
    input:Option<semio_framework_actor::RetainedTurnInput>,
    value:ManuallyDrop<Option<TurnResult>>,
    retirement:ManuallyDrop<Option<ControlledRetirement<TurnResult>>>,
}

impl OriginalTurnOutput {
    pub const fn empty()->Self{Self{input:None,value:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None)}}
    pub fn get(&self)->Option<&TurnResult>{self.value.as_ref()}
    pub fn is_empty(&self)->bool{self.input.is_none()&&self.value.is_none()&&self.retirement.is_none()}
    pub fn store(&mut self,value:TurnResult,input:semio_framework_actor::RetainedTurnInput){assert!(self.is_empty(),"resident turn output still owns its prior result");self.input=Some(input);*self.value=Some(value);}

    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if let Some(input)=self.input{if (input.operation,input.generation,input.work.clock_binding)!=(cx.operation().0,cx.generation().0,cx.original_clock_binding().unwrap_or(0))||cx.original_source_epoch().is_none_or(|epoch|epoch<input.epoch){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"resident turn output belongs to another original caller").into())}}
        if self.value.is_some(){
            if !TurnResult::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"turn result has an unsupported original descendant").into())}
            let demand=RetirementDemand{copy_bytes:size_of::<Option<TurnResult>>()+size_of::<Option<ControlledRetirement<TurnResult>>>(),depth:1,..Default::default()};
            close_original_retained_step(cx,demand,|_|{let value=self.value.take().expect("retained turn output");*self.retirement=Some(ControlledRetirement::retain(value));RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}})?;
            return Ok(false)
        }
        let Some(owner)=self.retirement.as_mut()else{return Ok(self.is_empty())};
        if !owner.terminal_is_empty(){
            let demand=owner.next_demand(cx.retained_grant().maximum_copy_bytes)?;
            close_original_retained_step(cx,demand,|grant|{let result=owner.step(grant);RetainedWorkOutcome{result,work:owner.last_turn_work()}})?;
            return Ok(false)
        }
        let demand=RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<TurnResult>>>()+size_of::<Option<semio_framework_actor::RetainedTurnInput>>(),depth:1,..Default::default()};
        close_original_retained_step(cx,demand,|_|{self.retirement.take();self.input=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}})?;
        Ok(self.is_empty())
    }
}

impl Drop for OriginalTurnOutput{fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"resident turn output reached Drop before original acknowledgement");}}

#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework_actor::RetainedTurnInput;
 use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
 fn clock()->Option<u64>{Some(100)}
 #[test]
 fn resident_turn_output_retains_denied_and_cancelled_descendants_until_exact_paid_close(){
  let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
  let grant=serde_json::from_value(law["grant"].clone()).unwrap();
  for cancelled in [false,true]{
   let mut caller=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),grant,clock,100000,100000);
   let input=caller.turn(grant,|cx|{cx.bind_original_source_epoch(1)?;RetainedTurnInput::borrow_original(cx)}).unwrap();
   let mut output=OriginalTurnOutput::empty();output.store(serde_json::from_value(law["result"].clone()).unwrap(),input);
   let pointer=output.get().unwrap().effects.as_ptr();
   let denied=semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:0,..grant};
   let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|{cx.bind_original_source_epoch(2)?;output.close_step(cx)}));
   assert!(!result.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(output.get().unwrap().effects.as_ptr(),pointer);
   let mut foreign=OriginalJobFixtureCaller::admit(OperationId(72),Generation(3),grant,clock,100000,100000);
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign.turn(grant,|cx|{cx.bind_original_source_epoch(2)?;output.close_step(cx)}));
   assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(foreign.receipt(),Default::default());assert_eq!(output.get().unwrap().effects.as_ptr(),pointer);foreign.retire(grant);
   if cancelled{caller.cancel().cancel_now();}
   let mut released=0;let mut denied_capacity=false;let mut denied_release=false;
   for epoch in 3..10000{
    if let Some(owner)=output.retirement.as_ref(){
     let demand=owner.next_demand(grant.maximum_copy_bytes).unwrap();
     let denied=if !denied_capacity&&demand.capacity_bytes>0{denied_capacity=true;Some(semio_framework_value::RetainedCloneGrant{maximum_capacity_bytes:0,..grant})}else if !denied_release&&demand.release_bytes>0{denied_release=true;Some(semio_framework_value::RetainedCloneGrant{maximum_release_bytes:0,..grant})}else{None};
     if let Some(denied)=denied{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|output.close_step(cx))}));assert!(!result.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(caller.receipt(),Default::default());continue}
    }
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|output.close_step(cx))}));
    let complete=result.unwrap();let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));released+=heap.released_bytes;
    if complete{break}
   }
   assert!(output.is_empty());assert!(released>0);assert!(denied_capacity&&denied_release);caller.retire(grant);
  }
 }
}

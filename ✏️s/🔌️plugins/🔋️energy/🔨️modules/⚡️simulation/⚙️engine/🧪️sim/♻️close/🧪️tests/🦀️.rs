//! 🧪️ Original source, actual allocator receipts and complete tail disposal exercise the concrete Energy owners.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,close_original_effect_step};
use semio_framework_value::retained_clone::RetainedCloneStep;
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:64<<20,maximum_capacity_bytes:64<<20,maximum_release_bytes:64<<20,maximum_depth:128}}
fn clock()->Option<u64>{Some(1)}
#[test]
fn original_terminal_backings_are_retired_before_energy_authority_drop(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 assert_eq!(law["owned"].as_array().unwrap().len(),16);
 let text=law["name"].as_str().unwrap();let values=law["temperatures"].as_array().unwrap();
 let bytes=text.len()+values.len()*std::mem::size_of::<f64>();
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(901),Generation(17),stock(),clock,u64::MAX,1_000_000);
 let mut decoder=OriginalJobDecodeOwner::empty();let mut original=None;
 let demand=RetirementDemand{copy_bytes:bytes,capacity_bytes:bytes,depth:1,..Default::default()};
 for _ in 0..4096 {
  if original.is_some(){break}
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(stock(),&mut decoder,|cx|{
   let mut scope=None;
   let mut turn=None;
   if !cx.with_native_receiving_work(&mut scope,demand,|control,_,_|{
    turn=Some(control.receive_construction_step(demand,|control,_|crate::retirement::energy_outcome(control.charge(bytes).map(|()|{
     let mut tail=EnergyAuthorityRetirement::default();
     tail.model.name.try_reserve_exact(text.len()).unwrap();tail.model.name.push_str(text);
     tail.previous_temperatures.try_reserve_exact(values.len()).unwrap();tail.previous_temperatures.extend(values.iter().map(|value|value.as_f64().unwrap()));
     let progress=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:tail.model.name.capacity()+tail.previous_temperatures.capacity()*std::mem::size_of::<f64>(),..Default::default()};
     original=Some(tail);RetainedCloneStep::Complete(progress)
    }))));
    Ok(())
   })?{return Ok(())}
   scope.expect("native receiving scope returned its original outcome").into_result()?;
   let turn=turn.expect("native receiving scope drove its construction step");
   let step=turn.result.map_err(|failure|{cx.consume_work(turn.work);failure})?;
   semio_framework_job::receive_original_receiving_work(cx,semio_framework_value::RetainedWorkOutcome{
    work:turn.work,result:Ok::<_,semio_framework_value::ValueError>(step)
   })?;
   Ok(())
  }));
  result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes));
 }
 assert_eq!(original.as_ref().unwrap().model.name,text);
 let mut close=None;
 caller.turn_with_decoder(stock(),&mut decoder,|cx|cx.with_original_retirement(|cx|{
  crate::retirement::energy_retained_step(cx,RetirementDemand{copy_bytes:2*std::mem::size_of::<ControlledRetirement<EnergyAuthorityRetirement>>()+std::mem::size_of::<EnergyAuthorityRetirement>(),depth:1,..Default::default()},|_|{
   close=Some(ControlledRetirement::new(original.take().unwrap()).unwrap_or_else(|_|panic!("concrete Energy owners support retirement")));
   Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:2*std::mem::size_of::<ControlledRetirement<EnergyAuthorityRetirement>>()+std::mem::size_of::<EnergyAuthorityRetirement>(),..Default::default()}))
  })?;Ok(())
 })).unwrap();
 let mut released=0;
 for turn in 0..100_000{
  if close.as_ref().unwrap().terminal_is_empty(){break}
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(stock(),&mut decoder,|cx|cx.with_original_retirement(|cx|{
   let close=close.as_mut().unwrap();close_original_effect_step(cx,close.selected_demand(128)?,|effect|close.step(128,effect)).map(|_|())
  })));
  result.unwrap();let progress=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));released+=heap.released_bytes;assert!(turn<99_999);
 }
 assert!(close.as_ref().unwrap().terminal_is_empty());assert!(released>=bytes);
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(close.take()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 while !decoder.terminal_is_empty(){caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())})).unwrap();}
 caller.retire(stock());
}

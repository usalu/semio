//! 🧫️ Original payload identity and independent serde output survive completion, foreign callers and cancellation.
use super::*;
use semio_os_spr as protocol;
use semio_os_store_operation_wire::{OriginalReplacementReceiving,ArtifactOperationWire};
use semio_framework_value::{RetainedCloneGrant,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeRetirementRecipient};
use semio_framework_job::{OriginalTurnFailure,close_original_effect_step,original_receiving_scope_headroom};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner};
pub(crate) fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:256}}
pub(crate) fn source_stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:1<<28,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,..stock()}}
pub(crate) fn clock()->Option<u64>{Some(1)}
pub(crate) fn grant_for(demand: RetirementDemand) -> RetainedCloneGrant {
 let base = stock();
 let (items, copy, _) = original_receiving_scope_headroom();
 let table = OriginalJobFixtureCaller::table_birth();
 let business = OriginalJobFixtureCaller::refreshed(RetainedCloneGrant {
  maximum_items: base.maximum_items + 2 * items,
  maximum_copy_bytes: 2 * (base.maximum_copy_bytes.max(demand.copy_bytes) + copy),
  maximum_capacity_bytes: base.maximum_capacity_bytes.max(2 * demand.capacity_bytes),
  maximum_release_bytes: base.maximum_release_bytes.max(2 * demand.release_bytes),
  maximum_depth: base.maximum_depth.max(demand.depth),
 });
 RetainedCloneGrant {
  maximum_items: business.maximum_items.max(table.maximum_items),
  maximum_copy_bytes: business.maximum_copy_bytes.max(table.maximum_copy_bytes),
  maximum_capacity_bytes: business.maximum_capacity_bytes.max(table.maximum_capacity_bytes),
  maximum_release_bytes: business.maximum_release_bytes.max(table.maximum_release_bytes),
  maximum_depth: business.maximum_depth.max(table.maximum_depth),
 }
}
pub(crate) fn turn<R>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{
 quoted_turn(caller,decoder,RetirementDemand::default(),run)
}
pub(crate) fn quoted_turn<R>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,demand:RetirementDemand,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{
 let before=caller.cumulative_receipt();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant_for(demand),decoder,run));let value=result.unwrap();let after=caller.cumulative_receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));value
}
pub(crate) fn close_decoder(caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner) {
 for _ in 0..4096 {
  if decoder.terminal_is_empty() { return; }
  let demand = decoder.retirement_demands(stock().maximum_copy_bytes).unwrap();
  let before = caller.cumulative_receipt();
  let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
   caller.turn(grant_for(demand), |cx| cx.with_original_retirement(|cx| decoder.close_on_source(cx)))
  });
  result.unwrap();
  let after = caller.cumulative_receipt();
  assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
 }
 assert!(decoder.terminal_is_empty());
}

fn reject_foreign(
 caller: &mut OriginalJobFixtureCaller,
 decoder: &mut OriginalJobDecodeOwner,
 mut run: impl FnMut(&mut NativeDecodeControl<'_>, RetainedCloneGrant, u64) -> Result<(), ValueError>,
) {
 for _ in 0..4096 {
  let checked = turn(caller, decoder, |cx| {
   let mut scope = None;
   let complete = cx.with_native_receiving_work(&mut scope, Default::default(), |control, grant, work| {
    let (refusal, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| run(control, grant, work));
    assert_eq!(refusal.unwrap_err().kind, ValueRefusalKind::InvariantViolated);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    Ok(())
   })?;
   if complete {
    let outcome = scope.expect("foreign probe completed its receiving scope");
    assert!(outcome.source_refusal.is_none() && outcome.call.is_ok());
    cx.settle_receiving_scope()?;
   }
   Ok(complete)
  });
  if checked { return; }
 }
 panic!("original receiving scope never admitted the foreign probe");
}

fn schedule(cutoff:Option<usize>){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();let payload=law["wire"].as_str().unwrap().as_bytes().to_vec();let pointer=payload.as_ptr();let length=payload.len();let mut row=protocol::EffectiveSupersession{transition_id:"original-transition".into(),actor:"original-actor".into(),timestamp:protocol::HybridLogicalTimestamp::new(0,0),scope:None,replacement:protocol::InputReplacement::Input{schema:law["schema"].as_str().unwrap().into(),payload}};
 let codec=ArtifactOperationWire::<Vec<String>>::canonical().with_original_receiving();let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<Vec<String>>>=Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<Vec<String>>::default());let mut owner=OriginalReplacementReceiving::default();let mut caller=OriginalJobFixtureCaller::admit(OperationId(991),Generation(17),source_stock(),clock,u64::MAX,16*1024*1024);caller.retain_dispatch().unwrap();let mut decoder=OriginalJobDecodeOwner::empty();let mut ready=false;
 for step in 0..100000 {
  if cutoff == Some(step) { break; }
  let demand = owner.demands(&codec, stock().maximum_copy_bytes).unwrap();
  ready = quoted_turn(&mut caller, &mut decoder, demand, |cx| owner.advance(&mut row.replacement, &codec, &factory, cx));
  if owner.retains_source() && !decoder.has_pending_receiving() {
   let mut recipient = NativeDecodeRetirementRecipient::new();
   reject_foreign(&mut caller, &mut decoder, |control, grant, work| {
    control.with_retirement_owner(0, |control| {
     let result = control.with_retirement_child(&mut recipient, |foreign| {
      owner.advance_native(&mut row.replacement, &codec, &factory, foreign, grant, work).map(|_| ())
     });
     (result, None)
    })
   });
   assert!(recipient.terminal_is_empty());
   let mut foreign = OriginalJobFixtureCaller::admit(OperationId(991), Generation(17), source_stock(), clock, u64::MAX, 16 * 1024 * 1024);
   foreign.retain_dispatch().unwrap();
   let mut foreign_decoder = OriginalJobDecodeOwner::empty();
   reject_foreign(&mut foreign, &mut foreign_decoder, |control, grant, work| {
    owner.advance_native(&mut row.replacement, &codec, &factory, control, grant, work).map(|_| ())
   });
   close_decoder(&mut foreign, &mut foreign_decoder);
   foreign.retire(stock());
  }
  if ready { break; }
 }
 if cutoff.is_none(){assert!(ready);let mut output=None;for _ in 0..4096 { if turn(&mut caller,&mut decoder,|cx|owner.take_input(&mut output,cx)){break} }let mut output=output.unwrap();assert_eq!(serde_json::to_value(output.borrow().unwrap().get()).unwrap(),law["value"]);while !output.terminal_is_empty(){turn(&mut caller,&mut decoder,|cx|close_original_effect_step(cx,output.close_demand(cx.retained_grant().maximum_copy_bytes)?,|effect|output.close_step(effect.grant().maximum_copy_bytes,effect)).map(|_|()));}}
  owner.begin_close();
  let mut returned_at=None;
  let mut closed_at=None;
  for index in 0..100000{
   let closed=turn(&mut caller,&mut decoder,|cx|owner.close(&mut row.replacement,cx));
   let protocol::InputReplacement::Input{payload,..}=&row.replacement else{panic!("original input variant")};
   if payload.as_ptr()==pointer&&payload.len()==length{let _=returned_at.get_or_insert(index);}
   if closed{closed_at=Some(index);break}
  }
  assert!(owner.terminal_is_empty());
  if cutoff.is_none(){assert_eq!(closed_at.unwrap()-returned_at.unwrap(),law["closeTurnsAfterPayloadReturn"].as_u64().unwrap() as usize);}
  let protocol::InputReplacement::Input{payload,..}=&row.replacement else{panic!("original input variant")};
  assert_eq!((payload.as_ptr(),payload.len()),(pointer,length));assert_eq!(std::str::from_utf8(payload).unwrap(),law["wire"].as_str().unwrap());drop(owner);
 let mut original=ControlledRetirement::new(row).ok().unwrap();while !original.terminal_is_empty(){turn(&mut caller,&mut decoder,|cx|close_original_effect_step(cx,original.next_demand(cx.retained_grant().maximum_copy_bytes)?,|effect|original.step(effect.grant().maximum_copy_bytes,effect)).map(|_|()));}let mut issuer=semio_framework_value::FactoryAuthority::new(factory);while !issuer.terminal_is_empty(){turn(&mut caller,&mut decoder,|cx|close_original_effect_step(cx,issuer.demands(cx.retained_grant().maximum_copy_bytes)?,|effect|issuer.step(effect.grant().maximum_copy_bytes,effect)).map(|_|()));}
 close_decoder(&mut caller,&mut decoder);caller.retire(stock());
}
#[test]
fn native_replacement_restores_same_payload_and_preserves_original_receipts_at_every_close(){schedule(None);let law:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();for cutoff in law["cancelPrefixes"].as_array().unwrap(){schedule(Some(cutoff.as_u64().unwrap()as usize));}}

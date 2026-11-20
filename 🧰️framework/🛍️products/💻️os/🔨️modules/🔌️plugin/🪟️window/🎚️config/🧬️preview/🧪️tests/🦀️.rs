//! 🧪️ Original boxed Window leaves retain their exact source while native previews settle and cancel.
use super::*;
use semio_framework_value::{ReplyStash,OriginalRetirementFailure,RetainedAllocationEffect,retained_clone::RetainedOwnedProjectionAdmission};
use crate::camera_owner::{IdentityWindowOwner,RetainedLoadCameraConfig,RetainedLoadCameraConfigMutation};
use semio_framework_value::{retained_clone::original_slot::RetainedOriginalSlot,retirement::controlled::ControlledRetirement};
use crate::original_genesis_tests::{Owner,State,Mutation};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OriginalTurnFailure,StepContext,OperationId,Generation};
pub(crate) fn grant(d:RetirementDemand)->RetainedCloneGrant{assert!(d.copy_bytes<=4096&&d.capacity_bytes<=4096,"native Window preview quote exceeds declared body: {d:?}");RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
pub(crate) fn wide()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:128}}
pub(crate) fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.refreshed_turn_with_decoder(g,decoder,run));let receipt=caller.receipt();assert!(caller.business_receipt().fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));result.unwrap()}
pub(crate) fn effect_turn(
 caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,d:RetirementDemand,
 run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>,
){
 turn(caller,decoder,grant(d),|cx|{
  let Some((result,work))=cx.with_original_source_effect(cx.retained_grant(),|_,effect|run(effect))? else{panic!("fixture effect was not admitted")};
  cx.consume_work(work);cx.consume_retained(result?)?;Ok(())
 })
}
pub(crate) fn projected_input(slot:&mut RetainedOriginalSlot<WindowConfigMutation>,factory:&Arc<dyn store::ArtifactOwnedValueRetirementFactory<WindowConfigMutation>>,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner)->RetainedOwnedProjection<WindowConfigMutation>{
 let key=slot.key();let mut read=ReplyStash::default();
 for _ in 0..4096{
  if slot.borrow(key).is_ok(){break}
  let d=slot.demands(key,4096).unwrap();
  effect_turn(caller,decoder,d,|effect|{
   let result=read.run(|reply,recover|(if recover{slot.recover_advance_read_into(key,factory,effect,reply)}else{slot.advance_read_into(key,factory,effect,reply)}).map(drop))?;
   result.transpose().map(|step|step.map(|v|v.progress()).unwrap_or_default()).map_err(OriginalRetirementFailure::into_value_error)
  });
 }
 let mut admission=RetainedOwnedProjectionAdmission::default();let mut retain=ReplyStash::default();let mut take=ReplyStash::default();let mut output=None;
 for _ in 0..4096{
  if admission.borrow().is_ok(){
   let d=admission.take_demand().unwrap();effect_turn(caller,decoder,d,|effect|Ok(take.run(|reply,recover|(if recover{admission.recover_take(reply)}else{admission.take(&mut output,effect,reply)}).map(drop))?.transpose()?.map(|step|step.progress()).unwrap_or_default()));
   if let Some(input)=output.take(){return input}
  }else{
   let d=slot.retain_read_demand(key,&admission).unwrap();effect_turn(caller,decoder,d,|effect|Ok(retain.run(|reply,recover|(if recover{admission.recover_admit(reply)}else{slot.retain_read_into(key,&mut admission,effect,reply)}).map(drop))?.transpose()?.map(|step|step.progress()).unwrap_or_default()));
  }
 }
 panic!("original input projection did not settle");
}
pub(crate) fn close_input(input:&mut RetainedOwnedProjection<WindowConfigMutation>,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 for _ in 0..65536{if input.terminal_is_empty(){return}let d=input.close_demand(4096).unwrap();turn(caller,decoder,grant(d),|cx|{semio_framework_job::close_original_effect_step(cx,d,|effect|input.close_step(4096,effect))?;Ok(())});}panic!("original window input remained live")
}
fn admit_input(preview:&mut WindowSemanticPreview<Owner>,input:&RetainedOwnedProjection<WindowConfigMutation>,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 for _ in 0..65536{if preview.input_held(){return}let d=preview.input_demand(input).unwrap();effect_turn(caller,decoder,d,|effect|Ok(preview.admit_input(input,effect)?.unwrap_or_default()))}panic!("original window input was not retained")
}
pub(crate) fn typed_close<T:semio_framework_value::retirement::RetireOwned>(original:T,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 let mut original=Some(original);let mut close=None;
 let d=RetirementDemand{copy_bytes:2*size_of::<T>()+size_of::<ControlledRetirement<T>>(),depth:1,..Default::default()};
 turn(caller,decoder,grant(d),|cx|{semio_framework_job::close_original_retained_step(cx,d,|_|{let result=ControlledRetirement::new(original.take().unwrap()).map_err(|(e,_)|e).map(|owner|{close=Some(owner);RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})});semio_framework_value::RetainedWorkOutcome{result,work:1}})?;Ok(())});
 let owner=close.as_mut().unwrap();for _ in 0..65536{if owner.terminal_is_empty(){return}let d=owner.next_demand(4096).unwrap();turn(caller,decoder,grant(d),|cx|{semio_framework_job::close_original_effect_step(cx,d,|effect|owner.step(4096,effect))?;Ok(())});}panic!("original typed window source remained live")
}
fn advance(preview:&mut WindowSemanticPreview<Owner>,store:&WindowConfigStore<Owner>,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant)->RetainedCloneStep{let d=preview.demand(store,4096,caller.fuel_remaining()).unwrap();let outer=if semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){crate::original_genesis_tests::grant()}else{g};turn(caller,decoder,outer,|cx|{if !semio_framework_value::retained_clone::retained_clone_grant_admits(cx.retained_grant(),d){return Ok(RetainedCloneStep::Progress(Default::default()))}let mut step=RetainedCloneStep::Progress(Default::default());crate::retained::receive_progress(cx,d,|receiving,grant,work|{step=preview.advance(store,4096,g.restricted_by(grant),receiving,work)?;Ok(step.progress())})?;Ok(step)})}
fn cancelled_leaf(store:&WindowConfigStore<Owner>,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,cut:usize){
 let mutation=WindowConfigMutation::of::<Owner>("original-preview-window",Mutation::Zoom{zoom:17.0});let pointer=mutation.mutation.as_any().downcast_ref::<Mutation>().unwrap()as*const Mutation;let mut slot=RetainedOriginalSlot::new(mutation);let factory:Arc<dyn store::ArtifactOwnedValueRetirementFactory<WindowConfigMutation>>=Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<WindowConfigMutation>::default());
 let d=WindowSemanticPreview::<Owner>::birth_demand();let mut preview=turn(caller,decoder,grant(d),|cx|{let(owner,p)=WindowSemanticPreview::<Owner>::begin(store,grant(d))?.unwrap();cx.consume_retained(p)?;cx.consume_fuel(1);Ok(owner)});
 let mut input=projected_input(&mut slot,&factory,caller,decoder);admit_input(&mut preview,&input,caller,decoder);close_input(&mut input,caller,decoder);
 for _ in 0..cut{let d=preview.demand(store,4096,caller.fuel_remaining()).unwrap();advance(&mut preview,store,caller,decoder,grant(d));}preview.cancel();for index in 0..65536{if preview.terminal_is_empty(){break;}let d=preview.demand(store,4096,caller.fuel_remaining()).unwrap();let g=grant(d);assert_eq!(advance(&mut preview,store,caller,decoder,RetainedCloneGrant{maximum_items:0,..g}).progress(),Default::default());advance(&mut preview,store,caller,decoder,g);assert!(index<65535,"cancelled native Window preview retained original owners at cut {cut}");}assert!(preview.terminal_is_empty());assert_eq!(slot.get().mutation.as_any().downcast_ref::<Mutation>().unwrap()as*const Mutation,pointer);typed_close(slot,caller,decoder);typed_close(semio_framework_value::FactoryAuthority::new(factory),caller,decoder);
}
#[test]
fn original_window_semantic_preview_matches_portable_outputs_and_retains_boxed_press_sources(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for law in corpus["cases"].as_array().unwrap(){let mut caller=OriginalJobFixtureCaller::admit(OperationId(871),Generation(19),RetainedCloneGrant{maximum_items:1<<25,maximum_copy_bytes:1<<29,maximum_capacity_bytes:1<<27,maximum_release_bytes:1<<27,maximum_depth:4096},semio_framework_job::default_now_us,u64::MAX,1<<28);caller.retain_dispatch().unwrap();let mut decoder=caller.admitted_decoder();let mut typed=TypedWindowConfigStoreOwner::<Owner>::new(protocol::ActorId("original-preview-actor".into()));
  for turn_index in 0..262144{if turn(&mut caller,&mut decoder,crate::original_genesis_tests::grant(),|cx|typed.ensure_partition("original-preview-window",cx)){break;}assert!(turn_index<262143,"native Window partition birth failed to settle");}
  let partition=typed.partitions.get_mut("original-preview-window").unwrap();assert!(partition.store.owned_disposer_installed());for cut in corpus["cancelPrefixes"].as_array().unwrap(){cancelled_leaf(&partition.store,&mut caller,&mut decoder,cut.as_u64().unwrap()as usize);}let d=WindowSemanticPreview::<Owner>::birth_demand();let mut preview=turn(&mut caller,&mut decoder,grant(d),|cx|{let(owner,receipt)=WindowSemanticPreview::<Owner>::begin(&partition.store,grant(d))?.unwrap();cx.consume_retained(receipt)?;cx.consume_fuel(1);Ok(owner)});
  for leaf in law["leaves"].as_array().unwrap(){let mutation=WindowConfigMutation::of::<Owner>("original-preview-window",Mutation::Zoom{zoom:leaf.as_f64().unwrap()});let pointer=mutation.mutation.as_any().downcast_ref::<Mutation>().unwrap()as*const Mutation;let mut slot=RetainedOriginalSlot::new(mutation);let factory:Arc<dyn store::ArtifactOwnedValueRetirementFactory<WindowConfigMutation>>=Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<WindowConfigMutation>::default());
   let mut input=projected_input(&mut slot,&factory,&mut caller,&mut decoder);admit_input(&mut preview,&input,&mut caller,&mut decoder);
   assert_eq!(slot.get().mutation.as_any().downcast_ref::<Mutation>().unwrap()as*const Mutation,pointer);close_input(&mut input,&mut caller,&mut decoder);
   for index in 0..65536{if preview.leaf_ready(){break;}let d=preview.demand(&partition.store,4096,caller.fuel_remaining()).unwrap();let funded=grant(d);for axis in 0..5{let mut denied=funded;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if d.release_bytes>0=>denied.maximum_release_bytes-=1,4 if d.depth>0=>denied.maximum_depth-=1,_=>continue}assert_eq!(advance(&mut preview,&partition.store,&mut caller,&mut decoder,denied).progress(),RetainedCloneProgress::default());}advance(&mut preview,&partition.store,&mut caller,&mut decoder,funded);assert!(index<65535,"native Window semantic leaf failed to settle");}assert!(preview.leaf_ready());typed_close(slot,&mut caller,&mut decoder);typed_close(semio_framework_value::FactoryAuthority::new(factory),&mut caller,&mut decoder);
  }
  let value=serde_json::to_value(semio_framework_value::ToValue::to_value(preview.base.as_ref().unwrap().snapshot())).unwrap();assert_eq!(value["zoom"],law["expected"]);preview.cancel();for index in 0..65536{if preview.terminal_is_empty(){break;}let d=preview.demand(&partition.store,4096,caller.fuel_remaining()).unwrap();advance(&mut preview,&partition.store,&mut caller,&mut decoder,grant(d));assert!(index<65535,"original Window preview failed to close");}assert!(preview.terminal_is_empty());
  for index in 0..65536{if typed.terminal_is_empty(){break;}{let d=typed.retirement_demands(4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)};turn(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|typed.close_step_context(g,cx).map(|_|()).map_err(Into::into)))};assert!(index<65535,"original Window partition failed to close: {:?}",typed.retirement_demands(4096));}assert!(typed.terminal_is_empty());
  for index in 0..65536{if decoder.terminal_is_empty(){break;}let d=decoder.retirement_demands(4096).unwrap();caller.refreshed_turn(crate::original_genesis_tests::grant(),|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())})).unwrap();assert!(index<65535,"original Window receiving owner failed to close");}assert!(decoder.terminal_is_empty());caller.retire(crate::original_genesis_tests::grant());
 }
}

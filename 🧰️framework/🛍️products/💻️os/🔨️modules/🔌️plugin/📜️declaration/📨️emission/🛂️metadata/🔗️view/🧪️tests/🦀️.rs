//! 🧪️ Native view birth and bounded aliases preserve the exact original body under the real caller and decoder.
use super::*;
#[path="routing.rs"]
mod routing;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_value::{RetainedCloneGrant,retirement::controlled::ControlledRetirement,RetainedWorkOutcome};
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<16,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128}}
#[test]
fn original_view_roots_and_routing_aliases_preserve_body_identity(){
 let(_,positive)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let mut bytes=Vec::<u8>::with_capacity(73);bytes.resize(73,0);std::hint::black_box(&bytes);drop(bytes);});assert_eq!(positive.requested_bytes,73);assert_eq!(positive.released_bytes,73);
 let law:serde_json::Value=serde_json::from_str(include_str!("../fixture.json")).unwrap();assert_eq!(law["rootCopies"],0);let mut caller=OriginalJobFixtureCaller::admit(OperationId(701),Generation(13),stock(),||Some(1),u64::MAX,1<<20);let mut decoder=OriginalJobDecodeOwner::empty();let mut view=ViewModel::new(semio_ui_locale::Locale::En,semio_ui_locale::Terminology::Native);view.panel_json=Some("original α🙂".repeat(8192));for row in law["windows"].as_array().unwrap(){view.window_instances.push(semio_framework::ViewWindowInstance{id:row["id"].as_str().unwrap().into(),window_kind_id:row["kind"].as_str().unwrap().into()});}let wire=serde_json::to_value(&view).unwrap();let text=view.panel_json.as_ref().unwrap().as_ptr();let mut source=Some(view);let mut root=None;let mut frame=semio_framework_value::native_decoding::NativeSharedAdmission::default();
 for _ in 0..1024{if root.is_some(){break}caller.turn_with_decoder(stock(),&mut decoder,|cx|ActionMeta::admit_view_root(&mut source,&mut root,&mut frame,cx)).unwrap();}let root=root.unwrap();assert!(source.is_none());assert_eq!(root.panel_json.as_ref().unwrap().as_ptr(),text);assert_eq!(serde_json::to_value(&*root).unwrap(),wire);let meta=ActionMeta{view_state:Some(root),..Default::default()};let mut alias=None;let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(stock(),&mut decoder,|cx|{assert!(meta.admit_routing_alias(&mut alias,cx)?);Ok(())}).unwrap());assert_eq!(heap.requested_bytes,0);let alias=alias.unwrap();assert!(Arc::ptr_eq(meta.view_state.as_ref().unwrap(),alias.view_state.as_ref().unwrap()));assert_eq!(alias.view_state.as_ref().unwrap().panel_json.as_ref().unwrap().as_ptr(),text);let mut close=ControlledRetirement::retain((meta,alias));for _ in 0..65536{if close.terminal_is_empty(){break}caller.turn_with_decoder(stock(),&mut decoder,|cx|cx.with_original_retirement(|cx|{let d=close.selected_demand(stock().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,d,|g|{let result=close.step(g);RetainedWorkOutcome{result,work:close.last_turn_work()}})?;Ok(())})).unwrap();}assert!(close.terminal_is_empty());for _ in 0..1024{if decoder.terminal_is_empty(){break}caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(stock().maximum_copy_bytes)?;decoder.close_on_source(cx)?;Ok(())})).unwrap();}assert!(decoder.terminal_is_empty());caller.retire(stock());
}

fn original_alias_turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,decoder,run));
 let receipt=caller.receipt();
 assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
 result.unwrap()
}
#[test]
fn original_view_routing_alias_denials_preserve_metadata_and_original_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("routing-alias.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(702),Generation(14),stock(),||Some(1),u64::MAX,1<<20);
  let mut decoder=OriginalJobDecodeOwner::empty();
  let mut view=ViewModel::new(semio_ui_locale::Locale::En,semio_ui_locale::Terminology::Native);
  view.panel_json=Some("x".repeat(law["bodyBytes"].as_u64().unwrap()as usize));
  let mut source=Some(view);let mut root=None;let mut frame=semio_framework_value::native_decoding::NativeSharedAdmission::default();
  for _ in 0..1024{if root.is_some(){break}original_alias_turn(&mut caller,&mut decoder,stock(),|cx|ActionMeta::admit_view_root(&mut source,&mut root,&mut frame,cx));}
  assert!(root.is_some()&&source.is_none());
  let metadata=Some(ActionMeta{actor:"original actor".into(),instance_id:73,view_state:root,expected_document_revision:Some([7;32])});
  let view=metadata.as_ref().unwrap().view_state.as_ref().unwrap();let pointer=Arc::as_ptr(view);let text=view.panel_json.as_ref().unwrap().as_ptr();
  let demand=ActionMeta::routing_alias_demands();let mut grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:demand.depth};
  match row["grant"].as_str().unwrap(){"items-zero"=>grant.maximum_items=0,"copy-minus-one"=>grant.maximum_copy_bytes-=1,"depth-zero"=>grant.maximum_depth=0,"cancelled"=>{caller.cancel().cancel_now();},"exact"=>{},_=>unreachable!()}
  let mut alias=None;let mut admission=semio_framework_job::OriginalConstructionAdmission::default();
  let admitted=original_alias_turn(&mut caller,&mut decoder,grant,|cx|metadata.as_ref().unwrap().admit_routing_alias(&mut admission,&mut alias,cx));
  assert_eq!(admitted,row["admitted"].as_bool().unwrap());
  assert_eq!(alias.is_some(),admitted);
  assert_eq!(Arc::as_ptr(metadata.as_ref().unwrap().view_state.as_ref().unwrap()),pointer);
  assert_eq!(metadata.as_ref().unwrap().view_state.as_ref().unwrap().panel_json.as_ref().unwrap().as_ptr(),text);
  if let Some(alias)=alias.as_ref(){assert_eq!(Arc::as_ptr(alias.view_state.as_ref().unwrap()),pointer);assert_eq!(alias.expected_document_revision,Some([7;32]));assert_eq!(alias.actor.as_str(),"original actor");}
  if !admitted{assert_eq!(caller.receipt(),Default::default());}
  let mut close=ControlledRetirement::retain((metadata,alias));
  for index in 0..65536{if close.terminal_is_empty(){break}original_alias_turn(&mut caller,&mut decoder,stock(),|cx|cx.with_original_retirement(|cx|{let d=close.selected_demand(stock().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,d,|g|{let result=close.step(g);RetainedWorkOutcome{result,work:close.last_turn_work()}})?;Ok(())}));assert!(index<65535);}
  assert!(close.terminal_is_empty());
  for index in 0..1024{if decoder.terminal_is_empty(){break}caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(stock().maximum_copy_bytes)?;decoder.close_on_source(cx)?;Ok(())})).unwrap();assert!(index<1023);}
  assert!(decoder.terminal_is_empty());caller.retire(stock());
 }
}

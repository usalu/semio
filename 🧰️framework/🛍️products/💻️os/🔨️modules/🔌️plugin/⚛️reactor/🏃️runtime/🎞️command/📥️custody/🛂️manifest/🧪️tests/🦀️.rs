//! 🧪️ Actual immutable Pack source and semantic fields stay bound to the genuine fixture Root.
use semio_plugin_decl_app::ArtifactApp;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_framework::FaultFrom;
use semio_framework::manifest::CommandOwnerAddress as ManifestCommandOwnerAddress;
use crate::ops_ingress::original_manifest_receiving::OriginalManifestCommandReceiving;
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_plugin_program::PluginProgram;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use serde_json::Value;
#[doc(hidden)]

#[cfg_attr(test,test)]
pub fn original_manifest_fields_keep_paid_source_and_denied_transfer(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let expected=DslValue::from(&corpus["invocation"]);let mut source=store::pack_rt::encode_wire_value(&expected);let original=source.as_ptr();
 let mut caller=semio_plugin_testing_caller::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let loan=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};let mut limits=store::os_pack::PackLimits::default();limits.max_depth=16;limits.max_total_alloc=1<<20;
 let mut offered=None;let before=cx.retained_progress();let fuel=cx.fuel_remaining();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:23,..loan},|same|{offered=OriginalManifestCommandReceiving::admit(&mut source,false,limits.clone(),same)?;Ok(offered.is_some())}));assert!(!result.unwrap());assert!(offered.is_none());assert_eq!(source.as_ptr(),original);assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let(ready,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|{offered=OriginalManifestCommandReceiving::admit(&mut source,false,limits,same)?;Ok(offered.is_some())}).unwrap());assert!(ready);let mut owner=offered.take().unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(source.is_empty());assert_eq!(owner.parser.as_ref().unwrap().input().unwrap().as_ptr(),original);
 let mut transferred=false;
 for _ in 0..semio_plugin_testing_caller::OriginalFixtureCaller::maximum_turns(){
  if owner.phase==2&&!transferred{let before=cx.retained_progress();let fuel=cx.fuel_remaining();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:23,..loan},|same|owner.advance_step(same)));assert!(!result.unwrap());assert_eq!(owner.phase,2);assert!(owner.fields[0].is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);transferred=true;}
  let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|owner.advance_step(same)));let ready=result.unwrap();let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));if ready{break}
 }
 assert!(transferred);let(address,args)=(owner.address.as_ref().unwrap(),&mut owner.args);assert_eq!(address.command_id,"setValue");assert!(matches!(&address.owner,ManifestCommandOwnerAddress::Mode{plugin_id,app_id,mode_id}if plugin_id=="fixture"&&app_id=="editor"&&mode_id=="author"));assert_eq!(serde_json::Value::from(args.as_ref().unwrap()),corpus["invocation"]["arguments"]);
 for _ in 0..semio_plugin_testing_caller::OriginalFixtureCaller::maximum_turns(){if owner.terminal_is_empty(){break}let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|owner.close_step(same)));result.unwrap();let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));}
 assert!(owner.terminal_is_empty());drop(owner);let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
 #[cfg(not(test))]
 original_manifest_close_batches_paid_steps();
}

#[cfg_attr(test,test)]
pub fn original_manifest_close_batches_paid_steps(){
 use super::{OriginalDecodedPluginCommand,retirement_slice};

 let corpus:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let policy=&corpus["retirement"];
 let wide=RetainedCloneGrant{maximum_items:policy["wideItems"].as_u64().unwrap() as usize,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
 let narrow=RetainedCloneGrant{maximum_items:policy["narrowItems"].as_u64().unwrap() as usize,..wide};
 let minimum=policy["minimumPaidItemsPerBatch"].as_u64().unwrap() as usize;
 let turns=policy["maximumTurns"].as_u64().unwrap();
 let mut decoded=OriginalDecodedPluginCommand::new(protocol::AppCommand::Command{seq:1,command:store::pack_rt::encode_wire_value(&DslValue::from(&corpus["invocation"])),view_state:vec![1,2,3]});
 let mut caller=semio_plugin_testing_caller::OriginalFixtureCaller::admit().unwrap();
 let mut cx=caller.context();
 let (source,_,receiving)=decoded.command_parts().unwrap();
 cx.with_retained_loan(wide,|cx|{
  let mut limits=store::os_pack::PackLimits::default();limits.max_file_len=source.len() as u64;limits.max_total_alloc=1<<20;limits.max_depth=64;
  *receiving=OriginalManifestCommandReceiving::admit(source,false,limits,cx)?;
  Ok(())
 }).unwrap();
 let owner=receiving.as_mut().unwrap();
 let mut ready=false;
 for _ in 0..turns{
  if cx.with_retained_loan(wide,|cx|owner.advance_step(cx)).unwrap(){ready=true;break}
 }
 assert!(ready);
 let receipt=cx.retained_progress();let work=cx.original_work_units_spent();
 assert!(!cx.with_retained_loan(narrow,|cx|retirement_slice(cx,|cx|owner.close_step(cx))).unwrap());
 assert_eq!((cx.retained_progress(),cx.original_work_units_spent()),(receipt,work));
 assert!(decoded.wire.original_command().is_some());

 let mut receiving_batched=false;
 for _ in 0..turns{
  let before=cx.retained_progress();let work=cx.original_work_units_spent();
  let owner=decoded.receiving.as_mut().unwrap();
  let done=cx.with_retained_loan(wide,|cx|retirement_slice(cx,|cx|owner.close_step(cx))).unwrap();
  let paid=cx.retained_progress().copied_items-before.copied_items;
  assert!(paid<=wide.maximum_items);
  assert!(cx.retained_progress().copied_bytes-before.copied_bytes<=wide.maximum_copy_bytes);
  if paid>=minimum{assert!(cx.original_work_units_spent()>work);receiving_batched=true;}
  if done{break}
 }
 assert!(receiving_batched&&decoded.receiving.as_ref().unwrap().terminal_is_empty());
 assert!(decoded.wire.original_command().is_some());

 let mut wire_batched=false;
 for _ in 0..turns{
  let before=cx.retained_progress();let work=cx.original_work_units_spent();
  let done=cx.with_retained_loan(wide,|cx|retirement_slice(cx,|cx|decoded.wire.close_original_step(cx))).unwrap();
  let paid=cx.retained_progress().copied_items-before.copied_items;
  assert!(paid<=wide.maximum_items);
  assert!(cx.retained_progress().copied_bytes-before.copied_bytes<=wide.maximum_copy_bytes);
  if paid>=minimum{assert!(cx.original_work_units_spent()>work);wire_batched=true;}
  if done{break}
 }
 assert!(wire_batched&&decoded.wire.terminal_is_empty());
 decoded.receiving.take();
 assert!(decoded.terminal_is_empty());
 drop(decoded);let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
}

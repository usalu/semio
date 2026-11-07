//! 🧪️ A genuine one-fuel caller probes original cached roots and closes their complete physical family.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,receive_original_retained_work,close_original_retained_step};
fn quote(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observe<T>(f:impl FnOnce()->T)->(T,usize,usize){semio_os_kernel_neural_engine_model::testkit::observe_ownership(f)}
#[test]
fn original_cached_reply_probe_matches_independent_fixture_with_one_fuel_and_exact_custody(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️read.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let(cache,born,freed)=observe(||{let cache=Arc::new(NeuralCache::new());for(key,value)in row["entries"].as_object().unwrap(){cache.seed(key.parse().unwrap(),serde_json::from_value(value.clone()).unwrap()).unwrap_or_else(|_|unreachable!());}cache});let original=born-freed;let hash=row["hash"].as_u64().unwrap();let mut read=NeuralCacheRead::new();let mut output=None;let mut ready=false;let mut births=0;let mut releases=0;
  let stock=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:128<<20,maximum_capacity_bytes:256<<20,maximum_release_bytes:512<<20,maximum_depth:4096};let mut caller=OriginalJobFixtureCaller::admit(OperationId(97146),Generation(17),stock,||Some(1000),1500,32768);
  for _ in 0..200{let d=read.next_demands(&cache,hash,4096).unwrap();let g=quote(d);
   for denied in [RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g},RetainedCloneGrant{maximum_depth:0,..g}]{let(turn,born,free)=observe(||read.advance(&cache,hash,denied));assert_eq!((turn.work,born,free),(0,0,0));assert!(turn.result.unwrap().value.is_none());}
   let(_,born,free)=observe(||caller.turn(g,|cx|cx.with_original_work_loan(1,1500,|cx|{let turn=read.advance(&cache,hash,cx.retained_grant());let work=turn.work;let advance=turn.result.unwrap();if let Some(value)=advance.value{output=value;ready=true}receive_original_retained_work(cx,RetainedWorkOutcome::<RetainedCloneStep,semio_framework_value::ValueError>{result:Ok(RetainedCloneStep::Progress(advance.progress)),work})?;assert!(work<=1);Ok(())})).unwrap());let p=caller.receipt();assert_eq!((born,free),(p.retained_capacity_bytes,p.released_bytes));births+=born;releases+=free;if ready{break}
  }assert!(ready);assert_eq!(output.as_ref().map(|v|serde_json::to_value(v).unwrap()).unwrap_or(serde_json::Value::Null),row["expected"]);
  if let Some(value)=output.as_ref(){assert_eq!(value.original_root_identity(),cache.inspect_original(hash,Dictionary::original_root_identity).unwrap())}
  let(mut close,born,free)=observe(||ControlledRetirement::new((CacheLease(cache),output)).unwrap_or_else(|_|unreachable!()));assert_eq!((born,free),(0,0));
  for _ in 0..100000{if close.terminal_is_empty(){break}let d=close.selected_demand(4096).unwrap();let(_,born,free)=observe(||caller.turn(quote(d),|cx|cx.with_original_work_loan(1,1500,|cx|{close_original_retained_step(cx,d,|g|{let result=close.step(g);RetainedWorkOutcome{result,work:close.last_turn_work()}})?;Ok(())})).unwrap());let p=caller.receipt();assert_eq!((born,free),(p.retained_capacity_bytes,p.released_bytes));births+=born;releases+=free;}
  assert!(close.terminal_is_empty());assert_eq!(original+births,releases);caller.retire(stock);
 }
}

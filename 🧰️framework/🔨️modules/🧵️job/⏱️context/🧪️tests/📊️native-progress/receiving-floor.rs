//! 🪜️ Drives the receiving floor through a live Source: the half share stays, a larger floor raises it, and a floor beyond the whole turn is refused for no more than an idle turn costs.
use super::{account,caller,close,measured,quantum,OriginalJobDecodeOwner,OriginalJobFixtureCaller};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
use serde_json::Value;

/// 🔭️ Runs one funded turn under the floor and returns the grant the receiving body was offered, if the scope opened.
fn offered(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,floor:RetirementDemand)->Option<RetainedCloneGrant>{
 let mut seen=None;
 measured(caller,decoder,|cx|{let mut out=None;account(cx,|cx|cx.with_native_receiving_work(&mut out,floor,|_,grant,_|{seen=Some(grant);Ok(())}))?;account(cx,|_|drop(out));if seen.is_some(){cx.consume_receiving_retained(cx.receiving_issued_grant().unwrap(),RetainedCloneProgress::default())?;}Ok(())});
 seen
}

/// 🧾️ The stock one turn took, in items and copy bytes.
fn fee(before:RetainedCloneGrant,after:RetainedCloneGrant)->(usize,usize){(before.maximum_items-after.maximum_items,before.maximum_copy_bytes-after.maximum_copy_bytes)}

#[test]
fn a_receiving_floor_raises_the_half_share_up_to_the_whole_turn_and_refuses_beyond_it_at_no_charge(){
 let law:Value=serde_json::from_str(include_str!("🧫️fixtures/receiving-floor.json")).unwrap();let n=|value:&Value,key:&str|value[key].as_u64().unwrap() as usize;
 let whole=quantum().maximum_copy_bytes;assert_eq!(n(&law,"turnCopyBytes"),whole);
 let mut caller=caller();let mut decoder=caller.admitted_decoder();let funded=caller.fuel_remaining();
 let idle={let stock=caller.remaining_grant();measured(&mut caller,&mut decoder,|_|Ok(()));fee(stock,caller.remaining_grant())};
 let share=(0..funded).find_map(|_|offered(&mut caller,&mut decoder,RetirementDemand::default())).expect("the automatic receiving share never opened");
 assert!(share.maximum_copy_bytes<=whole.div_ceil(2),"the automatic share exceeds half of the turn");
 for case in law["cases"].as_array().unwrap(){
  let id=case["id"].as_str().unwrap();let floor=RetirementDemand{copy_bytes:n(case,"floorCopyBytes"),depth:1,..Default::default()};
  match case["expect"].as_str().unwrap(){
   "share"=>{let grant=offered(&mut caller,&mut decoder,floor).unwrap_or_else(||panic!("{id} did not open"));assert_eq!(grant,share,"{id}");assert!(grant.maximum_copy_bytes>=floor.copy_bytes,"{id}");},
   "floor"=>{let grant=offered(&mut caller,&mut decoder,floor).unwrap_or_else(||panic!("{id} did not open"));assert_eq!(grant,RetainedCloneGrant{maximum_copy_bytes:floor.copy_bytes,..share},"{id}");assert!(grant.maximum_copy_bytes<whole,"{id}");},
   "refused"=>{assert!(floor.copy_bytes>whole,"{id}");for turn in 0..n(&law,"refusalTurns"){let stock=caller.remaining_grant();assert!(offered(&mut caller,&mut decoder,floor).is_none(),"{id} opened on turn {turn}");if turn>=n(&law,"settlingTurns"){let spent=fee(stock,caller.remaining_grant());assert!(spent.0<=idle.0&&spent.1<=idle.1,"{id} charged more than an idle turn on refused turn {turn}: {spent:?} against {idle:?}");}}},
   other=>panic!("unknown floor expectation {other}"),
  }
 }
 close(caller,decoder);
}

/// 🪙️ A quoted scope whose copy quote exceeds the whole turn never opens and is charged no more than an idle turn.
#[test]
fn a_quoted_receiving_scope_beyond_the_turn_is_refused_at_no_charge(){
 let law:Value=serde_json::from_str(include_str!("🧫️fixtures/receiving-quote-refusal.json")).unwrap();let n=|key:&str|law[key].as_u64().unwrap();
 let mut caller=caller();let mut decoder=caller.admitted_decoder();
 let idle={let stock=caller.remaining_grant();measured(&mut caller,&mut decoder,|_|Ok(()));fee(stock,caller.remaining_grant())};
 let quote=(RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:n("quoteCopyBytes") as usize,maximum_depth:1,..Default::default()},1000);
 for turn in 0..n("refusalTurns"){
  let stock=caller.remaining_grant();let mut opened=false;
  measured(&mut caller,&mut decoder,|cx|{let mut out=None;account(cx,|cx|cx.with_native_receiving_quoted(&mut out,quote,|_,_|{opened=true;Ok(())}))?;account(cx,|_|drop(out));Ok(())});
  assert!(!opened,"the over-quoted scope opened on turn {turn}");
  if turn>=n("settlingTurns"){let spent=fee(stock,caller.remaining_grant());assert!(spent.0<=idle.0&&spent.1<=idle.1,"refused turn {turn} charged {spent:?} against an idle {idle:?}");}
 }
 close(caller,decoder);
}

//! 🪜️ Settling an idle unquoted receiving scope refunds its whole unspent share, so the Source keeps only the actual spend and control fees.
use super::{account,caller,close,measured,testkit};
use semio_framework_value::RetainedCloneProgress;
use serde_json::Value;

#[test]
fn idle_unquoted_receiving_scopes_leave_no_held_share_on_the_source(){
 let law:Value=serde_json::from_str(include_str!("🧫️fixtures/receiving-refund.json")).unwrap();let n=|key:&str|law[key].as_u64().unwrap();
 let mut caller=caller();let mut decoder=caller.admitted_decoder();let mut settled=0;
 for _ in 0..100000{
  if settled==n("scopes"){break}
  let mut ran=false;let mut held=None;
  measured(&mut caller,&mut decoder,|cx|{let mut out=None;account(cx,|cx|cx.with_native_receiving(&mut out,|native,_|{ran=true;native.checkpoint()}))?;account(cx,|_|drop(out));if ran{cx.consume_receiving_retained(cx.receiving_issued_grant().unwrap(),RetainedCloneProgress::default())?;let snapshot=testkit::return_budget_snapshot(cx);held=Some((snapshot.held_items+snapshot.held_copy_bytes+snapshot.held_capacity_bytes+snapshot.held_release_bytes,snapshot.held_fuel,snapshot.claims));}Ok(())});
  if let Some((currency,fuel,claims))=held{settled+=1;assert_eq!((currency as u64,fuel,claims as u64),(n("heldAfterSettle"),n("heldAfterSettle"),n("claimsAfterSettle")),"scope {settled} kept its unspent share");}
 }
 assert_eq!(settled,n("scopes"));close(caller,decoder);
}

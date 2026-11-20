use super::*;
use semio_framework_value::retirement::{RetirementCursor,RetirementStep};
use semio_framework_value::AuthoringWallet;

fn close_turn(cursor: &mut PublicationHashRetirement, wallet: &mut AuthoringWallet, grant: RetainedCloneGrant) -> RetirementStep {
 let done = wallet.run_invoked(grant, 1, |effect| {
  let mut result = None;
  let complete = cursor.close_into(grant.maximum_copy_bytes, effect, OriginalEffectReply::borrow(&mut result).unwrap()).unwrap();
  assert!(complete.is_recorded());
  drop(complete);
  result.unwrap().unwrap()
 }).unwrap().unwrap();
 if let RetirementStep::Progress(progress, _) = &done.output {
  wallet.post(*progress).unwrap();
 }
 done.output
}

#[test]
fn publication_hash_fixed_header_clear_has_no_payload_copy(){
 let expected:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();
 let mut cursor=PublicationHashRetirement{value:Some(PublicationHash(semio_framework_hash::Sha256::new()))};
 let grant=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
 let mut wallet=AuthoringWallet::new(grant,2);
 let denied=close_turn(&mut cursor,&mut wallet,RetainedCloneGrant::default());assert!(cursor.value.is_some());assert!(matches!(&denied,RetirementStep::BudgetExhausted));assert_eq!(cursor.next_work_byte_demand().unwrap(),0);
 let clear=close_turn(&mut cursor,&mut wallet,grant);assert!(cursor.terminal_is_empty());
 let cached=close_turn(&mut cursor,&mut wallet,RetainedCloneGrant::default());
 let actual=serde_json::json!({"role":"publication","turns":[{"case":"denied","work":denied.work(),"items":0,"copyBytes":0},{"case":"clear","work":clear.work(),"items":1,"copyBytes":0},{"case":"cached","work":cached.work(),"items":0,"copyBytes":0}]});assert_eq!(actual,expected);assert!(matches!(clear,RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:0,..},1)));
}

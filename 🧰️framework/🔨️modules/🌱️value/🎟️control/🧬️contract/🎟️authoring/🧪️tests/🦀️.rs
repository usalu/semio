//! 🧪️ The authoring wallet debits exactly what its receipts record, refuses what its explicit stock cannot fund and never lends a free effect.
use super::*;
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:8,maximum_copy_bytes:1024,maximum_capacity_bytes:1024,maximum_release_bytes:1024,maximum_depth:4}}
#[test]
fn performed_work_is_debited_exactly_once(){
 let mut wallet=AuthoringWallet::new(stock(),100);
 let done=wallet.run(stock(),50,|effect|effect.performed(7)).unwrap().expect("admitted");
 assert_eq!((done.output.unwrap(),done.performed,wallet.remaining_work()),(true,7,93));
}
#[test]
fn an_effect_without_work_is_free_and_leaves_the_stock_whole(){
 let mut wallet=AuthoringWallet::new(stock(),100);
 let done=wallet.run(stock(),10,|effect|effect.grant()).unwrap().expect("admitted");
 assert_eq!((done.output,done.performed,wallet.remaining_work()),(stock(),0,100));
}
#[test]
fn what_the_stock_cannot_fund_is_refused_typed(){
 let mut wallet=AuthoringWallet::new(stock(),5);
 assert!(wallet.run(stock(),6,|_|()).is_err());
 let mut wider=stock();wider.maximum_items=9;
 assert!(wallet.run(wider,1,|_|()).is_err());
 assert_eq!(wallet.remaining_work(),5);
}
#[test]
fn physical_progress_is_posted_and_never_exceeds_the_stock(){
 let mut wallet=AuthoringWallet::new(stock(),5);
 wallet.post(RetainedCloneProgress{copied_items:3,copied_bytes:100,..Default::default()}).unwrap();
 assert_eq!(wallet.remaining().maximum_items,5);
 assert!(wallet.post(RetainedCloneProgress{copied_items:6,..Default::default()}).is_err());
 assert_eq!(wallet.remaining().maximum_items,5);
}
#[test]
fn a_suspended_poll_is_refused_not_awaited(){
 let mut wallet=AuthoringWallet::new(stock(),5);
 let pending=wallet.run_settled::<(),()>(stock(),1,|_|RetainedWorkPoll::Pending{work:0});
 assert!(pending.is_err());
 let complete=wallet.run_settled::<(),()>(stock(),1,|_|RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(()),work:0})).unwrap().expect("admitted");
 assert_eq!(complete.0.work,0);
}
#[test]
fn work_recorded_before_a_refused_settlement_is_still_debited(){
 let mut wallet=AuthoringWallet::new(stock(),100);
 assert!(wallet.run(stock(),50,|effect|effect.performed(60)).is_err());
 assert_eq!(wallet.remaining_work(),40);
}

#[test]
fn the_shared_corpus_settles_identically_through_the_authoring_wallet(){
 let corpus:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/🪙️effect-provider/🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){
  let maximum=case["maximumWork"].as_u64().unwrap();let mut wallet=AuthoringWallet::new(stock(),100);let steps:Vec<u64>=case["performs"].as_array().unwrap().iter().map(|step|step.as_u64().unwrap()).collect();
  let done=wallet.run(stock(),maximum,|effect|steps.iter().all(|step|effect.performed(*step).unwrap())).unwrap().expect("admitted");
  let performed=u128::from(case["performed"].as_u64().unwrap());
  assert!(done.output,"{}",case["name"]);assert_eq!(done.performed,performed,"{}",case["name"]);assert!(done.performed<=u128::from(maximum));assert_eq!(wallet.remaining_work(),100-performed,"{}",case["name"]);
 }
}
#[test]
fn an_overshooting_child_still_pays_the_reply_handoff(){
 let(grant,maximum)=invocation_extent(stock(),2).unwrap();
 for spend in [2u64,3,5]{
  let remaining=100u128;let mut authority=OriginalWorkAuthority::new(AuthoringDriver);let mut spent=0u128;
  let settled=authority.with_loan(&mut AuthoringContext{remaining:&remaining},|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);run_original_effect(&mut work,grant,maximum,&mut spent,|effect|invoke_original(effect,Some((stock(),2)),|inner|inner.performed(spend)))});
  assert!(!authority.has_live_actions());
  let expected=u128::from(spend)+u128::from(ORIGINAL_INVOCATION_WORK);
  if spend<=2{assert_eq!(settled.unwrap().unwrap().unwrap().performed,expected)}else{assert!(settled.unwrap().is_err());assert_eq!(spent,expected)}
 }
}

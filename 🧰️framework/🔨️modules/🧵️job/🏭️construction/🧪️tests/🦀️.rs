//! 🧫️ The lower native gate consumes the same neutral axes and authentic fixture roots.
use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::{OriginalJobFixtureCaller,OriginalJobDecodeOwner};
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100_000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128}}
fn clock()->Option<u64>{Some(1)}
fn finish(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){for _ in 0..10000{if decoder.terminal_is_empty(){return}caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{let demand=decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;crate::close_original_retained_step(cx,demand,|grant|decoder.close_step(grant))?;Ok(())})).unwrap();}panic!("original construction decoder close must terminate")}
#[test]
fn defining_five_currency_quotes_match_the_language_neutral_cases(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🏭️construction/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){let q=|key:&str,index:usize|row[key][index].as_u64().unwrap()as usize;let demand=OriginalConstructionDemand{items:q("quote",0),copy_bytes:q("quote",1),capacity_bytes:q("quote",2),release_bytes:q("quote",3),depth:q("quote",4)};let grant=RetainedCloneGrant{maximum_items:q("grant",0),maximum_copy_bytes:q("grant",1),maximum_capacity_bytes:q("grant",2),maximum_release_bytes:q("grant",3),maximum_depth:q("grant",4)};assert_eq!(demand.fits(grant),row["accepted"].as_bool().unwrap(),"{}",row["name"]);}
}
#[test]
fn distinct_authentic_roots_with_identical_semantic_ids_do_not_match(){
 let mut original=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),stock(),clock,u64::MAX,10000);
 let mut foreign=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),stock(),clock,u64::MAX,10000);
 let binding=original.turn(stock(),|cx|Ok(OriginalSourceBinding::borrow(cx))).unwrap();
 original.turn(stock(),|cx|{assert!(binding.matches(cx));Ok(())}).unwrap();assert_eq!(original.receipt(),Default::default());
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign.turn(stock(),|cx|{assert!(!binding.matches(cx));Ok(())}));result.unwrap();assert_eq!(heap,Default::default());assert_eq!(foreign.receipt(),Default::default());
 foreign.retire(stock());original.retire(stock());
}
#[test]
fn over_quote_preserves_the_born_candidate_and_actual_original_decoder_receipt(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(73),Generation(5),stock(),clock,u64::MAX,100000);let mut decoder=OriginalJobDecodeOwner::empty();let mut output:Option<Box<[u8;70000]>>=None;
 let demand=OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:69999,release_bytes:0,depth:1};
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(stock(),&mut decoder,|cx|admit_original_construction(cx,&mut output,demand,|_|Ok((Box::new([7;70000]),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:70000,..Default::default()})))));
 let failure=result.unwrap_err();assert!(failure.primary_receiving().unwrap().quote_refusal.is_some());assert_eq!(decoder.owned_bytes(),70000);assert_eq!(caller.receipt().retained_capacity_bytes,70000);assert_eq!((heap.requested_bytes,heap.released_bytes),(70000,0));assert_eq!(output.as_deref().unwrap()[69999],7);
 caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{crate::close_original_retained_step(cx,RetirementDemand{release_bytes:70000,depth:1,..Default::default()},|_|{drop(output.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:70000,..Default::default()}))})?;Ok(())})).unwrap();drop(failure);finish(&mut caller,&mut decoder);caller.retire(stock());
}

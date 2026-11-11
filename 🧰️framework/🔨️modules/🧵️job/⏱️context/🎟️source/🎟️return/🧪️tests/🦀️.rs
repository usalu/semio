use semio_framework_job_context::testkit;
use super::*;
#[test]
fn actual_original_source_binding_is_paid_once_without_renewing_its_lifetime_stock(){
 let row:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let s=&row["stock"];let stock=RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize};
 let mut caller=crate::OriginalJobFixtureCaller::admit(OperationId(row["operation"].as_u64().unwrap()),Generation(row["generation"].as_u64().unwrap()),stock,||Some(1),u64::MAX,s["fuel"].as_u64().unwrap());let quantum=crate::quantum(&mut caller,stock);
 caller.turn(RetainedCloneGrant{maximum_items:0,..quantum},|cx|{let before=cx.fuel_remaining();assert!(!testkit::admit_return_origin(cx)?);assert_eq!(before-cx.fuel_remaining(),row["deniedWork"].as_u64().unwrap());assert!(!testkit::return_budget_is_bound(cx));Ok(())}).unwrap();
 let bound=caller.turn(quantum,|cx|{let before=cx.fuel_remaining();assert!(testkit::admit_return_origin(cx)?);assert_eq!(before-cx.fuel_remaining(),row["bindWork"].as_u64().unwrap());let state=testkit::return_budget_snapshot(cx);assert!(state.conserved());assert_eq!(state.granted,stock);assert_eq!(state.held_items,0);assert_eq!(state.claims,0);Ok(state)}).unwrap();
 caller.turn(quantum,|cx|{let before=cx.fuel_remaining();assert!(testkit::admit_return_origin(cx)?);assert_eq!(before-cx.fuel_remaining(),row["cachedWork"].as_u64().unwrap());let state=testkit::return_budget_snapshot(cx);assert_eq!(state,bound);assert_eq!(cx.retained_progress().copied_bytes,row["copyBytes"].as_u64().unwrap()as usize);Ok(())}).unwrap();caller.retire(stock);
}

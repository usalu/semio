//! 🧾️ The receiving scope charges only the construction spend its semantic receipt has not already posted, and an unsettled scope refuses the next one.
use super::*;
fn progress(items:usize,bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:items,copied_bytes:bytes,..Default::default()}}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:8,maximum_copy_bytes:64,maximum_capacity_bytes:64,maximum_release_bytes:64,maximum_depth:4}}
#[test]
fn settle_owes_only_the_receipt_beyond_the_already_accepted_construction_spend(){
 let mut scope=OriginalReceivingScope::default();scope.open(grant()).unwrap();*scope.spent_mut()=progress(1,8);
 assert_eq!(scope.settle(grant(),progress(3,24)).unwrap(),progress(2,16));
 assert!(scope.issued().is_none());assert_eq!(scope.spent(),RetainedCloneProgress::default());
}
#[test]
fn an_unsettled_scope_refuses_the_next_open_and_a_receipt_below_accepted_spend_is_refused(){
 let mut scope=OriginalReceivingScope::default();scope.open(grant()).unwrap();*scope.spent_mut()=progress(2,8);
 assert!(scope.open(grant()).is_err());assert!(scope.settle(grant(),progress(1,8)).is_err());
}

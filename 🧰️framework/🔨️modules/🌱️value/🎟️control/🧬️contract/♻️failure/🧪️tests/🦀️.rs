//! 🧳️ Original allocation refusals remain distinct owners through paid failure attachment.
use super::*;
use crate::{NativeAllocationRefusal as Refusal,NativeAllocationRefusalVault as Vault};
#[test]
fn allocation_cause_preserves_original_owner_and_explicit_parent_progress(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("allocation-custody.json")).unwrap();
 let f=&cases[0];let cause=|key:&str|ValueError::new(ValueRefusalKind::InvariantViolated,f[key].as_str().unwrap().to_owned());
 let allocation=ValueError::new(ValueRefusalKind::OwnershipLimit,String::from("original allocation"));let pointer=allocation.message.as_ptr();
 let progress=RetainedCloneProgress{copied_bytes:f["selected"].as_u64().unwrap()as usize,..Default::default()};
 let child=OriginalRetirementFailure::from_primary(ValueError::literal(ValueRefusalKind::Canceled,"child").with_retained_progress(progress));
 let mut actual=None;let storage=admit_node(OriginalFailureStorage::birth_bytes(),&mut actual).result.unwrap().0;assert_eq!(actual,Some(OriginalFailureStorage::birth_bytes()));
 let parent=storage.attach_owned_parts(child,Some(cause("primary")),Some(cause("work")),Some(cause("receipt")),Some(Refusal::Owned(allocation))).with_retained_progress(RetainedCloneProgress{copied_bytes:f["override"].as_u64().unwrap()as usize,..Default::default()});
 assert_eq!(parent.allocation_refusal().unwrap().owned().unwrap().message.as_ptr(),pointer);assert_eq!(parent.child().unwrap().retained_progress(),progress);
 let(a,b,c,d,child,over)=parent.into_parts();let restored=OriginalRetirementFailure::from_owned_parts(a,b,c,d,child,over);
 assert_eq!(restored.primary().unwrap().message,"allocator");assert_eq!(restored.work_refusal().unwrap().message,"payer");assert_eq!(restored.receipt_refusal().unwrap().message,"physical receipt");assert_eq!(restored.retained_progress().copied_bytes,11);assert_eq!(restored.allocation_refusal().unwrap().owned().unwrap().message.as_ptr(),pointer);
}
#[test]
fn retained_allocation_cause_returns_only_to_its_original_vault(){
 let mut original=Vault::default();let mut foreign=Vault::default();let error=ValueError::new(ValueRefusalKind::OwnershipLimit,String::from("original refusal"));let pointer=error.message.as_ptr();let handle=original.retain(error).unwrap();
 let parent=OriginalRetirementFailure::from_allocation_refusal(Refusal::Retained(handle));assert!(parent.allocation_refusal().unwrap().owned().is_none());
 let(a,b,c,d,child,over)=parent.into_parts();assert!(a.is_none()&&b.is_none()&&c.is_none()&&child.is_none()&&over.is_none());
 let handle=d.unwrap().into_owned().unwrap_err();let handle=foreign.take(handle).unwrap_err();assert!(!original.terminal_is_empty());let error=original.take(handle).unwrap();assert_eq!(error.message.as_ptr(),pointer);assert!(original.terminal_is_empty()&&foreign.terminal_is_empty());
}

struct NodeWork;
impl crate::OriginalWorkRecipient for NodeWork{
 type Context<'turn,'source>=() where 'source:'turn;
 fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn validate_context<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,quote:crate::OriginalWorkQuote<'reply>,_:u64,_:u64)->crate::OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(true))}
 fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(Ok(()))}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.pending()}
}
pub(super) fn admit_node(capacity:usize,actual:&mut Option<usize>)->RetainedWorkOutcome<(OriginalFailureStorage,RetainedCloneProgress)>{let mut authority=crate::OriginalWorkAuthority::new(NodeWork);authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=crate::OriginalWork{state:crate::OriginalWorkState::fresh(),recipient:Some(&mut loan)};let(mut action,mut receipt,mut proof)=(None,None,None);assert_eq!(work.prepare_for_action(1,&mut action).unwrap(),Some(true));work.issue_performed_for_action(action,&mut receipt).unwrap();let grant=crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:OriginalFailureStorage::admission_demand().copy_bytes,maximum_capacity_bytes:capacity,maximum_depth:1,..Default::default()};let turn=OriginalFailureStorage::admit(&mut crate::RetainedAllocationEffect::new(grant,1,actual,receipt.as_mut().unwrap()));work.claim_performed_for_action(action,&mut receipt).unwrap();let performed=work.performed_total_for_action(action).unwrap();assert_eq!(turn.work,performed);if work.performed_seen_for_action(action).unwrap(){let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());assert!(partial.take().unwrap().source_refusal().is_none());assert!(work.commit_for_action(performed,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap()}else{work.cancel_for_action(&mut action).unwrap()}turn}).unwrap()}

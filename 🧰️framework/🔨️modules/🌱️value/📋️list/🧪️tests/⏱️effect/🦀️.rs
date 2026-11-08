//! ⏱️ Test-only grant adapters run one list action inside one real original effect and keep every grant-level law. See [`semio_framework_value_clone_source_testing::with_effect`] (a dev-dependency cycle forbids using it here).
use semio_framework_value_control_contract::{OriginalSourceTurnIdentity,OriginalWork,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,OriginalWorkSink,RetainedAllocationEffect,RetainedCloneGrant,ValueError};
use semio_framework_value_control_contract::RetainedCloneProgress;
use crate::{PageAllocation,PagedList,PagedListAllocationError,PagedListError,PagedListProgress,PagedListReleaseFailure,PagedListReleaseOutcome,PagedListReservationFailure,PagedListReservationOutcome};
pub const CPU:u64=1048576;
pub const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1048576,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:33554432,maximum_depth:256};
struct Producer{maximum:u64}
impl OriginalWorkRecipient for Producer{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.maximum)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
/// 🪢️ Runs one operation inside one actual original effect over the caller's allocation marker.
pub fn with_effect<R>(grant:RetainedCloneGrant,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
 let mut authority=OriginalWorkAuthority::new(Producer{maximum});
 let result=authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;assert_eq!(work.prepare_for_action(maximum,&mut action).unwrap(),Some(true));let mut receipt=None;work.issue_performed_for_action(action,&mut receipt).unwrap();let output={let mut effect=RetainedAllocationEffect::new(grant,maximum,actual,receipt.as_mut().unwrap());operation(&mut effect)};let performed=OriginalWorkSink::observed_work(receipt.as_ref().unwrap());work.claim_performed_for_action(action,&mut receipt).unwrap();if performed==0{work.cancel_for_action(&mut action).unwrap()}else{let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let partial=partial.take().expect("partial outcome");assert_eq!(partial.total(),performed);assert!(partial.source_refusal().is_none());assert!(work.commit_for_action(performed,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap()}output}).unwrap();
 assert!(!authority.has_live_actions());result
}
/// 🎟️ The grant a byte-priced list action receives: one item, the named allocation currency and generous others.
pub fn bytes(capacity:usize,release:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:64}}
fn reservation(outcome:semio_framework_value_control_contract::RetainedWorkOutcome<PagedListReservationOutcome,PagedListReservationFailure>)->Result<PagedListProgress,PagedListAllocationError>{match outcome.result{Ok(PagedListReservationOutcome::Ready(p))=>Ok(p),Ok(PagedListReservationOutcome::Denied)=>Ok(PagedListProgress::default()),Err(PagedListReservationFailure::Allocation{error,..})=>Err(error),Err(PagedListReservationFailure::Work(error))=>panic!("original work refused: {error:?}")}}
/// 🧾️ Grant-level views over the effect-based list actions.
pub trait Granted<T,const N:usize>{
 fn reserve_one_granted(&mut self,bytes:usize,actual:&mut Option<usize>)->Result<PagedListProgress,PagedListAllocationError>;
 fn reserve_one_using_granted<A:PageAllocation>(&mut self,bytes:usize,actual:&mut Option<usize>)->Result<PagedListProgress,PagedListAllocationError>;
 fn reserve_capacity_one_granted(&mut self,length:usize,bytes:usize,actual:&mut Option<usize>)->Result<PagedListProgress,PagedListAllocationError>;
 fn reserve_exact_capacity_one_granted(&mut self,length:usize,bytes:usize,actual:&mut Option<usize>)->Result<PagedListProgress,PagedListAllocationError>;
 fn release_empty_page_granted(&mut self,bytes:usize)->Result<PagedListProgress,PagedListError>;
 fn reserve_one_funded_granted(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->Result<RetainedCloneProgress,PagedListReservationFailure>;
 fn reserve_capacity_one_funded_granted(&mut self,length:usize,grant:RetainedCloneGrant,actual:&mut Option<usize>)->Result<RetainedCloneProgress,PagedListReservationFailure>;
 fn adopt_reserved_granted(&mut self,source:&mut Option<T>)->Result<PagedListProgress,ValueError>;
}
fn funded(outcome:semio_framework_value_control_contract::RetainedWorkOutcome<PagedListReservationOutcome<RetainedCloneProgress>,PagedListReservationFailure>)->Result<RetainedCloneProgress,PagedListReservationFailure>{outcome.result.map(|o|match o{PagedListReservationOutcome::Ready(p)=>p,PagedListReservationOutcome::Denied=>RetainedCloneProgress::default()})}
impl<T,const N:usize> Granted<T,N> for PagedList<T,N>{
 fn reserve_one_granted(&mut self,bytes:usize,actual:&mut Option<usize>)->Result<PagedListProgress,PagedListAllocationError>{reservation(with_effect(bytes_grant(bytes),CPU,actual,|e|self.reserve_one(e)))}
 fn reserve_one_using_granted<A:PageAllocation>(&mut self,bytes:usize,actual:&mut Option<usize>)->Result<PagedListProgress,PagedListAllocationError>{reservation(with_effect(bytes_grant(bytes),CPU,actual,|e|self.reserve_one_using::<A>(e)))}
 fn reserve_capacity_one_granted(&mut self,length:usize,bytes:usize,actual:&mut Option<usize>)->Result<PagedListProgress,PagedListAllocationError>{reservation(with_effect(bytes_grant(bytes),CPU,actual,|e|self.reserve_capacity_one(length,e)))}
 fn reserve_exact_capacity_one_granted(&mut self,length:usize,bytes:usize,actual:&mut Option<usize>)->Result<PagedListProgress,PagedListAllocationError>{reservation(with_effect(bytes_grant(bytes),CPU,actual,|e|self.reserve_exact_capacity_one(length,e)))}
 fn release_empty_page_granted(&mut self,bytes:usize)->Result<PagedListProgress,PagedListError>{let mut actual=None;match with_effect(self::bytes(0,bytes),CPU,&mut actual,|e|self.release_empty_page(e)).result{Ok(PagedListReleaseOutcome::Ready(p))=>Ok(p),Ok(PagedListReleaseOutcome::Denied)=>Ok(PagedListProgress::default()),Err(PagedListReleaseFailure::Release(error))=>Err(error),Err(PagedListReleaseFailure::Work(error))=>panic!("original work refused: {error:?}")}} fn reserve_one_funded_granted(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->Result<RetainedCloneProgress,PagedListReservationFailure>{funded(with_effect(grant,CPU,actual,|e|self.reserve_one_funded(e)))}
 fn reserve_capacity_one_funded_granted(&mut self,length:usize,grant:RetainedCloneGrant,actual:&mut Option<usize>)->Result<RetainedCloneProgress,PagedListReservationFailure>{funded(with_effect(grant,CPU,actual,|e|self.reserve_capacity_one_funded(length,e)))}
 fn adopt_reserved_granted(&mut self,source:&mut Option<T>)->Result<PagedListProgress,ValueError>{let mut actual=None;with_effect(STOCK,CPU,&mut actual,|e|self.adopt_reserved(source,e)).result}
}
fn bytes_grant(capacity:usize)->RetainedCloneGrant{bytes(capacity,0)}

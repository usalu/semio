//! ♻️ Existing encoder laws close typed owners with unchanged turn ceilings and finite cumulative stock.
use crate::{RetainedCloneGrant,eff,retirement::{RetireOwned,controlled::ControlledRetirement}};
pub(crate) fn close<T:RetireOwned>(value:T,turn:RetainedCloneGrant,mut work:u64){
 let stock=|amount:usize|amount.checked_mul(4096).expect("finite test close stock overflow");
 let mut bank=RetainedCloneGrant{maximum_items:stock(turn.maximum_items),maximum_copy_bytes:stock(turn.maximum_copy_bytes),maximum_capacity_bytes:stock(turn.maximum_capacity_bytes),maximum_release_bytes:stock(turn.maximum_release_bytes),maximum_depth:turn.maximum_depth};
 let(owner,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||ControlledRetirement::new(value).ok().expect("fixture owner must support typed retirement"));assert_eq!(heap,(0,0));let mut owner=owner;
 for _ in 0..4096{
  if owner.terminal_is_empty(){return}
  let grant=RetainedCloneGrant{maximum_items:turn.maximum_items.min(bank.maximum_items),maximum_copy_bytes:turn.maximum_copy_bytes.min(bank.maximum_copy_bytes),maximum_capacity_bytes:turn.maximum_capacity_bytes.min(bank.maximum_capacity_bytes),maximum_release_bytes:turn.maximum_release_bytes.min(bank.maximum_release_bytes),maximum_depth:turn.maximum_depth};
  let quote=owner.next_computational_work(grant.maximum_copy_bytes).expect("pending callbacks require their original issuer before close");assert!(quote<=work,"original test close work stock exhausted");
  let mut actual=None;let(poll,heap)=eff::observed(grant,quote,&mut actual,|effect|owner.step(effect));let p=eff::settled(poll).result.expect("actual typed fixture close refused").progress();let spent=u64::try_from(owner.last_turn_work()).expect("fixture close work fits");assert!(p.fits(grant));assert!(spent<=quote);assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert_eq!(actual,if heap.0==0{None}else{Some(heap.0)});assert!(owner.terminal_is_empty()||p!=Default::default()||spent>0,"fixture close made no admitted progress");
  bank.maximum_items-=p.copied_items;bank.maximum_copy_bytes-=p.copied_bytes;bank.maximum_capacity_bytes-=p.retained_capacity_bytes;bank.maximum_release_bytes-=p.released_bytes;work-=spent;
 }
 assert!(owner.terminal_is_empty(),"finite fixture close turn stock exhausted");
}

//! 🎟️ Actual work authority moves from an issued reservation into its original committed proof.
use super::{ValueError,ValueRefusalKind};
use std::sync::atomic::{AtomicU64,Ordering};
fn next_identity(counter:&AtomicU64)->u64{counter.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|id|id.checked_add(1)).expect("original work identity space exhausted")}
fn refused()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original work action requires its same live scope and issuer")}
/// 🪪️ Read-only scope coordinates grant no work authority.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalWorkScopeIdentity(u64);
impl OriginalWorkScopeIdentity{pub fn get(self)->u64{self.0}}
/// 🪪️ Read-only source coordinates grant no action authority.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalWorkIssuerIdentity(u64);
impl OriginalWorkIssuerIdentity{pub fn get(self)->u64{self.0}}
/// 🧭️ One non-cloneable original operation owns its work counter and all unfinished actions.
#[derive(Debug)]
pub struct OriginalWorkScope{identity:OriginalWorkScopeIdentity,open:usize,actual:u128}
impl OriginalWorkScope{
 pub fn new()->Self{static NEXT:AtomicU64=AtomicU64::new(1);Self{identity:OriginalWorkScopeIdentity(next_identity(&NEXT)),open:0,actual:0}}
 pub fn identity(&self)->OriginalWorkScopeIdentity{self.identity}
 pub fn actual_work(&self)->u128{self.actual}
 pub fn has_live_actions(&self)->bool{self.open!=0}
}
impl Default for OriginalWorkScope{fn default()->Self{Self::new()}}
impl Drop for OriginalWorkScope{fn drop(&mut self){assert!(std::thread::panicking()||self.open==0,"original work scope retains an issued action or committed proof")}}
/// 🏷️ Read-only issued coordinates cannot construct a reservation or proof.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalWorkActionIdentity{issuer:OriginalWorkIssuerIdentity,scope:OriginalWorkScopeIdentity,serial:u64}
/// 🎟️ The actual issuer creates this non-cloneable reservation against the original scope.
#[derive(Debug)]
pub struct OriginalWorkActionReservation{identity:OriginalWorkActionIdentity,source_turn:Option<crate::OriginalSourceTurnIdentity>,reserved:u64,observed:Option<u128>,paid:u128,partial:Option<OriginalWorkPartialReceipt>,partial_seen:bool,performed_issued:Option<u64>,next_performed:u64,live:bool,quote_started:bool,quote_entered:bool,quote_observed:Option<u64>,quote_consumed:bool,quote_reply:Option<Result<bool,ValueError>>}
impl OriginalWorkActionReservation{
 pub fn action_identity(&self)->OriginalWorkActionIdentity{self.identity}
 pub fn issuer_identity(&self)->OriginalWorkIssuerIdentity{self.identity.issuer}
 pub fn reserved_work(&self)->u64{self.reserved}
 pub fn observed_work(&self)->Option<u128>{self.observed}
 pub fn paid_work(&self)->u128{self.paid}
 pub fn matches_scope(&self,scope:&OriginalWorkScope)->bool{self.identity.scope==scope.identity}
}
impl Drop for OriginalWorkActionReservation{fn drop(&mut self){assert!(std::thread::panicking()||!self.live,"original work action retains its unreturned authority")}}
/// 📬️ A performed work delta remains owned until its original Source verdict is transferred.
#[derive(Debug)]
struct OriginalWorkPartialReceipt{serial:u64,delta:u128,total:u128,admitted:bool,unaccounted:Option<u64>,started:bool,entered:bool,reply:Option<Result<(),ValueError>>}
/// 🧾️ Returns the real charged delta and independent original Source refusal.
#[derive(Debug)]
pub struct OriginalWorkPartialOutcome{identity:OriginalWorkPerformedIdentity,delta:u128,total:u128,admitted:bool,source_refusal:Option<ValueError>}
impl OriginalWorkPartialOutcome{
 pub fn identity(&self)->OriginalWorkPerformedIdentity{self.identity}
 pub fn delta(&self)->u128{self.delta}
 pub fn total(&self)->u128{self.total}
 pub fn admitted(&self)->bool{self.admitted}
 pub fn source_refusal(&self)->Option<&ValueError>{self.source_refusal.as_ref()}
 pub fn into_parts(self)->(OriginalWorkPerformedIdentity,u128,u128,bool,Option<ValueError>){(self.identity,self.delta,self.total,self.admitted,self.source_refusal)}
}
/// 🧾️ The same issued action remains owned after CPU credit until its final callback returns.
#[derive(Debug)]
pub struct OriginalWorkActionProof{action:OriginalWorkActionReservation}
impl OriginalWorkActionProof{
 pub fn action_identity(&self)->OriginalWorkActionIdentity{self.action.identity}
 pub fn actual_work(&self)->u128{self.action.observed.expect("committed original action")}
 pub fn reserved_work(&self)->u64{self.action.reserved}
}
/// 🏛️ A real caller-owned issuer mints actions and consumes their exact returned authority.
#[derive(Debug)]
pub struct OriginalWorkActionIssuer{identity:OriginalWorkIssuerIdentity,next_action:u64,reserved:usize,committed:usize}
impl OriginalWorkActionIssuer{
 pub fn new()->Self{static NEXT:AtomicU64=AtomicU64::new(1);Self{identity:OriginalWorkIssuerIdentity(next_identity(&NEXT)),next_action:1,reserved:0,committed:0}}
 pub const fn birth_bytes()->usize{std::mem::size_of::<Self>()}
 pub fn identity(&self)->OriginalWorkIssuerIdentity{self.identity}
 pub fn reserved_actions(&self)->usize{self.reserved}
 pub fn committed_actions(&self)->usize{self.committed}
 pub fn has_live_actions(&self)->bool{self.reserved!=0||self.committed!=0}
 pub fn outstanding(&self)->usize{self.reserved+self.committed}
 pub fn issue(&mut self,scope:&mut OriginalWorkScope,work:u64)->Result<OriginalWorkActionReservation,ValueError>{
  self.reserved.checked_add(self.committed).and_then(|count|count.checked_add(1)).ok_or_else(refused)?;let next=self.next_action.checked_add(1).ok_or_else(refused)?;let reserved=self.reserved.checked_add(1).ok_or_else(refused)?;let open=scope.open.checked_add(1).ok_or_else(refused)?;let serial=self.next_action;self.next_action=next;self.reserved=reserved;scope.open=open;
  Ok(OriginalWorkActionReservation{identity:OriginalWorkActionIdentity{issuer:self.identity,scope:scope.identity,serial},source_turn:None,reserved:work,observed:None,paid:0,partial:None,partial_seen:false,performed_issued:None,next_performed:1,live:true,quote_started:false,quote_entered:false,quote_observed:None,quote_consumed:false,quote_reply:None})
 }
 pub fn owns_reservation(&self,scope:&OriginalWorkScope,action:&OriginalWorkActionReservation)->bool{action.live&&action.identity.issuer==self.identity&&action.identity.scope==scope.identity&&action.identity.serial<self.next_action&&self.reserved!=0&&scope.open!=0}
 pub fn cancel(&mut self,scope:&mut OriginalWorkScope,mut action:OriginalWorkActionReservation)->Result<(),(ValueError,OriginalWorkActionReservation)>{
  if !self.owns_reservation(scope,&action)||action.observed.is_some()||action.partial_seen||action.performed_issued.is_some()||action.quote_reply.is_some(){return Err((refused(),action))}self.reserved-=1;scope.open-=1;action.live=false;Ok(())
 }
 pub fn commit(&mut self,scope:&mut OriginalWorkScope,mut action:OriginalWorkActionReservation,actual:u128,output:&mut Option<OriginalWorkActionProof>)->Result<bool,(ValueError,OriginalWorkActionReservation)>{
  if !self.owns_reservation(scope,&action)||action.quote_reply.is_some()||action.partial.is_some()||action.performed_issued.is_some()||actual<action.paid||action.observed.is_some_and(|observed|observed!=actual){return Err((refused(),action))}
  if action.observed.is_none(){let Some(total)=scope.actual.checked_add(actual-action.paid)else{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original actual work counter overflow"),action))};scope.actual=total;action.observed=Some(actual)}
  if output.is_some(){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original committed proof output is occupied"),action))}
  let Some(committed)=self.committed.checked_add(1)else{return Err((refused(),action))};let admitted=actual<=u128::from(action.reserved);self.reserved-=1;self.committed=committed;*output=Some(OriginalWorkActionProof{action});Ok(admitted)
 }
 pub fn validates(&self,scope:&OriginalWorkScope,proof:&OriginalWorkActionProof,actual:u128)->bool{proof.action.live&&proof.action.identity.issuer==self.identity&&proof.action.identity.scope==scope.identity&&proof.action.identity.serial<self.next_action&&proof.action.observed==Some(actual)&&self.committed!=0&&scope.open!=0}
 pub fn finish(&mut self,scope:&mut OriginalWorkScope,proof:&mut Option<OriginalWorkActionProof>)->Result<(),ValueError>{
  let Some(original)=proof.as_ref()else{return Ok(())};if !self.validates(scope,original,original.actual_work()){return Err(refused())}let mut original=proof.take().expect("validated original proof");self.committed-=1;scope.open-=1;original.action.live=false;Ok(())
 }
}
impl Default for OriginalWorkActionIssuer{fn default()->Self{Self::new()}}
impl Drop for OriginalWorkActionIssuer{fn drop(&mut self){assert!(std::thread::panicking()||!self.has_live_actions(),"original work issuer retains an issued action or committed proof")}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="🤝️loan/🦀️.rs"]
mod loan;
pub use loan::{OriginalWorkAuthority,OriginalWorkRecipient,OriginalWorkLoan,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete};

#[path="⚙️operation/🦀️.rs"]
mod operation;
pub use operation::{OriginalWork,OriginalWorkState,OriginalWorkTurn,OriginalWorkPerformedReceipt,OriginalWorkPerformedIdentity,OriginalWorkSink};

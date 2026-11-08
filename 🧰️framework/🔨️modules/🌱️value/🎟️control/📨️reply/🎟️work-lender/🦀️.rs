//! 🧵️ Defines the same native performed-receipt lender used by borrowed replies.
#[macro_export]
macro_rules! original_work_lender_engine {()=>{
/// 🧵️ Lends sequential access to the same original receipt without issuing new Work.
pub struct OriginalWorkLender<'receipt>{receipt:std::sync::Mutex<&'receipt mut OriginalWorkPerformedReceipt>,limit:u128}
impl<'receipt> OriginalWorkLender<'receipt>{
 /// 🪢️ Holds the actual issued receipt and one shared Work window for all shorter loans.
 pub fn new(receipt:&'receipt mut OriginalWorkPerformedReceipt,maximum:u64)->Result<Self,ValueError>{if maximum>receipt.remaining_work_units()?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared Work window exceeds its original receipt"))}let limit=receipt.observed_work().checked_add(u128::from(maximum)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"shared Work window overflow"))?;Ok(Self{receipt:std::sync::Mutex::new(receipt),limit})}
 /// 🫴️ Lends the same receipt to one native Work consumer.
 pub fn loan(&self)->OriginalWorkLenderLoan<'_,'receipt>{OriginalWorkLenderLoan{lender:self}}
}
/// 🪢️ A short loan to the same receipt; no callback runs while its lock is held.
pub struct OriginalWorkLenderLoan<'loan,'receipt>{lender:&'loan OriginalWorkLender<'receipt>}
impl sealed_work_sink::Sealed for OriginalWorkLenderLoan<'_,'_>{}
impl OriginalWorkSink for OriginalWorkLenderLoan<'_,'_>{
 fn performed(&mut self,delta:u64)->Result<bool,ValueError>{let mut receipt=self.lender.receipt.lock().unwrap_or_else(|poisoned|poisoned.into_inner());let admitted=receipt.performed(delta)?;Ok(admitted&&receipt.observed_work()<=self.lender.limit)}
 fn observed_work(&self)->u128{self.lender.receipt.lock().unwrap_or_else(|poisoned|poisoned.into_inner()).observed_work()}
 fn remaining_work_units(&self)->Result<u64,ValueError>{let receipt=self.lender.receipt.lock().unwrap_or_else(|poisoned|poisoned.into_inner());let local=self.lender.limit.checked_sub(receipt.observed_work()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"shared Work window exceeded"))?;Ok(receipt.remaining_work_units()?.min(u64::try_from(local).map_err(|_|ValueError::invariant("shared Work window overflow"))?))}
 fn identity(&self)->OriginalWorkPerformedIdentity{self.lender.receipt.lock().unwrap_or_else(|poisoned|poisoned.into_inner()).identity()}
 fn source_turn(&self)->Option<crate::OriginalSourceTurnIdentity>{self.lender.receipt.lock().unwrap_or_else(|poisoned|poisoned.into_inner()).source_turn()}
}
}}

/// 🛡️ Defines the actual original Work guard in its lending owner.
#[macro_export]
macro_rules! original_work_performed_scope_engine {()=>{
pub struct OriginalWorkPerformedScope<'a>{state:&'a mut OriginalWorkState,action:Option<OriginalWorkActionIdentity>,receipt:&'a mut Option<OriginalWorkPerformedReceipt>,claim:&'a mut Option<Result<u128,ValueError>>,ceiling:Option<u128>,observed:u128}
impl OriginalWorkPerformedScope<'_>{
 /// 🎟️ Lends the original receipt with an exact remaining window and an admitted physical grant.
 fn receipt_for(&mut self,maximum:u64)->Result<&mut OriginalWorkPerformedReceipt,ValueError>{let receipt=self.receipt.as_mut().expect("original performed receipt remains resident");let unclaimed=OriginalWorkSink::observed_work(receipt).checked_sub(self.observed).ok_or_else(||ValueError::invariant("original performed receipt regressed"))?;let total=self.state.actual().checked_add(unclaimed).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original performed window overflow"))?;if self.ceiling.is_some_and(|ceiling|u128::from(maximum)>ceiling.saturating_sub(total))||maximum>receipt.remaining_work_units()?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original borrowed effect exceeds its remaining work"))}Ok(receipt)}
 pub fn effect<'turn>(&'turn mut self,grant:RetainedCloneGrant,maximum:u64,actual:&'turn mut Option<usize>)->Result<RetainedAllocationEffect<'turn>,ValueError>{Ok(RetainedAllocationEffect::new(grant,maximum,actual,self.receipt_for(maximum)?))}
 /// 🪢️ Lends the marked effect.
 pub fn construction_effect<'turn>(&'turn mut self,loan:&crate::OriginalConstructionLoan<'_>,action:&'turn mut crate::OriginalConstructionAction,maximum:u64)->Result<RetainedAllocationEffect<'turn>,ValueError>{loan.effect_for_action(action,maximum,self.receipt_for(maximum)?)}
}
impl Drop for OriginalWorkPerformedScope<'_>{fn drop(&mut self){let result=self.state.claim_performed_for_action(self.action,self.receipt).and_then(|()|self.state.performed_total_for_action(self.action));*self.claim=Some(result)}}
}}

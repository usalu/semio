//! ♻️ Neutral original cleanup loans preserve the exact borrowed physical recipient across retirement erasure.
use crate::ValueError;
/// 🧷️ This borrowed capability never owns a replacement ledger or independent capacity counter.
pub struct OriginalRetirementCloseLoan<'a>{ledger:u64,recipient:Option<u64>,charge:&'a mut dyn FnMut(usize)->Result<(),ValueError>}
impl<'a> OriginalRetirementCloseLoan<'a>{
 /// 🎟️ Borrows the defining original decoder rather than issuing another physical currency.
 /// # Safety
 /// The ledger, recipient and callback must belong to the same live original decoder retirement loan.
 /// Its caller grant and cumulative admission must remain enforced by that callback for this borrow.
 pub unsafe fn from_original(ledger:u64,recipient:Option<u64>,charge:&'a mut dyn FnMut(usize)->Result<(),ValueError>)->Self{Self{ledger,recipient,charge}}
 pub fn ledger_identity(&self)->u64{self.ledger}
 pub fn retirement_recipient_identity(&self)->Option<u64>{self.recipient}
 pub fn charge(&mut self,bytes:usize)->Result<(),ValueError>{(self.charge)(bytes)}
}

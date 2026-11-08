//! 🪙️ One finite original work stock lends every native test its recipient without per-site producers.
use super::*;
/// 🏦️ Admits work while `observed + additional <= limit`; an optional message makes performed-work settlement refuse.
pub struct FiniteWork{limit:u64,refusal:Option<&'static str>}
impl FiniteWork{fn settle(&self)->Result<(),ValueError>{self.refusal.map_or(Ok(()),|message|Err(ValueError::invariant(message)))}}
impl OriginalWorkRecipient for FiniteWork{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&()) ->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&()) ->Option<OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply> where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.limit)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(self.settle())}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(self.settle())}
}
/// 🪙️ The one authority a test lends with `with_loan(&mut (), ..)`.
pub fn finite_work(limit:u64,refusal:Option<&'static str>)->OriginalWorkAuthority<FiniteWork>{OriginalWorkAuthority::new(FiniteWork{limit,refusal})}

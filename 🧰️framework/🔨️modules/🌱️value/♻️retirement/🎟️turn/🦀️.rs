//! 🎟️ One original owner receives one complete grant and returns its actual custody receipt.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

/// 📬️ The first-party receipt contract shared by native cleanup and hosted lifecycles.
pub trait RetirementReceipt:Sized {
    fn retirement_progress(&self)->Option<RetainedCloneProgress>;
    fn retirement_complete(&self)->bool;
    fn retirement_yield()->Self;
}
impl RetirementReceipt for RetainedCloneStep {
    fn retirement_progress(&self)->Option<RetainedCloneProgress>{Some(self.progress())}
    fn retirement_complete(&self)->bool{matches!(self,Self::Complete(_))}
    fn retirement_yield()->Self{Self::Progress(Default::default())}
}

/// 🧳️ Provider refusals retain their original error independently of receipt violations.
#[derive(Debug)]
pub enum RetirementTurnError<E>{Owner(E),Receipt(ValueError)}

/// 🎛️ Preflights every independent axis before effects and preserves the original grant unchanged.
pub fn advance_retirement_turn<S:RetirementReceipt,E>(demand:RetirementDemand,grant:RetainedCloneGrant,owner:impl FnOnce(RetainedCloneGrant)->Result<(S,bool),E>)->Result<S,RetirementTurnError<E>>{
    if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(S::retirement_yield());}
    let(step,terminal_is_empty)=owner(grant).map_err(RetirementTurnError::Owner)?;
    if let Some(receipt)=step.retirement_progress(){if !receipt.fits(grant){return Err(RetirementTurnError::Receipt(ValueError::literal(ValueRefusalKind::InvariantViolated,"original retirement receipt exceeds its unchanged grant")));}}
    if step.retirement_complete()&&!terminal_is_empty{return Err(RetirementTurnError::Receipt(ValueError::literal(ValueRefusalKind::InvariantViolated,"original retirement completed without its terminal-empty witness")));}
    Ok(step)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

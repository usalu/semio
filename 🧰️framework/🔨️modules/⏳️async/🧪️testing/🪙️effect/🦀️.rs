//! 🪙️ Runs synchronous effect-shaped cancellation closers under one real original action.
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
const WORK:u64=1<<20;
const STEP_WORK:u64=64;
pub struct Turn(EffectBank);
impl Turn{
 pub fn new()->Self{Self(EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<12},WORK)))}
 pub fn run<T,E>(&mut self,grant:RetainedCloneGrant,call:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<T,E>)->RetainedWorkOutcome<T,E>{self.0.run_outcome(grant,STEP_WORK,call)}
}

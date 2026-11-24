//! 🏦️ Effect-shaped test fixtures: one self-funded authoring wallet runs each poll-shaped owner turn under a real invoked effect.
use semio_framework_value::{AuthoringWallet,RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome,RetainedWorkPoll};

pub(crate) const WORK:u64=1<<20;

/// 🏦️ Builds the self-funded wallet whose invoked effects stand for one original caller's turns.
pub(crate) fn wallet()->AuthoringWallet{AuthoringWallet::self_funded()}

/// 🧵️ Runs one poll-shaped turn under exactly `grant`; a refused or pending turn answers `None`.
pub(crate) fn run_turn<T,E>(wallet:&mut AuthoringWallet,grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->Option<RetainedWorkOutcome<T,E>>{
    match wallet.run_invoked(grant,WORK,operation).expect("invoked effect turn")?.output{RetainedWorkPoll::Complete(turn)=>Some(turn),RetainedWorkPoll::Pending{..}=>None}
}

/// 🧵️ Runs one poll-shaped turn that must complete.
pub(crate) fn run_poll<T,E>(wallet:&mut AuthoringWallet,grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{
    run_turn(wallet,grant,operation).expect("synchronous original turn stayed pending")
}

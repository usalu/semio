//! 🧪️ The mesh test binary's physical allocation observer and per-thread original effect bank.
use semio_framework_value_testing_admission as admission;
use semio_framework_value::{RetainedAllocationEffect,RetainedWorkOutcome,ValueError};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use std::cell::RefCell;

pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations;

thread_local!{static BANK:RefCell<Option<admission::EffectBank>>=const{RefCell::new(None)};}

fn new_bank()->admission::EffectBank{admission::EffectBank::new(admission::BankWallet::from_profile(&serde_json::json!({"items":1u64<<30,"copyBytes":1u64<<40,"capacityBytes":1u64<<40,"releaseBytes":1u64<<40,"depth":1u64<<20,"fuel":1u64<<60})))}

/// 🔥️ Builds this thread's original effect bank before a test observes allocations.
pub(crate) fn warm(){BANK.with(|slot|{let mut slot=slot.borrow_mut();if slot.is_none(){*slot=Some(new_bank())}})}

/// 🎟️ Lends one real original effect of `grant` to a single call.
pub(crate) fn fx<R>(grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
    BANK.with(|slot|{
        let mut bank=slot.borrow_mut().take().unwrap_or_else(new_bank);
        let mut result=None;
        let _=bank.run(grant,1<<40,|effect|{result=Some(run(effect));let work=effect.performed_work();admission::RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})});
        bank.take_admitted();
        *slot.borrow_mut()=Some(bank);
        result.expect("effect call ran")
    })
}

/// ♻️ Drives one reply-shaped action under a genuine producer authority, across pending turns.
pub(crate) fn drive<F:Send+'static>(grant:RetainedCloneGrant,call:impl for<'r,'e,'a> FnMut(&'e mut RetainedAllocationEffect<'a>,admission::OriginalEffectReply<'r,semio_framework_value::retained_clone::RetainedCloneStep,F>,bool)->Result<admission::OriginalEffectReplyComplete<'r,semio_framework_value::retained_clone::RetainedCloneStep,F>,ValueError>)->admission::RetainedWorkPoll<semio_framework_value::retained_clone::RetainedCloneStep,F>{
    BANK.with(|slot|{
        let mut bank=slot.borrow_mut().take().unwrap_or_else(new_bank);
        let(poll,_)=admission::driven(&mut bank,grant,1<<40,call);
        bank.take_admitted();
        *slot.borrow_mut()=Some(bank);
        poll
    })
}

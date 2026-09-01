//! 🪪️ Private non-reused numeric tags; neither work credit nor a tag is resident funding.

use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

//#region 🪪️Sequence
pub(super) struct InputRootSequence {
    pub(super) last: AtomicU64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum InputRootFault { Busy, Exhausted }

pub(super) static INPUT_ROOT_SEQUENCE: InputRootSequence = InputRootSequence::new();

impl InputRootSequence {
    pub(super) const fn new() -> Self { Self { last: AtomicU64::new(0) } }

    pub(super) fn try_next(&self) -> Result<NonZeroU64, InputRootFault> {
        let previous = self.last.load(Ordering::Acquire);
        let next = previous.checked_add(1).and_then(NonZeroU64::new).ok_or(InputRootFault::Exhausted)?;
        #[cfg(test)]
        super::input_root_tests::interfere_after_load(&self.last);
        self.last.compare_exchange(previous, next.get(), Ordering::AcqRel, Ordering::Acquire).map_err(|_| InputRootFault::Busy)?;
        Ok(next)
    }
}
//#endregion 🪪️Sequence


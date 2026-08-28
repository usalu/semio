//! 📮️ Fixed arena-slot handback obligations; producers never acquire the arena mutex.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UiArenaHandback { ReleaseAlias, ReturnClaim }

//#region 📫️FixedObligations
pub(crate) struct UiArenaHandbacks<const SLOTS: usize, const WORDS: usize> {
    releases: [AtomicU64; SLOTS],
    claims: [AtomicBool; SLOTS],
    ready: [AtomicU64; WORDS],
}

impl<const SLOTS: usize, const WORDS: usize> UiArenaHandbacks<SLOTS, WORDS> {
    pub(crate) const fn new() -> Self {
        assert!(WORDS * 64 >= SLOTS);
        Self { releases: [const { AtomicU64::new(0) }; SLOTS], claims: [const { AtomicBool::new(false) }; SLOTS], ready: [const { AtomicU64::new(0) }; WORDS] }
    }

    /// 📨️ Records an admitted owner; slot reuse remains forbidden until its obligation is consumed.
    pub(crate) fn record(&self, slot: usize, obligation: UiArenaHandback) {
        match obligation {
            UiArenaHandback::ReleaseAlias => { let previous = self.releases[slot].fetch_add(1, Ordering::AcqRel); assert!(previous != u64::MAX, "pending releases exceed admitted aliases"); }
            UiArenaHandback::ReturnClaim => { assert!(!self.claims[slot].swap(true, Ordering::AcqRel), "one exact claim returned twice"); }
        }
        self.ready[slot / 64].fetch_or(1u64 << (slot % 64), Ordering::Release);
    }

    /// 🧭️ Selects a ready slot fairly using only the fixed word envelope.
    pub(crate) fn next_slot(&self, start: usize) -> Option<usize> {
        let start = start % SLOTS;
        let first_word = start / 64;
        let first_bit = start % 64;
        let first = self.ready[first_word].load(Ordering::Acquire);
        let suffix = first & (u64::MAX << first_bit);
        if suffix != 0 { return Some(first_word * 64 + suffix.trailing_zeros() as usize); }
        for offset in 1..WORDS {
            let word = (first_word + offset) % WORDS;
            let ready = self.ready[word].load(Ordering::Acquire);
            if ready != 0 { return Some(word * 64 + ready.trailing_zeros() as usize); }
        }
        let prefix = first & !(u64::MAX << first_bit);
        (prefix != 0).then(|| first_word * 64 + prefix.trailing_zeros() as usize)
    }

    pub(crate) fn has_pending(&self) -> bool { self.ready.iter().any(|word| word.load(Ordering::Acquire) != 0) }

    pub(crate) fn has_slot_pending(&self, slot: usize) -> bool { self.claims[slot].load(Ordering::Acquire) || self.releases[slot].load(Ordering::Acquire) != 0 }

    /// 📥️ Requires the arena's sole consumer guard; rejected application must record the returned obligation again.
    pub(crate) fn take_one(&self, slot: usize) -> Option<UiArenaHandback> {
        self.ready[slot / 64].fetch_and(!(1u64 << (slot % 64)), Ordering::AcqRel);
        let obligation = if self.claims[slot].swap(false, Ordering::AcqRel) {
            Some(UiArenaHandback::ReturnClaim)
        } else if self.releases[slot].load(Ordering::Acquire) != 0 {
            self.releases[slot].fetch_sub(1, Ordering::AcqRel);
            Some(UiArenaHandback::ReleaseAlias)
        } else { None };
        if self.claims[slot].load(Ordering::Acquire) || self.releases[slot].load(Ordering::Acquire) != 0 {
            self.ready[slot / 64].fetch_or(1u64 << (slot % 64), Ordering::Release);
        }
        obligation
    }
}
//#endregion 📫️FixedObligations

#[cfg(test)]
#[path = "🧪️component.rs"]
mod tests;

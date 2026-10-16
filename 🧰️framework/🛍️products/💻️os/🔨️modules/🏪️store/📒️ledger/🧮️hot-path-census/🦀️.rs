//! 🧮️ Test-only census of whole-history work on the mutation path: full folds of the event log, edits hashed for prefix
//! digests, ids mirrored into the persisted cursor and revision records rebuilt — what the O(change) laws count.
//! Compiled for the crate's own unit tests and, behind the `hot-path-census` feature, for the kernel tests that count whole-history work across crates.

thread_local! {
    static CENSUS: std::cell::Cell<HotPathCensus> = const { std::cell::Cell::new(HotPathCensus { folds: 0, digested: 0, mirrored: 0, rebuilt: 0 }) };
}

/// 📊️ Counts since the thread's last [`take`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HotPathCensus {
    pub folds: usize,
    pub digested: usize,
    pub mirrored: usize,
    pub rebuilt: usize,
}

pub fn count(update: impl FnOnce(&mut HotPathCensus)) {
    CENSUS.with(|census| {
        let mut next = census.get();
        update(&mut next);
        census.set(next);
    });
}

/// 🧾️ The counts so far, resetting them.
pub fn take() -> HotPathCensus {
    CENSUS.with(|census| census.replace(HotPathCensus::default()))
}

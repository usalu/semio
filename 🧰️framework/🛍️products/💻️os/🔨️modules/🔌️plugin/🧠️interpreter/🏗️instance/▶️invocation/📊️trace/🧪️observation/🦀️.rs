//! 📊️ Per-category retained receipt observation for the retained-work tests.
use super::*;
use std::cell::Cell;

#[cfg(test)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Category {
    Scalar,
    FrameBirth,
    FrameReturn,
    Control,
    Memory,
    HostCall,
}

#[cfg(test)]
impl Category {
    const ALL: [Self; 6] = [
        Self::Scalar, Self::FrameBirth, Self::FrameReturn, Self::Control, Self::Memory, Self::HostCall,
    ];
}

#[cfg(test)]
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub(crate) struct Work {
    pub(crate) fuel: u64,
    pub(crate) items: u64,
    pub(crate) copy: u64,
    pub(crate) capacity: u64,
    pub(crate) release: u64,
}

#[cfg(test)]
impl Work {
    const ZERO: Self = Self { fuel: 0, items: 0, copy: 0, capacity: 0, release: 0 };

    fn observed(cx: &StepContext<'_>, fuel: u64) -> Self {
        let progress = cx.retained_progress();
        Self {
            fuel, items: progress.copied_items as u64, copy: progress.copied_bytes as u64,
            capacity: progress.retained_capacity_bytes as u64, release: progress.released_bytes as u64,
        }
    }

    fn since(self, before: Self) -> Self {
        Self {
            fuel: self.fuel - before.fuel, items: self.items - before.items, copy: self.copy - before.copy,
            capacity: self.capacity - before.capacity, release: self.release - before.release,
        }
    }

    pub(crate) fn add(&mut self, other: Self) {
        self.fuel = self.fuel.saturating_add(other.fuel);
        self.items = self.items.saturating_add(other.items);
        self.copy = self.copy.saturating_add(other.copy);
        self.capacity = self.capacity.saturating_add(other.capacity);
        self.release = self.release.saturating_add(other.release);
    }
}

#[cfg(test)]
type Breakdown = [Work; Category::ALL.len()];

#[cfg(test)]
pub(crate) struct Slice {
    category: Category,
    start: Work,
    before: Work,
    work: Breakdown,
}

#[cfg(test)]
impl Slice {
    pub(crate) fn begin(cx: &StepContext<'_>, category: Category) -> Option<Self> {
        if CAPTURE.with(|capture| capture.get().is_none()) {
            return None;
        }
        let start = Work::observed(cx, 0);
        Some(Self { category, start, before: start, work: [Work::ZERO; Category::ALL.len()] })
    }

    pub(crate) fn mark(&mut self, category: Category, cx: &StepContext<'_>, fuel: u64) {
        if self.category == category {
            return;
        }
        self.record(cx, fuel);
        self.category = category;
    }

    fn record(&mut self, cx: &StepContext<'_>, fuel: u64) {
        let current = Work::observed(cx, fuel);
        self.work[self.category as usize].add(current.since(self.before));
        self.before = current;
    }

    pub(crate) fn finish(mut self, cx: &StepContext<'_>, fuel: u64, pending: bool) {
        self.record(cx, fuel);
        let stalled = pending && self.before == self.start;
        CAPTURE.with(|capture| {
            if capture.get().is_some() {
                capture.set(Some((self.work, stalled)));
            }
        });
    }
}

#[cfg(test)]
thread_local! {
    static CAPTURE: Cell<Option<(Breakdown, bool)>> = const { Cell::new(None) };
}

#[cfg(test)]
pub(crate) fn capture<T>(run: impl FnOnce() -> T) -> (T, Breakdown, bool) {
    CAPTURE.with(|capture| {
        assert!(capture.replace(Some(([Work::ZERO; Category::ALL.len()], false))).is_none());
        let result = run();
        let (work, stalled) = capture.replace(None).unwrap();
        (result, work, stalled)
    })
}

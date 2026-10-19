//! 📶️ Indexed heap sorting retains only a fixed cursor and requests one observable action per turn.
use std::cmp::Ordering;

/// 🎛️ The source owner admits and executes this one action before advancing its cursor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeapSortWork {
    Advance,
    Compare { left: usize, right: usize },
    Swap { left: usize, right: usize },
    Complete,
}

/// 🚦️ Invalid observations preserve the exact pending action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeapSortFault { MissingComparison, UnexpectedComparison, AlreadyComplete, InvalidSnapshot }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase { Prepare, ChooseChild, CompareRoot, SwapRoot, Extract, Complete }

/// 🧭️ Borrowed index requests sort an original owner without allocating or copying its payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeapSortCursor { length: usize, end: usize, build: usize, root: usize, child: usize, building: bool, phase: Phase }

impl HeapSortCursor {
    pub fn new(length: usize) -> Self {
        Self { length, end: length, build: length / 2, root: 0, child: 0, building: true, phase: if length < 2 { Phase::Complete } else { Phase::Prepare } }
    }

    pub fn input_len(&self) -> usize { self.length }

    /// 📸️ The fixed portable state retains the exact next action.
    pub fn snapshot(&self) -> [u64; 8] { [self.length as u64, self.end as u64, self.build as u64, self.root as u64, self.child as u64, self.building as u64, self.phase as u64, 1] }

    /// 🪞️ Restore validates every index before exposing a source request.
    pub fn from_snapshot(state: [u64; 8]) -> Result<Self, HeapSortFault> {
        if state[7] != 1 || state[..5].iter().any(|value| *value > 9_007_199_254_740_991) || state[5] > 1 { return Err(HeapSortFault::InvalidSnapshot); }
        let scalar = |index| usize::try_from(state[index]).map_err(|_| HeapSortFault::InvalidSnapshot);
        let phase = match state[6] { 0 => Phase::Prepare, 1 => Phase::ChooseChild, 2 => Phase::CompareRoot, 3 => Phase::SwapRoot, 4 => Phase::Extract, 5 => Phase::Complete, _ => return Err(HeapSortFault::InvalidSnapshot) };
        let cursor = Self { length: scalar(0)?, end: scalar(1)?, build: scalar(2)?, root: scalar(3)?, child: scalar(4)?, building: state[5] != 0, phase };
        if cursor.end > cursor.length || cursor.build > cursor.length / 2 || cursor.root >= cursor.length.max(1) || cursor.child >= cursor.length.max(1) || (cursor.phase != Phase::Complete && (cursor.length < 2 || cursor.end == 0)) || (matches!(cursor.phase, Phase::CompareRoot | Phase::SwapRoot) && (cursor.root >= cursor.end || cursor.child >= cursor.end)) { return Err(HeapSortFault::InvalidSnapshot); }
        Ok(cursor)
    }

    fn left_child(&self) -> Option<usize> { self.root.checked_mul(2)?.checked_add(1).filter(|index| *index < self.end) }

    fn finish_sift(&mut self) { self.phase = if self.building { Phase::Prepare } else { Phase::Extract }; }

    pub fn next(&self) -> HeapSortWork {
        match self.phase {
            Phase::Prepare => HeapSortWork::Advance,
            Phase::ChooseChild => match self.left_child() {
                Some(left) if left + 1 < self.end => HeapSortWork::Compare { left, right: left + 1 },
                _ => HeapSortWork::Advance,
            },
            Phase::CompareRoot => HeapSortWork::Compare { left: self.root, right: self.child },
            Phase::SwapRoot => HeapSortWork::Swap { left: self.root, right: self.child },
            Phase::Extract if self.end > 1 => HeapSortWork::Swap { left: 0, right: self.end - 1 },
            Phase::Extract => HeapSortWork::Advance,
            Phase::Complete => HeapSortWork::Complete,
        }
    }

    pub fn advance(&mut self, comparison: Option<Ordering>) -> Result<(), HeapSortFault> {
        match (self.next(), comparison) {
            (HeapSortWork::Complete, _) => return Err(HeapSortFault::AlreadyComplete),
            (HeapSortWork::Compare { .. }, None) => return Err(HeapSortFault::MissingComparison),
            (HeapSortWork::Compare { .. }, Some(_)) => {},
            (_, Some(_)) => return Err(HeapSortFault::UnexpectedComparison),
            _ => {},
        }
        match self.phase {
            Phase::Prepare if self.build > 0 => {
                self.build -= 1;
                self.root = self.build;
                self.phase = Phase::ChooseChild;
            },
            Phase::Prepare => { self.building = false; self.phase = Phase::Extract; },
            Phase::ChooseChild => match self.left_child() {
                Some(left) => {
                    self.child = if comparison == Some(Ordering::Less) { left + 1 } else { left };
                    self.phase = Phase::CompareRoot;
                },
                None => self.finish_sift(),
            },
            Phase::CompareRoot if comparison == Some(Ordering::Less) => self.phase = Phase::SwapRoot,
            Phase::CompareRoot => self.finish_sift(),
            Phase::SwapRoot => { self.root = self.child; self.phase = Phase::ChooseChild; },
            Phase::Extract if self.end > 1 => { self.end -= 1; self.root = 0; self.phase = Phase::ChooseChild; },
            Phase::Extract => self.phase = Phase::Complete,
            Phase::Complete => unreachable!(),
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

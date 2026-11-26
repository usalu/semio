//! 🧮️ Borrowed numeric-order selection from an original variadic dictionary.
use semio_os_kernel_neural_engine_model::Dictionary;

#[derive(Clone, Copy)]
pub(super) struct NumericItem {
    index: usize,
    pub(super) canonical_rank: Option<usize>,
    pub(super) multiplicity: usize,
}

/// 🪜️ Visitation positions and an in-place numeric comparison, without a sorted signature allocation.
#[derive(Clone, Copy, Default)]
pub(super) struct VariadicSignature {
    scan: usize,
    offset: usize,
    number: usize,
    canonical: bool,
    floor: Option<usize>,
    candidate: Option<NumericItem>,
    pub(super) item_rank: Option<usize>,
    pub(super) repetitions: usize,
}

pub(super) enum Scan {
    Advanced,
    Selected(NumericItem),
    Exhausted,
}

impl VariadicSignature {
    pub(super) fn copy_demand(&self, items: Option<&Dictionary>) -> usize {
        usize::from(items.and_then(|items| items.entry_at_rank(self.scan)).is_some_and(|(key, _)| self.offset < key.len()))
    }

    fn next_key(&mut self) {
        self.scan += 1;
        self.offset = 0;
        self.number = 0;
        self.canonical = false;
    }

    fn record(&mut self) {
        let index = self.number;
        if self.floor.is_some_and(|floor| index <= floor) {
            return;
        }
        let canonical_rank = self.canonical.then_some(self.scan);
        match self.candidate.as_mut() {
            Some(item) if item.index < index => {}
            Some(item) if item.index == index => {
                item.multiplicity += 1;
                item.canonical_rank = item.canonical_rank.or(canonical_rank);
            }
            _ => self.candidate = Some(NumericItem { index, canonical_rank, multiplicity: 1 }),
        }
    }

    pub(super) fn step(&mut self, items: Option<&Dictionary>) -> Scan {
        let Some((key, _)) = items.and_then(|items| items.entry_at_rank(self.scan)) else {
            let Some(item) = self.candidate.take() else { return Scan::Exhausted };
            self.scan = 0;
            self.floor = Some(item.index);
            return Scan::Selected(item);
        };
        let Some(&byte) = key.as_bytes().get(self.offset) else {
            self.next_key();
            return Scan::Advanced;
        };
        if self.offset == 0 {
            self.canonical = byte.is_ascii_digit() && (byte != b'0' || key.len() == 1);
            if byte == b'+' && key.len() > 1 {
                self.offset += 1;
                return Scan::Advanced;
            }
        }
        let number = byte.is_ascii_digit().then(|| self.number.checked_mul(10)?.checked_add(usize::from(byte - b'0'))).flatten();
        let Some(number) = number else {
            self.next_key();
            return Scan::Advanced;
        };
        self.number = number;
        self.offset += 1;
        if self.offset == key.len() {
            self.record();
            self.next_key();
        }
        Scan::Advanced
    }
}

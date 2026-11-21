//! 🛂️ Received dictionaries keep their authored entry order while validating unique identities.

use super::{QuestionDictionary, QuestionDictionaryEntry};
use semio_framework_value::{
    NativeDecodeControl, RetainedAllocationEffect, RetainedCloneProgress, RetainedCloneStep,
    RetirementDemand, ValueError, ValueRefusalKind,
    sort::{HeapSortCursor, HeapSortWork},
};
use semio_framework_value_native_owned::native_decoding::{
    NativeReceive,
    owned::{NativeMapValidation, NativeReceiveTryMap, NativeTryMappedReceiver, tree::NativeValueSource},
};
use std::{cmp::Ordering, mem::size_of};

/// 📚️ The stored dictionary has one field containing the ordered entry sequence.
#[derive(semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[value(deny_unknown_fields)]
pub struct DictionaryWire {
    entries: Vec<QuestionDictionaryEntry>,
}

impl NativeReceiveTryMap<DictionaryWire> for QuestionDictionary {
    fn validate_native(value: &DictionaryWire) -> Result<(), ValueError> {
        let mut checkpoint = |_| true;
        let mut control = NativeDecodeControl::new(usize::MAX, &mut checkpoint);
        super::validate_entries(&value.entries, &mut control)
    }

    fn from_valid_native(value: DictionaryWire) -> Self {
        Self { entries: value.entries }
    }
}

impl<I: NativeValueSource + ?Sized> NativeReceive<I> for QuestionDictionary {
    type Receiver = NativeTryMappedReceiver<I, DictionaryWire, Self, UniqueIdentityValidation>;

    fn receiver() -> Self::Receiver {
        Self::Receiver::new()
    }
}

#[derive(Default)]
enum Phase {
    #[default]
    Reserve,
    Seed,
    Sort(HeapSortCursor),
    Validate,
    Release,
    Complete,
}

/// 🧮️ Sorts scratch indexes while leaving the received dictionary entries in their original order.
#[derive(Default)]
pub struct UniqueIdentityValidation {
    phase: Phase,
    indexes: Vec<usize>,
    adjacent: usize,
    compared: usize,
}

fn invalid(text: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, text)
}

fn compare(left: &str, right: &str, offset: &mut usize, bytes: usize) -> Option<Ordering> {
    let length = left.len().min(right.len());
    let end = (*offset + bytes).min(length);
    let order = left.as_bytes()[*offset..end].cmp(&right.as_bytes()[*offset..end]);
    if !order.is_eq() || end == length {
        *offset = 0;
        return Some(order.then_with(|| left.len().cmp(&right.len())));
    }
    *offset = end;
    None
}

impl UniqueIdentityValidation {
    fn pair<'a>(&self, value: &'a DictionaryWire, left: usize, right: usize) -> (&'a str, &'a str) {
        (&value.entries[self.indexes[left]].question_id, &value.entries[self.indexes[right]].question_id)
    }

    fn pair_demand(&self, value: &DictionaryWire, left: usize, right: usize, body: usize) -> usize {
        let (left, right) = self.pair(value, left, right);
        left.len().min(right.len()).saturating_sub(self.compared).min(body.max(1))
    }

    fn release(&mut self) -> RetainedCloneProgress {
        let released_bytes = self.indexes.capacity() * size_of::<usize>();
        self.indexes = Vec::new();
        self.phase = Phase::Complete;
        RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }
    }
}

impl NativeMapValidation<DictionaryWire, QuestionDictionary> for UniqueIdentityValidation {
    fn demands(&self, value: &DictionaryWire, body: usize) -> Result<RetirementDemand, ValueError> {
        let mut demand = RetirementDemand { depth: 1, ..Default::default() };
        match self.phase {
            Phase::Reserve if value.entries.len() > 1 => {
                demand.capacity_bytes = value.entries.len().checked_mul(size_of::<usize>())
                    .filter(|bytes| *bytes <= isize::MAX as usize)
                    .ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "dictionary identity index extent overflow"))?;
            }
            Phase::Seed => demand.copy_bytes = size_of::<usize>(),
            Phase::Sort(cursor) => {
                if let HeapSortWork::Compare { left, right } = cursor.next() {
                    demand.copy_bytes = self.pair_demand(value, left, right, body);
                }
            }
            Phase::Validate if self.adjacent + 1 < self.indexes.len() => {
                demand.copy_bytes = self.pair_demand(value, self.adjacent, self.adjacent + 1, body);
            }
            Phase::Release => return Ok(self.close_demand()),
            _ => {}
        }
        Ok(demand)
    }

    fn advance(&mut self, value: &DictionaryWire, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.demands(value, body)?;
        if !effect.performed(1)? {
            return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "dictionary identity validation exceeded its work authority"));
        }
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        match self.phase {
            Phase::Reserve if value.entries.len() <= 1 => self.phase = Phase::Complete,
            Phase::Reserve => {
                self.indexes.try_reserve_exact(value.entries.len())
                    .map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "dictionary identity index allocation failed"))?;
                progress.retained_capacity_bytes = self.indexes.capacity() * size_of::<usize>();
                *effect.actual() = Some(progress.retained_capacity_bytes);
                self.phase = Phase::Seed;
            }
            Phase::Seed => {
                self.indexes.push(self.indexes.len());
                if self.indexes.len() == value.entries.len() {
                    self.phase = Phase::Sort(HeapSortCursor::new(self.indexes.len()));
                }
            }
            Phase::Sort(mut cursor) => {
                let observed = match cursor.next() {
                    HeapSortWork::Advance => None,
                    HeapSortWork::Swap { left, right } => { self.indexes.swap(left, right); None }
                    HeapSortWork::Compare { left, right } => {
                        let (left, right) = self.pair(value, left, right);
                        let Some(order) = compare(left, right, &mut self.compared, demand.copy_bytes) else {
                            return Ok(RetainedCloneStep::Progress(progress));
                        };
                        Some(order)
                    }
                    HeapSortWork::Complete => {
                        self.phase = Phase::Validate;
                        return Ok(RetainedCloneStep::Progress(progress));
                    }
                };
                cursor.advance(observed).map_err(|_| invalid("dictionary identity sort changed its selected action"))?;
                self.phase = Phase::Sort(cursor);
            }
            Phase::Validate if self.adjacent + 1 < self.indexes.len() => {
                let (left, right) = self.pair(value, self.adjacent, self.adjacent + 1);
                match compare(left, right, &mut self.compared, demand.copy_bytes) {
                    Some(Ordering::Equal) => return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "duplicate QuestionDictionary question identity").with_retained_progress(progress)),
                    Some(_) => self.adjacent += 1,
                    None => {}
                }
            }
            Phase::Validate => self.phase = Phase::Release,
            Phase::Release => progress = self.release(),
            Phase::Complete => return Err(invalid("dictionary identity validation is already complete")),
        }
        Ok(if matches!(self.phase, Phase::Complete) {
            RetainedCloneStep::Complete(progress)
        } else {
            RetainedCloneStep::Progress(progress)
        })
    }

    fn close_demand(&self) -> RetirementDemand {
        RetirementDemand { release_bytes: self.indexes.capacity() * size_of::<usize>(), depth: 1, ..Default::default() }
    }

    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        if !effect.performed(1)? {
            return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "dictionary identity close exceeded its work authority"));
        }
        Ok(RetainedCloneStep::Complete(self.release()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.indexes.capacity() == 0
    }
}

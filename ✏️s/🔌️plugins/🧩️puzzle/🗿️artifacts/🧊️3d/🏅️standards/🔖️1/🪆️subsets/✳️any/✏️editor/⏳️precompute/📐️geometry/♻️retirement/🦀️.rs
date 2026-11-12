//! ♻️ Fixed planner containers retire each actual child, backing page and directory allocation separately.
use super::{FixedOwnerVec, FixedOwnerMap, FixedOwnerSet};
use semio_framework_value::{RetainedAllocationEffect, RetirementDemand, ValueError, ValueRefusalKind, retained_clone::retained_clone_grant_admits, retirement::{RetireOwned, RetirementCursor, RetirementStep}};
use std::mem::{ManuallyDrop, MaybeUninit, size_of};

fn pay(effect: &mut RetainedAllocationEffect<'_>) -> Result<(), RetirementStep> {
    match effect.performed(1) { Ok(true) => Ok(()), Ok(false) => Err(RetirementStep::Failure(ValueError::literal(ValueRefusalKind::WorkLimit, "fixed planner retirement exceeded its original work").into(), 1)), Err(error) => Err(RetirementStep::Failure(error.into(), 1)) }
}
fn child_birth(value: &impl RetireOwned) -> Result<usize, ValueError> {
    value.retirement_birth_bytes().ok_or_else(|| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "fixed planner child has no original retirement birth quote"))
}

#[doc(hidden)]
pub struct Vector<T: RetireOwned, const N: usize>(ManuallyDrop<FixedOwnerVec<T, N>>);
impl<T: RetireOwned, const N: usize> RetirementCursor for Vector<T, N> {
    semio_framework_value::native_retirement_reply!();
    fn next_computational_work(&self, _: usize) -> Result<u64, semio_framework_value::ValueError> { Ok(u64::from(!self.terminal_is_empty())) }
    fn next_demand(&self, _: usize) -> Result<RetirementDemand, ValueError> {
        Ok(if let Some(value) = self.0.len.checked_sub(1).and_then(|index| self.0.get(index)) {
            RetirementDemand { capacity_bytes: child_birth(value)?, depth: 1, ..Default::default() }
        } else if let Some(page) = self.0.pages.last() {
            RetirementDemand { release_bytes: page.len() * size_of::<MaybeUninit<T>>(), depth: 1, ..Default::default() }
        } else {
            RetirementDemand { release_bytes: self.0.pages.capacity() * size_of::<Box<[MaybeUninit<T>]>>(), depth: usize::from(self.0.pages.capacity() != 0), ..Default::default() }
        })
    }

    fn terminal_is_empty(&self) -> bool { self.0.len == 0 && self.0.pages.is_empty() && self.0.pages.capacity() == 0 }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}
impl<T: RetireOwned, const N: usize> Vector<T, N> {
    fn close_step(&mut self, _body: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetirementStep {
        let grant = effect.grant();
        let demand = match self.next_demand(grant.maximum_copy_bytes) { Ok(demand) => demand, Err(error) => return RetirementStep::Failure(error.into(), 0) };
        if !retained_clone_grant_admits(grant, demand) || effect.maximum_work_units() == 0 { return RetirementStep::BudgetExhausted; }
        if let Err(step) = pay(effect) { return step; }
        if let Some(value) = self.0.pop() { return RetirementStep::Child(Box::new(value.retirement_seed())); }
        if self.0.pages.pop().is_some() { return RetirementStep::Bytes(demand.release_bytes); }
        if self.0.pages.capacity() != 0 { drop(std::mem::take(&mut self.0.pages)); return RetirementStep::Bytes(demand.release_bytes); }
        RetirementStep::Complete(1)
    }
}

impl<T: RetireOwned, const N: usize> Drop for Vector<T, N> {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "fixed vector requires actual original retirement"); if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.0); } } }
}
impl<T: RetireOwned, const N: usize> RetireOwned for FixedOwnerVec<T, N> {
    type RetirementCursor = Vector<T, N>;
    fn retirement_seed(self) -> Self::RetirementCursor { Vector(ManuallyDrop::new(self)) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Vector<T, N>>()) }
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
}

#[doc(hidden)]
pub struct Map<K: RetireOwned, V: RetireOwned, const N: usize, const S: usize> {
    owner: ManuallyDrop<FixedOwnerMap<K, V, N, S>>,
    page: usize,
    slot: usize,
}
impl<K: RetireOwned, V: RetireOwned, const N: usize, const S: usize> RetirementCursor for Map<K, V, N, S> {
    semio_framework_value::native_retirement_reply!();
    fn next_computational_work(&self, _: usize) -> Result<u64, semio_framework_value::ValueError> { Ok(u64::from(!self.terminal_is_empty())) }
    fn next_demand(&self, _: usize) -> Result<RetirementDemand, ValueError> {
        Ok(if self.owner.len != 0 {
            match self.owner.pages.get(self.page).and_then(|page| page.get(self.slot)).and_then(Option::as_ref) {
                Some(value) => RetirementDemand { capacity_bytes: child_birth(value)?, depth: 1, ..Default::default() },
                None => RetirementDemand { copy_bytes: 2 * size_of::<usize>(), depth: 1, ..Default::default() },
            }
        } else if let Some(page) = self.owner.pages.last() {
            RetirementDemand { release_bytes: page.len() * size_of::<Option<(K, V)>>(), depth: 1, ..Default::default() }
        } else {
            RetirementDemand { release_bytes: self.owner.pages.capacity() * size_of::<Box<[Option<(K, V)>]>>(), depth: usize::from(self.owner.pages.capacity() != 0), ..Default::default() }
        })
    }

    fn terminal_is_empty(&self) -> bool { self.owner.len == 0 && self.owner.pages.is_empty() && self.owner.pages.capacity() == 0 }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}
impl<K: RetireOwned, V: RetireOwned, const N: usize, const S: usize> Map<K, V, N, S> {
    fn close_step(&mut self, _body: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetirementStep {
        let grant = effect.grant();
        let demand = match self.next_demand(grant.maximum_copy_bytes) { Ok(demand) => demand, Err(error) => return RetirementStep::Failure(error.into(), 0) };
        if !retained_clone_grant_admits(grant, demand) || effect.maximum_work_units() == 0 { return RetirementStep::BudgetExhausted; }
        if let Err(step) = pay(effect) { return step; }
        if self.owner.len != 0 {
            let Some(page) = self.owner.pages.get_mut(self.page) else { return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated, "fixed map lost an initialized original entry").into(), 1); };
            if self.slot == page.len() { self.page += 1; self.slot = 0; return RetirementStep::ProcessedBytes(demand.copy_bytes); }
            let value = page[self.slot].take();
            self.slot += 1;
            if let Some(value) = value { self.owner.len -= 1; return RetirementStep::Child(Box::new(value.retirement_seed())); }
            return RetirementStep::ProcessedBytes(demand.copy_bytes);
        }
        if self.owner.pages.pop().is_some() { return RetirementStep::Bytes(demand.release_bytes); }
        if self.owner.pages.capacity() != 0 { drop(std::mem::take(&mut self.owner.pages)); return RetirementStep::Bytes(demand.release_bytes); }
        RetirementStep::Complete(1)
    }
}

impl<K: RetireOwned, V: RetireOwned, const N: usize, const S: usize> Drop for Map<K, V, N, S> {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "fixed map requires actual original retirement"); if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.owner); } } }
}
impl<K: RetireOwned, V: RetireOwned, const N: usize, const S: usize> RetireOwned for FixedOwnerMap<K, V, N, S> {
    type RetirementCursor = Map<K, V, N, S>;
    fn retirement_seed(self) -> Self::RetirementCursor { Map { owner: ManuallyDrop::new(self), page: 0, slot: 0 } }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Map<K, V, N, S>>()) }
    fn controlled_retirement_supported() -> bool { K::controlled_retirement_supported() && V::controlled_retirement_supported() }
}
impl<K: RetireOwned, const N: usize, const S: usize> RetireOwned for FixedOwnerSet<K, N, S> {
    type RetirementCursor = <FixedOwnerMap<K, (), N, S> as RetireOwned>::RetirementCursor;
    fn retirement_seed(self) -> Self::RetirementCursor { self.values.retirement_seed() }
    fn retirement_birth_bytes(&self) -> Option<usize> { self.values.retirement_birth_bytes() }
    fn controlled_retirement_supported() -> bool { K::controlled_retirement_supported() }
}

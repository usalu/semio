//! 🎟️ Funded typed inference values retained across consecutive source runs.

use crate::{DepHash, InferenceCacheStats, InferenceWorkFailure};
use semio_framework_job::{close_original_effect_step, OriginalTurnFailure, StepContext};
use semio_framework_value::{retirement::{controlled::ControlledRetirement, RetireOwned}, RetainedCloneProgress, RetirementDemand, ValueError};
use semio_framework_value_persistent::list::{PagedList, PagedListReservationFailure, PagedListReservationOutcome};
use std::mem::{size_of, ManuallyDrop};

#[derive(semio_framework_value::RetireOwned)]
struct Entry<V: RetireOwned> { fingerprint: DepHash, value: V, bytes: usize }
type Rows<V> = PagedList<Option<Entry<V>>, { usize::MAX }>;

/// 🧺️ Keeps its physical pages and typed values within a fixed logical byte budget.
pub struct OriginalInferenceCache<V: RetireOwned> {
    rows: ManuallyDrop<Rows<V>>,
    cleared: ManuallyDrop<Option<ControlledRetirement<Rows<V>>>>,
    evicted: ManuallyDrop<Option<ControlledRetirement<Entry<V>>>>,
    oldest: usize,
    used: usize,
    budget: usize,
    stats: InferenceCacheStats,
}

impl<V: RetireOwned> OriginalInferenceCache<V> {
    pub fn new(budget: usize) -> Self {
        Self { rows: Default::default(), cleared: Default::default(), evicted: Default::default(), oldest: 0, used: 0, budget, stats: Default::default() }
    }

    pub fn enabled(&self) -> bool { self.budget != 0 && V::controlled_retirement_supported() }
    pub fn used_bytes(&self) -> usize { self.used }
    pub fn stats(&self) -> InferenceCacheStats { self.stats }
    pub fn terminal_is_empty(&self) -> bool { self.rows.allocated_bytes() == 0 && self.cleared.is_none() && self.evicted.is_none() }
    pub fn pending_retirement_demands(&self, body: usize) -> Result<Option<RetirementDemand>, ValueError> {
        if let Some(owner) = self.evicted.as_ref() { return close_demand(owner, body).map(Some); }
        if let Some(owner) = self.cleared.as_ref() { return close_demand(owner, body).map(Some); }
        Ok(None)
    }
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let copy_bytes = if self.rows.allocated_bytes() == 0 { 0 } else { size_of::<Rows<V>>() };
        Ok(self.pending_retirement_demands(body)?.unwrap_or(RetirementDemand { copy_bytes, depth: 1, ..Default::default() }))
    }
    pub fn finish_pending_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if !close(&mut self.evicted, cx)? { return Ok(false); }
        close(&mut self.cleared, cx)
    }

    pub(super) fn extent(&self) -> usize { self.rows.len() }
    pub(super) fn lookup_depth(&self, index: usize) -> Result<usize, ValueError> { self.rows.next_get_depth_demand(index).map_err(ValueError::from) }
    pub(super) fn fingerprint(&self, index: usize) -> Option<DepHash> { self.rows.get(index)?.as_ref().map(|entry| entry.fingerprint) }
    pub(super) fn value(&self, index: usize) -> Option<&V> { self.rows.get(index)?.as_ref().map(|entry| &entry.value) }
    pub(super) fn record_lookup(&mut self, hit: bool) {
        if hit { self.stats.hits = self.stats.hits.saturating_add(1); }
        else { self.stats.misses = self.stats.misses.saturating_add(1); }
    }

    pub(super) fn insert_step(&mut self, hash: DepHash, bytes: usize, donor: &mut Option<V>, vacancy: &mut Option<usize>, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if !close(&mut self.evicted, cx)? { return Ok(false); }
        let charge = bytes.saturating_add(size_of::<Option<Entry<V>>>());
        if charge > self.budget { return Err(ValueError::invariant("original inference cache received an oversized donor").into()); }
        if self.used > self.budget - charge {
            let index = self.oldest;
            let depth = self.rows.next_get_depth_demand(index).map_err(ValueError::from)?;
            if !metadata(cx, size_of::<Option<Entry<V>>>() + size_of::<usize>(), depth)? { return Ok(false); }
            self.oldest = (index + 1) % self.rows.len();
            if let Some(entry) = self.rows.get_mut(index).ok_or_else(|| ValueError::invariant("original inference cache eviction row disappeared"))?.take() {
                let bytes = entry.bytes;
                match ControlledRetirement::new(entry) {
                    Ok(owner) => *self.evicted = Some(owner),
                    Err((error, entry)) => { *self.rows.get_mut(index).unwrap() = Some(entry); self.oldest = index; return Err(error.into()); }
                }
                self.used -= bytes;
                self.stats.evictions = self.stats.evictions.saturating_add(1);
                if vacancy.is_none() { *vacancy = Some(index); }
            }
            return Ok(false);
        }
        if let Some(index) = *vacancy {
            let depth = self.rows.next_get_depth_demand(index).map_err(ValueError::from)?;
            if !metadata(cx, size_of::<Option<Entry<V>>>(), depth)? { return Ok(false); }
            let row = self.rows.get_mut(index).ok_or_else(|| ValueError::invariant("original inference cache vacant row disappeared"))?;
            if row.is_some() { return Err(ValueError::invariant("original inference cache vacancy was occupied").into()); }
            *row = Some(Entry { fingerprint: hash, value: donor.take().ok_or_else(|| ValueError::invariant("original inference cache donor disappeared"))?, bytes: charge });
        } else {
            if !self.rows.has_reserved_slot() {
                let limit = self.budget / size_of::<Option<Entry<V>>>();
                if self.rows.len() >= limit { return Err(ValueError::invariant("original inference cache lost vacant capacity").into()); }
                reserve(&mut self.rows, limit, cx)?;
                return Ok(false);
            }
            let depth = self.rows.next_push_depth_demand().map_err(ValueError::from)?;
            if !metadata(cx, size_of::<Option<Entry<V>>>(), depth)? { return Ok(false); }
            let value = donor.take().ok_or_else(|| ValueError::invariant("original inference cache donor disappeared"))?;
            if let Err(entry) = self.rows.push_reserved(Some(Entry { fingerprint: hash, value, bytes: charge })) {
                *donor = entry.map(|entry| entry.value);
                return Err(ValueError::invariant("original inference cache reserve disappeared").into());
            }
        }
        self.used += charge;
        Ok(true)
    }

    pub(super) fn admits(&self, bytes: usize) -> bool { bytes.saturating_add(size_of::<Option<Entry<V>>>()) <= self.budget }

    /// ♻️ Cleans the actual retained rows gradually and preserves statistics across a cold run.
    pub fn clear_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if self.terminal_is_empty() { return Ok(true); }
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if !close(&mut self.evicted, cx)? { return Ok(false); }
        if self.cleared.is_some() { return close(&mut self.cleared, cx); }
        if self.rows.allocated_bytes() == 0 { return Ok(true); }
        if !metadata(cx, size_of::<Rows<V>>(), 1)? { return Ok(false); }
        match ControlledRetirement::new(std::mem::take(&mut *self.rows)) {
            Ok(owner) => *self.cleared = Some(owner),
            Err((error, rows)) => { *self.rows = rows; return Err(error.into()); }
        }
        self.used = 0;
        self.oldest = 0;
        Ok(false)
    }
}

impl<V: RetireOwned> Drop for OriginalInferenceCache<V> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "original inference cache requires paid retirement");
    }
}

fn metadata(cx: &mut StepContext<'_>, bytes: usize, depth: usize) -> Result<bool, InferenceWorkFailure> {
    let grant = cx.retained_grant();
    if cx.retirement_should_yield() || grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth < depth { return Ok(false); }
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
    cx.consume_fuel(1);
    Ok(true)
}

fn close<T: RetireOwned>(owner: &mut Option<ControlledRetirement<T>>, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
    let Some(active) = owner.as_mut() else { return Ok(true); };
    if !active.terminal_is_empty() {
        let body = cx.retained_grant().maximum_copy_bytes;
        let demand = active.selected_demand(body)?;
        close_original_effect_step(cx, demand, |effect| active.step(body, effect))?;
        return Ok(false);
    }
    if !metadata(cx, size_of::<Option<ControlledRetirement<T>>>(), 1)? { return Ok(false); }
    owner.take();
    Ok(false)
}

fn close_demand<T: RetireOwned>(owner: &ControlledRetirement<T>, body: usize) -> Result<RetirementDemand, ValueError> {
    if !owner.terminal_is_empty() { return owner.selected_demand(body); }
    Ok(RetirementDemand { copy_bytes: size_of::<Option<ControlledRetirement<T>>>(), depth: 1, ..Default::default() })
}

fn reserve<V: RetireOwned>(rows: &mut Rows<V>, limit: usize, cx: &mut StepContext<'_>) -> Result<(), InferenceWorkFailure> {
    let Some((turn, _)) = cx.with_original_source_effect(cx.retained_grant(), |_, effect| rows.reserve_capacity_one_funded(limit, effect))? else { return Ok(()); };
    cx.consume_work(turn.work);
    match turn.result {
        Ok(PagedListReservationOutcome::Ready(progress)) => { cx.consume_retained(progress)?; }
        Ok(PagedListReservationOutcome::Denied) => {}
        Err(PagedListReservationFailure::Work(error)) => return Err(OriginalTurnFailure::debit_original(error, cx).into()),
        Err(PagedListReservationFailure::Allocation { error, copied_bytes }) => {
            let receipt = RetainedCloneProgress { copied_bytes, retained_capacity_bytes: error.allocated_bytes, ..Default::default() };
            return Err(OriginalTurnFailure::debit_original(ValueError::from(error.refusal()).with_retained_progress(receipt), cx).into());
        }
    }
    Ok(())
}

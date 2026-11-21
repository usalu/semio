//! 🎟️ Applies original trace increments and compacts paged footage under the caller’s grant.

use super::*;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_framework_value_persistent::list::{PagedListError, PagedListReleaseFailure, PagedListReleaseOutcome, PagedListReservationFailure, PagedListReservationOutcome};
use std::mem::{size_of, take};

const SNAPSHOT_ROWS: usize = 64;

/// 📚️ Borrowed source position. The store owns every retained backing independently of this cursor.
#[derive(Clone, Copy, Debug, Default)]
pub struct ToolRunTraceApplyCursor {
    source: Option<(usize, usize)>,
    page: usize,
    op: usize,
}

semio_framework_value::artifact_retire_leaf!(ToolRunTraceApplyCursor);

impl ToolRunTraceApplyCursor {
    pub fn new() -> Self { Self::default() }

    fn check(&self, pages: &[ToolRunTracePage], current: ToolRunIdentity) -> Result<(), ValueError> {
        if self.source.is_some_and(|key| key != self.binding(pages)) {
            return Err(ValueError::invariant("original tool trace page source changed"));
        }
        if let Some(page) = pages.get(self.page) {
            if page.identity.id != current.id || page.identity.generation != current.generation {
                return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "tool run trace page is stale"));
            }
        }
        Ok(())
    }

    fn binding(&self, pages: &[ToolRunTracePage]) -> (usize, usize) {
        (pages.as_ptr() as usize, pages.len())
    }

    fn next(&self, pages: &[ToolRunTracePage]) -> Option<ToolRunTraceOp> {
        pages.get(self.page)?.ops.get(self.op).copied()
    }
}

/// 📋️ One original operation or one bounded maintenance step.
#[derive(Default)]
pub struct ToolRunTraceFundedStep {
    pub progress: RetainedCloneProgress,
    pub applied: Option<ToolRunTraceOp>,
    pub evicted: u32,
    pub overflowed: u32,
    pub complete: bool,
}

#[derive(Default, semio_framework_value::RetireOwned)]
pub(super) struct Compaction {
    data: TraceBacking,
    rank: usize,
    group: Vec<ToolRunTraceOp>,
}

type Log = PagedList<ToolRunTraceLogPage, {usize::MAX}>;

#[derive(Clone, Copy)]
enum Slot { Record, Stamp, Rejected, Log }

#[derive(Clone, Copy)]
enum Close { Log, Records, Stamps, Rejected }

fn page_error(error: PagedListError) -> ValueError { ValueError::from(error) }

fn allocation_error(error: PagedListReservationFailure) -> ValueError {
    match error {
        PagedListReservationFailure::Work(error) => error,
        PagedListReservationFailure::Allocation { error, copied_bytes } => {
            let progress = receipt(copied_bytes, error.allocated_bytes, 0);
            ValueError::from(error.refusal()).with_retained_progress(progress)
        }
    }
}

fn release_error(error: PagedListReleaseFailure) -> ValueError {
    match error {
        PagedListReleaseFailure::Work(error) => error,
        PagedListReleaseFailure::Release(error) => page_error(error),
    }
}

fn receipt(copy: usize, capacity: usize, released: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: copy, retained_capacity_bytes: capacity, released_bytes: released }
}

fn fixed(copy: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes: copy, depth: 1, ..Default::default() }
}

fn perform(effect: &mut RetainedAllocationEffect<'_>, d: RetirementDemand) -> Result<bool, ValueError> {
    if !retained_clone_grant_admits(effect.grant(), d) || effect.remaining_work_units()? == 0 {
        return Ok(false);
    }
    effect.performed(1)
}

fn log_reserve(log: &Log) -> Result<Option<RetirementDemand>, ValueError> {
    if log.has_reserved_slot() {
        return Ok(None);
    }
    Ok(Some(RetirementDemand {
        copy_bytes: log.next_reserve_copy_byte_demand().map_err(page_error)?,
        capacity_bytes: log.next_allocation_bytes().map_err(page_error)?,
        depth: log.next_reserve_depth_demand().map_err(page_error)?,
        ..Default::default()
    }))
}

fn index_reserve<K: Copy + Ord, V>(index: &NumericIndex<K, V>, key: &K) -> Result<Option<RetirementDemand>, ValueError> {
    if index.find(key).is_some() {
        return Ok(None);
    }
    let demand = index.next_reserve_demand().map_err(page_error)?;
    Ok((demand.capacity_bytes != 0).then_some(demand))
}

fn next_slot(data: &TraceBacking, row: Option<(u64, ToolRunTraceRecord)>, log: bool) -> Result<Option<(Slot, RetirementDemand)>, ValueError> {
    if let Some((key, record)) = row {
        if let Some(demand) = index_reserve(&data.records, &key)? { return Ok(Some((Slot::Record, demand))); }
        if let Some(demand) = index_reserve(&data.stamps, &record.stamp)? { return Ok(Some((Slot::Stamp, demand))); }
        if record.verdict.is_rejected() {
            if let Some(demand) = index_reserve(&data.rejected, &record.stamp)? { return Ok(Some((Slot::Rejected, demand))); }
        }
    }
    Ok(if log { log_reserve(&data.log)?.map(|need| (Slot::Log, need)) } else { None })
}

fn reserve(data: &mut TraceBacking, slot: Slot, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
    let report = match slot {
        Slot::Record => data.records.reserve_one_funded(effect),
        Slot::Stamp => data.stamps.reserve_one_funded(effect),
        Slot::Rejected => data.rejected.reserve_one_funded(effect),
        Slot::Log => data.log.reserve_one_funded(effect),
    };
    match report.result.map_err(allocation_error)? {
        PagedListReservationOutcome::Denied => Ok(Default::default()),
        PagedListReservationOutcome::Ready(progress) => Ok(progress),
    }
}

fn insert_log(log: &mut Log, ops: TraceLogOps) {
    assert!(log.push_reserved(ToolRunTraceLogPage::new(ops)).is_ok(), "actual trace log slot reserved");
}

fn close_quote(data: &TraceBacking) -> Result<Option<(Close, RetirementDemand)>, ValueError> {
    if let Some(last) = data.log.last() {
        let allocation = match &last.ops {
            TraceLogOps::Group(ops) => ops.capacity() * size_of::<ToolRunTraceOp>(),
            TraceLogOps::Inline(..) => 0,
        };
        return Ok(Some((Close::Log, RetirementDemand { release_bytes: allocation, ..fixed(size_of::<ToolRunTraceLogPage>()) })));
    }
    if !data.log.terminal_is_empty() {
        return Ok(Some((Close::Log, data.log.next_release_demand().map_err(page_error)?)));
    }
    if !data.records.terminal_is_empty() {
        return Ok(Some((Close::Records, data.records.next_close_demand().map_err(page_error)?)));
    }
    if !data.stamps.terminal_is_empty() {
        return Ok(Some((Close::Stamps, data.stamps.next_close_demand().map_err(page_error)?)));
    }
    if !data.rejected.terminal_is_empty() {
        return Ok(Some((Close::Rejected, data.rejected.next_close_demand().map_err(page_error)?)));
    }
    Ok(None)
}

fn close(data: &mut TraceBacking, stage: Close, demand: RetirementDemand, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
    let report = match stage {
        Close::Log if !data.log.is_empty() => {
            if !perform(effect, demand)? { return Ok(Default::default()); }
            drop(data.log.pop());
            return Ok(receipt(demand.copy_bytes, 0, demand.release_bytes));
        }
        Close::Log => data.log.release_empty_page(effect),
        Close::Records => data.records.close_copy_step(effect),
        Close::Stamps => data.stamps.close_copy_step(effect),
        Close::Rejected => data.rejected.close_copy_step(effect),
    };
    match report.result.map_err(release_error)? {
        PagedListReleaseOutcome::Denied => Ok(Default::default()),
        PagedListReleaseOutcome::Ready(progress) if progress.progressed => Ok(receipt(demand.copy_bytes, 0, progress.released_allocation_bytes)),
        PagedListReleaseOutcome::Ready(_) => Ok(Default::default()),
    }
}

impl ToolRunTraceStore {
    fn compaction_birth_demand() -> RetirementDemand {
        RetirementDemand { capacity_bytes: size_of::<Compaction>(), ..fixed(size_of::<Compaction>()) }
    }

    fn compaction_release_demand(&self) -> RetirementDemand {
        RetirementDemand { release_bytes: self.compaction.capacity() * size_of::<Compaction>(), ..fixed(size_of::<Vec<Compaction>>()) }
    }

    fn needs_original_compaction(&self) -> bool {
        let limit = self.len().max(self.compact_floor).saturating_mul(2);
        self.logged_ops > limit
            || self.data.records.stored_keys() > limit
            || self.data.stamps.stored_keys() > limit
            || self.data.rejected.stored_keys() > limit
    }

    fn change(&self, op: ToolRunTraceOp) -> (Option<(u64, ToolRunTraceRecord)>, bool) {
        match op {
            ToolRunTraceOp::Clear => (None, true),
            ToolRunTraceOp::Retire { key } => (None, self.record(key).is_some()),
            ToolRunTraceOp::Upsert { key, verdict, reason, subject } => {
                if self.len() < self.capacity || self.record(key).is_some() || self.data.oldest_rejected().is_some() {
                    (Some((key, ToolRunTraceRecord { verdict, reason, subject, stamp: self.next_stamp })), true)
                } else {
                    (None, false)
                }
            }
        }
    }

    fn original_depth(&self) -> usize {
        2 + usize::from([
            self.data.records.height(self.data.records.root),
            self.data.stamps.height(self.data.stamps.root),
            self.data.rejected.height(self.data.rejected.root),
        ].into_iter().max().unwrap_or(0))
    }

    fn operation_demand(&self) -> RetirementDemand {
        RetirementDemand {
            copy_bytes: size_of::<ToolRunTraceApplyCursor>() + size_of::<ToolRunTraceOp>() + size_of::<ToolRunTraceLogPage>() + size_of::<ToolRunTraceRecord>(),
            depth: self.original_depth(),
            ..Default::default()
        }
    }

    fn snapshot_record(&self, rank: usize) -> Option<(u64, ToolRunTraceRecord)> {
        let (_, key) = self.data.stamps.entry_at_rank(rank.checked_sub(1)?)?;
        Some((*key, *self.record(*key).expect("original snapshot stamp names its record")))
    }

    fn next_compaction_demand(&self, c: &Compaction) -> Result<RetirementDemand, ValueError> {
        let total = self.len() + 1;
        if c.group.len() == SNAPSHOT_ROWS || c.rank == total && !c.group.is_empty() {
            return Ok(log_reserve(&c.data.log)?.unwrap_or_else(|| fixed(size_of::<ToolRunTraceLogPage>())));
        }
        if c.rank == total {
            return Ok(fixed(size_of::<TraceBacking>() + size_of::<Compaction>()));
        }
        if c.group.capacity() == 0 {
            return Ok(RetirementDemand { capacity_bytes: SNAPSHOT_ROWS.min(total - c.rank) * size_of::<ToolRunTraceOp>(), ..fixed(0) });
        }
        if let Some((_, d)) = next_slot(&c.data, self.snapshot_record(c.rank), false)? {
            return Ok(d);
        }
        Ok(self.operation_demand())
    }

    fn continue_compaction(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        let progress = if let Some(c) = self.compaction.first() {
            self.next_compaction_demand(c)?
        } else {
            let d = Self::compaction_birth_demand();
            if !perform(effect, d)? { return Ok(Default::default()); }
            self.compaction = reserve_vec(1, effect)?;
            self.compaction.push(Compaction::default());
            return Ok(receipt(d.copy_bytes, self.compaction.capacity() * size_of::<Compaction>(), 0));
        };
        let total = self.len() + 1;
        let rank = self.compaction.first().expect("original compaction").rank;
        let row = self.snapshot_record(rank);
        let op = row.map(|(key, _)| self.snapshot_op(key)).unwrap_or(ToolRunTraceOp::Clear);
        let c = self.compaction.first_mut().expect("original compaction");
        if c.group.len() == SNAPSHOT_ROWS || rank == total && !c.group.is_empty() {
            if log_reserve(&c.data.log)?.is_some() {
                return reserve(&mut c.data, Slot::Log, effect);
            }
            if !perform(effect, progress)? { return Ok(Default::default()); }
            insert_log(&mut c.data.log, TraceLogOps::Group(take(&mut c.group)));
            return Ok(receipt(progress.copy_bytes, 0, 0));
        }
        if rank == total {
            let original = u32::try_from(c.data.log.len()).map_err(|_| ValueError::invariant("trace compacted page count overflow"))?;
            let next = self.next_page.checked_add(original).ok_or_else(|| ValueError::invariant("trace compacted page ordinal overflow"))?;
            if !perform(effect, progress)? { return Ok(Default::default()); }
            std::mem::swap(&mut self.data, &mut c.data);
            self.retiring = true;
            self.log_base = self.next_page;
            self.next_page = next;
            self.logged_ops = total;
            return Ok(receipt(progress.copy_bytes, 0, 0));
        }
        if c.group.capacity() == 0 {
            if !perform(effect, progress)? { return Ok(Default::default()); }
            c.group = reserve_vec(SNAPSHOT_ROWS.min(total - rank), effect)?;
            return Ok(receipt(0, c.group.capacity() * size_of::<ToolRunTraceOp>(), 0));
        }
        if let Some((slot, _)) = next_slot(&c.data, row, false)? {
            return reserve(&mut c.data, slot, effect);
        }
        if !perform(effect, progress)? { return Ok(Default::default()); }
        if let Some((key, record)) = row {
            c.data.replace(key, record, true);
        }
        c.group.push(op);
        c.rank += 1;
        Ok(receipt(progress.copy_bytes, 0, 0))
    }

    /// 📏️ Quotes exactly one structural reservation, source advance, or retired trace page.
    pub fn next_funded_apply_demand(&self, cursor: &ToolRunTraceApplyCursor, pages: &[ToolRunTracePage]) -> Result<RetirementDemand, ValueError> {
        cursor.check(pages, self.identity)?;
        if let Some(c) = self.compaction.first() {
            return if self.retiring {
                Ok(close_quote(&c.data)?.map_or_else(|| self.compaction_release_demand(), |(_, demand)| demand))
            } else {
                self.next_compaction_demand(c)
            };
        }
        if self.needs_original_compaction() {
            return Ok(Self::compaction_birth_demand());
        }
        let Some(op) = cursor.next(pages) else {
            return Ok(fixed(size_of::<ToolRunTraceApplyCursor>()));
        };
        let (row, log) = self.change(op);
        Ok(next_slot(&self.data, row, log)?.map_or_else(|| self.operation_demand(), |(_, demand)| demand))
    }

    /// 🧵️ Advances the borrowed original pages with single-application receipts under a fresh grant.
    pub fn step_funded_apply(
        &mut self,
        cursor: &mut ToolRunTraceApplyCursor,
        pages: &[ToolRunTracePage],
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<ToolRunTraceFundedStep, ValueError> {
        let demand = self.next_funded_apply_demand(cursor, pages)?;
        if !retained_clone_grant_admits(effect.grant(), demand) {
            return Ok(Default::default());
        }
        let progress = if self.retiring {
            let old = &mut self.compaction.first_mut().expect("original trace retirement").data;
            if let Some((stage, demand)) = close_quote(old)? {
                close(old, stage, demand, effect)?
            } else if perform(effect, demand)? {
                self.compaction = Vec::new();
                self.retiring = false;
                receipt(demand.copy_bytes, 0, demand.release_bytes)
            } else { Default::default() }
        } else if !self.compaction.is_empty() || self.needs_original_compaction() {
            self.continue_compaction(effect)?
        } else if let Some(op) = cursor.next(pages) {
            let (row, log) = self.change(op);
            if let Some((slot, _)) = next_slot(&self.data, row, log)? {
                reserve(&mut self.data, slot, effect)?
            } else {
                if log && self.next_page == u32::MAX || row.is_some() && self.next_stamp == u64::MAX {
                    return Err(ValueError::invariant("original trace ordinal exhausted"));
                }
                if !perform(effect, demand)? {
                    return Ok(Default::default());
                }
                let change = self.apply_one(op, true);
                if change.count != 0 {
                    insert_log(&mut self.data.log, TraceLogOps::Inline(change.ops, change.count));
                    self.next_page += 1;
                    self.logged_ops += change.count;
                }
                cursor.source = Some(cursor.binding(pages));
                cursor.op += 1;
                return Ok(ToolRunTraceFundedStep {
                    progress: receipt(demand.copy_bytes, 0, 0), applied: Some(op),
                    evicted: change.evicted, overflowed: change.overflowed, complete: false,
                });
            }
        } else if cursor.page < pages.len() {
            if !perform(effect, demand)? { return Ok(Default::default()); }
            cursor.source = Some(cursor.binding(pages));
            cursor.page += 1;
            cursor.op = 0;
            receipt(demand.copy_bytes, 0, 0)
        } else {
            return Ok(ToolRunTraceFundedStep { complete: true, ..Default::default() });
        };
        Ok(ToolRunTraceFundedStep { progress, ..Default::default() })
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

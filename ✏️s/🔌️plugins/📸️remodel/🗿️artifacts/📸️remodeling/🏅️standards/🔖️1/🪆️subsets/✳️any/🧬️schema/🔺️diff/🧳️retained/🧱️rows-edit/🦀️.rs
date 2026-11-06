//! 🧱️ Owned keyed row edits compare bytes and move one row header per funded turn before returning a local candidate.
use super::{RemodelingNoPatch, RemodelingRow};
use protocol::RetainedKeyedRow;
use semio_framework_value::retained_clone::{retained_clone_grant_admits, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{OriginalRetirementFailure, OriginalRetirementOutcome, RetainedWorkOutcome, RetireOwned, RetirementDemand, ValueError, ValueRefusalKind};
use std::{
    cmp::Ordering,
    mem::{size_of, ManuallyDrop},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RowRefusal {
    Duplicate,
    Missing,
}
impl RowRefusal {
    pub(crate) fn code(self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate-id",
            Self::Missing => "missing-target",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RowEditStatus {
    Pending,
    Ready,
    Refused(RowRefusal),
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Search,
    Partition,
    Apply,
    InsertShift,
    RemoveShift,
    RemovePop,
    Settle,
    Ready,
    Refused,
    Taken,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Insert,
    Replace,
    Remove,
    Patch,
}
#[derive(Clone, Copy)]
struct State {
    phase: Phase,
    kind: Kind,
    cursor: usize,
    offset: usize,
    base: usize,
    size: usize,
    target: usize,
    ordering: Option<Ordering>,
    refusal: Option<RowRefusal>,
}

pub(crate) struct OwnedRowEdit<E: RetireOwned + RetainedKeyedRow> {
    input: ManuallyDrop<Option<Vec<E>>>,
    row: ManuallyDrop<Option<RemodelingRow<E, RemodelingNoPatch>>>,
    removed: ManuallyDrop<Option<E>>,
    retirement: RetainedCloneClose,
    state: State,
    closing: bool,
}
fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, "owned remodeling row edit changed its retained frontier")
}
fn scalar(copy_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
}

impl<E: RetireOwned + RetainedKeyedRow> OwnedRowEdit<E> {
    pub(crate) fn new(input: Vec<E>, row: RemodelingRow<E, RemodelingNoPatch>) -> Self {
        let kind = match &row {
            RemodelingRow::Insert { .. } => Kind::Insert,
            RemodelingRow::Replace { .. } => Kind::Replace,
            RemodelingRow::Remove { .. } => Kind::Remove,
            RemodelingRow::Patch { .. } => Kind::Patch,
        };
        Self {
            input: ManuallyDrop::new(Some(input)),
            row: ManuallyDrop::new(Some(row)),
            removed: ManuallyDrop::new(None),
            retirement: Default::default(),
            state: State { phase: Phase::Search, kind, cursor: 0, offset: 0, base: 0, size: 0, target: 0, ordering: None, refusal: None },
            closing: false,
        }
    }
    fn query(&self) -> Result<&String, ValueError> {
        match self.row.as_ref().ok_or_else(invalid)? {
            RemodelingRow::Insert { entity } | RemodelingRow::Replace { entity } => Ok(entity.retained_key()),
            RemodelingRow::Remove { key } | RemodelingRow::Patch { key, .. } => Ok(key),
        }
    }
    fn settling(&self) -> bool {
        self.state.phase == Phase::Settle && (!self.retirement.is_empty() || self.row.is_some() || self.removed.is_some())
    }
    fn compare_turn(&self) -> bool {
        matches!(self.state.phase, Phase::Search | Phase::Partition) && self.state.ordering.is_none() && self.input.as_ref().is_some_and(|input| self.state.cursor < input.len())
    }
    fn metadata() -> usize {
        2 * size_of::<State>()
    }
    fn local_work(&self) -> u64 {
        if self.compare_turn() {
            5
        } else {
            match self.state.phase {
                Phase::Apply if matches!(self.state.kind, Kind::Insert | Kind::Replace) => 4,
                Phase::InsertShift if self.state.cursor > self.state.target => 3,
                Phase::RemoveShift if self.input.as_ref().is_some_and(|input| self.state.cursor + 1 < input.len()) => 3,
                Phase::RemovePop => 3,
                _ => 2,
            }
        }
    }
    fn local_copy(&self) -> Result<usize, ValueError> {
        let extra = if self.compare_turn() {
            2 * size_of::<&String>() + 2 * size_of::<&[u8]>() + 2
        } else {
            match self.state.phase {
                Phase::Apply if matches!(self.state.kind, Kind::Insert | Kind::Replace) => 2 * size_of::<Option<RemodelingRow<E, RemodelingNoPatch>>>() + 4 * size_of::<E>() + size_of::<Option<E>>() + 2 * size_of::<usize>(),
                Phase::InsertShift if self.state.cursor > self.state.target => 3 * size_of::<E>(),
                Phase::RemoveShift if self.input.as_ref().is_some_and(|input| self.state.cursor + 1 < input.len()) => 3 * size_of::<E>(),
                Phase::RemovePop => 2 * size_of::<Option<E>>() + size_of::<usize>(),
                _ => 0,
            }
        };
        Self::metadata().checked_add(extra).ok_or_else(invalid)
    }
    fn settlement_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.retirement.is_empty() {
            self.retirement.demands(body)
        } else if self.row.is_some() {
            self.retirement.next_owner_demands::<RemodelingRow<E, RemodelingNoPatch>>(true, &None, body)
        } else {
            self.retirement.next_owner_demands::<E>(self.removed.is_some(), &None, body)
        }
    }
    fn settlement_work(&self, body: usize) -> Result<u64, ValueError> {
        if self.row.is_some() {
            self.retirement.next_owner_work::<RemodelingRow<E, RemodelingNoPatch>>(true, &None, body)
        } else {
            self.retirement.next_owner_work::<E>(self.removed.is_some(), &None, body)
        }
    }
    pub(crate) fn demand(&self, maximum_work_units: u64) -> Result<RetirementDemand, ValueError> {
        if self.closing || matches!(self.state.phase, Phase::Ready | Phase::Refused | Phase::Taken) {
            return Err(invalid());
        }
        if maximum_work_units == 0 {
            return Ok(Default::default());
        }
        if self.settling() {
            return self.settlement_demand(4096);
        }
        if self.local_work() > maximum_work_units {
            return Ok(Default::default());
        }
        if self.state.phase == Phase::Apply && self.state.kind == Kind::Insert && self.input.as_ref().is_some_and(|input| input.len() == input.capacity()) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "owned row insertion requires its prepared capacity"));
        }
        Ok(scalar(self.local_copy()?))
    }
    pub(crate) fn work_demand(&self, maximum_work_units: u64) -> Result<u64, ValueError> {
        self.demand(maximum_work_units)?;
        let work = if self.settling() { self.settlement_work(4096)? } else { self.local_work() };
        Ok(if work <= maximum_work_units { work } else { 0 })
    }
    fn settle(&mut self, grant: RetainedCloneGrant) -> OriginalRetirementOutcome {
        if !self.retirement.is_empty() {
            return self.retirement.step_granted(grant);
        }
        if self.row.is_some() {
            let turn = self.retirement.begin_granted(&mut self.row, grant);
            return RetainedWorkOutcome { result: turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into), work: turn.work };
        }
        let turn = self.retirement.begin_granted(&mut self.removed, grant);
        RetainedWorkOutcome { result: turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into), work: turn.work }
    }
    pub(crate) fn advance(&mut self, maximum_work_units: u64, grant: RetainedCloneGrant) -> OriginalRetirementOutcome {
        let mut work = 0;
        let result: Result<_, OriginalRetirementFailure> = (|| {
            let selected = self.work_demand(maximum_work_units)?;
            if selected == 0 {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            let demand = self.demand(maximum_work_units)?;
            if !retained_clone_grant_admits(grant, demand) {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            if self.settling() {
                let turn = self.settle(grant);
                work = turn.work;
                return turn.result.map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            let mut state = self.state;
            work = selected;
            let mut copied_bytes = demand.copy_bytes;
            if self.compare_turn() {
                let current = self.input.as_ref().ok_or_else(invalid)?[state.cursor].retained_key().as_bytes();
                let query = self.query()?.as_bytes();
                if state.offset == current.len().min(query.len()) {
                    state.ordering = Some(current.len().cmp(&query.len()));
                    copied_bytes -= 2;
                } else {
                    let ordering = current[state.offset].cmp(&query[state.offset]);
                    if ordering == Ordering::Equal {
                        state.offset += 1;
                    } else {
                        state.ordering = Some(ordering);
                    }
                }
            } else {
                match state.phase {
                    Phase::Search => {
                        let length = self.input.as_ref().ok_or_else(invalid)?.len();
                        if state.cursor == length {
                            if state.kind == Kind::Insert {
                                state.phase = Phase::Partition;
                                state.base = 0;
                                state.size = length;
                                state.cursor = if length > 1 { length / 2 } else { 0 };
                                state.offset = 0;
                                state.ordering = None;
                            } else {
                                state.phase = Phase::Refused;
                                state.refusal = Some(RowRefusal::Missing);
                            }
                        } else if state.ordering.take().ok_or_else(invalid)? == Ordering::Equal {
                            if state.kind == Kind::Insert {
                                state.phase = Phase::Refused;
                                state.refusal = Some(RowRefusal::Duplicate);
                            } else {
                                state.target = state.cursor;
                                state.phase = Phase::Apply;
                            }
                        } else {
                            state.cursor += 1;
                            state.offset = 0;
                        }
                    }
                    Phase::Partition => {
                        if state.size == 0 {
                            state.target = 0;
                            state.phase = Phase::Apply;
                        } else {
                            let less = state.ordering.take().ok_or_else(invalid)? == Ordering::Less;
                            if state.size > 1 {
                                let half = state.size / 2;
                                if less {
                                    state.base = state.cursor;
                                }
                                state.size -= half;
                                state.cursor = if state.size > 1 { state.base + state.size / 2 } else { state.base };
                                state.offset = 0;
                            } else {
                                state.target = state.base + usize::from(less);
                                state.phase = Phase::Apply;
                            }
                        }
                    }
                    Phase::Apply => match state.kind {
                        Kind::Insert => {
                            let input = self.input.as_mut().ok_or_else(invalid)?;
                            let row = self.row.take().ok_or_else(invalid)?;
                            match row {
                                RemodelingRow::Insert { entity } => {
                                    input.push(entity);
                                    state.cursor = input.len() - 1;
                                    state.phase = Phase::InsertShift;
                                }
                                other => {
                                    *self.row = Some(other);
                                    return Err(invalid().into());
                                }
                            }
                        }
                        Kind::Replace => {
                            if self.removed.is_some() {
                                return Err(invalid().into());
                            }
                            let target = self.input.as_mut().and_then(|input| input.get_mut(state.target)).ok_or_else(invalid)?;
                            let row = self.row.take().ok_or_else(invalid)?;
                            match row {
                                RemodelingRow::Replace { entity } => {
                                    *self.removed = Some(std::mem::replace(target, entity));
                                    state.phase = Phase::Settle;
                                }
                                other => {
                                    *self.row = Some(other);
                                    return Err(invalid().into());
                                }
                            }
                        }
                        Kind::Remove => {
                            state.cursor = state.target;
                            state.phase = Phase::RemoveShift;
                        }
                        Kind::Patch => {
                            state.phase = Phase::Settle;
                        }
                    },
                    Phase::InsertShift => {
                        if state.cursor > state.target {
                            self.input.as_mut().ok_or_else(invalid)?.swap(state.cursor, state.cursor - 1);
                            state.cursor -= 1;
                        } else {
                            state.phase = Phase::Settle;
                        }
                    }
                    Phase::RemoveShift => {
                        let input = self.input.as_mut().ok_or_else(invalid)?;
                        if state.cursor + 1 < input.len() {
                            input.swap(state.cursor, state.cursor + 1);
                            state.cursor += 1;
                        } else {
                            state.phase = Phase::RemovePop;
                        }
                    }
                    Phase::RemovePop => {
                        *self.removed = self.input.as_mut().ok_or_else(invalid)?.pop();
                        state.phase = Phase::Settle;
                    }
                    Phase::Settle => {
                        state.phase = Phase::Ready;
                    }
                    _ => return Err(invalid().into()),
                }
            }
            self.state = state;
            Ok(if self.status() == RowEditStatus::Pending {
                RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() })
            } else {
                RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() })
            })
        })();
        RetainedWorkOutcome { result, work }
    }
    pub(crate) fn status(&self) -> RowEditStatus {
        match self.state.phase {
            Phase::Ready => RowEditStatus::Ready,
            Phase::Refused => RowEditStatus::Refused(self.state.refusal.expect("quoted row refusal")),
            _ => RowEditStatus::Pending,
        }
    }
    pub(crate) fn take_demand(&self) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.state.phase != Phase::Ready {
            return Err(invalid());
        }
        Ok(scalar(2 * size_of::<Option<Vec<E>>>() + Self::metadata()))
    }
    pub(crate) fn take_work_demand(&self) -> Result<u64, ValueError> {
        self.take_demand()?;
        Ok(3)
    }
    pub(crate) fn take(&mut self, grant: RetainedCloneGrant) -> RetainedWorkOutcome<Option<(Vec<E>, RetainedCloneProgress)>, ValueError> {
        let demand = match self.take_demand() {
            Ok(demand) => demand,
            Err(error) => return RetainedWorkOutcome { result: Err(error), work: 0 },
        };
        if !retained_clone_grant_admits(grant, demand) {
            return RetainedWorkOutcome { result: Ok(None), work: 0 };
        }
        let mut state = self.state;
        state.phase = Phase::Taken;
        let value = self.input.take().expect("quoted owned row result");
        self.state = state;
        RetainedWorkOutcome { result: Ok(Some((value, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))), work: 3 }
    }
    pub(crate) fn begin_close(&mut self) {
        self.closing = true;
    }
    pub(crate) fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.closing {
            return Err(invalid());
        }
        if !self.retirement.is_empty() {
            return self.retirement.demands(body);
        }
        if self.input.is_some() {
            return self.retirement.next_owner_demands::<Vec<E>>(true, &None, body);
        }
        self.settlement_demand(body)
    }
    pub(crate) fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        self.close_demand(body)?;
        if self.input.is_some() && self.retirement.is_empty() {
            self.retirement.next_owner_work::<Vec<E>>(true, &None, body)
        } else {
            self.settlement_work(body)
        }
    }
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> OriginalRetirementOutcome {
        let demand = match self.close_demand(grant.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => return RetainedWorkOutcome { result: Err(error.into()), work: 0 },
        };
        if !retained_clone_grant_admits(grant, demand) {
            return RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work: 0 };
        }
        let turn = if !self.retirement.is_empty() {
            self.retirement.step_granted(grant)
        } else if self.input.is_some() {
            let turn = self.retirement.begin_granted(&mut self.input, grant);
            RetainedWorkOutcome { result: turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into), work: turn.work }
        } else {
            self.settle(grant)
        };
        RetainedWorkOutcome { result: turn.result.map(|step| if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) }), work: turn.work }
    }
    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.input.is_none() && self.row.is_none() && self.removed.is_none() && self.retirement.is_empty()
    }
}
impl<E: RetireOwned + RetainedKeyedRow> Drop for OwnedRowEdit<E> {
    fn drop(&mut self) {
        let empty = self.terminal_is_empty();
        assert!(std::thread::panicking() || empty, "owned remodeling row edit requires funded closure");
        if empty {
            unsafe {
                ManuallyDrop::drop(&mut self.input);
                ManuallyDrop::drop(&mut self.row);
                ManuallyDrop::drop(&mut self.removed);
            }
        }
    }
}
#[cfg(test)]
#[path = "🧪️.rs"]
mod tests;

//! 🗂️ One original-funded ID and ordinal index supplies exact source uniqueness lookups.
use semio_framework_value::retained_clone::{RetainedCloneProgress, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;
use std::mem::size_of;
#[cfg(test)]
#[path = "🧪️.rs"]
mod tests;
pub(super) trait IfcSourceIds {
    fn source_len(&self) -> usize;
    fn source_id(&self, row: usize) -> Result<u64, ValueError>;
}
impl IfcSourceIds for IfcSnapshot {
    fn source_len(&self) -> usize {
        self.entities.len()
    }
    fn source_id(&self, row: usize) -> Result<u64, ValueError> {
        Ok(self.entities[row].id)
    }
}
#[derive(Clone, Copy)]
struct Row {
    id: u64,
    ordinal: usize,
}
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    binding: Option<(usize, u64)>,
    phase: u8,
    root: usize,
    end: usize,
    sift: usize,
    done: bool,
}
semio_framework_value::artifact_retire_leaf!(Row, State);
#[derive(Default, semio_framework_value::RetireOwned)]
pub(super) struct IfcSourceIndex {
    rows: Vec<Row>,
    state: State,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum IdMatch {
    Missing,
    Unique(usize),
    Ambiguous,
}
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
impl IfcSourceIndex {
    pub(super) fn demands<S: IfcSourceIds>(&self, snapshot: &S) -> Result<RetirementDemand, ValueError> {
        self.demands_len(snapshot.source_len())
    }
    pub(super) fn demands_len(&self, length: usize) -> Result<RetirementDemand, ValueError> {
        let grow = self.state.binding.is_some() && self.state.phase == 0 && self.rows.len() < length && self.rows.len() == self.rows.capacity();
        let capacity = if grow { self.rows.capacity().checked_mul(2).unwrap_or(usize::MAX).max(8).min(length) } else { 0 };
        let bytes = capacity.checked_mul(size_of::<Row>()).ok_or_else(|| invalid("IFC source index capacity overflow"))?;
        Ok(RetirementDemand {
            copy_bytes: 4 * size_of::<Row>() + 2 * size_of::<State>() + if grow { self.rows.len() * size_of::<Row>() } else { 0 },
            capacity_bytes: bytes,
            release_bytes: if grow { self.rows.capacity() * size_of::<Row>() } else { 0 },
            depth: 1,
            ..Default::default()
        })
    }
    pub(super) fn advance<S: IfcSourceIds>(&mut self, snapshot: &S, revision: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC original source index cancelled").into());
        }
        let binding = (snapshot as *const S as usize, revision);
        if self.state.binding.is_some_and(|original| original != binding) {
            return Err(invalid("IFC original source index source changed").into());
        }
        let demand = self.demands(snapshot)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
            return Ok(false);
        }
        if self.state.done {
            return Ok(true);
        }
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        if self.state.binding.is_none() {
            self.state.binding = Some(binding);
        } else if demand.capacity_bytes > 0 {
            let old = self.rows.capacity();
            self.rows.try_reserve_exact(demand.capacity_bytes / size_of::<Row>() - self.rows.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC source index allocation failed"))?;
            progress.retained_capacity_bytes = self.rows.capacity() * size_of::<Row>();
            progress.released_bytes = old * size_of::<Row>();
        } else {
            match self.state.phase {
                0 => {
                    if self.rows.len() < snapshot.source_len() {
                        self.rows.push(Row { id: snapshot.source_id(self.rows.len())?, ordinal: self.rows.len() });
                    } else {
                        self.state.root = self.rows.len() / 2;
                        self.state.end = self.rows.len();
                        self.state.phase = 1;
                    }
                }
                1 => {
                    if self.state.root > 0 {
                        self.state.root -= 1;
                        self.state.sift = self.state.root;
                        self.state.phase = 2;
                    } else {
                        self.state.phase = 3;
                    }
                }
                2 | 4 => {
                    let child = self.state.sift.checked_mul(2).and_then(|value| value.checked_add(1)).unwrap_or(usize::MAX);
                    if child >= self.state.end {
                        self.state.phase = if self.state.phase == 2 { 1 } else { 3 };
                    } else {
                        let child = if child + 1 < self.state.end && (self.rows[child].id, self.rows[child].ordinal) < (self.rows[child + 1].id, self.rows[child + 1].ordinal) { child + 1 } else { child };
                        let root = self.state.sift;
                        if (self.rows[root].id, self.rows[root].ordinal) < (self.rows[child].id, self.rows[child].ordinal) {
                            self.rows.swap(root, child);
                            self.state.sift = child;
                        } else {
                            self.state.phase = if self.state.phase == 2 { 1 } else { 3 };
                        }
                    }
                }
                _ => {
                    if self.state.end <= 1 {
                        self.state.done = true;
                    } else {
                        self.state.end -= 1;
                        self.rows.swap(0, self.state.end);
                        self.state.sift = 0;
                        self.state.phase = 4;
                    }
                }
            }
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(self.state.done)
    }
    pub(super) fn identity<S: IfcSourceIds>(&self, snapshot: &S, revision: u64) -> Result<(usize, usize), ValueError> {
        if !self.state.done || self.state.binding != Some((snapshot as *const S as usize, revision)) {
            return Err(invalid("IFC source index has no completed original source"));
        }
        Ok((self.rows.as_ptr() as usize, self.rows.len()))
    }
    pub(super) fn find(&self, id: u64) -> IdMatch {
        let Ok(index) = self.rows.binary_search_by_key(&id, |row| row.id) else {
            return IdMatch::Missing;
        };
        if index > 0 && self.rows[index - 1].id == id || index + 1 < self.rows.len() && self.rows[index + 1].id == id { IdMatch::Ambiguous } else { IdMatch::Unique(self.rows[index].ordinal) }
    }
}

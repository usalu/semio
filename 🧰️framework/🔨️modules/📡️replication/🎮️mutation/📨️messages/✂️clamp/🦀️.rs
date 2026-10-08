//! ✂️ One final severity selection consumes original native message pages under physical grants.
use super::{MutationMessage, MutationMessageRetirement, ReplayMessageAccumulator, MUTATION_MESSAGE_ENTRY_BYTES, MUTATION_MESSAGE_OWNER_BYTES, MUTATION_MESSAGE_TARGET_BYTES};
use semio_framework_diagnostic::Severity;
use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneProgress}, ValueError, ValueRefusalKind};
use std::mem::{ManuallyDrop, size_of};
const ROWS: usize = MUTATION_MESSAGE_ENTRY_BYTES / MUTATION_MESSAGE_OWNER_BYTES;
const CASCADE: &str = "mutation.cascade";
#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Reserve, Measure, Rank, Drain, PrefixReserve, PrefixCopy, PrefixPlace, Summary, CodeReserve, CodeCopy, BodyReserve, BodyCopy, SummaryPlace, Reverse, Pages, Finished }

pub struct FinalMessageSelection {
    phase: Phase,
    output: ManuallyDrop<Vec<MutationMessage>>,
    pending: Option<MutationMessage>,
    active: Option<MutationMessageRetirement>,
    index: usize,
    target: usize,
    measuring: Option<usize>,
    total: usize,
    original: usize,
    budget: usize,
    used: usize,
    level: usize,
    kept: usize,
    threshold: Option<(Severity, usize)>,
    worst: Option<(Severity, usize)>,
    first_dropped: Option<Option<u32>>,
    cut: usize,
    summary: [u8; 34],
    summary_len: usize,
    changed: bool,
    closing: bool,
    work: u64,
}
fn digits(mut value: usize) -> usize { let mut count = 1; while value >= 10 { value /= 10; count += 1; } count }
fn failure() -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, "final message selection lost its admitted physical owner") }
impl FinalMessageSelection {
    pub fn new(edit_id_bytes: usize, original_rows: usize) -> Self {
        let summary = MUTATION_MESSAGE_OWNER_BYTES + CASCADE.len() + digits(original_rows) + " more messages".len();
        Self { phase: Phase::Reserve, output: ManuallyDrop::new(Vec::new()), pending: None, active: None, index: 0, target: 0, measuring: None, total: edit_id_bytes, original: original_rows, budget: MUTATION_MESSAGE_ENTRY_BYTES.saturating_sub(edit_id_bytes).saturating_sub(summary), used: 0, level: 0, kept: 0, threshold: None, worst: None, first_dropped: None, cut: 0, summary: [0; 34], summary_len: 0, changed: false, closing: false, work: 0 }
    }
    pub fn changed(&self) -> bool { self.changed }
    pub fn completed_work(&self) -> u64 { self.work }
    pub fn is_finished(&self) -> bool { self.phase == Phase::Finished && !self.closing }
    pub fn messages(&self) -> &[MutationMessage] { &self.output }
    pub fn next_capacity_byte_demand(&self) -> usize { match self.phase { Phase::Reserve => ROWS * size_of::<MutationMessage>(), Phase::PrefixReserve => self.cut, Phase::CodeReserve => CASCADE.len(), Phase::BodyReserve => self.summary_len, _ => 0 } }
    pub fn next_copy_byte_demand(&self, source: &ReplayMessageAccumulator) -> usize {
        if self.active.is_some() && !matches!(self.phase, Phase::PrefixReserve | Phase::PrefixCopy) { return self.active.as_ref().unwrap().next_copy_byte_demand(); }
        match self.phase { Phase::Drain if !source.rows().is_empty() => size_of::<MutationMessage>(), Phase::PrefixCopy => self.active.as_ref().and_then(|active| active.body()).and_then(|body| body.get(self.pending.as_ref().map_or(0, |row| row.message.len())..self.cut)).and_then(|body| body.chars().next()).map_or(0, char::len_utf8), Phase::PrefixPlace => size_of::<MutationMessage>(), Phase::CodeCopy if self.pending.as_ref().is_some_and(|row| row.code.0.len() < CASCADE.len()) => 1, Phase::BodyCopy if self.pending.as_ref().is_some_and(|row| row.message.len() < self.summary_len) => 1, Phase::Summary => self.summary.len(), Phase::Reverse if self.index < self.cut/2 => 2 * size_of::<MutationMessage>(), Phase::Reverse if self.pending.is_some() => size_of::<MutationMessage>(), _ => 0 }
    }
    pub fn next_release_byte_demand(&self, source: &ReplayMessageAccumulator) -> Result<usize, ValueError> { if let Some(active) = self.active.as_ref().filter(|_| !matches!(self.phase, Phase::PrefixReserve | Phase::PrefixCopy)) { return Ok(active.next_release_byte_demand()); } if self.phase == Phase::Pages { source.rows().next_release_allocation_bytes().map_err(ValueError::from) } else { Ok(0) } }
    pub fn next_depth_demand(&self) -> usize { usize::from(!self.is_finished()) }
    pub fn step(&mut self, source: &mut ReplayMessageAccumulator, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.closing || self.is_finished() || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(idle); }
        let capacity = self.next_capacity_byte_demand(); let copy = self.next_copy_byte_demand(source); let release = self.next_release_byte_demand(source)?;
        if grant.maximum_capacity_bytes < capacity || grant.maximum_copy_bytes < copy || grant.maximum_release_bytes < release { return Ok(idle); }
        if let Some(active) = self.active.as_mut().filter(|_| !matches!(self.phase, Phase::PrefixReserve | Phase::PrefixCopy)) { let progress = active.close_step(grant); if active.terminal_is_empty() { self.active = None; } self.work = self.work.saturating_add(progress.copied_items as u64); return Ok(progress); }
        let mut progress = RetainedCloneProgress { copied_items: 1, ..idle };
        match self.phase {
            Phase::Reserve => { self.output.try_reserve_exact(ROWS).map_err(|_| failure())?; progress.retained_capacity_bytes = self.output.capacity() * size_of::<MutationMessage>(); self.phase = Phase::Measure; }
            Phase::Measure | Phase::Rank => {
                if self.phase == Phase::Rank && self.level == 4 { self.phase = Phase::Drain; }
                else if let Some(row) = source.rows().get(self.index) {
                    let matching = self.phase == Phase::Measure || row.level == [Severity::Fatal, Severity::Error, Severity::Warning, Severity::Info][self.level];
                    if !matching { self.index += 1; }
                    else if self.measuring.is_none() { self.measuring = Some(MUTATION_MESSAGE_OWNER_BYTES.saturating_add(row.code.0.len()).saturating_add(row.message.len())); self.target = 0; if self.phase == Phase::Measure && self.worst.is_none_or(|(level, _)| row.level > level) { self.worst = Some((row.level, self.index)); } }
                    else if let Some(target) = row.target.get(self.target) { self.measuring = Some(self.measuring.unwrap().saturating_add(MUTATION_MESSAGE_TARGET_BYTES).saturating_add(target.len())); self.target += 1; }
                    else { let bytes = self.measuring.take().unwrap(); if self.phase == Phase::Measure { self.total = self.total.saturating_add(bytes); self.index += 1; } else if self.used.saturating_add(bytes) > self.budget { self.phase = Phase::Drain; } else { self.used += bytes; self.kept += 1; self.threshold = Some((row.level, self.index)); self.index += 1; } }
                } else if self.phase == Phase::Measure { self.changed = self.total > MUTATION_MESSAGE_ENTRY_BYTES; self.phase = if self.changed { Phase::Rank } else { Phase::Drain }; self.index = 0; }
                else { self.level += 1; self.index = 0; }
            }
            Phase::Drain => {
                if let Some(mut row) = source.rows_mut().pop() {
                    progress.copied_bytes = size_of::<MutationMessage>(); let index = source.rows().len();
                    let keep = !self.changed || self.threshold.is_some_and(|(level, last)| row.level > level || row.level == level && index <= last);
                    if keep { self.push(row)?; }
                    else if self.kept == 0 && self.worst.is_some_and(|(_, worst)| worst == index) {
                        self.cut = self.budget.saturating_sub(MUTATION_MESSAGE_OWNER_BYTES).saturating_sub(row.code.0.len()).min(row.message.len()); while !row.message.is_char_boundary(self.cut) { self.cut -= 1; }
                        self.pending = Some(MutationMessage { level: row.level, code: std::mem::take(&mut row.code.0).into(), message: String::new(), target: Vec::new(), op_index: row.op_index });
                        self.active = Some(MutationMessageRetirement::new(row)); self.phase = Phase::PrefixReserve;
                    } else { self.first_dropped = Some(row.op_index); self.active = Some(MutationMessageRetirement::new(row)); }
                } else { self.cut = self.output.len(); self.phase = if self.first_dropped.is_some() { Phase::Summary } else { Phase::Reverse }; self.index = 0; }
            }
            Phase::PrefixReserve => { let row = self.pending.as_mut().ok_or_else(failure)?; row.message.try_reserve_exact(self.cut).map_err(|_| failure())?; progress.retained_capacity_bytes = row.message.capacity(); self.phase = Phase::PrefixCopy; }
            Phase::PrefixCopy => { let row = self.pending.as_mut().ok_or_else(failure)?; if let Some(scalar) = self.active.as_ref().and_then(MutationMessageRetirement::body).and_then(|body| body.get(row.message.len()..self.cut)).and_then(|body| body.chars().next()) { row.message.push(scalar); progress.copied_bytes = scalar.len_utf8(); } else { self.phase = Phase::PrefixPlace; } }
            Phase::PrefixPlace => { let row = self.pending.take().ok_or_else(failure)?; self.push(row)?; progress.copied_bytes = size_of::<MutationMessage>(); self.phase = Phase::Drain; }
            Phase::Summary => {
                let dropped = self.original - self.output.len(); let length = digits(dropped); let mut value = dropped; for index in (0..length).rev() { self.summary[index] = b'0' + (value % 10) as u8; value /= 10; } self.summary[length..length + 14].copy_from_slice(b" more messages"); self.summary_len = length + 14;
                self.pending = Some(MutationMessage { level: Severity::Info, code: String::new().into(), message: String::new(), target: Vec::new(), op_index: self.first_dropped.take().flatten() }); progress.copied_bytes = self.summary.len(); self.phase = Phase::CodeReserve;
            }
            Phase::CodeReserve => { let row = self.pending.as_mut().ok_or_else(failure)?; row.code.0.try_reserve_exact(CASCADE.len()).map_err(|_| failure())?; progress.retained_capacity_bytes = row.code.0.capacity(); self.phase = Phase::CodeCopy; }
            Phase::CodeCopy => { let row = self.pending.as_mut().ok_or_else(failure)?; if row.code.0.len() < CASCADE.len() { row.code.0.push(CASCADE.as_bytes()[row.code.0.len()] as char); progress.copied_bytes = 1; } else { self.phase = Phase::BodyReserve; } }
            Phase::BodyReserve => { let row = self.pending.as_mut().ok_or_else(failure)?; row.message.try_reserve_exact(self.summary_len).map_err(|_| failure())?; progress.retained_capacity_bytes = row.message.capacity(); self.phase = Phase::BodyCopy; }
            Phase::BodyCopy => { let row = self.pending.as_mut().ok_or_else(failure)?; if row.message.len() < self.summary_len { row.message.push(self.summary[row.message.len()] as char); progress.copied_bytes = 1; } else { self.phase = Phase::SummaryPlace; } }
            Phase::SummaryPlace => { self.cut = self.output.len(); self.phase = Phase::Reverse; self.index = 0; }
            Phase::Reverse => { if self.index < self.cut/2 { let last = self.cut - self.index - 1; self.output.swap(self.index, last); self.index += 1; progress.copied_bytes = 2 * size_of::<MutationMessage>(); } else if let Some(row) = self.pending.take() { self.push(row)?; progress.copied_bytes = size_of::<MutationMessage>(); } else { self.phase = Phase::Pages; } }
            Phase::Pages => { let page = source.rows_mut().release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?; progress.copied_items = usize::from(page.progressed); progress.released_bytes = page.released_allocation_bytes; if source.terminal_is_empty() { self.phase = Phase::Finished; } }
            Phase::Finished => {}
        }
        self.work = self.work.saturating_add(progress.copied_items as u64); Ok(progress)
    }
    fn push(&mut self, row: MutationMessage) -> Result<(), ValueError> { if self.output.len() == self.output.capacity() { self.pending = Some(row); return Err(failure()); } self.output.push(row); Ok(()) }
    pub fn take(&mut self) -> Option<Vec<MutationMessage>> { self.is_finished().then(|| std::mem::take(&mut *self.output)) }
    pub fn begin_close(&mut self) { self.closing = true; }
    pub fn next_close_copy_byte_demand(&self) -> usize { self.active.as_ref().map_or_else(|| usize::from(self.pending.is_some() || !self.output.is_empty()) * size_of::<MutationMessage>(), MutationMessageRetirement::next_copy_byte_demand) }
    pub fn next_close_release_byte_demand(&self) -> usize { self.active.as_ref().map_or_else(|| if self.pending.is_none() && self.output.is_empty() { self.output.capacity() * size_of::<MutationMessage>() } else { 0 }, MutationMessageRetirement::next_release_byte_demand) }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> RetainedCloneProgress {
        let idle = RetainedCloneProgress::default(); if !self.closing || grant.maximum_items == 0 || grant.maximum_depth == 0 { return idle; }
        if let Some(active) = self.active.as_mut() { let progress = active.close_step(grant); if active.terminal_is_empty() { self.active = None; } return progress; }
        let copy = self.next_close_copy_byte_demand(); if grant.maximum_copy_bytes < copy { return idle; }
        if let Some(row) = self.pending.take().or_else(|| self.output.pop()) { self.active = Some(MutationMessageRetirement::new(row)); return RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..idle }; }
        let release = self.next_close_release_byte_demand(); if grant.maximum_release_bytes < release { return idle; } drop(std::mem::take(&mut *self.output)); RetainedCloneProgress { copied_items: usize::from(release > 0), released_bytes: release, ..idle }
    }
    pub fn terminal_is_empty(&self) -> bool { self.output.is_empty() && self.output.capacity() == 0 && self.pending.is_none() && self.active.is_none() }
}
impl Drop for FinalMessageSelection { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "final selection must transfer or close every retained row and scaffold"); if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.output); } } } }
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

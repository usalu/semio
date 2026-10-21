//! 📨️ Prepares original history diagnostics and independent command-report ownership before publication.
use super::*;
use semio_framework_os_spr::command::{MutationMessageLedgerRetirement, MutationMessagePublication, MutationMessagePublicationBatch, MutationMessagePublicationJoin};
use semio_framework_value::{RetirementDemand, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use semio_framework_os_store_ledger::{ARTIFACT_EDIT_MESSAGE_OWNER_BYTES, ARTIFACT_EDIT_MESSAGE_TARGET_BYTES};
use std::mem::{ManuallyDrop, size_of};

pub struct ArtifactStoreDiagnosticRecord { entry:semio_framework_os_spr::EditMessages,bytes:usize }
impl ArtifactStoreDiagnosticRecord {
    pub fn entry(&self)->&semio_framework_os_spr::EditMessages { &self.entry }
    pub fn bytes(&self)->usize { self.bytes }
    pub fn into_entry(self)->semio_framework_os_spr::EditMessages { self.entry }
}

pub struct ArtifactStoreDiagnosticStage {
    pub batch: Option<MutationMessagePublicationBatch>,
    pub rows: usize,
    join: Option<MutationMessagePublicationJoin>,
    ready: Option<MutationMessagePublication>,
    original: Option<MutationMessageLedgerRetirement>,
    report: Option<MutationMessageLedgerRetirement>,
    records_retirement: Option<semio_framework_os_spr::command::EditMessageLedgerRetirement>,
    prefix: Option<semio_framework_os_spr::command::MutationMessageBorrowedPublication>,
    prefix_ready: Option<MutationMessagePublication>,
    prefix_done: bool,
    pub displaced: Option<Box<MutationMessageLedgerRetirement>>,
    bytes: usize,
    measure_target: Option<usize>,
    records: ManuallyDrop<Vec<semio_framework_os_spr::EditMessages>>,
    identity: [String; 2],
    severity_index: usize,
    worst: Option<semio_framework_diagnostic::Severity>,
    phase: u8,
    closing: bool,
}

impl Default for ArtifactStoreDiagnosticStage { fn default() -> Self { Self { batch: None, rows: 0, join: None, ready: None, original: None, report: None, records_retirement: None, prefix: None, prefix_ready: None, prefix_done: false, displaced: None, bytes: 0, measure_target: None, records: ManuallyDrop::new(Vec::new()), identity: Default::default(), severity_index: 0, worst: None, phase: 0, closing: false } } }

impl ArtifactStoreDiagnosticStage {
    pub fn from_publication(owner: Option<MutationMessagePublication>) -> Self { let mut stage = Self::default(); if let Some(owner) = owner { stage.rows = owner.original_messages().len(); stage.ready = Some(owner); stage.phase = 1; stage.prefix_done = true; } stage }
    pub fn prepared(&self) -> bool { self.phase == 10 && !self.closing }
    pub fn row_count(&self) -> usize { self.rows }
    pub fn transfer_copy_demand()->usize { size_of::<semio_framework_os_spr::EditMessages>()+size_of::<usize>()+size_of::<Vec<semio_framework_os_spr::EditMessages>>()+size_of::<Option<semio_framework_diagnostic::Severity>>() }
    pub fn measured_bytes(&self, identity: &str) -> Result<usize, ValueError> { self.bytes.checked_add(identity.len()).ok_or_else(|| diagnostic_fault("diagnostic entry byte extent overflow")) }
    pub fn joined(&self) -> bool { self.phase >= 1 }
    pub fn joined_transfer_ready(&self) -> bool { self.phase==1 && !self.closing && self.ready.as_ref().is_some_and(MutationMessagePublication::ready) }
    pub fn advance_join(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { if self.joined() { Ok(RetainedCloneStep::Complete(Default::default())) } else { self.advance("", grant) } }
    pub fn take_joined(&mut self, grant: RetainedCloneGrant) -> Option<(MutationMessagePublication, RetainedCloneProgress)> {
        let copy = size_of::<MutationMessagePublication>();
        if self.phase != 1 || self.closing || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy { return None; }
        self.closing = true;
        Some((self.ready.take()?, RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() }))
    }
    pub fn admit_displaced(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let idle = RetainedCloneProgress::default(); let capacity = size_of::<MutationMessageLedgerRetirement>();
        if self.displaced.is_some() || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_capacity_bytes < capacity { return Ok(idle); }
        self.displaced = Some(Box::new(MutationMessageLedgerRetirement::empty()));
        Ok(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..idle })
    }

    pub fn prepare_prefix(&mut self, source: Option<&[semio_framework_os_spr::MutationMessage]>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.prefix_done { return Ok(RetainedCloneStep::Complete(idle)); }
        if self.closing || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(idle)); }
        let Some(source) = source else { if grant.maximum_copy_bytes < size_of::<bool>() { return Ok(RetainedCloneStep::Progress(idle)); } self.prefix_done = true; return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<bool>(), ..idle })); };
        if let Some(owner) = self.prefix_ready.as_ref() {
            let count = self.rows.checked_add(owner.original_messages().len()).ok_or_else(|| diagnostic_fault("diagnostic prefix row census overflow"))?;
            let copy = size_of::<MutationMessagePublication>();
            if grant.maximum_copy_bytes < copy { return Ok(RetainedCloneStep::Progress(idle)); }
            let original = self.prefix_ready.take().unwrap();
            return match self.batch.as_mut().ok_or_else(|| diagnostic_fault("diagnostic prefix batch is absent"))?.append(original, grant) { Ok(receipt) => { self.rows = count; self.prefix_done = true; self.prefix = None; Ok(RetainedCloneStep::Complete(receipt)) }, Err(original) => { self.prefix_ready = Some(original); Ok(RetainedCloneStep::Progress(idle)) } };
        }
        if let Some(owner) = self.prefix.as_mut() {
            if owner.ready() { let Some((ready, receipt)) = owner.take(grant) else { return Ok(RetainedCloneStep::Progress(idle)); }; self.prefix_ready = Some(ready); return Ok(RetainedCloneStep::Progress(receipt)); }
            return owner.advance(source, grant);
        }
        let copy = size_of::<semio_framework_os_spr::command::MutationMessageBorrowedPublication>();
        if grant.maximum_copy_bytes < copy { return Ok(RetainedCloneStep::Progress(idle)); }
        self.prefix = Some(Default::default());
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..idle }))
    }

    pub fn advance(&mut self, edit_id: &str, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.prepared() { return Ok(RetainedCloneStep::Complete(idle)); }
        if self.closing || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(idle)); }
        let mut receipt = RetainedCloneProgress { copied_items: 1, ..idle };
        match self.phase {
            0 => {
                if self.join.is_none() {
                    if grant.maximum_copy_bytes < size_of::<MutationMessagePublicationJoin>() { return Ok(RetainedCloneStep::Progress(idle)); }
                    let batch = self.batch.take().ok_or_else(|| diagnostic_fault("diagnostic batch originals are absent"))?;
                    self.join = Some(MutationMessagePublicationJoin::new(batch, self.rows)); receipt.copied_bytes = size_of::<MutationMessagePublicationJoin>();
                } else {
                    let join = self.join.as_mut().unwrap();
                    if join.ready() {
                        let Some((ready, progress)) = join.take(grant)? else { return Ok(RetainedCloneStep::Progress(idle)); };
                        self.ready = Some(ready); self.join = None; self.phase = 1; receipt = progress;
                    } else { return join.advance(grant); }
                }
            }
            1 => {
                if edit_id.is_empty()||edit_id.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES{return Err(diagnostic_fault("diagnostic selection requires actual bounded edit context"));}
                let ready=self.ready.as_mut().ok_or_else(||diagnostic_fault("joined diagnostic original is absent"))?;
                if !ready.selection_ready(edit_id.len()){return ready.select(edit_id.len(),grant);}
                self.rows=ready.original_messages().len();
                let ready = self.ready.as_ref().unwrap();
                if self.severity_index == ready.original_messages().len() { self.phase = 2; }
                else {
                    let row = &ready.original_messages()[self.severity_index];
                    if let Some(index) = self.measure_target {
                        if let Some(target) = row.target.get(index) { self.bytes = self.bytes.checked_add(ARTIFACT_EDIT_MESSAGE_TARGET_BYTES).and_then(|bytes| bytes.checked_add(target.len())).ok_or_else(|| diagnostic_fault("diagnostic target byte extent overflow"))?; self.measure_target = Some(index + 1); }
                        else { self.measure_target = None; self.severity_index += 1; }
                    } else {
                        self.bytes = self.bytes.checked_add(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES).and_then(|bytes| bytes.checked_add(row.code.0.len())).and_then(|bytes| bytes.checked_add(row.message.len())).ok_or_else(|| diagnostic_fault("diagnostic row byte extent overflow"))?;
                        self.worst = Some(self.worst.map_or(row.level, |previous| previous.max(row.level))); self.measure_target = Some(0);
                    }
                }
            }
            2 | 4 => {
                if edit_id.is_empty() || edit_id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES { return Err(diagnostic_fault("diagnostic context exceeds its actual edit identity bound")); }
                let index = usize::from(self.phase == 4);
                if grant.maximum_capacity_bytes < edit_id.len() { return Ok(RetainedCloneStep::Progress(idle)); }
                self.identity[index].try_reserve_exact(edit_id.len()).map_err(|_| diagnostic_fault("diagnostic identity allocation failed"))?;
                receipt.retained_capacity_bytes = self.identity[index].capacity(); self.phase += 1;
            }
            3 | 5 => {
                let index = usize::from(self.phase == 5);
                let target = &mut self.identity[index];
                if !edit_id.starts_with(target.as_str()) { return Err(diagnostic_fault("diagnostic edit context changed during preparation")); }
                if let Some(scalar) = edit_id[target.len()..].chars().next() {
                    if grant.maximum_copy_bytes < scalar.len_utf8() { return Ok(RetainedCloneStep::Progress(idle)); }
                    target.push(scalar); receipt.copied_bytes = scalar.len_utf8();
                } else { self.phase += 1; }
            }
            6 => {
                let frame = 2 * size_of::<MutationMessageLedgerRetirement>();
                if grant.maximum_copy_bytes < frame { return Ok(RetainedCloneStep::Progress(idle)); }
                let Some((original, report, progress)) = self.ready.as_mut().ok_or_else(|| diagnostic_fault("diagnostic ready owner is absent"))?.take(RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - frame, ..grant })? else { return Ok(RetainedCloneStep::Progress(idle)); };
                self.original = Some(MutationMessageLedgerRetirement::new(std::mem::take(&mut self.identity[0]), original));
                self.report = Some(MutationMessageLedgerRetirement::new(std::mem::take(&mut self.identity[1]), report));
                receipt = RetainedCloneProgress { copied_bytes: progress.copied_bytes + frame, ..progress }; self.phase = 7;
            }
            7 => {
                let ready = self.ready.as_mut().ok_or_else(|| diagnostic_fault("transferred diagnostic shell is absent"))?;
                ready.begin_close();
                if grant.maximum_depth < 2 { return Ok(RetainedCloneStep::Progress(idle)); }
                let step = ready.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })?;
                if ready.terminal_is_empty() { self.ready = None; self.phase = 8; }
                receipt = step.progress();
            }
            8 => {
                let capacity = size_of::<semio_framework_os_spr::EditMessages>();
                if grant.maximum_capacity_bytes < capacity { return Ok(RetainedCloneStep::Progress(idle)); }
                self.records.try_reserve_exact(1).map_err(|_| diagnostic_fault("diagnostic command record allocation failed"))?;
                receipt.retained_capacity_bytes = self.records.capacity() * capacity; self.phase = 9;
            }
            9 => {
                let copy = size_of::<semio_framework_os_spr::EditMessages>();
                if grant.maximum_copy_bytes < copy { return Ok(RetainedCloneStep::Progress(idle)); }
                let Some((record, progress)) = self.report.as_mut().ok_or_else(|| diagnostic_fault("diagnostic report original is absent"))?.take_edit_messages_controlled(RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - copy, ..grant })? else { return Ok(RetainedCloneStep::Progress(idle)); };
                self.records.push(record); self.report = None; receipt = RetainedCloneProgress { copied_bytes: progress.copied_bytes + copy, ..progress }; self.phase = 10;
            }
            _ => return Err(diagnostic_fault("diagnostic staging already completed")),
        }
        Ok(if self.prepared() { RetainedCloneStep::Complete(receipt) } else { RetainedCloneStep::Progress(receipt) })
    }

    pub fn original_messages(&self) -> &[semio_framework_os_spr::MutationMessage] { self.original.as_ref().map_or_else(|| self.ready.as_ref().map_or(&[], MutationMessagePublication::original_messages), MutationMessageLedgerRetirement::retained_messages) }

    pub fn take(&mut self, grant: RetainedCloneGrant) -> Result<Option<(ArtifactStoreDiagnosticRecord, Vec<semio_framework_os_spr::EditMessages>, Option<semio_framework_diagnostic::Severity>, RetainedCloneProgress)>, ValueError> {
        let frame = size_of::<usize>()+size_of::<Vec<semio_framework_os_spr::EditMessages>>() + size_of::<Option<semio_framework_diagnostic::Severity>>();
        if !self.prepared() || grant.maximum_copy_bytes < frame { return Ok(None); }
        let bytes=self.measured_bytes(&self.records.first().ok_or_else(||diagnostic_fault("diagnostic report context is absent"))?.edit_id)?;
        let Some((original, receipt)) = self.original.as_mut().ok_or_else(|| diagnostic_fault("diagnostic durable original is absent"))?.take_edit_messages_controlled(RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - frame, ..grant })? else { return Ok(None); };
        self.original = None; self.closing = true;
        Ok(Some((ArtifactStoreDiagnosticRecord{entry:original,bytes}, std::mem::take(&mut *self.records), self.worst.take(), RetainedCloneProgress { copied_bytes: receipt.copied_bytes + frame, ..receipt })))
    }

    pub fn begin_close(&mut self) { self.closing = true; if let Some(owner) = self.batch.as_mut() { owner.begin_close(); } if let Some(owner) = self.join.as_mut() { owner.begin_close(); } if let Some(owner) = self.ready.as_mut() { owner.begin_close(); } if let Some(owner) = self.prefix.as_mut() { owner.begin_close(); } if let Some(owner) = self.prefix_ready.as_mut() { owner.begin_close(); } }
    pub fn terminal_is_empty(&self) -> bool { self.displaced.is_none() && self.batch.is_none() && self.join.is_none() && self.ready.is_none() && self.original.is_none() && self.report.is_none() && self.records_retirement.is_none() && self.prefix.is_none() && self.prefix_ready.is_none() && self.records.capacity() == 0 && self.identity.iter().all(|value| value.capacity() == 0) }

    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let nested = |owner: &dyn ErasedSnapshotRetirement, frame: usize| -> Result<RetirementDemand, ValueError> {
            if owner.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: frame, depth: 1, ..Default::default() }); }
            let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||diagnostic_fault("diagnostic stage close depth overflow"))?;Ok(demand)
        };
        if let Some(owner) = self.displaced.as_ref() { if owner.terminal_is_empty() { return Ok(RetirementDemand { release_bytes: size_of::<MutationMessageLedgerRetirement>(), depth: 1, ..Default::default() }); } return nested(owner.as_ref(), size_of::<MutationMessageLedgerRetirement>()); }
        if let Some(owner) = self.batch.as_ref() { return nested(owner, size_of::<MutationMessagePublicationBatch>()); }
        if let Some(owner) = self.prefix.as_ref() { return nested(owner, size_of::<semio_framework_os_spr::command::MutationMessageBorrowedPublication>()); }
        if let Some(owner) = self.prefix_ready.as_ref() { return nested(owner, size_of::<MutationMessagePublication>()); }
        if let Some(owner) = self.join.as_ref() { return nested(owner, size_of::<MutationMessagePublicationJoin>()); }
        if let Some(owner) = self.ready.as_ref() { return nested(owner, size_of::<MutationMessagePublication>()); }
        if let Some(owner) = self.original.as_ref().or(self.report.as_ref()) { return nested(owner, size_of::<MutationMessageLedgerRetirement>()); }
        if let Some(owner) = self.records_retirement.as_ref() { return nested(owner, size_of::<semio_framework_os_spr::command::EditMessageLedgerRetirement>()); }
        if self.records.capacity() != 0 { return Ok(RetirementDemand { copy_bytes: size_of::<semio_framework_os_spr::command::EditMessageLedgerRetirement>(), depth: 1, ..Default::default() }); }
        Ok(RetirementDemand { release_bytes: self.identity.iter().find(|value| value.capacity() != 0).map_or(0, String::capacity), depth: 1, ..Default::default() })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(idle)); }
        if !self.closing { return Ok(RetainedCloneStep::Progress(idle)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(idle)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(owner) = self.displaced.as_mut() { if owner.terminal_is_empty() { self.displaced = None; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..idle })); } return owner.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        macro_rules! close_owner {
            ($slot:expr) => { if let Some(owner) = $slot.as_mut() {
                if owner.terminal_is_empty() { $slot = None; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..idle })); }
                return owner.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
            } };
        }
        close_owner!(self.batch);
        close_owner!(self.prefix);
        close_owner!(self.prefix_ready);
        close_owner!(self.join);
        close_owner!(self.ready);
        close_owner!(self.original);
        close_owner!(self.report);
        close_owner!(self.records_retirement);
        if self.records.capacity() != 0 { self.records_retirement = Some(semio_framework_os_spr::command::EditMessageLedgerRetirement::new(std::mem::take(&mut *self.records))); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..idle })); }
        if let Some(value) = self.identity.iter_mut().find(|value| value.capacity() != 0) { drop(std::mem::take(value)); }
        let receipt = RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..idle };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(receipt) } else { RetainedCloneStep::Progress(receipt) })
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreDiagnosticStage {
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.close_demands(body) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Self::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { Self::terminal_is_empty(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.depth) }
}

impl Drop for ArtifactStoreDiagnosticStage {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "diagnostic stage requires exact terminal closure"); if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.records); } } }
}

fn diagnostic_fault(message: &'static str) -> ValueError { ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message) }

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;


//! 🔁️ Private replay diagnostics preparation borrows a fenced source and publishes by one original-owner swap.
use super::*;
use protocol::mutation::MessageCopyCursor;
use semio_framework_value::{RetirementDemand, ValueRefusalKind};
use std::mem::ManuallyDrop;
#[derive(Clone, Copy)]
enum Selected {
    Baseline(u16),
    Replay(usize),
}
#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Validate,
    Duplicate,
    Admit,
    Baseline,
    Search,
    Append,
    Measure,
    Target,
    IdBirth,
    IdCopy,
    MessagesBirth,
    Message,
    MessageTake,
    MessageClose,
    Move,
    Probe,
    Insert,
    Ready,
    Published,
}
/// 🧬️ A pure retained codec; its enclosing publisher must validate its original Store/result source fence each turn.
pub struct OriginalReplayDiagnostics {
    source: Option<(usize, usize, usize, usize)>,
    ledger: ManuallyDrop<Option<ArtifactEditMessageLedger>>,
    retiring: Option<ArtifactEditMessageLedgerRetirement>,
    row: ManuallyDrop<Option<protocol::EditMessages>>,
    row_close: Option<ArtifactStoreMessageLedgerRetirement>,
    copy: Option<MessageCopyCursor>,
    selected: Option<Selected>,
    phase: Phase,
    validate: usize,
    compare: usize,
    baseline: u16,
    next: u16,
    append: usize,
    message: usize,
    target: usize,
    bytes: usize,
    bucket: usize,
    closing: bool,
}
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, message)
}
fn limit(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::OwnershipLimit, message)
}
fn fixed(bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes: bytes, depth: 1, ..Default::default() }
}
fn fits(d: RetirementDemand, g: RetainedCloneGrant) -> bool {
    g.maximum_items > 0 && g.maximum_copy_bytes >= d.copy_bytes && g.maximum_capacity_bytes >= d.capacity_bytes && g.maximum_release_bytes >= d.release_bytes && g.maximum_depth >= d.depth
}
fn event(d: RetirementDemand) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, retained_capacity_bytes: d.capacity_bytes, released_bytes: d.release_bytes }
}
impl Default for OriginalReplayDiagnostics {
    fn default() -> Self {
        Self {
            source: None,
            ledger: ManuallyDrop::new(Some(ArtifactEditMessageLedger::empty())),
            retiring: None,
            row: ManuallyDrop::new(None),
            row_close: None,
            copy: None,
            selected: None,
            phase: Phase::Validate,
            validate: 0,
            compare: 0,
            baseline: ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE,
            next: ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE,
            append: 0,
            message: 0,
            target: 0,
            bytes: 0,
            bucket: 0,
            closing: false,
        }
    }
}
impl OriginalReplayDiagnostics {
    fn check(&self, baseline: &ArtifactEditMessageLedger, rows: &[protocol::EditMessages]) -> Result<(), ValueError> {
        let source = (std::ptr::from_ref(baseline) as usize, baseline.len(), rows.as_ptr() as usize, rows.len());
        if self.source.is_some_and(|original| original != source) {
            return Err(invalid("replay diagnostics changed its fenced source"));
        }
        Ok(())
    }
    fn selected<'a>(&self, baseline: &'a ArtifactEditMessageLedger, rows: &'a [protocol::EditMessages]) -> Result<&'a protocol::EditMessages, ValueError> {
        match self.selected.ok_or_else(|| invalid("replay diagnostics lost selected row"))? {
            Selected::Baseline(slot) => Ok(unsafe { baseline.slots[usize::from(slot)].assume_init_ref() }),
            Selected::Replay(rank) => rows.get(rank).ok_or_else(|| invalid("replay diagnostics lost original result row")),
        }
    }
    fn select(&mut self, selected: Selected) {
        self.selected = Some(selected);
        self.message = 0;
        self.target = 0;
        self.bytes = 0;
        self.phase = Phase::Measure;
    }
    fn finish_row(&mut self) {
        self.selected = None;
        if self.baseline != ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE {
            self.baseline = self.next;
            self.phase = Phase::Baseline;
        } else {
            self.append += 1;
            self.phase = Phase::Append;
        }
    }
    fn add_bytes(&mut self, bytes: usize) -> Result<(), ValueError> {
        self.bytes = self.bytes.checked_add(bytes).filter(|n| *n <= ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES).ok_or_else(|| limit("replay diagnostics row exceeds its fixed byte authority"))?;
        Ok(())
    }
    /// 📏️ Quotes exactly one physical birth, comparison, source visit or body fragment.
    pub fn demands(&self, baseline: &ArtifactEditMessageLedger, rows: &[protocol::EditMessages], body: usize) -> Result<RetirementDemand, ValueError> {
        self.check(baseline, rows)?;
        if self.closing {
            return self.close_demands(body);
        }
        if let Some(row) = self.row_close.as_ref() {
            return row.next_demand(body);
        }
        let mut d = fixed(0);
        match self.phase {
            Phase::Admit => {
                if !self.ledger.as_ref().unwrap().admission_is_complete() {
                    return self.ledger.as_ref().unwrap().admission_demands();
                }
            }
            Phase::IdBirth => {
                let row = self.selected(baseline, rows)?;
                d.capacity_bytes = row.edit_id.len();
                d.copy_bytes = size_of::<protocol::EditMessages>();
            }
            Phase::IdCopy => {
                let row = self.selected(baseline, rows)?;
                let copied = self.row.as_ref().unwrap().edit_id.len();
                d.copy_bytes = row.edit_id[copied..].chars().next().map_or(0, char::len_utf8);
            }
            Phase::MessagesBirth => {
                d.capacity_bytes = self.selected(baseline, rows)?.messages.len().checked_mul(size_of::<protocol::mutation::MutationMessage>()).ok_or_else(|| limit("replay diagnostics message vector extent overflowed"))?;
            }
            Phase::Message => {
                let row = self.selected(baseline, rows)?;
                if let Some(message) = row.messages.get(self.message) {
                    let copy = self.copy.as_ref().ok_or_else(|| invalid("replay diagnostics lost message cursor"))?;
                    d.copy_bytes = copy.next_copy_byte_demand(message)?;
                    d.capacity_bytes = copy.next_capacity_byte_demand(message)?;
                }
            }
            Phase::MessageTake => d.copy_bytes = 2 * size_of::<protocol::mutation::MutationMessage>(),
            Phase::MessageClose => return self.copy.as_ref().unwrap().next_demand(body),
            Phase::Move => d.copy_bytes = 2 * size_of::<protocol::EditMessages>(),
            Phase::Insert => d.copy_bytes = 2 * size_of::<protocol::EditMessages>(),
            _ => {}
        }
        Ok(d)
    }
    /// 📥️ Prepares one event and retains every original row on any currency refusal.
    pub fn advance(&mut self, baseline: &ArtifactEditMessageLedger, rows: &mut [protocol::EditMessages], grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.check(baseline, rows)?;
        if self.closing {
            return Err(invalid("replay diagnostics preparation is closing"));
        }
        if self.prepared() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        let d = self.demands(baseline, rows, grant.maximum_copy_bytes)?;
        if !fits(d, grant) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if let Some(row) = self.row_close.as_mut() {
            let p = row.close_step(grant)?.progress();
            if row.terminal_is_empty() {
                self.row_close = None;
            }
            return Ok(RetainedCloneStep::Progress(p));
        }
        if self.source.is_none() {
            self.source = Some((std::ptr::from_ref(baseline) as usize, baseline.len(), rows.as_ptr() as usize, rows.len()));
            if rows.len() > ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY {
                return Err(limit("replay diagnostics result count exceeds its fixed authority"));
            }
        }
        let mut p = event(d);
        match self.phase {
            Phase::Validate => {
                if self.validate == rows.len() {
                    self.phase = Phase::Admit;
                } else {
                    let id = &rows[self.validate].edit_id;
                    if id.is_empty() || id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES {
                        return Err(limit("replay diagnostics result identity is outside its fixed authority"));
                    }
                    self.compare = 0;
                    self.phase = Phase::Duplicate;
                }
            }
            Phase::Duplicate => {
                if self.compare == self.validate {
                    self.validate += 1;
                    self.phase = Phase::Validate;
                } else {
                    if rows[self.compare].edit_id == rows[self.validate].edit_id {
                        return Err(limit("replay diagnostics result contains duplicate identities"));
                    }
                    self.compare += 1;
                }
            }
            Phase::Admit => {
                let ledger = self.ledger.as_mut().unwrap();
                if ledger.admission_is_complete() {
                    self.baseline = baseline.head;
                    self.phase = Phase::Baseline;
                } else {
                    p = ledger.admit_next(grant)?;
                }
            }
            Phase::Baseline => {
                if self.baseline == ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE {
                    self.phase = Phase::Append;
                } else {
                    let rank = usize::from(self.baseline);
                    let row = unsafe { baseline.slots[rank].assume_init_ref() };
                    if row.edit_id.is_empty() || row.edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES {
                        return Err(limit("replay diagnostics baseline identity is outside its fixed authority"));
                    }
                    self.next = baseline.next[rank];
                    self.compare = 0;
                    self.phase = Phase::Search;
                }
            }
            Phase::Search => {
                if self.compare == rows.len() {
                    self.select(Selected::Baseline(self.baseline));
                } else {
                    let row = unsafe { baseline.slots[usize::from(self.baseline)].assume_init_ref() };
                    if row.edit_id == rows[self.compare].edit_id {
                        self.select(Selected::Replay(self.compare));
                    } else {
                        self.compare += 1;
                    }
                }
            }
            Phase::Append => {
                if self.append == rows.len() {
                    self.phase = Phase::Ready;
                } else if rows[self.append].edit_id.is_empty() {
                    self.append += 1;
                } else {
                    self.select(Selected::Replay(self.append));
                }
            }
            Phase::Measure => {
                let row = self.selected(baseline, rows)?;
                if self.message == 0 && self.bytes == 0 {
                    self.add_bytes(row.edit_id.len())?;
                } else if self.message == row.messages.len() {
                    self.message = 0;
                    self.phase = match self.selected.unwrap() {
                        Selected::Replay(_) => Phase::Move,
                        Selected::Baseline(_) => Phase::IdBirth,
                    };
                } else {
                    let message = &row.messages[self.message];
                    let bytes = ARTIFACT_EDIT_MESSAGE_OWNER_BYTES.checked_add(message.code.0.len()).and_then(|n| n.checked_add(message.message.len())).ok_or_else(|| limit("replay diagnostics row byte extent overflowed"))?;
                    self.add_bytes(bytes)?;
                    self.target = 0;
                    self.phase = Phase::Target;
                }
            }
            Phase::Target => {
                let row = self.selected(baseline, rows)?;
                if self.target == row.messages[self.message].target.len() {
                    self.message += 1;
                    self.phase = Phase::Measure;
                } else {
                    let bytes = ARTIFACT_EDIT_MESSAGE_TARGET_BYTES.checked_add(row.messages[self.message].target[self.target].len()).ok_or_else(|| limit("replay diagnostics target extent overflowed"))?;
                    self.add_bytes(bytes)?;
                    self.target += 1;
                }
            }
            Phase::IdBirth => {
                let mut id = String::new();
                id.try_reserve_exact(d.capacity_bytes).map_err(|_| limit("replay diagnostics identity allocation failed"))?;
                p.retained_capacity_bytes = id.capacity();
                *self.row = Some(protocol::EditMessages { edit_id: id, messages: Vec::new() });
                self.phase = Phase::IdCopy;
            }
            Phase::IdCopy => {
                let source = self.selected(baseline, rows)?;
                let id = &mut self.row.as_mut().unwrap().edit_id;
                if let Some(scalar) = source.edit_id[id.len()..].chars().next() {
                    id.push(scalar);
                } else {
                    self.phase = Phase::MessagesBirth;
                }
            }
            Phase::MessagesBirth => {
                let count = self.selected(baseline, rows)?.messages.len();
                let row = self.row.as_mut().unwrap();
                row.messages.try_reserve_exact(count).map_err(|_| limit("replay diagnostics message allocation failed"))?;
                p.retained_capacity_bytes = row.messages.capacity() * size_of::<protocol::mutation::MutationMessage>();
                self.copy = Some(MessageCopyCursor::new());
                self.phase = Phase::Message;
            }
            Phase::Message => {
                let source = self.selected(baseline, rows)?;
                if let Some(message) = source.messages.get(self.message) {
                    let copy = self.copy.as_mut().unwrap();
                    p = copy.advance(message, grant)?.progress();
                    if copy.ready() {
                        self.phase = Phase::MessageTake;
                    }
                } else {
                    self.copy.as_mut().unwrap().begin_close();
                    self.phase = Phase::MessageClose;
                }
            }
            Phase::MessageTake => {
                let copy = self.copy.as_mut().unwrap();
                self.row.as_mut().unwrap().messages.push(copy.take().unwrap());
                copy.begin_close();
                self.phase = Phase::MessageClose;
            }
            Phase::MessageClose => {
                let copy = self.copy.as_mut().unwrap();
                p = copy.close_step(grant)?.progress();
                if copy.terminal_is_empty() {
                    self.copy = None;
                    if self.row.as_ref().unwrap().messages.len() == self.selected(baseline, rows)?.messages.len() {
                        self.bucket = Self::initial_bucket(self.row.as_ref().unwrap());
                        self.phase = Phase::Probe;
                    } else {
                        self.message += 1;
                        self.copy = Some(MessageCopyCursor::new());
                        self.phase = Phase::Message;
                    }
                }
            }
            Phase::Move => {
                let Selected::Replay(rank) = self.selected.unwrap() else { return Err(invalid("replay diagnostics move requires original result")) };
                *self.row = Some(std::mem::replace(&mut rows[rank], protocol::EditMessages { edit_id: String::new(), messages: Vec::new() }));
                if self.row.as_ref().unwrap().messages.is_empty() {
                    let row = self.row.take().unwrap();
                    self.row_close = Some(ArtifactStoreMessageLedgerRetirement::new(row.edit_id, row.messages));
                    self.finish_row();
                } else {
                    self.bucket = Self::initial_bucket(self.row.as_ref().unwrap());
                    self.phase = Phase::Probe;
                }
            }
            Phase::Probe => {
                let ledger = self.ledger.as_ref().unwrap();
                if ledger.len == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY {
                    return Err(limit("replay diagnostics prepared count exceeds its fixed authority"));
                }
                if ledger.buckets[self.bucket] == ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY {
                    self.phase = Phase::Insert;
                } else {
                    self.bucket = (self.bucket + 1) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
                }
            }
            Phase::Insert => {
                let row = self.row.take().unwrap();
                let ledger = self.ledger.as_mut().unwrap();
                ledger.free_len -= 1;
                let slot = usize::from(ledger.free[ledger.free_len]);
                ledger.buckets[self.bucket] = slot as u16;
                ledger.slots[slot].write(row);
                ledger.generations[slot] = 1;
                ledger.occupied[slot] = true;
                ledger.previous[slot] = ledger.tail;
                ledger.next[slot] = ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE;
                if ledger.tail == ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE {
                    ledger.head = slot as u16;
                } else {
                    ledger.next[usize::from(ledger.tail)] = slot as u16;
                }
                ledger.tail = slot as u16;
                ledger.len += 1;
                self.finish_row();
            }
            Phase::Ready | Phase::Published => {}
        }
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant, p, "original replay diagnostics preparation")?;
        Ok(if self.prepared() { RetainedCloneStep::Complete(p) } else { RetainedCloneStep::Progress(p) })
    }
    fn initial_bucket(row: &protocol::EditMessages) -> usize {
        ArtifactEditMessageLedger::hash(&row.edit_id) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY
    }
    pub fn prepared(&self) -> bool {
        self.phase == Phase::Ready && !self.closing
    }
    pub fn swap_demands(&self) -> RetirementDemand {
        fixed(2 * size_of::<ArtifactEditMessageLedger>() + size_of::<ArtifactEditMessageLedgerRetirement>())
    }
    /// 🪪️ Checks the exact prepared original baseline before a common publisher moves any visible root.
    pub fn can_swap_prepared(&self,baseline:&ArtifactEditMessageLedger,grant:RetainedCloneGrant)->bool{self.prepared()&&self.retiring.is_none()&&self.source.is_some_and(|source|source.0==std::ptr::from_ref(baseline)as usize&&source.1==baseline.len())&&fits(self.swap_demands(),grant)}
    /// 🔄️ Publishes the complete private ledger and retains the exact displaced owner for paid closure.
    pub fn swap_prepared(&mut self, baseline: &mut ArtifactEditMessageLedger, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        if !self.prepared() || self.retiring.is_some() || self.source.is_none_or(|source| source.0 != std::ptr::from_ref(baseline) as usize || source.1 != baseline.len()) {
            return Err(invalid("replay diagnostics swap lost its prepared fenced baseline"));
        }
        let d = self.swap_demands();
        if !fits(d, grant) {
            return Ok(Default::default());
        }
        let old = std::mem::replace(baseline, self.ledger.take().unwrap());
        self.retiring = Some(ArtifactEditMessageLedgerRetirement::new(old));
        self.phase = Phase::Published;
        Ok(event(d))
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
        if let Some(copy) = self.copy.as_mut() {
            copy.begin_close();
        }
    }
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(copy) = self.copy.as_ref() {
            return copy.next_demand(body);
        }
        if let Some(row) = self.row_close.as_ref() {
            return row.next_demand(body);
        }
        if self.row.is_some() {
            return Ok(fixed(size_of::<protocol::EditMessages>()));
        }
        if let Some(ledger) = self.retiring.as_ref() {
            return ledger.demands(body);
        }
        if self.ledger.is_some() {
            return Ok(fixed(size_of::<ArtifactEditMessageLedger>() + size_of::<ArtifactEditMessageLedgerRetirement>()));
        }
        Ok(fixed(0))
    }
    /// ♻️ Closes partial copies, moved result rows and all private or displaced physical backing.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Err(invalid("replay diagnostics closure was not requested"));
        }
        let d = self.close_demands(grant.maximum_copy_bytes)?;
        if !fits(d, grant) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let mut p = event(d);
        if let Some(copy) = self.copy.as_mut() {
            p = copy.close_step(grant)?.progress();
            if copy.terminal_is_empty() {
                self.copy = None;
            }
        } else if let Some(row) = self.row_close.as_mut() {
            p = row.close_step(grant)?.progress();
            if row.terminal_is_empty() {
                self.row_close = None;
            }
        } else if let Some(row) = self.row.take() {
            self.row_close = Some(ArtifactStoreMessageLedgerRetirement::new(row.edit_id, row.messages));
        } else if let Some(ledger) = self.retiring.as_mut() {
            p = ledger.close_step(grant)?.progress();
            if ledger.terminal_is_empty() {
                self.retiring = None;
            }
        } else if let Some(ledger) = self.ledger.take() {
            self.retiring = Some(ArtifactEditMessageLedgerRetirement::new(ledger));
        } else {
            self.source = None;
            self.selected = None;
        }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(p) } else { RetainedCloneStep::Progress(p) })
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.source.is_none() && self.ledger.is_none() && self.retiring.is_none() && self.row.is_none() && self.row_close.is_none() && self.copy.is_none()
    }
}
impl Drop for OriginalReplayDiagnostics {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "replay diagnostics retains every partial or displaced original until physical closure")
    }
}

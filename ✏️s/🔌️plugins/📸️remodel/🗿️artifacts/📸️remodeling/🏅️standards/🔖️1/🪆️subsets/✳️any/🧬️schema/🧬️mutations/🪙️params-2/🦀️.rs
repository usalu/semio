//! 🪙️ remodeling parameter mutations: the eight update-*-params leaves (part 2).

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;

#[cfg(test)]
use semio_framework_value_testing_allocator as _;

/// 🧮️ Fixed-width source text for parameter refusal diagnostics; retained custody and grants belong to the preparation that holds it.
pub struct RetainedParameterNumericText {
    bytes: [u8; 256],
    len: usize,
}

impl std::fmt::Write for RetainedParameterNumericText {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        let end = self.len.checked_add(value.len()).ok_or(std::fmt::Error)?;
        let destination = self.bytes.get_mut(self.len..end).ok_or(std::fmt::Error)?;
        destination.copy_from_slice(value.as_bytes());
        self.len = end;
        Ok(())
    }
}

impl RetainedParameterNumericText {
    /// 📏️ Ingest's 155 fixed bytes, two ten-digit integers and at most 56 binary32 decimal bytes fit in 256.
    pub const MAX_BYTES: usize = 256;

    fn empty() -> Self { Self { bytes: [0; Self::MAX_BYTES], len: 0 } }

    /// 🥣️ Formats the ingest invariant without allocating a string.
    pub fn ingest(sharpness: f32, frames: u32, stride: u32) -> Result<Self, std::fmt::Error> {
        use std::fmt::Write;
        let mut text = Self::empty();
        write!(text, "Ingest params need a finite non-negative min sharpness and positive max frames/frame sample stride (got min_sharpness={sharpness}, max_frames={frames}, frame_sample_stride={stride}).")?;
        Ok(text)
    }

    /// 🌠️ Formats the feature invariant without allocating a string.
    pub fn feature(count: u32, edge: f32) -> Result<Self, std::fmt::Error> {
        use std::fmt::Write;
        let mut text = Self::empty();
        write!(text, "Feature params need a positive target count and a finite non-negative edge threshold (got target_count={count}, edge_threshold={edge}).")?;
        Ok(text)
    }

    /// 🪢️ Formats the matching invariant without allocating a string.
    pub fn matching(ratio: f32) -> Result<Self, std::fmt::Error> {
        use std::fmt::Write;
        let mut text = Self::empty();
        write!(text, "Match ratio test {ratio} must be finite and within (0, 1].")?;
        Ok(text)
    }

    /// 👁️ Borrows only the initialized diagnostic bytes.
    pub fn as_str(&self) -> &str { std::str::from_utf8(&self.bytes[..self.len]).expect("numeric diagnostic text is UTF-8") }
}

use semio_framework_value::{ErasedSnapshotRetirement, OriginalRetirementFailure, OriginalRetirementOutcome, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneBinding, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep};
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::{FeatureParams, IngestParams, MatchParams};
use std::mem::{size_of, ManuallyDrop};

/// 🎯️ The three fixed-width original parameter records that can produce numeric diagnostic text.
#[derive(Clone, Copy)]
pub enum RetainedParameterTextSource<'a> {
    Ingest(RetainedCloneRef<'a, IngestParams>),
    Feature(RetainedCloneRef<'a, FeatureParams>),
    Matching(RetainedCloneRef<'a, MatchParams>),
}

impl RetainedParameterTextSource<'_> {
    fn kind(self) -> u8 { match self { Self::Ingest(_) => 1, Self::Feature(_) => 2, Self::Matching(_) => 3 } }
    fn check(self, binding: &Option<RetainedCloneBinding>) -> Result<bool, ValueError> { match self { Self::Ingest(x) => x.check_binding(binding), Self::Feature(x) => x.check_binding(binding), Self::Matching(x) => x.check_binding(binding) } }
    fn binding_bytes(self) -> usize { match self { Self::Ingest(x) => x.binding_copy_bytes(), Self::Feature(x) => x.binding_copy_bytes(), Self::Matching(x) => x.binding_copy_bytes() } }
    fn bind(self, binding: &mut Option<RetainedCloneBinding>, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<Option<RetainedCloneProgress>, ValueError> {
        match self { Self::Ingest(x) => x.bind(binding, effect), Self::Feature(x) => x.bind(binding, effect), Self::Matching(x) => x.bind(binding, effect) }
    }
    fn format(self) -> Result<RetainedParameterNumericText, std::fmt::Error> {
        match self {
            Self::Ingest(x) => { let p = x.get(); RetainedParameterNumericText::ingest(p.min_sharpness, p.max_frames, p.frame_sample_stride) },
            Self::Feature(x) => { let p = x.get(); RetainedParameterNumericText::feature(p.target_count, p.edge_threshold) },
            Self::Matching(x) => RetainedParameterNumericText::matching(x.get().ratio_test),
        }
    }
}

fn text_invalid() -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, "retained parameter text lost its source or phase") }
fn text_scalar(copy_bytes: usize) -> RetirementDemand { RetirementDemand { copy_bytes, depth: 1, ..Default::default() } }

/// 🧳️ Binds one genuine source, formats its fixed-width fields, and stages the owned text under separate grants.
pub struct RetainedParameterTextOwner {
    binding: Option<RetainedCloneBinding>,
    source_kind: u8,
    text: Option<RetainedParameterNumericText>,
    output: ManuallyDrop<Option<String>>,
    retirement: RetainedCloneClose,
    offset: usize,
    phase: u8,
    closing: bool,
}

impl RetainedParameterTextOwner {
    /// 🌱️ Creates an empty owner without inspecting a source.
    pub fn new() -> Self { Self { binding: None, source_kind: 0, text: None, output: ManuallyDrop::new(None), retirement: Default::default(), offset: 0, phase: 0, closing: false } }

    /// 📏️ Quotes source binding, fixed-buffer formatting, allocation, or the next bounded byte copy.
    pub fn demand(&self, source: RetainedParameterTextSource<'_>, body: usize, maximum_work_units: u64) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.phase > 4 || (self.source_kind != 0 && self.source_kind != source.kind()) { return Err(text_invalid()); }
        source.check(&self.binding)?;
        Ok(match self.phase {
            0 => text_scalar(source.binding_bytes() + 2 * size_of::<u8>()),
            1 => text_scalar(2 * size_of::<Option<RetainedParameterNumericText>>() + 2 * RetainedParameterNumericText::MAX_BYTES + size_of::<u8>()),
            2 => RetirementDemand { copy_bytes: 2 * size_of::<Option<String>>() + size_of::<u8>(), capacity_bytes: self.text.as_ref().ok_or_else(text_invalid)?.as_str().len(), depth: 1, ..Default::default() },
            3 => { let text = self.text.as_ref().ok_or_else(text_invalid)?.as_str(); let n = text.len().saturating_sub(self.offset).min(body).min(usize::try_from(maximum_work_units.saturating_sub(1)).unwrap_or(usize::MAX)); text_scalar(size_of::<usize>() + size_of::<u8>() + n) },
            4 => RetirementDemand::default(),
            _ => return Err(text_invalid()),
        })
    }

    /// ⛽️ The numeric primitive is bounded by binary32's fixed width and the 256-byte template buffer.
    pub fn work_demand(&self, source: RetainedParameterTextSource<'_>, body: usize, maximum_work_units: u64) -> Result<u64, ValueError> {
        let d = self.demand(source, body, maximum_work_units)?;
        if maximum_work_units == 0 || self.phase == 4 { return Ok(0); }
        let work = match self.phase { 1 => 1 + 2 * RetainedParameterNumericText::MAX_BYTES as u64, 3 => 1 + d.copy_bytes.saturating_sub(size_of::<usize>() + size_of::<u8>()) as u64, _ => 1 };
        Ok(if work <= maximum_work_units { work } else { 0 })
    }

    /// ▶️ Performs only the admitted source binding, fixed scalar format, reservation, or byte segment.
    pub fn advance(&mut self, source: RetainedParameterTextSource<'_>, body: usize, maximum_work_units: u64, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementOutcome {
        let grant = effect.grant();
        let base = effect.performed_work();
        let mut work: u128 = 0;
        let result: Result<_, OriginalRetirementFailure> = (|| {
            let selected = self.work_demand(source, body, maximum_work_units)?;
            if self.phase == 4 { return Ok(RetainedCloneStep::Complete(Default::default())); }
            if selected == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let d = self.demand(source, body, maximum_work_units)?;
            if !protocol::retained_mutation_demand_fits(d, grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, ..Default::default() };
            work = 1;
            match self.phase {
                0 => { let turn = source.bind(&mut self.binding, &mut effect.nested(RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - 2 * size_of::<u8>(), ..grant })); work = turn.work; progress = turn.result?.ok_or_else(text_invalid)?; progress.copied_bytes += 2 * size_of::<u8>(); self.source_kind = source.kind(); self.phase = 1; },
                1 => { let text = match source.format() { Ok(text) => text, Err(_) => { work = u128::from(selected); return Err(text_invalid().with_retained_progress(progress).into()); } }; work = 1 + RetainedParameterNumericText::MAX_BYTES as u128 + text.as_str().len() as u128; self.text = Some(text); self.phase = 2; },
                2 => { let mut value = String::new(); value.try_reserve_exact(d.capacity_bytes).map_err(|_| ValueError::literal(ValueRefusalKind::OwnershipLimit, "parameter text reservation failed"))?; progress.retained_capacity_bytes = value.capacity(); *self.output = Some(value); self.phase = 3; },
                3 => { let text = self.text.as_ref().ok_or_else(text_invalid)?.as_str(); let n = d.copy_bytes - size_of::<usize>() - size_of::<u8>(); if n == 0 && self.offset < text.len() { work = 0; return Ok(RetainedCloneStep::Progress(Default::default())); } let end = self.offset + n; self.output.as_mut().ok_or_else(text_invalid)?.push_str(&text[self.offset..end]); self.offset = end; work += n as u128; if self.offset == text.len() { self.phase = 4; } },
                _ => return Err(text_invalid().into()),
            }
            if !progress.fits(grant) { return Err(text_invalid().with_retained_progress(progress).into()); }
            Ok(if self.phase == 4 { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
        })();
        protocol::retained_settle(effect, base, result, work)
    }

    /// 📤️ Transfers the completed original string only under its own physical grant.
    pub fn take(&mut self, grant: RetainedCloneGrant) -> RetainedWorkOutcome<Option<(String, RetainedCloneProgress)>, ValueError> {
        if self.phase != 4 || self.closing { return RetainedWorkOutcome { result: Err(text_invalid()), work: 0 }; }
        let d = self.take_demand();
        if !protocol::retained_mutation_demand_fits(d, grant) { return RetainedWorkOutcome { result: Ok(None), work: 0 }; }
        self.phase = 5;
        RetainedWorkOutcome { result: self.output.take().ok_or_else(text_invalid).map(|value| Some((value, RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, ..Default::default() }))), work: 1 }
    }
    /// 📏️ Quotes the owned string handoff.
    pub fn take_demand(&self) -> RetirementDemand { text_scalar(2 * size_of::<Option<String>>() + size_of::<u8>()) }

    /// 🛑️ Begins retirement of partial text and the original source binding.
    pub fn begin_close(&mut self) { self.closing = true; }
    /// 📐️ Quotes the next retained child or binding retirement.
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.closing { return Err(text_invalid()); }
        if !self.retirement.is_empty() || self.output.is_some() { return self.retirement.next_owner_demands::<String>(self.output.is_some(), &None, body); }
        if self.text.is_some() { return Ok(text_scalar(2 * size_of::<Option<RetainedParameterNumericText>>())); }
        RetainedCloneBinding::close_demand(&self.binding, body)
    }
    /// ⛽️ Quotes the selected child or binding retirement work.
    pub fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        self.close_demand(body)?;
        if !self.retirement.is_empty() { return self.retirement.next_owner_work::<String>(false, &None, body); }
        if self.output.is_some() { return self.retirement.next_owner_work::<String>(true, &None, body); }
        if self.text.is_some() { return Ok(1); }
        RetainedCloneBinding::close_work_demand(&self.binding, body)
    }
    /// ♻️ Retires one retained child or source alias.
    pub fn close_step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let grant = effect.grant();
        let base = effect.performed_work();
        let mut work: u128 = 0;
        let mut pending = None;
        let result: Result<_, OriginalRetirementFailure> = (|| {
            let d = self.close_demand(body)?;
            if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            if !protocol::retained_mutation_demand_fits(d, grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            if !self.retirement.is_empty() {
                return match self.retirement.step_granted(body, effect) {
                    RetainedWorkPoll::Pending { work: owed } => { pending = Some(owed); Ok(RetainedCloneStep::Progress(Default::default())) }
                    RetainedWorkPoll::Complete(turn) => { work = turn.work; Ok(RetainedCloneStep::Progress(turn.result?.progress())) }
                };
            }
            let turn = self.retirement.begin_granted(&mut self.output, effect); work += turn.work; if let Some(step) = turn.result? { return Ok(RetainedCloneStep::Progress(step.progress())); }
            if self.text.is_some() { self.text = None; work += 1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, ..Default::default() })); }
            match RetainedCloneBinding::close_one(&mut self.binding, body, effect) {
                RetainedWorkPoll::Pending { work: owed } => { pending = Some(owed); Ok(RetainedCloneStep::Progress(Default::default())) }
                RetainedWorkPoll::Complete(turn) => { work += turn.work; let step = turn.result?; Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) }) }
            }
        })();
        match pending {
            Some(owed) => RetainedWorkPoll::Pending { work: owed },
            None => RetainedWorkPoll::Complete(protocol::retained_settle(effect, base, result, work)),
        }
    }
    /// 🫙️ Reports whether all original custody has been retired.
    pub fn terminal_is_empty(&self) -> bool { self.binding.is_none() && self.text.is_none() && self.output.is_none() && self.retirement.is_empty() }
}

impl Drop for RetainedParameterTextOwner {
    fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(empty || std::thread::panicking(), "parameter text owner requires funded closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.output); } } }
}

fn diagnostic_nested(mut d: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    d.copy_bytes = d.copy_bytes.checked_add(size_of::<u8>()).ok_or_else(text_invalid)?;
    d.depth = d.depth.checked_add(1).ok_or_else(text_invalid)?;
    Ok(d)
}
fn diagnostic_child(g: RetainedCloneGrant) -> RetainedCloneGrant { RetainedCloneGrant { maximum_copy_bytes: g.maximum_copy_bytes - size_of::<u8>(), maximum_depth: g.maximum_depth - 1, ..g } }
fn diagnostic_progress(mut p: RetainedCloneProgress) -> Result<RetainedCloneProgress, ValueError> { p.copied_bytes = p.copied_bytes.checked_add(size_of::<u8>()).ok_or_else(text_invalid)?; Ok(p) }

/// 📨️ Owns one fatal numeric diagnostic and its original source through publication or cancellation.
pub struct RetainedParameterDiagnosticOwner {
    text: RetainedParameterTextOwner,
    code: ManuallyDrop<Option<String>>,
    message: ManuallyDrop<Option<String>>,
    rows: ManuallyDrop<Option<Vec<protocol::MutationMessage>>>,
    ledger: Option<protocol::MutationMessageLedgerRetirement>,
    retirement: RetainedCloneClose,
    offset: usize,
    phase: u8,
    closing: bool,
}

impl RetainedParameterDiagnosticOwner {
    /// 🌱️ Creates an empty diagnostic owner without observing the source.
    pub fn new() -> Self { Self { text: RetainedParameterTextOwner::new(), code: ManuallyDrop::new(None), message: ManuallyDrop::new(None), rows: ManuallyDrop::new(None), ledger: None, retirement: Default::default(), offset: 0, phase: 0, closing: false } }

    fn check(&self, source: RetainedParameterTextSource<'_>) -> Result<(), ValueError> {
        if self.closing || self.phase > 6 || (self.text.source_kind != 0 && self.text.source_kind != source.kind()) { return Err(text_invalid()); }
        source.check(&self.text.binding)?;
        Ok(())
    }

    /// 📏️ Quotes one original text child, code allocation, code segment, row allocation or row assembly.
    pub fn demand(&self, source: RetainedParameterTextSource<'_>, body: usize, maximum_work_units: u64) -> Result<RetirementDemand, ValueError> {
        self.check(source)?;
        Ok(match self.phase {
            0 => return diagnostic_nested(self.text.demand(source, body, maximum_work_units.saturating_sub(1))?),
            1 => { let mut d = self.text.take_demand(); d.copy_bytes += 2 * size_of::<Option<String>>() + size_of::<u8>(); d },
            2 => RetirementDemand { copy_bytes: 2 * size_of::<Option<String>>() + size_of::<u8>(), capacity_bytes: "mutation.invariant".len(), depth: 1, ..Default::default() },
            3 => { let n = "mutation.invariant".len().saturating_sub(self.offset).min(body).min(usize::try_from(maximum_work_units.saturating_sub(1)).unwrap_or(usize::MAX)); text_scalar(size_of::<usize>() + size_of::<u8>() + n) },
            4 => RetirementDemand { copy_bytes: 2 * size_of::<Option<Vec<protocol::MutationMessage>>>() + size_of::<u8>(), capacity_bytes: size_of::<protocol::MutationMessage>(), depth: 1, ..Default::default() },
            5 => text_scalar(2 * size_of::<protocol::MutationMessage>() + 2 * size_of::<Option<protocol::MutationMessageLedgerRetirement>>() + 2 * size_of::<Option<String>>() + 2 * size_of::<Option<Vec<protocol::MutationMessage>>>() + size_of::<u8>()),
            6 => RetirementDemand::default(),
            _ => return Err(text_invalid()),
        })
    }

    /// ⛽️ Quotes the actual selected child or bounded local operation.
    pub fn work_demand(&self, source: RetainedParameterTextSource<'_>, body: usize, maximum_work_units: u64) -> Result<u64, ValueError> {
        let d = self.demand(source, body, maximum_work_units)?;
        if maximum_work_units == 0 || self.phase == 6 { return Ok(0); }
        let work = match self.phase {
            0 => { let child = self.text.work_demand(source, body, maximum_work_units.saturating_sub(1))?; if child == 0 { return Ok(0); } child.checked_add(1).ok_or_else(text_invalid)? },
            3 => 1 + d.copy_bytes.saturating_sub(size_of::<usize>() + size_of::<u8>()) as u64,
            5 => 3,
            _ => 1,
        };
        Ok(if work <= maximum_work_units { work } else { 0 })
    }

    /// ▶️ Constructs one retained component without evaluating a cold mutation.
    pub fn advance(&mut self, source: RetainedParameterTextSource<'_>, body: usize, maximum_work_units: u64, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementOutcome {
        let grant = effect.grant();
        let base = effect.performed_work();
        let mut work: u128 = 0;
        let result: Result<_, OriginalRetirementFailure> = (|| {
            let selected = self.work_demand(source, body, maximum_work_units)?;
            if self.phase == 6 { return Ok(RetainedCloneStep::Complete(Default::default())); }
            if selected == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let d = self.demand(source, body, maximum_work_units)?;
            if !protocol::retained_mutation_demand_fits(d, grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, ..Default::default() };
            work = 1;
            match self.phase {
                0 => { let turn = self.text.advance(source, body, maximum_work_units - 1, &mut effect.nested(diagnostic_child(grant))); work += turn.work; let step = turn.result.map_err(|error| { let p = diagnostic_progress(error.retained_progress()).unwrap_or_else(|_| error.retained_progress()); error.with_retained_progress(p) })?; progress = diagnostic_progress(step.progress())?; if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 1; } },
                1 => { let extra = 2 * size_of::<Option<String>>() + size_of::<u8>(); let turn = self.text.take(RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - extra, ..grant }); work = turn.work; let (message, p) = turn.result?.ok_or_else(text_invalid)?; *self.message = Some(message); progress = p; progress.copied_bytes += extra; self.phase = 2; },
                2 => { let mut code = String::new(); code.try_reserve_exact(d.capacity_bytes).map_err(|_| ValueError::literal(ValueRefusalKind::OwnershipLimit, "parameter diagnostic code reservation failed"))?; progress.retained_capacity_bytes = code.capacity(); *self.code = Some(code); self.phase = 3; },
                3 => { let code = "mutation.invariant"; let n = d.copy_bytes - size_of::<usize>() - size_of::<u8>(); if n == 0 && self.offset < code.len() { work = 0; return Ok(RetainedCloneStep::Progress(Default::default())); } let end = self.offset + n; self.code.as_mut().ok_or_else(text_invalid)?.push_str(&code[self.offset..end]); self.offset = end; work += n as u128; if end == code.len() { self.phase = 4; } },
                4 => { let mut rows = Vec::new(); rows.try_reserve_exact(1).map_err(|_| ValueError::literal(ValueRefusalKind::OwnershipLimit, "parameter diagnostic row reservation failed"))?; progress.retained_capacity_bytes = rows.capacity() * size_of::<protocol::MutationMessage>(); *self.rows = Some(rows); self.phase = 5; },
                5 => { if self.code.is_none() || self.message.is_none() || self.rows.is_none() { return Err(text_invalid().into()); } let row = protocol::MutationMessage { level: semio_framework_diagnostic::Severity::Fatal, code: semio_framework_diagnostic::FaultCode(self.code.take().expect("validated diagnostic code")), message: self.message.take().expect("validated diagnostic text"), target: Vec::new(), op_index: None }; self.rows.as_mut().expect("validated diagnostic vector").push(row); self.ledger = Some(protocol::MutationMessageLedgerRetirement::new(String::new(), self.rows.take().expect("validated diagnostic vector"))); self.phase = 6; work = 3; },
                _ => return Err(text_invalid().into()),
            }
            if !progress.fits(grant) { return Err(text_invalid().with_retained_progress(progress).into()); }
            Ok(if self.phase == 6 { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
        })();
        protocol::retained_settle(effect, base, result, work)
    }

    /// 📏️ Quotes the diagnostic vector handoff.
    pub fn take_demand(&self) -> RetirementDemand { text_scalar(size_of::<Vec<protocol::MutationMessage>>() + size_of::<u8>()) }
    /// 📤️ Transfers the original diagnostic row once under its grant.
    pub fn take(&mut self, grant: RetainedCloneGrant) -> RetainedWorkOutcome<Option<(Vec<protocol::MutationMessage>, RetainedCloneProgress)>, ValueError> {
        if self.phase != 6 || self.closing { return RetainedWorkOutcome { result: Err(text_invalid()), work: 0 }; }
        let d = self.take_demand();
        if !protocol::retained_mutation_demand_fits(d, grant) { return RetainedWorkOutcome { result: Ok(None), work: 0 }; }
        let result = self.ledger.as_mut().ok_or_else(text_invalid).and_then(|ledger| ledger.take_messages_controlled(RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - size_of::<u8>(), ..grant })).and_then(|value| value.ok_or_else(text_invalid)).map(|(rows, mut progress)| { progress.copied_bytes += size_of::<u8>(); self.phase = 7; Some((rows, progress)) });
        RetainedWorkOutcome { work: u128::from(result.is_ok()), result }
    }

    /// 🛑️ Begins cleanup of every partially prepared component and original binding.
    pub fn begin_close(&mut self) { self.closing = true; self.text.begin_close(); }
    /// 📐️ Quotes the next original child, owned string, vector or ledger retirement.
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.closing { return Err(text_invalid()); }
        if !self.text.terminal_is_empty() { return diagnostic_nested(self.text.close_demand(body)?); }
        if !self.retirement.is_empty() { return self.retirement.next_owner_demands::<String>(false, &None, body); }
        if self.code.is_some() || self.message.is_some() { return self.retirement.next_owner_demands::<String>(true, &None, body); }
        if self.rows.is_some() { return Ok(text_scalar(2 * size_of::<Option<Vec<protocol::MutationMessage>>>() + 2 * size_of::<Option<protocol::MutationMessageLedgerRetirement>>())); }
        if let Some(ledger) = self.ledger.as_ref() {
            if ledger.terminal_is_empty() { return Ok(text_scalar(2 * size_of::<Option<protocol::MutationMessageLedgerRetirement>>())); }
            return Ok(RetirementDemand { copy_bytes: ledger.next_copy_byte_demand(), capacity_bytes: ledger.next_capacity_byte_demand(body)?, release_bytes: ledger.next_release_byte_demand()?, depth: ledger.next_depth_demand()? });
        }
        Ok(RetirementDemand::default())
    }
    /// ⛽️ Quotes one selected close turn.
    pub fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        self.close_demand(body)?;
        if !self.text.terminal_is_empty() { return self.text.close_work_demand(body)?.checked_add(1).ok_or_else(text_invalid); }
        if !self.retirement.is_empty() { return self.retirement.next_owner_work::<String>(false, &None, body); }
        if self.code.is_some() || self.message.is_some() { return self.retirement.next_owner_work::<String>(true, &None, body); }
        if self.rows.is_some() { return Ok(1); }
        if let Some(ledger) = self.ledger.as_ref() { return if ledger.terminal_is_empty() { Ok(1) } else { ledger.next_computational_work(body) }; }
        Ok(0)
    }
    /// ♻️ Retires one actual original component or the bounded diagnostic ledger.
    pub fn close_step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let grant = effect.grant();
        let base = effect.performed_work();
        let mut work: u128 = 0;
        let mut pending = None;
        let result: Result<_, OriginalRetirementFailure> = (|| {
            let d = self.close_demand(body)?;
            if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            if !protocol::retained_mutation_demand_fits(d, grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            if !self.text.terminal_is_empty() {
                return match self.text.close_step(body.saturating_sub(size_of::<u8>()), &mut effect.nested(diagnostic_child(grant))) {
                    RetainedWorkPoll::Pending { work: owed } => { pending = Some(owed); Ok(RetainedCloneStep::Progress(Default::default())) }
                    RetainedWorkPoll::Complete(turn) => { work = 1 + turn.work; let step = turn.result.map_err(|error| { let p = diagnostic_progress(error.retained_progress()).unwrap_or_else(|_| error.retained_progress()); error.with_retained_progress(p) })?; Ok(RetainedCloneStep::Progress(diagnostic_progress(step.progress())?)) }
                };
            }
            if !self.retirement.is_empty() {
                return match self.retirement.step_granted(body, effect) {
                    RetainedWorkPoll::Pending { work: owed } => { pending = Some(owed); Ok(RetainedCloneStep::Progress(Default::default())) }
                    RetainedWorkPoll::Complete(turn) => { work = turn.work; Ok(RetainedCloneStep::Progress(turn.result?.progress())) }
                };
            }
            for slot in [&mut self.code, &mut self.message] { let turn = self.retirement.begin_granted(slot, effect); work += turn.work; if let Some(step) = turn.result? { return Ok(RetainedCloneStep::Progress(step.progress())); } }
            if let Some(rows) = self.rows.take() { self.ledger = Some(protocol::MutationMessageLedgerRetirement::new(String::new(), rows)); work = 1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, ..Default::default() })); }
            if let Some(ledger) = self.ledger.as_mut() { if ledger.terminal_is_empty() { self.ledger = None; work = 1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, ..Default::default() })); } let step = ledger.close_step(grant); work = u128::from(ledger.last_turn_work()); return Ok(RetainedCloneStep::Progress(step?.progress())); }
            Ok(RetainedCloneStep::Complete(Default::default()))
        })();
        match pending {
            Some(owed) => RetainedWorkPoll::Pending { work: owed },
            None => RetainedWorkPoll::Complete(protocol::retained_settle(effect, base, result, work)),
        }
    }
    /// 🫙️ Confirms no child or diagnostic custody remains.
    pub fn terminal_is_empty(&self) -> bool { self.text.terminal_is_empty() && self.code.is_none() && self.message.is_none() && self.rows.is_none() && self.ledger.is_none() && self.retirement.is_empty() }
}

impl Drop for RetainedParameterDiagnosticOwner {
    fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(empty || std::thread::panicking(), "parameter diagnostic owner requires funded closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.code); ManuallyDrop::drop(&mut self.message); ManuallyDrop::drop(&mut self.rows); } } }
}

#[cfg(test)]
mod numeric_text_tests {
    use super::{RetainedParameterDiagnosticOwner, RetainedParameterNumericText, RetainedParameterTextOwner, RetainedParameterTextSource};
    use semio_framework_value::{RetirementDemand, retained_clone::{RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep}};

    fn grant(d: RetirementDemand) -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: d.copy_bytes, maximum_capacity_bytes: d.capacity_bytes, maximum_release_bytes: d.release_bytes, maximum_depth: d.depth }
    }

    fn admitted<T: semio_framework_value::retirement::RetireOwned + Sync + 'static>(value: T) -> RetainedCloneSource<T> {
        let mut admission = RetainedCloneSource::admission_owned(value, ());
        let mut output = None;
        let g = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 4096 };
        for _ in 0..64 {
            if admission.terminal_is_empty() { break; }
            if admission.phase() == "ready" { admission.take_output(&mut output, g).result.unwrap(); } else { let mut actual = None; let turn = admission.advance(g, &mut actual); let progress = turn.result.unwrap().progress(); if let Some(bytes) = actual { assert_eq!(bytes, progress.retained_capacity_bytes); } }
        }
        output.expect("original source admission")
    }

    fn source() -> RetainedCloneSource<semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::IngestParams> {
        let mut value = semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::IngestParams::default();
        value.frame_sample_stride = 0;
        admitted(value)
    }

    fn close(owner: &mut RetainedParameterTextOwner) {
        owner.begin_close();
        for _ in 0..1000 {
            if owner.terminal_is_empty() { return; }
            let d = owner.close_demand(16).unwrap();
            let g = grant(d);
            let (denied, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(RetainedCloneGrant { maximum_items: 0, ..g }));
            assert_eq!((born, released), (0, 0));
            assert_eq!(denied.work, 0);
            let work = owner.close_work_demand(16).unwrap();
            let (turn, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(g));
            assert!(turn.work <= work);
            let progress = turn.result.unwrap().progress();
            assert!(progress.fits(g));
            assert_eq!((born, released), (progress.retained_capacity_bytes, progress.released_bytes));
        }
        panic!("text owner did not close");
    }

    #[test]
    fn funded_numeric_text_denial_and_each_cancelled_frontier_close() {
        for cancel in 0..40 {
            let mut original = source();
            let mut owner = RetainedParameterTextOwner::new();
            for _ in 0..cancel {
                let input = RetainedParameterTextSource::Ingest(original.borrow());
                let d = owner.demand(input, 16, 4096).unwrap();
                let g = grant(d);
                let denied = owner.advance(input, 16, 0, g);
                assert_eq!(denied.work, 0);
                let (denied, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.advance(input, 16, 4096, RetainedCloneGrant { maximum_items: 0, ..g }));
                assert_eq!((born, released), (0, 0));
                assert_eq!(denied.work, 0);
                let (turn, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.advance(input, 16, 4096, g));
                assert!(turn.work <= 4096);
                let step = turn.result.unwrap();
                assert_eq!((born, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                if matches!(step, RetainedCloneStep::Complete(_)) { break; }
            }
            close(&mut owner);
            while !original.terminal_is_empty() {
                let d = original.next_close_demand(4096).unwrap();
                original.close_step(grant(d)).result.unwrap();
            }
        }
    }

    #[test]
    fn funded_numeric_text_transfers_exact_original_message_and_rejects_source_change() {
        let mut original = source();
        let mut other = source();
        let mut owner = RetainedParameterTextOwner::new();
        let input = RetainedParameterTextSource::Ingest(original.borrow());
        let first = grant(owner.demand(input, 8, 4096).unwrap());
        owner.advance(input, 8, 4096, first).result.unwrap();
        assert!(owner.demand(RetainedParameterTextSource::Ingest(other.borrow()), 8, 4096).is_err());
        let format = grant(owner.demand(input, 8, 4096).unwrap());
        assert_eq!(owner.work_demand(input, 8, 512).unwrap(), 0);
        assert_eq!(owner.advance(input, 8, 512, format).work, 0);
        assert_eq!(owner.advance(input, 8, 4096, RetainedCloneGrant { maximum_copy_bytes: format.maximum_copy_bytes - 1, ..format }).work, 0);
        for _ in 0..100 {
            let d = owner.demand(input, 8, 4096).unwrap();
            let expected_work = owner.work_demand(input, 8, 4096).unwrap();
            let g = grant(d);
            let turn = owner.advance(input, 8, 4096, g);
            assert!(turn.work <= expected_work);
            let step = turn.result.unwrap();
            assert!(step.progress().fits(g));
            if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        }
        let g = grant(owner.take_demand());
        assert_eq!(owner.take(RetainedCloneGrant { maximum_items: 0, ..g }).work, 0);
        let (message, receipt) = owner.take(g).result.unwrap().unwrap();
        assert!(receipt.fits(g));
        assert_eq!(message, "Ingest params need a finite non-negative min sharpness and positive max frames/frame sample stride (got min_sharpness=0.3, max_frames=200, frame_sample_stride=0).");
        let mut output = std::mem::ManuallyDrop::new(Some(message));
        let mut retirement = semio_framework_value::retained_clone::RetainedCloneClose::default();
        while output.is_some() || !retirement.is_empty() {
            let d = retirement.next_owner_demands::<String>(output.is_some(), &None, 4096).unwrap();
            let g = grant(d);
            let turn = if retirement.is_empty() { retirement.begin_granted(&mut output, g).result.unwrap().unwrap() } else { retirement.step_granted(g).result.unwrap() };
            assert!(turn.progress().fits(g));
        }
        unsafe { std::mem::ManuallyDrop::drop(&mut output); }
        close(&mut owner);
        for source in [&mut original, &mut other] {
            while !source.terminal_is_empty() { source.close_step(grant(source.next_close_demand(4096).unwrap())).result.unwrap(); }
        }
    }

    #[test]
    fn funded_numeric_diagnostic_closes_each_partial_frontier_and_transfers_one_row() {
        for cancel in 0..90 {
            let mut original = source();
            let mut owner = RetainedParameterDiagnosticOwner::new();
            let input = RetainedParameterTextSource::Ingest(original.borrow());
            let mut ready = false;
            for _ in 0..cancel {
                let d = owner.demand(input, 8, 4096).unwrap();
                let g = grant(d);
                let (denied, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.advance(input, 8, 4096, RetainedCloneGrant { maximum_items: 0, ..g }));
                assert_eq!((born, released), (0, 0));
                assert_eq!(denied.work, 0);
                let work = owner.work_demand(input, 8, 4096).unwrap();
                let (turn, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.advance(input, 8, 4096, g));
                assert!(turn.work <= work);
                let step = turn.result.unwrap();
                assert!(step.progress().fits(g));
                assert_eq!((born, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                if matches!(step, RetainedCloneStep::Complete(_)) { ready = true; break; }
            }
            if ready {
                let g = grant(owner.take_demand());
                assert_eq!(owner.take(RetainedCloneGrant { maximum_items: 0, ..g }).work, 0);
                let (rows, progress) = owner.take(g).result.unwrap().unwrap();
                assert!(progress.fits(g));
                assert_eq!(rows.len(), 1);
                assert_eq!(rows[0].level, semio_framework_diagnostic::Severity::Fatal);
                assert_eq!(rows[0].code.0, "mutation.invariant");
                assert_eq!(rows[0].message, "Ingest params need a finite non-negative min sharpness and positive max frames/frame sample stride (got min_sharpness=0.3, max_frames=200, frame_sample_stride=0).");
                assert!(rows[0].target.is_empty());
                let mut ledger = protocol::MutationMessageLedgerRetirement::new(String::new(), rows);
                while !ledger.terminal_is_empty() {
                    let d = RetirementDemand { copy_bytes: ledger.next_copy_byte_demand(), capacity_bytes: ledger.next_capacity_byte_demand(4096).unwrap(), release_bytes: ledger.next_release_byte_demand().unwrap(), depth: ledger.next_depth_demand().unwrap() };
                    let g = grant(d);
                    assert!(ledger.close_step(g).unwrap().progress().fits(g));
                }
            }
            owner.begin_close();
            for _ in 0..1000 {
                if owner.terminal_is_empty() { break; }
                let d = owner.close_demand(8).unwrap();
                let g = grant(d);
                let (denied, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(RetainedCloneGrant { maximum_items: 0, ..g }));
                assert_eq!((born, released), (0, 0));
                assert_eq!(denied.work, 0);
                let work = owner.close_work_demand(8).unwrap();
                let (turn, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(g));
                assert!(turn.work <= work);
                let progress = turn.result.unwrap().progress();
                assert!(progress.fits(g));
                assert_eq!((born, released), (progress.retained_capacity_bytes, progress.released_bytes));
            }
            assert!(owner.terminal_is_empty());
            while !original.terminal_is_empty() { original.close_step(grant(original.next_close_demand(4096).unwrap())).result.unwrap(); }
        }
    }

    #[test]
    fn funded_numeric_diagnostic_matches_all_neutral_messages() {
        fn float(value: &serde_json::Value) -> f32 {
            match value.as_str() { Some("NaN") => f32::NAN, Some("inf") => f32::INFINITY, Some("-inf") => f32::NEG_INFINITY, Some(other) => panic!("unknown float {other}"), None => value.as_f64().unwrap() as f32 }
        }
        fn check(input: RetainedParameterTextSource<'_>, expected: &str) {
            let mut owner = RetainedParameterDiagnosticOwner::new();
            for _ in 0..100 {
                let d = owner.demand(input, 8, 4096).unwrap();
                let turn = owner.advance(input, 8, 4096, grant(d));
                if matches!(turn.result.unwrap(), RetainedCloneStep::Complete(_)) { break; }
            }
            let (rows, _) = owner.take(grant(owner.take_demand())).result.unwrap().unwrap();
            assert_eq!(rows[0].message, expected);
            let mut ledger = protocol::MutationMessageLedgerRetirement::new(String::new(), rows);
            while !ledger.terminal_is_empty() {
                let d = RetirementDemand { copy_bytes: ledger.next_copy_byte_demand(), capacity_bytes: ledger.next_capacity_byte_demand(4096).unwrap(), release_bytes: ledger.next_release_byte_demand().unwrap(), depth: ledger.next_depth_demand().unwrap() };
                ledger.close_step(grant(d)).unwrap();
            }
            owner.begin_close();
            while !owner.terminal_is_empty() { owner.close_step(grant(owner.close_demand(4096).unwrap())).result.unwrap(); }
        }
        fn close_original<T: semio_framework_value::retirement::RetireOwned + Sync + 'static>(source: &mut RetainedCloneSource<T>) {
            while !source.terminal_is_empty() { source.close_step(grant(source.next_close_demand(4096).unwrap())).result.unwrap(); }
        }
        let cases: serde_json::Value = serde_json::from_str(include_str!("../🧪️tests/🧫️retained-params/💬️.json")).unwrap();
        for case in cases.as_array().unwrap() {
            let expected = case["message"].as_str().unwrap();
            match case["facet"].as_str().unwrap() {
                "ingest" => { let mut p = super::IngestParams::default(); p.min_sharpness = float(&case["a"]); p.max_frames = case["b"].as_u64().unwrap() as u32; p.frame_sample_stride = case["c"].as_u64().unwrap() as u32; let mut source = admitted(p); check(RetainedParameterTextSource::Ingest(source.borrow()), expected); close_original(&mut source); },
                "feature" => { let mut p = super::FeatureParams::default(); p.target_count = case["a"].as_u64().unwrap() as u32; p.edge_threshold = float(&case["b"]); let mut source = admitted(p); check(RetainedParameterTextSource::Feature(source.borrow()), expected); close_original(&mut source); },
                "matching" => { let mut p = super::MatchParams::default(); p.ratio_test = float(&case["a"]); let mut source = admitted(p); check(RetainedParameterTextSource::Matching(source.borrow()), expected); close_original(&mut source); },
                other => panic!("unknown facet {other}"),
            }
        }
    }

    #[test]
    fn fixed_numeric_text_preserves_native_display() {
        let text = RetainedParameterNumericText::ingest(0.5, u32::MAX, 0).unwrap();
        assert_eq!(text.as_str(), "Ingest params need a finite non-negative min sharpness and positive max frames/frame sample stride (got min_sharpness=0.5, max_frames=4294967295, frame_sample_stride=0).");
        assert_eq!(RetainedParameterNumericText::matching(f32::NEG_INFINITY).unwrap().as_str(), "Match ratio test -inf must be finite and within (0, 1].");
        assert_eq!(RetainedParameterNumericText::feature(4000, f32::INFINITY).unwrap().as_str(), "Feature params need a positive target count and a finite non-negative edge threshold (got target_count=4000, edge_threshold=inf).");
        assert!(RetainedParameterNumericText::ingest(f32::MAX, u32::MAX, u32::MAX).is_ok());
        assert!(RetainedParameterNumericText::ingest(f32::MIN, u32::MAX, u32::MAX).is_ok());
        assert!(RetainedParameterNumericText::ingest(f32::MIN_POSITIVE, u32::MAX, u32::MAX).is_ok());
        assert!(RetainedParameterNumericText::ingest(f32::from_bits(1), u32::MAX, u32::MAX).is_ok());
        assert!(RetainedParameterNumericText::ingest(f32::from_bits(0x8000_0001), u32::MAX, u32::MAX).is_ok());
    }

    #[test]
    fn fixed_numeric_text_matches_neutral_messages() {
        fn float(value: &serde_json::Value) -> f32 {
            match value.as_str() { Some("NaN") => f32::NAN, Some("inf") => f32::INFINITY, Some("-inf") => f32::NEG_INFINITY, Some(other) => panic!("unknown float {other}"), None => value.as_f64().unwrap() as f32 }
        }
        let cases: serde_json::Value = serde_json::from_str(include_str!("../🧪️tests/🧫️retained-params/💬️.json")).unwrap();
        for case in cases.as_array().unwrap() {
            let text = match case["facet"].as_str().unwrap() {
                "ingest" => RetainedParameterNumericText::ingest(float(&case["a"]), case["b"].as_u64().unwrap() as u32, case["c"].as_u64().unwrap() as u32),
                "feature" => RetainedParameterNumericText::feature(case["a"].as_u64().unwrap() as u32, float(&case["b"])),
                "matching" => RetainedParameterNumericText::matching(float(&case["a"])),
                other => panic!("unknown facet {other}"),
            }.unwrap();
            assert_eq!(text.as_str(), case["message"].as_str().unwrap(), "{case}");
        }
    }

    #[test]
    fn fixed_numeric_text_refuses_overflow_without_partial_write() {
        use std::fmt::Write;
        let mut text = RetainedParameterNumericText::empty();
        text.write_str("prefix").unwrap();
        assert!(text.write_str(&"x".repeat(256)).is_err());
        assert_eq!(text.as_str(), "prefix");
    }
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod update_feature_params {
                            #[path = "../🌠update-feature-params/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🌠update-feature-params/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🌠update-feature-params/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod update_sfm_params {
                            #[path = "../🧮update-sfm-params/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🧮update-sfm-params/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🧮update-sfm-params/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod update_mesh_params {
                            #[path = "../🕸️update-mesh-params/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🕸️update-mesh-params/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🕸️update-mesh-params/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod update_geo_params {
                            #[path = "../🌐update-geo-params/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🌐update-geo-params/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🌐update-geo-params/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}

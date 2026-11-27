use super::{classic_graph_quote::{GraphLoan, GraphQuote, QUOTE_WORK}, classic_graph_writer::{AppendFrame, ClassicGraphWriter},
    increment::{Compact, IncrementPlan, IncrementRoom, IncrementRows, OriginalBase, PAGE_WORK}};

use std::io::Write;
use semio_framework_value::{ByteReadSource, RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_plugin::{MediaConstruction, __job::{OriginalConstructionDemand, OriginalTurnFailure, StepContext}};
use crate::standards::v1_7::subsets::base::{
    io::import::original_custody::{self, Appender, RetainedOriginal},
    modules::{lexer::CosGrant, lift::ControlledLiftEvent},
    schema::snapshot::PdfSnapshot,
};
use super::{graph::PinnedPdfGraph, exact_graph_proof::{COMPARE_WORK, ExactGraphProof, ExactGraphRefusal,
    ExactGraphTurn, ProjectionTrailer, projection_trailer, projection_trailer_extent},
    ordinary_graft::{GRAFT_WORK, OrdinaryGraft, OrdinaryRows}};
use super::producer::{PdfEmissionTurn, PdfNativeSaveProgram, PlanDecision, SaveCause, cause_of};

/// 🔁️ The reference reconciliation's quote must settle within this many funded turns, else the grant can never carry it.
const REFERENCE_TURNS: usize = 256;

/// ⏳️ Turns the reconciliation may wait for a turn funded with its whole quoted work before the grant is judged too small.
const REFERENCE_WAITS: usize = 4096;

/// 🛤️ How this Save spells the file: an appended section over the retained original, or the whole document written compactly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Route { Undecided, Increment, Compact }

pub struct ClassicPdfSaveProgram {
    original: Option<OriginalBase>,
    document: Option<String>,
    appended: Option<Appender>,
    written: usize,
    route: Route,
    increment: Option<IncrementPlan>,
    compact: Option<Compact>,
    entered: bool,
    graph: PinnedPdfGraph,
    maximum: usize,
    graft: OrdinaryGraft,
    graft_grant: Option<(usize, usize, usize)>,
    revision: Option<[u8; 32]>,
    quote: Option<GraphQuote>,
    control_trailer: Option<ProjectionTrailer>,
    control_birth: MediaConstruction,
    control_ready: bool,
    proof: ExactGraphProof,
    proof_grant: Option<(ControlledLiftEvent, usize)>,
    proof_ready: bool,
    scratch: Option<Vec<u8>>,
    scratch_birth: MediaConstruction,
    offsets: Option<Vec<u32>>,
    offsets_birth: MediaConstruction,
    prefix: usize,
    writer: Option<ClassicGraphWriter>,
    reference: Option<PinnedPdfGraph>,
    reference_birth: MediaConstruction,
    reference_wanted: bool,
    reference_turns: usize,
    reference_waits: usize,
}

impl ClassicPdfSaveProgram {
    /// 🗃️ Takes the exact current source after the caller has funded the reconciliation owner.
    pub fn new(graph: PinnedPdfGraph, maximum: usize) -> Self {
        Self { original: None, document: None, appended: None, written: 0, route: Route::Undecided, increment: None, compact: None, entered: false, graph, maximum, graft: Default::default(), graft_grant: None, revision: None, quote: None, scratch: None,
            control_trailer: None, control_birth: Default::default(), control_ready: false,
            proof: ExactGraphProof::new(), proof_grant: None, proof_ready: false,
            scratch_birth: Default::default(), offsets: None, offsets_birth: Default::default(), prefix: 0, writer: None,
            reference: None, reference_birth: Default::default(), reference_wanted: false, reference_turns: 0, reference_waits: 0 }
    }

    /// 📎️ Binds the retained original bytes so an imported document is saved by appending to them whenever its edits allow.
    pub fn with_original(mut self, original: OriginalBase) -> Self {
        self.original = Some(original);
        self
    }

    /// 🗃️ Appends to the file the local custody holds for `document` (its last saved file, else the one it was opened from) whenever the edits allow, and rebases the custody onto the saved file after a successful Save.
    pub fn with_document(mut self, document: String) -> Self {
        self.document = Some(document);
        self
    }

    /// ♻️ Makes the file this Save appended the base of the document's next Save; a no-op for every other route.
    pub fn commit(&mut self) {
        if self.route != Route::Increment { return; }
        let (Some(document), Some(plan), Some(appended)) = (self.document.as_deref(), self.increment.as_ref(), self.appended.take()) else { return };
        let Some(held) = plan.held() else { return };
        let mut file = held.file.clone();
        file.append(&appended.finish());
        original_custody::rebase(document, RetainedOriginal { file, objects: plan.measured(), touched: plan.replaced() });
    }

    /// 🧭️ Why the Save needs the whole-document reconciliation the grant cannot carry: the appended section's refusal, else the reconciliation's own memory.
    fn cause(&self) -> SaveCause { self.compact.map_or(SaveCause::Memory, cause_of) }

    /// 🪂️ The bounded ordinary route hands every edit it cannot spell to one whole-document reconciliation; a second refusal is final.
    fn refuse(&mut self) -> PlanDecision {
        if self.reference_wanted || self.route == Route::Increment { return PlanDecision::ReferenceRequired(self.cause()); }
        self.reference_wanted = true;
        self.quote = None;
        self.graft_grant = None;
        PlanDecision::Pending
    }

    /// ♻️ Transfers every retained child to the enclosing job's original close path.
    pub fn retire(self) -> ClassicPdfSaveRetirement {
        let (written, table) = self.writer.map_or((None, None), |writer| { let (buffer, table) = writer.into_parts(); (Some(buffer), Some(table)) });
        ClassicPdfSaveRetirement {
            graph: self.graph, pending_graft: self.graft, proof: self.proof, control_trailer: self.control_trailer,
            control_birth: self.control_birth, scratch_birth: self.scratch_birth,
            reference: self.reference, reference_birth: self.reference_birth,
            increment: self.increment.map(IncrementPlan::into_rows),
            offsets_birth: self.offsets_birth,
            offsets: table.or(self.offsets),
            scratch: written.or(self.scratch),
        }
    }
}

pub struct ClassicPdfSaveRetirement {
    pub graph: PinnedPdfGraph,
    pub pending_graft: OrdinaryGraft,
    pub proof: ExactGraphProof,
    pub control_trailer: Option<ProjectionTrailer>,
    pub control_birth: MediaConstruction,
    pub scratch_birth: MediaConstruction,
    pub reference: Option<PinnedPdfGraph>,
    pub reference_birth: MediaConstruction,
    pub increment: Option<IncrementRows>,
    pub offsets_birth: MediaConstruction,
    pub offsets: Option<Vec<u32>>,
    pub scratch: Option<Vec<u8>>,
}

impl PdfNativeSaveProgram<PdfSnapshot> for ClassicPdfSaveProgram {
    fn prepare_original(
        &mut self, snapshot: &PdfSnapshot, revision: [u8; 32], cx: &mut StepContext<'_>,
    ) -> Result<PlanDecision, OriginalTurnFailure> {
        cx.original_source_parent()?;
        self.graph.loan(snapshot, revision).map_err(|_| invalid("current PDF graph has another original revision"))?;
        if cx.should_yield() || cx.is_cancelled() { return Ok(PlanDecision::Pending); }
        if self.writer.is_some() { return Ok(PlanDecision::Ready); }
        if self.route == Route::Undecided {
            self.route = match (self.original.clone(), self.document.clone()) {
                (Some(original), _) if self.graph.needs_ordinary_graft() => { self.increment = Some(IncrementPlan::new(original)); Route::Increment }
                (None, Some(document)) if self.graph.needs_ordinary_graft() && !snapshot.objects.is_empty() => {
                    self.increment = Some(IncrementPlan::retained(document));
                    self.appended = Some(Appender::default());
                    Route::Increment
                }
                _ => Route::Compact,
            };
        }
        if self.route == Route::Increment && !self.entered {
            let base = self.graph.loan(snapshot, revision).map_err(|_| invalid("current PDF graph has another original revision"))?;
            let plan = self.increment.as_mut().ok_or_else(|| invalid("appended PDF section has no plan"))?;
            if !plan.ready() {
                let retained = cx.retained_grant();
                if cx.fuel_remaining() < PAGE_WORK as u64 || retained.maximum_items == 0 || retained.maximum_depth == 0 {
                    return Ok(PlanDecision::Pending);
                }
                cx.set_stage("pdf-save-increment");
                let room = IncrementRoom { copy: retained.maximum_copy_bytes, capacity: retained.maximum_capacity_bytes };
                match plan.step(base, snapshot, room) {
                    Ok(turn) => {
                        cx.consume_fuel((turn.work as u64).min(cx.fuel_remaining()).max(1));
                        if turn.capacity != 0 {
                            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: turn.copied,
                                retained_capacity_bytes: turn.capacity, ..Default::default() })?;
                        }
                    }
                    Err(error) => { self.compact = Some(error); self.route = Route::Compact }
                }
                return Ok(PlanDecision::Pending);
            }
            let info = plan.info();
            self.graph.enter_increment(info);
            self.entered = true;
            return Ok(PlanDecision::Pending);
        }
        if self.reference_wanted && self.reference.is_none() {
            let frame = size_of::<PinnedPdfGraph>();
            let ceiling = cx.original_turn_ceiling().0;
            let Some(capacity) = PinnedPdfGraph::reference_extent(snapshot).filter(|capacity|
                *capacity <= ceiling.maximum_capacity_bytes && capacity.saturating_add(frame) <= ceiling.maximum_copy_bytes)
                else { return Ok(PlanDecision::ReferenceRequired(self.cause())); };
            let work = (capacity / 16 + snapshot.objects.len()) as u64;
            if work > cx.original_turn_ceiling().1 { return Ok(PlanDecision::ReferenceRequired(self.cause())); }
            cx.set_stage("pdf-save-reconcile");
            if cx.fuel_remaining() < work {
                self.reference_waits += 1;
                return Ok(if self.reference_waits > REFERENCE_WAITS { PlanDecision::ReferenceRequired(self.cause()) } else { PlanDecision::Pending });
            }
            self.reference_turns += 1;
            if self.reference_turns > REFERENCE_TURNS { return Ok(PlanDecision::ReferenceRequired(self.cause())); }
            let ready = self.reference_birth.built(cx, &mut self.reference, OriginalConstructionDemand {
                items: 1, copy_bytes: frame + capacity, capacity_bytes: capacity, release_bytes: 0, depth: 1,
            }, || {
                let graph = PinnedPdfGraph::prepare_reference(snapshot, revision)
                    .map_err(|_| invalid("PDF reference reconciliation refused the current typed document"))?;
                let actual = graph.reference_capacity();
                if actual > capacity { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "PDF reference graph exceeds its quoted capacity")); }
                Ok((graph, RetainedCloneProgress { copied_items: 1, copied_bytes: frame + actual, retained_capacity_bytes: actual, ..Default::default() }))
            })?;
            if ready { cx.consume_fuel(work); }
            return Ok(PlanDecision::Pending);
        }
        let base = self.reference.as_ref().unwrap_or(&self.graph).loan(snapshot, revision)
            .map_err(|_| invalid("current PDF graph has another original revision"))?;
        let delta;
        let loan = if self.route == Route::Increment {
            let rows = self.increment.as_ref().ok_or_else(|| invalid("appended PDF section has no plan"))?.rows();
            delta = GraphLoan { version: base.version, objects: rows, order: None, trailer: self.graph.trailer(),
                overlay: Default::default(), resolver: Some(&base) };
            delta
        } else { base };
        if self.reference.is_none() && self.route != Route::Increment && self.graph.needs_ordinary_graft() {
            let retained = cx.retained_grant();
            if self.graft.ready() {
                let copied = size_of::<OrdinaryRows>();
                if retained.maximum_items == 0 || retained.maximum_copy_bytes < copied || retained.maximum_depth == 0 {
                    return Ok(PlanDecision::Pending);
                }
                let rows = std::mem::take(&mut self.graft).into_rows()
                    .map_err(|_| invalid("ordinary PDF graft missed its publication frontier"))?;
                self.graph.install_ordinary_graft(rows);
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, ..Default::default() })?;
                cx.consume_fuel(1);
                return Ok(PlanDecision::Pending);
            }
            if cx.fuel_remaining() < GRAFT_WORK as u64 || retained.maximum_items == 0 || retained.maximum_depth < 8 {
                return Ok(PlanDecision::Pending);
            }
            let grant = CosGrant { work: GRAFT_WORK, allocation_bytes: retained.maximum_capacity_bytes,
                release_bytes: retained.maximum_release_bytes };
            let step = match self.graft.step(loan, snapshot, grant, retained.maximum_copy_bytes) {
                Ok(step) => step,
                Err(_) => return Ok(self.refuse()),
            };
            cx.consume_fuel(step.work as u64);
            if step.allocated != 0 {
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: step.copied_bytes,
                    retained_capacity_bytes: step.allocated, ..Default::default() })?;
            }
            if let Some(bytes) = step.needs {
                let blocked = (bytes, retained.maximum_capacity_bytes, retained.maximum_copy_bytes);
                if self.graft_grant.replace(blocked) == Some(blocked) {
                    return Err(invalid("ordinary PDF graft exceeds its retained allocation grant").into());
                }
            } else {
                self.graft_grant = None;
            }
            return Ok(PlanDecision::Pending);
        }
        if self.quote.is_none() {
            if cx.fuel_remaining() < QUOTE_WORK as u64 { return Ok(PlanDecision::Pending); }
            let started = match (self.route, self.increment.as_ref().and_then(IncrementPlan::tail)) {
                (Route::Increment, Some(tail)) => GraphQuote::start_append(loan, self.maximum, tail.length),
                _ => GraphQuote::start(loan, self.maximum),
            };
            self.quote = match started {
                Ok(quote) => Some(quote),
                Err(_) => return Ok(self.refuse()),
            };
            cx.consume_fuel(QUOTE_WORK as u64);
            return Ok(PlanDecision::Pending);
        }
        let quote = self.quote.as_mut().expect("started PDF graph quote");
        if quote.maximum().is_none() {
            if cx.fuel_remaining() < QUOTE_WORK as u64 { return Ok(PlanDecision::Pending); }
            let planned = quote.next(loan, QUOTE_WORK);
            cx.consume_fuel(QUOTE_WORK as u64 + quote.take_extra() as u64);
            if planned.is_err() { return Ok(self.refuse()); }
            return Ok(PlanDecision::Pending);
        }

        if self.reference.is_some() || self.route == Route::Increment {
            self.control_ready = true;
            self.proof_ready = true;
        }
        if !self.control_ready {
            let Ok(capacity) = projection_trailer_extent(loan) else { return Ok(self.refuse()); };
            self.control_ready = self.control_birth.built(cx, &mut self.control_trailer, OriginalConstructionDemand {
                items: 1, copy_bytes: capacity, capacity_bytes: capacity, release_bytes: 0, depth: 1,
            }, || {
                let grant = CosGrant { work: 1, allocation_bytes: capacity, release_bytes: 0 };
                let (trailer, capacity) = projection_trailer(loan, grant).map_err(|_| invalid("PDF semantic trailer is malformed"))?
                    .ok_or_else(|| invalid("PDF semantic trailer lost its admitted allocation"))?;
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: capacity,
                    retained_capacity_bytes: capacity, ..Default::default() };
                Ok((trailer, progress))
            })?;
            return Ok(PlanDecision::Pending);
        }

        if !self.proof_ready {
            let granted = cx.retained_grant();
            let body = self.proof.next_inline_body_bytes();
            let result_body = if self.proof.comparing() { 0 } else { size_of::<PdfSnapshot>() };
            let slots = self.proof.reserving_empty_slots();
            let work = if self.proof.comparing() { COMPARE_WORK } else { 1 };
            if cx.fuel_remaining() < work as u64 || granted.maximum_items == 0 || granted.maximum_depth == 0
                || granted.maximum_copy_bytes < body + result_body
            { return Ok(PlanDecision::Pending); }
            let allocation = if slots { granted.maximum_release_bytes } else { granted.maximum_copy_bytes - body - result_body };
            let grant = CosGrant { work, allocation_bytes: granted.maximum_capacity_bytes.min(allocation),
                release_bytes: granted.maximum_release_bytes };
            let trailer = self.control_trailer.as_ref().ok_or_else(|| invalid("PDF semantic trailer is absent"))?;
            match self.proof.advance(loan, trailer, snapshot, grant, false) {
                Ok(ExactGraphTurn::Compared { work, ready }) => {
                    cx.consume_fuel(work as u64);
                    self.proof_ready = ready;
                }
                Ok(ExactGraphTurn::Lift(turn)) => {
                    let allocated = turn.allocated_bytes;
                    let copy = if slots { 0 } else { allocated };
                    let inline = turn.work.saturating_mul(body) + if matches!(turn.event, Ok(ControlledLiftEvent::Complete)) { result_body } else { 0 };
                    cx.consume_retained(RetainedCloneProgress {
                        copied_items: usize::from(allocated != 0 || inline != 0 || matches!(turn.event, Ok(ControlledLiftEvent::Complete))),
                        copied_bytes: copy.saturating_add(inline), retained_capacity_bytes: allocated,
                        released_bytes: turn.released_bytes, ..Default::default()
                    })?;
                    cx.consume_fuel(turn.work as u64);
                    if matches!(turn.event, Ok(ControlledLiftEvent::Deferred(_))) { return Ok(self.refuse()); }
                    match &turn.event {
                        Ok(event @ ControlledLiftEvent::NeedsAllocation { .. }) if turn.work == 0 => {
                            let blocked = (event.clone(), grant.allocation_bytes);
                            if self.proof_grant.replace(blocked) == Some((event.clone(), grant.allocation_bytes)) {
                                return Ok(self.refuse());
                            }
                        }
                        Ok(event @ ControlledLiftEvent::NeedsRelease { .. }) if turn.work == 0 => {
                            let blocked = (event.clone(), grant.release_bytes);
                            if self.proof_grant.replace(blocked) == Some((event.clone(), grant.release_bytes)) {
                                return Ok(self.refuse());
                            }
                        }
                        Ok(ControlledLiftEvent::More | ControlledLiftEvent::Complete
                            | ControlledLiftEvent::NeedsAllocation { .. } | ControlledLiftEvent::NeedsRelease { .. }) => {
                            self.proof_grant = None;
                        }
                        _ => return Err(invalid("current PDF graph requires a different semantic receiver").into()),
                    }
                }
                Err(ExactGraphRefusal::Shape | ExactGraphRefusal::Semantic) => return Ok(self.refuse()),
                Err(_) => return Err(invalid("candidate PDF graph lost its original source").into()),
            }
            return Ok(PlanDecision::Pending);
        }

        let (prefix, objects) = (quote.prefix(), quote.objects());
        let ceiling = cx.original_turn_ceiling().0;
        let table = objects.saturating_mul(size_of::<u32>());
        if prefix > ceiling.maximum_copy_bytes || prefix > ceiling.maximum_capacity_bytes || table > ceiling.maximum_capacity_bytes {
            return Ok(self.refuse());
        }
        let copied = size_of::<Vec<u8>>();
        if self.scratch.is_none() || !self.scratch_birth.terminal_is_empty() {
            self.scratch_birth.built(cx, &mut self.scratch, OriginalConstructionDemand {
                items: 1, copy_bytes: copied, capacity_bytes: prefix, release_bytes: 0, depth: 1,
            }, || {
                let mut buffer = Vec::new();
                buffer.try_reserve_exact(prefix).map_err(|_| ValueError::literal(
                    ValueRefusalKind::AllocationFailed, "classic PDF scratch admission refused",
                ))?;
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: copied,
                    retained_capacity_bytes: buffer.capacity(), ..Default::default() };
                Ok((buffer, progress))
            })?;
            return Ok(PlanDecision::Pending);
        }
        if self.offsets.is_none() || !self.offsets_birth.terminal_is_empty() {
            self.offsets_birth.built(cx, &mut self.offsets, OriginalConstructionDemand {
                items: 1, copy_bytes: size_of::<Vec<u32>>(), capacity_bytes: table, release_bytes: 0, depth: 1,
            }, || {
                let mut offsets = Vec::new();
                offsets.try_reserve_exact(objects).map_err(|_| ValueError::literal(
                    ValueRefusalKind::AllocationFailed, "classic PDF offset table admission refused",
                ))?;
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Vec<u32>>(),
                    retained_capacity_bytes: offsets.capacity() * size_of::<u32>(), ..Default::default() };
                Ok((offsets, progress))
            })?;
            return Ok(PlanDecision::Pending);
        }
        let buffer = self.scratch.take().ok_or_else(|| invalid("classic PDF scratch receiver is empty"))?;
        let offsets = self.offsets.take().ok_or_else(|| invalid("classic PDF offset receiver is empty"))?;
        let started = match (self.route, self.increment.as_ref()) {
            (Route::Increment, Some(plan)) => {
                let tail = plan.tail().ok_or_else(|| invalid("appended PDF section lost its original tail"))?;
                let frame = AppendFrame { length: tail.length, prev: tail.startxref, size: plan.size(), stream: tail.stream, separator: !tail.ends_with_eol };
                ClassicGraphWriter::start_append(quote, buffer, offsets, frame)
            }
            _ => ClassicGraphWriter::start(quote, buffer, offsets),
        };
        match started {
            Ok(writer) => self.writer = Some(writer),
            Err((_, buffer, offsets)) => {
                self.scratch = Some(buffer);
                self.offsets = Some(offsets);
                return Err(invalid("classic PDF graph writer refused its admitted buffers").into());
            }
        }
        self.prefix = prefix;
        self.revision = Some(revision);
        Ok(PlanDecision::Ready)
    }

    fn emit_original(&mut self, snapshot: &PdfSnapshot, sink: &mut dyn Write, grant: usize) -> PdfEmissionTurn {
        let result = (|| {
            let revision = self.revision.ok_or_else(|| invalid("classic PDF has no captured revision"))?;
            let base = self.reference.as_ref().unwrap_or(&self.graph).loan(snapshot, revision).map_err(|_| invalid("classic PDF source changed"))?;
            let delta;
            let loan = if self.route == Route::Increment {
                let rows = self.increment.as_ref().ok_or_else(|| invalid("appended PDF section has no plan"))?.rows();
                delta = GraphLoan { version: base.version, objects: rows, order: None, trailer: self.graph.trailer(),
                    overlay: Default::default(), resolver: Some(&base) };
                delta
            } else { base };
            let base = self.increment.as_ref();
            let original = base.and_then(IncrementPlan::source).map(|source| &**source as &dyn ByteReadSource);
            let base_len = base.and_then(IncrementPlan::tail).map_or(0, |tail| tail.length);
            let writer = self.writer.as_mut().ok_or_else(|| invalid("classic PDF has no admitted writer"))?;
            let mut tee = Tee { inner: sink, written: &mut self.written, base: base_len, appended: if self.route == Route::Increment { self.appended.as_mut() } else { None } };
            writer.step(loan, &mut tee, grant, original).map_err(|_| invalid("classic PDF writer refused its next span"))
        })();
        match result {
            Ok(turn) => PdfEmissionTurn { result: Ok(turn.done), read: turn.prepared,
                work: if turn.control { self.prefix.saturating_sub(turn.prepared) } else { 0 } },
            Err(error) => PdfEmissionTurn { result: Err(error), read: 0, work: 0 },
        }
    }
}

fn invalid(detail: &'static str) -> ValueError { ValueError::invariant(detail) }

/// 🪞️ Forwards the written bytes and keeps a copy of every byte beyond the base file for the custody rebase.
struct Tee<'a> {
    inner: &'a mut dyn Write,
    written: &'a mut usize,
    base: usize,
    appended: Option<&'a mut Appender>,
}

impl Write for Tee<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let count = self.inner.write(bytes)?;
        if let Some(appended) = self.appended.as_deref_mut() {
            let skip = self.base.saturating_sub(*self.written).min(count);
            appended.push(&bytes[skip..count]);
        }
        *self.written += count;
        Ok(count)
    }

    fn flush(&mut self) -> std::io::Result<()> { self.inner.flush() }
}

/// 📦️ Admits the shallow retained-control candidate without copying any current graph object.
pub fn admit_retained_candidate(
    snapshot: &PdfSnapshot, revision: [u8; 32], output: &mut Option<PinnedPdfGraph>,
    construction: &mut MediaConstruction, cx: &mut StepContext<'_>,
) -> Result<bool, OriginalTurnFailure> {
    let capacity = PinnedPdfGraph::retained_control_extent(snapshot)
        .ok_or_else(|| invalid("PDF requires reconciled graph preparation"))?;
    let copied = size_of::<PinnedPdfGraph>() + capacity;
    construction.built(cx, output, OriginalConstructionDemand {
        items: 1, copy_bytes: copied, capacity_bytes: capacity, release_bytes: 0, depth: 1,
    }, || {
        let graph = PinnedPdfGraph::retained_candidate(snapshot, revision)
            .map_err(|_| invalid("current retained PDF graph control could not be admitted"))?;
        let actual = graph.retained_control_capacity();
        Ok((graph, RetainedCloneProgress { copied_items: 1,
            copied_bytes: size_of::<PinnedPdfGraph>() + actual, retained_capacity_bytes: actual, ..Default::default() }))
    })
}

use super::{classic_graph_quote::{GraphQuote, MAX_PREFIX, QUOTE_WORK}, classic_graph_writer::ClassicGraphWriter};

use std::io::Write;
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_plugin::{MediaConstruction, __job::{OriginalConstructionDemand, OriginalTurnFailure, StepContext}};
use crate::standards::v1_7::subsets::base::{
    modules::{lexer::CosGrant, lift::ControlledLiftEvent},
    schema::snapshot::PdfSnapshot,
};
use super::{graph::PinnedPdfGraph, exact_graph_proof::{COMPARE_WORK, ExactGraphProof, ExactGraphRefusal,
    ExactGraphTurn, ProjectionTrailer, projection_trailer, projection_trailer_extent},
    ordinary_graft::{GRAFT_WORK, OrdinaryGraft, OrdinaryRows}};
use super::producer::{PdfEmissionTurn, PdfNativeSaveProgram, PlanDecision};

pub struct ClassicPdfSaveProgram {
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
    proof_ready: bool,
    scratch: Option<Vec<u8>>,
    scratch_birth: MediaConstruction,
    writer: Option<ClassicGraphWriter>,
}

impl ClassicPdfSaveProgram {
    /// 🗃️ Takes the exact current source after the caller has funded the reconciliation owner.
    pub fn new(graph: PinnedPdfGraph, maximum: usize) -> Self {
        Self { graph, maximum, graft: Default::default(), graft_grant: None, revision: None, quote: None, scratch: None,
            control_trailer: None, control_birth: Default::default(), control_ready: false, proof: ExactGraphProof::new(), proof_ready: false,
            scratch_birth: Default::default(), writer: None }
    }

    /// ♻️ Transfers every retained child to the enclosing job's original close path.
    pub fn retire(self) -> ClassicPdfSaveRetirement {
        ClassicPdfSaveRetirement {
            graph: self.graph, pending_graft: self.graft, proof: self.proof, control_trailer: self.control_trailer,
            control_birth: self.control_birth, scratch_birth: self.scratch_birth,
            scratch: self.writer.map(ClassicGraphWriter::into_buffer).or(self.scratch),
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
    pub scratch: Option<Vec<u8>>,
}

impl PdfNativeSaveProgram<PdfSnapshot> for ClassicPdfSaveProgram {
    fn prepare_original(
        &mut self, snapshot: &PdfSnapshot, revision: [u8; 32], cx: &mut StepContext<'_>,
    ) -> Result<PlanDecision, OriginalTurnFailure> {
        cx.original_source_parent()?;
        let loan = self.graph.loan(snapshot, revision)
            .map_err(|_| invalid("current PDF graph has another original revision"))?;
        if cx.should_yield() || cx.is_cancelled() { return Ok(PlanDecision::Pending); }
        if self.writer.is_some() { return Ok(PlanDecision::Ready); }
        if self.graph.needs_ordinary_graft() {
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
                Err(_) => return Ok(PlanDecision::ReferenceRequired),
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
            self.quote = match GraphQuote::start(loan, self.maximum) {
                Ok(quote) => Some(quote),
                Err(_) => return Ok(PlanDecision::ReferenceRequired),
            };
            cx.consume_fuel(QUOTE_WORK as u64);
            return Ok(PlanDecision::Pending);
        }
        let quote = self.quote.as_mut().expect("started PDF graph quote");
        if quote.maximum().is_none() {
            if cx.fuel_remaining() < QUOTE_WORK as u64 { return Ok(PlanDecision::Pending); }
            let planned = quote.next(loan, QUOTE_WORK);
            cx.consume_fuel(QUOTE_WORK as u64);
            if planned.is_err() { return Ok(PlanDecision::ReferenceRequired); }
            return Ok(PlanDecision::Pending);
        }

        if !self.control_ready {
            let Ok(capacity) = projection_trailer_extent(loan) else { return Ok(PlanDecision::ReferenceRequired); };
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
                    if matches!(turn.event, Ok(ControlledLiftEvent::Deferred(_))) { return Ok(PlanDecision::ReferenceRequired); }
                    if !matches!(turn.event, Ok(ControlledLiftEvent::More | ControlledLiftEvent::Complete
                        | ControlledLiftEvent::NeedsAllocation { .. } | ControlledLiftEvent::NeedsRelease { .. }))
                    {
                        return Err(invalid("current PDF graph requires a different semantic receiver").into());
                    }
                }
                Err(ExactGraphRefusal::Shape | ExactGraphRefusal::Semantic) => return Ok(PlanDecision::ReferenceRequired),
                Err(_) => return Err(invalid("candidate PDF graph lost its original source").into()),
            }
            return Ok(PlanDecision::Pending);
        }

        let copied = size_of::<Vec<u8>>();
        let ready = self.scratch_birth.built(cx, &mut self.scratch, OriginalConstructionDemand {
            items: 1, copy_bytes: copied, capacity_bytes: MAX_PREFIX, release_bytes: 0, depth: 1,
        }, || {
            let mut buffer = Vec::new();
            buffer.try_reserve_exact(MAX_PREFIX).map_err(|_| ValueError::literal(
                ValueRefusalKind::AllocationFailed, "classic PDF scratch admission refused",
            ))?;
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: copied,
                retained_capacity_bytes: buffer.capacity(), ..Default::default() };
            Ok((buffer, progress))
        })?;
        if !ready { return Ok(PlanDecision::Pending); }
        let buffer = self.scratch.take().ok_or_else(|| invalid("classic PDF scratch receiver is empty"))?;
        match ClassicGraphWriter::start(quote, buffer) {
            Ok(writer) => self.writer = Some(writer),
            Err((_, buffer)) => {
                self.scratch = Some(buffer);
                return Err(invalid("classic PDF graph writer refused its admitted buffer").into());
            }
        }
        self.revision = Some(revision);
        Ok(PlanDecision::Ready)
    }

    fn emit_original(&mut self, snapshot: &PdfSnapshot, sink: &mut dyn Write, grant: usize) -> PdfEmissionTurn {
        let result = (|| {
            let revision = self.revision.ok_or_else(|| invalid("classic PDF has no captured revision"))?;
            let loan = self.graph.loan(snapshot, revision).map_err(|_| invalid("classic PDF source changed"))?;
            let writer = self.writer.as_mut().ok_or_else(|| invalid("classic PDF has no admitted writer"))?;
            writer.step(loan, sink, grant).map_err(|_| invalid("classic PDF writer refused its next span"))
        })();
        match result {
            Ok(turn) => PdfEmissionTurn { result: Ok(turn.done), read: turn.prepared,
                work: if turn.control { MAX_PREFIX.saturating_sub(turn.prepared) } else { 0 } },
            Err(error) => PdfEmissionTurn { result: Err(error), read: 0, work: 0 },
        }
    }
}

fn invalid(detail: &'static str) -> ValueError { ValueError::invariant(detail) }

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

//! 🔁️ Mounted PDF natural-file import over the original admitted source and step budgets.

use super::original_custody::{self, GraphDigest, Stash, DIGEST_STEP_BYTES};
use super::retained_graph::{admit_box, exact_position, ImportedGraphRetirement, PdfByteGraphEvent, PdfByteGraphReceipt, PdfFilePreparationCursor, PreparedPdfFile};
use crate::standards::v1_7::subsets::base::modules::lexer::{CosGrant, CosStatus, PdfCosCursor};
use crate::standards::v1_7::subsets::base::modules::lift::{ControlledDocumentLiftCursor, ControlledLiftEvent, DocumentLiftCursor};
use crate::standards::v1_7::subsets::base::modules::xref::{XrefIndexStatus, XrefStreamError};
use crate::standards::v1_7::subsets::base::schema::graph_source::GraphSource;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfPage, PdfSnapshot, STDIO_PDF17_DOCUMENT_SCHEMA};
use semio_framework_value::{ByteReadSource, ErasedSnapshotRetirement, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_plugin::{MediaError, NaturalFileDecodeCursor, NaturalFileDecodeOwner, NaturalFilePagedSource, NaturalFileDecodeProgress, NaturalFileDecodeStep, PluginLifecycleStep, StepContext, ToolExecutionContract, NATURAL_FILE_PORT};
use std::mem::{size_of, take};
use store::os_store::snapshot_read::{artifact_retirement_box_close_step, artifact_retirement_box_demands, settle_close_poll};

const COPY_PER_WORK: usize = 128;
/// 📏️ Stream body bytes one decode work unit covers: a body unit inflates, filters or moves one byte, which costs a small fraction of a token unit, so one contract unit is a run of this many bytes.
const BODY_BYTES_PER_UNIT: usize = 64;
/// 📏️ Copy one body byte costs: one byte into the filter's staging or history, one into the reserved output page; a lift run unit stands for the same two bytes.
const BODY_COPY_PER_BYTE: usize = ControlledDocumentLiftCursor::RUN_UNIT_COPY;


#[derive(Clone, Copy)]
enum Phase { AllocateFile, PrepareFile, TransferFile, StashFile, AllocateLift, Lift, TransferLift, ClosePositions, DigestGraph, Publish, Done, Closing }

/// 📥️ Selects the mounted decoder before moving its original source.
pub fn admit_pdf_natural_decode(
    bytes: &mut Option<Vec<u8>>, contract: ToolExecutionContract,
    receiving: &mut Option<NaturalFileDecodeOwner<PdfSnapshot>>, context: &mut StepContext<'_>,
) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
    if receiving.is_some() { return Ok(true); }
    let source = bytes.as_ref().ok_or_else(|| ValueError::invariant("missing original PDF natural file source"))?;
    let controlled = source.len() > contract.max_work_units_per_step as usize;
    let capacity = if controlled { size_of::<NaturalPdfDecoder>() } else { 0 };
    let copy = capacity + size_of::<NaturalFileDecodeOwner<PdfSnapshot>>();
    let grant = context.retained_grant();
    if context.should_yield() || context.is_cancelled() || grant.maximum_items == 0 || grant.maximum_depth == 0
        || copy > grant.maximum_copy_bytes || capacity > grant.maximum_capacity_bytes
    {
        return Ok(false);
    }
    if controlled {
        let mut decoder = admit_box(NaturalPdfDecoder::new(contract)).ok_or_else(|| {
            ValueError::literal(ValueRefusalKind::AllocationFailed, "unable to allocate the controlled PDF file decoder")
        })?;
        decoder.source = PdfNaturalSource::Contiguous(bytes.take().expect("original PDF source is present"));
        *receiving = Some(NaturalFileDecodeOwner::Controlled(decoder));
    } else {
        *receiving = Some(NaturalFileDecodeOwner::Synchronous(bytes.take().expect("original PDF source is present")));
    }
    context.consume_fuel(1);
    context.consume_retained(RetainedCloneProgress {
        copied_items: 1, copied_bytes: copy, retained_capacity_bytes: capacity, ..Default::default()
    })?;
    Ok(true)
}

/// 🏗️ Creates the same native owner for explicit cold tool construction.
pub fn build_pdf_natural_decode(bytes: Vec<u8>, contract: ToolExecutionContract) -> Result<NaturalFileDecodeOwner<PdfSnapshot>, MediaError> {
    if bytes.len() <= contract.max_work_units_per_step as usize { return Ok(NaturalFileDecodeOwner::Synchronous(bytes)); }
    let mut decoder = admit_box(NaturalPdfDecoder::new(contract)).ok_or_else(|| failure("unable to allocate the controlled PDF file decoder"))?;
    decoder.source = PdfNaturalSource::Contiguous(bytes);
    Ok(NaturalFileDecodeOwner::Controlled(decoder))
}

/// 📏️ Bounds the independently admitted ordinary PDF source extent.
pub const PDF_PAGED_NATURAL_SOURCE_MAXIMUM_BYTES: u32 = 16 * 1024 * 1024;

/// 📄️ Donates the exact sealed page owner to a paid PDF decoder frame.
pub fn admit_pdf_paged_natural_decode(
    source: &mut Option<NaturalFilePagedSource>, contract: ToolExecutionContract,
    receiving: &mut Option<NaturalFileDecodeOwner<PdfSnapshot>>, context: &mut StepContext<'_>,
) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
    if receiving.is_some() { return Ok(true); }
    let original = source.as_ref().filter(|owner| owner.original_is_untouched()).and_then(|owner| owner.original())
        .ok_or_else(|| ValueError::invariant("missing sealed original PDF pages"))?;
    if original.byte_len() == 0 || original.byte_len() > PDF_PAGED_NATURAL_SOURCE_MAXIMUM_BYTES as usize {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "original PDF extent exceeds its declared source ceiling").into());
    }
    let capacity = size_of::<NaturalPdfDecoder>();
    let copy = capacity + size_of::<NaturalFileDecodeOwner<PdfSnapshot>>();
    let grant = context.retained_grant();
    if context.should_yield() || context.is_cancelled() || grant.maximum_items == 0 || grant.maximum_depth == 0
        || copy > grant.maximum_copy_bytes || capacity > grant.maximum_capacity_bytes
    {
        return Ok(false);
    }
    let mut decoder = admit_box(NaturalPdfDecoder::new(contract)).ok_or_else(||
        ValueError::literal(ValueRefusalKind::AllocationFailed, "unable to allocate the controlled PDF file decoder"))?;
    decoder.source = PdfNaturalSource::Paged(source.take().expect("sealed original PDF pages"));
    *receiving = Some(NaturalFileDecodeOwner::Controlled(decoder));
    context.consume_fuel(1);
    context.consume_retained(RetainedCloneProgress {
        copied_items: 1, copied_bytes: copy, retained_capacity_bytes: capacity, ..Default::default()
    })?;
    Ok(true)
}

enum PdfNaturalSource {
    Contiguous(Vec<u8>),
    Paged(NaturalFilePagedSource),
}

impl PdfNaturalSource {
    fn reader(&self) -> Option<&dyn ByteReadSource> {
        match self {
            Self::Contiguous(source) => Some(source),
            Self::Paged(source) => source.original().map(|bytes| bytes as &dyn ByteReadSource),
        }
    }

    fn contiguous_capacity(&self) -> usize {
        match self { Self::Contiguous(bytes) => bytes.capacity(), Self::Paged(_) => 0 }
    }

    fn close_paged(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<semio_framework_value::RetainedCloneStep>, ValueError> {
        let Self::Paged(source) = self else { return Ok(None); };
        let progress = settle_close_poll(source.step(effect.grant().maximum_copy_bytes, effect))?;
        if source.terminal_is_empty() { *self = Self::Contiguous(Vec::new()); }
        Ok(Some(progress))
    }

    fn terminal_is_empty(&self) -> bool {
        match self { Self::Contiguous(bytes) => bytes.capacity() == 0, Self::Paged(source) => source.terminal_is_empty() }
    }
}

struct NaturalPdfDecoder {
    phase: Phase,
    source: PdfNaturalSource,
    preparing: Option<Box<PdfFilePreparationCursor>>,
    file: Option<PreparedPdfFile>,
    lifting: Option<Box<ControlledDocumentLiftCursor>>,
    snapshot: Option<PdfSnapshot>,
    partial_page: Option<PdfPage>,
    snapshot_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    trailer_retirement: Option<PdfCosCursor>,
    graph_retirement: ImportedGraphRetirement,
    contract: ToolExecutionContract,
    progress: NaturalFileDecodeProgress,
    live_bytes: usize,
    next_release: usize,
    blocked: Option<(PdfByteGraphEvent, usize)>,
    peak: (usize, usize),
    demanded: usize,
    stall: u8,
    stall_at: Option<String>,
    stash: Stash,
    digest: GraphDigest,
}

impl NaturalPdfDecoder {
    fn new(contract: ToolExecutionContract) -> Self {
        Self {
            phase: Phase::AllocateFile, source: PdfNaturalSource::Contiguous(Vec::new()), preparing: None, file: None, lifting: None,
            snapshot: None, partial_page: None, snapshot_retirement: None, trailer_retirement: None,
            graph_retirement: Default::default(), contract, progress: Default::default(), live_bytes: 0, next_release: 0, blocked: None, peak: (0, 0), demanded: 0, stall: 0, stall_at: None,
            stash: Stash::default(), digest: GraphDigest::default(),
        }
    }

    fn next(&mut self, grant: CosGrant, bodies: bool) -> Result<PdfByteGraphReceipt, MediaError> {
        use PdfByteGraphEvent as Event;
        match self.phase {
            Phase::AllocateFile => {
                let bytes = size_of::<PdfFilePreparationCursor>();
                if bytes > grant.allocation_bytes { return Ok(turn(Event::NeedsAllocation { bytes }, 0)); }
                let preparing = PdfFilePreparationCursor::new(self.contract.max_decoded_items, self.contract.max_output_bytes);
                self.preparing = Some(admit_box(preparing).ok_or_else(|| failure("unable to allocate PDF file preparation"))?);
                self.phase = Phase::PrepareFile;
                Ok(PdfByteGraphReceipt { allocated_bytes: bytes, ..turn(Event::More, 1) })
            }
            Phase::PrepareFile => {
                let source = self.source.reader().ok_or_else(|| failure("original PDF source was retired before preparation"))?;
                let mut step = self.preparing.as_mut().expect("PDF file preparation").advance(source, grant, false);
                if matches!(step.event, Ok(Event::Complete)) {
                    self.phase = Phase::TransferFile;
                    step.event = Ok(Event::More);
                }
                Ok(step)
            }
            Phase::TransferFile => {
                let bytes = size_of::<PdfFilePreparationCursor>();
                if bytes > grant.release_bytes { return Ok(turn(Event::NeedsRelease { bytes }, 0)); }
                self.file = self.preparing.as_mut().expect("PDF file transfer").take();
                self.preparing = None;
                self.progress.consumed_input_bytes = self.source.reader().ok_or_else(|| failure("original PDF source was retired before transfer"))?.byte_len();
                self.phase = Phase::StashFile;
                Ok(PdfByteGraphReceipt { released_bytes: bytes, ..turn(Event::More, 1) })
            }
            Phase::StashFile => {
                let source = self.source.reader().ok_or_else(|| failure("original PDF source was retired before custody"))?;
                if self.stash.advance(source) { self.phase = Phase::AllocateLift; }
                Ok(turn(Event::More, 1))
            }
            Phase::AllocateLift => {
                let bytes = size_of::<ControlledDocumentLiftCursor>();
                if bytes > grant.allocation_bytes { return Ok(turn(Event::NeedsAllocation { bytes }, 0)); }
                self.lifting = Some(admit_box(DocumentLiftCursor::controlled()).ok_or_else(|| failure("unable to allocate PDF semantic lift"))?);
                self.phase = Phase::Lift;
                Ok(PdfByteGraphReceipt { allocated_bytes: bytes, ..turn(Event::More, 1) })
            }
            Phase::Lift => {
                let file = self.file.as_ref().expect("semantic PDF source");
                let position = |id| exact_position(&file.graph.positions, id);
                let mut source = GraphSource::with_positions(&file.graph.objects, &position);
                let lifting = self.lifting.as_mut().expect("semantic PDF owner");
                let step = if bodies { lifting.advance_bodies(&file.trailer, file.header.version(), &mut source, grant, false) } else { lifting.advance(&file.trailer, file.header.version(), &mut source, grant, false) };
                let event = match step.event {
                    Ok(ControlledLiftEvent::Complete) => { self.phase = Phase::TransferLift; Ok(Event::More) }
                    Ok(ControlledLiftEvent::Deferred(_)) => Ok(Event::ReferenceRequired),
                    other => lift_event(other),
                };
                Ok(PdfByteGraphReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes })
            }
            Phase::TransferLift => {
                let bytes = size_of::<ControlledDocumentLiftCursor>();
                if bytes > grant.release_bytes { return Ok(turn(Event::NeedsRelease { bytes }, 0)); }
                let snapshot = self.lifting.as_mut().expect("semantic PDF transfer").take_result()
                    .ok_or_else(|| failure("completed semantic PDF could not transfer"))?;
                self.lifting = None;
                self.snapshot = Some(snapshot);
                self.file.as_mut().expect("PDF positions retirement").graph.positions.begin_close();
                self.phase = Phase::ClosePositions;
                Ok(PdfByteGraphReceipt { released_bytes: bytes, ..turn(Event::More, 1) })
            }
            Phase::ClosePositions => {
                let step = self.file.as_mut().expect("PDF original graph").graph.positions.close_step(grant);
                let event = step.status.map_err(XrefStreamError::Structure).map(|status| match status {
                    XrefIndexStatus::Closed => { self.phase = Phase::DigestGraph; Event::More }
                    XrefIndexStatus::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
                    _ => Event::More,
                });
                Ok(PdfByteGraphReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes })
            }
            Phase::DigestGraph => {
                let objects = &self.file.as_ref().expect("PDF graph digest").graph.objects;
                self.digest.advance(objects.len(), |index| objects.get(index), DIGEST_STEP_BYTES);
                if self.digest.done(objects.len()) { self.phase = Phase::Publish; }
                Ok(turn(Event::More, 1))
            }
            Phase::Publish => Ok(turn(Event::Complete, 1)),
            _ => Err(failure("PDF natural file decoder no longer advanceable")),
        }
    }

    fn publish_snapshot(&mut self) -> Result<PdfSnapshot, MediaError> {
        let output = self.snapshot.as_ref().expect("PDF typed data is owned through publication");
        if output.schema != STDIO_PDF17_DOCUMENT_SCHEMA || !output.objects.is_empty() || !output.trailer.is_empty() {
            return Err(failure("unsettled semantic PDF snapshot has incompatible raw lanes"));
        }
        let file = self.file.take().expect("PDF original data is owned through publication");
        let mut snapshot = self.snapshot.take().expect("PDF typed data is owned through publication");
        let digest = take(&mut self.digest).finish(file.graph.objects.len());
        let stash = take(&mut self.stash).finish();
        if !file.graph.objects.is_empty() { original_custody::retain(stash, digest); }
        snapshot.objects = file.graph.objects;
        snapshot.trailer = file.trailer;
        self.phase = Phase::Done;
        Ok(snapshot)
    }

    fn copy_body(&self) -> usize {
        match self.phase {
            Phase::TransferFile => size_of::<PreparedPdfFile>(),
            Phase::TransferLift => size_of::<PdfSnapshot>(),
            Phase::Publish => size_of::<PreparedPdfFile>() + size_of::<PdfSnapshot>(),
            _ => COPY_PER_WORK,
        }.max(COPY_PER_WORK)
    }

    /// 🏃️ Whether the next call runs bodies: preparation decoding stream bodies, or a lift stage that moves image bytes or runs a page program.
    fn decodes_body_bytes(&self) -> bool {
        match self.phase {
            Phase::PrepareFile => self.preparing.as_ref().is_some_and(|owner| owner.decoding_body_bytes()),
            Phase::Lift => self.lifting.as_ref().is_some_and(|owner| owner.runs_bodies()),
            _ => false,
        }
    }

    /// 📏️ The fewest body units a call may be granted: a lift run must fit its costliest step.
    fn body_floor(&self) -> usize {
        if matches!(self.phase, Phase::Lift) { ControlledDocumentLiftCursor::RUN_UNIT_MAXIMUM } else { 1 }
    }


    /// ✍️ Bytes the next preparation unit writes when it fills one index slot or queue entry, which it is charged instead of a token unit.
    fn writing_bytes(&self) -> Option<usize> {
        if !matches!(self.phase, Phase::PrepareFile) { return None; }
        self.preparing.as_ref().and_then(|owner| owner.writing_bytes())
    }

    fn reserves_empty_output(&self) -> bool {
        match self.phase {
            Phase::PrepareFile => self.preparing.as_ref().is_some_and(|owner| owner.reserving_empty_backing()),
            Phase::Lift => self.lifting.as_ref().is_some_and(|owner| owner.reserving_empty_slots()),
            _ => false,
        }
    }

    /// 🧾️ Charges one call: `copy_body` per reported cursor unit, the allocation as copy unless it only reserved empty output, and `per_unit` cursor units (body bytes) as one contract unit of fuel and progress.
    fn record(&mut self, step: &PdfByteGraphReceipt, copy_body: usize, per_unit: usize, inline_item: bool, empty_output: bool, context: &mut StepContext<'_>) -> Result<(), MediaError> {
        let grant = context.retained_grant();
        let allocation_copy = if empty_output { 0 } else { step.allocated_bytes };
        let copied = allocation_copy.saturating_add(step.work.saturating_mul(copy_body));
        let units = step.work.div_ceil(per_unit);
        let copied_items = usize::from(step.allocated_bytes != 0
            || inline_item && step.work != 0 && matches!(step.event, Ok(PdfByteGraphEvent::More))
            || matches!(step.event, Ok(PdfByteGraphEvent::Object | PdfByteGraphEvent::Complete)));
        if copied > grant.maximum_copy_bytes {
            return Err(failure("PDF decoder exceeded its admitted copy grant"));
        }
        context.consume_fuel(units as u64);
        context.consume_retained(RetainedCloneProgress {
            copied_items, copied_bytes: copied, retained_capacity_bytes: step.allocated_bytes, released_bytes: step.released_bytes, ..Default::default()
        }).map_err(|error| failure(&error.to_string()))?;
        self.progress.completed_work_units = self.progress.completed_work_units.saturating_add(units as u64);
        self.live_bytes = self.live_bytes.saturating_add(step.allocated_bytes).checked_sub(step.released_bytes)
            .ok_or_else(|| failure("PDF decoder released backing it does not own"))?;
        if let Some(releasable) = self.releasable_bytes().filter(|releasable| exceeds_ceiling(self.live_bytes, *releasable, self.contract.max_output_bytes)) {
            return Err(failure(&format!("natural-file decoder retains at least {} output bytes it can no longer release; maximum is {}", self.live_bytes - releasable, self.contract.max_output_bytes)));
        }
        Ok(())
    }

    /// 🔒️ Backing the decoder credits back before it completes: the lift's tables and builder state, its own cursor and the position index. What is live beyond it stays output however the rest of the file reads, so a document already over the ceiling is refused in that turn, not after the lift has copied the rest of the file; `None` while preparation still owns transient backing.
    fn releasable_bytes(&self) -> Option<usize> {
        if !matches!(self.phase, Phase::Lift | Phase::TransferLift | Phase::ClosePositions | Phase::DigestGraph | Phase::Publish) { return None; }
        if self.lifting.as_ref().is_some_and(|owner| !owner.between_pages()) { return None; }
        let lift = self.lifting.as_ref().map_or(0, |owner| owner.held_bytes() + size_of::<ControlledDocumentLiftCursor>());
        let positions = self.file.as_ref().map_or(0, |file| file.graph.positions.held_bytes());
        Some(lift + positions)
    }

    fn close_native(&mut self, grant: CosGrant) -> PdfByteGraphReceipt {
        use PdfByteGraphEvent as Event;
        if let Some(owner) = self.preparing.as_mut() {
            let mut step = owner.close_step(grant);
            if matches!(step.event, Ok(Event::Closed)) {
                if step.work == 0 {
                    let bytes = size_of::<PdfFilePreparationCursor>();
                    if bytes > grant.release_bytes { return turn(Event::NeedsRelease { bytes }, 0); }
                    self.preparing = None;
                    return release_turn(bytes);
                }
                step.event = Ok(Event::More);
            }
            return step;
        }
        if let Some(owner) = self.lifting.as_mut() {
            let step = owner.close_step(grant);
            let event = match step.event {
                Ok(ControlledLiftEvent::Closed) if step.work == 0 => {
                    let bytes = size_of::<ControlledDocumentLiftCursor>();
                    if bytes > grant.release_bytes { return turn(Event::NeedsRelease { bytes }, 0); }
                    let partial = owner.take_partial();
                    if self.snapshot.is_none() { self.snapshot = partial.snapshot; }
                    self.partial_page = partial.page;
                    self.lifting = None;
                    return release_turn(bytes);
                }
                other => lift_event(other),
            };
            return PdfByteGraphReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes };
        }
        if let Some(cursor) = self.trailer_retirement.as_mut() {
            let step = cursor.close_step(grant);
            if step.status == Ok(CosStatus::Closed) { self.trailer_retirement = None; }
            let event = step.status.map_err(XrefStreamError::Structure).map(|status| match status {
                CosStatus::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
                _ => Event::More,
            });
            return PdfByteGraphReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes };
        }
        if let Some(file) = self.file.as_mut() {
            if file.trailer.capacity() != 0 {
                self.trailer_retirement = Some(PdfCosCursor::retire_dictionary(take(&mut file.trailer)));
                return turn(Event::More, 1);
            }
            let step = self.graph_retirement.close_step(&mut file.graph, grant);
            let event = step.event.map(|status| match status {
                super::retained_graph::GraphImportEvent::Closed => { self.file = None; Event::More }
                super::retained_graph::GraphImportEvent::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
                _ => Event::More,
            });
            return PdfByteGraphReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes };
        }
        let PdfNaturalSource::Contiguous(source) = &mut self.source else { return turn(Event::Closed, 0); };
        let bytes = source.capacity();
        if bytes == 0 { return turn(Event::Closed, 0); }
        if bytes > grant.release_bytes { return turn(Event::NeedsRelease { bytes }, 0); }
        *source = Vec::new();
        release_turn(bytes)
    }
}

impl NaturalFileDecodeCursor<PdfSnapshot> for NaturalPdfDecoder {
    fn advance(&mut self, context: &mut StepContext<'_>) -> Result<NaturalFileDecodeStep<PdfSnapshot>, MediaError> {
        use PdfByteGraphEvent as Event;
        let start = self.progress.completed_work_units;
        let max_work = self.contract.max_work_units_per_step;
        let mut admitted = false;
        while !context.is_cancelled() && self.progress.completed_work_units - start < max_work {
            if admitted && context.should_yield() { break; }
            admitted = true;
            let grant = context.retained_grant();
            let inline_body = self.lifting.as_ref().map_or(0, |owner| owner.next_inline_body_bytes());
            let body = self.decodes_body_bytes();
            let written = if body { None } else { self.writing_bytes() };
            let copy_body = if body { BODY_COPY_PER_BYTE } else { written.map_or_else(|| self.copy_body().max(inline_body), |bytes| bytes.max(BODY_COPY_PER_BYTE)) };
            if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy_body { break; }
            let empty_output = self.reserves_empty_output();
            let remaining = max_work - (self.progress.completed_work_units - start);
            let running = body || written.is_some() && self.blocked.is_none();
            let demanded = take(&mut self.demanded);
            let work = if running { run_work(remaining, context.fuel_remaining(), grant.maximum_copy_bytes, demanded, copy_body) } else { 1 };
            if work < self.body_floor() && body { break; }
            let allocation_boundary = if empty_output { grant.maximum_release_bytes } else { grant.maximum_copy_bytes - copy_body * work };
            let grant = CosGrant { work, allocation_bytes: grant.maximum_capacity_bytes.min(allocation_boundary), release_bytes: grant.maximum_release_bytes };
            self.peak = (self.peak.0.max(grant.allocation_bytes), self.peak.1.max(grant.release_bytes));
            let phase = self.phase;
            let step = self.next(grant, body)?;
            let idle = step.work == 0 && matches!(step.event, Ok(Event::More)) && std::mem::discriminant(&self.phase) == std::mem::discriminant(&phase);
            if idle {
                let where_ = self.preparing.as_ref().map(|cursor| cursor.trace()).unwrap_or_else(|| "-".to_string());
                if self.stall_at.as_deref() != Some(where_.as_str()) {
                    self.stall = 0;
                    self.stall_at = Some(where_.clone());
                }
                self.stall = self.stall.saturating_add(1);
                if self.stall > 64 {
                    return Err(failure(&format!("PDF decoder made no progress during {} at {where_}", phase_name(phase))));
                }
                continue;
            }
            self.stall = 0;
            self.stall_at = None;
            self.record(&step, copy_body, if running { BODY_BYTES_PER_UNIT } else { 1 }, inline_body != 0, empty_output, context)?;
            if step.work != 0 { self.blocked = None; }
            match step.event {
                Ok(Event::Object) => self.progress.decoded_items += 1,
                Ok(Event::More) => {}
                Ok(Event::Complete) => {
                    self.progress.owned_output_bytes = self.live_bytes;
                    return Ok(NaturalFileDecodeStep::Complete { snapshot: self.publish_snapshot()?, progress: self.progress });
                }
                Ok(event @ (Event::NeedsAllocation { bytes } | Event::NeedsRelease { bytes })) => {
                    if let Event::NeedsAllocation { bytes } = event { self.demanded = bytes; }
                    if self.progress.completed_work_units == start {
                        let (available, peak) = if matches!(event, Event::NeedsAllocation { .. }) { (grant.allocation_bytes, self.peak.0) } else { (grant.release_bytes, self.peak.1) };
                        if refused_for_good(&mut self.blocked, event, available, peak) {
                            let stage = self.lifting.as_ref().map_or_else(String::new, |owner| format!(" {} after {} pages", owner.stage_name(), owner.pages_lifted()));
                            return Err(failure(&format!("PDF decoder needs a {bytes}-byte backing grant during {}{stage}", phase_name(self.phase))));
                        }
                        return Ok(NaturalFileDecodeStep::Yield);
                    }
                    break;
                }
                Ok(Event::ReferenceRequired) => {
                    return Err(match self.lifting.as_ref().and_then(|owner| owner.deferred_detail()) {
                        Some((reason, detail, pages)) => failure(&format!("PDF semantic import requires the reference reader for {reason:?} {detail} after {pages} pages")),
                        None => failure("PDF requires reference decoding for this file structure"),
                    });
                }
                Ok(Event::NotPdf) => return Err(failure("input does not identify a PDF")),
                Err(error) => return Err(failure(&xref_message(error))),
                _ => return Err(failure("unexpected PDF decoder continuation")),
            }
        }
        if self.progress.completed_work_units == start { Ok(NaturalFileDecodeStep::Yield) }
        else { Ok(NaturalFileDecodeStep::Progress(self.progress)) }
    }

    fn begin_close(&mut self) {
        if let Some(owner) = self.preparing.as_mut() { owner.begin_close(); }
        if let Some(owner) = self.lifting.as_mut() { owner.begin_close(); }
        self.phase = Phase::Closing;
    }

    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<PluginLifecycleStep, ValueError> {
        if self.snapshot_retirement.is_some() {
            let body = effect.grant().maximum_copy_bytes;
            let step = settle_close_poll(artifact_retirement_box_close_step(&mut self.snapshot_retirement, body, effect))?;
            return Ok(PluginLifecycleStep::Progress(step.progress()));
        }
        if self.snapshot.is_some() { return admit_retirement(&mut self.snapshot, &mut self.snapshot_retirement, effect); }
        if self.partial_page.is_some() { return admit_retirement(&mut self.partial_page, &mut self.snapshot_retirement, effect); }
        let grant = effect.grant();
        if effect.remaining_work_units()? == 0 || grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(PluginLifecycleStep::Progress(Default::default()));
        }
        let step = self.close_native(CosGrant { work: 1, allocation_bytes: 0, release_bytes: grant.maximum_release_bytes });
        if step.work != 0 && !effect.performed(step.work as u64)? {
            return Err(ValueError::invariant("PDF decoder retirement exceeded original work"));
        }
        let progress = RetainedCloneProgress { released_bytes: step.released_bytes, ..Default::default() };
        self.next_release = 0;
        match step.event {
            Ok(PdfByteGraphEvent::Closed) => {
                if let Some(settled) = self.source.close_paged(effect)? {
                    return Ok(if self.source.terminal_is_empty() { PluginLifecycleStep::Complete(settled.progress()) }
                        else { PluginLifecycleStep::Progress(settled.progress()) });
                }
                Ok(PluginLifecycleStep::Complete(progress))
            }
            Ok(PdfByteGraphEvent::NeedsRelease { bytes }) => {
                self.next_release = bytes;
                Ok(PluginLifecycleStep::Progress(progress))
            }
            Ok(_) => Ok(PluginLifecycleStep::Progress(progress)),
            Err(_) => Err(ValueError::invariant("PDF decoder native retirement failed")),
        }
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(retirement) = self.snapshot_retirement.as_ref() { return artifact_retirement_box_demands(retirement, body); }
        if self.snapshot.is_some() { return Ok(retirement_birth::<PdfSnapshot>()); }
        if self.partial_page.is_some() { return Ok(retirement_birth::<PdfPage>()); }
        let files_retired = self.preparing.is_none() && self.file.is_none() && self.lifting.is_none() && self.trailer_retirement.is_none();
        if files_retired {
            if let PdfNaturalSource::Paged(source) = &self.source { return source.next_demand(body); }
        }
        let raw_release = if files_retired { self.source.contiguous_capacity() } else { 0 };
        Ok(RetirementDemand { release_bytes: self.next_release.max(raw_release), depth: 1, ..Default::default() })
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.phase, Phase::Closing) && self.source.terminal_is_empty() && self.preparing.is_none() && self.file.is_none()
            && self.lifting.is_none() && self.snapshot.is_none() && self.partial_page.is_none() && self.snapshot_retirement.is_none()
            && self.trailer_retirement.is_none()
    }
}

fn retirement_birth<T: semio_framework_value::RetireOwned>() -> RetirementDemand {
    RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(), depth: 1, ..Default::default() }
}

fn admit_retirement<T: semio_framework_value::RetireOwned + Send + 'static>(
    value: &mut Option<T>, owner: &mut Option<Box<dyn ErasedSnapshotRetirement>>, effect: &mut RetainedAllocationEffect<'_>,
) -> Result<PluginLifecycleStep, ValueError> {
    let progress = match semio_framework_value::retirement::admit_owned_retirement(value, effect).result? {
        Some((retirement, progress)) => { *owner = Some(retirement); progress }
        None => Default::default(),
    };
    Ok(PluginLifecycleStep::Progress(progress))
}



/// 🧭 Names the decoder phase that stopped consuming work.
fn phase_name(phase: Phase) -> &'static str {
    match phase {
        Phase::AllocateFile => "allocate-file",
        Phase::PrepareFile => "prepare-file",
        Phase::TransferFile => "transfer-file",
        Phase::StashFile => "stash-file",
        Phase::AllocateLift => "allocate-lift",
        Phase::Lift => "lift",
        Phase::TransferLift => "transfer-lift",
        Phase::ClosePositions => "close-positions",
        Phase::DigestGraph => "digest-graph",
        Phase::Publish => "publish",
        Phase::Done => "done",
        Phase::Closing => "closing",
    }
}

fn turn(event: PdfByteGraphEvent, work: usize) -> PdfByteGraphReceipt {
    PdfByteGraphReceipt { event: Ok(event), work, allocated_bytes: 0, released_bytes: 0 }
}

fn release_turn(bytes: usize) -> PdfByteGraphReceipt {
    PdfByteGraphReceipt { released_bytes: bytes, ..turn(PdfByteGraphEvent::More, 1) }
}

fn lift_event(event: Result<ControlledLiftEvent, &'static str>) -> Result<PdfByteGraphEvent, XrefStreamError> {
    event.map_err(XrefStreamError::Structure).map(|event| match event {
        ControlledLiftEvent::NeedsAllocation { bytes } => PdfByteGraphEvent::NeedsAllocation { bytes },
        ControlledLiftEvent::NeedsRelease { bytes } => PdfByteGraphEvent::NeedsRelease { bytes },
        _ => PdfByteGraphEvent::More,
    })
}

fn xref_message(error: XrefStreamError) -> String {
    match error { XrefStreamError::Structure(reason) => reason.into(), XrefStreamError::Decode(error) => error.into_message() }
}

/// 📏️ The units one bulk call may run when each copies `unit_copy` bytes: the step's remaining units and fuel at [`BODY_BYTES_PER_UNIT`] each, and at most half of the copy grant left after the allocation the previous call demanded, so that allocation still fits what the run leaves of the grant.
fn run_work(remaining_work: u64, fuel: u64, maximum_copy: usize, demanded: usize, unit_copy: usize) -> usize {
    let units = usize::try_from(remaining_work.min(fuel)).unwrap_or(usize::MAX);
    units.saturating_mul(BODY_BYTES_PER_UNIT).min(maximum_copy.saturating_sub(demanded) / (2 * unit_copy)).max(1)
}

/// 🧱️ Whether a demand no grant can meet: the same demand met the same `available` backing twice in a row, and that backing was the largest any call of this decoder was offered (`peak`), so a fresh turn would offer no more; a turn whose grant earlier calls already spent offers less than the peak and only yields.
fn refused_for_good(blocked: &mut Option<(PdfByteGraphEvent, usize)>, event: PdfByteGraphEvent, available: usize, peak: usize) -> bool {
    blocked.replace((event, available)) == Some((event, available)) && available >= peak
}

/// 🧱️ Whether `live` output bytes, less the `releasable` ones the decoder credits back before it completes, exceed the contract's `maximum`.
fn exceeds_ceiling(live: usize, releasable: usize, maximum: usize) -> bool { live.saturating_sub(releasable) > maximum }

fn failure(reason: &str) -> MediaError { MediaError::Payload(NATURAL_FILE_PORT.into(), reason.into()) }

#[cfg(test)]
mod tests {
    use super::*;

    /// 🧱️ LAW: a demand is refused for good only when the same demand failed twice against the largest backing ever offered; the institution diagram's lift asked for 65,536 released bytes after one turn had spent its release grant (0 offered, 1,048,576 at most), which used to fail the import and now yields to the next turn.
    #[test]
    fn a_demand_blocked_by_a_spent_turn_grant_yields_and_only_a_full_grant_refuses() {
        let release = PdfByteGraphEvent::NeedsRelease { bytes: 65_536 };
        let mut blocked = None;
        assert!(!refused_for_good(&mut blocked, release, 65_326, 1_048_576), "first block yields");
        assert!(!refused_for_good(&mut blocked, release, 0, 1_048_576), "a spent grant yields");
        assert!(!refused_for_good(&mut blocked, release, 0, 1_048_576), "the same spent grant again still yields in the same turn");
        assert!(!refused_for_good(&mut blocked, release, 1_048_576, 1_048_576), "a changed backing yields");
        assert!(refused_for_good(&mut blocked, release, 1_048_576, 1_048_576), "the same demand against the full grant twice is refused");
        let allocation = PdfByteGraphEvent::NeedsAllocation { bytes: 2 << 20 };
        let mut blocked = None;
        assert!(!refused_for_good(&mut blocked, allocation, 1 << 20, 1 << 20));
        assert!(refused_for_good(&mut blocked, allocation, 1 << 20, 1 << 20));
    }

    /// 🧱️ LAW: the decoder refuses an over-ceiling document as soon as the output it can no longer release exceeds the ceiling: backing it still credits back never counts, the ceiling itself is allowed, and the bytes are compared exactly.
    #[test]
    fn output_the_decoder_can_no_longer_release_is_refused_the_turn_it_passes_the_ceiling() {
        const MAXIMUM: usize = 32 << 20;
        assert!(!exceeds_ceiling(MAXIMUM, 0, MAXIMUM), "the ceiling itself is allowed");
        assert!(exceeds_ceiling(MAXIMUM + 1, 0, MAXIMUM), "one byte over is refused");
        assert!(!exceeds_ceiling(55_096_972, 55_096_972 - MAXIMUM, MAXIMUM), "backing credited back before completion never counts");
        assert!(exceeds_ceiling(55_096_972, 55_096_972 - MAXIMUM - 1, MAXIMUM), "what stays held does");
        assert!(!exceeds_ceiling(0, 1, MAXIMUM), "a release above the live bytes cannot underflow");
    }

    /// 📏️ The natural-file contract the registered route decodes under (`NATURAL_SOURCE_TOOL_CONTRACT`: 8,192 objects, 4,096 work units per step, 32 MiB of retained output).
    const NATURAL_CONTRACT: ToolExecutionContract = ToolExecutionContract::resumable(1, 8_192, 4_096, 33_554_432, 7_500, 1, 1);

    /// 📂️ The bytes of a ledger document, found by walking up from the crate to the first directory that holds its path.
    fn ledger_file(relative: &str) -> Vec<u8> {
        let start = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let found = start.ancestors().map(|directory| directory.join(relative)).find(|path| path.is_file()).unwrap_or_else(|| panic!("ledger document {relative} is not under {start:?}"));
        std::fs::read(found).unwrap()
    }

    /// 🧮️ How a model decode ended: the output bytes live at its end, the steps it took, and the pages lifted and steps taken when the decoder refused the document early.
    #[derive(Debug)]
    struct Decoded { live: usize, steps: usize, refused: Option<(usize, usize)> }

    /// 🧮️ Decodes `input` through the real decoder phases under the contract's ceiling and the importer's accounting of output bytes (what its steps allocate less what they release), without host turns: runs for body bytes and slot writes, every demand granted at once.
    fn decode_model(input: Vec<u8>, contract: ToolExecutionContract) -> Result<Decoded, String> {
        use PdfByteGraphEvent as Event;
        let mut decoder = NaturalPdfDecoder::new(contract);
        decoder.source = PdfNaturalSource::Contiguous(input);
        let mut grant = CosGrant { work: 1, allocation_bytes: 0, release_bytes: 0 };
        let (mut live, mut steps) = (0usize, 0usize);
        loop {
            let body = decoder.decodes_body_bytes();
            let written = if body { None } else { decoder.writing_bytes() };
            grant.work = if body { 4096.max(decoder.body_floor()) } else if written.is_some() { 4096 } else { 1 };
            steps += 1;
            let step = decoder.next(grant, body).map_err(|error| error.to_string())?;
            live = (live + step.allocated_bytes).checked_sub(step.released_bytes).ok_or("a step released backing it does not own")?;
            if let Some(releasable) = decoder.releasable_bytes().filter(|releasable| exceeds_ceiling(live, *releasable, contract.max_output_bytes)) {
                let pages = decoder.lifting.as_ref().map_or(0, |owner| owner.pages_lifted());
                return Ok(Decoded { live: live - releasable, steps, refused: Some((pages, steps)) });
            }
            match step.event {
                Ok(Event::NeedsAllocation { bytes }) => grant.allocation_bytes = bytes,
                Ok(Event::NeedsRelease { bytes }) => grant.release_bytes = bytes,
                Ok(Event::More | Event::Object) => { grant.allocation_bytes = 0; grant.release_bytes = 0; }
                Ok(Event::Complete) => return Ok(Decoded { live, steps, refused: None }),
                other => return Err(format!("{other:?}")),
            }
        }
    }

    /// 🧱️ LAW: every real-world document decodes within the natural-file contract's 32 MiB of retained output (the thesis retained 55,096,972 bytes before the role dependencies were stored as generators and the lanes charged what they retain), and the decoder refuses a document over its ceiling as soon as the output it cannot release exceeds it: the thesis under three quarters of its own output is refused well before its last page, with the output at refusal within 4 MiB of the ceiling.
    #[test]
    fn every_real_world_document_decodes_within_the_output_ceiling_and_an_oversize_one_is_refused_early() {
        let ledger: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🌍️real-world/🔣️.json")).unwrap();
        let mut report = Vec::new();
        let mut thesis = None;
        for case in ledger["documents"].as_array().unwrap() {
            let (name, input) = (case["name"].as_str().unwrap(), ledger_file(case["path"].as_str().unwrap()));
            let decoded = decode_model(input.clone(), NATURAL_CONTRACT).unwrap_or_else(|error| panic!("{name}: {error}"));
            assert_eq!(decoded.refused, None, "{name}: decoded within the ceiling");
            assert!(decoded.live <= NATURAL_CONTRACT.max_output_bytes, "{name}: retains {} output bytes, above {}", decoded.live, NATURAL_CONTRACT.max_output_bytes);
            report.push((name.to_owned(), input.len(), decoded.live, decoded.steps));
            if name == "bachelor thesis" { thesis = Some((input, decoded)); }
        }
        println!("output bytes (name, file, retained, steps): {report:?}");
        let (input, decoded) = thesis.expect("the ledger holds the bachelor thesis");
        let ceiling = decoded.live / 4 * 3;
        let refusing = ToolExecutionContract::resumable(1, 8_192, 4_096, ceiling, 7_500, 1, 1);
        let early = decode_model(input, refusing).unwrap();
        let (pages, steps) = early.refused.expect("a document over its ceiling is refused");
        assert!(pages < 65 && steps < decoded.steps, "refused after {pages} of 65 pages and {steps} of {} steps", decoded.steps);
        assert!(early.live > ceiling && early.live <= ceiling + (4 << 20), "the output at refusal is {} against a ceiling of {ceiling}", early.live);
    }

    /// 📏️ LAW: a bulk run leaves the allocation the previous call demanded fundable, so a large dictionary vector (the institution diagram's 278,528 bytes against a 524,175-byte grant) never stalls at one unit per turn.
    #[test]
    fn a_bulk_run_leaves_the_demanded_allocation_fundable() {
        let maximum = 524_175;
        for demanded in [0, 1, 4096, 278_528, 400_000, maximum - 4] {
            let work = run_work(u64::MAX, u64::MAX, maximum, demanded, BODY_COPY_PER_BYTE);
            assert!(work >= 1);
            assert!(maximum - work * BODY_COPY_PER_BYTE >= demanded.min(maximum - BODY_COPY_PER_BYTE), "demanded {demanded}: a run of {work} units leaves {}", maximum - work * BODY_COPY_PER_BYTE);
        }
        assert!(run_work(u64::MAX, u64::MAX, maximum, 278_528, BODY_COPY_PER_BYTE) < run_work(u64::MAX, u64::MAX, maximum, 0, BODY_COPY_PER_BYTE));
    }
}

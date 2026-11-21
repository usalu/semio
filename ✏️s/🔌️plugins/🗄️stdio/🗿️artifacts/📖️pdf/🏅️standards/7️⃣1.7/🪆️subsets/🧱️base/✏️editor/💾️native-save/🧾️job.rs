//! 💾️ Registered Original PDF binary Save job for the PDF 1.7 editor.

use std::{mem::ManuallyDrop, ptr::NonNull};
use semio_plugin::ArtifactApp;
use semio_framework_job_publication::{JobPublicationKind, RetainedJobPublication};
use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind,
    retirement::controlled::ControlledRetirement};
use semio_os_kernel::os_store::snapshot_read::settle_close_poll;
use semio_plugin::{ArtifactMediaExportJobRequest, ArtifactOutputChunks, ArtifactReservedJob, MediaConstruction,
    app::{ArtifactMediaExportRequestRetirement, plugin_grant_funds, plugin_typed_owner_demand, plugin_typed_owner_original},
    __job::{Generation, InteractiveJob, InteractiveJobCloseStep, InteractiveJobStep, JobOutcomeBorrow, JobOutcomeDescriptor,
        JobOutcomeKind, JobOutcomeView, OperationId, OriginalConstructionDemand, OriginalJobReturnClaim, OriginalSourceBinding,
        OriginalTurnFailure, StepContext, close_original_retained_effect_step}};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use super::graph::PinnedPdfGraph;
use super::{close::ProgramClose, producer::{FORMAT, PORT, PdfSaveProducer, TOOL},
    program::{ClassicPdfSaveProgram, admit_retained_candidate}};

type Request<A> = ArtifactMediaExportJobRequest<A>;
type RequestClose<A> = ArtifactMediaExportRequestRetirement<A>;

const CHECKPOINT_BYTES: usize = 64 * 1024;

pub struct PdfGraphSaveJob<A: ArtifactApp<Snapshot = PdfSnapshot>> {
    source: (OperationId, Generation),
    binding: Option<OriginalSourceBinding>,
    request: ManuallyDrop<Option<Request<A>>>,
    request_close: ManuallyDrop<Option<RequestClose<A>>>,
    candidate: ManuallyDrop<Option<PinnedPdfGraph>>,
    candidate_close: ManuallyDrop<Option<ControlledRetirement<PinnedPdfGraph>>>,
    candidate_birth: MediaConstruction,
    program_birth: MediaConstruction,
    body_ready: bool,
    candidate_ready: bool,
    program_ready: bool,
    program: ManuallyDrop<Option<Box<ClassicPdfSaveProgram>>>,
    program_close: ManuallyDrop<Option<ProgramClose>>,
    producer: PdfSaveProducer,
    publication: RetainedJobPublication,
    checkpoint: Option<usize>,
    last_checkpoint: usize,
    checkpoint_delivered: bool,
    complete: bool,
    closing: bool,
}

impl<A: ArtifactApp<Snapshot = PdfSnapshot>> PdfGraphSaveJob<A> {
    fn new(source: (OperationId, Generation)) -> Self {
        Self { source, binding: None, request: Default::default(), request_close: Default::default(),
            candidate: Default::default(), candidate_close: Default::default(), candidate_birth: Default::default(),
            body_ready: false, candidate_ready: false, program_ready: false,
            program_birth: Default::default(), program: Default::default(), program_close: Default::default(),
            producer: Default::default(), publication: RetainedJobPublication::new(),
            checkpoint: None, last_checkpoint: 0, checkpoint_delivered: false, complete: false, closing: false }
    }

    /// 📥️ Admits the body first and donates exactly the original request only after its body settles.
    pub fn admit_original(
        request: &mut Option<Request<A>>, body: &mut Option<Box<dyn ArtifactReservedJob + Send>>,
        construction: &mut MediaConstruction, cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        cx.original_source_parent()?;
        if cx.should_yield() || cx.is_cancelled() { return Ok(false); }
        let body_ready = match body.as_mut() {
            None => false,
            Some(owner) => (&mut **owner as &mut dyn std::any::Any).downcast_mut::<Self>()
                .ok_or_else(|| invalid("PDF Save requires its exact body"))?.body_ready,
        };
        if !body_ready {
            let source = request.as_ref().ok_or_else(|| invalid("PDF Save requires its original request"))?;
            if source.port != PORT || source.tool_id != TOOL || !source.snapshot_close.can_release(&source.snapshot) {
                return Err(invalid("PDF Save received a foreign result port or close lease").into());
            }
            let identity = source.source_identity;
            let size = size_of::<Self>();
            let ready = construction.built(cx, body, birth(size), || {
                let pointer = allocate::<Self>()?;
                let retained: Box<dyn ArtifactReservedJob + Send> = unsafe {
                    pointer.as_ptr().write(Self::new(identity));
                    Box::from_raw(pointer.as_ptr())
                };
                Ok((retained, admitted(size)))
            })?;
            if ready {
                let owner = (&mut **body.as_mut().expect("settled PDF body") as &mut dyn std::any::Any)
                    .downcast_mut::<Self>().expect("constructed PDF body");
                owner.body_ready = true;
            }
            return Ok(false);
        }
        let owner = (&mut **body.as_mut().expect("admitted PDF body") as &mut dyn std::any::Any)
            .downcast_mut::<Self>().ok_or_else(|| invalid("PDF Save requires its exact body"))?;
        if owner.request.is_some() {
            return if request.is_none() { Ok(true) } else { Err(invalid("PDF Save cannot overwrite an original request").into()) };
        }
        let bytes = size_of::<Request<A>>();
        if !funded(cx, copy(bytes)) { return Ok(false); }
        let source = request.take().ok_or_else(|| invalid("PDF Save request was not donated"))?;
        *owner.request = Some(source);
        moved(cx, bytes, 0)?;
        Ok(false)
    }

    fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        cx.original_source_parent()?;
        let binding = *self.binding.get_or_insert_with(|| OriginalSourceBinding::borrow(cx));
        let original = self.request.as_ref().ok_or_else(|| invalid("PDF Save has no admitted request"))?;
        if !binding.matches(cx) || self.source != (cx.operation(), cx.generation())
            || self.source != original.source_identity || !original.snapshot_close.can_release(&original.snapshot)
        {
            return Err(invalid("PDF Save lost its pinned revision owner").into());
        }
        if self.program_ready {
            let program = self.program.as_mut().expect("settled PDF program");
            self.complete = self.producer.advance(original, program.as_mut(), cx)?;
            let bytes = original.output_chunks.bytes();
            if !self.complete && bytes.saturating_sub(self.last_checkpoint) >= CHECKPOINT_BYTES {
                self.checkpoint = Some(bytes);
            }
            return Ok(());
        }
        if PinnedPdfGraph::retained_control_extent(&original.snapshot).is_none() {
            self.complete = self.producer.decline(original, cx)?;
            return Ok(());
        }
        if !self.candidate_ready {
            self.candidate_ready = admit_retained_candidate(
                &original.snapshot, original.canonical_base_revision, &mut self.candidate, &mut self.candidate_birth, cx)?;
            return Ok(());
        }
        let maximum = ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES - FORMAT.len();
        let bytes = size_of::<ClassicPdfSaveProgram>();
        if !funded(cx, RetirementDemand { capacity_bytes: bytes, ..copy(bytes) }) {
            return Err(invalid("PDF Save program exceeds the admitted body frame").into());
        }
        self.program_ready = self.program_birth.built(cx, &mut self.program, birth(bytes), || {
            let pointer = allocate::<ClassicPdfSaveProgram>()?;
            let graph = self.candidate.take().expect("funded PDF graph receiver");
            let program = unsafe {
                pointer.as_ptr().write(ClassicPdfSaveProgram::new(graph, maximum));
                Box::from_raw(pointer.as_ptr())
            };
            Ok((program, admitted(bytes)))
        })?;
        Ok(())
    }

    fn step_turn<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        if self.closing || cx.is_cancelled() { return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into); }
        if self.complete { return JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into); }
        if cx.should_yield() { return Ok(None); }
        if self.checkpoint_delivered {
            let demand = self.publication.retirement_demands()?;
            close_original_retained_effect_step(cx, demand, |effect| self.publication.close_step(effect).result)?;
            if self.publication.terminal_is_empty() {
                self.checkpoint_delivered = false;
                self.last_checkpoint = self.checkpoint.take().expect("delivered PDF checkpoint");
            }
            return Ok(None);
        }
        if let Some(bytes) = self.checkpoint {
            let result = self.publication.advance_from_source(JobPublicationKind::Checkpoint { applied_progress: bytes as u64 },
                b"{\"stage\":\"pdf-save\"}", cx)?;
            if result.is_some() { self.checkpoint_delivered = true; }
            return Ok(result);
        }
        cx.set_stage("pdf-save");
        self.advance(cx)?;
        Ok(None)
    }

    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.publication.terminal_is_empty() { return self.publication.retirement_demands(); }
        if !self.producer.terminal_is_empty() { return self.producer.retirement_demands(body); }
        if !self.program_birth.terminal_is_empty() { return self.program_birth.close_demand(body); }
        if !self.candidate_birth.terminal_is_empty() { return self.candidate_birth.close_demand(body); }
        if self.program.is_some() {
            return Ok(RetirementDemand { release_bytes: size_of::<ClassicPdfSaveProgram>(),
                ..copy(size_of::<ClassicPdfSaveProgram>()) });
        }
        if let Some(close) = self.program_close.as_ref() { return close.demands(body); }
        if self.candidate.is_some() || self.candidate_close.is_some() {
            return plugin_typed_owner_demand(&self.candidate, &self.candidate_close, body);
        }
        if self.request.is_some() { return Ok(copy(size_of::<RequestClose<A>>())); }
        if let Some(close) = self.request_close.as_ref().filter(|owner| !owner.terminal_is_empty()) {
            return close.retirement_demands(body);
        }
        Ok(copy(0))
    }

    fn close_next(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        let demand = self.close_demand(cx.retained_grant().maximum_copy_bytes)?;
        if !self.publication.terminal_is_empty() {
            return close_original_retained_effect_step(cx, demand, |effect| self.publication.close_step(effect).result);
        }
        if !self.producer.terminal_is_empty() { return self.producer.close_step(cx); }
        if let Some(progress) = self.program_birth.close_step(cx)? { return Ok(InteractiveJobCloseStep::Pending { progress }); }
        if let Some(progress) = self.candidate_birth.close_step(cx)? { return Ok(InteractiveJobCloseStep::Pending { progress }); }
        if let Some(close) = self.program_close.as_mut() {
            let step = close.close_step(cx)?;
            if close.terminal_is_empty() { self.program_close.take(); }
            return Ok(step);
        }
        close_original_retained_effect_step(cx, demand, |effect| {
            let grant = effect.grant();
            if self.program.is_some() || self.request.is_some() && self.candidate.is_none() && self.candidate_close.is_none() {
                if grant.maximum_items == 0 || !plugin_grant_funds(demand, grant) || !effect.performed(1)? {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                if let Some(program) = self.program.take() {
                    *self.program_close = Some(ProgramClose::new((*program).retire()));
                } else if let Some(request) = self.request.take() {
                    *self.request_close = Some(RequestClose::new(request));
                }
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {
                    copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default()
                }));
            }
            if self.candidate.is_some() || self.candidate_close.is_some() {
                return settle_close_poll(plugin_typed_owner_original(&mut self.candidate, &mut self.candidate_close, effect));
            }
            if let Some(close) = self.request_close.as_mut() {
                if !close.terminal_is_empty() { return settle_close_poll(close.close_step(effect)); }
                self.request_close.take();
            }
            Ok(RetainedCloneStep::Complete(Default::default()))
        })
    }
}

impl<A: ArtifactApp<Snapshot = PdfSnapshot>> InteractiveJob for PdfGraphSaveJob<A> {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>, _claim: &mut OriginalJobReturnClaim<'_>) -> InteractiveJobStep<'a> {
        InteractiveJobStep::from_callback(self.step_turn(cx))
    }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete if self.complete => descriptor.complete(None, None),
            JobOutcomeKind::Yield => descriptor.yielded(),
            _ => self.publication.borrow_outcome(descriptor),
        }
    }
    fn begin_close(&mut self) { self.closing = true; }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.close_demand(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.depth) }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        self.closing = true;
        let progress = self.close_next(cx)?.progress();
        Ok(if self.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress } }
            else { InteractiveJobCloseStep::Pending { progress } })
    }

    fn terminal_is_empty(&self) -> bool {
        self.publication.terminal_is_empty() && self.producer.terminal_is_empty()
            && self.program_birth.terminal_is_empty() && self.candidate_birth.terminal_is_empty()
            && self.program.is_none() && self.program_close.is_none() && self.candidate.is_none()
            && self.candidate_close.is_none() && self.request.is_none() && self.request_close.is_none()
    }
}

impl<A: ArtifactApp<Snapshot = PdfSnapshot>> ArtifactReservedJob for PdfGraphSaveJob<A> {}
impl<A: ArtifactApp<Snapshot = PdfSnapshot>> Drop for PdfGraphSaveJob<A> {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "PDF Save job requires paid retirement"); }
}

fn invalid(detail: &'static str) -> ValueError { ValueError::invariant(detail) }
fn birth(bytes: usize) -> OriginalConstructionDemand {
    OriginalConstructionDemand { items: 1, copy_bytes: bytes, capacity_bytes: bytes, release_bytes: 0, depth: 1 }
}
fn admitted(bytes: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: bytes, ..Default::default() }
}
fn copy(bytes: usize) -> RetirementDemand { RetirementDemand { copy_bytes: bytes, depth: 1, ..Default::default() } }
fn funded(cx: &StepContext<'_>, demand: RetirementDemand) -> bool {
    cx.retained_grant().maximum_items != 0 && plugin_grant_funds(demand, cx.retained_grant())
}
fn moved(cx: &mut StepContext<'_>, copied_bytes: usize, released_bytes: usize) -> Result<(), OriginalTurnFailure> {
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes, released_bytes, ..Default::default() })?;
    cx.consume_fuel(1);
    Ok(())
}
fn allocate<T>() -> Result<NonNull<T>, ValueError> {
    NonNull::new(unsafe { std::alloc::alloc(std::alloc::Layout::new::<T>()) }.cast::<T>())
        .ok_or_else(|| ValueError::literal(ValueRefusalKind::AllocationFailed, "PDF Save control allocation refused"))
}

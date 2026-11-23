//! 📤️ Resumable binary PDF export producer for the registered Original media job.

use std::{io::Write, mem::ManuallyDrop};
use semio_plugin::{ArtifactApp, ArtifactMediaExportJobRequest, ArtifactMediaExportResult, ArtifactOutputChunks, MediaClass, MediaError, MediaForm, MediaType, MediaConstruction};
use semio_plugin::__job::{InteractiveJobCloseStep, close_original_retained_effect_step, OriginalConstructionDemand, OriginalSourceBinding, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, RetirementDemand, ValueError};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_os_kernel::os_store::snapshot_read::settle_close_poll;
use semio_plugin::app::{plugin_typed_owner_demand, plugin_typed_owner_original};
use super::{page::{admit_pdf_save_page, donate_pdf_save_page}, window::write_pdf_original_turn};

pub const PORT: &str = "artifact:native";
pub const TOOL: &str = "export-media:artifact:native";
pub const FORMAT: &str = "s.stdio.pdf@1.7";
pub const MIME: &str = "application/pdf";
pub(crate) const REFERENCE_REQUIRED: &str = "the PDF object graph is outside the bounded native Save qualification and no reference writer is registered for Save";
pub(crate) const MEDIA_TYPE: MediaType = MediaType { class: MediaClass::Data, form: MediaForm::Document };

/// 📜️ The graph planner binds the authoritative source and proves coverage, total extent and the ordinary-object bound.
pub(crate) enum PlanDecision { Pending, Ready, ReferenceRequired }

pub(crate) trait PdfNativeSaveProgram<S> {
    fn prepare_original(&mut self, snapshot: &S, revision: [u8; 32], cx: &mut StepContext<'_>) -> Result<PlanDecision, OriginalTurnFailure>;
    fn emit_original(&mut self, snapshot: &S, sink: &mut dyn Write, grant: usize) -> PdfEmissionTurn;
}

pub(crate) struct PdfEmissionTurn {
    pub(crate) result: Result<bool, ValueError>,
    pub(crate) read: usize,
    pub(crate) work: usize,
}

#[derive(Default)]
pub struct PdfSaveProducer {
    pub(crate) page: ManuallyDrop<Option<Vec<u8>>>,
    pub(crate) pending: ManuallyDrop<Option<Result<ArtifactMediaExportResult, MediaError>>>,
    pending_close: ManuallyDrop<Option<ControlledRetirement<Result<ArtifactMediaExportResult, MediaError>>>>,
    page_close: ManuallyDrop<Option<ControlledRetirement<Vec<u8>>>>,
    pub(crate) construction: MediaConstruction,
    pub(crate) published: bool,
    binding: Option<OriginalSourceBinding>,
    ready: bool,
    page_ready: bool,
    result_ready: bool,
    encoded: bool,
    sealed: bool,
}

impl PdfSaveProducer {
    /// 📌️ Advances only the admitted request's Original source with its paired snapshot close supplier.
    pub(crate) fn advance<A: ArtifactApp>(
        &mut self,
        request: &ArtifactMediaExportJobRequest<A>,
        program: &mut impl PdfNativeSaveProgram<A::Snapshot>,
        cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        cx.original_source_parent()?;
        let binding = *self.binding.get_or_insert_with(|| OriginalSourceBinding::borrow(cx));
        if request.port != PORT || request.tool_id != TOOL
            || request.source_identity != (cx.operation(), cx.generation())
            || !request.snapshot_close.can_release(&request.snapshot) || !binding.matches(cx)
        {
            return Err(ValueError::invariant("PDF export lost its registered original source").into());
        }
        if cx.should_yield() || cx.is_cancelled() { return Ok(false); }
        if !self.ready {
            if self.published || self.pending.is_some() { return self.decline(request, cx); }
            match program.prepare_original(&request.snapshot, request.canonical_base_revision, cx)? {
                PlanDecision::Pending => return Ok(false),
                PlanDecision::Ready => self.ready = true,
                PlanDecision::ReferenceRequired => return self.decline(request, cx),
            }
        }
        if self.result_ready || self.published {
            return request.completion.complete_original(&mut self.pending, &mut self.published, cx);
        }
        if self.page_ready && self.page.as_ref().is_some_and(|page| !page.is_empty()
            && (self.encoded || page.len() == ArtifactOutputChunks::CHUNK_BYTES))
        {
            donate_pdf_save_page(&mut self.page, self.encoded, &request.output_chunks, Some(&request.output_credit), cx)?;
            self.page_ready = self.page.is_some();
            return Ok(false);
        }
        if self.encoded {
            if !self.sealed {
                if request.output_chunks.bytes() == 0 {
                    return Err(ValueError::invariant("PDF export emitted an empty file").into());
                }
                self.sealed = request.output_chunks.seal_original(cx)?;
                return Ok(false);
            }
            let capacity = FORMAT.len() + MIME.len();
            let bytes = size_of::<ArtifactMediaExportResult>() + capacity;
            self.result_ready = self.construction.built(cx, &mut self.pending, OriginalConstructionDemand {
                items: 1, copy_bytes: bytes, capacity_bytes: capacity, release_bytes: 0, depth: 1,
            }, || {
                let mut schema = String::new();
                let mut mime_type = String::new();
                schema.try_reserve_exact(FORMAT.len()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "PDF export metadata allocation refused"))?;
                mime_type.try_reserve_exact(MIME.len()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "PDF export metadata allocation refused"))?;
                schema.push_str(FORMAT);
                mime_type.push_str(MIME);
                request.output_credit.credit_original(FORMAT.len())?;
                let retained_capacity_bytes = schema.capacity() + mime_type.capacity();
                let result = ArtifactMediaExportResult {
                    media_type: MEDIA_TYPE,
                    schema,
                    mime_type,
                    chunks: request.output_chunks.clone(),
                };
                Ok((Ok(result), RetainedCloneProgress {
                    copied_items: 1, copied_bytes: bytes, retained_capacity_bytes, ..Default::default()
                }))
            })?;
            return Ok(false);
        }
        if !self.page_ready {
            self.page_ready = admit_pdf_save_page(&mut self.page, &mut self.construction, cx)?;
            return Ok(false);
        }
        let page = self.page.as_mut().ok_or_else(|| ValueError::invariant("PDF export lost its admitted page"))?;
        if let Some(result) = write_pdf_original_turn(page, cx, |sink, grant| {
            let turn = program.emit_original(&request.snapshot, sink, grant);
            let read = turn.read;
            (turn, read)
        })? {
            cx.consume_fuel(result.work as u64);
            self.encoded = result.result?;
        }
        Ok(false)
    }

    pub(crate) fn decline<A: ArtifactApp>(
        &mut self, request: &ArtifactMediaExportJobRequest<A>, cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        if self.ready || self.page.is_some() || self.encoded || self.sealed || request.output_chunks.bytes() != 0 {
            return Err(ValueError::invariant("PDF reference qualification reached the emission frontier").into());
        }
        if self.pending.is_none() && !self.published {
            if cx.should_yield() || cx.is_cancelled() { return Ok(false); }
            let capacity = PORT.len() + REFERENCE_REQUIRED.len();
            let bytes = size_of::<Result<ArtifactMediaExportResult, MediaError>>() + capacity;
            self.construction.built(cx, &mut self.pending, OriginalConstructionDemand {
                items: 1, copy_bytes: bytes, capacity_bytes: capacity, release_bytes: 0, depth: 1,
            }, || {
                let mut port = String::new();
                let mut reason = String::new();
                port.try_reserve_exact(PORT.len()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "PDF export refusal allocation refused"))?;
                reason.try_reserve_exact(REFERENCE_REQUIRED.len()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "PDF export refusal allocation refused"))?;
                port.push_str(PORT);
                reason.push_str(REFERENCE_REQUIRED);
                let retained_capacity_bytes = port.capacity() + reason.capacity();
                Ok((Err(MediaError::Payload(port, reason)), RetainedCloneProgress {
                    copied_items: 1, copied_bytes: bytes, retained_capacity_bytes, ..Default::default()
                }))
            })?;
            return Ok(false);
        }
        request.completion.complete_original(&mut self.pending, &mut self.published, cx)
    }

    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.construction.terminal_is_empty() {
            return self.construction.close_demand(body);
        }
        if self.pending.is_some() || self.pending_close.is_some() {
            return plugin_typed_owner_demand(&self.pending, &self.pending_close, body);
        }
        plugin_typed_owner_demand(&self.page, &self.page_close, body)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.construction.terminal_is_empty()
            && self.pending.is_none() && self.pending_close.is_none()
            && self.page.is_none() && self.page_close.is_none()
    }

    /// ♻️ Retires the partially emitted page and untransferred result on the original caller.
    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if let Some(progress) = self.construction.close_step(cx)? {
            return Ok(if self.terminal_is_empty() {
                InteractiveJobCloseStep::Complete { progress }
            } else {
                InteractiveJobCloseStep::Pending { progress }
            });
        }
        let demand = self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
        let step = close_original_retained_effect_step(cx, demand, |effect| {
            if self.pending.is_some() || self.pending_close.is_some() {
                return settle_close_poll(plugin_typed_owner_original(&mut self.pending, &mut self.pending_close, effect));
            }
            settle_close_poll(plugin_typed_owner_original(&mut self.page, &mut self.page_close, effect))
        })?;
        Ok(if self.terminal_is_empty() {
            InteractiveJobCloseStep::Complete { progress: step.progress() }
        } else {
            InteractiveJobCloseStep::Pending { progress: step.progress() }
        })
    }
}

impl Drop for PdfSaveProducer {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "PDF Save requires its original paid close");
    }
}

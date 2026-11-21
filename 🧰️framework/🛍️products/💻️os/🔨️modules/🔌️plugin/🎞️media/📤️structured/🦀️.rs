//! 📤️ Bounded structured media produced from a retained original artifact snapshot.

use semio_framework::{InteractiveJobClassification, MediaError, MediaType, ToolExecutionContract, ToolFactoryKey, ToolJobFactory};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, InteractiveJobStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, OriginalConstructionDemand, OriginalJobReturnClaim, OriginalSourceBinding, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep, RetirementDemand, RetirementPollExit, NativeEncodeControl, ValueError, ValueRefusalKind, retained_clone::RetainedCloneClose};
use crate::{ArtifactApp, ArtifactMediaExportJobRequest, ArtifactOwnedToolJobFactory, ArtifactReservedJob, ArtifactReservedToolJob, ArtifactToolPublicationContract, MediaConstruction};
use crate::app::{ArtifactMediaExportResult, ArtifactOutputChunks};
use semio_framework_pack_json::{JsonBorrowedWriteCursor, JsonWriteSource};
use crate::media_contract::ArtifactMediaExportRequestRetirement;

const MIME: &str = "application/json";
type Request<S> = ArtifactMediaExportJobRequest<<S as StructuredMediaSource>::App>;
type ResultPage = Result<ArtifactMediaExportResult, MediaError>;

const JSON_ACTIONS_PER_TURN: usize = 16;

/// 🎞️ Binds bounded borrowed media projection to an application's registered output ports.
pub trait StructuredMediaSource: 'static {
    type App: ArtifactApp;
    type Projection: Default + Send + semio_framework_value::RetireOwned;
    const TOOL_IDS: &'static [&'static str];
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract];
    const MAX_BYTES: usize;
    const PREPARATION_COPY_BYTES: usize;
    const OVERSIZE: &'static str = "media exceeds its bounded output size";
    fn descriptor(port: &str) -> Option<(MediaType, &'static str)>;
    fn prepare(port: &str, snapshot: &<Self::App as ArtifactApp>::Snapshot, projection: &mut Self::Projection) -> Result<bool, &'static str>;
    fn borrow<'a>(port: &str, snapshot: &'a <Self::App as ArtifactApp>::Snapshot, projection: &'a Self::Projection) -> Result<impl JsonWriteSource + 'a, ValueError>;
}

/// 🎟️ Registers each exact output in the application's normal tool registry.
pub struct StructuredMediaExportFactory<S>(Vec<ToolFactoryKey>, std::marker::PhantomData<fn() -> S>);
impl<S: StructuredMediaSource> StructuredMediaExportFactory<S> {
    pub fn new(controller: &str) -> Self {
        Self(S::TOOL_IDS.iter().map(|id| ToolFactoryKey::new(controller, *id)).collect(), std::marker::PhantomData)
    }
}
impl<S: StructuredMediaSource> ToolJobFactory for StructuredMediaExportFactory<S> {
    type Payload = ArtifactReservedToolJob;
    type Job = ArtifactReservedToolJob;
    fn keys(&self) -> &[ToolFactoryKey] { &self.0 }
    fn payload_schema_id(&self) -> &str { "semio.media.structured-export" }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(4_096, 1, JSON_ACTIONS_PER_TURN as u64, S::MAX_BYTES, 2_000, 64, 1)
    }
    crate::retained_command::reserved_tool_job_factory_admission!();
}
impl<S: StructuredMediaSource> ArtifactOwnedToolJobFactory for StructuredMediaExportFactory<S> {
    type Owner = S::App;
    const TOOL_IDS: &'static [&'static str] = S::TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = S::App::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = S::PUBLICATION_CONTRACTS;
}

#[derive(semio_framework_value::RetireOwned)]
struct Buffers<P: semio_framework_value::RetireOwned> {
    projection: P,
    source: Option<OriginalSourceBinding>,
    cursor: JsonBorrowedWriteCursor,
    prepared: bool,
    failure: Option<&'static str>,
    json: Option<Vec<u8>>,
    encoded: Option<(MediaType, String, Vec<u8>)>,
    page: Option<Vec<u8>>,
    result: Option<ResultPage>,
}

impl<P: Default + semio_framework_value::RetireOwned> Buffers<P> {
    fn new(schema_len: usize, output_limit: usize) -> Self {
        let mut cursor = JsonBorrowedWriteCursor::new();
        cursor.set_output_ceiling(output_limit.saturating_sub(schema_len));
        Self {
            projection: P::default(), source: None, cursor, prepared: false, failure: None,
            json: None, encoded: None, page: None, result: None,
        }
    }
}

/// 🏗️ Advances the exact mounted snapshot export through result and chunk publication.
pub struct StructuredMediaExport<S: StructuredMediaSource> {
    request: Option<Request<S>>,
    retirement: Option<ArtifactMediaExportRequestRetirement<S::App>>,
    buffers: Option<Buffers<S::Projection>>,
    buffer_close: RetainedCloneClose,
    position: usize,
    published: bool,
    closing: bool,
}

fn header(copy_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
}

#[path = "🔤️json/🦀️.rs"]
mod json;
use json::advance_projection;

impl<S: StructuredMediaSource> StructuredMediaExport<S> {
    pub fn admit(
        request: &mut Option<Request<S>>, output: &mut Option<Box<dyn ArtifactReservedJob + Send>>,
        construction: &mut MediaConstruction, cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        cx.original_source_parent()?;
        if cx.should_yield() || cx.is_cancelled() { return Ok(false); }
        if output.is_none() || construction.settling() {
            let request = request.as_ref().ok_or_else(|| ValueError::invariant("structured media export requires its original request"))?;
            if request.tool_id.strip_prefix("export-media:") != Some(request.port.as_str())
                || !S::TOOL_IDS.contains(&request.tool_id.as_str())
                || S::descriptor(&request.port).is_none()
            {
                return Err(ValueError::invariant("structured media export received a foreign port").into());
            }
            let bytes = size_of::<Self>();
            construction.built(cx, output, OriginalConstructionDemand {
                items: 1, copy_bytes: bytes, capacity_bytes: bytes, release_bytes: 0, depth: 1,
            }, || Ok((Box::new(Self {
                request: None, retirement: None, buffers: None, buffer_close: Default::default(),
                position: 0, published: false, closing: false,
            }), RetainedCloneProgress {
                copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: bytes, ..Default::default()
            })))?;
            return Ok(false);
        }
        let owner = (output.as_deref_mut().unwrap() as &mut dyn std::any::Any)
            .downcast_mut::<Self>().ok_or_else(|| ValueError::invariant("structured media export requires its exact retained body"))?;
        if owner.request.is_some() { return Ok(request.is_none()); }
        let original = request.as_ref().ok_or_else(|| ValueError::invariant("structured media export lost its original input"))?;
        let (_, schema) = S::descriptor(&original.port).ok_or_else(|| ValueError::invariant("structured media export received a foreign port"))?;
        let bytes = size_of::<Request<S>>() + size_of::<Buffers<S::Projection>>();
        if !cx.retained_work_can_enter(header(bytes)) { return Ok(false); }
        owner.request = Some(request.take().ok_or_else(|| ValueError::invariant("structured media export lost its original input"))?);
        owner.buffers = Some(Buffers::new(schema.len(), S::MAX_BYTES));
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }

    fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'_>>, OriginalTurnFailure> {
        if self.closing || cx.is_cancelled() { return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into); }
        if self.published { return JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into); }
        if cx.should_yield() { return Ok(None); }
        let request = self.request.as_ref().ok_or_else(|| ValueError::invariant("structured media export lost its source"))?;
        if request.source_identity != (cx.operation(), cx.generation()) {
            return Err(ValueError::invariant("structured media export request changed original operation").into());
        }
        let buffers = self.buffers.as_mut().ok_or_else(|| ValueError::invariant("structured media export lost its buffers"))?;
        if buffers.result.is_some() {
            request.completion.complete_original(&mut buffers.result, &mut self.published, cx)?;
        } else if buffers.encoded.is_none() {
            advance_projection::<S>(request, buffers, cx)?;
        } else if buffers.page.is_some() {
            request.output_chunks.push_original(&mut buffers.page, Some(&request.output_credit), cx)?;
        } else {
            cx.set_stage("structured-media-output");
            let (media_type, schema, source) = buffers.encoded.as_mut().unwrap();
            if self.position < source.len() {
                let end = (self.position + ArtifactOutputChunks::CHUNK_BYTES).min(source.len());
                let count = end - self.position;
                let copy_bytes = count + size_of::<Vec<u8>>();
                if !cx.retained_work_can_enter(RetirementDemand { capacity_bytes: count, ..header(copy_bytes) }) { return Ok(None); }
                buffers.page = Some(source[self.position..end].to_vec());
                self.position = end;
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, retained_capacity_bytes: count, ..Default::default() })?;
                cx.consume_fuel(1);
            } else if !request.output_chunks.is_sealed() {
                request.output_chunks.seal_original(cx)?;
            } else {
                let copy_bytes = size_of::<ResultPage>() + MIME.len();
                if !cx.retained_work_can_enter(RetirementDemand { capacity_bytes: MIME.len(), ..header(copy_bytes) }) { return Ok(None); }
                request.output_credit.credit_original(schema.len())?;
                buffers.result = Some(ArtifactMediaExportResult::structured(*media_type, std::mem::take(schema), MIME, request.output_chunks.clone())
                    .map_err(|fault| MediaError::Payload(request.port.clone(), fault.message)));
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, retained_capacity_bytes: MIME.len(), ..Default::default() })?;
                cx.consume_fuel(1);
            }
        }
        Ok(None)
    }

    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.buffers.is_some() || !self.buffer_close.is_empty() {
            return self.buffer_close.next_owner_demands::<Buffers<S::Projection>>(self.buffers.is_some(), &None, body);
        }
        if self.request.is_some() {
            return Ok(header(size_of::<Request<S>>() + size_of::<ArtifactMediaExportRequestRetirement<S::App>>()));
        }
        match &self.retirement {
            Some(retirement) if !retirement.terminal_is_empty() => retirement.retirement_demands(body),
            _ => Ok(header(0)),
        }
    }
}

impl<S: StructuredMediaSource> ArtifactReservedJob for StructuredMediaExport<S> {}

impl<S: StructuredMediaSource> InteractiveJob for StructuredMediaExport<S> {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>, _: &mut OriginalJobReturnClaim<'_>) -> InteractiveJobStep<'a> {
        InteractiveJobStep::from_callback(self.advance(cx))
    }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Yield => descriptor.yielded(),
            _ => descriptor.complete(None, None),
        }
    }
    fn begin_close(&mut self) { self.closing = true; }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.depth) }
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        self.begin_close();
        let body = cx.retained_grant().maximum_copy_bytes;
        let demand = self.close_demands(body)?;
        let step = semio_framework_job::close_original_effect_step(cx, demand, |effect| {
            if !self.buffer_close.is_empty() { return self.buffer_close.step_granted(body, effect); }
            if self.buffers.is_some() {
                let turn = self.buffer_close.begin_granted(&mut self.buffers, effect);
                return semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome {
                    work: turn.work,
                    result: turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Complete(Default::default()))).map_err(Into::into),
                });
            }
            if let Some(retirement) = self.retirement.as_mut() {
                if !retirement.terminal_is_empty() { return retirement.close_step(effect); }
            }
            let result = (|| {
                if !effect.performed(1)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                if let Some(request) = self.request.take() {
                    self.retirement = Some(ArtifactMediaExportRequestRetirement::new(request));
                } else {
                    self.retirement = None;
                }
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
            })();
            RetirementPollExit::settle(effect, result)
        })?;
        Ok(match step {
            InteractiveJobCloseStep::Blocked => InteractiveJobCloseStep::Blocked,
            _ if self.terminal_is_empty() => InteractiveJobCloseStep::Complete { progress: step.progress() },
            _ => InteractiveJobCloseStep::Pending { progress: step.progress() },
        })
    }
    fn terminal_is_empty(&self) -> bool {
        self.request.is_none() && self.retirement.is_none() && self.buffers.is_none() && self.buffer_close.is_empty()
    }
}

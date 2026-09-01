//! 🧊️ Browser WebGPU surface lifecycle over the owned A1 byte/page port.

use crate::abi::{AbiBytes, AbiControl, AbiErrorCode, AbiHandle, AbiMessage, AbiPage, AbiPort, AbiPortPoll, AbiRequest, AbiRequestId, AbiWorkBudget, ABI_MAX_PAGES_PER_TRANSFER};

//#region 🧬️Contract

pub const GPU_SURFACE_OPERATION_CREATE: u16 = 1_825;
pub const GPU_SURFACE_OPERATION_RESIZE: u16 = 1_826;
pub const GPU_SURFACE_OPERATION_FRAME: u16 = 1_827;
pub const GPU_SURFACE_OPERATION_DEVICE_LOSS: u16 = 1_828;
pub const GPU_SURFACE_OPERATION_RECOVER: u16 = 1_829;
pub const GPU_SURFACE_OPERATION_DROP: u16 = 1_830;
pub const GPU_MAX_SURFACE_SESSIONS: usize = 8;
pub const GPU_MAX_IN_FLIGHT_FRAMES: usize = 4;
pub const GPU_MAX_IN_FLIGHT_PAGES: usize = 8;
pub const GPU_MAX_IN_FLIGHT_CONTROLS: usize = 8;
pub const GPU_MAX_FRAME_BYTES: usize = 4_096;
pub const GPU_MAX_OUTCOME_BYTES: usize = 8_192;
pub const GPU_MAX_CALLBACK_WORK_UNITS: usize = 64;
pub const GPU_CALLBACK_DEADLINE_MILLISECONDS: u64 = 8;
pub const GPU_MAX_PORT_MESSAGE_BYTES: usize = GPU_MAX_FRAME_BYTES + 64;
pub const GPU_SURFACE_SCHEMA_JSON: &str = include_str!("../../🧬️schema/🔣️surface-port.json");
pub const GPU_SURFACE_TRACE_FIXTURE: &str = include_str!("../../🧪️fixtures/📒️surface-port.tsv");
pub const GPU_SURFACE_LIMITS_FIXTURE: &str = include_str!("../../🧪️fixtures/📐️surface-port-limits.tsv");

/// 🪪 Stable browser surface identity; its value is also the owned raw-canvas selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SurfaceId(u32);

impl SurfaceId {
    pub fn try_new(value: u32) -> Result<Self, AbiErrorCode> {
        if value == 0 {
            Err(AbiErrorCode::UnknownHandle)
        } else {
            Ok(Self(value))
        }
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

/// ♻️ Non-zero generation guarding every surface operation and outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SurfaceGeneration(u32);

impl SurfaceGeneration {
    pub fn try_new(value: u32) -> Result<Self, AbiErrorCode> {
        if value == 0 {
            Err(AbiErrorCode::StaleGeneration)
        } else {
            Ok(Self(value))
        }
    }

    pub const fn get(self) -> u32 {
        self.0
    }

    fn next(self) -> Result<Self, AbiErrorCode> {
        self.0.checked_add(1).map(Self).ok_or(AbiErrorCode::GenerationExhausted)
    }
}

/// 📐 Owned physical canvas dimensions; zero width or height deliberately parks a surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CanvasMetrics {
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
}

impl CanvasMetrics {
    pub fn try_new(width: u32, height: u32, scale_factor: f32) -> Result<Self, AbiErrorCode> {
        if !scale_factor.is_finite() || scale_factor <= 0.0 {
            Err(AbiErrorCode::MalformedLength)
        } else {
            Ok(Self { width, height, scale_factor })
        }
    }

    pub const fn is_parked(self) -> bool {
        self.width == 0 || self.height == 0
    }
}

/// 🧭 Canvas and adapter validation performed only by the JavaScript/A2 owner.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuHostStatus {
    Ready = 0,
    MissingCanvas = 1,
    BadCanvas = 2,
    UnsupportedAdapter = 3,
}

/// 🚨 Explicit browser GPU loss classification.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuLossReason {
    Surface = 1,
    Device = 2,
    Timeout = 3,
}

/// 🧯 Owned surface-domain failure without browser or GPU SDK values.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuErrorCode {
    MalformedEnvelope = 1,
    MissingCanvas = 2,
    BadCanvas = 3,
    UnsupportedAdapter = 4,
    UnknownSurface = 5,
    StaleGeneration = 6,
    SurfaceLost = 7,
    DeviceLost = 8,
    Busy = 9,
    LimitExceeded = 10,
    Cancelled = 11,
    Closed = 12,
    GenerationExhausted = 13,
}

/// 📬 Paged outcome emitted for every accepted, rejected, cancelled, or lost request.
#[derive(Clone, Debug, PartialEq)]
pub enum GpuOutcome {
    Created { request_id: AbiRequestId, surface: SurfaceId, generation: SurfaceGeneration, metrics: CanvasMetrics },
    Resized { request_id: AbiRequestId, surface: SurfaceId, generation: SurfaceGeneration, metrics: CanvasMetrics },
    FrameAccepted { request_id: AbiRequestId, surface: SurfaceId, generation: SurfaceGeneration, frame_id: u64, payload_bytes: u32 },
    Lost { request_id: AbiRequestId, surface: SurfaceId, generation: SurfaceGeneration, reason: GpuLossReason },
    Recovered { request_id: AbiRequestId, surface: SurfaceId, previous: SurfaceGeneration, generation: SurfaceGeneration, metrics: CanvasMetrics },
    Dropped { request_id: AbiRequestId, surface: SurfaceId, generation: SurfaceGeneration },
    Cancelled { request_id: AbiRequestId, surface: SurfaceId, generation: SurfaceGeneration },
    Rejected { request_id: AbiRequestId, surface: SurfaceId, generation: SurfaceGeneration, code: GpuErrorCode },
}

/// ⏳ One bounded callback step.
#[derive(Clone, Debug, PartialEq)]
pub enum GpuStep {
    Progress { completed_units: usize, total_units: usize },
    AwaitingHost,
    AwaitingAcknowledgement,
    Outcome(GpuOutcome),
    PageSent,
    Closing { remaining_resources: usize },
    TerminalEmpty,
}

//#endregion 🧬️Contract

//#region 🚧️Admission

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GpuAdmissionLedger {
    sessions: usize,
    frames: usize,
    pages: usize,
    controls: usize,
}

impl GpuAdmissionLedger {
    pub const fn sessions(self) -> usize {
        self.sessions
    }
    pub const fn frames(self) -> usize {
        self.frames
    }
    pub const fn pages(self) -> usize {
        self.pages
    }
    pub const fn controls(self) -> usize {
        self.controls
    }

    pub fn try_admit_session(&mut self) -> Result<(), AbiErrorCode> {
        admit(&mut self.sessions, GPU_MAX_SURFACE_SESSIONS)
    }
    pub fn try_admit_frame(&mut self) -> Result<(), AbiErrorCode> {
        admit(&mut self.frames, GPU_MAX_IN_FLIGHT_FRAMES)
    }
    pub fn try_admit_page(&mut self) -> Result<(), AbiErrorCode> {
        admit(&mut self.pages, GPU_MAX_IN_FLIGHT_PAGES)
    }
    pub fn try_admit_control(&mut self) -> Result<(), AbiErrorCode> {
        admit(&mut self.controls, GPU_MAX_IN_FLIGHT_CONTROLS)
    }
    pub fn release_session(&mut self) {
        release(&mut self.sessions);
    }
    pub fn release_frame(&mut self) {
        release(&mut self.frames);
    }
    pub fn release_page(&mut self) {
        release(&mut self.pages);
    }
    pub fn release_control(&mut self) {
        release(&mut self.controls);
    }
}

fn admit(value: &mut usize, maximum: usize) -> Result<(), AbiErrorCode> {
    if *value == maximum {
        Err(AbiErrorCode::Busy)
    } else {
        *value += 1;
        Ok(())
    }
}

fn release(value: &mut usize) {
    *value = value.saturating_sub(1);
}

//#endregion 🚧️Admission

//#region 🔌️Adapter

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SurfaceHealth {
    Healthy,
    Lost(GpuLossReason),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SurfaceRecord {
    id: SurfaceId,
    canvas: u32,
    generation: SurfaceGeneration,
    metrics: CanvasMetrics,
    health: SurfaceHealth,
    in_flight_frames: usize,
}

struct PendingRequest {
    request: AbiRequest,
    inspected: usize,
}

struct OutstandingPage {
    handle: AbiHandle,
    index: u32,
    request_id: AbiRequestId,
    request_generation: u32,
    surface: SurfaceId,
    frame_owner: Option<AbiBytes>,
}

/// 🔌 Dependency-free A1 port adapter retaining every surface and page until explicit completion.
pub struct WebGpuSurfaceAdapter<P: AbiPort> {
    port: P,
    surfaces: Vec<SurfaceRecord>,
    pending_request: Option<PendingRequest>,
    pending_control: Option<AbiControl>,
    outbound: Option<AbiMessage>,
    pages: Vec<OutstandingPage>,
    acknowledged: Vec<(AbiHandle, u32)>,
    admission: GpuAdmissionLedger,
    next_page_index: u32,
    closing: bool,
}

impl<P: AbiPort> WebGpuSurfaceAdapter<P> {
    pub fn new(port: P) -> Self {
        Self {
            port,
            surfaces: Vec::with_capacity(GPU_MAX_SURFACE_SESSIONS),
            pending_request: None,
            pending_control: None,
            outbound: None,
            pages: Vec::with_capacity(GPU_MAX_IN_FLIGHT_PAGES),
            acknowledged: Vec::with_capacity(GPU_MAX_IN_FLIGHT_PAGES),
            admission: GpuAdmissionLedger::default(),
            next_page_index: 0,
            closing: false,
        }
    }

    pub const fn admission(&self) -> GpuAdmissionLedger {
        self.admission
    }

    pub fn surface_metrics(&self, surface: SurfaceId) -> Option<CanvasMetrics> {
        self.surfaces.iter().find(|record| record.id == surface).map(|record| record.metrics)
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.surfaces.is_empty() && self.pending_request.is_none() && self.pending_control.is_none() && self.outbound.is_none() && self.pages.is_empty()
    }

    pub fn advance(&mut self, budget: AbiWorkBudget) -> Result<GpuStep, AbiErrorCode> {
        let permitted = callback_permit(budget)?;
        if self.closing {
            return self.close_step(permitted);
        }
        if let Some(message) = self.outbound.take() {
            match self.port.try_send(message, bounded_budget(budget, permitted)) {
                Ok(()) => return Ok(GpuStep::PageSent),
                Err(rejected) => {
                    self.outbound = Some(rejected.message);
                    return Err(rejected.code);
                }
            }
        }
        if self.pending_control.is_some() {
            return self.process_control();
        }
        if self.pending_request.is_some() {
            return self.inspect_request(permitted);
        }
        match self.port.poll(bounded_budget(budget, 1))? {
            AbiPortPoll::Pending => Ok(GpuStep::AwaitingHost),
            AbiPortPoll::Closed => {
                self.begin_close();
                self.close_step(permitted)
            }
            AbiPortPoll::Message(AbiMessage::Request(request)) => {
                if request.generation == 0 || request.bytes.len() > maximum_command_bytes() {
                    return Err(AbiErrorCode::LimitExceeded);
                }
                self.pending_request = Some(PendingRequest { request, inspected: 0 });
                self.inspect_request(permitted)
            }
            AbiPortPoll::Message(AbiMessage::Control(control)) => {
                self.admission.try_admit_control()?;
                self.pending_control = Some(control);
                self.process_control()
            }
            AbiPortPoll::Message(_) => Err(AbiErrorCode::MalformedTag),
        }
    }

    fn inspect_request(&mut self, permitted: usize) -> Result<GpuStep, AbiErrorCode> {
        let pending = self.pending_request.as_mut().expect("request presence checked");
        let total = pending.request.bytes.len();
        pending.inspected += permitted.min(total.saturating_sub(pending.inspected));
        if pending.inspected < total {
            return Ok(GpuStep::Progress { completed_units: pending.inspected, total_units: total });
        }
        if self.admission.pages() == GPU_MAX_IN_FLIGHT_PAGES {
            return Ok(GpuStep::AwaitingAcknowledgement);
        }
        let request = self.pending_request.take().expect("request retained until complete").request;
        let outcome = match self.execute(&request) {
            Ok(outcome) => outcome,
            Err(code) => {
                let identity = decode_request_identity(request.operation.get(), request.bytes.as_slice()).unwrap_or_else(|_| fallback_identity(request.request_id, request.generation));
                self.rejected(&request, identity, gpu_error_from_abi(code))
            }
        };
        let request_id = request.request_id;
        let request_generation = request.generation;
        let frame_owner = matches!(outcome, GpuOutcome::FrameAccepted { .. }).then_some(request.bytes);
        self.queue_outcome(outcome.clone(), request_id, request_generation, frame_owner)?;
        Ok(GpuStep::Outcome(outcome))
    }

    fn execute(&mut self, request: &AbiRequest) -> Result<GpuOutcome, AbiErrorCode> {
        let operation = request.operation.get();
        let mut decoder = Decoder::new(request.bytes.as_slice());
        if decoder.u8()? != 1 {
            return Ok(self.rejected(request, decoder.identity(), GpuErrorCode::MalformedEnvelope));
        }
        match operation {
            GPU_SURFACE_OPERATION_CREATE => self.create(request, &mut decoder),
            GPU_SURFACE_OPERATION_RESIZE => self.resize(request, &mut decoder),
            GPU_SURFACE_OPERATION_FRAME => self.frame(request, &mut decoder),
            GPU_SURFACE_OPERATION_DEVICE_LOSS => self.lose(request, &mut decoder),
            GPU_SURFACE_OPERATION_RECOVER => self.recover(request, &mut decoder),
            GPU_SURFACE_OPERATION_DROP => self.drop_surface(request, &mut decoder),
            _ => Ok(self.rejected(request, decoder.identity(), GpuErrorCode::MalformedEnvelope)),
        }
    }

    fn create(&mut self, request: &AbiRequest, decoder: &mut Decoder<'_>) -> Result<GpuOutcome, AbiErrorCode> {
        let status = decode_host_status(decoder.u8()?)?;
        let surface = SurfaceId::try_new(decoder.u32()?)?;
        let canvas = decoder.u32()?;
        let generation = SurfaceGeneration::try_new(decoder.u32()?)?;
        let metrics = CanvasMetrics::try_new(decoder.u32()?, decoder.u32()?, decoder.f32()?)?;
        decoder.finish()?;
        let rejected = match status {
            GpuHostStatus::Ready if canvas == 0 => Some(GpuErrorCode::MissingCanvas),
            GpuHostStatus::Ready => None,
            GpuHostStatus::MissingCanvas => Some(GpuErrorCode::MissingCanvas),
            GpuHostStatus::BadCanvas => Some(GpuErrorCode::BadCanvas),
            GpuHostStatus::UnsupportedAdapter => Some(GpuErrorCode::UnsupportedAdapter),
        };
        if let Some(code) = rejected {
            return Ok(self.rejected(request, (surface, generation), code));
        }
        if self.surfaces.iter().any(|record| record.id == surface) {
            return Ok(self.rejected(request, (surface, generation), GpuErrorCode::Busy));
        }
        if self.admission.try_admit_session().is_err() {
            return Ok(self.rejected(request, (surface, generation), GpuErrorCode::LimitExceeded));
        }
        self.surfaces.push(SurfaceRecord { id: surface, canvas, generation, metrics, health: SurfaceHealth::Healthy, in_flight_frames: 0 });
        Ok(GpuOutcome::Created { request_id: request.request_id, surface, generation, metrics })
    }

    fn resize(&mut self, request: &AbiRequest, decoder: &mut Decoder<'_>) -> Result<GpuOutcome, AbiErrorCode> {
        let (surface, generation) = decoder.surface_identity()?;
        let metrics = CanvasMetrics::try_new(decoder.u32()?, decoder.u32()?, decoder.f32()?)?;
        decoder.finish()?;
        let Some(index) = self.surface_index(surface, generation) else {
            return Ok(self.rejected(request, (surface, generation), GpuErrorCode::StaleGeneration));
        };
        self.surfaces[index].metrics = metrics;
        Ok(GpuOutcome::Resized { request_id: request.request_id, surface, generation, metrics })
    }

    fn frame(&mut self, request: &AbiRequest, decoder: &mut Decoder<'_>) -> Result<GpuOutcome, AbiErrorCode> {
        let (surface, generation) = decoder.surface_identity()?;
        let frame_id = decoder.u64()?;
        let payload = decoder.bytes(GPU_MAX_FRAME_BYTES)?;
        decoder.finish()?;
        let Some(index) = self.surface_index(surface, generation) else {
            return Ok(self.rejected(request, (surface, generation), GpuErrorCode::StaleGeneration));
        };
        let code = match self.surfaces[index].health {
            SurfaceHealth::Healthy => None,
            SurfaceHealth::Lost(GpuLossReason::Surface) => Some(GpuErrorCode::SurfaceLost),
            SurfaceHealth::Lost(_) => Some(GpuErrorCode::DeviceLost),
        };
        if let Some(code) = code {
            return Ok(self.rejected(request, (surface, generation), code));
        }
        if self.admission.try_admit_frame().is_err() {
            return Ok(self.rejected(request, (surface, generation), GpuErrorCode::LimitExceeded));
        }
        self.surfaces[index].in_flight_frames += 1;
        Ok(GpuOutcome::FrameAccepted { request_id: request.request_id, surface, generation, frame_id, payload_bytes: payload.len() as u32 })
    }

    fn lose(&mut self, request: &AbiRequest, decoder: &mut Decoder<'_>) -> Result<GpuOutcome, AbiErrorCode> {
        let (surface, generation) = decoder.surface_identity()?;
        let reason = decode_loss_reason(decoder.u8()?)?;
        decoder.finish()?;
        let Some(index) = self.surface_index(surface, generation) else {
            return Ok(self.rejected(request, (surface, generation), GpuErrorCode::StaleGeneration));
        };
        self.surfaces[index].health = SurfaceHealth::Lost(reason);
        Ok(GpuOutcome::Lost { request_id: request.request_id, surface, generation, reason })
    }

    fn recover(&mut self, request: &AbiRequest, decoder: &mut Decoder<'_>) -> Result<GpuOutcome, AbiErrorCode> {
        let (surface, previous) = decoder.surface_identity()?;
        decoder.finish()?;
        let Some(index) = self.surface_index(surface, previous) else {
            return Ok(self.rejected(request, (surface, previous), GpuErrorCode::StaleGeneration));
        };
        if matches!(self.surfaces[index].health, SurfaceHealth::Healthy) {
            return Ok(self.rejected(request, (surface, previous), GpuErrorCode::Busy));
        }
        let generation = match previous.next() {
            Ok(value) => value,
            Err(_) => return Ok(self.rejected(request, (surface, previous), GpuErrorCode::GenerationExhausted)),
        };
        self.surfaces[index].generation = generation;
        self.surfaces[index].health = SurfaceHealth::Healthy;
        let metrics = self.surfaces[index].metrics;
        Ok(GpuOutcome::Recovered { request_id: request.request_id, surface, previous, generation, metrics })
    }

    fn drop_surface(&mut self, request: &AbiRequest, decoder: &mut Decoder<'_>) -> Result<GpuOutcome, AbiErrorCode> {
        let (surface, generation) = decoder.surface_identity()?;
        decoder.finish()?;
        let Some(index) = self.surface_index(surface, generation) else {
            return Ok(self.rejected(request, (surface, generation), GpuErrorCode::StaleGeneration));
        };
        if self.surfaces[index].in_flight_frames != 0 {
            return Ok(self.rejected(request, (surface, generation), GpuErrorCode::Busy));
        }
        self.surfaces.remove(index);
        self.admission.release_session();
        Ok(GpuOutcome::Dropped { request_id: request.request_id, surface, generation })
    }

    fn rejected(&self, request: &AbiRequest, identity: (SurfaceId, SurfaceGeneration), code: GpuErrorCode) -> GpuOutcome {
        GpuOutcome::Rejected { request_id: request.request_id, surface: identity.0, generation: identity.1, code }
    }

    fn surface_index(&self, surface: SurfaceId, generation: SurfaceGeneration) -> Option<usize> {
        self.surfaces.iter().position(|record| record.id == surface && record.generation == generation)
    }

    fn queue_outcome(&mut self, outcome: GpuOutcome, request_id: AbiRequestId, request_generation: u32, frame_owner: Option<AbiBytes>) -> Result<(), AbiErrorCode> {
        self.admission.try_admit_page()?;
        let Some(index) = (0..ABI_MAX_PAGES_PER_TRANSFER).map(|offset| (self.next_page_index + offset) % ABI_MAX_PAGES_PER_TRANSFER).find(|candidate| self.pages.iter().all(|page| page.index != *candidate)) else {
            self.admission.release_page();
            return Err(AbiErrorCode::Busy);
        };
        let (surface, generation) = outcome_identity(&outcome);
        let handle = AbiHandle::try_new(surface.get(), generation.get())?;
        let bytes = encode_outcome(&outcome);
        if bytes.len() > GPU_MAX_OUTCOME_BYTES {
            self.admission.release_page();
            return Err(AbiErrorCode::LimitExceeded);
        }
        let page = AbiPage::try_new(handle, index, bytes).map_err(|rejected| rejected.code)?;
        self.next_page_index = (index + 1) % ABI_MAX_PAGES_PER_TRANSFER;
        self.pages.push(OutstandingPage { handle, index, request_id, request_generation, surface, frame_owner });
        self.outbound = Some(AbiMessage::Page(page));
        Ok(())
    }

    fn process_control(&mut self) -> Result<GpuStep, AbiErrorCode> {
        let control = self.pending_control.take().expect("control presence checked");
        self.admission.release_control();
        match control {
            AbiControl::Acknowledge { handle, index } => {
                if self.acknowledged.contains(&(handle, index)) {
                    return Err(AbiErrorCode::DuplicateAcknowledgement);
                }
                let page = self.remove_page(handle, index)?;
                if self.acknowledged.len() == GPU_MAX_IN_FLIGHT_PAGES {
                    self.acknowledged.remove(0);
                }
                self.acknowledged.push((handle, index));
                self.release_page_owner(page);
                Ok(GpuStep::AwaitingHost)
            }
            AbiControl::Close { handle } => {
                let position = self.pages.iter().position(|page| page.handle == handle).ok_or(AbiErrorCode::UnknownHandle)?;
                let page = self.pages.remove(position);
                self.admission.release_page();
                self.release_page_owner(page);
                Ok(GpuStep::AwaitingHost)
            }
            AbiControl::Cancel { request_id, generation } => self.cancel_request(request_id, generation),
        }
    }

    fn cancel_request(&mut self, request_id: AbiRequestId, generation: u32) -> Result<GpuStep, AbiErrorCode> {
        if self.admission.pages() == GPU_MAX_IN_FLIGHT_PAGES {
            return Err(AbiErrorCode::Busy);
        }
        if self.pending_request.as_ref().is_some_and(|pending| pending.request.request_id == request_id && pending.request.generation == generation) {
            let request = self.pending_request.take().expect("matched pending request").request;
            let identity = decode_request_identity(request.operation.get(), request.bytes.as_slice()).unwrap_or(fallback_identity(request_id, generation));
            let outcome = GpuOutcome::Cancelled { request_id, surface: identity.0, generation: identity.1 };
            self.queue_outcome(outcome.clone(), request_id, generation, None)?;
            return Ok(GpuStep::Outcome(outcome));
        }
        let position = self.pages.iter().position(|page| page.request_id == request_id).ok_or(AbiErrorCode::LateReply)?;
        if self.pages[position].request_generation != generation {
            return Err(AbiErrorCode::StaleGeneration);
        }
        let old = self.pages.remove(position);
        let surface = old.surface;
        self.admission.release_page();
        self.release_page_owner(old);
        let surface_generation = SurfaceGeneration::try_new(generation)?;
        let outcome = GpuOutcome::Cancelled { request_id, surface, generation: surface_generation };
        self.queue_outcome(outcome.clone(), request_id, generation, None)?;
        Ok(GpuStep::Outcome(outcome))
    }

    fn remove_page(&mut self, handle: AbiHandle, index: u32) -> Result<OutstandingPage, AbiErrorCode> {
        let position = self.pages.iter().position(|page| page.handle == handle && page.index == index).ok_or_else(|| classify_page_handle(&self.pages, handle))?;
        self.admission.release_page();
        Ok(self.pages.remove(position))
    }

    fn release_page_owner(&mut self, page: OutstandingPage) {
        if page.frame_owner.is_some() {
            self.admission.release_frame();
            if let Some(record) = self.surfaces.iter_mut().find(|record| record.id == page.surface) {
                record.in_flight_frames = record.in_flight_frames.saturating_sub(1);
            }
        }
    }

    fn close_step(&mut self, permitted: usize) -> Result<GpuStep, AbiErrorCode> {
        if permitted == 0 {
            return Err(AbiErrorCode::NoCredit);
        }
        if self.outbound.take().is_some() {
            return Ok(GpuStep::Closing { remaining_resources: self.remaining_resources() });
        }
        if self.pending_request.take().is_some() {
            return Ok(GpuStep::Closing { remaining_resources: self.remaining_resources() });
        }
        if self.pending_control.take().is_some() {
            self.admission.release_control();
            return Ok(GpuStep::Closing { remaining_resources: self.remaining_resources() });
        }
        if let Some(page) = self.pages.pop() {
            self.admission.release_page();
            self.release_page_owner(page);
            return Ok(GpuStep::Closing { remaining_resources: self.remaining_resources() });
        }
        if self.surfaces.pop().is_some() {
            self.admission.release_session();
            return Ok(GpuStep::Closing { remaining_resources: self.remaining_resources() });
        }
        Ok(GpuStep::TerminalEmpty)
    }

    fn remaining_resources(&self) -> usize {
        self.surfaces.len() + self.pages.len() + usize::from(self.pending_request.is_some()) + usize::from(self.pending_control.is_some()) + usize::from(self.outbound.is_some())
    }
}

//#endregion 🔌️Adapter

//#region 🧱️Codec

fn maximum_command_bytes() -> usize {
    GPU_MAX_PORT_MESSAGE_BYTES - 20
}

fn callback_permit(budget: AbiWorkBudget) -> Result<usize, AbiErrorCode> {
    if budget.cancelled {
        return Err(AbiErrorCode::Cancelled);
    }
    if budget.interrupted {
        return Err(AbiErrorCode::Interrupted);
    }
    if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
        return Err(AbiErrorCode::DeadlineExceeded);
    }
    if budget.byte_credit == 0 {
        return Err(AbiErrorCode::NoCredit);
    }
    Ok(budget.byte_credit.min(GPU_MAX_CALLBACK_WORK_UNITS))
}

fn bounded_budget(mut budget: AbiWorkBudget, credit: usize) -> AbiWorkBudget {
    budget.byte_credit = credit.min(GPU_MAX_CALLBACK_WORK_UNITS);
    budget
}

fn decode_host_status(value: u8) -> Result<GpuHostStatus, AbiErrorCode> {
    match value {
        0 => Ok(GpuHostStatus::Ready),
        1 => Ok(GpuHostStatus::MissingCanvas),
        2 => Ok(GpuHostStatus::BadCanvas),
        3 => Ok(GpuHostStatus::UnsupportedAdapter),
        _ => Err(AbiErrorCode::MalformedTag),
    }
}

fn decode_loss_reason(value: u8) -> Result<GpuLossReason, AbiErrorCode> {
    match value {
        1 => Ok(GpuLossReason::Surface),
        2 => Ok(GpuLossReason::Device),
        3 => Ok(GpuLossReason::Timeout),
        _ => Err(AbiErrorCode::MalformedTag),
    }
}

fn fallback_identity(request_id: AbiRequestId, generation: u32) -> (SurfaceId, SurfaceGeneration) {
    let slot = u32::try_from(request_id.0).ok().filter(|value| *value != 0).unwrap_or(1);
    (SurfaceId(slot), SurfaceGeneration(generation.max(1)))
}

fn decode_request_identity(operation: u16, bytes: &[u8]) -> Result<(SurfaceId, SurfaceGeneration), AbiErrorCode> {
    let mut decoder = Decoder::new(bytes);
    decoder.u8()?;
    if operation == GPU_SURFACE_OPERATION_CREATE {
        decoder.u8()?;
        let surface = SurfaceId::try_new(decoder.u32()?)?;
        decoder.u32()?;
        Ok((surface, SurfaceGeneration::try_new(decoder.u32()?)?))
    } else {
        decoder.surface_identity()
    }
}

fn outcome_identity(outcome: &GpuOutcome) -> (SurfaceId, SurfaceGeneration) {
    match outcome {
        GpuOutcome::Created { surface, generation, .. }
        | GpuOutcome::Resized { surface, generation, .. }
        | GpuOutcome::FrameAccepted { surface, generation, .. }
        | GpuOutcome::Lost { surface, generation, .. }
        | GpuOutcome::Recovered { surface, generation, .. }
        | GpuOutcome::Dropped { surface, generation, .. }
        | GpuOutcome::Cancelled { surface, generation, .. }
        | GpuOutcome::Rejected { surface, generation, .. } => (*surface, *generation),
    }
}

fn encode_outcome(outcome: &GpuOutcome) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(40);
    bytes.push(1);
    let (tag, request_id, surface, generation) = match outcome {
        GpuOutcome::Created { request_id, surface, generation, .. } => (1, request_id, surface, generation),
        GpuOutcome::Resized { request_id, surface, generation, .. } => (2, request_id, surface, generation),
        GpuOutcome::FrameAccepted { request_id, surface, generation, .. } => (3, request_id, surface, generation),
        GpuOutcome::Lost { request_id, surface, generation, .. } => (4, request_id, surface, generation),
        GpuOutcome::Recovered { request_id, surface, generation, .. } => (5, request_id, surface, generation),
        GpuOutcome::Dropped { request_id, surface, generation } => (6, request_id, surface, generation),
        GpuOutcome::Cancelled { request_id, surface, generation } => (7, request_id, surface, generation),
        GpuOutcome::Rejected { request_id, surface, generation, .. } => (8, request_id, surface, generation),
    };
    bytes.push(tag);
    bytes.extend_from_slice(&request_id.0.to_le_bytes());
    bytes.extend_from_slice(&surface.get().to_le_bytes());
    bytes.extend_from_slice(&generation.get().to_le_bytes());
    match outcome {
        GpuOutcome::Created { metrics, .. } | GpuOutcome::Resized { metrics, .. } => {
            bytes.extend_from_slice(&metrics.width.to_le_bytes());
            bytes.extend_from_slice(&metrics.height.to_le_bytes());
            bytes.extend_from_slice(&metrics.scale_factor.to_bits().to_le_bytes());
        }
        GpuOutcome::FrameAccepted { frame_id, payload_bytes, .. } => {
            bytes.extend_from_slice(&frame_id.to_le_bytes());
            bytes.extend_from_slice(&payload_bytes.to_le_bytes());
        }
        GpuOutcome::Lost { reason, .. } => bytes.push(*reason as u8),
        GpuOutcome::Recovered { previous, metrics, .. } => {
            bytes.extend_from_slice(&previous.get().to_le_bytes());
            bytes.extend_from_slice(&metrics.width.to_le_bytes());
            bytes.extend_from_slice(&metrics.height.to_le_bytes());
            bytes.extend_from_slice(&metrics.scale_factor.to_bits().to_le_bytes());
        }
        GpuOutcome::Rejected { code, .. } => bytes.extend_from_slice(&(*code as u16).to_le_bytes()),
        GpuOutcome::Dropped { .. } | GpuOutcome::Cancelled { .. } => {}
    }
    bytes
}

fn classify_page_handle(pages: &[OutstandingPage], actual: AbiHandle) -> AbiErrorCode {
    let Some(page) = pages.iter().find(|page| page.handle.slot() == actual.slot()) else {
        return AbiErrorCode::UnknownHandle;
    };
    if actual.generation() < page.handle.generation() {
        AbiErrorCode::AbaHandle
    } else if actual.generation() > page.handle.generation() {
        AbiErrorCode::StaleGeneration
    } else {
        AbiErrorCode::OutOfOrderPage
    }
}

fn gpu_error_from_abi(code: AbiErrorCode) -> GpuErrorCode {
    match code {
        AbiErrorCode::StaleGeneration | AbiErrorCode::AbaHandle => GpuErrorCode::StaleGeneration,
        AbiErrorCode::LimitExceeded => GpuErrorCode::LimitExceeded,
        AbiErrorCode::Cancelled => GpuErrorCode::Cancelled,
        AbiErrorCode::Closed => GpuErrorCode::Closed,
        AbiErrorCode::GenerationExhausted => GpuErrorCode::GenerationExhausted,
        AbiErrorCode::Busy => GpuErrorCode::Busy,
        _ => GpuErrorCode::MalformedEnvelope,
    }
}

struct Decoder<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> Decoder<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }
    fn take(&mut self, count: usize) -> Result<&'a [u8], AbiErrorCode> {
        let end = self.cursor.checked_add(count).ok_or(AbiErrorCode::MalformedLength)?;
        let value = self.bytes.get(self.cursor..end).ok_or(AbiErrorCode::MissingField)?;
        self.cursor = end;
        Ok(value)
    }
    fn u8(&mut self) -> Result<u8, AbiErrorCode> {
        Ok(self.take(1)?[0])
    }
    fn u32(&mut self) -> Result<u32, AbiErrorCode> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().map_err(|_| AbiErrorCode::MissingField)?))
    }
    fn u64(&mut self) -> Result<u64, AbiErrorCode> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().map_err(|_| AbiErrorCode::MissingField)?))
    }
    fn f32(&mut self) -> Result<f32, AbiErrorCode> {
        Ok(f32::from_bits(self.u32()?))
    }
    fn bytes(&mut self, maximum: usize) -> Result<&'a [u8], AbiErrorCode> {
        let length = self.u32()? as usize;
        if length > maximum {
            return Err(AbiErrorCode::LimitExceeded);
        }
        self.take(length)
    }
    fn surface_identity(&mut self) -> Result<(SurfaceId, SurfaceGeneration), AbiErrorCode> {
        Ok((SurfaceId::try_new(self.u32()?)?, SurfaceGeneration::try_new(self.u32()?)?))
    }
    fn identity(&self) -> (SurfaceId, SurfaceGeneration) {
        let mut decoder = Decoder::new(self.bytes);
        decode_identity_with(&mut decoder).unwrap_or((SurfaceId(1), SurfaceGeneration(1)))
    }
    fn finish(&self) -> Result<(), AbiErrorCode> {
        if self.cursor == self.bytes.len() {
            Ok(())
        } else {
            Err(AbiErrorCode::MalformedLength)
        }
    }
}

fn decode_identity_with(decoder: &mut Decoder<'_>) -> Result<(SurfaceId, SurfaceGeneration), AbiErrorCode> {
    decoder.u8()?;
    decoder.surface_identity()
}

//#endregion 🧱️Codec

//#region 🧠️LinearMemory

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[link(wasm_import_module = "semio_webgpu_surface")]
extern "C" {
    fn send(pointer: *const u8, length: usize) -> u32;
    fn poll(pointer: *mut u8, capacity: usize) -> u32;
}

/// 🧠 Generated-import implementation translating only linear-memory pointer/length pairs.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub struct LinearMemoryWebGpuSurfacePort;

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl AbiPort for LinearMemoryWebGpuSurfacePort {
    fn try_send(&mut self, message: AbiMessage, budget: AbiWorkBudget) -> Result<(), crate::abi::AbiPortRejection> {
        if let Err(code) = callback_permit(budget) {
            return Err(crate::abi::AbiPortRejection { code, message });
        }
        let bytes = crate::abi::encode_abi_message(&message);
        if bytes.len() > GPU_MAX_PORT_MESSAGE_BYTES {
            return Err(crate::abi::AbiPortRejection { code: AbiErrorCode::LimitExceeded, message });
        }
        let status = unsafe { send(bytes.as_ptr(), bytes.len()) };
        if status == 0 {
            Ok(())
        } else {
            Err(crate::abi::AbiPortRejection { code: decode_import_error(status), message })
        }
    }

    fn poll(&mut self, budget: AbiWorkBudget) -> Result<AbiPortPoll, AbiErrorCode> {
        callback_permit(budget)?;
        let mut bytes = vec![0; GPU_MAX_CALLBACK_WORK_UNITS];
        let mut length = unsafe { poll(bytes.as_mut_ptr(), bytes.len()) };
        if length == 0 {
            return Ok(AbiPortPoll::Pending);
        }
        if length == u32::MAX {
            return Ok(AbiPortPoll::Closed);
        }
        if length as usize > GPU_MAX_PORT_MESSAGE_BYTES {
            return Err(AbiErrorCode::LimitExceeded);
        }
        if length as usize > bytes.len() {
            bytes.resize(length as usize, 0);
            length = unsafe { poll(bytes.as_mut_ptr(), bytes.len()) };
            if length as usize > bytes.len() {
                return Err(AbiErrorCode::LimitExceeded);
            }
        }
        bytes.truncate(length as usize);
        crate::abi::decode_abi_message(&bytes).map(AbiPortPoll::Message)
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
fn decode_import_error(value: u32) -> AbiErrorCode {
    match value {
        5 => AbiErrorCode::LimitExceeded,
        7 => AbiErrorCode::UnknownHandle,
        8 => AbiErrorCode::StaleGeneration,
        9 => AbiErrorCode::AbaHandle,
        10 => AbiErrorCode::DuplicateAcknowledgement,
        11 => AbiErrorCode::Interrupted,
        12 => AbiErrorCode::Cancelled,
        17 => AbiErrorCode::DeadlineExceeded,
        18 => AbiErrorCode::NoCredit,
        19 => AbiErrorCode::Busy,
        20 => AbiErrorCode::Closed,
        _ => AbiErrorCode::MalformedTag,
    }
}

//#endregion 🧠️LinearMemory

//#region 🧪️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::{AbiOperation, AbiPageBytes, AbiPortRejection};
    use std::collections::VecDeque;

    #[derive(Default)]
    struct MockPort {
        incoming: VecDeque<AbiMessage>,
        sent: Vec<AbiMessage>,
        reject_send: bool,
        closed: bool,
    }

    impl AbiPort for MockPort {
        fn try_send(&mut self, message: AbiMessage, _budget: AbiWorkBudget) -> Result<(), AbiPortRejection> {
            if self.reject_send {
                self.reject_send = false;
                Err(AbiPortRejection { code: AbiErrorCode::Interrupted, message })
            } else {
                self.sent.push(message);
                Ok(())
            }
        }
        fn poll(&mut self, _budget: AbiWorkBudget) -> Result<AbiPortPoll, AbiErrorCode> {
            Ok(self.incoming.pop_front().map(AbiPortPoll::Message).unwrap_or(if self.closed { AbiPortPoll::Closed } else { AbiPortPoll::Pending }))
        }
    }

    fn request(operation: u16, id: u64, generation: u32, body: Vec<u8>) -> AbiMessage {
        AbiMessage::Request(AbiRequest { operation: AbiOperation::try_new(operation).unwrap(), request_id: AbiRequestId(id), generation, bytes: AbiBytes::try_new(body).unwrap() })
    }

    fn create_body(status: u8, surface: u32, canvas: u32, generation: u32, width: u32, height: u32) -> Vec<u8> {
        let mut body = vec![1, status];
        for value in [surface, canvas, generation, width, height, 1.0f32.to_bits()] {
            body.extend_from_slice(&value.to_le_bytes());
        }
        body
    }

    fn identity_body(surface: u32, generation: u32) -> Vec<u8> {
        let mut body = vec![1];
        body.extend_from_slice(&surface.to_le_bytes());
        body.extend_from_slice(&generation.to_le_bytes());
        body
    }

    fn resize_body(surface: u32, generation: u32, width: u32, height: u32) -> Vec<u8> {
        let mut body = identity_body(surface, generation);
        for value in [width, height, 2.0f32.to_bits()] {
            body.extend_from_slice(&value.to_le_bytes());
        }
        body
    }

    fn frame_body(surface: u32, generation: u32, frame: u64, payload: Vec<u8>) -> Vec<u8> {
        let mut body = identity_body(surface, generation);
        body.extend_from_slice(&frame.to_le_bytes());
        body.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        body.extend(payload);
        body
    }

    fn loss_body(surface: u32, generation: u32, reason: u8) -> Vec<u8> {
        let mut body = identity_body(surface, generation);
        body.push(reason);
        body
    }

    fn run_request(adapter: &mut WebGpuSurfaceAdapter<MockPort>) -> GpuOutcome {
        loop {
            match adapter.advance(AbiWorkBudget::credits(64)).unwrap() {
                GpuStep::Outcome(outcome) => return outcome,
                GpuStep::Progress { .. } => {}
                other => panic!("unexpected {other:?}"),
            }
        }
    }

    fn push(adapter: &mut WebGpuSurfaceAdapter<MockPort>, message: AbiMessage) {
        adapter.port.incoming.push_back(message);
    }

    fn ack_latest(adapter: &mut WebGpuSurfaceAdapter<MockPort>) {
        let (handle, index) = adapter.pages.last().map(|page| (page.handle, page.index)).unwrap();
        push(adapter, AbiMessage::Control(AbiControl::Acknowledge { handle, index }));
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::PageSent);
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::AwaitingHost);
    }

    fn create(adapter: &mut WebGpuSurfaceAdapter<MockPort>, surface: u32, generation: u32) -> GpuOutcome {
        push(adapter, request(GPU_SURFACE_OPERATION_CREATE, surface as u64, generation, create_body(0, surface, surface, generation, 640, 480)));
        run_request(adapter)
    }

    #[test]
    fn schema_and_language_neutral_ledgers_declare_every_operation_and_limit() {
        for operation in ["create", "resize", "frame", "deviceLoss", "recover", "drop"] {
            assert!(GPU_SURFACE_SCHEMA_JSON.contains(operation));
        }
        let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
        for (index, line) in GPU_SURFACE_TRACE_FIXTURE.lines().filter(|line| !line.starts_with('#')).enumerate() {
            let fields: Vec<_> = line.split('\t').collect();
            let operation = fields[1].parse().unwrap();
            let generation = fields[2].parse().unwrap();
            let body = fields[3].as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect();
            push(&mut adapter, request(operation, index as u64 + 1, generation, body));
            let outcome = run_request(&mut adapter);
            let actual = match outcome {
                GpuOutcome::Created { .. } => "created",
                GpuOutcome::Resized { .. } => "resized",
                GpuOutcome::FrameAccepted { .. } => "frameAccepted",
                GpuOutcome::Lost { .. } => "lost",
                GpuOutcome::Recovered { .. } => "recovered",
                GpuOutcome::Dropped { .. } => "dropped",
                GpuOutcome::Cancelled { .. } => "cancelled",
                GpuOutcome::Rejected { .. } => "rejected",
            };
            assert_eq!(actual, fields[4]);
            ack_latest(&mut adapter);
        }
        let limits: Vec<_> = GPU_SURFACE_LIMITS_FIXTURE.lines().filter(|line| !line.starts_with('#')).map(|line| line.split('\t').collect::<Vec<_>>()).collect();
        assert_eq!(limits.len(), 8);
        for row in limits {
            assert_eq!(row[2].parse::<usize>().unwrap(), row[1].parse::<usize>().unwrap() + 1);
        }
    }

    #[test]
    fn create_resize_frame_and_drop_trace_is_ack_controlled() {
        let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
        assert!(matches!(create(&mut adapter, 1, 1), GpuOutcome::Created { .. }));
        ack_latest(&mut adapter);
        push(&mut adapter, request(GPU_SURFACE_OPERATION_RESIZE, 2, 1, resize_body(1, 1, 800, 600)));
        assert!(matches!(run_request(&mut adapter), GpuOutcome::Resized { metrics: CanvasMetrics { width: 800, height: 600, .. }, .. }));
        ack_latest(&mut adapter);
        push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 3, 1, frame_body(1, 1, 7, vec![1, 2, 3])));
        assert!(matches!(run_request(&mut adapter), GpuOutcome::FrameAccepted { frame_id: 7, payload_bytes: 3, .. }));
        ack_latest(&mut adapter);
        push(&mut adapter, request(GPU_SURFACE_OPERATION_DROP, 4, 1, identity_body(1, 1)));
        assert!(matches!(run_request(&mut adapter), GpuOutcome::Dropped { .. }));
        assert_eq!(adapter.admission.sessions(), 0);
    }

    #[test]
    fn zero_sized_surface_is_valid_and_parked() {
        let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
        push(&mut adapter, request(GPU_SURFACE_OPERATION_CREATE, 1, 1, create_body(0, 1, 1, 1, 0, 0)));
        let GpuOutcome::Created { metrics, .. } = run_request(&mut adapter) else { panic!() };
        assert!(metrics.is_parked());
    }

    #[test]
    fn bad_missing_and_unsupported_canvas_outcomes_do_not_replace_last_valid_surface() {
        let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
        assert!(matches!(create(&mut adapter, 1, 1), GpuOutcome::Created { .. }));
        ack_latest(&mut adapter);
        for (id, status, code) in [(2, 1, GpuErrorCode::MissingCanvas), (3, 2, GpuErrorCode::BadCanvas), (4, 3, GpuErrorCode::UnsupportedAdapter)] {
            push(&mut adapter, request(GPU_SURFACE_OPERATION_CREATE, id, 1, create_body(status, id as u32, id as u32, 1, 10, 10)));
            assert!(matches!(run_request(&mut adapter), GpuOutcome::Rejected { code: actual, .. } if actual == code));
            ack_latest(&mut adapter);
        }
        assert_eq!(adapter.surface_metrics(SurfaceId::try_new(1).unwrap()).unwrap().width, 640);
        assert_eq!(adapter.admission.sessions(), 1);
    }

    #[test]
    fn loss_stale_frame_and_deterministic_recovery_retain_metrics() {
        let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
        create(&mut adapter, 1, 1);
        ack_latest(&mut adapter);
        push(&mut adapter, request(GPU_SURFACE_OPERATION_DEVICE_LOSS, 2, 1, loss_body(1, 1, 2)));
        assert!(matches!(run_request(&mut adapter), GpuOutcome::Lost { reason: GpuLossReason::Device, .. }));
        ack_latest(&mut adapter);
        push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 3, 1, frame_body(1, 1, 1, Vec::new())));
        assert!(matches!(run_request(&mut adapter), GpuOutcome::Rejected { code: GpuErrorCode::DeviceLost, .. }));
        ack_latest(&mut adapter);
        push(&mut adapter, request(GPU_SURFACE_OPERATION_RECOVER, 4, 1, identity_body(1, 1)));
        assert!(matches!(run_request(&mut adapter), GpuOutcome::Recovered { previous: SurfaceGeneration(1), generation: SurfaceGeneration(2), .. }));
        ack_latest(&mut adapter);
        push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 5, 1, frame_body(1, 1, 2, Vec::new())));
        assert!(matches!(run_request(&mut adapter), GpuOutcome::Rejected { code: GpuErrorCode::StaleGeneration, .. }));
        assert_eq!(adapter.surface_metrics(SurfaceId(1)).unwrap().width, 640);
    }

    #[test]
    fn session_frame_page_and_control_admission_are_exact_at_max_and_max_plus_one() {
        let mut ledger = GpuAdmissionLedger::default();
        for _ in 0..GPU_MAX_SURFACE_SESSIONS {
            ledger.try_admit_session().unwrap();
        }
        assert_eq!(ledger.try_admit_session(), Err(AbiErrorCode::Busy));
        for _ in 0..GPU_MAX_IN_FLIGHT_FRAMES {
            ledger.try_admit_frame().unwrap();
        }
        assert_eq!(ledger.try_admit_frame(), Err(AbiErrorCode::Busy));
        for _ in 0..GPU_MAX_IN_FLIGHT_PAGES {
            ledger.try_admit_page().unwrap();
        }
        assert_eq!(ledger.try_admit_page(), Err(AbiErrorCode::Busy));
        for _ in 0..GPU_MAX_IN_FLIGHT_CONTROLS {
            ledger.try_admit_control().unwrap();
        }
        assert_eq!(ledger.try_admit_control(), Err(AbiErrorCode::Busy));
    }

    #[test]
    fn frame_payload_max_is_accepted_and_max_plus_one_rejected_before_frame_admission() {
        let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
        create(&mut adapter, 1, 1);
        ack_latest(&mut adapter);
        push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 2, 1, frame_body(1, 1, 1, vec![0; GPU_MAX_FRAME_BYTES])));
        assert!(matches!(run_request(&mut adapter), GpuOutcome::FrameAccepted { payload_bytes, .. } if payload_bytes as usize == GPU_MAX_FRAME_BYTES));
        ack_latest(&mut adapter);
        push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 3, 1, frame_body(1, 1, 2, vec![0; GPU_MAX_FRAME_BYTES + 1])));
        assert!(matches!(run_request(&mut adapter), GpuOutcome::Rejected { code: GpuErrorCode::LimitExceeded, .. }));
        assert_eq!(adapter.admission.frames(), 0);
    }

    #[test]
    fn actual_page_and_frame_ledgers_stop_at_max_without_mutating_plus_one_request() {
        let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
        create(&mut adapter, 1, 1);
        ack_latest(&mut adapter);
        for request_id in 10..10 + GPU_MAX_IN_FLIGHT_PAGES as u64 {
            push(&mut adapter, request(GPU_SURFACE_OPERATION_RESIZE, request_id, 1, resize_body(1, 1, request_id as u32, 1)));
            assert!(matches!(run_request(&mut adapter), GpuOutcome::Resized { .. }));
            assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::PageSent);
        }
        let before = adapter.surface_metrics(SurfaceId(1)).unwrap();
        push(&mut adapter, request(GPU_SURFACE_OPERATION_RESIZE, 99, 1, resize_body(1, 1, 999, 1)));
        assert_eq!(adapter.advance(AbiWorkBudget::credits(64)).unwrap(), GpuStep::AwaitingAcknowledgement);
        assert_eq!(adapter.surface_metrics(SurfaceId(1)).unwrap(), before);
        assert_eq!(adapter.admission.pages(), GPU_MAX_IN_FLIGHT_PAGES);
    }

    #[test]
    fn cancelled_frame_releases_owner_and_returns_paged_cancel_outcome() {
        let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
        create(&mut adapter, 1, 1);
        ack_latest(&mut adapter);
        push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 2, 1, frame_body(1, 1, 7, vec![9; 100])));
        assert!(matches!(run_request(&mut adapter), GpuOutcome::FrameAccepted { .. }));
        push(&mut adapter, AbiMessage::Control(AbiControl::Cancel { request_id: AbiRequestId(2), generation: 2 }));
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::PageSent);
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)), Err(AbiErrorCode::StaleGeneration));
        assert_eq!(adapter.admission.frames(), 1);
        push(&mut adapter, AbiMessage::Control(AbiControl::Cancel { request_id: AbiRequestId(2), generation: 1 }));
        assert!(matches!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::Outcome(GpuOutcome::Cancelled { .. })));
        assert_eq!(adapter.admission.frames(), 0);
    }

    #[test]
    fn interrupted_callback_and_rejected_send_retain_exact_state_for_retry() {
        let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
        push(&mut adapter, request(GPU_SURFACE_OPERATION_CREATE, 1, 1, create_body(0, 1, 1, 1, 640, 480)));
        assert_eq!(adapter.advance(AbiWorkBudget { byte_credit: 64, now_ms: 0, deadline_ms: None, cancelled: false, interrupted: true }), Err(AbiErrorCode::Interrupted));
        assert!(matches!(run_request(&mut adapter), GpuOutcome::Created { .. }));
        adapter.port.reject_send = true;
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)), Err(AbiErrorCode::Interrupted));
        assert!(adapter.outbound.is_some());
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::PageSent);
    }

    #[test]
    fn lost_stale_and_duplicate_page_controls_are_explicit() {
        let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
        create(&mut adapter, 1, 2);
        let (handle, index) = adapter.pages.last().map(|page| (page.handle, page.index)).unwrap();
        push(&mut adapter, AbiMessage::Control(AbiControl::Acknowledge { handle: AbiHandle::try_new(99, 2).unwrap(), index }));
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::PageSent);
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)), Err(AbiErrorCode::UnknownHandle));
        push(&mut adapter, AbiMessage::Control(AbiControl::Acknowledge { handle: AbiHandle::try_new(1, 1).unwrap(), index }));
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)), Err(AbiErrorCode::AbaHandle));
        push(&mut adapter, AbiMessage::Control(AbiControl::Acknowledge { handle, index }));
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::AwaitingHost);
        push(&mut adapter, AbiMessage::Control(AbiControl::Acknowledge { handle, index }));
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)), Err(AbiErrorCode::DuplicateAcknowledgement));
    }

    #[test]
    fn callback_work_is_capped_and_close_reaches_terminal_empty_incrementally() {
        let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
        push(&mut adapter, request(GPU_SURFACE_OPERATION_CREATE, 1, 1, create_body(0, 1, 1, 1, 640, 480)));
        assert!(matches!(adapter.advance(AbiWorkBudget::credits(usize::MAX)).unwrap(), GpuStep::Outcome(_)));
        adapter.begin_close();
        let mut steps = 0;
        while !adapter.terminal_is_empty() {
            assert!(matches!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::Closing { .. } | GpuStep::TerminalEmpty));
            steps += 1;
            assert!(steps < 16);
        }
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::TerminalEmpty);
        assert_eq!(adapter.admission, GpuAdmissionLedger::default());
    }

    #[test]
    fn page_close_is_exact_and_releases_frame_owner() {
        let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
        create(&mut adapter, 1, 1);
        ack_latest(&mut adapter);
        push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 2, 1, frame_body(1, 1, 1, vec![1])));
        run_request(&mut adapter);
        let handle = adapter.pages.last().unwrap().handle;
        assert_eq!(adapter.pages.last().unwrap().frame_owner.as_ref().unwrap().len(), 22);
        push(&mut adapter, AbiMessage::Control(AbiControl::Close { handle }));
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::PageSent);
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::AwaitingHost);
        assert_eq!(adapter.admission.frames(), 0);
        assert_eq!(adapter.admission.pages(), 0);
    }

    #[test]
    fn malformed_page_body_fixture_type_remains_dependency_free() {
        let page = AbiPage { handle: AbiHandle::try_new(1, 1).unwrap(), index: 0, bytes: AbiPageBytes::try_new(Vec::new()).unwrap() };
        assert_eq!(page.bytes.len(), 0);
    }
}

//#endregion 🧪️Tests

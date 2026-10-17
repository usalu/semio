//! 🧬️ Flow editor reactive features over the owned A1 byte/message ABI.

use semio_framework_abi::{
    AbiBytes, AbiControl, AbiCursorStep, AbiError, AbiErrorCode, AbiEvent, AbiEventCode, AbiHandle, AbiHandleTable, AbiMessage, AbiMessageBytes, AbiPage, AbiPageReader, AbiPort, AbiPortPoll, AbiPortRejection, AbiReply, AbiReplyLedger,
    AbiRequest, AbiRequestId, AbiStatus, AbiStatusCode, AbiWorkBudget, ABI_MAX_BODY_BYTES, ABI_MAX_IN_FLIGHT_HANDLES, ABI_MAX_IN_FLIGHT_REQUESTS, ABI_MAX_MESSAGE_BYTES, ABI_MAX_TRANSFER_BYTES,
};
use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};

//#region 🧬️Contract

pub const FLOW_OPERATION_FIRST: u16 = 2_500;
pub const FLOW_OPERATION_OPEN: u16 = 2_500;
pub const FLOW_OPERATION_ATTACH_SURFACE: u16 = 2_575;
pub const FLOW_OPERATION_SURFACE_STATUS: u16 = 2_576;
pub const FLOW_OPERATION_RENDER_FRAME: u16 = 2_581;
pub const FLOW_OPERATION_DISPOSE: u16 = 2_607;
pub const FLOW_OPERATION_LAST: u16 = 2_610;

pub const FLOW_EVENT_ADMITTED: u16 = 2_650;
pub const FLOW_EVENT_PROGRESS: u16 = 2_651;
pub const FLOW_EVENT_CHECKPOINT: u16 = 2_652;
pub const FLOW_EVENT_PREVIEW: u16 = 2_653;
pub const FLOW_EVENT_SURFACE_STATUS: u16 = 2_654;
pub const FLOW_EVENT_OUTPUT: u16 = 2_655;
pub const FLOW_EVENT_TERMINAL: u16 = 2_656;
pub const FLOW_EVENT_SESSION_TERMINAL: u16 = 2_657;

pub const FLOW_MAX_REQUEST_BYTES: usize = ABI_MAX_BODY_BYTES;
pub const FLOW_MAX_INLINE_REPLY_BYTES: usize = ABI_MAX_MESSAGE_BYTES;
pub const FLOW_MAX_OUTPUT_BYTES: usize = ABI_MAX_TRANSFER_BYTES;
pub const FLOW_MAX_RESOURCES: usize = ABI_MAX_IN_FLIGHT_HANDLES;
pub const FLOW_MAX_REQUESTS: usize = ABI_MAX_IN_FLIGHT_REQUESTS;
pub const FLOW_MAX_OUTBOUND: usize = 64;
pub const FLOW_MAX_EVENTS_IN_FLIGHT: usize = 64;
pub const FLOW_MAX_WORK_UNITS: usize = 64;
pub const FLOW_DEADLINE_MILLISECONDS: u64 = 8;

#[derive(Debug, PartialEq, Eq)]
pub enum FlowFailureCause {
    Message(String),
    Value(semio_framework_value::ValueError),
    Geometry(semio_framework_os_flow_geometry::GeometryWorkFailure),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FlowFailure {
    pub code: AbiErrorCode,
    pub cause: FlowFailureCause,
    pub retained_progress:RetainedCloneProgress,
}

impl FlowFailure {
    pub fn new(code: AbiErrorCode, message: impl Into<String>) -> Self {
        Self { code, cause:FlowFailureCause::Message(message.into()),retained_progress:Default::default() }
    }
    /// 📥️ Moves the complete original refusal across the Flow boundary without copying its prose.
    pub fn from_value(code:AbiErrorCode,error:semio_framework_value::ValueError)->Self{let retained_progress=error.retained_progress();Self{code,cause:FlowFailureCause::Value(error),retained_progress}}
    /// 🪢️ Keeps the same original geometry cause, physical effects and consumed work together.
    pub fn from_geometry(code:AbiErrorCode,original:semio_framework_os_flow_geometry::GeometryWorkFailure)->Self{Self{code,retained_progress:original.work.retained,cause:FlowFailureCause::Geometry(original)}}
    /// ⛽️ Reads actual original work without replacing or formatting the producer cause.
    pub fn consumed_work_units(&self)->usize{match &self.cause{FlowFailureCause::Geometry(original)=>original.work.consumed_units,_=>0}}
    /// 🔤️ Borrows the original cause when preparing its external reply.
    pub fn message(&self)->&str{match &self.cause{FlowFailureCause::Message(message)=>message,FlowFailureCause::Value(error)=>&error.message,FlowFailureCause::Geometry(original)=>&original.primary.message}}
    /// 🧾️ Preserves actual retained effects when a native refusal crosses the original Flow boundary.
    pub fn with_retained_progress(mut self,progress:RetainedCloneProgress)->Self{self.retained_progress=progress;self}

}

impl semio_framework_value::retirement::RetireOwned for FlowFailure {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::RetireOwned;match self.cause{FlowFailureCause::Message(message)=>message.retirement(),FlowFailureCause::Value(error)=>error.retirement(),FlowFailureCause::Geometry(original)=>original.retirement()}}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::RetireOwned;match &self.cause{FlowFailureCause::Message(message)=>message.retirement_birth_bytes(),FlowFailureCause::Value(error)=>error.retirement_birth_bytes(),FlowFailureCause::Geometry(original)=>original.retirement_birth_bytes()}}
    fn controlled_retirement_supported()->bool{true}
}

#[derive(Debug, PartialEq, Eq)]
pub enum FlowFeatureStep {
    Yield,
    Progress { completed: u64, total: u64 },
    Checkpoint(Vec<u8>),
    Preview(Vec<u8>),
    SurfaceStatus(Vec<u8>),
    RetainedPage(Vec<u8>),
    Complete(Vec<u8>),
    Failed(FlowFailure),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowFeatureAdmission {
    pub session: AbiHandle,
    pub request_generation: u32,
}

pub trait FlowFeature {
    fn step(&mut self, budget: AbiWorkBudget) -> FlowFeatureStep;
    /// 🎟️ Original producers require the initiating native context and retain refusal custody.
    fn step_on_original_caller(&mut self,_:&mut semio_framework_job::StepContext<'_>)->FlowFeatureStep{FlowFeatureStep::Failed(FlowFailure::from_value(AbiErrorCode::NoCredit,semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"Flow feature has no original caller producer")))}
    fn cancel(&mut self, budget: AbiWorkBudget) -> Result<(), FlowFailure>;

    fn preflight_acknowledge(&self, _: u32) -> Result<(), FlowFailure> {
        Err(FlowFailure::new(AbiErrorCode::UnknownHandle, "feature has no retained page"))
    }

    fn acknowledge(&mut self, _: u32, _: AbiWorkBudget) -> Result<(), FlowFailure> {
        Err(FlowFailure::new(AbiErrorCode::UnknownHandle, "feature has no retained page"))
    }

    fn close_step(&mut self, budget: AbiWorkBudget) -> Result<RetainedCloneStep, FlowFailure>;
}

//#endregion 🧬️Contract

//#region 🧱️PayloadCodec

pub struct FlowPayloadReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> FlowPayloadReader<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    pub fn u8(&mut self) -> Result<u8, AbiErrorCode> {
        Ok(self.take(1)?[0])
    }

    pub fn u32(&mut self) -> Result<u32, AbiErrorCode> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().map_err(|_| AbiErrorCode::MalformedLength)?))
    }

    pub fn u64(&mut self) -> Result<u64, AbiErrorCode> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().map_err(|_| AbiErrorCode::MalformedLength)?))
    }

    pub fn f64(&mut self) -> Result<f64, AbiErrorCode> {
        let value = f64::from_le_bytes(self.take(8)?.try_into().map_err(|_| AbiErrorCode::MalformedLength)?);
        value.is_finite().then_some(value).ok_or(AbiErrorCode::MalformedTag)
    }

    pub fn bool(&mut self) -> Result<bool, AbiErrorCode> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(AbiErrorCode::MalformedTag),
        }
    }

    pub fn bytes(&mut self) -> Result<&'a [u8], AbiErrorCode> {
        let length = self.u32()? as usize;
        self.take(length)
    }

    pub fn text(&mut self) -> Result<&'a str, AbiErrorCode> {
        std::str::from_utf8(self.bytes()?).map_err(|_| AbiErrorCode::InvalidUtf8)
    }

    pub fn optional_text(&mut self) -> Result<Option<&'a str>, AbiErrorCode> {
        match self.u8()? {
            0 => Ok(None),
            1 => self.text().map(Some),
            _ => Err(AbiErrorCode::MalformedTag),
        }
    }

    pub fn handle(&mut self) -> Result<AbiHandle, AbiErrorCode> {
        AbiHandle::try_new(self.u32()?, self.u32()?)
    }

    pub fn finish(self) -> Result<(), AbiErrorCode> {
        (self.cursor == self.bytes.len()).then_some(()).ok_or(AbiErrorCode::MalformedLength)
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], AbiErrorCode> {
        let end = self.cursor.checked_add(length).ok_or(AbiErrorCode::MalformedLength)?;
        let value = self.bytes.get(self.cursor..end).ok_or(AbiErrorCode::MalformedLength)?;
        self.cursor = end;
        Ok(value)
    }
}

#[derive(Default)]
pub struct FlowPayloadWriter {
    bytes: Vec<u8>,
}

impl FlowPayloadWriter {
    pub fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn f64(&mut self, value: f64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn handle(&mut self, value: AbiHandle) {
        self.u32(value.slot());
        self.u32(value.generation());
    }

    pub fn bytes(&mut self, value: &[u8]) -> Result<(), AbiErrorCode> {
        self.u32(u32::try_from(value.len()).map_err(|_| AbiErrorCode::LimitExceeded)?);
        self.bytes.extend_from_slice(value);
        Ok(())
    }

    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

//#endregion 🧱️PayloadCodec

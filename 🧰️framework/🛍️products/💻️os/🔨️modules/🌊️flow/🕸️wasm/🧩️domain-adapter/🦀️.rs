//! 🧩️ The Flow domain adapter: surface and host custody, argument preflight, program state and the program feature.

use semio_framework_os_infinite::board::io::text::dag_input::selection as dag_selection_io;
use semio_framework_artifact_flow_flow::artifact::Widget;
use semio_framework_os_infinite::board::ports::directed_dag as dag;
use semio_framework_canvas as canvas;
use semio_framework_os_flow_vcs::{FlowRetainedVcs, FlowVcsAuthority, FlowVcsCloseDemands, FlowVcsFault, FlowVcsGrant, FlowVcsHandle, FlowVcsPage, FlowVcsPoll};
use protocol::{FlowFailure, FlowFeature, FlowFeatureAdmission, FlowFeatureStep, FlowPayloadReader, FlowPayloadWriter};
use semio_framework_value::{retained_clone::{RetainedCloneStep, RetainedCloneProgress, RetainedCloneGrant, admit_retained_clone_close}, retirement::controlled::ControlledRetirement};
use semio_framework_abi::{decode_abi_message, encode_abi_message, AbiErrorCode, AbiMessage, AbiPort, AbiPortPoll, AbiWorkBudget};
use serde_json::json;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use ui_webgpu::{CanvasMetrics, SurfaceGeneration, SurfaceId};
use semio_framework_os_flow_host_core::*;

#[path = "../📡️protocol/🦀️.rs"]
pub mod protocol;
#[path = "../🌿️vcs-feature/🦀️.rs"]
mod vcs_feature;
pub use vcs_feature::*;
#[path = "../🛠️adapter-ops/🦀️.rs"]
mod adapter_ops;
#[path = "../🔤️argument-helpers/🦀️.rs"]
mod argument_helpers;
pub use argument_helpers::*;
#[path = "../🖼️surface-presentation/🦀️.rs"]
mod surface_presentation;
pub use surface_presentation::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceState {
    Pending,
    Ready,
    Lost,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlowSurface {
    pub id: SurfaceId,
    pub generation: SurfaceGeneration,
    pub metrics: CanvasMetrics,
    pub state: SurfaceState,
}

pub struct FlowDomainAdapter {
    pub host: FlowDomainHost,
    pub dag_read_lease:Weak<FlowDagReadLease>,
    pub vcs: Option<FlowRetainedVcs>,
    pub surface: Option<FlowSurface>,
    pub width: u32,
    pub height: u32,
    pub dpr: f64,
    /// 🧱️ Retained frame-reply buffer — see `render_frame`.
    pub frame_payload: String,
    pub frame_retirement:Option<ControlledRetirement<String>>,
}

impl Default for FlowDomainAdapter {
    fn default() -> Self {
        Self { host: FlowDomainHost::Open(FlowHost::default()), dag_read_lease:Weak::new(), vcs: None, surface: None, width: 1, height: 1, dpr: 1.0, frame_payload: String::new(),frame_retirement:None }
    }
}

pub enum FlowDomainHost {
    Open(FlowHost),
    Closing(FlowHostRetirement),
    Closed,
}

impl std::ops::Deref for FlowDomainHost {
    type Target = FlowHost;
    fn deref(&self) -> &Self::Target {
        match self { Self::Open(host) => host, _ => panic!("closed Flow domain host") }
    }
}

impl std::ops::DerefMut for FlowDomainHost {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self { Self::Open(host) => host, _ => panic!("closed Flow domain host") }
    }
}

pub fn flow_vcs_grant(budget: AbiWorkBudget) -> FlowVcsGrant {
    let maximum_deadline = budget.now_ms.saturating_add(protocol::FLOW_DEADLINE_MILLISECONDS);
    FlowVcsGrant { retained: budget.retained, items: 1, bytes: budget.byte_credit, outputs: 1, events: 1, controls: 1, fuel: 1, now_milliseconds: budget.now_ms, deadline_milliseconds: budget.deadline_ms.unwrap_or(maximum_deadline).min(maximum_deadline), interrupted: budget.interrupted || budget.cancelled }
}

#[derive(Clone, Copy)]
pub(super) enum FlowArgumentKind {
    Text,
    OptionalText,
    Number,
    Integer,
    U32,
    U8,
    Boolean,
    Bytes,
}

#[derive(Clone, Copy)]
pub struct FlowArgumentSpan {
    name: &'static str,
    pub(super) kind: FlowArgumentKind,
    start: usize,
    pub(super) end: usize,
    pub(super) present: bool,
}

impl FlowArgumentSpan {
    pub const EMPTY: Self = Self { name: "", kind: FlowArgumentKind::U8, start: 0, end: 0, present: false };
}

pub struct FlowArguments {
    pub payload: Vec<u8>,
    pub spans: [FlowArgumentSpan; 8],
    pub count: usize,
}

impl FlowArguments {
    pub fn preflight(operation: u16, payload: Vec<u8>) -> Result<Self, FlowFailure> {
        let fields = flow_operation_fields(operation);
        if fields.len() > 8 {
            return Err(abi_failure(AbiErrorCode::LimitExceeded));
        }
        let mut spans = [FlowArgumentSpan::EMPTY; 8];
        let mut cursor = 0usize;
        for (index, (name, kind)) in fields.iter().copied().enumerate() {
            let mut present = true;
            if matches!(kind, FlowArgumentKind::OptionalText) {
                present = match *payload.get(cursor).ok_or_else(|| abi_failure(AbiErrorCode::MalformedLength))? {
                    0 => false,
                    1 => true,
                    _ => return Err(abi_failure(AbiErrorCode::MalformedTag)),
                };
                cursor += 1;
            }
            let (start, end) = if !present {
                (cursor, cursor)
            } else {
                match kind {
                    FlowArgumentKind::Text | FlowArgumentKind::OptionalText | FlowArgumentKind::Bytes => {
                        let prefix_end = cursor.checked_add(4).ok_or_else(|| abi_failure(AbiErrorCode::MalformedLength))?;
                        let prefix: [u8; 4] = payload.get(cursor..prefix_end).ok_or_else(|| abi_failure(AbiErrorCode::MalformedLength))?.try_into().map_err(|_| abi_failure(AbiErrorCode::MalformedLength))?;
                        let start = prefix_end;
                        let end = start.checked_add(u32::from_le_bytes(prefix) as usize).ok_or_else(|| abi_failure(AbiErrorCode::MalformedLength))?;
                        if end > payload.len() {
                            return Err(abi_failure(AbiErrorCode::MalformedLength));
                        }
                        cursor = end;
                        (start, end)
                    }
                    FlowArgumentKind::Number | FlowArgumentKind::Integer => {
                        let end = cursor.checked_add(8).ok_or_else(|| abi_failure(AbiErrorCode::MalformedLength))?;
                        if end > payload.len() {
                            return Err(abi_failure(AbiErrorCode::MalformedLength));
                        }
                        let start = cursor;
                        cursor = end;
                        (start, end)
                    }
                    FlowArgumentKind::U32 => {
                        let end = cursor.checked_add(4).ok_or_else(|| abi_failure(AbiErrorCode::MalformedLength))?;
                        if end > payload.len() {
                            return Err(abi_failure(AbiErrorCode::MalformedLength));
                        }
                        let start = cursor;
                        cursor = end;
                        (start, end)
                    }
                    FlowArgumentKind::U8 | FlowArgumentKind::Boolean => {
                        let end = cursor.checked_add(1).ok_or_else(|| abi_failure(AbiErrorCode::MalformedLength))?;
                        if end > payload.len() {
                            return Err(abi_failure(AbiErrorCode::MalformedLength));
                        }
                        let start = cursor;
                        cursor = end;
                        (start, end)
                    }
                }
            };
            spans[index] = FlowArgumentSpan { name, kind, start, end, present };
        }
        if cursor != payload.len() {
            return Err(abi_failure(AbiErrorCode::MalformedLength));
        }
        Ok(Self { payload, spans, count: fields.len() })
    }

    pub(super) fn get(&self, key: &str) -> Option<FlowArgumentSpan> {
        self.spans[..self.count].iter().copied().find(|span| span.name == key)
    }

    pub(super) fn bytes(&self, span: FlowArgumentSpan) -> &[u8] {
        &self.payload[span.start..span.end]
    }
}

fn flow_operation_fields(operation: u16) -> &'static [(&'static str, FlowArgumentKind)] {
    use FlowArgumentKind::*;
    match operation {
        2_501..=2_503 => &[("sessionGeneration", U32), ("baseRevision", Integer), ("parentRevision", Integer)],
        2_505 | 2_506 | 2_513 | 2_514 | 2_515 | 2_525 | 2_528 | 2_529 | 2_531 | 2_579 | 2_580 | 2_610 => &[("json", Text)],
        2_507 | 2_509 => &[("widgetId", Text), ("index", Integer)],
        2_508 | 2_510 => &[("widgetId", Text), ("portId", Text)],
        2_511 => &[("fromId", Text), ("fromPort", Text), ("toId", Text), ("toPort", Text)],
        2_526 => &[("widgetId", OptionalText)],
        2_527 => &[("widgetId", OptionalText), ("port", OptionalText)],
        2_530 | 2_534 | 2_553 => &[("widgetId", Text)],
        2_532 => &[("clusterId", Text)],
        2_536 => &[("widgetId", Text), ("value", Number)],
        2_538 => &[("widgetId", Text), ("text", Text)],
        2_539 => &[("widgetId", Text), ("worldX", Number), ("worldY", Number)],
        2_540 => &[("chunk", Text)],
        2_543 => &[("direction", Text), ("extend", Boolean)],
        2_545 => &[("visible", Boolean)],
        2_546 => &[("widgetId", Text), ("src", Text)],
        2_548 => &[("widgetId", Text), ("name", Text)],
        2_549 => &[("widgetId", Text), ("schema", Text)],
        2_550 | 2_551 => &[("descriptorJson", Text), ("worldX", Number), ("worldY", Number)],
        2_554 => &[("widgetId", Text), ("x", Number), ("y", Number)],
        2_555 => &[("anchorId", Text), ("anchorOutPort", Text), ("midId", Text), ("midInPort", Text), ("midOutPort", Text)],
        2_556 => &[("anchorId", Text), ("dx", Number), ("dy", Number)],
        2_557 => &[("widgetId", Text), ("paramsJson", Text)],
        2_558 => &[("fromId", Text), ("toId", Text)],
        2_559 => &[("synapseId", Text)],
        2_564 | 2_584 => &[("sx", Number), ("sy", Number)],
        2_565 => &[("x", Number), ("y", Number), ("zoom", Number)],
        2_567 => &[("sx", Number), ("sy", Number), ("deltaX", Number), ("deltaY", Number), ("zoomGesture", Boolean)],
        2_568 => &[("active", Boolean)],
        2_570 => &[("enabled", Boolean)],
        2_571 => &[("world", Number)],
        2_572 => &[("label", Text)],
        2_575 => &[("surface", U32), ("surfaceGeneration", U32), ("width", U32), ("height", U32), ("dpr", Number)],
        2_576 => &[("surface", U32), ("surfaceGeneration", U32), ("status", Text)],
        2_578 => &[("width", U32), ("height", U32), ("dpr", Number)],
        2_582 => &[("sx", Number), ("sy", Number), ("button", U8), ("shift", Boolean), ("ctrlOrMeta", Boolean), ("alt", Boolean), ("pan", Boolean)],
        2_583 | 2_587 => &[("sx", Number), ("sy", Number), ("shift", Boolean), ("ctrlOrMeta", Boolean), ("alt", Boolean)],
        2_585 => &[("domain", Text), ("id", Text)],
        2_588 => &[("method", Text), ("mode", Text)],
        2_593 => &[("mode", Text)],
        2_599 => &[("handle", Text), ("tolerance", Number)],
        2_600 | 2_601 | 2_602 | 2_607 => &[("handle", Text)],
        2_605 => &[("width", U32), ("height", U32), ("mask", Bytes), ("threshold", Number), ("simplifyEpsilon", Number)],
        2_606 => &[("aJson", Text), ("bJson", Text), ("operation", Text)],
        _ => &[],
    }
}

pub trait FlowActionState {
    fn retained_receipt(&self)->Option<RetainedCloneProgress>{None}
    fn work_receipt(&self)->Option<usize>{None}
    #[cfg(test)]
    fn operation(&self) -> u16;
    fn advance(&mut self, domain: &mut FlowDomainAdapter, arguments: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep;
    fn advance_on_original_caller(&mut self,_:&mut FlowDomainAdapter,_:&FlowArguments,_:&mut semio_framework_job::StepContext<'_>)->FlowFeatureStep{FlowFeatureStep::Failed(FlowFailure::from_value(AbiErrorCode::NoCredit,semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"Flow action has no original caller producer")))}
    fn bind_observer(&mut self,_:Rc<FlowOperationObserver>){}
    fn bind_admission(&mut self,_:FlowFeatureAdmission)->Result<(),FlowFailure>{Ok(())}
    fn close_step(&mut self,_:&mut FlowArguments,_:AbiWorkBudget)->Result<bool,FlowFailure>{Ok(true)}
}

#[macro_export]
macro_rules! flow_result {
    ($($body:tt)*) => {{
        (|| -> Result<Vec<u8>, FlowFailure> { $($body)* })()
    }};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowProgramPhase {
    Decode,
    Validate,
    Checkpoint,
    Domain,
    Encode,
    Publish,
    Complete,
    Sealed,
}

#[derive(Clone, Copy, Debug, Default)]
struct FlowUtf8State {
    remaining: u8,
    next_min: u8,
    next_max: u8,
}

trait FlowStringArraySource {
    fn len(&self) -> usize;
    fn item(&self, index: usize) -> Option<&str>;
}

pub struct FlowPreviewOffSource<'a>(pub &'a FlowHost);

impl FlowStringArraySource for FlowPreviewOffSource<'_> {
    fn len(&self) -> usize {
        self.0.host_snapshot.widgets.len()
    }

    fn item(&self, index: usize) -> Option<&str> {
        match self.0.host_snapshot.widgets.get(index) {
            Some(Widget::Neuron { id, preview: false, .. }) => Some(id),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlowStringArrayPhase {
    CensusItem,
    CensusText,
    Open,
    Seek,
    Separator,
    QuoteOpen,
    Text,
    Escape,
    QuoteClose,
    Close,
    Complete,
}

pub enum FlowStringArrayStep {
    Progress { completed: usize, total: usize },
    Census { bytes: usize },
    Byte(u8),
    Complete,
}

pub struct FlowStringArrayCursor {
    pub(super) phase: FlowStringArrayPhase,
    item_cursor: usize,
    text_cursor: usize,
    item_count: usize,
    emitted_count: usize,
    census_bytes: usize,
    output_cursor: usize,
    escape: [u8; 6],
    escape_length: u8,
    escape_cursor: u8,
}

impl Default for FlowStringArrayCursor {
    fn default() -> Self {
        Self { phase: FlowStringArrayPhase::CensusItem, item_cursor: 0, text_cursor: 0, item_count: 0, emitted_count: 0, census_bytes: 2, output_cursor: 0, escape: [0; 6], escape_length: 0, escape_cursor: 0 }
    }
}

impl FlowStringArrayCursor {
    fn escape(byte: u8) -> ([u8; 6], u8) {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        match byte {
            b'"' => ([b'\\', b'"', 0, 0, 0, 0], 2),
            b'\\' => ([b'\\', b'\\', 0, 0, 0, 0], 2),
            0x00..=0x1f => ([b'\\', b'u', b'0', b'0', HEX[usize::from(byte >> 4)], HEX[usize::from(byte & 0x0f)]], 6),
            _ => ([byte, 0, 0, 0, 0, 0], 1),
        }
    }

    pub(super) fn step(&mut self, source: &impl FlowStringArraySource) -> Result<FlowStringArrayStep, FlowFailure> {
        let count = source.len();
        match self.phase {
            FlowStringArrayPhase::CensusItem => {
                if self.item_cursor == count {
                    if self.census_bytes > protocol::FLOW_MAX_OUTPUT_BYTES {
                        return Err(abi_failure(AbiErrorCode::LimitExceeded));
                    }
                    self.item_cursor = 0;
                    self.text_cursor = 0;
                    self.phase = FlowStringArrayPhase::Open;
                    return Ok(FlowStringArrayStep::Census { bytes: self.census_bytes });
                }
                if source.item(self.item_cursor).is_some() {
                    self.census_bytes = self.census_bytes.checked_add(2 + usize::from(self.item_count != 0)).ok_or_else(|| abi_failure(AbiErrorCode::LimitExceeded))?;
                    self.phase = FlowStringArrayPhase::CensusText;
                } else {
                    self.item_cursor += 1;
                }
                Ok(FlowStringArrayStep::Progress { completed: self.item_cursor, total: count })
            }
            FlowStringArrayPhase::CensusText => {
                let text = source.item(self.item_cursor).ok_or_else(|| abi_failure(AbiErrorCode::MalformedTag))?.as_bytes();
                if self.text_cursor == text.len() {
                    self.item_count += 1;
                    self.item_cursor += 1;
                    self.text_cursor = 0;
                    self.phase = FlowStringArrayPhase::CensusItem;
                } else {
                    self.census_bytes = self.census_bytes.checked_add(usize::from(Self::escape(text[self.text_cursor]).1)).ok_or_else(|| abi_failure(AbiErrorCode::LimitExceeded))?;
                    self.text_cursor += 1;
                }
                Ok(FlowStringArrayStep::Progress { completed: self.item_cursor, total: count })
            }
            FlowStringArrayPhase::Open => {
                self.phase = FlowStringArrayPhase::Seek;
                self.output_cursor += 1;
                Ok(FlowStringArrayStep::Byte(b'['))
            }
            FlowStringArrayPhase::Seek => {
                if self.item_cursor == count {
                    self.phase = FlowStringArrayPhase::Close;
                } else if source.item(self.item_cursor).is_some() {
                    self.phase = if self.emitted_count == 0 { FlowStringArrayPhase::QuoteOpen } else { FlowStringArrayPhase::Separator };
                } else {
                    self.item_cursor += 1;
                }
                Ok(FlowStringArrayStep::Progress { completed: self.item_cursor, total: count })
            }
            FlowStringArrayPhase::Separator => {
                self.phase = FlowStringArrayPhase::QuoteOpen;
                self.output_cursor += 1;
                Ok(FlowStringArrayStep::Byte(b','))
            }
            FlowStringArrayPhase::QuoteOpen => {
                self.phase = FlowStringArrayPhase::Text;
                self.output_cursor += 1;
                Ok(FlowStringArrayStep::Byte(b'"'))
            }
            FlowStringArrayPhase::Text => {
                let text = source.item(self.item_cursor).ok_or_else(|| abi_failure(AbiErrorCode::MalformedTag))?.as_bytes();
                if self.text_cursor == text.len() {
                    self.phase = FlowStringArrayPhase::QuoteClose;
                    return Ok(FlowStringArrayStep::Progress { completed: self.output_cursor, total: self.census_bytes });
                }
                let (escape, length) = Self::escape(text[self.text_cursor]);
                if length == 1 {
                    self.text_cursor += 1;
                    self.output_cursor += 1;
                    Ok(FlowStringArrayStep::Byte(escape[0]))
                } else {
                    self.escape = escape;
                    self.escape_length = length;
                    self.escape_cursor = 0;
                    self.phase = FlowStringArrayPhase::Escape;
                    Ok(FlowStringArrayStep::Progress { completed: self.output_cursor, total: self.census_bytes })
                }
            }
            FlowStringArrayPhase::Escape => {
                let byte = self.escape[usize::from(self.escape_cursor)];
                self.escape_cursor += 1;
                self.output_cursor += 1;
                if self.escape_cursor == self.escape_length {
                    self.text_cursor += 1;
                    self.phase = FlowStringArrayPhase::Text;
                }
                Ok(FlowStringArrayStep::Byte(byte))
            }
            FlowStringArrayPhase::QuoteClose => {
                self.emitted_count += 1;
                self.item_cursor += 1;
                self.text_cursor = 0;
                self.phase = FlowStringArrayPhase::Seek;
                self.output_cursor += 1;
                Ok(FlowStringArrayStep::Byte(b'"'))
            }
            FlowStringArrayPhase::Close => {
                self.phase = FlowStringArrayPhase::Complete;
                self.output_cursor += 1;
                Ok(FlowStringArrayStep::Byte(b']'))
            }
            FlowStringArrayPhase::Complete if self.output_cursor == self.census_bytes => Ok(FlowStringArrayStep::Complete),
            FlowStringArrayPhase::Complete => Err(abi_failure(AbiErrorCode::LimitExceeded)),
        }
    }
}

pub struct FlowProgramState {
    pub(super) phase: FlowProgramPhase,
    field_cursor: usize,
    byte_cursor: usize,
    validated_fields: usize,
    pub domain_cursor: usize,
    encode_cursor: usize,
    utf8: [FlowUtf8State; 8],
    source_output: Option<Vec<u8>>,
    pub(super) output: Vec<u8>,
}

impl FlowProgramState {
    pub(super) fn new(_arguments: &FlowArguments) -> Self {
        Self { phase: FlowProgramPhase::Decode, field_cursor: 0, byte_cursor: 0, validated_fields: 0, domain_cursor: 0, encode_cursor: 0, utf8: [FlowUtf8State::default(); 8], source_output: None, output: Vec::new() }
    }

    pub fn decode_step(&mut self, arguments: &FlowArguments) -> FlowFeatureStep {
        while self.field_cursor < arguments.count && self.byte_cursor == arguments.spans[self.field_cursor].end - arguments.spans[self.field_cursor].start {
            if self.utf8[self.field_cursor].remaining != 0 {
                return FlowFeatureStep::Failed(abi_failure(AbiErrorCode::InvalidUtf8));
            }
            self.field_cursor += 1;
            self.byte_cursor = 0;
        }
        if self.field_cursor == arguments.count {
            self.phase = FlowProgramPhase::Validate;
            return FlowFeatureStep::Progress { completed: arguments.payload.len() as u64, total: arguments.payload.len() as u64 };
        }
        let span = arguments.spans[self.field_cursor];
        let byte = arguments.payload[span.start + self.byte_cursor];
        if matches!(span.kind, FlowArgumentKind::Text | FlowArgumentKind::OptionalText) && span.present {
            if let Err(failure) = validate_utf8_byte(&mut self.utf8[self.field_cursor], byte) {
                return FlowFeatureStep::Failed(failure);
            }
        }
        self.byte_cursor += 1;
        let completed = arguments.spans[..self.field_cursor].iter().map(|field| field.end - field.start).sum::<usize>() + self.byte_cursor;
        FlowFeatureStep::Progress { completed: completed as u64, total: arguments.payload.len() as u64 }
    }

    pub fn validate_step(&mut self, arguments: &FlowArguments) -> FlowFeatureStep {
        if self.validated_fields == arguments.count {
            self.phase = FlowProgramPhase::Checkpoint;
            return FlowFeatureStep::Progress { completed: self.validated_fields as u64, total: arguments.count as u64 };
        }
        let span = arguments.spans[self.validated_fields];
        let bytes = &arguments.payload[span.start..span.end];
        let valid = match span.kind {
            FlowArgumentKind::Number => f64::from_le_bytes(bytes.try_into().unwrap_or([0; 8])).is_finite(),
            FlowArgumentKind::Boolean => matches!(bytes, [0] | [1]),
            FlowArgumentKind::Text | FlowArgumentKind::OptionalText => self.utf8[self.validated_fields].remaining == 0,
            _ => true,
        };
        if !valid {
            return FlowFeatureStep::Failed(abi_failure(AbiErrorCode::MalformedTag));
        }
        self.validated_fields += 1;
        FlowFeatureStep::Progress { completed: self.validated_fields as u64, total: arguments.count as u64 }
    }

    pub fn checkpoint_step(&mut self, operation: u16) -> FlowFeatureStep {
        self.phase = FlowProgramPhase::Domain;
        let mut checkpoint = FlowPayloadWriter::default();
        checkpoint.u32(operation as u32);
        checkpoint.u64(self.validated_fields as u64);
        FlowFeatureStep::Checkpoint(checkpoint.finish())
    }

    pub fn domain_ready_step(&mut self) -> FlowFeatureStep {
        self.domain_cursor += 1;
        FlowFeatureStep::Progress { completed: self.domain_cursor as u64, total: 2 }
    }

    pub fn finish_domain(&mut self, result: Result<Vec<u8>, FlowFailure>) -> FlowFeatureStep {
        match result {
            Ok(output) if output.len() <= protocol::FLOW_MAX_OUTPUT_BYTES => {
                let total = output.len();
                self.output = Vec::with_capacity(total);
                self.source_output = Some(output);
                self.phase = FlowProgramPhase::Encode;
                FlowFeatureStep::Progress { completed: 0, total: total as u64 }
            }
            Ok(_) => {
                self.phase = FlowProgramPhase::Sealed;
                FlowFeatureStep::Failed(abi_failure(AbiErrorCode::LimitExceeded))
            }
            Err(failure) => {
                self.phase = FlowProgramPhase::Sealed;
                FlowFeatureStep::Failed(failure)
            }
        }
    }

    pub fn begin_incremental_output(&mut self, bytes: usize) -> Result<(), FlowFailure> {
        if bytes > protocol::FLOW_MAX_OUTPUT_BYTES || !self.output.is_empty() || self.output.capacity() != 0 {
            return Err(abi_failure(AbiErrorCode::LimitExceeded));
        }
        self.output = Vec::with_capacity(bytes);
        Ok(())
    }

    pub fn finish_incremental_output(&mut self) -> FlowFeatureStep {
        self.encode_cursor = self.output.len();
        self.phase = FlowProgramPhase::Publish;
        FlowFeatureStep::Progress { completed: self.output.len() as u64, total: self.output.len() as u64 }
    }

    pub fn encode_step(&mut self) -> FlowFeatureStep {
        let total = self.source_output.as_ref().map_or(0, Vec::len);
        if self.encode_cursor == total {
            self.source_output.take();
            self.phase = FlowProgramPhase::Publish;
            return FlowFeatureStep::Progress { completed: total as u64, total: total as u64 };
        }
        self.output.push(self.source_output.as_ref().expect("retained Flow source output")[self.encode_cursor]);
        self.encode_cursor += 1;
        FlowFeatureStep::Progress { completed: self.encode_cursor as u64, total: total as u64 }
    }

    pub fn publish_step(&mut self, domain: &FlowDomainAdapter, operation: u16) -> FlowFeatureStep {
        self.phase = FlowProgramPhase::Complete;
        if matches!(operation, 2_575 | 2_576) {
            FlowFeatureStep::SurfaceStatus(domain.surface_status_bytes())
        } else {
            let mut preview = FlowPayloadWriter::default();
            preview.u32(operation as u32);
            preview.u64(self.domain_cursor as u64);
            FlowFeatureStep::Preview(preview.finish())
        }
    }

    pub fn complete_step(&mut self) -> FlowFeatureStep {
        self.phase = FlowProgramPhase::Sealed;
        FlowFeatureStep::Complete(std::mem::take(&mut self.output))
    }
}

fn validate_utf8_byte(state: &mut FlowUtf8State, byte: u8) -> Result<(), FlowFailure> {
    if state.remaining != 0 {
        if byte < state.next_min || byte > state.next_max {
            return Err(abi_failure(AbiErrorCode::InvalidUtf8));
        }
        state.remaining -= 1;
        state.next_min = 0x80;
        state.next_max = 0xbf;
        return Ok(());
    }
    match byte {
        0x00..=0x7f => {}
        0xc2..=0xdf => {
            state.remaining = 1;
            state.next_min = 0x80;
            state.next_max = 0xbf;
        }
        0xe0 => {
            state.remaining = 2;
            state.next_min = 0xa0;
            state.next_max = 0xbf;
        }
        0xe1..=0xec | 0xee..=0xef => {
            state.remaining = 2;
            state.next_min = 0x80;
            state.next_max = 0xbf;
        }
        0xed => {
            state.remaining = 2;
            state.next_min = 0x80;
            state.next_max = 0x9f;
        }
        0xf0 => {
            state.remaining = 3;
            state.next_min = 0x90;
            state.next_max = 0xbf;
        }
        0xf1..=0xf3 => {
            state.remaining = 3;
            state.next_min = 0x80;
            state.next_max = 0xbf;
        }
        0xf4 => {
            state.remaining = 3;
            state.next_min = 0x80;
            state.next_max = 0x8f;
        }
        _ => return Err(abi_failure(AbiErrorCode::InvalidUtf8)),
    }
    Ok(())
}

struct FlowDagReadLease {admission:FlowFeatureAdmission,active:Cell<bool>}
pub struct FlowOperationObserver {cancelled:Cell<bool>,interrupted:Cell<bool>,deadline_us:Cell<Option<u64>>,failure:Cell<Option<AbiErrorCode>>,bound:Cell<bool>}
impl FlowOperationObserver {
    pub(super) fn new()->Self{Self{cancelled:Cell::new(false),interrupted:Cell::new(false),deadline_us:Cell::new(None),failure:Cell::new(None),bound:Cell::new(false)}}
    pub fn check_budget(&self,budget:AbiWorkBudget)->Result<(),FlowFailure>{
        let code=if self.cancelled.get()||budget.cancelled{Some(AbiErrorCode::Cancelled)}else if self.interrupted.get()||budget.interrupted{Some(AbiErrorCode::Interrupted)}else if budget.deadline_ms.is_some_and(|deadline|budget.now_ms>=deadline){Some(AbiErrorCode::DeadlineExceeded)}else{None};if let Some(code)=code{self.failure.set(Some(code));Err(abi_failure(code))}else{Ok(())}
    }
    pub fn begin_poll(&self,budget:AbiWorkBudget)->Result<(),FlowFailure>{
        self.interrupted.set(budget.interrupted);self.check_budget(budget)?;
        let now=semio_framework_job::default_now_us().ok_or_else(||abi_failure(AbiErrorCode::Busy))?;
        let remaining=budget.deadline_ms.map_or(protocol::FLOW_DEADLINE_MILLISECONDS,|deadline|deadline.saturating_sub(budget.now_ms)).min(protocol::FLOW_DEADLINE_MILLISECONDS);
        self.deadline_us.set(Some(now.saturating_add(remaining.saturating_mul(1000))));Ok(())
    }
    pub fn observe(&self)->bool{
        let code=if self.cancelled.get(){Some(AbiErrorCode::Cancelled)}else if self.interrupted.get(){Some(AbiErrorCode::Interrupted)}else{match(semio_framework_job::default_now_us(),self.deadline_us.get()){(Some(now),Some(deadline))if now<deadline=>None,_=>Some(AbiErrorCode::DeadlineExceeded)}};
        if let Some(code)=code{self.failure.set(Some(code));false}else{true}
    }
}


pub struct FlowProgramFeature {
    pub domain: Rc<RefCell<FlowDomainAdapter>>,
    pub arguments: FlowArguments,
    pub action: Box<dyn FlowActionState>,
    pub observer:Rc<FlowOperationObserver>,
    pub admission:FlowFeatureAdmission,
    pub operation:u16,
    pub dag_read_lease:Option<Rc<FlowDagReadLease>>,
    pub terminal_failure:Option<AbiErrorCode>,
}

impl FlowProgramFeature {
    fn failed(&mut self,failure:FlowFailure)->FlowFeatureStep{
        self.terminal_failure=Some(failure.code);
        if let Some(lease)=self.dag_read_lease.take(){lease.active.set(false);}
        FlowFeatureStep::Failed(failure)
    }
    fn guard(&self, budget: AbiWorkBudget) -> Result<(), FlowFailure> {
        if self.observer.cancelled.get() || budget.cancelled {
            Err(FlowFailure::new(AbiErrorCode::Cancelled, "Flow feature cancelled"))
        } else if budget.interrupted {
            Err(FlowFailure::new(AbiErrorCode::Interrupted, "Flow feature interrupted"))
        } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
            Err(FlowFailure::new(AbiErrorCode::DeadlineExceeded, "Flow feature deadline"))
        } else if budget.byte_credit == 0 {
            Err(FlowFailure::new(AbiErrorCode::NoCredit, "Flow feature requires credit"))
        } else {
            Ok(())
        }
    }
}

impl FlowFeature for FlowProgramFeature {
    fn step_on_original_caller(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->FlowFeatureStep{
        if cx.should_yield(){return FlowFeatureStep::Yield}
        let mut domain=match self.domain.try_borrow_mut(){Ok(domain)=>domain,Err(_)=>return FlowFeatureStep::Yield};
        self.action.advance_on_original_caller(&mut domain,&self.arguments,cx)
    }
    /// ⚡️ Spends the whole granted byte credit on ONE poll instead of one byte.
    ///
    /// Every incremental phase — argument decode, the retained-DAG cursors, output encode — advances
    /// by a single byte and answers `Progress`, and the bridge turns each `Progress` into its own ABI
    /// event the host must poll, decode and reply to. A poll granted 4 096 bytes of credit therefore
    /// moved one byte, so a flow operation's payload crossed at roughly 68 KB/s: a 20 KB node-graph
    /// draw list cost ~20 000 round trips and a board frame took 200–450 ms to present while idle,
    /// and over 2.5 s under a scroll gesture sharing the pipe with the overlay reads
    /// (`📓️flow-scroll-render-perf-2026-09-15.md` §3).
    ///
    /// The budget already says how much one poll may do. Looping until the credit is spent honours it
    /// and publishes the LAST progress of the run, so `completed`/`total` still advance monotonically
    /// and a watcher sees the same ladder at coarser granularity. Every other step — `Yield`,
    /// `Complete`, `Failed`, a page, a checkpoint, a preview — returns immediately, so nothing that
    /// carries a payload or ends the operation is ever coalesced away, and the loop is bounded by the
    /// credit whatever the action does.
    fn step(&mut self, budget: AbiWorkBudget) -> FlowFeatureStep {
        if let Some(code)=self.terminal_failure{return FlowFeatureStep::Failed(abi_failure(code))}
        if self.dag_read_lease.as_ref().is_some_and(|lease|lease.active.get()==false){return self.failed(abi_failure(AbiErrorCode::StaleGeneration))}
        if budget.byte_credit==0&&!budget.cancelled&&!budget.interrupted&&budget.deadline_ms.is_none_or(|deadline|budget.now_ms<deadline){return FlowFeatureStep::Yield;}
        if let Err(failure) = self.guard(budget) {
            return self.failed(failure);
        }
        if budget.byte_credit==0{return FlowFeatureStep::Yield;}
        if let Err(failure)=self.observer.begin_poll(budget){return self.failed(failure)}
        let mut domain = self.domain.borrow_mut();
        if let Some(lease)=domain.dag_read_lease.upgrade().filter(|lease|lease.active.get()){
            if lease.admission!=self.admission||self.dag_read_lease.as_ref().is_none_or(|owned|!Rc::ptr_eq(owned,&lease)){return FlowFeatureStep::Yield;}
        }
        if matches!(self.operation,2514|2515|2518|2519|2520|2522|2523|2525|2528)&&self.dag_read_lease.is_none(){
            let lease=Rc::new(FlowDagReadLease{admission:self.admission,active:Cell::new(true)});domain.dag_read_lease=Rc::downgrade(&lease);self.dag_read_lease=Some(lease);
        }
        let mut spent = 0usize;
        let mut remaining=budget;
        loop {
            if !self.observer.observe(){let code=self.observer.failure.get().unwrap_or(AbiErrorCode::DeadlineExceeded);drop(domain);return self.failed(abi_failure(code))}
            remaining.byte_credit=budget.byte_credit-spent;
            let before_work=self.action.work_receipt();
            let before=self.action.retained_receipt();
            let step = self.action.advance(&mut domain, &self.arguments, remaining);
            if let(Some(before),Some(after))=(before,self.action.retained_receipt()){
                let spent=[after.copied_items.checked_sub(before.copied_items),after.copied_bytes.checked_sub(before.copied_bytes),after.retained_capacity_bytes.checked_sub(before.retained_capacity_bytes),after.released_bytes.checked_sub(before.released_bytes)];
                let available=[remaining.retained.maximum_items,remaining.retained.maximum_copy_bytes,remaining.retained.maximum_capacity_bytes,remaining.retained.maximum_release_bytes];
                let mut left=[0;4];for index in 0..4{let Some(value)=spent[index].and_then(|spent|available[index].checked_sub(spent))else{drop(domain);return self.failed(abi_failure(AbiErrorCode::NoCredit))};left[index]=value;}
                remaining.retained.maximum_items=left[0];remaining.retained.maximum_copy_bytes=left[1];remaining.retained.maximum_capacity_bytes=left[2];remaining.retained.maximum_release_bytes=left[3];
            }
            let FlowFeatureStep::Progress { completed, total } = step else {
                if let FlowFeatureStep::Failed(failure)=step{drop(domain);return self.failed(failure)}
                if matches!(step,FlowFeatureStep::Complete(_)){if let Some(lease)=self.dag_read_lease.take(){lease.active.set(false);}}
                return step;
            };
            let price=match(before_work,self.action.work_receipt()){(Some(before),Some(after))=>match after.checked_sub(before){Some(work)=>work.max(1),None=>{drop(domain);return self.failed(abi_failure(AbiErrorCode::LimitExceeded))}},_=>1};
            spent=match spent.checked_add(price){Some(value)if value<=budget.byte_credit=>value,_=>{drop(domain);return self.failed(abi_failure(AbiErrorCode::NoCredit))}};
            if spent >= budget.byte_credit {
                return FlowFeatureStep::Progress { completed, total };
            }
        }
    }

    fn cancel(&mut self, _: AbiWorkBudget) -> Result<(), FlowFailure> {
        self.observer.cancelled.set(true);
        self.terminal_failure=Some(AbiErrorCode::Cancelled);
        if let Some(lease)=self.dag_read_lease.take(){lease.active.set(false);}
        Ok(())
    }
    fn close_step(&mut self,budget:AbiWorkBudget)->Result<RetainedCloneStep,FlowFailure>{
        self.terminal_failure.get_or_insert(AbiErrorCode::Cancelled);
        if let Some(lease)=self.dag_read_lease.take(){lease.active.set(false);}
        let before=self.action.retained_receipt();let result=self.action.close_step(&mut self.arguments,budget);let after=self.action.retained_receipt();
        let progress=match(before,after){
            (Some(before),Some(after))=>RetainedCloneProgress{copied_items:after.copied_items.checked_sub(before.copied_items).ok_or_else(||flow_close_failure(AbiErrorCode::NoCredit))?,copied_bytes:after.copied_bytes.checked_sub(before.copied_bytes).ok_or_else(||flow_close_failure(AbiErrorCode::NoCredit))?,retained_capacity_bytes:after.retained_capacity_bytes.checked_sub(before.retained_capacity_bytes).ok_or_else(||flow_close_failure(AbiErrorCode::NoCredit))?,released_bytes:after.released_bytes.checked_sub(before.released_bytes).ok_or_else(||flow_close_failure(AbiErrorCode::NoCredit))?},
            _=>Default::default(),
        };
        match result{Ok(true)=>Ok(RetainedCloneStep::Complete(progress)),Ok(false)=>Ok(RetainedCloneStep::Progress(progress)),Err(failure)if failure.retained_progress==RetainedCloneProgress::default()=>Err(failure.with_retained_progress(progress)),Err(failure)=>Err(failure)}
    }

}

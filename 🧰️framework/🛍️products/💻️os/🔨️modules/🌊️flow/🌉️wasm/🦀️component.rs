//! 🌉️ Flow editor owned byte/message bridge and primitive linear-memory exports.

#[path = "🦀️protocol.rs"]
pub mod protocol;

use crate::artifact::Widget;
use crate::drawing::*;
use crate::host::*;
use crate::infinite::board::ports::directed_dag as dag;
use crate::infinite::canvas;
use crate::vcs::{FlowRetainedVcs, FlowVcsAuthority, FlowVcsFault, FlowVcsGrant, FlowVcsHandle, FlowVcsPage, FlowVcsPoll};
use protocol::{FlowBridge, FlowDomain, FlowFailure, FlowFeature, FlowFeatureAdmission, FlowFeatureStep, FlowPayloadReader, FlowPayloadWriter};
use semio_framework::abi::{decode_abi_message, encode_abi_message, AbiErrorCode, AbiMessage, AbiPort, AbiPortPoll, AbiWorkBudget};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::rc::Rc;
use ui_webgpu::abi::AbiErrorCode as SurfaceAbiErrorCode;
use ui_webgpu::{CanvasMetrics, SurfaceGeneration, SurfaceId};

//#region 🔖️ReactiveFeatures

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SurfaceState {
    Pending,
    Ready,
    Lost,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct FlowSurface {
    id: SurfaceId,
    generation: SurfaceGeneration,
    metrics: CanvasMetrics,
    state: SurfaceState,
}

struct FlowDomainAdapter {
    host: FlowHost,
    vcs: Option<FlowRetainedVcs>,
    surface: Option<FlowSurface>,
    width: u32,
    height: u32,
    dpr: f64,
}

impl Default for FlowDomainAdapter {
    fn default() -> Self {
        Self { host: FlowHost::default(), vcs: None, surface: None, width: 1, height: 1, dpr: 1.0 }
    }
}

#[derive(Clone, Copy)]
enum FlowArgumentKind {
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
struct FlowArgumentSpan {
    name: &'static str,
    kind: FlowArgumentKind,
    start: usize,
    end: usize,
    present: bool,
}

impl FlowArgumentSpan {
    const EMPTY: Self = Self { name: "", kind: FlowArgumentKind::U8, start: 0, end: 0, present: false };
}

struct FlowArguments {
    payload: Vec<u8>,
    spans: [FlowArgumentSpan; 8],
    count: usize,
}

impl FlowArguments {
    fn preflight(operation: u16, payload: Vec<u8>) -> Result<Self, FlowFailure> {
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

    fn get(&self, key: &str) -> Option<FlowArgumentSpan> {
        self.spans[..self.count].iter().copied().find(|span| span.name == key)
    }

    fn bytes(&self, span: FlowArgumentSpan) -> &[u8] {
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
        2_600 | 2_601 | 2_602 | 2_603 | 2_607 => &[("handle", Text)],
        2_604 => &[("dataBase64", Text)],
        2_605 => &[("width", U32), ("height", U32), ("mask", Bytes), ("threshold", Number), ("simplifyEpsilon", Number)],
        2_606 => &[("aJson", Text), ("bJson", Text), ("operation", Text)],
        2_608 => &[("meshJson", Text)],
        _ => &[],
    }
}

trait FlowActionState {
    fn operation(&self) -> u16;
    fn advance(&mut self, domain: &mut FlowDomainAdapter, arguments: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep;
}

macro_rules! flow_result {
    ($($body:tt)*) => {{
        (|| -> Result<Vec<u8>, FlowFailure> { $($body)* })()
    }};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlowProgramPhase {
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

struct FlowPreviewOffSource<'a>(&'a FlowHost);

impl FlowStringArraySource for FlowPreviewOffSource<'_> {
    fn len(&self) -> usize {
        self.0.fixture.widgets.len()
    }

    fn item(&self, index: usize) -> Option<&str> {
        match self.0.fixture.widgets.get(index) {
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

enum FlowStringArrayStep {
    Progress { completed: usize, total: usize },
    Census { bytes: usize },
    Byte(u8),
    Complete,
}

struct FlowStringArrayCursor {
    phase: FlowStringArrayPhase,
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

    fn step(&mut self, source: &impl FlowStringArraySource) -> Result<FlowStringArrayStep, FlowFailure> {
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

struct FlowProgramState {
    phase: FlowProgramPhase,
    field_cursor: usize,
    byte_cursor: usize,
    validated_fields: usize,
    domain_cursor: usize,
    encode_cursor: usize,
    decoded: [Vec<u8>; 8],
    utf8: [FlowUtf8State; 8],
    source_output: Option<Vec<u8>>,
    output: Vec<u8>,
}

impl FlowProgramState {
    fn new(arguments: &FlowArguments) -> Self {
        let decoded = std::array::from_fn(|index| {
            let capacity = arguments.spans.get(index).filter(|_| index < arguments.count).map_or(0, |span| span.end - span.start);
            Vec::with_capacity(capacity)
        });
        Self { phase: FlowProgramPhase::Decode, field_cursor: 0, byte_cursor: 0, validated_fields: 0, domain_cursor: 0, encode_cursor: 0, decoded, utf8: [FlowUtf8State::default(); 8], source_output: None, output: Vec::new() }
    }

    fn decode_step(&mut self, arguments: &FlowArguments) -> FlowFeatureStep {
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
        self.decoded[self.field_cursor].push(byte);
        self.byte_cursor += 1;
        let completed = arguments.spans[..self.field_cursor].iter().map(|field| field.end - field.start).sum::<usize>() + self.byte_cursor;
        FlowFeatureStep::Progress { completed: completed as u64, total: arguments.payload.len() as u64 }
    }

    fn validate_step(&mut self, arguments: &FlowArguments) -> FlowFeatureStep {
        if self.validated_fields == arguments.count {
            self.phase = FlowProgramPhase::Checkpoint;
            return FlowFeatureStep::Progress { completed: self.validated_fields as u64, total: arguments.count as u64 };
        }
        let span = arguments.spans[self.validated_fields];
        let bytes = &self.decoded[self.validated_fields];
        let valid = match span.kind {
            FlowArgumentKind::Number => f64::from_le_bytes(bytes.as_slice().try_into().unwrap_or([0; 8])).is_finite(),
            FlowArgumentKind::Boolean => matches!(bytes.as_slice(), [0] | [1]),
            FlowArgumentKind::Text | FlowArgumentKind::OptionalText => self.utf8[self.validated_fields].remaining == 0,
            _ => true,
        };
        if !valid {
            return FlowFeatureStep::Failed(abi_failure(AbiErrorCode::MalformedTag));
        }
        self.validated_fields += 1;
        FlowFeatureStep::Progress { completed: self.validated_fields as u64, total: arguments.count as u64 }
    }

    fn checkpoint_step(&mut self, operation: u16) -> FlowFeatureStep {
        self.phase = FlowProgramPhase::Domain;
        let mut checkpoint = FlowPayloadWriter::default();
        checkpoint.u32(operation as u32);
        checkpoint.u64(self.validated_fields as u64);
        FlowFeatureStep::Checkpoint(checkpoint.finish())
    }

    fn domain_ready_step(&mut self) -> FlowFeatureStep {
        self.domain_cursor += 1;
        FlowFeatureStep::Progress { completed: self.domain_cursor as u64, total: 2 }
    }

    fn finish_domain(&mut self, result: Result<Vec<u8>, FlowFailure>) -> FlowFeatureStep {
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

    fn begin_incremental_output(&mut self, bytes: usize) -> Result<(), FlowFailure> {
        if bytes > protocol::FLOW_MAX_OUTPUT_BYTES || !self.output.is_empty() || self.output.capacity() != 0 {
            return Err(abi_failure(AbiErrorCode::LimitExceeded));
        }
        self.output = Vec::with_capacity(bytes);
        Ok(())
    }

    fn finish_incremental_output(&mut self) -> FlowFeatureStep {
        self.encode_cursor = self.output.len();
        self.phase = FlowProgramPhase::Publish;
        FlowFeatureStep::Progress { completed: self.output.len() as u64, total: self.output.len() as u64 }
    }

    fn encode_step(&mut self) -> FlowFeatureStep {
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

    fn publish_step(&mut self, domain: &FlowDomainAdapter, operation: u16) -> FlowFeatureStep {
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

    fn complete_step(&mut self) -> FlowFeatureStep {
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

struct FlowProgramFeature {
    domain: Rc<RefCell<FlowDomainAdapter>>,
    arguments: FlowArguments,
    action: Box<dyn FlowActionState>,
    cancelled: bool,
}

impl FlowProgramFeature {
    fn guard(&self, budget: AbiWorkBudget) -> Result<(), FlowFailure> {
        if self.cancelled || budget.cancelled {
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
    fn step(&mut self, budget: AbiWorkBudget) -> FlowFeatureStep {
        if let Err(failure) = self.guard(budget) {
            return FlowFeatureStep::Failed(failure);
        }
        self.action.advance(&mut self.domain.borrow_mut(), &self.arguments, budget)
    }

    fn cancel(&mut self, _: AbiWorkBudget) -> Result<(), FlowFailure> {
        self.cancelled = true;
        Ok(())
    }
}

impl FlowDomain for FlowDomainAdapter {
    fn bind_session(&mut self, session: semio_framework::abi::AbiHandle) {
        self.vcs = Some(FlowRetainedVcs::new(crate::artifact::FlowFixture::default(), session.generation(), 0, 0));
    }

    fn start_feature(domain: Rc<RefCell<Self>>, admission: FlowFeatureAdmission, operation: u16, payload: Vec<u8>) -> Result<Box<dyn FlowFeature>, FlowFailure> {
        if matches!(operation, 2_501..=2_503) {
            return FlowVcsFeature::admit(domain, admission, operation, payload).map(|feature| Box::new(feature) as Box<dyn FlowFeature>);
        }
        let arguments = FlowArguments::preflight(operation, payload)?;
        let action = flow_action(operation, &arguments).ok_or_else(|| abi_failure(AbiErrorCode::UnknownOperation))?;
        Ok(Box::new(FlowProgramFeature { domain, arguments, action, cancelled: false }))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlowVcsFeaturePhase {
    Admit,
    Poll,
    Fault,
    Lease,
    Resume,
    Retry,
    AwaitAcknowledgement,
    CloseOperation,
    CloseRetired,
    Complete,
}

struct FlowVcsFeature {
    domain: Rc<RefCell<FlowDomainAdapter>>,
    admission: FlowFeatureAdmission,
    authority: FlowVcsAuthority,
    handle: Option<FlowVcsHandle>,
    page: Option<FlowVcsPage>,
    phase: FlowVcsFeaturePhase,
    fault_requested: bool,
    terminal_failure: Option<FlowFailure>,
}

impl FlowVcsFeature {
    fn admit(domain: Rc<RefCell<FlowDomainAdapter>>, admission: FlowFeatureAdmission, operation: u16, payload: Vec<u8>) -> Result<Self, FlowFailure> {
        let mut reader = FlowPayloadReader::new(&payload);
        let authority = FlowVcsAuthority { session_generation: reader.u32().map_err(abi_code_failure)?, base_revision: reader.u64().map_err(abi_code_failure)?, parent_revision: reader.u64().map_err(abi_code_failure)? };
        reader.finish().map_err(abi_code_failure)?;
        if authority.session_generation != admission.session.generation() || admission.request_generation == 0 {
            return Err(abi_failure(AbiErrorCode::StaleGeneration));
        }
        let current = domain.borrow().vcs.as_ref().ok_or_else(|| abi_failure(AbiErrorCode::Closed))?.authority();
        if current != authority {
            return Err(flow_vcs_failure(FlowVcsFault::StaleAuthority));
        }
        Ok(Self { domain, admission, authority, handle: None, page: None, phase: FlowVcsFeaturePhase::Admit, fault_requested: operation == 2_502, terminal_failure: None })
    }

    fn grant(&self, budget: AbiWorkBudget) -> FlowVcsGrant {
        FlowVcsGrant {
            items: 1,
            bytes: budget.byte_credit,
            outputs: 1,
            events: 1,
            controls: 1,
            fuel: 1,
            now_milliseconds: budget.now_ms,
            deadline_milliseconds: budget.deadline_ms.unwrap_or_else(|| budget.now_ms.saturating_add(protocol::FLOW_DEADLINE_MILLISECONDS)),
            interrupted: budget.interrupted || budget.cancelled,
        }
    }

    fn close_cursor_step(&mut self, budget: AbiWorkBudget) -> Result<bool, FlowFailure> {
        let grant = self.grant(budget);
        let handle = self.handle.expect("admitted Flow VCS handle");
        let mut domain = self.domain.borrow_mut();
        let vcs = domain.vcs.as_mut().ok_or_else(|| abi_failure(AbiErrorCode::Closed))?;
        match self.phase {
            FlowVcsFeaturePhase::CloseOperation => {
                if vcs.close_operation_step(handle, grant).map_err(flow_vcs_failure)? {
                    self.phase = FlowVcsFeaturePhase::CloseRetired;
                }
                Ok(false)
            }
            FlowVcsFeaturePhase::CloseRetired => {
                match vcs.close_retired_step(grant) {
                    Ok(true) => {
                        self.phase = FlowVcsFeaturePhase::Complete;
                        return Ok(true);
                    }
                    Ok(false) | Err(FlowVcsFault::ClosePending) => {}
                    Err(fault) => return Err(flow_vcs_failure(fault)),
                }
                Ok(false)
            }
            FlowVcsFeaturePhase::Complete => Ok(true),
            _ => Err(abi_failure(AbiErrorCode::Busy)),
        }
    }

    fn encode_checkpoint(&self, operation: u64, revision: u64) -> Vec<u8> {
        let mut writer = FlowPayloadWriter::default();
        writer.u32(self.admission.session.slot());
        writer.u32(self.admission.session.generation());
        writer.u32(self.admission.request_generation);
        writer.u64(operation);
        writer.u64(revision);
        writer.finish()
    }

    fn encode_preview(&self, widgets: u32, synapses: u32, layout: u32) -> Vec<u8> {
        let mut writer = FlowPayloadWriter::default();
        writer.u32(widgets);
        writer.u32(synapses);
        writer.u32(layout);
        writer.finish()
    }

    fn encode_page(&self, page: FlowVcsPage) -> Vec<u8> {
        let mut writer = FlowPayloadWriter::default();
        writer.u32(self.admission.session.slot());
        writer.u32(self.admission.session.generation());
        writer.u32(self.admission.request_generation);
        writer.u32(self.authority.session_generation);
        writer.u64(self.authority.base_revision);
        writer.u64(self.authority.parent_revision);
        let handle = self.handle.expect("leased Flow VCS handle");
        writer.u64(handle.operation);
        writer.u8(handle.slot);
        writer.u32(handle.generation);
        writer.u64(page.sequence);
        writer.u64(page.operation);
        writer.u32(page.session_generation);
        writer.u64(page.revision);
        writer.u64(page.parent_revision);
        writer.u64(page.document_generation);
        writer.u32(page.widget_count);
        writer.u32(page.synapse_count);
        writer.u32(page.layout_count);
        writer.u64(page.semantic_digest);
        writer.finish()
    }
}

impl FlowFeature for FlowVcsFeature {
    fn step(&mut self, budget: AbiWorkBudget) -> FlowFeatureStep {
        let grant = self.grant(budget);
        match self.phase {
            FlowVcsFeaturePhase::Admit => {
                let result = self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.begin_checkpoint(self.authority));
                match result {
                    Ok(handle) => {
                        self.handle = Some(handle);
                        self.phase = FlowVcsFeaturePhase::Poll;
                        FlowFeatureStep::Progress { completed: 0, total: 3 }
                    }
                    Err(fault) => FlowFeatureStep::Failed(flow_vcs_failure(fault)),
                }
            }
            FlowVcsFeaturePhase::Poll => {
                let handle = self.handle.expect("admitted Flow VCS handle");
                let result = self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.poll(handle, grant));
                match result {
                    Ok(FlowVcsPoll::Progress { completed, total }) => FlowFeatureStep::Progress { completed: u64::from(completed), total: u64::from(total) },
                    Ok(FlowVcsPoll::Checkpoint { operation, revision }) => {
                        if self.fault_requested {
                            self.terminal_failure = Some(FlowFailure::new(AbiErrorCode::MalformedTag, "Flow VCS requested fault"));
                            self.phase = FlowVcsFeaturePhase::Fault;
                        }
                        FlowFeatureStep::Checkpoint(self.encode_checkpoint(operation, revision))
                    }
                    Ok(FlowVcsPoll::Preview { widgets, synapses, layout }) => FlowFeatureStep::Preview(self.encode_preview(widgets, synapses, layout)),
                    Ok(FlowVcsPoll::PageReady { .. }) => {
                        self.phase = FlowVcsFeaturePhase::Lease;
                        FlowFeatureStep::Yield
                    }
                    Ok(FlowVcsPoll::Terminal) => {
                        self.phase = FlowVcsFeaturePhase::CloseOperation;
                        FlowFeatureStep::Yield
                    }
                    Err(fault) => {
                        self.terminal_failure = Some(flow_vcs_failure(fault));
                        self.phase = FlowVcsFeaturePhase::Fault;
                        FlowFeatureStep::Yield
                    }
                }
            }
            FlowVcsFeaturePhase::Fault => {
                let handle = self.handle.expect("admitted Flow VCS handle");
                let result = self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.fault(handle, grant));
                match result {
                    Ok(()) => {
                        self.phase = FlowVcsFeaturePhase::CloseOperation;
                        FlowFeatureStep::Yield
                    }
                    Err(fault) => FlowFeatureStep::Failed(flow_vcs_failure(fault)),
                }
            }
            FlowVcsFeaturePhase::Lease => {
                let handle = self.handle.expect("admitted Flow VCS handle");
                match self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.take_page(handle)) {
                    Ok(page) => {
                        self.page = Some(page);
                        self.phase = FlowVcsFeaturePhase::Resume;
                        FlowFeatureStep::Yield
                    }
                    Err(fault) => FlowFeatureStep::Failed(flow_vcs_failure(fault)),
                }
            }
            FlowVcsFeaturePhase::Resume => {
                let sequence = self.page.expect("leased Flow VCS page").sequence;
                let handle = self.handle.expect("admitted Flow VCS handle");
                match self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.resume_page(handle, sequence)) {
                    Ok(()) => {
                        self.phase = FlowVcsFeaturePhase::Retry;
                        FlowFeatureStep::Yield
                    }
                    Err(fault) => FlowFeatureStep::Failed(flow_vcs_failure(fault)),
                }
            }
            FlowVcsFeaturePhase::Retry => {
                let sequence = self.page.expect("retained Flow VCS page").sequence;
                let handle = self.handle.expect("admitted Flow VCS handle");
                match self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.retry_page(handle, sequence)) {
                    Ok(page) => {
                        self.page = Some(page);
                        self.phase = FlowVcsFeaturePhase::AwaitAcknowledgement;
                        FlowFeatureStep::RetainedPage(self.encode_page(page))
                    }
                    Err(fault) => FlowFeatureStep::Failed(flow_vcs_failure(fault)),
                }
            }
            FlowVcsFeaturePhase::AwaitAcknowledgement => FlowFeatureStep::Yield,
            FlowVcsFeaturePhase::CloseOperation | FlowVcsFeaturePhase::CloseRetired => match self.close_cursor_step(budget) {
                Ok(false) => FlowFeatureStep::Yield,
                Ok(true) if self.terminal_failure.is_some() => FlowFeatureStep::Failed(self.terminal_failure.as_ref().expect("terminal Flow VCS failure").clone()),
                Ok(true) => FlowFeatureStep::Complete(Vec::new()),
                Err(failure) => FlowFeatureStep::Failed(failure),
            },
            FlowVcsFeaturePhase::Complete if self.terminal_failure.is_some() => FlowFeatureStep::Failed(self.terminal_failure.as_ref().expect("terminal Flow VCS failure").clone()),
            FlowVcsFeaturePhase::Complete => FlowFeatureStep::Complete(Vec::new()),
        }
    }

    fn cancel(&mut self, budget: AbiWorkBudget) -> Result<(), FlowFailure> {
        if self.phase == FlowVcsFeaturePhase::Admit {
            self.phase = FlowVcsFeaturePhase::Complete;
            return Ok(());
        }
        let grant = self.grant(budget);
        let handle = self.handle.expect("admitted Flow VCS handle");
        self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.cancel(handle, grant)).map_err(flow_vcs_failure)?;
        self.phase = FlowVcsFeaturePhase::CloseOperation;
        Ok(())
    }

    fn preflight_acknowledge(&self, index: u32) -> Result<(), FlowFailure> {
        if index == 0 && self.phase == FlowVcsFeaturePhase::AwaitAcknowledgement && self.page.is_some() {
            Ok(())
        } else {
            Err(abi_failure(AbiErrorCode::OutOfOrderPage))
        }
    }

    fn acknowledge(&mut self, index: u32, _: AbiWorkBudget) -> Result<(), FlowFailure> {
        self.preflight_acknowledge(index)?;
        let sequence = self.page.expect("preflight retained Flow VCS page").sequence;
        let handle = self.handle.expect("admitted Flow VCS handle");
        self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.acknowledge_page(handle, sequence)).map_err(flow_vcs_failure)?;
        self.page = None;
        self.phase = FlowVcsFeaturePhase::Poll;
        Ok(())
    }

    fn close_step(&mut self, budget: AbiWorkBudget) -> Result<bool, FlowFailure> {
        if self.phase == FlowVcsFeaturePhase::Admit {
            self.phase = FlowVcsFeaturePhase::Complete;
            return Ok(true);
        }
        if self.phase == FlowVcsFeaturePhase::Poll {
            let grant = self.grant(budget);
            let handle = self.handle.expect("admitted Flow VCS handle");
            match self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.poll(handle, grant)).map_err(flow_vcs_failure)? {
                FlowVcsPoll::Terminal => {
                    self.phase = FlowVcsFeaturePhase::CloseOperation;
                    return Ok(false);
                }
                _ => return Err(abi_failure(AbiErrorCode::Busy)),
            }
        }
        self.close_cursor_step(budget)
    }
}

fn abi_code_failure(code: AbiErrorCode) -> FlowFailure {
    abi_failure(code)
}

fn flow_vcs_failure(fault: FlowVcsFault) -> FlowFailure {
    let code = match fault {
        FlowVcsFault::Closed => AbiErrorCode::Closed,
        FlowVcsFault::Full | FlowVcsFault::Limit | FlowVcsFault::Depth => AbiErrorCode::LimitExceeded,
        FlowVcsFault::WrongHandle | FlowVcsFault::StaleHandle => AbiErrorCode::UnknownHandle,
        FlowVcsFault::StaleAuthority => AbiErrorCode::StaleGeneration,
        FlowVcsFault::DuplicateControl => AbiErrorCode::DuplicateAcknowledgement,
        FlowVcsFault::InsufficientGrant => AbiErrorCode::NoCredit,
        FlowVcsFault::WrongPage => AbiErrorCode::OutOfOrderPage,
        _ => AbiErrorCode::MalformedTag,
    };
    FlowFailure::new(code, format!("Flow VCS {fault:?}"))
}

struct FlowAction2504 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2504 {
    fn operation(&self) -> u16 {
        2_504
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_504),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.catalogue_json().map(String::into_bytes).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_504),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2505 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2505 {
    fn operation(&self) -> u16 {
        2_505
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_505),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_host_catalogue_json(text(args, "json")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_505),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2506 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2506 {
    fn operation(&self) -> u16 {
        2_506
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_506),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_neuron_kind_infos_json(text(args, "json")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_506),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2507 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2507 {
    fn operation(&self) -> u16 {
        2_507
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_507),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        let widget = text(args, "widgetId")?;
                        let index = usize::try_from(integer(args, "index")?).map_err(|_| abi_failure(AbiErrorCode::LimitExceeded))?;
                        if 2_507 == 2_507 { domain.host.add_input_port(widget, index) } else { domain.host.add_output_port(widget, index) }.map(|_| Vec::new()).map_err(domain_error)
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_507),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2508 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2508 {
    fn operation(&self) -> u16 {
        2_508
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_508),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        let widget = text(args, "widgetId")?;
                        let port = text(args, "portId")?;
                        if 2_508 == 2_508 { domain.host.remove_input_port(widget, port) } else { domain.host.remove_output_port(widget, port) }.map(|_| Vec::new()).map_err(domain_error)
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_508),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2509 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2509 {
    fn operation(&self) -> u16 {
        2_509
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_509),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        let widget = text(args, "widgetId")?;
                        let index = usize::try_from(integer(args, "index")?).map_err(|_| abi_failure(AbiErrorCode::LimitExceeded))?;
                        if 2_509 == 2_507 { domain.host.add_input_port(widget, index) } else { domain.host.add_output_port(widget, index) }.map(|_| Vec::new()).map_err(domain_error)
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_509),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2510 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2510 {
    fn operation(&self) -> u16 {
        2_510
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_510),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        let widget = text(args, "widgetId")?;
                        let port = text(args, "portId")?;
                        if 2_510 == 2_508 { domain.host.remove_input_port(widget, port) } else { domain.host.remove_output_port(widget, port) }.map(|_| Vec::new()).map_err(domain_error)
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_510),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2511 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2511 {
    fn operation(&self) -> u16 {
        2_511
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_511),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.connect_ports(text(args, "fromId")?, text(args, "fromPort")?, text(args, "toId")?, text(args, "toPort")?).map(String::into_bytes).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_511),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2512 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2512 {
    fn operation(&self) -> u16 {
        2_512
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_512),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.compiled_wire_literal().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_512),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2513 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2513 {
    fn operation(&self) -> u16 {
        2_513
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_513),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.apply_eval_outputs_json(text(args, "json")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_513),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2514 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2514 {
    fn operation(&self) -> u16 {
        2_514
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_514),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        let json = text(args, "json")?;
                        let value: Value = serde_json::from_str(json).map_err(domain_error)?;
                        let active = value.get("active").and_then(Value::as_str);
                        let stale: Vec<String> = value.get("stale").and_then(Value::as_array).map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_owned)).collect()).unwrap_or_default();
                        domain.host.set_computing_progress(active, &stale);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_514),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2515 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2515 {
    fn operation(&self) -> u16 {
        2_515
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_515),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_node_statuses_from_json(text(args, "json")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_515),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2516 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2516 {
    fn operation(&self) -> u16 {
        2_516
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_516),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.clear_computing_widget_ids();
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_516),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2517 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2517 {
    fn operation(&self) -> u16 {
        2_517
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_517),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.preview_text().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_517),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2518 {
    program: FlowProgramState,
    selection: dag::DagSelectedNodesJsonCursor,
}

impl FlowActionState for FlowAction2518 {
    fn operation(&self) -> u16 {
        2_518
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_518),
            FlowProgramPhase::Domain => {
                self.program.domain_cursor += 1;
                let grant = dag::DagCursorGrant { fuel: 1, now_milliseconds: budget.now_ms, deadline_milliseconds: budget.deadline_ms.unwrap_or(u64::MAX), cancelled: budget.cancelled, interrupted: budget.interrupted };
                match self.selection.step(&domain.host.dag, grant) {
                    Ok(dag::DagCursorStep::Census { bytes }) => match self.program.begin_incremental_output(bytes) {
                        Ok(()) => FlowFeatureStep::Progress { completed: 0, total: bytes as u64 },
                        Err(failure) => FlowFeatureStep::Failed(failure),
                    },
                    Ok(dag::DagCursorStep::Byte(byte)) => {
                        self.program.output.push(byte);
                        FlowFeatureStep::Progress { completed: self.program.output.len() as u64, total: self.program.output.capacity() as u64 }
                    }
                    Ok(dag::DagCursorStep::Progress { completed, total }) => FlowFeatureStep::Progress { completed: completed as u64, total: total as u64 },
                    Ok(dag::DagCursorStep::Complete) => self.program.finish_incremental_output(),
                    Err(fault) => FlowFeatureStep::Failed(dag_cursor_failure(fault)),
                }
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_518),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2519 {
    program: FlowProgramState,
    selection: dag::DagSelectedNodesJsonCursor,
}

impl FlowActionState for FlowAction2519 {
    fn operation(&self) -> u16 {
        2_519
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_519),
            FlowProgramPhase::Domain => {
                self.program.domain_cursor += 1;
                let grant = dag::DagCursorGrant { fuel: 1, now_milliseconds: budget.now_ms, deadline_milliseconds: budget.deadline_ms.unwrap_or(u64::MAX), cancelled: budget.cancelled, interrupted: budget.interrupted };
                match self.selection.step(&domain.host.dag, grant) {
                    Ok(dag::DagCursorStep::Census { bytes }) => match self.program.begin_incremental_output(bytes) {
                        Ok(()) => FlowFeatureStep::Progress { completed: 0, total: bytes as u64 },
                        Err(failure) => FlowFeatureStep::Failed(failure),
                    },
                    Ok(dag::DagCursorStep::Byte(byte)) => {
                        self.program.output.push(byte);
                        FlowFeatureStep::Progress { completed: self.program.output.len() as u64, total: self.program.output.capacity() as u64 }
                    }
                    Ok(dag::DagCursorStep::Progress { completed, total }) => FlowFeatureStep::Progress { completed: completed as u64, total: total as u64 },
                    Ok(dag::DagCursorStep::Complete) => self.program.finish_incremental_output(),
                    Err(fault) => FlowFeatureStep::Failed(dag_cursor_failure(fault)),
                }
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_519),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2520 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2520 {
    fn operation(&self) -> u16 {
        2_520
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_520),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.selection_domains_json().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_520),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2521 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2521 {
    fn operation(&self) -> u16 {
        2_521
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_521),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.hovered_widget_id().unwrap_or_default().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_521),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2522 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2522 {
    fn operation(&self) -> u16 {
        2_522
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_522),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.hovered_channel_json().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_522),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2523 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2523 {
    fn operation(&self) -> u16 {
        2_523
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_523),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.selected_channels_json().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_523),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2524 {
    program: FlowProgramState,
    items: FlowStringArrayCursor,
}

impl FlowActionState for FlowAction2524 {
    fn operation(&self) -> u16 {
        2_524
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_524),
            FlowProgramPhase::Domain => {
                self.program.domain_cursor += 1;
                match self.items.step(&FlowPreviewOffSource(&domain.host)) {
                    Ok(FlowStringArrayStep::Census { bytes }) => match self.program.begin_incremental_output(bytes) {
                        Ok(()) => FlowFeatureStep::Progress { completed: 0, total: bytes as u64 },
                        Err(failure) => FlowFeatureStep::Failed(failure),
                    },
                    Ok(FlowStringArrayStep::Byte(byte)) => {
                        self.program.output.push(byte);
                        FlowFeatureStep::Progress { completed: self.program.output.len() as u64, total: self.program.output.capacity() as u64 }
                    }
                    Ok(FlowStringArrayStep::Progress { completed, total }) => FlowFeatureStep::Progress { completed: completed as u64, total: total as u64 },
                    Ok(FlowStringArrayStep::Complete) => self.program.finish_incremental_output(),
                    Err(failure) => FlowFeatureStep::Failed(failure),
                }
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_524),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2525 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2525 {
    fn operation(&self) -> u16 {
        2_525
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_525),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_selection_json(text(args, "json")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_525),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2526 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2526 {
    fn operation(&self) -> u16 {
        2_526
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_526),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_hover(optional_text(args, "widgetId")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_526),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2527 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2527 {
    fn operation(&self) -> u16 {
        2_527
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_527),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_hover_channel(optional_text(args, "widgetId")?, optional_text(args, "port")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_527),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2528 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2528 {
    fn operation(&self) -> u16 {
        2_528
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_528),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_selected_channels_json(text(args, "json")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_528),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2529 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2529 {
    fn operation(&self) -> u16 {
        2_529
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_529),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_preview_off_json(text(args, "json")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_529),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2530 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2530 {
    fn operation(&self) -> u16 {
        2_530
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_530),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.toggle_preview(text(args, "widgetId")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_530),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2531 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2531 {
    fn operation(&self) -> u16 {
        2_531
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_531),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        let ids: Vec<String> = serde_json::from_str(text(args, "json")?).map_err(domain_error)?;
                        domain.host.collapse_selection(&ids).map(String::into_bytes).map_err(domain_error)
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_531),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2532 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2532 {
    fn operation(&self) -> u16 {
        2_532
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_532),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.explode_cluster(text(args, "clusterId")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_532),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2533 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2533 {
    fn operation(&self) -> u16 {
        2_533
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_533),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.take_pending_export_click().unwrap_or_default().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_533),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2534 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2534 {
    fn operation(&self) -> u16 {
        2_534
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_534),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.export_payload_json(text(args, "widgetId")?).map(String::into_bytes).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_534),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2535 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2535 {
    fn operation(&self) -> u16 {
        2_535
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_535),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.take_pending_cluster_explode().unwrap_or_default().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_535),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2536 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2536 {
    fn operation(&self) -> u16 {
        2_536
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_536),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_slider_value(text(args, "widgetId")?, number(args, "value")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_536),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2537 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2537 {
    fn operation(&self) -> u16 {
        2_537
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_537),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.slider_overlay_state_json().map(String::into_bytes).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_537),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2538 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2538 {
    fn operation(&self) -> u16 {
        2_538
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_538),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_note_text(text(args, "widgetId")?, text(args, "text")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_538),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2539 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2539 {
    fn operation(&self) -> u16 {
        2_539
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_539),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.begin_note_edit(text(args, "widgetId")?, number(args, "worldX")?, number(args, "worldY")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_539),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2540 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2540 {
    fn operation(&self) -> u16 {
        2_540
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_540),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.note_insert_text(text(args, "chunk")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_540),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2541 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2541 {
    fn operation(&self) -> u16 {
        2_541
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_541),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.note_backspace();
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_541),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2542 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2542 {
    fn operation(&self) -> u16 {
        2_542
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_542),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.note_delete_forward();
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_542),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2543 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2543 {
    fn operation(&self) -> u16 {
        2_543
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_543),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.note_move_caret(text(args, "direction")?, boolean(args, "extend")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_543),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2544 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2544 {
    fn operation(&self) -> u16 {
        2_544
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_544),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.note_commit_edit();
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_544),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2545 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2545 {
    fn operation(&self) -> u16 {
        2_545
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_545),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_note_caret_visible(boolean(args, "visible")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_545),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2546 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2546 {
    fn operation(&self) -> u16 {
        2_546
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_546),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_image_src(text(args, "widgetId")?, text(args, "src")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_546),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2547 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2547 {
    fn operation(&self) -> u16 {
        2_547
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_547),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.schemas_json().map(String::into_bytes).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_547),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2548 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2548 {
    fn operation(&self) -> u16 {
        2_548
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_548),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_variable_name(text(args, "widgetId")?, text(args, "name")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_548),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2549 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2549 {
    fn operation(&self) -> u16 {
        2_549
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_549),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_variable_schema(text(args, "widgetId")?, text(args, "schema")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_549),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2550 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2550 {
    fn operation(&self) -> u16 {
        2_550
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_550),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.add_widget(text(args, "descriptorJson")?, number(args, "worldX")?, number(args, "worldY")?).map(String::into_bytes).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_550),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2551 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2551 {
    fn operation(&self) -> u16 {
        2_551
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_551),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.set_ghost_widget(text(args, "descriptorJson")?, number(args, "worldX")?, number(args, "worldY")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_551),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2552 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2552 {
    fn operation(&self) -> u16 {
        2_552
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_552),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.clear_ghost_widget();
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_552),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2553 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2553 {
    fn operation(&self) -> u16 {
        2_553
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_553),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.remove_widget(text(args, "widgetId")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_553),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2554 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2554 {
    fn operation(&self) -> u16 {
        2_554
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_554),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.move_widget(text(args, "widgetId")?, number(args, "x")?, number(args, "y")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_554),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2555 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2555 {
    fn operation(&self) -> u16 {
        2_555
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_555),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> =
                    flow_result! { domain.host.insert_between(text(args, "anchorId")?, text(args, "anchorOutPort")?, text(args, "midId")?, text(args, "midInPort")?, text(args, "midOutPort")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_555),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2556 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2556 {
    fn operation(&self) -> u16 {
        2_556
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_556),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.make_space(text(args, "anchorId")?, number(args, "dx")?, number(args, "dy")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_556),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2557 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2557 {
    fn operation(&self) -> u16 {
        2_557
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_557),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.set_neuron_params(text(args, "widgetId")?, text(args, "paramsJson")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_557),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2558 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2558 {
    fn operation(&self) -> u16 {
        2_558
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_558),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.connect(text(args, "fromId")?, text(args, "toId")?).map(String::into_bytes).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_558),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2559 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2559 {
    fn operation(&self) -> u16 {
        2_559
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_559),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.disconnect(text(args, "synapseId")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_559),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2560 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2560 {
    fn operation(&self) -> u16 {
        2_560
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_560),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(vec![u8::from(domain.host.undo())]) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_560),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2561 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2561 {
    fn operation(&self) -> u16 {
        2_561
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_561),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(vec![u8::from(domain.host.redo())]) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_561),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2562 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2562 {
    fn operation(&self) -> u16 {
        2_562
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_562),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(vec![u8::from(domain.host.can_undo())]) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_562),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2563 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2563 {
    fn operation(&self) -> u16 {
        2_563
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_563),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(vec![u8::from(domain.host.can_redo())]) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_563),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2564 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2564 {
    fn operation(&self) -> u16 {
        2_564
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_564),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        let (x, y) = domain.host.world_from_screen(number(args, "sx")?, number(args, "sy")?);
                        Ok(format!("{{\"x\":{x},\"y\":{y}}}").into_bytes())
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_564),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2565 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2565 {
    fn operation(&self) -> u16 {
        2_565
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_565),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_camera(number(args, "x")?, number(args, "y")?, number(args, "zoom")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_565),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2566 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2566 {
    fn operation(&self) -> u16 {
        2_566
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_566),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let camera = &domain.host.fixture.camera;
                let result: Result<Vec<u8>, FlowFailure> = Ok(format!("{{\"x\":{},\"y\":{},\"zoom\":{}}}", camera.x, camera.y, camera.zoom).into_bytes());
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_566),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2567 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2567 {
    fn operation(&self) -> u16 {
        2_567
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_567),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.wheel_screen(number(args, "sx")?, number(args, "sy")?, number(args, "deltaX")?, number(args, "deltaY")?, boolean(args, "zoomGesture")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_567),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2568 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2568 {
    fn operation(&self) -> u16 {
        2_568
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_568),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.dag.set_wheel_zoom_active(boolean(args, "active")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_568),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2569 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2569 {
    fn operation(&self) -> u16 {
        2_569
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_569),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(dag::dag_lod_scale_json().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_569),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2570 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2570 {
    fn operation(&self) -> u16 {
        2_570
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_570),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_automatic_lod(boolean(args, "enabled")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_570),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2571 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2571 {
    fn operation(&self) -> u16 {
        2_571
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_571),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_proximity_distance(number(args, "world")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_571),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2572 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2572 {
    fn operation(&self) -> u16 {
        2_572
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_572),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_forced_draw_lod_label(text(args, "label")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_572),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2573 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2573 {
    fn operation(&self) -> u16 {
        2_573
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_573),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.draw_lod_label().as_bytes().to_vec()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_573),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2574 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2574 {
    fn operation(&self) -> u16 {
        2_574
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_574),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.label_overlay_paint_state_json().map(String::into_bytes).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_574),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2575 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2575 {
    fn operation(&self) -> u16 {
        2_575
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_575),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.attach_surface(args) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_575),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2576 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2576 {
    fn operation(&self) -> u16 {
        2_576
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_576),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.apply_surface_status(args) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_576),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2577 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2577 {
    fn operation(&self) -> u16 {
        2_577
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_577),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(vec![u8::from(domain.surface.is_some_and(|surface| surface.state == SurfaceState::Ready))]) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_577),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2578 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2578 {
    fn operation(&self) -> u16 {
        2_578
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_578),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.set_size(u32_integer(args, "width")?, u32_integer(args, "height")?, number(args, "dpr")?)?;
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_578),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2579 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2579 {
    fn operation(&self) -> u16 {
        2_579
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_579),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_canvas_theme_from_json(text(args, "json")?).map_err(domain_error)?;
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_579),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2580 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2580 {
    fn operation(&self) -> u16 {
        2_580
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_580),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.reorganize(text(args, "json")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_580),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2581 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2581 {
    fn operation(&self) -> u16 {
        2_581
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_581),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.render_frame() };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_581),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2582 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2582 {
    fn operation(&self) -> u16 {
        2_582
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_582),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.pointer_down_screen(number(args, "sx")?, number(args, "sy")?, u8_integer(args, "button")?, boolean(args, "shift")?, boolean(args, "ctrlOrMeta")?, boolean(args, "alt")?, boolean(args, "pan")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_582),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2583 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2583 {
    fn operation(&self) -> u16 {
        2_583
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_583),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.pointer_move_screen(number(args, "sx")?, number(args, "sy")?, boolean(args, "shift")?, boolean(args, "ctrlOrMeta")?, boolean(args, "alt")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_583),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2584 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2584 {
    fn operation(&self) -> u16 {
        2_584
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_584),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.pick_targets_at_screen_json(number(args, "sx")?, number(args, "sy")?).into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_584),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2585 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2585 {
    fn operation(&self) -> u16 {
        2_585
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_585),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.entity_screen_json(text(args, "domain")?, text(args, "id")?).into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_585),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2586 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2586 {
    fn operation(&self) -> u16 {
        2_586
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_586),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(vec![u8::from(domain.host.widget_drag_active())]) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_586),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2587 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2587 {
    fn operation(&self) -> u16 {
        2_587
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_587),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.pointer_up_screen(number(args, "sx")?, number(args, "sy")?, boolean(args, "shift")?, boolean(args, "ctrlOrMeta")?, boolean(args, "alt")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_587),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2588 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2588 {
    fn operation(&self) -> u16 {
        2_588
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_588),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_selection_options(text(args, "method")?, text(args, "mode")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_588),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2589 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2589 {
    fn operation(&self) -> u16 {
        2_589
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_589),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.selection_preview_points_json().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_589),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2590 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2590 {
    fn operation(&self) -> u16 {
        2_590
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_590),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(vec![u8::from(domain.host.selection_preview_crossing())]) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_590),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2591 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2591 {
    fn operation(&self) -> u16 {
        2_591
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_591),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.selection_preview_method().as_bytes().to_vec()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_591),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2592 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2592 {
    fn operation(&self) -> u16 {
        2_592
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_592),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.selection_union_bounds_screen_json().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_592),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2593 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2593 {
    fn operation(&self) -> u16 {
        2_593
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_593),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.align_selection(text(args, "mode")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_593),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2594 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2594 {
    fn operation(&self) -> u16 {
        2_594
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_594),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.preselect_widget_ids_json().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_594),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2595 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2595 {
    fn operation(&self) -> u16 {
        2_595
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_595),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(vec![u8::from(domain.host.cancel_area_select())]) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_595),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2596 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2596 {
    fn operation(&self) -> u16 {
        2_596
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_596),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.delete_selection().map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_596),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2597 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2597 {
    fn operation(&self) -> u16 {
        2_597
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_597),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(vec![u8::from(domain.host.has_selection())]) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_597),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2598 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2598 {
    fn operation(&self) -> u16 {
        2_598
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_598),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.select_all();
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_598),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2599 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2599 {
    fn operation(&self) -> u16 {
        2_599
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_599),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        let value = match crate::tessellate_geometry(text(args, "handle")?, number(args, "tolerance")?) {
                            Ok(mesh) => serde_json::to_string(&mesh).map_err(domain_error)?,
                            Err(error) => json!({ "error": error }).to_string(),
                        };
                        Ok(value.into_bytes())
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_599),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2600 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2600 {
    fn operation(&self) -> u16 {
        2_600
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_600),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(render_scene_json(text(args, "handle")?).into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_600),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2601 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2601 {
    fn operation(&self) -> u16 {
        2_601
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_601),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(export_svg_json(text(args, "handle")?).into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_601),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2602 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2602 {
    fn operation(&self) -> u16 {
        2_602
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_602),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(export_pdf_json(text(args, "handle")?).into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_602),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2603 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2603 {
    fn operation(&self) -> u16 {
        2_603
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_603),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(export_dwg_json(text(args, "handle")?).into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_603),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2604 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2604 {
    fn operation(&self) -> u16 {
        2_604
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_604),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(import_dwg_json(text(args, "dataBase64")?).into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_604),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2605 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2605 {
    fn operation(&self) -> u16 {
        2_605
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_605),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        let mask = bytes(args, "mask")?;
                        Ok(trace_bitmap_json(u32_integer(args, "width")?, u32_integer(args, "height")?, &mask, number(args, "threshold")?, number(args, "simplifyEpsilon")?).into_bytes())
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_605),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2606 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2606 {
    fn operation(&self) -> u16 {
        2_606
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_606),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(boolean_segments_json(text(args, "aJson")?, text(args, "bJson")?, text(args, "operation")?).into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_606),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2607 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2607 {
    fn operation(&self) -> u16 {
        2_607
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_607),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        crate::dispose_geometry(text(args, "handle")?);
                        dispose_drawing(text(args, "handle")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_607),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2608 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2608 {
    fn operation(&self) -> u16 {
        2_608
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_608),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(dwg_encode_mesh(text(args, "meshJson")?).into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_608),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2609 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2609 {
    fn operation(&self) -> u16 {
        2_609
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_609),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.fixture_json().map(String::into_bytes).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_609),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

struct FlowAction2610 {
    program: FlowProgramState,
}

impl FlowActionState for FlowAction2610 {
    fn operation(&self) -> u16 {
        2_610
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_610),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        let fixture = FlowHost::parse_fixture_json(text(args, "json")?).map_err(domain_error)?;
                        domain.host.resync_fixture_from_scene(fixture);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_610),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

fn flow_action(operation: u16, arguments: &FlowArguments) -> Option<Box<dyn FlowActionState>> {
    match operation {
        2_504 => Some(Box::new(FlowAction2504 { program: FlowProgramState::new(arguments) })),
        2_505 => Some(Box::new(FlowAction2505 { program: FlowProgramState::new(arguments) })),
        2_506 => Some(Box::new(FlowAction2506 { program: FlowProgramState::new(arguments) })),
        2_507 => Some(Box::new(FlowAction2507 { program: FlowProgramState::new(arguments) })),
        2_508 => Some(Box::new(FlowAction2508 { program: FlowProgramState::new(arguments) })),
        2_509 => Some(Box::new(FlowAction2509 { program: FlowProgramState::new(arguments) })),
        2_510 => Some(Box::new(FlowAction2510 { program: FlowProgramState::new(arguments) })),
        2_511 => Some(Box::new(FlowAction2511 { program: FlowProgramState::new(arguments) })),
        2_512 => Some(Box::new(FlowAction2512 { program: FlowProgramState::new(arguments) })),
        2_513 => Some(Box::new(FlowAction2513 { program: FlowProgramState::new(arguments) })),
        2_514 => Some(Box::new(FlowAction2514 { program: FlowProgramState::new(arguments) })),
        2_515 => Some(Box::new(FlowAction2515 { program: FlowProgramState::new(arguments) })),
        2_516 => Some(Box::new(FlowAction2516 { program: FlowProgramState::new(arguments) })),
        2_517 => Some(Box::new(FlowAction2517 { program: FlowProgramState::new(arguments) })),
        2_518 => Some(Box::new(FlowAction2518 { program: FlowProgramState::new(arguments), selection: dag::DagSelectedNodesJsonCursor::default() })),
        2_519 => Some(Box::new(FlowAction2519 { program: FlowProgramState::new(arguments), selection: dag::DagSelectedNodesJsonCursor::edges() })),
        2_520 => Some(Box::new(FlowAction2520 { program: FlowProgramState::new(arguments) })),
        2_521 => Some(Box::new(FlowAction2521 { program: FlowProgramState::new(arguments) })),
        2_522 => Some(Box::new(FlowAction2522 { program: FlowProgramState::new(arguments) })),
        2_523 => Some(Box::new(FlowAction2523 { program: FlowProgramState::new(arguments) })),
        2_524 => Some(Box::new(FlowAction2524 { program: FlowProgramState::new(arguments), items: FlowStringArrayCursor::default() })),
        2_525 => Some(Box::new(FlowAction2525 { program: FlowProgramState::new(arguments) })),
        2_526 => Some(Box::new(FlowAction2526 { program: FlowProgramState::new(arguments) })),
        2_527 => Some(Box::new(FlowAction2527 { program: FlowProgramState::new(arguments) })),
        2_528 => Some(Box::new(FlowAction2528 { program: FlowProgramState::new(arguments) })),
        2_529 => Some(Box::new(FlowAction2529 { program: FlowProgramState::new(arguments) })),
        2_530 => Some(Box::new(FlowAction2530 { program: FlowProgramState::new(arguments) })),
        2_531 => Some(Box::new(FlowAction2531 { program: FlowProgramState::new(arguments) })),
        2_532 => Some(Box::new(FlowAction2532 { program: FlowProgramState::new(arguments) })),
        2_533 => Some(Box::new(FlowAction2533 { program: FlowProgramState::new(arguments) })),
        2_534 => Some(Box::new(FlowAction2534 { program: FlowProgramState::new(arguments) })),
        2_535 => Some(Box::new(FlowAction2535 { program: FlowProgramState::new(arguments) })),
        2_536 => Some(Box::new(FlowAction2536 { program: FlowProgramState::new(arguments) })),
        2_537 => Some(Box::new(FlowAction2537 { program: FlowProgramState::new(arguments) })),
        2_538 => Some(Box::new(FlowAction2538 { program: FlowProgramState::new(arguments) })),
        2_539 => Some(Box::new(FlowAction2539 { program: FlowProgramState::new(arguments) })),
        2_540 => Some(Box::new(FlowAction2540 { program: FlowProgramState::new(arguments) })),
        2_541 => Some(Box::new(FlowAction2541 { program: FlowProgramState::new(arguments) })),
        2_542 => Some(Box::new(FlowAction2542 { program: FlowProgramState::new(arguments) })),
        2_543 => Some(Box::new(FlowAction2543 { program: FlowProgramState::new(arguments) })),
        2_544 => Some(Box::new(FlowAction2544 { program: FlowProgramState::new(arguments) })),
        2_545 => Some(Box::new(FlowAction2545 { program: FlowProgramState::new(arguments) })),
        2_546 => Some(Box::new(FlowAction2546 { program: FlowProgramState::new(arguments) })),
        2_547 => Some(Box::new(FlowAction2547 { program: FlowProgramState::new(arguments) })),
        2_548 => Some(Box::new(FlowAction2548 { program: FlowProgramState::new(arguments) })),
        2_549 => Some(Box::new(FlowAction2549 { program: FlowProgramState::new(arguments) })),
        2_550 => Some(Box::new(FlowAction2550 { program: FlowProgramState::new(arguments) })),
        2_551 => Some(Box::new(FlowAction2551 { program: FlowProgramState::new(arguments) })),
        2_552 => Some(Box::new(FlowAction2552 { program: FlowProgramState::new(arguments) })),
        2_553 => Some(Box::new(FlowAction2553 { program: FlowProgramState::new(arguments) })),
        2_554 => Some(Box::new(FlowAction2554 { program: FlowProgramState::new(arguments) })),
        2_555 => Some(Box::new(FlowAction2555 { program: FlowProgramState::new(arguments) })),
        2_556 => Some(Box::new(FlowAction2556 { program: FlowProgramState::new(arguments) })),
        2_557 => Some(Box::new(FlowAction2557 { program: FlowProgramState::new(arguments) })),
        2_558 => Some(Box::new(FlowAction2558 { program: FlowProgramState::new(arguments) })),
        2_559 => Some(Box::new(FlowAction2559 { program: FlowProgramState::new(arguments) })),
        2_560 => Some(Box::new(FlowAction2560 { program: FlowProgramState::new(arguments) })),
        2_561 => Some(Box::new(FlowAction2561 { program: FlowProgramState::new(arguments) })),
        2_562 => Some(Box::new(FlowAction2562 { program: FlowProgramState::new(arguments) })),
        2_563 => Some(Box::new(FlowAction2563 { program: FlowProgramState::new(arguments) })),
        2_564 => Some(Box::new(FlowAction2564 { program: FlowProgramState::new(arguments) })),
        2_565 => Some(Box::new(FlowAction2565 { program: FlowProgramState::new(arguments) })),
        2_566 => Some(Box::new(FlowAction2566 { program: FlowProgramState::new(arguments) })),
        2_567 => Some(Box::new(FlowAction2567 { program: FlowProgramState::new(arguments) })),
        2_568 => Some(Box::new(FlowAction2568 { program: FlowProgramState::new(arguments) })),
        2_569 => Some(Box::new(FlowAction2569 { program: FlowProgramState::new(arguments) })),
        2_570 => Some(Box::new(FlowAction2570 { program: FlowProgramState::new(arguments) })),
        2_571 => Some(Box::new(FlowAction2571 { program: FlowProgramState::new(arguments) })),
        2_572 => Some(Box::new(FlowAction2572 { program: FlowProgramState::new(arguments) })),
        2_573 => Some(Box::new(FlowAction2573 { program: FlowProgramState::new(arguments) })),
        2_574 => Some(Box::new(FlowAction2574 { program: FlowProgramState::new(arguments) })),
        2_575 => Some(Box::new(FlowAction2575 { program: FlowProgramState::new(arguments) })),
        2_576 => Some(Box::new(FlowAction2576 { program: FlowProgramState::new(arguments) })),
        2_577 => Some(Box::new(FlowAction2577 { program: FlowProgramState::new(arguments) })),
        2_578 => Some(Box::new(FlowAction2578 { program: FlowProgramState::new(arguments) })),
        2_579 => Some(Box::new(FlowAction2579 { program: FlowProgramState::new(arguments) })),
        2_580 => Some(Box::new(FlowAction2580 { program: FlowProgramState::new(arguments) })),
        2_581 => Some(Box::new(FlowAction2581 { program: FlowProgramState::new(arguments) })),
        2_582 => Some(Box::new(FlowAction2582 { program: FlowProgramState::new(arguments) })),
        2_583 => Some(Box::new(FlowAction2583 { program: FlowProgramState::new(arguments) })),
        2_584 => Some(Box::new(FlowAction2584 { program: FlowProgramState::new(arguments) })),
        2_585 => Some(Box::new(FlowAction2585 { program: FlowProgramState::new(arguments) })),
        2_586 => Some(Box::new(FlowAction2586 { program: FlowProgramState::new(arguments) })),
        2_587 => Some(Box::new(FlowAction2587 { program: FlowProgramState::new(arguments) })),
        2_588 => Some(Box::new(FlowAction2588 { program: FlowProgramState::new(arguments) })),
        2_589 => Some(Box::new(FlowAction2589 { program: FlowProgramState::new(arguments) })),
        2_590 => Some(Box::new(FlowAction2590 { program: FlowProgramState::new(arguments) })),
        2_591 => Some(Box::new(FlowAction2591 { program: FlowProgramState::new(arguments) })),
        2_592 => Some(Box::new(FlowAction2592 { program: FlowProgramState::new(arguments) })),
        2_593 => Some(Box::new(FlowAction2593 { program: FlowProgramState::new(arguments) })),
        2_594 => Some(Box::new(FlowAction2594 { program: FlowProgramState::new(arguments) })),
        2_595 => Some(Box::new(FlowAction2595 { program: FlowProgramState::new(arguments) })),
        2_596 => Some(Box::new(FlowAction2596 { program: FlowProgramState::new(arguments) })),
        2_597 => Some(Box::new(FlowAction2597 { program: FlowProgramState::new(arguments) })),
        2_598 => Some(Box::new(FlowAction2598 { program: FlowProgramState::new(arguments) })),
        2_599 => Some(Box::new(FlowAction2599 { program: FlowProgramState::new(arguments) })),
        2_600 => Some(Box::new(FlowAction2600 { program: FlowProgramState::new(arguments) })),
        2_601 => Some(Box::new(FlowAction2601 { program: FlowProgramState::new(arguments) })),
        2_602 => Some(Box::new(FlowAction2602 { program: FlowProgramState::new(arguments) })),
        2_603 => Some(Box::new(FlowAction2603 { program: FlowProgramState::new(arguments) })),
        2_604 => Some(Box::new(FlowAction2604 { program: FlowProgramState::new(arguments) })),
        2_605 => Some(Box::new(FlowAction2605 { program: FlowProgramState::new(arguments) })),
        2_606 => Some(Box::new(FlowAction2606 { program: FlowProgramState::new(arguments) })),
        2_607 => Some(Box::new(FlowAction2607 { program: FlowProgramState::new(arguments) })),
        2_608 => Some(Box::new(FlowAction2608 { program: FlowProgramState::new(arguments) })),
        2_609 => Some(Box::new(FlowAction2609 { program: FlowProgramState::new(arguments) })),
        2_610 => Some(Box::new(FlowAction2610 { program: FlowProgramState::new(arguments) })),
        _ => None,
    }
}

impl FlowDomainAdapter {
    fn attach_surface(&mut self, args: &FlowArguments) -> Result<Vec<u8>, FlowFailure> {
        let id = SurfaceId::try_new(u32_integer(args, "surface")?).map_err(surface_abi_failure)?;
        let generation = SurfaceGeneration::try_new(u32_integer(args, "surfaceGeneration")?).map_err(surface_abi_failure)?;
        if let Some(current) = self.surface {
            return Err(FlowFailure::new(if current.id == id && current.generation == generation { AbiErrorCode::Busy } else { AbiErrorCode::StaleGeneration }, "Flow surface requires exact close before replacement"));
        }
        let width = u32_integer(args, "width")?;
        let height = u32_integer(args, "height")?;
        let dpr = number(args, "dpr")?;
        let metrics = CanvasMetrics::try_new(width, height, dpr as f32).map_err(surface_abi_failure)?;
        self.width = width;
        self.height = height;
        self.dpr = dpr;
        self.host.set_viewport(width, height, dpr);
        self.surface = Some(FlowSurface { id, generation, metrics, state: SurfaceState::Pending });
        Ok(self.surface_status_bytes())
    }

    fn apply_surface_status(&mut self, args: &FlowArguments) -> Result<Vec<u8>, FlowFailure> {
        let id = SurfaceId::try_new(u32_integer(args, "surface")?).map_err(surface_abi_failure)?;
        let generation = SurfaceGeneration::try_new(u32_integer(args, "surfaceGeneration")?).map_err(surface_abi_failure)?;
        let current = self.surface.as_mut().ok_or_else(|| abi_failure(AbiErrorCode::UnknownHandle))?;
        if current.id != id {
            return Err(abi_failure(AbiErrorCode::UnknownHandle));
        }
        if generation.get() < current.generation.get() {
            return Err(abi_failure(AbiErrorCode::AbaHandle));
        }
        if generation != current.generation {
            return Err(abi_failure(AbiErrorCode::StaleGeneration));
        }
        match text(args, "status")? {
            "created" | "recovered" => current.state = SurfaceState::Ready,
            "lost" | "device-lost" => current.state = SurfaceState::Lost,
            "cancelled" | "rejected" => self.surface = None,
            _ => return Err(abi_failure(AbiErrorCode::MalformedTag)),
        }
        Ok(self.surface_status_bytes())
    }

    fn set_size(&mut self, width: u32, height: u32, dpr: f64) -> Result<(), FlowFailure> {
        let metrics = CanvasMetrics::try_new(width, height, dpr as f32).map_err(surface_abi_failure)?;
        self.width = width;
        self.height = height;
        self.dpr = dpr;
        self.host.set_viewport(width, height, dpr);
        if let Some(surface) = self.surface.as_mut() {
            surface.metrics = metrics;
        }
        Ok(())
    }

    fn render_frame(&mut self) -> Result<Vec<u8>, FlowFailure> {
        let surface = self.surface.filter(|surface| surface.state == SurfaceState::Ready).ok_or_else(|| abi_failure(AbiErrorCode::UnknownHandle))?;
        self.host.sync_dag_ghost();
        let mut scene = canvas::Scene::new();
        self.host.paint_scene(&mut scene, self.width, self.height, self.dpr);
        let fixture = self.host.fixture_json().map_err(domain_error)?;
        let labels = self.host.label_overlay_paint_state_json().map_err(domain_error)?;
        Ok(format!("{{\"surface\":{},\"surfaceGeneration\":{},\"width\":{},\"height\":{},\"dpr\":{},\"fixture\":{fixture},\"labels\":{labels}}}", surface.id.get(), surface.generation.get(), self.width, self.height, self.dpr).into_bytes())
    }

    fn surface_status_bytes(&self) -> Vec<u8> {
        self.surface
            .map(|surface| {
                let status = match surface.state {
                    SurfaceState::Pending => "pending",
                    SurfaceState::Ready => "created",
                    SurfaceState::Lost => "lost",
                };
                format!("{{\"surface\":{},\"surfaceGeneration\":{},\"status\":\"{status}\"}}", surface.id.get(), surface.generation.get()).into_bytes()
            })
            .unwrap_or_else(|| br#"{"status":"cancelled"}"#.to_vec())
    }
}

fn text<'a>(args: &'a FlowArguments, key: &str) -> Result<&'a str, FlowFailure> {
    let span = args.get(key).ok_or_else(|| FlowFailure::new(AbiErrorCode::MissingField, key))?;
    if !matches!(span.kind, FlowArgumentKind::Text) {
        return Err(FlowFailure::new(AbiErrorCode::MalformedTag, key));
    }
    std::str::from_utf8(args.bytes(span)).map_err(|_| FlowFailure::new(AbiErrorCode::InvalidUtf8, key))
}

fn optional_text<'a>(args: &'a FlowArguments, key: &str) -> Result<Option<&'a str>, FlowFailure> {
    let span = args.get(key).ok_or_else(|| FlowFailure::new(AbiErrorCode::MissingField, key))?;
    if !matches!(span.kind, FlowArgumentKind::OptionalText) {
        return Err(FlowFailure::new(AbiErrorCode::MalformedTag, key));
    }
    span.present.then(|| std::str::from_utf8(args.bytes(span)).map_err(|_| FlowFailure::new(AbiErrorCode::InvalidUtf8, key))).transpose()
}

fn number(args: &FlowArguments, key: &str) -> Result<f64, FlowFailure> {
    let span = args.get(key).ok_or_else(|| FlowFailure::new(AbiErrorCode::MissingField, key))?;
    if !matches!(span.kind, FlowArgumentKind::Number) {
        return Err(FlowFailure::new(AbiErrorCode::MalformedTag, key));
    }
    let value = f64::from_le_bytes(args.bytes(span).try_into().map_err(|_| FlowFailure::new(AbiErrorCode::MalformedLength, key))?);
    value.is_finite().then_some(value).ok_or_else(|| FlowFailure::new(AbiErrorCode::MalformedTag, key))
}

fn integer(args: &FlowArguments, key: &str) -> Result<u64, FlowFailure> {
    let span = args.get(key).ok_or_else(|| FlowFailure::new(AbiErrorCode::MissingField, key))?;
    match span.kind {
        FlowArgumentKind::Integer => Ok(u64::from_le_bytes(args.bytes(span).try_into().map_err(|_| FlowFailure::new(AbiErrorCode::MalformedLength, key))?)),
        FlowArgumentKind::U32 => Ok(u32::from_le_bytes(args.bytes(span).try_into().map_err(|_| FlowFailure::new(AbiErrorCode::MalformedLength, key))?) as u64),
        FlowArgumentKind::U8 => Ok(u64::from(args.bytes(span)[0])),
        _ => Err(FlowFailure::new(AbiErrorCode::MalformedTag, key)),
    }
}

fn u32_integer(args: &FlowArguments, key: &str) -> Result<u32, FlowFailure> {
    u32::try_from(integer(args, key)?).map_err(|_| abi_failure(AbiErrorCode::LimitExceeded))
}

fn u8_integer(args: &FlowArguments, key: &str) -> Result<u8, FlowFailure> {
    u8::try_from(integer(args, key)?).map_err(|_| abi_failure(AbiErrorCode::LimitExceeded))
}

fn boolean(args: &FlowArguments, key: &str) -> Result<bool, FlowFailure> {
    let span = args.get(key).ok_or_else(|| FlowFailure::new(AbiErrorCode::MissingField, key))?;
    if !matches!(span.kind, FlowArgumentKind::Boolean) {
        return Err(FlowFailure::new(AbiErrorCode::MalformedTag, key));
    }
    match args.bytes(span)[0] {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(FlowFailure::new(AbiErrorCode::MalformedTag, key)),
    }
}

fn bytes<'a>(args: &'a FlowArguments, key: &str) -> Result<&'a [u8], FlowFailure> {
    let span = args.get(key).ok_or_else(|| FlowFailure::new(AbiErrorCode::MissingField, key))?;
    if !matches!(span.kind, FlowArgumentKind::Bytes) {
        return Err(FlowFailure::new(AbiErrorCode::MalformedTag, key));
    }
    Ok(args.bytes(span))
}

fn ok() -> Result<Vec<u8>, FlowFailure> {
    Ok(Vec::new())
}
fn domain_error(error: impl std::fmt::Display) -> FlowFailure {
    FlowFailure::new(AbiErrorCode::MalformedTag, error.to_string())
}
fn abi_failure(code: AbiErrorCode) -> FlowFailure {
    FlowFailure::new(code, code.to_string())
}

fn dag_cursor_failure(fault: dag::DagCursorFault) -> FlowFailure {
    abi_failure(match fault {
        dag::DagCursorFault::Cancelled => AbiErrorCode::Cancelled,
        dag::DagCursorFault::Interrupted => AbiErrorCode::Interrupted,
        dag::DagCursorFault::Deadline => AbiErrorCode::DeadlineExceeded,
        dag::DagCursorFault::NoFuel => AbiErrorCode::NoCredit,
        dag::DagCursorFault::Limit => AbiErrorCode::LimitExceeded,
        dag::DagCursorFault::Sealed => AbiErrorCode::Sealed,
    })
}

fn surface_abi_failure(code: SurfaceAbiErrorCode) -> FlowFailure {
    let code = match code {
        SurfaceAbiErrorCode::MalformedTag => AbiErrorCode::MalformedTag,
        SurfaceAbiErrorCode::MalformedLength => AbiErrorCode::MalformedLength,
        SurfaceAbiErrorCode::InvalidUtf8 => AbiErrorCode::InvalidUtf8,
        SurfaceAbiErrorCode::MissingField => AbiErrorCode::MissingField,
        SurfaceAbiErrorCode::LimitExceeded => AbiErrorCode::LimitExceeded,
        SurfaceAbiErrorCode::UnknownOperation => AbiErrorCode::UnknownOperation,
        SurfaceAbiErrorCode::UnknownHandle => AbiErrorCode::UnknownHandle,
        SurfaceAbiErrorCode::StaleGeneration => AbiErrorCode::StaleGeneration,
        SurfaceAbiErrorCode::AbaHandle => AbiErrorCode::AbaHandle,
        SurfaceAbiErrorCode::DuplicateAcknowledgement => AbiErrorCode::DuplicateAcknowledgement,
        SurfaceAbiErrorCode::Interrupted => AbiErrorCode::Interrupted,
        SurfaceAbiErrorCode::Cancelled => AbiErrorCode::Cancelled,
        SurfaceAbiErrorCode::Sealed => AbiErrorCode::Sealed,
        SurfaceAbiErrorCode::LateReply => AbiErrorCode::LateReply,
        SurfaceAbiErrorCode::DuplicateReply => AbiErrorCode::DuplicateReply,
        SurfaceAbiErrorCode::OutOfOrderPage => AbiErrorCode::OutOfOrderPage,
        SurfaceAbiErrorCode::DeadlineExceeded => AbiErrorCode::DeadlineExceeded,
        SurfaceAbiErrorCode::NoCredit => AbiErrorCode::NoCredit,
        SurfaceAbiErrorCode::Busy => AbiErrorCode::Busy,
        SurfaceAbiErrorCode::Closed => AbiErrorCode::Closed,
        SurfaceAbiErrorCode::GenerationExhausted => AbiErrorCode::GenerationExhausted,
    };
    abi_failure(code)
}

fn dwg_encode_mesh(mesh_json: &str) -> String {
    let Ok(mesh) = serde_json::from_str::<semio_framework::MeshData>(mesh_json) else {
        return json!({ "error": "invalid mesh json" }).to_string();
    };
    let drawing = semio_s_plugin_stdio::artifacts::dwg::mesh_to_dwg_drawing(&mesh);
    match semio_s_plugin_stdio::artifacts::dwg::dwg_to_bytes(&drawing) {
        Ok(bytes) => {
            use base64::Engine;
            json!({ "dwg": base64::engine::general_purpose::STANDARD.encode(bytes) }).to_string()
        }
        Err(error) => json!({ "error": error }).to_string(),
    }
}

//#endregion 🔖️ReactiveFeatures

//#region 🌉️LinearMemory

fn flow_bridge_clock_ready() -> bool {
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    { semio_framework_job::install_browser_monotonic_clock().is_ok() }
    #[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
    { semio_framework_job::default_now_us().is_some() }
}

struct RetainedMessage {
    _message: AbiMessage,
    bytes: Vec<u8>,
}

thread_local! {
    static BRIDGE: RefCell<FlowBridge<FlowDomainAdapter>> = RefCell::new(FlowBridge::new(FlowDomainAdapter::default));
    static RETAINED: RefCell<Option<RetainedMessage>> = const { RefCell::new(None) };
}

#[unsafe(no_mangle)]
pub extern "C" fn flow_bridge_allocate(length: usize) -> *mut u8 {
    if length == 0 || length > protocol::FLOW_MAX_REQUEST_BYTES + 32 || !flow_bridge_clock_ready() {
        return std::ptr::null_mut();
    }
    let mut bytes = Vec::<u8>::with_capacity(length);
    let pointer = bytes.as_mut_ptr();
    std::mem::forget(bytes);
    pointer
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn flow_bridge_release(pointer: *mut u8, capacity: usize) {
    if !pointer.is_null() && capacity != 0 && capacity <= protocol::FLOW_MAX_REQUEST_BYTES + 32 {
        drop(unsafe { Vec::from_raw_parts(pointer, 0, capacity) });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn flow_bridge_send(pointer: *const u8, length: usize, byte_credit: usize, now_ms: u64, deadline_ms: u64) -> i32 {
    if pointer.is_null() || length == 0 || length > protocol::FLOW_MAX_REQUEST_BYTES + 32 || !flow_bridge_clock_ready() {
        return -1;
    }
    let Ok(message) = decode_abi_message(unsafe { std::slice::from_raw_parts(pointer, length) }) else {
        return -1;
    };
    let budget = AbiWorkBudget { byte_credit, now_ms, deadline_ms: (deadline_ms != 0).then_some(deadline_ms), cancelled: false, interrupted: false };
    BRIDGE.with(|bridge| bridge.borrow_mut().try_send(message, budget).map(|_| 1).unwrap_or(-1))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn flow_bridge_poll(pointer: *mut u8, capacity: usize, byte_credit: usize, now_ms: u64, deadline_ms: u64) -> i32 {
    if pointer.is_null() || capacity == 0 || capacity > protocol::FLOW_MAX_REQUEST_BYTES + 32 || !flow_bridge_clock_ready() {
        return -1;
    }
    RETAINED.with(|retained| {
        if retained.borrow().is_none() {
            let budget = AbiWorkBudget { byte_credit, now_ms, deadline_ms: (deadline_ms != 0).then_some(deadline_ms), cancelled: false, interrupted: false };
            match BRIDGE.with(|bridge| bridge.borrow_mut().poll(budget)) {
                Ok(AbiPortPoll::Message(message)) => *retained.borrow_mut() = Some(RetainedMessage { bytes: encode_abi_message(&message), _message: message }),
                Ok(AbiPortPoll::Pending) => return 0,
                Ok(AbiPortPoll::Closed) | Err(_) => return -1,
            }
        }
        let mut retained = retained.borrow_mut();
        let value = retained.as_ref().expect("retained Flow message");
        if value.bytes.len() > capacity {
            return i32::try_from(value.bytes.len()).unwrap_or(-1);
        }
        unsafe { std::ptr::copy_nonoverlapping(value.bytes.as_ptr(), pointer, value.bytes.len()) };
        let length = value.bytes.len();
        retained.take();
        i32::try_from(length).unwrap_or(-1)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn flow_bridge_begin_close() {
    if !flow_bridge_clock_ready() { return; }
    RETAINED.with(|retained| retained.borrow_mut().take());
    BRIDGE.with(|bridge| bridge.borrow_mut().begin_close());
}

#[unsafe(no_mangle)]
pub extern "C" fn flow_bridge_terminal_is_empty() -> i32 {
    if !flow_bridge_clock_ready() { return 0; }
    i32::from(RETAINED.with(|retained| retained.borrow().is_none()) && BRIDGE.with(|bridge| bridge.borrow().terminal_is_empty()))
}

//#endregion 🌉️LinearMemory

//#region 🧪️DomainLaws

#[cfg(test)]
mod domain_laws {
    use super::*;

    fn bridge_budget() -> AbiWorkBudget {
        AbiWorkBudget { byte_credit: 4_096, now_ms: 0, deadline_ms: Some(8), cancelled: false, interrupted: false }
    }

    fn bridge_request(operation: u16, request: u64, generation: u32, bytes: Vec<u8>) -> AbiMessage {
        AbiMessage::Request(semio_framework::abi::AbiRequest {
            operation: semio_framework::abi::AbiOperation::try_new(operation).unwrap(),
            request_id: semio_framework::abi::AbiRequestId(request),
            generation,
            bytes: semio_framework::abi::AbiBytes::try_new(bytes).unwrap(),
        })
    }

    fn bridge_poll(bridge: &mut FlowBridge<FlowDomainAdapter>) -> AbiMessage {
        for _ in 0..512 {
            if let AbiPortPoll::Message(message) = bridge.poll(bridge_budget()).unwrap() {
                return message;
            }
        }
        panic!("production Flow VCS bridge did not progress")
    }

    fn acknowledge_bridge_event(bridge: &mut FlowBridge<FlowDomainAdapter>, event: &semio_framework::abi::AbiEvent) {
        bridge
            .try_send(AbiMessage::Reply(semio_framework::abi::AbiReply { request_id: event.request_id, generation: event.generation, status: semio_framework::abi::AbiStatus::OK, bytes: semio_framework::abi::AbiBytes::default() }), bridge_budget())
            .unwrap();
    }

    fn text_payload(value: &str) -> Vec<u8> {
        let mut writer = FlowPayloadWriter::default();
        writer.bytes(value.as_bytes()).unwrap();
        writer.finish()
    }

    fn surface_payload(surface: u32, generation: u32, width: u32, height: u32, dpr: f64) -> Vec<u8> {
        let mut writer = FlowPayloadWriter::default();
        writer.u32(surface);
        writer.u32(generation);
        writer.u32(width);
        writer.u32(height);
        writer.f64(dpr);
        writer.finish()
    }

    fn surface_status_payload(surface: u32, generation: u32, status: &str) -> Vec<u8> {
        let mut writer = FlowPayloadWriter::default();
        writer.u32(surface);
        writer.u32(generation);
        writer.bytes(status.as_bytes()).unwrap();
        writer.finish()
    }

    fn run(domain: &mut FlowDomainAdapter, operation: u16, payload: Vec<u8>) -> Result<Vec<u8>, FlowFailure> {
        let arguments = FlowArguments::preflight(operation, payload)?;
        let mut action = flow_action(operation, &arguments).ok_or_else(|| abi_failure(AbiErrorCode::UnknownOperation))?;
        loop {
            match action.advance(domain, &arguments, AbiWorkBudget::credits(1)) {
                FlowFeatureStep::Complete(output) => return Ok(output),
                FlowFeatureStep::Failed(failure) => return Err(failure),
                FlowFeatureStep::Yield | FlowFeatureStep::Progress { .. } | FlowFeatureStep::Checkpoint(_) | FlowFeatureStep::Preview(_) | FlowFeatureStep::SurfaceStatus(_) | FlowFeatureStep::RetainedPage(_) => {}
            }
        }
    }

    #[test]
    fn malformed_omitted_and_unknown_selection_data_remain_owned() {
        let mut domain = FlowDomainAdapter::default();
        for json in ["{", "{}", r#"{"widgets":[],"futureOptional":true}"#] {
            run(&mut domain, 2_525, text_payload(json)).unwrap();
        }
        assert_eq!(domain.host.selected_widget_ids_json(), "[]");
    }

    #[test]
    fn surface_generation_cancel_loss_and_recovery_fail_closed() {
        let mut domain = FlowDomainAdapter::default();
        run(&mut domain, 2_575, surface_payload(7, 2, 800, 600, 2.0)).unwrap();
        assert_eq!(run(&mut domain, 2_575, surface_payload(7, 2, 800, 600, 2.0)).unwrap_err().code, AbiErrorCode::Busy);
        assert_eq!(run(&mut domain, 2_576, surface_status_payload(8, 2, "created")).unwrap_err().code, AbiErrorCode::UnknownHandle);
        assert_eq!(run(&mut domain, 2_576, surface_status_payload(7, 1, "created")).unwrap_err().code, AbiErrorCode::AbaHandle);
        assert_eq!(run(&mut domain, 2_576, surface_status_payload(7, 3, "created")).unwrap_err().code, AbiErrorCode::StaleGeneration);
        run(&mut domain, 2_576, surface_status_payload(7, 2, "created")).unwrap();
        run(&mut domain, 2_576, surface_status_payload(7, 2, "device-lost")).unwrap();
        run(&mut domain, 2_576, surface_status_payload(7, 2, "recovered")).unwrap();
        run(&mut domain, 2_576, surface_status_payload(7, 2, "cancelled")).unwrap();
        assert!(domain.surface.is_none());
    }

    #[test]
    fn malformed_fixed_width_is_rejected_before_surface_mutation() {
        let mut domain = FlowDomainAdapter::default();
        let mut payload = surface_payload(u32::MAX, 1, 1, 1, 1.0);
        payload.push(0);
        let rejected = run(&mut domain, 2_575, payload).unwrap_err();
        assert_eq!(rejected.code, AbiErrorCode::MalformedLength);
        assert!(domain.surface.is_none());
    }

    #[test]
    fn every_schema_feature_has_a_distinct_action_binding() {
        let actions: Vec<u16> = (2_504..=2_610)
            .map(|operation| flow_action(operation, &FlowArguments::preflight(operation, Vec::new()).unwrap_or_else(|_| FlowArguments { payload: Vec::new(), spans: [FlowArgumentSpan::EMPTY; 8], count: 0 })).unwrap().operation())
            .collect();
        assert_eq!(actions, (2_504..=2_610).collect::<Vec<_>>());
    }

    #[test]
    fn synchronized_document_json_is_the_exact_retained_document() {
        let mut domain = FlowDomainAdapter::default();
        let mut expected = crate::artifact::FlowFixture::default();
        expected.schema = "flow.fixture.synchronized".into();
        let json = serde_json::to_string(&expected).unwrap();
        run(&mut domain, 2_610, text_payload(&json)).unwrap();
        let actual: crate::artifact::FlowFixture = serde_json::from_slice(&run(&mut domain, 2_609, Vec::new()).unwrap()).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn cancellation_prevents_the_bound_domain_action() {
        let domain = Rc::new(RefCell::new(FlowDomainAdapter::default()));
        let session = semio_framework::abi::AbiHandle::try_new(1, 1).unwrap();
        domain.borrow_mut().bind_session(session);
        let mut writer = FlowPayloadWriter::default();
        writer.u32(1);
        writer.u64(0);
        writer.u64(0);
        let admission = FlowFeatureAdmission { session, request_generation: 1 };
        let mut feature = FlowDomainAdapter::start_feature(domain, admission, 2_501, writer.finish()).unwrap();
        assert!(matches!(feature.step(AbiWorkBudget::credits(64)), FlowFeatureStep::Progress { completed: 0, total: 3 }));
        feature.cancel(AbiWorkBudget::credits(64)).unwrap();
        assert!(!feature.close_step(AbiWorkBudget::credits(64)).unwrap());
    }

    #[test]
    fn production_bridge_installs_vcs_authority_page_ack_retry_and_incremental_close() {
        let mut bridge = FlowBridge::new(FlowDomainAdapter::default);
        bridge.try_send(bridge_request(protocol::FLOW_OPERATION_OPEN, 1, 1, Vec::new()), bridge_budget()).unwrap();
        let AbiMessage::Reply(opened) = bridge_poll(&mut bridge) else { panic!("Flow session reply") };
        let mut opened_reader = FlowPayloadReader::new(opened.bytes.as_slice());
        let session = opened_reader.handle().unwrap();
        opened_reader.finish().unwrap();

        let mut payload = FlowPayloadWriter::default();
        payload.handle(session);
        payload.u32(session.generation());
        payload.u64(0);
        payload.u64(0);
        bridge.try_send(bridge_request(2_501, 2, 1, payload.finish()), bridge_budget()).unwrap();

        let mut operation = None;
        let mut page_seen = false;
        let mut events = [false; 7];
        loop {
            match bridge_poll(&mut bridge) {
                AbiMessage::Event(event) => {
                    if event.event.get() == protocol::FLOW_EVENT_ADMITTED {
                        operation = Some(FlowPayloadReader::new(event.bytes.as_slice()).handle().unwrap());
                    }
                    events[usize::from(event.event.get() - protocol::FLOW_EVENT_ADMITTED)] = true;
                    acknowledge_bridge_event(&mut bridge, &event);
                }
                AbiMessage::Page(page) => {
                    let operation = operation.expect("admitted Flow VCS operation");
                    assert_eq!(page.handle, operation);
                    assert!(!page.bytes.is_empty());
                    assert!(bridge.try_send(AbiMessage::Control(semio_framework::abi::AbiControl::Acknowledge { handle: operation, index: page.index + 1 }), bridge_budget()).is_err());
                    bridge.try_send(AbiMessage::Control(semio_framework::abi::AbiControl::Acknowledge { handle: operation, index: page.index }), bridge_budget()).unwrap();
                    page_seen = true;
                }
                AbiMessage::Reply(reply) if reply.request_id.0 == 2 => {
                    assert_eq!(reply.status, semio_framework::abi::AbiStatus::OK);
                    assert!(reply.bytes.is_empty());
                    break;
                }
                _ => {}
            }
        }
        assert!(page_seen);
        for event in [protocol::FLOW_EVENT_ADMITTED, protocol::FLOW_EVENT_PROGRESS, protocol::FLOW_EVENT_CHECKPOINT, protocol::FLOW_EVENT_PREVIEW, protocol::FLOW_EVENT_OUTPUT, protocol::FLOW_EVENT_TERMINAL] {
            assert!(events[usize::from(event - protocol::FLOW_EVENT_ADMITTED)]);
        }

        let mut fault_payload = FlowPayloadWriter::default();
        fault_payload.handle(session);
        fault_payload.u32(session.generation());
        fault_payload.u64(1);
        fault_payload.u64(0);
        bridge.try_send(bridge_request(2_502, 3, 1, fault_payload.finish()), bridge_budget()).unwrap();
        loop {
            match bridge_poll(&mut bridge) {
                AbiMessage::Event(event) => acknowledge_bridge_event(&mut bridge, &event),
                AbiMessage::Reply(reply) if reply.request_id.0 == 3 => {
                    assert_ne!(reply.status, semio_framework::abi::AbiStatus::OK);
                    break;
                }
                _ => {}
            }
        }

        let mut cancel_payload = FlowPayloadWriter::default();
        cancel_payload.handle(session);
        cancel_payload.u32(session.generation());
        cancel_payload.u64(1);
        cancel_payload.u64(0);
        bridge.try_send(bridge_request(2_501, 4, 1, cancel_payload.finish()), bridge_budget()).unwrap();
        let AbiMessage::Event(admitted) = bridge_poll(&mut bridge) else { panic!("cancel admission") };
        assert_eq!(admitted.event.get(), protocol::FLOW_EVENT_ADMITTED);
        acknowledge_bridge_event(&mut bridge, &admitted);
        let AbiMessage::Event(progress) = bridge_poll(&mut bridge) else { panic!("cancel progress") };
        assert_eq!(progress.event.get(), protocol::FLOW_EVENT_PROGRESS);
        acknowledge_bridge_event(&mut bridge, &progress);
        bridge.try_send(AbiMessage::Control(semio_framework::abi::AbiControl::Cancel { request_id: semio_framework::abi::AbiRequestId(4), generation: 1 }), bridge_budget()).unwrap();
        loop {
            match bridge_poll(&mut bridge) {
                AbiMessage::Event(event) => acknowledge_bridge_event(&mut bridge, &event),
                AbiMessage::Reply(reply) if reply.request_id.0 == 4 => {
                    assert_ne!(reply.status, semio_framework::abi::AbiStatus::OK);
                    break;
                }
                _ => {}
            }
        }
        bridge.try_send(AbiMessage::Control(semio_framework::abi::AbiControl::Close { handle: session }), bridge_budget()).unwrap();
        bridge.begin_close();
        assert!(bridge.terminal_is_empty());
    }

    #[test]
    fn production_reachability_fixture_and_hostile_source_census_reject_the_old_route() {
        let component = include_str!("🦀️component.rs");
        let production = component.split_once("//#region 🧪️DomainLaws").expect("Flow production/test boundary").0;
        let bridge_protocol = include_str!("🦀️protocol.rs");
        let protocol_production = bridge_protocol.split_once("//#region 🧪️Laws").expect("Flow protocol production/test boundary").0;
        let schema = protocol::FLOW_ABI_SCHEMA;
        let host = include_str!("📦️packages/🟨️javascript/🟨️flow-host.js");
        let browser = include_str!("📦️packages/🟨️javascript/🟨️flow-browser.js");
        let packaged_host = include_str!("../🫀️core/pkg/🟨️flow-host.js");
        let packaged_browser = include_str!("../🫀️core/pkg/🟨️flow-browser.js");
        let package_manifest = include_str!("../🫀️core/pkg/package.json");
        let package_build = include_str!("../🫀️core/📦️packages/🦀️rust/📜️script.ts");
        let production_loader = include_str!("../../📺️renderer/🧑️‍🎨️engine/🧱️elements/WasmSessionLoader/🟦️component.tsx");
        for symbol in ["vcsCheckpoint", "vcsFault", "vcsRetryCheckpoint"] {
            assert!(schema.contains(symbol));
            assert!(host.contains(symbol));
            assert!(packaged_host.contains(symbol));
        }
        for legacy in ["loadFixtureJson", "resyncFixtureJson", "fixtureJson"] {
            assert!(!schema.contains(legacy));
            assert!(!host.contains(legacy));
            assert!(!packaged_host.contains(legacy));
        }
        let callers = [
            ["FlowRetained", "Vcs::new"].concat(),
            [".begin_check", "point(self.authority)"].concat(),
            ["vcs.poll(", "handle, grant)"].concat(),
            ["vcs.take_", "page(handle)"].concat(),
            ["vcs.resume_", "page(handle, sequence)"].concat(),
            ["vcs.retry_", "page(handle, sequence)"].concat(),
            ["vcs.acknowledge_", "page(handle, sequence)"].concat(),
            ["vcs.cancel(", "handle, grant)"].concat(),
            ["vcs.fault(", "handle, grant)"].concat(),
            ["vcs.close_operation_", "step(handle, grant)"].concat(),
            ["vcs.close_retired_", "step(grant)"].concat(),
        ];
        for caller in callers {
            assert!(production.contains(&caller), "missing production Flow VCS caller {caller}");
        }
        for legacy_mapping in [["2_501 => Some(Box::new(Flow", "Action2501"].concat(), ["2_502 => Some(Box::new(Flow", "Action2502"].concat(), ["2_503 => Some(Box::new(Flow", "Action2503"].concat()] {
            assert!(!production.contains(&legacy_mapping));
        }
        assert!(production.contains("FlowBridge::new(FlowDomainAdapter::default)"));
        assert!(production.contains("matches!(operation, 2_501..=2_503)"));
        assert!(protocol_production.contains("D::start_feature(session.borrow().domain.clone(), admission, code, payload)"));
        assert!(browser.contains("createFlowFeatures(host)"));
        assert!(browser.contains("Object.keys(FlowOperation).slice(1)"));
        for export in ["flow_bridge_send", "flow_bridge_poll", "flow_bridge_begin_close", "flow_bridge_terminal_is_empty"] {
            assert!(production.contains(export));
            assert!(host.contains(export));
        }
        for control in ["encodeCancel(", "encodeClose(", "encodeAcknowledge("] {
            assert!(host.contains(control));
        }
        assert_eq!(host, packaged_host);
        assert_eq!(browser, packaged_browser);
        assert!(package_manifest.contains("\"./🟨️flow-browser.js\""));
        assert!(package_build.contains("manifest.exports"));
        assert!(package_build.contains("copyFileSync(join(BROWSER_BRIDGE_DIR, name)"));
        assert!(production_loader.contains("import(\"@semio-tech/flow-core/🟨️flow-browser.js\")"));
        let rows: Vec<&str> = protocol::FLOW_VCS_PRODUCTION_LEDGER.lines().skip(1).collect();
        assert_eq!(rows.len(), 6);
        for unit in ["begin_checkpoint", "poll", "take_page", "resume_page", "retry_page", "acknowledge_page", "cancel", "fault", "close_operation_step", "close_retired_step"] {
            assert!(protocol::FLOW_VCS_PRODUCTION_LEDGER.contains(unit));
        }
    }

    #[test]
    fn selected_widget_query_uses_census_and_multiple_cancellable_grants() {
        let mut domain = FlowDomainAdapter::default();
        let arguments = FlowArguments::preflight(2_518, Vec::new()).unwrap();
        let mut action = flow_action(2_518, &arguments).unwrap();
        let mut grants = 0usize;
        let output = loop {
            grants += 1;
            match action.advance(&mut domain, &arguments, AbiWorkBudget::credits(1)) {
                FlowFeatureStep::Complete(output) => break output,
                FlowFeatureStep::Failed(failure) => panic!("selected widget cursor failed: {failure:?}"),
                _ => {}
            }
        };
        assert!(grants > output.len() + 4);
        assert_eq!(output, domain.host.selected_widget_ids_json().into_bytes());

        let mut action = flow_action(2_518, &arguments).unwrap();
        assert!(matches!(action.advance(&mut domain, &arguments, AbiWorkBudget::credits(1)), FlowFeatureStep::Progress { .. }));
        let mut cancelled = AbiWorkBudget::credits(1);
        cancelled.cancelled = true;
        assert!(matches!(action.advance(&mut domain, &arguments, cancelled), FlowFeatureStep::Failed(FlowFailure { code: AbiErrorCode::Cancelled, .. })));
    }
}

//#endregion 🧪️DomainLaws

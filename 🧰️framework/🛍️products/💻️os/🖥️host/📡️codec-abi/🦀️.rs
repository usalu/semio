//! 📡️ Host codec ABI: operation schema, error codes, codec session, retained and direct codec services.
//#region 🧬️Schema
use semio_framework::{AbiBytes, AbiControl, AbiCursorStep, AbiError, AbiErrorCode, AbiEvent, AbiEventCode, AbiHandle, AbiHandleTable, AbiMessageBytes, AbiOperation, AbiPage, AbiPageReader, AbiRejectedPage, AbiReply, AbiRequest, AbiRequestId, AbiStatus, AbiStatusCode, AbiWorkBudget, ABI_MAX_BODY_BYTES, ABI_MAX_MESSAGE_BYTES, ABI_MAX_PAGES_PER_TRANSFER, ABI_MAX_PAGE_BYTES};

pub const OS_HOST_CODEC_SCHEMA_JSON: &str = include_str!("../🧬️schema/🔣️.json");
pub const OS_HOST_CODEC_MAX_INPUT_BYTES: usize = ABI_MAX_BODY_BYTES;
pub const OS_HOST_CODEC_MAX_OUTPUT_BYTES: usize = ABI_MAX_BODY_BYTES;
pub const OS_HOST_CODEC_MAX_KIND_COUNT: usize = 256;
pub const OS_HOST_CODEC_MAX_KIND_BYTES: usize = ABI_MAX_MESSAGE_BYTES;
pub const OS_HOST_CODEC_PROGRESS_EVENT: u16 = 1;

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsHostCodecOperation {
    MediaAcceptFilterKinds = 1539,
    NormalizeStdioFormatKind = 1540,
}

impl OsHostCodecOperation {
    pub fn from_abi(operation: AbiOperation) -> Result<Self, AbiErrorCode> {
        match operation.get() {
            1539 => Ok(Self::MediaAcceptFilterKinds),
            1540 => Ok(Self::NormalizeStdioFormatKind),
            _ => Err(AbiErrorCode::UnknownOperation),
        }
    }

    pub fn abi(self) -> AbiOperation {
        AbiOperation::try_new(self as u16).expect("schema operation codes are bounded and non-zero")
    }

    const fn reply_kind(self) -> u8 {
        match self {
            Self::MediaAcceptFilterKinds => 2,
            Self::NormalizeStdioFormatKind => 3,
        }
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsHostCodecErrorCode {
    MalformedRequest = 1,
    MissingKindArray = 4,
    UnknownKind = 5,
    InvalidUtf8 = 6,
    InputLimit = 7,
    OutputLimit = 8,
    InvalidState = 9,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OsHostCodecFailure {
    pub code: OsHostCodecErrorCode,
    pub message: Vec<u8>,
}

impl OsHostCodecFailure {
    fn fixed(code: OsHostCodecErrorCode, message: &'static str) -> Self {
        Self { code, message: message.as_bytes().to_vec() }
    }

    fn abi_code(&self) -> AbiErrorCode {
        match self.code {
            OsHostCodecErrorCode::MalformedRequest | OsHostCodecErrorCode::InvalidState => AbiErrorCode::MalformedTag,
            OsHostCodecErrorCode::MissingKindArray => AbiErrorCode::MissingField,
            OsHostCodecErrorCode::UnknownKind => AbiErrorCode::UnknownOperation,
            OsHostCodecErrorCode::InvalidUtf8 => AbiErrorCode::InvalidUtf8,
            OsHostCodecErrorCode::InputLimit | OsHostCodecErrorCode::OutputLimit => AbiErrorCode::LimitExceeded,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OsHostCodecFormat {
    pub short_id: String,
    pub extensions: Vec<String>,
}

trait OsHostFormatResolver {
    fn resolve_format(&mut self, kind: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure>;
}

#[derive(Default)]
struct RegisteredOsHostFormatResolver;

impl OsHostFormatResolver for RegisteredOsHostFormatResolver {
    fn resolve_format(&mut self, kind: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure> {
        semio_framework_os_kernel::io::format_descriptor(kind)
            .map(|row| row.map(|row| OsHostCodecFormat { short_id: row.short_id, extensions: row.extensions }))
            .map_err(|_| OsHostCodecFailure::fixed(OsHostCodecErrorCode::InvalidState, "stdio format registry unavailable"))
    }
}
//#endregion 🧬️Schema

//#region ⏳️RetainedOperation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OsHostCodecPhase {
    Input = 1,
    Decode = 2,
    Output = 3,
    AwaitingAcknowledgement = 4,
    Reply = 5,
    Closing = 6,
    Closed = 7,
    Cancelled = 8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsHostCodecStepState {
    Progress,
    InputAcknowledged,
    OutputPage,
    AwaitingAcknowledgement,
    Reply,
    Idle,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OsHostCodecStep {
    pub state: OsHostCodecStepState,
    pub event: AbiEvent,
    pub input_acknowledgement: Option<AbiControl>,
    pub page: Option<AbiPage>,
    pub reply: Option<AbiReply>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OsHostCodecCancelOutcome {
    pub page: Option<AbiPage>,
    pub admitted_byte_credits: usize,
    pub copied_bytes: usize,
}

#[derive(Clone, Copy)]
struct OsHostCodecProgress {
    phase: OsHostCodecPhase,
    completed: usize,
    total: usize,
}

struct PendingInputPage {
    page: AbiPage,
    cursor: usize,
    retire_cursor: usize,
}

struct Utf8Cursor {
    remaining: u8,
    next_minimum: u8,
    next_maximum: u8,
    invalid: bool,
}

impl Utf8Cursor {
    fn new() -> Self {
        Self { remaining: 0, next_minimum: 0x80, next_maximum: 0xbf, invalid: false }
    }

    fn feed(&mut self, byte: u8) {
        if self.invalid {
            return;
        }
        if self.remaining != 0 {
            if byte < self.next_minimum || byte > self.next_maximum {
                self.invalid = true;
                return;
            }
            self.remaining -= 1;
            self.next_minimum = 0x80;
            self.next_maximum = 0xbf;
            return;
        }
        match byte {
            0x00..=0x7f => {}
            0xc2..=0xdf => self.remaining = 1,
            0xe0 => {
                self.remaining = 2;
                self.next_minimum = 0xa0;
            }
            0xe1..=0xec | 0xee..=0xef => self.remaining = 2,
            0xed => {
                self.remaining = 2;
                self.next_maximum = 0x9f;
            }
            0xf0 => {
                self.remaining = 3;
                self.next_minimum = 0x90;
            }
            0xf1..=0xf3 => self.remaining = 3,
            0xf4 => {
                self.remaining = 3;
                self.next_maximum = 0x8f;
            }
            _ => self.invalid = true,
        }
    }

    fn complete(&self) -> bool {
        !self.invalid && self.remaining == 0
    }
}

struct FilterKindsStructuralCursor {
    header: [u8; 3],
    header_cursor: usize,
    count: usize,
    index: usize,
    length: [u8; 2],
    length_cursor: usize,
    expected_kind_bytes: usize,
    kind: [u8; OS_HOST_CODEC_MAX_KIND_BYTES],
    kind_cursor: usize,
    utf8: Utf8Cursor,
    output: Option<Vec<u8>>,
    complete: bool,
    failure: Option<OsHostCodecFailure>,
}

impl FilterKindsStructuralCursor {
    fn new() -> Self {
        Self {
            header: [0; 3],
            header_cursor: 0,
            count: 0,
            index: 0,
            length: [0; 2],
            length_cursor: 0,
            expected_kind_bytes: 0,
            kind: [0; OS_HOST_CODEC_MAX_KIND_BYTES],
            kind_cursor: 0,
            utf8: Utf8Cursor::new(),
            output: Some(Vec::new()),
            complete: false,
            failure: None,
        }
    }

    fn fail(&mut self, code: OsHostCodecErrorCode, message: &'static str) {
        if self.failure.is_none() {
            self.failure = Some(OsHostCodecFailure::fixed(code, message));
        }
    }

    fn feed<R: OsHostFormatResolver>(&mut self, byte: u8, resolver: &mut R) {
        if self.failure.is_some() {
            return;
        }
        if self.complete {
            self.fail(OsHostCodecErrorCode::MalformedRequest, "trailing stdio format kind array bytes");
            return;
        }
        if self.header_cursor < self.header.len() {
            self.header[self.header_cursor] = byte;
            self.header_cursor += 1;
            if self.header_cursor == self.header.len() {
                self.finish_header();
            }
            return;
        }
        if self.length_cursor < self.length.len() {
            self.length[self.length_cursor] = byte;
            self.length_cursor += 1;
            if self.length_cursor == self.length.len() {
                self.expected_kind_bytes = u16::from_le_bytes(self.length) as usize;
                if self.expected_kind_bytes > OS_HOST_CODEC_MAX_KIND_BYTES {
                    self.fail(OsHostCodecErrorCode::InputLimit, "stdio format kind exceeds input limit");
                } else if self.expected_kind_bytes == 0 {
                    self.finish_kind(resolver);
                }
            }
            return;
        }
        if self.kind_cursor >= self.expected_kind_bytes || self.kind_cursor >= self.kind.len() {
            self.fail(OsHostCodecErrorCode::MalformedRequest, "malformed stdio format kind array");
            return;
        }
        self.kind[self.kind_cursor] = byte;
        self.kind_cursor += 1;
        self.utf8.feed(byte);
        if self.utf8.invalid {
            self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
        } else if self.kind_cursor == self.expected_kind_bytes {
            if self.utf8.complete() {
                self.finish_kind(resolver);
            } else {
                self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
            }
        }
    }

    fn finish_header(&mut self) {
        if self.header[0] != 1 {
            self.fail(OsHostCodecErrorCode::MalformedRequest, "invalid stdio format kind array version");
            return;
        }
        self.count = u16::from_le_bytes([self.header[1], self.header[2]]) as usize;
        if self.count > OS_HOST_CODEC_MAX_KIND_COUNT {
            self.fail(OsHostCodecErrorCode::InputLimit, "stdio format kind array exceeds item limit");
        } else if self.count == 0 {
            self.complete = true;
        }
    }

    fn finish_kind<R: OsHostFormatResolver>(&mut self, resolver: &mut R) {
        let kind = match std::str::from_utf8(&self.kind[..self.kind_cursor]) {
            Ok(kind) => kind,
            Err(_) => {
                self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
                return;
            }
        };
        let row = match resolver.resolve_format(kind) {
            Ok(Some(row)) => row,
            Ok(None) => {
                self.fail(OsHostCodecErrorCode::UnknownKind, "unknown stdio format kind");
                return;
            }
            Err(failure) => {
                self.failure = Some(failure);
                return;
            }
        };
        for extension in row.extensions {
            let output = self.output.as_mut().expect("filter output is retained until seal");
            let separator = usize::from(!output.is_empty());
            let Some(output_bytes) = output.len().checked_add(separator).and_then(|bytes| bytes.checked_add(extension.len())).and_then(|bytes| bytes.checked_add(6)) else {
                self.fail(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit");
                return;
            };
            if output_bytes > OS_HOST_CODEC_MAX_OUTPUT_BYTES {
                self.fail(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit");
                return;
            }
            if separator != 0 {
                output.push(b',');
            }
            output.extend_from_slice(extension.as_bytes());
        }
        self.index += 1;
        self.length = [0; 2];
        self.length_cursor = 0;
        self.expected_kind_bytes = 0;
        self.kind[..self.kind_cursor].fill(0);
        self.kind_cursor = 0;
        self.utf8 = Utf8Cursor::new();
        self.complete = self.index == self.count;
    }

    fn finish(&mut self) -> Result<Vec<u8>, OsHostCodecFailure> {
        if self.header_cursor == 0 {
            self.fail(OsHostCodecErrorCode::MissingKindArray, "missing stdio format kind array");
        } else if self.header_cursor != self.header.len() || !self.complete || self.length_cursor != 0 || self.kind_cursor != 0 {
            self.fail(OsHostCodecErrorCode::MalformedRequest, "truncated stdio format kind array");
        }
        if let Some(failure) = self.failure.take() {
            return Err(failure);
        }
        self.output.take().ok_or_else(|| OsHostCodecFailure::fixed(OsHostCodecErrorCode::InvalidState, "stdio format filter output is unavailable"))
    }

    fn close_one(&mut self) -> bool {
        if self.output.as_mut().is_some_and(|output| output.pop().is_some()) {
            return false;
        }
        if self.kind_cursor != 0 {
            self.kind_cursor -= 1;
            self.kind[self.kind_cursor] = 0;
            return false;
        }
        if self.length_cursor != 0 {
            self.length_cursor -= 1;
            self.length[self.length_cursor] = 0;
            return false;
        }
        if self.header_cursor != 0 {
            self.header_cursor -= 1;
            self.header[self.header_cursor] = 0;
            return false;
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.output.as_ref().is_none_or(Vec::is_empty) && self.kind_cursor == 0 && self.length_cursor == 0 && self.header_cursor == 0
    }
}

struct NormalizeKindStructuralCursor {
    declared_input_bytes: usize,
    kind: [u8; OS_HOST_CODEC_MAX_KIND_BYTES],
    kind_cursor: usize,
    utf8: Utf8Cursor,
    output: Option<Vec<u8>>,
    failure: Option<OsHostCodecFailure>,
}

impl NormalizeKindStructuralCursor {
    fn new(declared_input_bytes: usize) -> Self {
        let failure = (declared_input_bytes == 0).then(|| OsHostCodecFailure::fixed(OsHostCodecErrorCode::MalformedRequest, "missing stdio format kind"));
        Self { declared_input_bytes, kind: [0; OS_HOST_CODEC_MAX_KIND_BYTES], kind_cursor: 0, utf8: Utf8Cursor::new(), output: None, failure }
    }

    fn fail(&mut self, code: OsHostCodecErrorCode, message: &'static str) {
        if self.failure.is_none() {
            self.failure = Some(OsHostCodecFailure::fixed(code, message));
        }
    }

    fn feed<R: OsHostFormatResolver>(&mut self, byte: u8, resolver: &mut R) {
        if self.failure.is_some() {
            return;
        }
        if self.kind_cursor >= self.declared_input_bytes || self.kind_cursor >= self.kind.len() {
            self.fail(OsHostCodecErrorCode::InputLimit, "stdio format kind exceeds input limit");
            return;
        }
        self.kind[self.kind_cursor] = byte;
        self.kind_cursor += 1;
        self.utf8.feed(byte);
        if self.utf8.invalid {
            self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
        } else if self.kind_cursor == self.declared_input_bytes {
            if self.utf8.complete() {
                self.resolve(resolver);
            } else {
                self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
            }
        }
    }

    fn resolve<R: OsHostFormatResolver>(&mut self, resolver: &mut R) {
        let kind = match std::str::from_utf8(&self.kind[..self.kind_cursor]) {
            Ok(kind) => kind,
            Err(_) => {
                self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
                return;
            }
        };
        match resolver.resolve_format(kind) {
            Ok(Some(row)) if row.short_id.len().checked_add(6).is_some_and(|bytes| bytes <= OS_HOST_CODEC_MAX_OUTPUT_BYTES) => self.output = Some(row.short_id.into_bytes()),
            Ok(Some(_)) => self.fail(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit"),
            Ok(None) => self.fail(OsHostCodecErrorCode::UnknownKind, "unknown stdio format kind"),
            Err(failure) => self.failure = Some(failure),
        }
    }

    fn finish(&mut self) -> Result<Vec<u8>, OsHostCodecFailure> {
        if self.kind_cursor != self.declared_input_bytes {
            self.fail(OsHostCodecErrorCode::MalformedRequest, "truncated stdio format kind");
        } else if !self.utf8.complete() {
            self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
        }
        if let Some(failure) = self.failure.take() {
            return Err(failure);
        }
        self.output.take().ok_or_else(|| OsHostCodecFailure::fixed(OsHostCodecErrorCode::InvalidState, "normalized stdio format kind output is unavailable"))
    }

    fn close_one(&mut self) -> bool {
        if self.output.as_mut().is_some_and(|output| output.pop().is_some()) {
            return false;
        }
        if self.kind_cursor != 0 {
            self.kind_cursor -= 1;
            self.kind[self.kind_cursor] = 0;
            return false;
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.output.as_ref().is_none_or(Vec::is_empty) && self.kind_cursor == 0
    }
}

enum OsHostCodecInput {
    Filter(FilterKindsStructuralCursor),
    Normalize(NormalizeKindStructuralCursor),
}

impl OsHostCodecInput {
    fn new(operation: OsHostCodecOperation, declared_input_bytes: usize) -> Self {
        match operation {
            OsHostCodecOperation::MediaAcceptFilterKinds => Self::Filter(FilterKindsStructuralCursor::new()),
            OsHostCodecOperation::NormalizeStdioFormatKind => Self::Normalize(NormalizeKindStructuralCursor::new(declared_input_bytes)),
        }
    }

    fn feed<R: OsHostFormatResolver>(&mut self, byte: u8, resolver: &mut R) {
        match self {
            Self::Filter(cursor) => cursor.feed(byte, resolver),
            Self::Normalize(cursor) => cursor.feed(byte, resolver),
        }
    }

    fn filter(&mut self) -> Option<&mut FilterKindsStructuralCursor> {
        match self {
            Self::Filter(cursor) => Some(cursor),
            _ => None,
        }
    }

    fn normalize(&mut self) -> Option<&mut NormalizeKindStructuralCursor> {
        match self {
            Self::Normalize(cursor) => Some(cursor),
            _ => None,
        }
    }

    fn retire_one(&mut self) -> bool {
        match self {
            Self::Filter(cursor) => !cursor.close_one(),
            Self::Normalize(cursor) => !cursor.close_one(),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Filter(cursor) => cursor.terminal_is_empty(),
            Self::Normalize(cursor) => cursor.terminal_is_empty(),
        }
    }
}

struct OsHostCodecSession {
    request: AbiRequest,
    operation: OsHostCodecOperation,
    handle: Option<AbiHandle>,
    declared_input_bytes: usize,
    input_bytes_received: usize,
    input: OsHostCodecInput,
    pending_input: Option<PendingInputPage>,
    next_input_index: u32,
    sealed: bool,
    decoded: bool,
    output: Option<AbiPageReader>,
    output_bytes: usize,
    output_copied: usize,
    reply_emitted: bool,
    terminal_failure: Option<OsHostCodecFailure>,
    sequence: u32,
    cancelled: bool,
    closing: bool,
}

impl OsHostCodecSession {
    fn new(request: AbiRequest, operation: OsHostCodecOperation, declared_input_bytes: usize) -> Self {
        Self {
            request,
            operation,
            handle: None,
            declared_input_bytes,
            input_bytes_received: 0,
            input: OsHostCodecInput::new(operation, declared_input_bytes),
            pending_input: None,
            next_input_index: 0,
            sealed: false,
            decoded: false,
            output: None,
            output_bytes: 0,
            output_copied: 0,
            reply_emitted: false,
            terminal_failure: None,
            sequence: 0,
            cancelled: false,
            closing: false,
        }
    }

    fn handle(&self) -> AbiHandle {
        self.handle.expect("service assigns a handle before exposing a session")
    }

    fn event(&mut self, phase: OsHostCodecPhase, completed: usize, total: usize) -> AbiEvent {
        self.sequence = self.sequence.saturating_add(1);
        let mut body = Vec::with_capacity(18);
        body.push(1);
        body.push(phase as u8);
        body.extend_from_slice(&(completed as u64).to_le_bytes());
        body.extend_from_slice(&(total as u64).to_le_bytes());
        AbiEvent {
            request_id: self.request.request_id,
            generation: self.request.generation,
            sequence: self.sequence,
            event: AbiEventCode::try_new(OS_HOST_CODEC_PROGRESS_EVENT).expect("schema event code is bounded"),
            status: AbiStatus::OK,
            bytes: AbiBytes::try_new(body).expect("fixed progress body is bounded"),
        }
    }

    fn step_result(&mut self, state: OsHostCodecStepState, progress: OsHostCodecProgress, input_acknowledgement: Option<AbiControl>, page: Option<AbiPage>, reply: Option<AbiReply>) -> OsHostCodecStep {
        OsHostCodecStep { state, event: self.event(progress.phase, progress.completed, progress.total), input_acknowledgement, page, reply }
    }

    fn offer(&mut self, page: AbiPage) -> Result<(), AbiRejectedPage> {
        let reject = |code, page: AbiPage| AbiRejectedPage { code, handle: page.handle, index: page.index, bytes: page.bytes.into_vec() };
        if page.handle != self.handle() {
            let code = classify_handle(self.handle(), page.handle);
            return Err(reject(code, page));
        }
        if self.cancelled {
            return Err(reject(AbiErrorCode::Cancelled, page));
        }
        if self.closing {
            return Err(reject(AbiErrorCode::Closed, page));
        }
        if self.sealed {
            return Err(reject(AbiErrorCode::Sealed, page));
        }
        if self.pending_input.is_some() {
            return Err(reject(AbiErrorCode::Busy, page));
        }
        let total = self.input_bytes_received.checked_add(page.bytes.len());
        if page.index >= ABI_MAX_PAGES_PER_TRANSFER || page.bytes.len() > ABI_MAX_PAGE_BYTES || total.is_none_or(|total| total > self.declared_input_bytes || total > OS_HOST_CODEC_MAX_INPUT_BYTES) {
            return Err(reject(AbiErrorCode::LimitExceeded, page));
        }
        if page.index != self.next_input_index {
            return Err(reject(AbiErrorCode::OutOfOrderPage, page));
        }
        self.pending_input = Some(PendingInputPage { page, cursor: 0, retire_cursor: 0 });
        Ok(())
    }

    fn seal(&mut self) -> Result<(), AbiErrorCode> {
        if self.cancelled {
            return Err(AbiErrorCode::Cancelled);
        }
        if self.pending_input.is_some() {
            return Err(AbiErrorCode::Busy);
        }
        if self.input_bytes_received != self.declared_input_bytes {
            return Err(AbiErrorCode::MalformedLength);
        }
        self.sealed = true;
        Ok(())
    }

    fn cancel(&mut self) -> OsHostCodecCancelOutcome {
        self.cancelled = true;
        if let Some(output) = self.output.as_mut() {
            output.cancel();
        }
        let pending = self.pending_input.take();
        OsHostCodecCancelOutcome { admitted_byte_credits: pending.as_ref().map_or(0, |pending| pending.page.bytes.len()), copied_bytes: pending.as_ref().map_or(0, |pending| pending.cursor), page: pending.map(|pending| pending.page) }
    }

    fn failure_reply(&self, failure: &OsHostCodecFailure) -> AbiReply {
        let message = AbiMessageBytes::try_new(failure.message.clone()).unwrap_or_else(|_| AbiMessageBytes::from_text("bounded OS host codec failure").expect("fixed fallback is bounded"));
        AbiReply {
            request_id: self.request.request_id,
            generation: self.request.generation,
            status: AbiStatus { code: AbiStatusCode::Rejected, error: Some(AbiError { code: failure.abi_code(), message }) },
            bytes: AbiBytes::try_new((failure.code as u16).to_le_bytes().to_vec()).expect("error code body is bounded"),
        }
    }

    fn success_reply(&self) -> AbiReply {
        let mut summary = Vec::with_capacity(6);
        summary.push(1);
        summary.push(self.operation.reply_kind());
        summary.extend_from_slice(&(self.output_bytes as u32).to_le_bytes());
        AbiReply { request_id: self.request.request_id, generation: self.request.generation, status: AbiStatus::OK, bytes: AbiBytes::try_new(summary).expect("reply summary is bounded") }
    }

    fn install_output(&mut self, bytes: &[u8]) -> Result<(), OsHostCodecFailure> {
        let payload_len = bytes.len().checked_add(6).ok_or_else(|| OsHostCodecFailure::fixed(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit"))?;
        if payload_len > OS_HOST_CODEC_MAX_OUTPUT_BYTES {
            return Err(OsHostCodecFailure::fixed(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit"));
        }
        let mut payload = Vec::with_capacity(payload_len);
        payload.push(1);
        payload.push(self.operation.reply_kind());
        payload.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(bytes);
        self.install_payload(payload)
    }

    fn install_payload(&mut self, payload: Vec<u8>) -> Result<(), OsHostCodecFailure> {
        if payload.len() > OS_HOST_CODEC_MAX_OUTPUT_BYTES {
            return Err(OsHostCodecFailure::fixed(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit"));
        }
        self.output_bytes = payload.len();
        self.output = Some(AbiPageReader::try_new(self.handle(), payload).map_err(|_| OsHostCodecFailure::fixed(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds transfer limit"))?);
        self.decoded = true;
        Ok(())
    }

    fn execute(&mut self) -> Result<(), OsHostCodecFailure> {
        match self.operation {
            OsHostCodecOperation::NormalizeStdioFormatKind => {
                let output = self.input.normalize().expect("normalize operation owns a structural cursor").finish()?;
                self.install_output(&output)
            }
            OsHostCodecOperation::MediaAcceptFilterKinds => {
                let output = self.input.filter().expect("filter operation owns a structural cursor").finish()?;
                self.install_output(&output)
            }
        }
    }

    fn step<R: OsHostFormatResolver>(&mut self, resolver: &mut R, budget: AbiWorkBudget) -> Result<OsHostCodecStep, AbiErrorCode> {
        if self.cancelled {
            return Err(AbiErrorCode::Cancelled);
        }
        if self.closing {
            return Err(AbiErrorCode::Closed);
        }
        validate_budget(budget)?;
        if self.pending_input.is_some() {
            let (byte, index, complete) = {
                let pending = self.pending_input.as_mut().expect("checked pending input");
                let byte = if pending.cursor < pending.page.bytes.len() {
                    let byte = pending.page.bytes.as_slice()[pending.cursor];
                    pending.cursor += 1;
                    Some(byte)
                } else {
                    None
                };
                (byte, pending.page.index, pending.cursor == pending.page.bytes.len())
            };
            if let Some(byte) = byte {
                self.input_bytes_received += 1;
                self.input.feed(byte, resolver);
            }
            if complete {
                self.pending_input = None;
                self.next_input_index += 1;
                let acknowledgement = AbiControl::Acknowledge { handle: self.handle(), index };
                return Ok(self.step_result(OsHostCodecStepState::InputAcknowledged, OsHostCodecProgress { phase: OsHostCodecPhase::Input, completed: self.input_bytes_received, total: self.declared_input_bytes }, Some(acknowledgement), None, None));
            }
            return Ok(self.step_result(OsHostCodecStepState::Progress, OsHostCodecProgress { phase: OsHostCodecPhase::Decode, completed: self.input_bytes_received, total: self.declared_input_bytes }, None, None, None));
        }
        if !self.sealed {
            return Ok(self.step_result(OsHostCodecStepState::Idle, OsHostCodecProgress { phase: OsHostCodecPhase::Input, completed: self.input_bytes_received, total: self.declared_input_bytes }, None, None, None));
        }
        if let Some(failure) = self.terminal_failure.clone() {
            if self.reply_emitted {
                return Ok(self.step_result(OsHostCodecStepState::Idle, OsHostCodecProgress { phase: OsHostCodecPhase::Reply, completed: 1, total: 1 }, None, None, None));
            }
            self.reply_emitted = true;
            let reply = self.failure_reply(&failure);
            return Ok(self.step_result(OsHostCodecStepState::Reply, OsHostCodecProgress { phase: OsHostCodecPhase::Reply, completed: 1, total: 1 }, None, None, Some(reply)));
        }
        if !self.decoded {
            match self.execute() {
                Ok(()) => return Ok(self.step_result(OsHostCodecStepState::Progress, OsHostCodecProgress { phase: OsHostCodecPhase::Decode, completed: 1, total: 1 }, None, None, None)),
                Err(failure) => {
                    self.terminal_failure = Some(failure.clone());
                    self.reply_emitted = true;
                    let reply = self.failure_reply(&failure);
                    return Ok(self.step_result(OsHostCodecStepState::Reply, OsHostCodecProgress { phase: OsHostCodecPhase::Reply, completed: 1, total: 1 }, None, None, Some(reply)));
                }
            }
        }
        let output_step = self.output.as_mut().expect("decoded operation owns a paged output").read_step(AbiWorkBudget { byte_credit: 1, ..budget })?;
        match output_step {
            AbiCursorStep::Advanced(copied) => {
                self.output_copied = self.output_copied.saturating_add(copied).min(self.output_bytes);
                Ok(self.step_result(OsHostCodecStepState::Progress, OsHostCodecProgress { phase: OsHostCodecPhase::Output, completed: self.output_copied, total: self.output_bytes }, None, None, None))
            }
            AbiCursorStep::PageComplete(index) => {
                let page = self.output.as_ref().and_then(AbiPageReader::page).cloned().expect("page-complete retains the exact page until ACK");
                self.output_copied = (index as usize * ABI_MAX_PAGE_BYTES + page.bytes.len()).min(self.output_bytes);
                Ok(self.step_result(OsHostCodecStepState::OutputPage, OsHostCodecProgress { phase: OsHostCodecPhase::Output, completed: self.output_copied, total: self.output_bytes }, None, Some(page), None))
            }
            AbiCursorStep::AwaitingAcknowledgement(_) => Ok(self.step_result(OsHostCodecStepState::AwaitingAcknowledgement, OsHostCodecProgress { phase: OsHostCodecPhase::AwaitingAcknowledgement, completed: self.output_copied, total: self.output_bytes }, None, None, None)),
            AbiCursorStep::Complete => {
                if self.reply_emitted {
                    Ok(self.step_result(OsHostCodecStepState::Idle, OsHostCodecProgress { phase: OsHostCodecPhase::Reply, completed: self.output_bytes, total: self.output_bytes }, None, None, None))
                } else {
                    self.reply_emitted = true;
                    let reply = self.success_reply();
                    Ok(self.step_result(OsHostCodecStepState::Reply, OsHostCodecProgress { phase: OsHostCodecPhase::Reply, completed: self.output_bytes, total: self.output_bytes }, None, None, Some(reply)))
                }
            }
            AbiCursorStep::Idle => Ok(self.step_result(OsHostCodecStepState::Idle, OsHostCodecProgress { phase: OsHostCodecPhase::Output, completed: 0, total: self.output_bytes }, None, None, None)),
        }
    }

    fn acknowledge(&mut self, control: AbiControl) -> Result<(), AbiErrorCode> {
        self.output.as_mut().ok_or(AbiErrorCode::UnknownHandle)?.acknowledge(control)
    }

    fn close_step(&mut self, budget: AbiWorkBudget) -> Result<bool, AbiErrorCode> {
        self.closing = true;
        let input_empty = self.input.terminal_is_empty();
        let failure_empty = self.terminal_failure.as_ref().is_none_or(|failure| failure.message.is_empty());
        if self.pending_input.is_none() && input_empty && failure_empty && self.output.as_ref().is_none_or(AbiPageReader::terminal_is_empty) {
            return Ok(true);
        }
        validate_budget(budget)?;
        if let Some(pending) = self.pending_input.as_mut() {
            if pending.page.bytes.is_empty() {
                self.pending_input = None;
                return Ok(false);
            }
            pending.retire_cursor += 1;
            if pending.retire_cursor == pending.page.bytes.len() {
                self.pending_input = None;
            }
            return Ok(false);
        }
        if self.input.retire_one() {
            return Ok(false);
        }
        if let Some(failure) = self.terminal_failure.as_mut() {
            if failure.message.pop().is_some() {
                return Ok(false);
            }
        }
        if let Some(output) = self.output.as_mut() {
            return output.close_step(AbiWorkBudget { byte_credit: 1, ..budget }).map(|step| step == AbiCursorStep::Complete);
        }
        Ok(true)
    }
}

fn validate_budget(budget: AbiWorkBudget) -> Result<(), AbiErrorCode> {
    if budget.cancelled {
        Err(AbiErrorCode::Cancelled)
    } else if budget.interrupted {
        Err(AbiErrorCode::Interrupted)
    } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
        Err(AbiErrorCode::DeadlineExceeded)
    } else if budget.byte_credit == 0 {
        Err(AbiErrorCode::NoCredit)
    } else {
        Ok(())
    }
}

fn classify_handle(expected: AbiHandle, actual: AbiHandle) -> AbiErrorCode {
    if expected.slot() != actual.slot() {
        AbiErrorCode::UnknownHandle
    } else if actual.generation() < expected.generation() {
        AbiErrorCode::AbaHandle
    } else if actual.generation() > expected.generation() {
        AbiErrorCode::StaleGeneration
    } else {
        AbiErrorCode::UnknownHandle
    }
}

fn request_slot(request_id: AbiRequestId) -> usize {
    (request_id.0 % 256) as usize
}

struct RetainedOsHostCodecService<R> {
    resolver: R,
    handles: AbiHandleTable<OsHostCodecSession>,
    requests: [Option<AbiHandle>; 256],
}

impl<R: OsHostFormatResolver> RetainedOsHostCodecService<R> {
    pub fn new(resolver: R) -> Self {
        Self { resolver, handles: AbiHandleTable::new(), requests: [None; 256] }
    }

    pub fn begin(&mut self, request: AbiRequest) -> Result<AbiHandle, (AbiErrorCode, AbiRequest)> {
        let operation = match OsHostCodecOperation::from_abi(request.operation) {
            Ok(operation) => operation,
            Err(code) => return Err((code, request)),
        };
        if request.generation == 0 {
            return Err((AbiErrorCode::StaleGeneration, request));
        }
        let declared_input_bytes = match decode_request_metadata(request.bytes.as_slice()) {
            Ok(value) => value,
            Err(code) => return Err((code, request)),
        };
        if operation == OsHostCodecOperation::NormalizeStdioFormatKind && declared_input_bytes > OS_HOST_CODEC_MAX_KIND_BYTES {
            return Err((AbiErrorCode::LimitExceeded, request));
        }
        let slot = request_slot(request.request_id);
        if self.requests[slot].is_some() {
            return Err((AbiErrorCode::Busy, request));
        }
        let session = OsHostCodecSession::new(request, operation, declared_input_bytes);
        let handle = match self.handles.open(session) {
            Ok(handle) => handle,
            Err((code, session)) => return Err((code, session.request)),
        };
        self.handles.get_mut(handle).expect("new handle resolves").handle = Some(handle);
        self.requests[slot] = Some(handle);
        Ok(handle)
    }

    pub fn offer(&mut self, handle: AbiHandle, page: AbiPage) -> Result<(), AbiRejectedPage> {
        match self.handles.get_mut(handle) {
            Ok(session) => session.offer(page),
            Err(code) => Err(AbiRejectedPage { code, handle: page.handle, index: page.index, bytes: page.bytes.into_vec() }),
        }
    }

    pub fn seal(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
        self.handles.get_mut(handle)?.seal()
    }

    pub fn step(&mut self, handle: AbiHandle, budget: AbiWorkBudget) -> Result<OsHostCodecStep, AbiErrorCode> {
        self.handles.get_mut(handle)?.step(&mut self.resolver, budget)
    }

    pub fn control(&mut self, control: AbiControl) -> Result<Option<OsHostCodecCancelOutcome>, AbiErrorCode> {
        match control {
            AbiControl::Cancel { request_id, generation } => {
                let handle = self.requests[request_slot(request_id)].ok_or(AbiErrorCode::UnknownHandle)?;
                let session = self.handles.get_mut(handle)?;
                if session.request.request_id != request_id {
                    return Err(AbiErrorCode::UnknownHandle);
                }
                if generation < session.request.generation {
                    return Err(AbiErrorCode::AbaHandle);
                }
                if generation > session.request.generation {
                    return Err(AbiErrorCode::StaleGeneration);
                }
                Ok(Some(session.cancel()))
            }
            AbiControl::Acknowledge { handle, .. } => {
                self.handles.get_mut(handle)?.acknowledge(control)?;
                Ok(None)
            }
            AbiControl::Close { .. } => Err(AbiErrorCode::MalformedTag),
        }
    }

    pub fn close_step(&mut self, control: AbiControl, budget: AbiWorkBudget) -> Result<bool, AbiErrorCode> {
        let AbiControl::Close { handle } = control else { return Err(AbiErrorCode::MalformedTag) };
        let complete = self.handles.get_mut(handle)?.close_step(budget)?;
        if complete {
            let session = self.handles.close(handle)?;
            let slot = request_slot(session.request.request_id);
            if self.requests[slot] == Some(handle) {
                self.requests[slot] = None;
            }
        }
        Ok(complete)
    }

    pub fn lose(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
        let session = self.handles.lose(handle)?;
        let slot = request_slot(session.request.request_id);
        if self.requests[slot] == Some(handle) {
            self.requests[slot] = None;
        }
        Ok(())
    }
}

/// 🌉️ Owned retained OS-host codec service; UI callers can submit only A1 requests and pages.
pub struct OsHostCodecService {
    retained: RetainedOsHostCodecService<RegisteredOsHostFormatResolver>,
}

impl Default for OsHostCodecService {
    fn default() -> Self {
        Self::new()
    }
}

impl OsHostCodecService {
    pub fn new() -> Self {
        Self { retained: RetainedOsHostCodecService::new(RegisteredOsHostFormatResolver) }
    }

    pub fn begin(&mut self, request: AbiRequest) -> Result<AbiHandle, (AbiErrorCode, AbiRequest)> {
        self.retained.begin(request)
    }

    pub fn offer(&mut self, handle: AbiHandle, page: AbiPage) -> Result<(), AbiRejectedPage> {
        self.retained.offer(handle, page)
    }

    pub fn seal(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
        self.retained.seal(handle)
    }

    pub fn step(&mut self, handle: AbiHandle, budget: AbiWorkBudget) -> Result<OsHostCodecStep, AbiErrorCode> {
        self.retained.step(handle, budget)
    }

    pub fn control(&mut self, control: AbiControl) -> Result<Option<OsHostCodecCancelOutcome>, AbiErrorCode> {
        self.retained.control(control)
    }

    pub fn close_step(&mut self, control: AbiControl, budget: AbiWorkBudget) -> Result<bool, AbiErrorCode> {
        self.retained.close_step(control, budget)
    }

    pub fn lose(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
        self.retained.lose(handle)
    }
}

fn decode_request_metadata(bytes: &[u8]) -> Result<usize, AbiErrorCode> {
    if bytes.len() != 5 || bytes[0] != 1 {
        return Err(AbiErrorCode::MalformedTag);
    }
    let declared = u32::from_le_bytes(bytes[1..5].try_into().expect("fixed metadata width")) as usize;
    if declared > OS_HOST_CODEC_MAX_INPUT_BYTES {
        Err(AbiErrorCode::LimitExceeded)
    } else {
        Ok(declared)
    }
}
//#endregion ⏳️RetainedOperation

//#region 🧪️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️codec-abi-unit/🦀️.rs");
//#endregion 🧪️Tests

//! 🌉️ Owned, domain-neutral byte/message ABI and retained paged-transfer kernel.

use std::fmt::{Display, Formatter};

//#region 🧬️Schema

pub const ABI_VERSION: u8 = 1;
pub const ABI_MAX_OPERATION_CODE: u16 = 4_095;
pub const ABI_MAX_EVENT_CODE: u16 = 4_095;
pub const ABI_MAX_BODY_BYTES: usize = 1_048_576;
pub const ABI_MAX_PAGE_BYTES: usize = 65_536;
pub const ABI_MAX_MESSAGE_BYTES: usize = 1_024;
pub const ABI_MAX_PAGES_PER_TRANSFER: u32 = 256;
pub const ABI_MAX_TRANSFER_BYTES: usize = ABI_MAX_PAGE_BYTES * ABI_MAX_PAGES_PER_TRANSFER as usize;
pub const ABI_MAX_IN_FLIGHT_HANDLES: usize = 64;
pub const ABI_MAX_IN_FLIGHT_REQUESTS: usize = 256;
pub const ABI_SCHEMA_JSON: &str = include_str!("🧬️schema/🔣️.json");
pub const ABI_LEDGER_FIXTURE: &str = include_str!("🧪️fixtures/📊️.tsv");
pub const ABI_LIMITS_FIXTURE: &str = include_str!("🧪️fixtures/📐️limits.tsv");

/// 🔢 Valid, non-zero domain operation code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AbiOperation(u16);

impl AbiOperation {
    pub fn try_new(code: u16) -> Result<Self, AbiErrorCode> {
        if code == 0 || code > ABI_MAX_OPERATION_CODE {
            Err(AbiErrorCode::UnknownOperation)
        } else {
            Ok(Self(code))
        }
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}

/// 📣 Valid, non-zero domain event code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AbiEventCode(u16);

impl AbiEventCode {
    pub fn try_new(code: u16) -> Result<Self, AbiErrorCode> {
        if code == 0 || code > ABI_MAX_EVENT_CODE {
            Err(AbiErrorCode::MalformedTag)
        } else {
            Ok(Self(code))
        }
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}

/// 🪪 Correlates one request/reply family without borrowing host objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AbiRequestId(pub u64);

/// ♻️ Opaque slot plus generation; equality is the exact handback identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AbiHandle {
    slot: u32,
    generation: u32,
}

impl AbiHandle {
    pub fn try_new(slot: u32, generation: u32) -> Result<Self, AbiErrorCode> {
        if slot == 0 {
            Err(AbiErrorCode::UnknownHandle)
        } else if generation == 0 {
            Err(AbiErrorCode::StaleGeneration)
        } else {
            Ok(Self { slot, generation })
        }
    }

    pub const fn slot(self) -> u32 {
        self.slot
    }

    pub const fn generation(self) -> u32 {
        self.generation
    }
}

/// 📦 Owned bounded request/reply/event body bytes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AbiBytes(Vec<u8>);

impl AbiBytes {
    pub fn try_new(bytes: Vec<u8>) -> Result<Self, AbiRejectedBytes> {
        if bytes.len() > ABI_MAX_BODY_BYTES {
            Err(AbiRejectedBytes { code: AbiErrorCode::LimitExceeded, bytes })
        } else {
            Ok(Self(bytes))
        }
    }

    pub const fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// 📄 Owned bytes for exactly one bounded transfer page.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AbiPageBytes(Vec<u8>);

impl AbiPageBytes {
    pub fn try_new(bytes: Vec<u8>) -> Result<Self, AbiRejectedBytes> {
        if bytes.len() > ABI_MAX_PAGE_BYTES {
            Err(AbiRejectedBytes { code: AbiErrorCode::LimitExceeded, bytes })
        } else {
            Ok(Self(bytes))
        }
    }

    pub const fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// 💬 Owned bounded UTF-8 diagnostic bytes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AbiMessageBytes(Vec<u8>);

impl AbiMessageBytes {
    pub fn try_new(bytes: Vec<u8>) -> Result<Self, AbiRejectedBytes> {
        let code = if bytes.len() > ABI_MAX_MESSAGE_BYTES {
            Some(AbiErrorCode::LimitExceeded)
        } else if std::str::from_utf8(&bytes).is_err() {
            Some(AbiErrorCode::InvalidUtf8)
        } else {
            None
        };
        match code {
            Some(code) => Err(AbiRejectedBytes { code, bytes }),
            None => Ok(Self(bytes)),
        }
    }

    pub fn from_text(message: &str) -> Result<Self, AbiErrorCode> {
        Self::try_new(message.as_bytes().to_vec()).map_err(|rejected| rejected.code)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).expect("AbiMessageBytes construction validates UTF-8")
    }

    pub const fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }
}

/// ↩️ Capacity rejection that returns the caller's exact allocation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiRejectedBytes {
    pub code: AbiErrorCode,
    pub bytes: Vec<u8>,
}

/// 🚦 Stable wire status codes.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbiStatusCode {
    Ok = 0,
    Pending = 1,
    Cancelled = 2,
    Rejected = 3,
    Failed = 4,
    Closed = 5,
}

impl AbiStatusCode {
    fn decode(code: u16) -> Result<Self, AbiErrorCode> {
        match code {
            0 => Ok(Self::Ok),
            1 => Ok(Self::Pending),
            2 => Ok(Self::Cancelled),
            3 => Ok(Self::Rejected),
            4 => Ok(Self::Failed),
            5 => Ok(Self::Closed),
            _ => Err(AbiErrorCode::MalformedTag),
        }
    }
}

/// 🧯 Stable owned failure codes used by codecs, ports, and retained cursors.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbiErrorCode {
    MalformedTag = 1,
    MalformedLength = 2,
    InvalidUtf8 = 3,
    MissingField = 4,
    LimitExceeded = 5,
    UnknownOperation = 6,
    UnknownHandle = 7,
    StaleGeneration = 8,
    AbaHandle = 9,
    DuplicateAcknowledgement = 10,
    Interrupted = 11,
    Cancelled = 12,
    Sealed = 13,
    LateReply = 14,
    DuplicateReply = 15,
    OutOfOrderPage = 16,
    DeadlineExceeded = 17,
    NoCredit = 18,
    Busy = 19,
    Closed = 20,
    GenerationExhausted = 21,
}

impl AbiErrorCode {
    fn decode(code: u16) -> Result<Self, AbiErrorCode> {
        match code {
            1 => Ok(Self::MalformedTag),
            2 => Ok(Self::MalformedLength),
            3 => Ok(Self::InvalidUtf8),
            4 => Ok(Self::MissingField),
            5 => Ok(Self::LimitExceeded),
            6 => Ok(Self::UnknownOperation),
            7 => Ok(Self::UnknownHandle),
            8 => Ok(Self::StaleGeneration),
            9 => Ok(Self::AbaHandle),
            10 => Ok(Self::DuplicateAcknowledgement),
            11 => Ok(Self::Interrupted),
            12 => Ok(Self::Cancelled),
            13 => Ok(Self::Sealed),
            14 => Ok(Self::LateReply),
            15 => Ok(Self::DuplicateReply),
            16 => Ok(Self::OutOfOrderPage),
            17 => Ok(Self::DeadlineExceeded),
            18 => Ok(Self::NoCredit),
            19 => Ok(Self::Busy),
            20 => Ok(Self::Closed),
            21 => Ok(Self::GenerationExhausted),
            _ => Err(Self::MalformedTag),
        }
    }
}

impl Display for AbiErrorCode {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for AbiErrorCode {}

/// 🧾 Optional bounded diagnostic attached to a non-success status.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiError {
    pub code: AbiErrorCode,
    pub message: AbiMessageBytes,
}

/// 🚥 Owned status and optional diagnostic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiStatus {
    pub code: AbiStatusCode,
    pub error: Option<AbiError>,
}

impl AbiStatus {
    pub const OK: Self = Self { code: AbiStatusCode::Ok, error: None };
}

/// 📨 Domain operation admission envelope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiRequest {
    pub operation: AbiOperation,
    pub request_id: AbiRequestId,
    pub generation: u32,
    pub bytes: AbiBytes,
}

/// 📬 Correlated operation completion envelope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiReply {
    pub request_id: AbiRequestId,
    pub generation: u32,
    pub status: AbiStatus,
    pub bytes: AbiBytes,
}

/// 📣 Ordered correlated progress or lifecycle event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiEvent {
    pub request_id: AbiRequestId,
    pub generation: u32,
    pub sequence: u32,
    pub event: AbiEventCode,
    pub status: AbiStatus,
    pub bytes: AbiBytes,
}

/// 📄 One exact ACK-controlled transfer page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiPage {
    pub handle: AbiHandle,
    pub index: u32,
    pub bytes: AbiPageBytes,
}

impl AbiPage {
    pub fn try_new(handle: AbiHandle, index: u32, bytes: Vec<u8>) -> Result<Self, AbiRejectedPage> {
        if index >= ABI_MAX_PAGES_PER_TRANSFER || bytes.len() > ABI_MAX_PAGE_BYTES {
            Err(AbiRejectedPage { code: AbiErrorCode::LimitExceeded, handle, index, bytes })
        } else {
            Ok(Self { handle, index, bytes: AbiPageBytes(bytes) })
        }
    }
}

/// ↩️ Page rejection preserving handle, index, and allocation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiRejectedPage {
    pub code: AbiErrorCode,
    pub handle: AbiHandle,
    pub index: u32,
    pub bytes: Vec<u8>,
}

/// 🎛️ Exact lifecycle control; every handle-bearing variant includes its generation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbiControl {
    Cancel { request_id: AbiRequestId, generation: u32 },
    Close { handle: AbiHandle },
    Acknowledge { handle: AbiHandle, index: u32 },
}

/// 🧱 Closed set of wire envelopes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AbiMessage {
    Request(AbiRequest),
    Reply(AbiReply),
    Event(AbiEvent),
    Page(AbiPage),
    Control(AbiControl),
}

//#endregion 🧬️Schema

//#region 🧱️Codec

/// 🧱 Encodes the canonical versioned, little-endian structural ledger.
pub fn encode_abi_message(message: &AbiMessage) -> Vec<u8> {
    let mut encoder = Encoder::new();
    encoder.u8(ABI_VERSION);
    match message {
        AbiMessage::Request(value) => {
            encoder.u8(1);
            encoder.u16(value.operation.get());
            encoder.u64(value.request_id.0);
            encoder.u32(value.generation);
            encoder.bytes(value.bytes.as_slice());
        }
        AbiMessage::Reply(value) => {
            encoder.u8(2);
            encoder.u64(value.request_id.0);
            encoder.u32(value.generation);
            encoder.status(&value.status);
            encoder.bytes(value.bytes.as_slice());
        }
        AbiMessage::Event(value) => {
            encoder.u8(3);
            encoder.u64(value.request_id.0);
            encoder.u32(value.generation);
            encoder.u32(value.sequence);
            encoder.u16(value.event.get());
            encoder.status(&value.status);
            encoder.bytes(value.bytes.as_slice());
        }
        AbiMessage::Page(value) => {
            encoder.u8(4);
            encoder.handle(value.handle);
            encoder.u32(value.index);
            encoder.bytes(value.bytes.as_slice());
        }
        AbiMessage::Control(value) => {
            encoder.u8(5);
            match value {
                AbiControl::Cancel { request_id, generation } => {
                    encoder.u8(1);
                    encoder.u64(request_id.0);
                    encoder.u32(*generation);
                }
                AbiControl::Close { handle } => {
                    encoder.u8(2);
                    encoder.handle(*handle);
                }
                AbiControl::Acknowledge { handle, index } => {
                    encoder.u8(3);
                    encoder.handle(*handle);
                    encoder.u32(*index);
                }
            }
        }
    }
    encoder.finish()
}

/// 🔬 Decodes exactly one structural ledger record and rejects trailing or partial fields.
pub fn decode_abi_message(bytes: &[u8]) -> Result<AbiMessage, AbiErrorCode> {
    let mut decoder = Decoder::new(bytes);
    if decoder.u8()? != ABI_VERSION {
        return Err(AbiErrorCode::MalformedTag);
    }
    let message = match decoder.u8()? {
        1 => AbiMessage::Request(AbiRequest { operation: AbiOperation::try_new(decoder.u16()?)?, request_id: AbiRequestId(decoder.u64()?), generation: decoder.u32()?, bytes: AbiBytes(decoder.bytes(ABI_MAX_BODY_BYTES)?) }),
        2 => AbiMessage::Reply(AbiReply { request_id: AbiRequestId(decoder.u64()?), generation: decoder.u32()?, status: decoder.status()?, bytes: AbiBytes(decoder.bytes(ABI_MAX_BODY_BYTES)?) }),
        3 => AbiMessage::Event(AbiEvent {
            request_id: AbiRequestId(decoder.u64()?),
            generation: decoder.u32()?,
            sequence: decoder.u32()?,
            event: AbiEventCode::try_new(decoder.u16()?)?,
            status: decoder.status()?,
            bytes: AbiBytes(decoder.bytes(ABI_MAX_BODY_BYTES)?),
        }),
        4 => {
            let handle = decoder.handle()?;
            let index = decoder.u32()?;
            if index >= ABI_MAX_PAGES_PER_TRANSFER {
                return Err(AbiErrorCode::LimitExceeded);
            }
            AbiMessage::Page(AbiPage { handle, index, bytes: AbiPageBytes(decoder.bytes(ABI_MAX_PAGE_BYTES)?) })
        }
        5 => AbiMessage::Control(match decoder.u8()? {
            1 => AbiControl::Cancel { request_id: AbiRequestId(decoder.u64()?), generation: decoder.u32()? },
            2 => AbiControl::Close { handle: decoder.handle()? },
            3 => {
                let handle = decoder.handle()?;
                let index = decoder.u32()?;
                if index >= ABI_MAX_PAGES_PER_TRANSFER {
                    return Err(AbiErrorCode::LimitExceeded);
                }
                AbiControl::Acknowledge { handle, index }
            }
            _ => return Err(AbiErrorCode::MalformedTag),
        }),
        _ => return Err(AbiErrorCode::MalformedTag),
    };
    if decoder.remaining() != 0 {
        return Err(AbiErrorCode::MalformedLength);
    }
    Ok(message)
}

struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }

    fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    fn u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn bytes(&mut self, bytes: &[u8]) {
        self.u32(bytes.len() as u32);
        self.bytes.extend_from_slice(bytes);
    }

    fn handle(&mut self, handle: AbiHandle) {
        self.u32(handle.slot);
        self.u32(handle.generation);
    }

    fn status(&mut self, status: &AbiStatus) {
        self.u16(status.code as u16);
        match &status.error {
            None => self.u8(0),
            Some(error) => {
                self.u8(1);
                self.u16(error.code as u16);
                self.u16(error.message.as_bytes().len() as u16);
                self.bytes.extend_from_slice(error.message.as_bytes());
            }
        }
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

    fn remaining(&self) -> usize {
        self.bytes.len() - self.cursor
    }

    fn take(&mut self, count: usize, incomplete: AbiErrorCode) -> Result<&'a [u8], AbiErrorCode> {
        let end = self.cursor.checked_add(count).ok_or(AbiErrorCode::MalformedLength)?;
        if end > self.bytes.len() {
            return Err(incomplete);
        }
        let bytes = &self.bytes[self.cursor..end];
        self.cursor = end;
        Ok(bytes)
    }

    fn u8(&mut self) -> Result<u8, AbiErrorCode> {
        Ok(self.take(1, AbiErrorCode::MissingField)?[0])
    }

    fn u16(&mut self) -> Result<u16, AbiErrorCode> {
        Ok(u16::from_le_bytes(self.take(2, AbiErrorCode::MissingField)?.try_into().expect("fixed width")))
    }

    fn u32(&mut self) -> Result<u32, AbiErrorCode> {
        Ok(u32::from_le_bytes(self.take(4, AbiErrorCode::MissingField)?.try_into().expect("fixed width")))
    }

    fn u64(&mut self) -> Result<u64, AbiErrorCode> {
        Ok(u64::from_le_bytes(self.take(8, AbiErrorCode::MissingField)?.try_into().expect("fixed width")))
    }

    fn bytes(&mut self, limit: usize) -> Result<Vec<u8>, AbiErrorCode> {
        let len = self.u32()? as usize;
        if len > limit {
            return Err(AbiErrorCode::LimitExceeded);
        }
        Ok(self.take(len, AbiErrorCode::MalformedLength)?.to_vec())
    }

    fn handle(&mut self) -> Result<AbiHandle, AbiErrorCode> {
        AbiHandle::try_new(self.u32()?, self.u32()?)
    }

    fn status(&mut self) -> Result<AbiStatus, AbiErrorCode> {
        let code = AbiStatusCode::decode(self.u16()?)?;
        let error = match self.u8()? {
            0 => None,
            1 => {
                let code = AbiErrorCode::decode(self.u16()?)?;
                let len = self.u16()? as usize;
                if len > ABI_MAX_MESSAGE_BYTES {
                    return Err(AbiErrorCode::LimitExceeded);
                }
                let bytes = self.take(len, AbiErrorCode::MalformedLength)?.to_vec();
                Some(AbiError { code, message: AbiMessageBytes::try_new(bytes).map_err(|rejected| rejected.code)? })
            }
            _ => return Err(AbiErrorCode::MalformedTag),
        };
        Ok(AbiStatus { code, error })
    }
}

//#endregion 🧱️Codec

//#region ⏳️RetainedTransfer

/// ⏱️ One explicit retained-copy allowance; no cursor advances when admission fails.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AbiWorkBudget {
    pub byte_credit: usize,
    pub now_ms: u64,
    pub deadline_ms: Option<u64>,
    pub cancelled: bool,
    pub interrupted: bool,
}

impl AbiWorkBudget {
    pub const fn credits(byte_credit: usize) -> Self {
        Self { byte_credit, now_ms: 0, deadline_ms: None, cancelled: false, interrupted: false }
    }

    fn permit(self, remaining: usize) -> Result<usize, AbiErrorCode> {
        if self.cancelled {
            return Err(AbiErrorCode::Cancelled);
        }
        if self.interrupted {
            return Err(AbiErrorCode::Interrupted);
        }
        if self.deadline_ms.is_some_and(|deadline| self.now_ms >= deadline) {
            return Err(AbiErrorCode::DeadlineExceeded);
        }
        if self.byte_credit == 0 {
            return Err(AbiErrorCode::NoCredit);
        }
        Ok(self.byte_credit.min(remaining))
    }
}

/// 🔄 Cursor step shared by retained readers, writers, and close retirement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbiCursorStep {
    Advanced(usize),
    PageComplete(u32),
    AwaitingAcknowledgement(u32),
    Idle,
    Complete,
}

/// ↩️ Cancellation handback plus the exact admitted and already-copied byte credits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiCancelOutcome {
    pub page: Option<AbiPage>,
    pub admitted_byte_credits: usize,
    pub copied_bytes: usize,
}

struct PendingWrite {
    page: AbiPage,
    cursor: usize,
}

/// ✍️ Retains an admitted page and advances its copy only under explicit work credit.
pub struct AbiPageWriter {
    handle: AbiHandle,
    bytes: Vec<u8>,
    next_index: u32,
    pending: Option<PendingWrite>,
    sealed: bool,
    cancelled: bool,
    closing: bool,
}

impl AbiPageWriter {
    pub fn new(handle: AbiHandle) -> Self {
        Self { handle, bytes: Vec::new(), next_index: 0, pending: None, sealed: false, cancelled: false, closing: false }
    }

    pub fn offer(&mut self, page: AbiPage) -> Result<(), AbiRejectedPage> {
        let reject = |code, page: AbiPage| AbiRejectedPage { code, handle: page.handle, index: page.index, bytes: page.bytes.into_vec() };
        if let Err(code) = classify_handle(self.handle, page.handle) {
            return Err(reject(code, page));
        }
        if self.closing {
            return Err(reject(AbiErrorCode::Closed, page));
        }
        if self.cancelled {
            return Err(reject(AbiErrorCode::Cancelled, page));
        }
        if self.sealed {
            return Err(reject(AbiErrorCode::Sealed, page));
        }
        if self.pending.is_some() {
            return Err(reject(AbiErrorCode::Busy, page));
        }
        if page.index != self.next_index {
            return Err(reject(AbiErrorCode::OutOfOrderPage, page));
        }
        if self.next_index >= ABI_MAX_PAGES_PER_TRANSFER || self.bytes.len().checked_add(page.bytes.len()).is_none_or(|len| len > ABI_MAX_TRANSFER_BYTES) {
            return Err(reject(AbiErrorCode::LimitExceeded, page));
        }
        self.pending = Some(PendingWrite { page, cursor: 0 });
        Ok(())
    }

    pub fn write_step(&mut self, budget: AbiWorkBudget) -> Result<AbiCursorStep, AbiErrorCode> {
        if self.cancelled {
            return Err(AbiErrorCode::Cancelled);
        }
        let Some(pending) = self.pending.as_mut() else {
            return Ok(if self.sealed { AbiCursorStep::Complete } else { AbiCursorStep::Idle });
        };
        let remaining = pending.page.bytes.len() - pending.cursor;
        if remaining == 0 {
            let index = pending.page.index;
            self.pending = None;
            self.next_index += 1;
            return Ok(AbiCursorStep::PageComplete(index));
        }
        let permitted = budget.permit(remaining)?;
        let end = pending.cursor + permitted;
        self.bytes.extend_from_slice(&pending.page.bytes.as_slice()[pending.cursor..end]);
        pending.cursor = end;
        if end == pending.page.bytes.len() {
            let index = pending.page.index;
            self.pending = None;
            self.next_index += 1;
            Ok(AbiCursorStep::PageComplete(index))
        } else {
            Ok(AbiCursorStep::Advanced(permitted))
        }
    }

    pub fn seal(&mut self) -> Result<(), AbiErrorCode> {
        if self.cancelled {
            return Err(AbiErrorCode::Cancelled);
        }
        if self.pending.is_some() {
            return Err(AbiErrorCode::Busy);
        }
        self.sealed = true;
        Ok(())
    }

    pub fn cancel(&mut self) -> AbiCancelOutcome {
        self.cancelled = true;
        let page = self.pending.take().map(|pending| pending.page);
        AbiCancelOutcome { admitted_byte_credits: page.as_ref().map_or(0, |page| page.bytes.len()), copied_bytes: self.bytes.len(), page }
    }

    pub const fn is_sealed(&self) -> bool {
        self.sealed
    }

    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn close_step(&mut self, budget: AbiWorkBudget) -> Result<AbiCursorStep, AbiErrorCode> {
        self.closing = true;
        let pending_len = self.pending.as_ref().map_or(0, |pending| pending.page.bytes.len());
        let remaining = self.bytes.len() + pending_len;
        if remaining == 0 {
            self.pending = None;
            return Ok(AbiCursorStep::Complete);
        }
        let permitted = budget.permit(remaining)?;
        let from_pending = permitted.min(pending_len);
        if let Some(pending) = self.pending.as_mut() {
            let new_len = pending.page.bytes.0.len() - from_pending;
            pending.page.bytes.0.truncate(new_len);
            pending.cursor = pending.cursor.min(new_len);
            if pending.page.bytes.is_empty() {
                self.pending = None;
            }
        }
        let from_bytes = permitted - from_pending;
        self.bytes.truncate(self.bytes.len() - from_bytes.min(self.bytes.len()));
        Ok(if self.terminal_is_empty() { AbiCursorStep::Complete } else { AbiCursorStep::Advanced(permitted) })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.bytes.is_empty() && self.pending.is_none()
    }
}

/// 📖 Retains source, staging cursor, and one exact outstanding page until its ACK arrives.
pub struct AbiPageReader {
    handle: AbiHandle,
    source: Vec<u8>,
    source_cursor: usize,
    next_index: u32,
    staging: Vec<u8>,
    target_len: usize,
    outstanding: Option<AbiPage>,
    last_acked: Option<u32>,
    cancelled: bool,
    closing: bool,
}

impl AbiPageReader {
    pub fn try_new(handle: AbiHandle, source: Vec<u8>) -> Result<Self, AbiRejectedBytes> {
        if source.len() > ABI_MAX_TRANSFER_BYTES {
            return Err(AbiRejectedBytes { code: AbiErrorCode::LimitExceeded, bytes: source });
        }
        Ok(Self { handle, source, source_cursor: 0, next_index: 0, staging: Vec::new(), target_len: 0, outstanding: None, last_acked: None, cancelled: false, closing: false })
    }

    pub fn read_step(&mut self, budget: AbiWorkBudget) -> Result<AbiCursorStep, AbiErrorCode> {
        if self.closing {
            return Err(AbiErrorCode::Closed);
        }
        if self.cancelled {
            return Err(AbiErrorCode::Cancelled);
        }
        if let Some(page) = &self.outstanding {
            return Ok(AbiCursorStep::AwaitingAcknowledgement(page.index));
        }
        if self.source_cursor == self.source.len() {
            return Ok(AbiCursorStep::Complete);
        }
        let target_len = if self.target_len == 0 { ABI_MAX_PAGE_BYTES.min(self.source.len() - self.source_cursor) } else { self.target_len };
        let remaining = target_len - self.staging.len();
        let permitted = budget.permit(remaining)?;
        self.staging.try_reserve_exact(permitted).map_err(|_| AbiErrorCode::LimitExceeded)?;
        self.target_len = target_len;
        let end = self.source_cursor + permitted;
        self.staging.extend_from_slice(&self.source[self.source_cursor..end]);
        self.source_cursor = end;
        if self.staging.len() == self.target_len {
            let index = self.next_index;
            let bytes = std::mem::take(&mut self.staging);
            self.target_len = 0;
            self.outstanding = Some(AbiPage { handle: self.handle, index, bytes: AbiPageBytes(bytes) });
            Ok(AbiCursorStep::PageComplete(index))
        } else {
            Ok(AbiCursorStep::Advanced(permitted))
        }
    }

    pub fn page(&self) -> Option<&AbiPage> {
        self.outstanding.as_ref()
    }

    pub fn acknowledge(&mut self, control: AbiControl) -> Result<(), AbiErrorCode> {
        let AbiControl::Acknowledge { handle, index } = control else {
            return Err(AbiErrorCode::MalformedTag);
        };
        classify_handle(self.handle, handle)?;
        if self.last_acked == Some(index) {
            return Err(AbiErrorCode::DuplicateAcknowledgement);
        }
        let Some(page) = self.outstanding.as_ref() else {
            return Err(AbiErrorCode::UnknownHandle);
        };
        if page.index != index {
            return Err(AbiErrorCode::OutOfOrderPage);
        }
        self.outstanding = None;
        self.last_acked = Some(index);
        self.next_index += 1;
        Ok(())
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn close_step(&mut self, budget: AbiWorkBudget) -> Result<AbiCursorStep, AbiErrorCode> {
        self.closing = true;
        let outstanding_len = self.outstanding.as_ref().map_or(0, |page| page.bytes.len());
        let remaining = self.source.len() + self.staging.len() + outstanding_len;
        if remaining == 0 {
            self.outstanding = None;
            return Ok(AbiCursorStep::Complete);
        }
        let mut permitted = budget.permit(remaining)?;
        if let Some(page) = self.outstanding.as_mut() {
            let remove = permitted.min(page.bytes.len());
            page.bytes.0.truncate(page.bytes.len() - remove);
            permitted -= remove;
            if page.bytes.is_empty() {
                self.outstanding = None;
            }
        }
        let remove = permitted.min(self.staging.len());
        self.staging.truncate(self.staging.len() - remove);
        permitted -= remove;
        self.source.truncate(self.source.len() - permitted.min(self.source.len()));
        self.source_cursor = self.source_cursor.min(self.source.len());
        Ok(if self.terminal_is_empty() { AbiCursorStep::Complete } else { AbiCursorStep::Advanced(budget.byte_credit.min(remaining)) })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.source.is_empty() && self.staging.is_empty() && self.outstanding.is_none()
    }
}

fn classify_handle(expected: AbiHandle, actual: AbiHandle) -> Result<(), AbiErrorCode> {
    if actual.slot != expected.slot {
        Err(AbiErrorCode::UnknownHandle)
    } else if actual.generation < expected.generation {
        Err(AbiErrorCode::AbaHandle)
    } else if actual.generation > expected.generation {
        Err(AbiErrorCode::StaleGeneration)
    } else {
        Ok(())
    }
}

//#endregion ⏳️RetainedTransfer

//#region 🗃️Ledgers

struct HandleSlot<T> {
    generation: u32,
    value: Option<T>,
}

/// 🗃️ Fixed-capacity owned handle table with generation-preserving slot reuse.
pub struct AbiHandleTable<T> {
    slots: Vec<HandleSlot<T>>,
    free: Vec<usize>,
    quarantined: usize,
}

impl<T> Default for AbiHandleTable<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AbiHandleTable<T> {
    pub const fn new() -> Self {
        Self { slots: Vec::new(), free: Vec::new(), quarantined: 0 }
    }

    pub fn open(&mut self, value: T) -> Result<AbiHandle, (AbiErrorCode, T)> {
        if let Some(index) = self.free.pop() {
            let slot = &mut self.slots[index];
            let Some(generation) = slot.generation.checked_add(1) else {
                self.quarantined += 1;
                return Err((AbiErrorCode::GenerationExhausted, value));
            };
            slot.generation = generation;
            slot.value = Some(value);
            return Ok(AbiHandle { slot: index as u32 + 1, generation: slot.generation });
        }
        if self.slots.len() == ABI_MAX_IN_FLIGHT_HANDLES {
            let code = if self.quarantined == 0 { AbiErrorCode::LimitExceeded } else { AbiErrorCode::GenerationExhausted };
            return Err((code, value));
        }
        self.slots.push(HandleSlot { generation: 1, value: Some(value) });
        Ok(AbiHandle { slot: self.slots.len() as u32, generation: 1 })
    }

    pub fn get(&self, handle: AbiHandle) -> Result<&T, AbiErrorCode> {
        let slot = self.slot(handle)?;
        slot.value.as_ref().ok_or(AbiErrorCode::UnknownHandle)
    }

    pub fn get_mut(&mut self, handle: AbiHandle) -> Result<&mut T, AbiErrorCode> {
        let slot = self.slot_mut(handle)?;
        slot.value.as_mut().ok_or(AbiErrorCode::UnknownHandle)
    }

    pub fn close(&mut self, handle: AbiHandle) -> Result<T, AbiErrorCode> {
        let index = handle.slot.checked_sub(1).ok_or(AbiErrorCode::UnknownHandle)? as usize;
        let slot = self.slots.get_mut(index).ok_or(AbiErrorCode::UnknownHandle)?;
        compare_generation(slot.generation, handle.generation)?;
        let value = slot.value.take().ok_or(AbiErrorCode::UnknownHandle)?;
        if slot.generation == u32::MAX {
            self.quarantined += 1;
        } else {
            self.free.push(index);
        }
        Ok(value)
    }

    pub fn lose(&mut self, handle: AbiHandle) -> Result<T, AbiErrorCode> {
        self.close(handle)
    }

    fn slot(&self, handle: AbiHandle) -> Result<&HandleSlot<T>, AbiErrorCode> {
        let slot = handle.slot.checked_sub(1).and_then(|index| self.slots.get(index as usize)).ok_or(AbiErrorCode::UnknownHandle)?;
        compare_generation(slot.generation, handle.generation)?;
        Ok(slot)
    }

    fn slot_mut(&mut self, handle: AbiHandle) -> Result<&mut HandleSlot<T>, AbiErrorCode> {
        let slot = handle.slot.checked_sub(1).and_then(|index| self.slots.get_mut(index as usize)).ok_or(AbiErrorCode::UnknownHandle)?;
        compare_generation(slot.generation, handle.generation)?;
        Ok(slot)
    }
}

fn compare_generation(current: u32, actual: u32) -> Result<(), AbiErrorCode> {
    if actual < current {
        Err(AbiErrorCode::AbaHandle)
    } else if actual > current {
        Err(AbiErrorCode::StaleGeneration)
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct ReplySlot {
    request_id: AbiRequestId,
    generation: u32,
    state: ReplyState,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReplyState {
    Active,
    Completed,
    Lost,
}

/// 📬 Fixed-capacity correlation ledger rejecting duplicate and late completions.
pub struct AbiReplyLedger {
    slots: [Option<ReplySlot>; ABI_MAX_IN_FLIGHT_REQUESTS],
}

impl Default for AbiReplyLedger {
    fn default() -> Self {
        Self::new()
    }
}

impl AbiReplyLedger {
    pub const fn new() -> Self {
        Self { slots: [None; ABI_MAX_IN_FLIGHT_REQUESTS] }
    }

    pub fn admit(&mut self, request_id: AbiRequestId, generation: u32) -> Result<(), AbiErrorCode> {
        let index = reply_slot_index(request_id);
        if let Some(slot) = self.slots[index].as_ref() {
            if slot.request_id != request_id && slot.state == ReplyState::Active {
                return Err(AbiErrorCode::Busy);
            }
            if slot.request_id == request_id && generation <= slot.generation {
                return Err(if generation == slot.generation && slot.state == ReplyState::Active { AbiErrorCode::Busy } else { AbiErrorCode::AbaHandle });
            }
        }
        self.slots[index] = Some(ReplySlot { request_id, generation, state: ReplyState::Active });
        Ok(())
    }

    pub fn accept(&mut self, reply: &AbiReply) -> Result<(), AbiErrorCode> {
        let slot = self.slots[reply_slot_index(reply.request_id)].as_mut().filter(|slot| slot.request_id == reply.request_id).ok_or(AbiErrorCode::LateReply)?;
        if slot.generation != reply.generation || slot.state == ReplyState::Lost {
            return Err(AbiErrorCode::LateReply);
        }
        if slot.state == ReplyState::Completed {
            return Err(AbiErrorCode::DuplicateReply);
        }
        slot.state = ReplyState::Completed;
        Ok(())
    }

    pub fn lose(&mut self, request_id: AbiRequestId, generation: u32) -> Result<(), AbiErrorCode> {
        let slot = self.slots[reply_slot_index(request_id)].as_mut().filter(|slot| slot.request_id == request_id && slot.generation == generation).ok_or(AbiErrorCode::UnknownHandle)?;
        slot.state = ReplyState::Lost;
        Ok(())
    }
}

fn reply_slot_index(request_id: AbiRequestId) -> usize {
    (request_id.0 % ABI_MAX_IN_FLIGHT_REQUESTS as u64) as usize
}

//#endregion 🗃️Ledgers

//#region 🔌️Port

/// 💤 Result of polling the future generated host shim through an owned port.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AbiPortPoll {
    Pending,
    Message(AbiMessage),
    Closed,
}

/// ↩️ Port admission failure returning the exact owned envelope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiPortRejection {
    pub code: AbiErrorCode,
    pub message: AbiMessage,
}

/// 🔌 Primitive-only seam implemented later by native or generated JS host shims.
pub trait AbiPort {
    fn try_send(&mut self, message: AbiMessage, budget: AbiWorkBudget) -> Result<(), AbiPortRejection>;
    fn poll(&mut self, budget: AbiWorkBudget) -> Result<AbiPortPoll, AbiErrorCode>;
}

//#endregion 🔌️Port

//#region 🧪️Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(code: u16) -> AbiOperation {
        AbiOperation::try_new(code).unwrap()
    }

    fn bytes(value: Vec<u8>) -> AbiBytes {
        AbiBytes::try_new(value).unwrap()
    }

    fn handle(slot: u32, generation: u32) -> AbiHandle {
        AbiHandle { slot, generation }
    }

    fn reply(request_id: u64, generation: u32) -> AbiReply {
        AbiReply { request_id: AbiRequestId(request_id), generation, status: AbiStatus::OK, bytes: bytes(Vec::new()) }
    }

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    fn unhex(value: &str) -> Vec<u8> {
        value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
    }

    fn fixtures() -> Vec<(&'static str, Vec<u8>)> {
        ABI_LEDGER_FIXTURE
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| {
                let (name, value) = line.split_once('\t').unwrap();
                (name, unhex(value))
            })
            .collect()
    }

    #[test]
    fn schema_and_language_agnostic_fixture_publish_all_owned_contracts() {
        for name in ["AbiRequest", "AbiReply", "AbiEvent", "AbiPage", "AbiControl", "AbiError"] {
            assert!(ABI_SCHEMA_JSON.contains(&format!("\"{name}\"")), "missing {name}");
        }
        for limit in [ABI_MAX_OPERATION_CODE, ABI_MAX_EVENT_CODE] {
            assert!(ABI_SCHEMA_JSON.contains(&limit.to_string()));
        }
        assert_eq!(fixtures().len(), 8);
        let expected_limits = [
            ("operation-code", ABI_MAX_OPERATION_CODE as u64),
            ("event-code", ABI_MAX_EVENT_CODE as u64),
            ("body-bytes", ABI_MAX_BODY_BYTES as u64),
            ("page-bytes", ABI_MAX_PAGE_BYTES as u64),
            ("message-bytes", ABI_MAX_MESSAGE_BYTES as u64),
            ("pages-per-transfer", ABI_MAX_PAGES_PER_TRANSFER as u64),
            ("transfer-bytes", ABI_MAX_TRANSFER_BYTES as u64),
            ("in-flight-handles", ABI_MAX_IN_FLIGHT_HANDLES as u64),
            ("in-flight-requests", ABI_MAX_IN_FLIGHT_REQUESTS as u64),
            ("handle-generation", u32::MAX as u64),
        ];
        let parsed_limits: Vec<_> = ABI_LIMITS_FIXTURE
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| {
                let fields: Vec<_> = line.split('\t').collect();
                (fields[0], fields[1].parse::<u64>().unwrap(), fields[2].parse::<u64>().unwrap())
            })
            .collect();
        assert_eq!(parsed_limits.len(), expected_limits.len());
        for ((name, maximum, plus_one), expected) in parsed_limits.into_iter().zip(expected_limits) {
            assert_eq!((name, maximum, plus_one), (expected.0, expected.1, expected.1 + 1));
        }
    }

    #[test]
    fn empty_single_max_and_max_plus_one_preserve_bounds_and_caller_bytes() {
        let empty = AbiMessage::Request(AbiRequest { operation: operation(1), request_id: AbiRequestId(0), generation: 1, bytes: bytes(Vec::new()) });
        assert_eq!(decode_abi_message(&encode_abi_message(&empty)), Ok(empty));
        let single = AbiPage::try_new(handle(1, 1), 0, vec![7]).unwrap();
        assert_eq!(single.bytes.as_slice(), &[7]);
        let maximum = AbiPage::try_new(handle(1, 1), ABI_MAX_PAGES_PER_TRANSFER - 1, vec![3; ABI_MAX_PAGE_BYTES]).unwrap();
        assert_eq!(maximum.bytes.len(), ABI_MAX_PAGE_BYTES);
        let original = vec![9; ABI_MAX_PAGE_BYTES + 1];
        let rejected = AbiPage::try_new(handle(1, 1), 0, original.clone()).unwrap_err();
        assert_eq!(rejected.code, AbiErrorCode::LimitExceeded);
        assert_eq!(rejected.bytes, original);
        assert_eq!(AbiOperation::try_new(ABI_MAX_OPERATION_CODE).unwrap().get(), ABI_MAX_OPERATION_CODE);
        assert_eq!(AbiOperation::try_new(ABI_MAX_OPERATION_CODE + 1), Err(AbiErrorCode::UnknownOperation));
        let maximum_body = AbiBytes::try_new(vec![5; ABI_MAX_BODY_BYTES]).unwrap();
        assert_eq!(maximum_body.len(), ABI_MAX_BODY_BYTES);
        let maximum_message = AbiMessageBytes::try_new(vec![b'm'; ABI_MAX_MESSAGE_BYTES]).unwrap();
        assert_eq!(maximum_message.as_bytes().len(), ABI_MAX_MESSAGE_BYTES);
        let oversized_message = vec![b'm'; ABI_MAX_MESSAGE_BYTES + 1];
        assert_eq!(AbiMessageBytes::try_new(oversized_message.clone()).unwrap_err().bytes, oversized_message);
    }

    #[test]
    fn native_and_wasm_ledger_is_fixed_little_endian_and_deterministic() {
        let records = [
            AbiMessage::Request(AbiRequest { operation: operation(7), request_id: AbiRequestId(1), generation: 2, bytes: bytes(vec![b'A']) }),
            AbiMessage::Reply(AbiReply { request_id: AbiRequestId(1), generation: 2, status: AbiStatus::OK, bytes: bytes(vec![b'B']) }),
            AbiMessage::Event(AbiEvent { request_id: AbiRequestId(1), generation: 2, sequence: 3, event: AbiEventCode::try_new(4).unwrap(), status: AbiStatus::OK, bytes: bytes(vec![b'C']) }),
            AbiMessage::Page(AbiPage::try_new(handle(5, 6), 7, vec![b'D']).unwrap()),
            AbiMessage::Page(AbiPage::try_new(handle(5, 6), 0, Vec::new()).unwrap()),
            AbiMessage::Control(AbiControl::Cancel { request_id: AbiRequestId(8), generation: 9 }),
            AbiMessage::Control(AbiControl::Close { handle: handle(5, 6) }),
            AbiMessage::Control(AbiControl::Acknowledge { handle: handle(5, 6), index: 7 }),
        ];
        for ((name, expected), record) in fixtures().into_iter().zip(records) {
            let encoded = encode_abi_message(&record);
            assert_eq!(hex(&encoded), hex(&expected), "{name}");
            assert_eq!(decode_abi_message(&encoded), Ok(record), "{name}");
        }
    }

    #[test]
    fn malformed_tag_length_utf8_and_missing_optional_fail_closed() {
        assert_eq!(decode_abi_message(&[ABI_VERSION, 255]), Err(AbiErrorCode::MalformedTag));
        let mut unknown_operation = fixtures()[0].1.clone();
        unknown_operation[2..4].copy_from_slice(&0u16.to_le_bytes());
        assert_eq!(decode_abi_message(&unknown_operation), Err(AbiErrorCode::UnknownOperation));
        let mut malformed_length = encode_abi_message(&AbiMessage::Request(AbiRequest { operation: operation(1), request_id: AbiRequestId(1), generation: 1, bytes: bytes(vec![1]) }));
        malformed_length[16..20].copy_from_slice(&2u32.to_le_bytes());
        assert_eq!(decode_abi_message(&malformed_length), Err(AbiErrorCode::MalformedLength));
        let mut invalid_utf8 = vec![ABI_VERSION, 2];
        invalid_utf8.extend_from_slice(&1u64.to_le_bytes());
        invalid_utf8.extend_from_slice(&1u32.to_le_bytes());
        invalid_utf8.extend_from_slice(&(AbiStatusCode::Failed as u16).to_le_bytes());
        invalid_utf8.push(1);
        invalid_utf8.extend_from_slice(&(AbiErrorCode::Interrupted as u16).to_le_bytes());
        invalid_utf8.extend_from_slice(&1u16.to_le_bytes());
        invalid_utf8.push(0xff);
        invalid_utf8.extend_from_slice(&0u32.to_le_bytes());
        assert_eq!(decode_abi_message(&invalid_utf8), Err(AbiErrorCode::InvalidUtf8));
        let mut missing_optional = vec![ABI_VERSION, 2];
        missing_optional.extend_from_slice(&1u64.to_le_bytes());
        missing_optional.extend_from_slice(&1u32.to_le_bytes());
        missing_optional.extend_from_slice(&(AbiStatusCode::Ok as u16).to_le_bytes());
        assert_eq!(decode_abi_message(&missing_optional), Err(AbiErrorCode::MissingField));
        let mut zero_handle = fixtures()[3].1.clone();
        zero_handle[2..6].copy_from_slice(&0u32.to_le_bytes());
        assert_eq!(decode_abi_message(&zero_handle), Err(AbiErrorCode::UnknownHandle));
    }

    #[test]
    fn retained_writer_is_credit_deadline_and_interruption_aware() {
        let identity = handle(2, 4);
        let mut writer = AbiPageWriter::new(identity);
        writer.offer(AbiPage::try_new(identity, 0, b"abcd".to_vec()).unwrap()).unwrap();
        assert_eq!(writer.write_step(AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(4) }), Err(AbiErrorCode::Interrupted));
        assert!(writer.bytes().is_empty());
        assert_eq!(writer.write_step(AbiWorkBudget { now_ms: 7, deadline_ms: Some(7), ..AbiWorkBudget::credits(4) }), Err(AbiErrorCode::DeadlineExceeded));
        assert!(writer.bytes().is_empty());
        assert_eq!(writer.write_step(AbiWorkBudget::credits(2)), Ok(AbiCursorStep::Advanced(2)));
        assert_eq!(writer.write_step(AbiWorkBudget::credits(2)), Ok(AbiCursorStep::PageComplete(0)));
        assert_eq!(writer.bytes(), b"abcd");
    }

    #[test]
    fn exact_ack_duplicate_ack_and_generation_errors_are_distinct() {
        let identity = handle(3, 9);
        let mut reader = AbiPageReader::try_new(identity, b"page".to_vec()).unwrap();
        assert_eq!(reader.read_step(AbiWorkBudget::credits(4)), Ok(AbiCursorStep::PageComplete(0)));
        assert_eq!(reader.acknowledge(AbiControl::Acknowledge { handle: handle(4, 9), index: 0 }), Err(AbiErrorCode::UnknownHandle));
        assert_eq!(reader.acknowledge(AbiControl::Acknowledge { handle: handle(3, 8), index: 0 }), Err(AbiErrorCode::AbaHandle));
        assert_eq!(reader.acknowledge(AbiControl::Acknowledge { handle: handle(3, 10), index: 0 }), Err(AbiErrorCode::StaleGeneration));
        let ack = AbiControl::Acknowledge { handle: identity, index: 0 };
        assert_eq!(reader.acknowledge(ack), Ok(()));
        assert_eq!(reader.acknowledge(ack), Err(AbiErrorCode::DuplicateAcknowledgement));
        assert_eq!(reader.read_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::Complete));
    }

    #[test]
    fn handle_table_rejects_unknown_stale_and_aba_reuse() {
        let mut table = AbiHandleTable::new();
        assert_eq!(table.get(handle(7, 1)), Err(AbiErrorCode::UnknownHandle));
        let first = table.open("first").unwrap();
        assert_eq!(table.get(handle(first.slot, first.generation + 1)), Err(AbiErrorCode::StaleGeneration));
        assert_eq!(table.close(first), Ok("first"));
        let second = table.open("second").unwrap();
        assert_eq!(second.slot, first.slot);
        assert_ne!(second.generation, first.generation);
        assert_eq!(table.get(first), Err(AbiErrorCode::AbaHandle));
        assert_eq!(table.get(second), Ok(&"second"));
    }

    #[test]
    fn handle_generation_exhaustion_quarantines_slots_without_aliasing() {
        let mut one = AbiHandleTable::new();
        let first = one.open("near-max").unwrap();
        one.slots[0].generation = u32::MAX - 1;
        let near_max = handle(first.slot, u32::MAX - 1);
        assert_eq!(one.close(near_max), Ok("near-max"));
        let maximum = one.open("maximum").unwrap();
        assert_eq!(maximum, handle(first.slot, u32::MAX));
        assert_eq!(one.close(maximum), Ok("maximum"));
        let replacement = one.open("fresh-slot").unwrap();
        assert_ne!(replacement.slot, maximum.slot);
        assert_eq!(one.get(maximum), Err(AbiErrorCode::UnknownHandle));

        let mut exhausted = AbiHandleTable::new();
        let mut handles = Vec::new();
        for value in 0..ABI_MAX_IN_FLIGHT_HANDLES {
            let opened = exhausted.open(value).unwrap();
            exhausted.slots[value].generation = u32::MAX;
            handles.push(handle(opened.slot, u32::MAX));
        }
        for (value, handle) in handles.into_iter().enumerate() {
            assert_eq!(exhausted.close(handle), Ok(value));
        }
        assert_eq!(exhausted.open(999), Err((AbiErrorCode::GenerationExhausted, 999)));
    }

    #[test]
    fn cancel_before_and_after_seal_are_terminal_and_non_advancing() {
        let identity = handle(1, 1);
        let mut before = AbiPageWriter::new(identity);
        assert_eq!(before.cancel(), AbiCancelOutcome { page: None, admitted_byte_credits: 0, copied_bytes: 0 });
        let rejected = before.offer(AbiPage::try_new(identity, 0, vec![1]).unwrap()).unwrap_err();
        assert_eq!(rejected.code, AbiErrorCode::Cancelled);
        assert_eq!(rejected.bytes, vec![1]);
        let mut after = AbiPageWriter::new(identity);
        after.seal().unwrap();
        assert_eq!(after.cancel(), AbiCancelOutcome { page: None, admitted_byte_credits: 0, copied_bytes: 0 });
        assert!(after.is_sealed() && after.is_cancelled());
        assert_eq!(after.write_step(AbiWorkBudget::credits(1)), Err(AbiErrorCode::Cancelled));

        let mut admitted = AbiPageWriter::new(identity);
        admitted.offer(AbiPage::try_new(identity, 0, b"credits".to_vec()).unwrap()).unwrap();
        assert_eq!(admitted.write_step(AbiWorkBudget::credits(2)), Ok(AbiCursorStep::Advanced(2)));
        let outcome = admitted.cancel();
        assert_eq!((outcome.admitted_byte_credits, outcome.copied_bytes), (7, 2));
        let handback = outcome.page.expect("admitted page handback");
        assert_eq!((handback.handle, handback.index, handback.bytes.as_slice()), (identity, 0, b"credits".as_slice()));
        assert_eq!(admitted.write_step(AbiWorkBudget::credits(5)), Err(AbiErrorCode::Cancelled));
        assert_eq!(admitted.bytes(), b"cr");
        assert_eq!(admitted.close_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::Advanced(1)));
        assert_eq!(admitted.close_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::Complete));
        assert!(admitted.terminal_is_empty());
        assert_eq!(admitted.cancel(), AbiCancelOutcome { page: None, admitted_byte_credits: 0, copied_bytes: 0 });
    }

    #[test]
    fn lost_handle_late_reply_and_duplicate_reply_cannot_cross_generations() {
        let mut handles = AbiHandleTable::new();
        let lost = handles.open(7u8).unwrap();
        assert_eq!(handles.lose(lost), Ok(7));
        assert_eq!(handles.get(lost), Err(AbiErrorCode::UnknownHandle));
        let mut ledger = AbiReplyLedger::new();
        ledger.admit(AbiRequestId(11), 3).unwrap();
        ledger.lose(AbiRequestId(11), 3).unwrap();
        assert_eq!(ledger.accept(&reply(11, 3)), Err(AbiErrorCode::LateReply));
        ledger.admit(AbiRequestId(11), 4).unwrap();
        assert_eq!(ledger.accept(&reply(11, 3)), Err(AbiErrorCode::LateReply));
        assert_eq!(ledger.accept(&reply(11, 4)), Ok(()));
        assert_eq!(ledger.accept(&reply(11, 4)), Err(AbiErrorCode::DuplicateReply));
        ledger.admit(AbiRequestId(267), 1).unwrap();
        assert_eq!(ledger.admit(AbiRequestId(523), 1), Err(AbiErrorCode::Busy));
        ledger.lose(AbiRequestId(267), 1).unwrap();
        ledger.admit(AbiRequestId(523), 1).unwrap();
        assert_eq!(ledger.accept(&reply(267, 1)), Err(AbiErrorCode::LateReply));
    }

    #[test]
    fn reader_preflights_every_rejection_before_allocation_or_copy() {
        let identity = handle(4, 2);
        let mut reader = AbiPageReader::try_new(identity, vec![1; ABI_MAX_PAGE_BYTES]).unwrap();
        let rejected = [
            (AbiWorkBudget::credits(0), AbiErrorCode::NoCredit),
            (AbiWorkBudget { cancelled: true, ..AbiWorkBudget::credits(1) }, AbiErrorCode::Cancelled),
            (AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(1) }, AbiErrorCode::Interrupted),
            (AbiWorkBudget { now_ms: 9, deadline_ms: Some(9), ..AbiWorkBudget::credits(1) }, AbiErrorCode::DeadlineExceeded),
        ];
        for (budget, code) in rejected {
            assert_eq!(reader.read_step(budget), Err(code));
            assert_eq!((reader.source_cursor, reader.target_len, reader.staging.len(), reader.staging.capacity()), (0, 0, 0, 0));
        }
        assert_eq!(reader.read_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::Advanced(1)));
        assert_eq!((reader.source_cursor, reader.target_len, reader.staging.len()), (1, ABI_MAX_PAGE_BYTES, 1));
    }

    #[test]
    fn page_and_transfer_max_plus_one_return_exact_allocations() {
        let identity = handle(1, 2);
        let mut writer = AbiPageWriter::new(identity);
        let page = AbiPage::try_new(identity, 1, vec![4]).unwrap();
        let rejected = writer.offer(page).unwrap_err();
        assert_eq!(rejected.code, AbiErrorCode::OutOfOrderPage);
        assert_eq!((rejected.handle, rejected.index, rejected.bytes), (identity, 1, vec![4]));
        let original = vec![8; ABI_MAX_TRANSFER_BYTES + 1];
        let rejected = AbiPageReader::try_new(identity, original.clone()).err().unwrap();
        assert_eq!(rejected.bytes, original);

        let mut page_count = AbiPageWriter::new(identity);
        for index in 0..ABI_MAX_PAGES_PER_TRANSFER {
            page_count.offer(AbiPage::try_new(identity, index, Vec::new()).unwrap()).unwrap();
            assert_eq!(page_count.write_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::PageComplete(index)));
        }
        let rejected = page_count.offer(AbiPage { handle: identity, index: ABI_MAX_PAGES_PER_TRANSFER, bytes: AbiPageBytes::default() }).unwrap_err();
        assert_eq!(rejected.code, AbiErrorCode::LimitExceeded);
        assert!(rejected.bytes.is_empty());
    }

    #[test]
    fn interrupted_port_callback_returns_the_exact_owned_message() {
        struct FixturePort;

        impl AbiPort for FixturePort {
            fn try_send(&mut self, message: AbiMessage, budget: AbiWorkBudget) -> Result<(), AbiPortRejection> {
                if let Err(code) = budget.permit(1) {
                    Err(AbiPortRejection { code, message })
                } else {
                    Ok(())
                }
            }

            fn poll(&mut self, budget: AbiWorkBudget) -> Result<AbiPortPoll, AbiErrorCode> {
                budget.permit(1).map(|_| AbiPortPoll::Pending)
            }
        }

        let message = AbiMessage::Request(AbiRequest { operation: operation(1), request_id: AbiRequestId(19), generation: 3, bytes: bytes(vec![1, 2, 3]) });
        let rejection = FixturePort.try_send(message.clone(), AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(1) }).unwrap_err();
        assert_eq!(rejection, AbiPortRejection { code: AbiErrorCode::Interrupted, message });
    }

    #[test]
    fn interrupted_close_retains_state_and_terminal_empty_is_idempotent() {
        let identity = handle(5, 7);
        let mut writer = AbiPageWriter::new(identity);
        writer.offer(AbiPage::try_new(identity, 0, b"close".to_vec()).unwrap()).unwrap();
        writer.write_step(AbiWorkBudget::credits(5)).unwrap();
        assert_eq!(writer.close_step(AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(5) }), Err(AbiErrorCode::Interrupted));
        assert_eq!(writer.bytes(), b"close");
        assert_eq!(writer.close_step(AbiWorkBudget::credits(2)), Ok(AbiCursorStep::Advanced(2)));
        assert_eq!(writer.close_step(AbiWorkBudget::credits(3)), Ok(AbiCursorStep::Complete));
        assert!(writer.terminal_is_empty());
        assert_eq!(writer.close_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::Complete));
        assert!(writer.terminal_is_empty());

        let mut reader = AbiPageReader::try_new(identity, b"reader".to_vec()).unwrap();
        reader.read_step(AbiWorkBudget::credits(2)).unwrap();
        assert_eq!(reader.close_step(AbiWorkBudget { cancelled: true, ..AbiWorkBudget::credits(8) }), Err(AbiErrorCode::Cancelled));
        while !reader.terminal_is_empty() {
            reader.close_step(AbiWorkBudget::credits(2)).unwrap();
        }
        assert_eq!(reader.close_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::Complete));
    }
}

//#endregion 🧪️Tests

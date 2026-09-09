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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests

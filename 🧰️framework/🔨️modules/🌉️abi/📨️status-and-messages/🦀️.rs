//! 📨️ Status and messages of the ABI: the status and error codes, the request, reply, event and page records and the control and message enums. See [`super`].

use super::*;

//#region 🧬️Schema
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
    pub(super) fn decode(code: u16) -> Result<Self, AbiErrorCode> {
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
    pub(super) fn decode(code: u16) -> Result<Self, AbiErrorCode> {
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

/// 📩️ Domain operation admission envelope.
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

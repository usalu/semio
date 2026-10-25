//! 🪪️ Limits and identifiers of the ABI: the version and size ceilings, the schema document, the operation and event codes, the request id, the handle and the byte newtypes. See [`super`].

use super::*;

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

pub const ABI_SCHEMA_JSON: &str = include_str!("../🧬️schema/🔣️.json");

/// 🔢 Valid, non-zero domain operation code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AbiOperation(pub(super) u16);

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
pub struct AbiEventCode(pub(super) u16);

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

/// 🔗️ Correlates one request/reply family without borrowing host objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AbiRequestId(pub u64);

/// ♻️ Opaque slot plus generation; equality is the exact handback identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AbiHandle {
    pub(super) slot: u32,
    pub(super) generation: u32,
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
pub struct AbiBytes(pub(super) Vec<u8>);

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
pub struct AbiPageBytes(pub(super) Vec<u8>);

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
pub struct AbiMessageBytes(pub(super) Vec<u8>);

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
//#endregion 🧬️Schema

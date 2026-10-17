//! 🧬️ Operation schema of the codec ABI: the schema document and limit constants, the operation and error codes, failure and format records and the format resolver. See [`super`].

use super::*;

//#region 🧬️Schema
pub const OS_HOST_CODEC_SCHEMA_JSON: &str = include_str!("../../🧬️schema/🔣️.json");

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

    pub(super) const fn reply_kind(self) -> u8 {
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
    pub(super) fn fixed(code: OsHostCodecErrorCode, message: &'static str) -> Self {
        Self { code, message: message.as_bytes().to_vec() }
    }

    pub(super) fn abi_code(&self) -> AbiErrorCode {
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

pub(super) trait OsHostFormatResolver {
    fn resolve_format(&mut self, kind: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure>;
}

#[derive(Default)]
pub(super) struct RegisteredOsHostFormatResolver;

impl OsHostFormatResolver for RegisteredOsHostFormatResolver {
    fn resolve_format(&mut self, kind: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure> {
        semio_framework_os_kernel::io::format_descriptor(kind)
            .map(|row| row.map(|row| OsHostCodecFormat { short_id: row.short_id, extensions: row.extensions }))
            .map_err(|_| OsHostCodecFailure::fixed(OsHostCodecErrorCode::InvalidState, "stdio format registry unavailable"))
    }
}
//#endregion 🧬️Schema

//! ❌️ JSON error: every parse failure with its byte offset, the nesting depth limit and the error conversions. See [`super`].

use super::*;

//#region 🔖️Errors
/// 🚨️ Every parse failure this crate can produce, with a byte offset into the input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonError {
    Native(ValueError),
    UnexpectedEof,
    UnexpectedByte { found: u8, offset: usize },
    InvalidNumber(usize),
    InvalidEscape(usize),
    InvalidUnicodeEscape(usize),
    UnpairedSurrogate(usize),
    ControlCharacterInString { byte: u8, offset: usize },
    InvalidUtf8,
    TrailingData(usize),
    MaxDepthExceeded(u32),
    DuplicateMember { name: String, offset: usize },
}

impl fmt::Display for JsonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native(error) => error.fmt(formatter),
            Self::UnexpectedEof => formatter.write_str("unexpected end of input"),
            Self::UnexpectedByte { found, offset } => write!(formatter, "unexpected byte {found:?} at offset {offset}"),
            Self::InvalidNumber(offset) => write!(formatter, "invalid number literal at offset {offset}"),
            Self::InvalidEscape(offset) => write!(formatter, "invalid escape sequence at offset {offset}"),
            Self::InvalidUnicodeEscape(offset) => write!(formatter, "invalid \\u escape at offset {offset}"),
            Self::UnpairedSurrogate(offset) => write!(formatter, "unpaired UTF-16 surrogate at offset {offset}"),
            Self::ControlCharacterInString { byte, offset } => write!(formatter, "control character 0x{byte:02x} in string at offset {offset}"),
            Self::InvalidUtf8 => formatter.write_str("invalid UTF-8 in input"),
            Self::TrailingData(offset) => write!(formatter, "trailing data at offset {offset}"),
            Self::MaxDepthExceeded(depth) => write!(formatter, "maximum nesting depth {depth} exceeded"),
            Self::DuplicateMember { name, offset } => write!(formatter, "duplicate member {name:?} at offset {offset}"),
        }
    }
}

impl std::error::Error for JsonError {}

impl From<ValueError> for JsonError {fn from(error:ValueError)->Self {Self::Native(error)}}

impl JsonError {
    /// 🧭️ Retains parse semantics at the owned value refusal boundary.
    pub const fn kind(&self) -> ValueRefusalKind { match self { Self::Native(error) => error.kind, Self::MaxDepthExceeded(_) => ValueRefusalKind::DepthLimit, _ => ValueRefusalKind::InvalidValue } }
    /// 🌱️ Carries parse refusal into typed native construction.
    pub fn into_value_error(self) -> ValueError { match self { Self::Native(error)=>error, error=>ValueError::new(error.kind(),error.to_string()) } }
}

impl semio_framework_value::retirement::RetireOwned for JsonError {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::*;match self{Self::Native(error)=>error.retirement(),Self::DuplicateMember{name,..}=>name.retirement(),Self::UnexpectedByte{found,offset}|Self::ControlCharacterInString{byte:found,offset}=>leaf((found,offset)),Self::InvalidNumber(offset)|Self::InvalidEscape(offset)|Self::InvalidUnicodeEscape(offset)|Self::UnpairedSurrogate(offset)|Self::TrailingData(offset)=>leaf(offset),Self::MaxDepthExceeded(depth)=>leaf(depth),Self::UnexpectedEof|Self::InvalidUtf8=>leaf(())}}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::*;match self{Self::Native(error)=>error.retirement_birth_bytes(),Self::DuplicateMember{name,..}=>name.retirement_birth_bytes(),Self::UnexpectedByte{..}|Self::ControlCharacterInString{..}=>Some(leaf_birth_bytes::<(u8,usize)>()),Self::InvalidNumber(_)|Self::InvalidEscape(_)|Self::InvalidUnicodeEscape(_)|Self::UnpairedSurrogate(_)|Self::TrailingData(_)=>Some(leaf_birth_bytes::<usize>()),Self::MaxDepthExceeded(_)=>Some(leaf_birth_bytes::<u32>()),Self::UnexpectedEof|Self::InvalidUtf8=>Some(leaf_birth_bytes::<()>())}}
    fn controlled_retirement_supported()->bool{true}
}

/// 🛡️ Recursion ceiling for nested arrays/objects — matches `serde_json`'s own default
/// (128), the value this repo's fixtures were authored against.
pub const MAX_DEPTH: u32 = 128;
//#endregion 🔖️Errors

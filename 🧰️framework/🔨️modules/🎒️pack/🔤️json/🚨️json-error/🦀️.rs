//! ❌️ JSON error: every parse failure with its byte offset, the nesting depth limit and the error conversions. See [`super`].

use super::*;

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
    pub const fn kind(&self) -> ValueRefusalKind { match self { Self::Native(error) => error.kind, Self::MaxDepthExceeded(_) => ValueRefusalKind::DepthLimit, _ => ValueRefusalKind::InvalidValue } }
    pub fn into_value_error(self) -> ValueError { match self { Self::Native(error)=>error, error=>ValueError::new(error.kind(),error.to_string()) } }
}

impl semio_framework_value::retirement::RetireOwned for JsonError {
    type RetirementCursor=<String as semio_framework_value::retirement::RetireOwned>::RetirementCursor;
    fn retirement_seed(self)->Self::RetirementCursor{use semio_framework_value::retirement::RetireOwned;match self{Self::Native(error)=>error.retirement_seed(),Self::DuplicateMember{name,..}=>name.retirement_seed(),_=>String::new().retirement_seed()}}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self::RetirementCursor>())}
    fn controlled_retirement_supported()->bool{true}
}

pub const MAX_DEPTH: u32 = 128;

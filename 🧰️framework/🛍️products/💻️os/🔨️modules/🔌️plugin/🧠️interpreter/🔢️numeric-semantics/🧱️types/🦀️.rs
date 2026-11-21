//! 🧱️ Core WebAssembly values, value types and execution errors.

use super::{canonical_f32, canonical_f64};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
    FuncRef,
    ExternRef,
}

impl ValueType {
    #[doc(hidden)]
    pub fn parse(byte: u8) -> Result<Self, CoreError> {
        match byte {
            0x7f => Ok(Self::I32),
            0x7e => Ok(Self::I64),
            0x7d => Ok(Self::F32),
            0x7c => Ok(Self::F64),
            0x70 => Ok(Self::FuncRef),
            0x6f => Ok(Self::ExternRef),
            _ => Err(CoreError::Decode(format!("unsupported value type 0x{byte:02x}"))),
        }
    }

    #[doc(hidden)]
    pub fn zero(self) -> Value {
        match self {
            Self::I32 => Value::I32(0),
            Self::I64 => Value::I64(0),
            Self::F32 => Value::F32(0),
            Self::F64 => Value::F64(0),
            Self::FuncRef => Value::FuncRef(None),
            Self::ExternRef => Value::ExternRef(None),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
    FuncRef(Option<u32>),
    ExternRef(Option<u64>),
}

impl Value {
    pub fn f32(value: f32) -> Self {
        Self::F32(canonical_f32(value).to_bits())
    }

    pub fn f64(value: f64) -> Self {
        Self::F64(canonical_f64(value).to_bits())
    }

    pub fn value_type(self) -> ValueType {
        match self {
            Self::I32(_) => ValueType::I32,
            Self::I64(_) => ValueType::I64,
            Self::F32(_) => ValueType::F32,
            Self::F64(_) => ValueType::F64,
            Self::FuncRef(_) => ValueType::FuncRef,
            Self::ExternRef(_) => ValueType::ExternRef,
        }
    }

    #[doc(hidden)]
    pub fn as_i32(self) -> Result<i32, CoreError> {
        match self {
            Self::I32(value) => Ok(value),
            _ => Err(CoreError::Trap("expected i32".into())),
        }
    }

    #[doc(hidden)]
    pub fn as_i64(self) -> Result<i64, CoreError> {
        match self {
            Self::I64(value) => Ok(value),
            _ => Err(CoreError::Trap("expected i64".into())),
        }
    }

    #[doc(hidden)]
    pub fn as_f32(self) -> Result<f32, CoreError> {
        match self {
            Self::F32(value) => Ok(f32::from_bits(value)),
            _ => Err(CoreError::Trap("expected f32".into())),
        }
    }

    #[doc(hidden)]
    pub fn as_f64(self) -> Result<f64, CoreError> {
        match self {
            Self::F64(value) => Ok(f64::from_bits(value)),
            _ => Err(CoreError::Trap("expected f64".into())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoreError {
    Decode(String),
    Validation(String),
    Trap(String),
    State(String),
    Host(String),
}

impl Display for CoreError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode(message) => write!(formatter, "wasm decode: {message}"),
            Self::Validation(message) => write!(formatter, "wasm validation: {message}"),
            Self::Trap(message) => write!(formatter, "wasm trap: {message}"),
            Self::State(message) => write!(formatter, "wasm state: {message}"),
            Self::Host(message) => write!(formatter, "wasm host: {message}"),
        }
    }
}

impl std::error::Error for CoreError {}

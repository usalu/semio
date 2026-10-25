//! 🚨️ Protocol error identity and paid cause retirement below wire codecs.

#[cfg(test)]
#[global_allocator]
static HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

/// ❌️ The one error type every `protocol_*` public fn returns; never leaks `std::io::Error`.
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolError {
    Pack(semio_framework_pack_error::PackError),
    ChainMismatch { commit_seq: u64 },
    TornTail(u64),
    UnknownCriticalRecord(u8),
    DictMiss(u32),
    DictOutOfOrder { expected: u32, actual: u32 },
    VerifierRequired,
    SignatureInvalid { commit_seq: u64 },
    FrameFraming(u64),
    LimitExceeded(&'static str),
    Malformed { what: &'static str, offset: u64, detail: String },
    Io(String),
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pack(error) => error.fmt(formatter),
            Self::ChainMismatch { commit_seq } => write!(formatter, "chain mismatch at commit {commit_seq}"),
            Self::TornTail(offset) => write!(formatter, "torn tail at offset {offset}"),
            Self::UnknownCriticalRecord(kind) => write!(formatter, "unknown critical record kind {kind:#x}"),
            Self::DictMiss(index) => write!(formatter, "dictionary index out of range: {index}"),
            Self::DictOutOfOrder { expected, actual } => write!(formatter, "dictionary out of order: expected base_count {expected}, got {actual}"),
            Self::VerifierRequired => formatter.write_str("signature verification required but no verifier supplied"),
            Self::SignatureInvalid { commit_seq } => write!(formatter, "signature invalid for commit {commit_seq}"),
            Self::FrameFraming(offset) => write!(formatter, "frame back_len mismatch at offset {offset}"),
            Self::LimitExceeded(limit) => write!(formatter, "limit exceeded: {limit}"),
            Self::Malformed { what, offset, detail } => write!(formatter, "malformed {what} at offset {offset}: {detail}"),
            Self::Io(message) => write!(formatter, "io error: {message}"),
        }
    }
}

impl std::error::Error for ProtocolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Pack(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_framework_pack_error::PackError> for ProtocolError {
    fn from(error: semio_framework_pack_error::PackError) -> Self {
        Self::Pack(error)
    }
}

impl From<semio_framework_pack_error::PackRefusal> for ProtocolError {
    fn from(error: semio_framework_pack_error::PackRefusal) -> Self {
        Self::Pack(semio_framework_pack_error::PackError::Refusal(error))
    }
}

semio_framework_diagnostic::fault_from_error!(ProtocolError, semio_framework_diagnostic::FaultOrigin::Module, "module.protocol");


#[path="♻️retirement/🦀️.rs"]
mod cause_retirement;
pub use cause_retirement::{protocol_cause_retirement_demand,protocol_error_retirement_demand,close_protocol_cause_one,close_protocol_error_one};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

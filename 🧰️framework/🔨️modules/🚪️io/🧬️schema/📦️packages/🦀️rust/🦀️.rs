//! 🚪️ Product-neutral I/O vocabulary with one definition owner.
#[path = "../../🦀️.rs"]
mod vocabulary;
pub use vocabulary::{
    IoPayload,
    CARRIER_BINARY,
    CARRIER_TEXT,
    SQLITE_SNAPSHOT,
    Confidence,
    IoFidelity,
    IoError,
    IoOutcome,
    IoResult,
    IoEntryDescriptor,
    IoRoute,
    register_io_schema_exports,
};

#[cfg(test)]
#[path="../../🏛️ownership/🧪️tests/🦀️.rs"]
mod ownership_tests;

#[cfg(test)]
#[path = "../../⚠️refusal/🧪️tests/🦀️.rs"]
mod refusal_tests;




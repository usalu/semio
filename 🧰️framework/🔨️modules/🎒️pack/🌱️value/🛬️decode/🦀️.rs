//! 🪝️ Retained record decoding: the value and record-body cursors that admit a wire body one granted step at a time, plus source decode, receiving and the stack storage they own.

use semio_framework_value::{DslValue, Number, ValueError, ValueRefusalKind};
use semio_framework_pack_error::PackRefusal;
use semio_framework_dsl_record::{FieldValue, RecordValue, WireNode};
use semio_framework_replication_codec::PackLimits;
use semio_framework_pack_record_schema::{ELEM_BOOL, ELEM_ENUM, ELEM_F64, ELEM_FALLBACK, ELEM_INT, ELEM_STR, ELEM_UINT, TAG_ABSENT, TAG_BLOCK, TAG_BYTES, TAG_BYTES_CHUNKED, TAG_ENUM, TAG_EXPR, TAG_F64, TAG_FALSE, TAG_INT, TAG_LIST, TAG_MAP, TAG_NULL, TAG_PACKED_F64, TAG_PACKED_VARINT, TAG_RECORD, TAG_STATEMENTS, TAG_STR, TAG_STR_INLINE, TAG_TABLE_SOA, TAG_TRUE, TAG_TUPLE, TAG_UINT, TAG_VALUE, TAG_WIRE};

#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

#[path = "📖️source/🦀️.rs"]
mod original_record_source;
pub use original_record_source::{RetainedRecordBodySourceDecode,RetainedRecordBodyReadLimits,RetainedRecordBodySourceStep};

#[path="🎮️driver/🦀️.rs"]
mod driver;
pub use driver::{RetainedValueDriver,retained_pack_close_progress};
#[path="📦️document/🦀️.rs"]
mod document;
pub use document::{RetainedDocumentDriver,INTRINSIC_SCHEMA_GRAPH};

#[path = "🫴️receiving/🦀️.rs"]
mod original_record_receiving;
pub use original_record_receiving::RetainedRecordReceiving;

#[path = "../🎟️storage/🦀️.rs"]
mod original_storage;

#[path = "🧩️value-tokens-and-parsers/🦀️.rs"]
mod value_tokens_and_parsers;
pub use value_tokens_and_parsers::*;

#[path = "🧭️value-cursor/🦀️.rs"]
mod value_cursor;
pub use value_cursor::*;

#[path = "📜️record-body-cursor/🦀️.rs"]
mod record_body_cursor;
pub use record_body_cursor::*;

#[cfg(test)]
#[path = "../🎟️storage/🧪️tests/🦀️.rs"]
mod original_storage_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

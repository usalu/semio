//! 📡️ Host codec ABI: operation schema, error codes, codec session, retained and direct codec services.
//#region 🧬️Schema
use semio_framework::{AbiBytes, AbiControl, AbiCursorStep, AbiError, AbiErrorCode, AbiEvent, AbiEventCode, AbiHandle, AbiHandleTable, AbiMessageBytes, AbiOperation, AbiPage, AbiPageReader, AbiRejectedPage, AbiReply, AbiRequest, AbiRequestId, AbiStatus, AbiStatusCode, AbiWorkBudget, ABI_MAX_BODY_BYTES, ABI_MAX_MESSAGE_BYTES, ABI_MAX_PAGES_PER_TRANSFER, ABI_MAX_PAGE_BYTES};

#[path = "🧬️operation-schema/🦀️.rs"]
mod operation_schema;
pub use operation_schema::*;

#[path = "📊️progress-records/🦀️.rs"]
mod progress_records;
pub use progress_records::*;

#[path = "🔤️utf8-cursor/🦀️.rs"]
mod utf8_cursor;
use utf8_cursor::*;

#[path = "🔎️filter-kinds-cursor/🦀️.rs"]
mod filter_kinds_cursor;
use filter_kinds_cursor::*;

#[path = "🪪️normalize-kind-cursor/🦀️.rs"]
mod normalize_kind_cursor;
use normalize_kind_cursor::*;

#[path = "🎟️codec-session/🦀️.rs"]
mod codec_session;
use codec_session::*;

#[path = "🛰️retained-service/🦀️.rs"]
mod retained_service;
use retained_service::*;

#[path = "🚪️direct-service/🦀️.rs"]
mod direct_service;
pub use direct_service::*;

//#endregion 🧬️Schema

//#region 🧪️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️codec-abi-unit/🦀️.rs");
//#endregion 🧪️Tests

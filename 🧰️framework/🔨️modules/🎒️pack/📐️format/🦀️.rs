//! 📦️ `pack_format` — the `SPK` binary document container: exact byte-level header/footer/
//! segment/manifest/symbols/chunk-table layout, a `PackWriter`/`PackFile` pair for building and
//! random-access reading packs of any size, three verification levels trading speed for
//! integrity guarantees, forward-scan recovery for footer-less/truncated files, and an optional
//! deflate codec. Every length is validated against `crate::PackLimits` before allocation.
//!
//! Layout notes: see the `## pack_format` section of the wave-0 contract at
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/contract.md`. One deliberate
//! deviation is documented in `🔖️Footer` below: the contract's prose arithmetic for footer size
//! (`"= 80 bytes exactly"`) undercounts its own `footer_crc32` field by 4 bytes; this crate
//! implements the mathematically self-consistent 84-byte footer and exports `FOOTER_SIZE` so
//! downstream crates never have to hardcode the number themselves.

use semio_framework_pack_error::PackRefusal;
use semio_framework_replication_codec::{crc32c, read_varint_u64, write_varint_u64, ByteRange, ChunkId, CodecId, CompressionCodec, ContentHash, NoCompression, PackLimits};
use semio_framework_pack_source::{PackSink, PackSource};
use semio_framework_value::{ValueRefusalKind, ValueError};
use std::mem::size_of;
use semio_framework_value::native_decoding::NativeDecodeControl;

#[path="🛫️encoding/🦀️.rs"]
mod controlled_encoding;
pub use controlled_encoding::ControlledPackWriter;

#[path = "🧮️controlled-allocation/🦀️.rs"]
mod controlled_allocation;
use controlled_allocation::*;

#[path = "🧱️header-and-footer/🦀️.rs"]
mod header_and_footer;
pub use header_and_footer::*;

#[path = "🧩️segments-and-symbols/🦀️.rs"]
mod segments_and_symbols;
pub use segments_and_symbols::*;

#[path = "📜️chunk-table-and-manifest/🦀️.rs"]
mod chunk_table_and_manifest;
pub use chunk_table_and_manifest::*;

#[path = "✍️pack-writer/🦀️.rs"]
mod pack_writer;
pub use pack_writer::*;

#[path = "📖️pack-reader-and-recovery/🦀️.rs"]
mod pack_reader_and_recovery;
pub use pack_reader_and_recovery::*;

//#region 🔖️Verify
/// 🛡️ The exact shared Pack verification policy controls CRC and content hash checks.
pub use semio_framework_replication_codec::PackVerificationLevel as VerificationLevel;
//#endregion 🔖️Verify

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔣️absolute-varint/🦀️.rs"]
mod absolute_varint_tests;

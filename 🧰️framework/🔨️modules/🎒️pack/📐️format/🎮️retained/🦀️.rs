//! 🪢️ Retained canonical Pack source admission: bounded, cancellable byte-at-a-time cursors for pages, segments, varints and the pack anchors.
//! The framing, writer and reader live in `semio-framework-pack-format`; the catalog cursors live in `semio-framework-pack-format-catalog`.

use semio_framework_pack_error::PackRefusal;
use semio_framework_replication_codec::{ContentHash, PackLimits};
use semio_framework_value::{ValueRefusalKind, RetirementDemand, ValueError};
use semio_framework_value_persistent::{list::PagedListError, list::PagedListAllocationError, list::PagedListRefusalKind};
use std::mem::size_of;
use semio_framework_pack_format::{FOOTER_MAGIC, FOOTER_SIZE, FORMAT_VERSION_MAJOR, Footer, HEADER_SIZE, Header, MAGIC, REQUIRED_KNOWN_MASK, Superblock};
#[cfg(test)]
use semio_framework_replication_codec::{ByteRange, CodecId};
#[cfg(test)]
use semio_framework_pack_source::PackSink;

#[path = "📄️pages-and-source-state/🦀️.rs"]
mod pages_and_source_state;
pub use pages_and_source_state::*;

#[path = "🚰️source-cursor/🦀️.rs"]
mod source_cursor;

#[path = "🧱️segment-and-varint-cursors/🦀️.rs"]
mod segment_and_varint_cursors;
pub use segment_and_varint_cursors::*;

#[path = "⚓️catalog-and-anchor-cursors/🦀️.rs"]
mod catalog_and_anchor_cursors;
pub use catalog_and_anchor_cursors::*;

#[cfg(test)]
#[path = "../🧪️tests/🔬️retained-pack-source-laws/🦀️.rs"]
mod retained_pack_source_laws;

#[cfg(test)]
#[path = "../🧭️producer/📥️source/🧪️tests/🦀️.rs"]
mod source_producer_authority_tests;

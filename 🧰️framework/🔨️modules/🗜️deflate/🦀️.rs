//! 🗜️ First-party raw DEFLATE (RFC 1951) codec with owned Value refusal identities.
//! `miniz_oxide` is the differential test oracle in the `🧪️Oracle` region.
//! `inflate`/`Inflater` decompress anything `miniz_oxide::deflate::compress_to_vec` already
//! produced (persisted `.spk`/`.spr` payloads must keep decoding). `compress` is a real LZ77 match
//! finder over a single fixed-Huffman block (`BTYPE=1`) — not a stored-block fallback — so segment
//! payloads still shrink. Only raw DEFLATE is implemented (no zlib/RFC 1950 wrapper): every call
//! site (`📡️replication/⚙️codec`) already used `miniz_oxide::DataFormat::Raw`. See
//! <https://www.rfc-editor.org/rfc/rfc1951>.

use semio_framework_value::{ValueError,ValueRefusalKind};

#[path = "📋️errors-and-tables/🦀️.rs"]
mod errors_and_tables;
pub use errors_and_tables::*;

#[path = "🔁️bit-io-and-huffman/🦀️.rs"]
mod bit_io_and_huffman;
use bit_io_and_huffman::*;

#[path = "🧱️inflate-state/🦀️.rs"]
mod inflate_state;
pub use inflate_state::*;

#[path = "🚰️inflater/🦀️.rs"]
mod inflater;
pub use inflater::*;

#[path = "📤️deflate-encoder/🦀️.rs"]
mod deflate_encoder;
pub use deflate_encoder::*;

//#region 🔖️Deflate

#[path="🫳️measure/🦀️.rs"]
mod measurement;
pub use measurement::DeflateMeasure;
#[path = "🛫️encode/⏳️cursor/🦀️.rs"]
mod retained_encoding;
pub use retained_encoding::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, RetainedDeflateEncoder, RetainedDeflatePhase, RetainedDeflateProgress};
//#endregion 🔖️Deflate

#[path = "🎒️zip/🦀️.rs"]
pub mod zip_archive;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🛫️encode/⏳️cursor/🧪️tests/🦀️.rs"]
mod retained_encoder_tests;

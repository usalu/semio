//! 🚪️ IO stdio.png (1.2/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via `crate::register`), not per-leaf
//! register(). Relocated from `⚙️engine` verbatim (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 2: codecs live in `🚪️io/`).
//!
//! 🖼️ Native PNG syntax admits precise owned image records; canonical publication owns filtering, compression, framing and CRCs.
/// 🧩 Borrowed PNG chunk type tag and payload.
type PngChunkView<'a> = ([u8; 4], &'a [u8]);

use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};
use semio_s_artifact_stdio_png_snapshot::standards::v1_2::subsets::any::schema::snapshot::{PngAncillaryChunk, PngImage, PngBackground, PngChromaticities, PngColorType, PngPhysicalDims, PngRgb, PngSrgbIntent, PngTextChunk, PngTextKind, PngTimestamp, PngTransparency, PngSnapshot};

#[path = "🧾️records-and-chunk-io/🦀️.rs"]
mod records_and_chunk_io;
pub use records_and_chunk_io::*;

#[path = "✅️header-and-structure/🦀️.rs"]
mod header_and_structure;
pub use header_and_structure::*;

#[path = "🎞️filters-and-adam7/🦀️.rs"]
mod filters_and_adam7;
pub use filters_and_adam7::*;

#[path = "🖼️unpack-and-ancillary/🦀️.rs"]
mod unpack_and_ancillary;
use unpack_and_ancillary::*;

#[path = "🧭️layout-and-projection/🦀️.rs"]
mod layout_and_projection;
pub use layout_and_projection::*;

#[path = "✍️native-encode/🦀️.rs"]
mod native_encode;
pub use native_encode::*;

#[path = "🔓️native-decode/🦀️.rs"]
mod native_decode;
use native_decode::*;



//#region 🧪️CodecTests

//#endregion 🧪️CodecTests

#[path="🚦️native/🦀️.rs"]
pub mod native;

pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::io::sqlite;





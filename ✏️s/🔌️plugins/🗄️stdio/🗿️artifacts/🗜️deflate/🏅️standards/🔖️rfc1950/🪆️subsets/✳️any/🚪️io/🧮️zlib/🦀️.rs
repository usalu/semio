//! 🧮️ Pure RFC1950/RFC1951 byte codec of `stdio.deflate` -- Adler-32, bit IO, Huffman, LZ77, inflate and the streaming and tuned encoders. Byte to byte only: it knows no typed snapshot, so the snapshot codecs of the typed artifact sit above it.

//#region 🦑️DissolvedEngineCodec
// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
// MACHINES). Pure format algorithms — Adler32, bit IO, Huffman, LZ77 (~1,080 LOC) — kept HERE
// with the codec that is their only caller, per that ticket's rule 6 ("deflate's Huffman/LZ77 is
// the clearest case"): they have no `DeflateSnapshot` dependency of their own, but they are not
// genuinely artifact-independent either (no other stdio artifact shares this exact bitstream), so
// a module-engine one level up would be a distinction without a difference. `zlib_compress`/
// `zlib_decompress` stay byte<->byte (load-bearing for other artifacts' own internal zlib framing
// — PNG IDAT, PDF stream objects — which have never gone through a `DeflateSnapshot` and must not
// start doing so here); `encode_deflate_snapshot`/`decode_deflate_snapshot` are the RFC1950
// container<->typed-snapshot pair `DeflateSnapshot`'s `ArtifactDsl`/`ArtifactPack` impls call.
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};

#[path = "🔢️adler-bit-io-and-huffman/🦀️.rs"]
mod adler_bit_io_and_huffman;
pub use adler_bit_io_and_huffman::*;

#[path = "🔎️lz77-matching/🦀️.rs"]
mod lz77_matching;
use lz77_matching::*;
pub use lz77_matching::{HASH_SIZE, WINDOW};

#[path = "🧵️deflate-encode-job/🦀️.rs"]
mod deflate_encode_job;
pub use deflate_encode_job::*;

#[path = "📥️retained-decoder/🦀️.rs"]
mod retained_decoder;
pub use retained_decoder::*;

#[path = "📤️retained-encoder/🦀️.rs"]
mod retained_encoder;
pub use retained_encoder::*;

#[path = "🚚️publication-and-job-steps/🦀️.rs"]
mod publication_and_job_steps;
use publication_and_job_steps::*;

#[path = "🔐️flat-close-and-checkpoint/🦀️.rs"]
mod flat_close_and_checkpoint;
pub use flat_close_and_checkpoint::*;

#[path = "🎚️tuned-encode/🦀️.rs"]
mod tuned_encode;
pub use tuned_encode::*;

#[path = "🔓️inflate-and-zlib-frame/🦀️.rs"]
mod inflate_and_zlib_frame;
pub use inflate_and_zlib_frame::*;

#[path = "../🗜️dynamic/🦀️.rs"]
mod dynamic;

//#region DeflateCodec

//#region 🧵️StreamingEncode

pub use semio_s_artifact_stdio_deflate_examples::standards::v_rfc1950::subsets::any::io::zlib::publication;
use publication::{Kind as PublicationKind, Publication};
#[cfg(test)] use publication::{close_result};

//#endregion 🧵️StreamingEncode

//#endregion DeflateCodec

//#region 🧪️CodecTests

//#endregion 🧪️CodecTests
//#endregion 🦑️DissolvedEngineCodec

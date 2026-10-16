//! 🎒️ os pack facade. The `.spk` container (header, footer, segments, manifest, chunk table, recovery,
//! sources) and the schema-driven record codec (`pack::record`) live in the product-neutral
//! `🧰️framework/🔨️modules/🎒️pack` crates; what stays here is the pack runtime bridge, the stored-hash
//! read, the command line and the arbitrary/law testkit.

pub use pack::*;

//#region 🔖️Bridge
#[path = "🌉️bridge/🦀️.rs"]
pub mod pack_rt;
//#endregion 🔖️Bridge

//#region 🔖️StoredHash
/// #⃣ Reads only the trailing footer of an encoded pack file and returns its stored
/// `content_hash` — no header/manifest/document decode needed. Not `pack::content_hash`, which hashes
/// the bytes it is given.
pub fn stored_content_hash(bytes: &[u8]) -> Result<ContentHash, PackError> {
    ::semio_framework_async::poll::resolve_ready(read_footer_only(&bytes)).map(|footer| footer.content_hash).map_err(PackError::from)
}
//#endregion 🔖️StoredHash

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(any(test, feature = "record-codec-laws"))]
#[path = "🧪️tests/🧬️record-codec-laws/🦀️.rs"]
pub mod record_codec_laws;

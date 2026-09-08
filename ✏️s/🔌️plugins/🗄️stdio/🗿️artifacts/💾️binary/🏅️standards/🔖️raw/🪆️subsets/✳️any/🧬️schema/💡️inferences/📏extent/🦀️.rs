//! 📏 `extent` — one named inference: the ONLY honest census an opaque `bytes: Vec<u8>` blob
//! supports — its real byte length, whether it is empty, and a real content digest. `binary/raw`
//! is deliberately the repo's minimal-structure artifact (see `📸️snapshot/🦀️.rs`'s own
//! single-field shape); it has no header, no chunk table, no entry list, so this facet does NOT
//! fabricate one — it reports exactly what the bytes themselves honestly are.

use crate::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//#region 🔖️Extent
/// 📏️ binary's real extent over its opaque `bytes` blob.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BinaryExtent {
    pub byte_length: u64,
    pub is_empty: bool,
    pub content_digest: String,
}

/// 📏️ Computes [`BinaryExtent`] — `byteLength`/`isEmpty` read `bytes.len()` directly;
/// `contentDigest` folds `bytes` through `std`'s own `DefaultHasher` (same std-only reasoning
/// `🎒️zip/🗃entries` and `🗜️deflate/🪟window` already established for a single scalar digest).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_binary_extent(snapshot: &BinarySnapshot) -> BinaryExtent {
    let mut hasher = DefaultHasher::new();
    snapshot.bytes.hash(&mut hasher);
    BinaryExtent { byte_length: snapshot.bytes.len() as u64, is_empty: snapshot.bytes.is_empty(), content_digest: format!("{:016x}", hasher.finish()) }
}

impl Default for BinaryExtent {
    fn default() -> Self {
        compute_binary_extent(&BinarySnapshot::default())
    }
}
//#endregion 🔖️Extent

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//! 📏 `extent` — one named inference: the ONLY honest census an opaque `bytes: Vec<u8>` blob
//! supports — its real byte length, whether it is empty, and a real content digest. `binary/raw`
//! is deliberately the repo's minimal-structure artifact (see `📸️snapshot/🦀️component.rs`'s own
//! single-field shape); it has no header, no chunk table, no entry list, so this facet does NOT
//! fabricate one — it reports exactly what the bytes themselves honestly are.

use crate::artifacts::binary::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//#region 🔖️Extent
/// 📏️ binary's real extent over its opaque `bytes` blob.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryExtent {
    pub byte_length: u64,
    pub is_empty: bool,
    pub content_digest: String,
}

/// 📏️ Computes [`BinaryExtent`] — `byteLength`/`isEmpty` read `bytes.len()` directly;
/// `contentDigest` folds `bytes` through `std`'s own `DefaultHasher` (same std-only reasoning
/// `🎒️zip/🗃entries` and `🗜️deflate/🪟window` already established for a single scalar digest).
pub fn compute_binary_extent(snapshot: &BinarySnapshot) -> BinaryExtent {
    let mut hasher = DefaultHasher::new();
    snapshot.bytes.hash(&mut hasher);
    BinaryExtent { byte_length: snapshot.bytes.len() as u64, is_empty: snapshot.bytes.is_empty(), content_digest: format!("{:016x}", hasher.finish()) }
}
//#endregion 🔖️Extent

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[test]
    fn real_bytes_yield_a_real_nonzero_extent() {
        let snapshot = BinarySnapshot { bytes: vec![1, 2, 3, 4, 5], ..BinarySnapshot::default() };
        let extent = compute_binary_extent(&snapshot);
        assert_eq!(extent.byte_length, 5);
        assert!(!extent.is_empty);
    }

    #[test]
    fn empty_bytes_yield_an_honest_empty_extent() {
        let extent = compute_binary_extent(&BinarySnapshot::default());
        assert_eq!(extent.byte_length, 0);
        assert!(extent.is_empty);
    }

    #[test]
    fn different_bytes_yield_different_digests() {
        let a = BinarySnapshot { bytes: vec![1, 2, 3], ..BinarySnapshot::default() };
        let b = BinarySnapshot { bytes: vec![9, 9, 9], ..BinarySnapshot::default() };
        assert_ne!(compute_binary_extent(&a).content_digest, compute_binary_extent(&b).content_digest);
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = BinarySnapshot { bytes: vec![7, 7, 7], ..BinarySnapshot::default() };
        assert_eq!(compute_binary_extent(&snapshot), compute_binary_extent(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_binary_extent(&BinarySnapshot::default()), BinaryExtent::default());
    }
}
//#endregion 🧪️Tests

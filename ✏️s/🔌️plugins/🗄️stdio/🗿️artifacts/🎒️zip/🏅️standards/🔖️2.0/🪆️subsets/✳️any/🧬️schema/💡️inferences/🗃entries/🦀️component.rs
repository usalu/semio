//! 🗃 `entries` — one named inference: a real central-directory-style census over the archive's
//! decompressed `entries` (real entry count, real total decompressed size, a deterministic
//! content digest over every entry's name+bytes in archive order). ZIP already keeps a real
//! central directory for exactly this kind of summary — this facet is the honest in-memory
//! equivalent over the already-decoded `ZipSnapshot`, not a re-parse of the wire format.

use crate::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::ZipSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//#region 🔖️Entries
/// 🗃️ Real central-directory-style census over `entries`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipEntries {
    pub entry_count: u32,
    pub total_uncompressed_size: u64,
    pub content_digest: String,
}

/// 🗃️ `entryCount` = `entries.len()`; `totalUncompressedSize` = sum of every entry's real
/// decompressed `data.len()` (`ZipEntry::data` is always the decompressed payload — see the
/// snapshot's own doc comment — so this is a genuine uncompressed-bytes total, not a guess from
/// compressed sizes); `contentDigest` folds every entry's `(name, data)` pair, in archive order,
/// through `std`'s own `DefaultHasher` (same std-only, no-external-crate reasoning
/// `🏠️home/🆔digest` and `🔋️model/🗃entries` already established for a single scalar digest).
pub async fn compute_zip_entries(snapshot: &ZipSnapshot) -> ZipEntries {
    let mut hasher = DefaultHasher::new();
    let mut total_uncompressed_size: u64 = 0;
    for entry in &snapshot.entries {
        entry.name.hash(&mut hasher);
        entry.data.hash(&mut hasher);
        total_uncompressed_size += entry.data.len() as u64;
    }
    ZipEntries { entry_count: snapshot.entries.len() as u32, total_uncompressed_size, content_digest: format!("{:016x}", hasher.finish()) }
}

impl Default for ZipEntries {
    async fn default() -> Self {
        compute_zip_entries(&ZipSnapshot::default())
    }
}
//#endregion 🔖️Entries

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::ZipEntry;

    async fn entry(name: &str, data: &[u8]) -> ZipEntry {
        ZipEntry { name: name.into(), data: data.to_vec(), ..ZipEntry::default() }
    }

    #[test]
    async fn real_entries_are_counted_and_sized_exactly() {
        let snapshot = ZipSnapshot { entries: vec![entry("a.txt", b"hello"), entry("b.txt", b"world!")], ..ZipSnapshot::default() };
        let entries = compute_zip_entries(&snapshot);
        assert_eq!(entries.entry_count, 2);
        assert_eq!(entries.total_uncompressed_size, 5 + 6);
    }

    #[test]
    async fn empty_archive_yields_a_real_zero_census() {
        let entries = compute_zip_entries(&ZipSnapshot::default());
        assert_eq!(entries.entry_count, 0);
        assert_eq!(entries.total_uncompressed_size, 0);
    }

    #[test]
    async fn different_content_yields_a_different_digest() {
        let a = ZipSnapshot { entries: vec![entry("a.txt", b"hello")], ..ZipSnapshot::default() };
        let b = ZipSnapshot { entries: vec![entry("a.txt", b"goodbye")], ..ZipSnapshot::default() };
        assert_ne!(compute_zip_entries(&a).content_digest, compute_zip_entries(&b).content_digest);
    }

    #[test]
    async fn inference_determinism_law() {
        let snapshot = ZipSnapshot { entries: vec![entry("a.txt", b"hello")], ..ZipSnapshot::default() };
        assert_eq!(compute_zip_entries(&snapshot), compute_zip_entries(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(compute_zip_entries(&ZipSnapshot::default()), ZipEntries::default());
    }
}
//#endregion 🧪️Tests

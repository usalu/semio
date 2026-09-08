//! 🗃 `entries` — one named inference: a real central-directory-style census over the archive's
//! decompressed `entries` (real entry count, real total decompressed size, a deterministic
//! content digest over every entry's name+bytes in archive order). ZIP already keeps a real
//! central directory for exactly this kind of summary — this facet is the honest in-memory
//! equivalent over the already-decoded `ZipSnapshot`, not a re-parse of the wire format.

use crate::standards::v2_0::subsets::base::schema::snapshot::ZipSnapshot;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//#region 🔖️Entries
/// 🗃️ Real central-directory-style census over `entries`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
/// `🏠️home/🆔digest` and `🔋️model/🗃️entries` already established for a single scalar digest).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_zip_entries(snapshot: &ZipSnapshot) -> ZipEntries {
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
    fn default() -> Self {
        compute_zip_entries(&ZipSnapshot::default())
    }
}
//#endregion 🔖️Entries

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

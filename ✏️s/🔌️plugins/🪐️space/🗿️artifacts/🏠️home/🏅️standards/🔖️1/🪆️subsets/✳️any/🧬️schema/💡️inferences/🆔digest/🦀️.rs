//! 🆔 `digest` — one named inference: a deterministic content fingerprint of the persisted S Home
//! snapshot (`schema` + `catalogGeneration`). Whole-snapshot scalar, not per-entity, so this leaf
//! holds a plain pure function rather than an `InferredField` chain — the family root's
//! `impl protocol::Inference<SHomeSnapshot>` calls it directly.

use crate::SHomeSnapshot;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//#region 🔖️ContentDigest
/// 🆔️ Deterministic (within a process) fingerprint over every persistent field — changes iff
/// `schema` or `catalogGeneration` changes. Std-only (`DefaultHasher`, no external hash crate):
/// this document is two scalars, so a full merkle/blake3 chain (as `puzzle3d`'s `flat-position`
/// uses for its per-entity graph) would be pure overhead here.
pub fn compute_content_digest(snapshot: &SHomeSnapshot) -> String {
    let mut hasher = DefaultHasher::new();
    snapshot.schema.hash(&mut hasher);
    snapshot.catalog_generation.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}
//#endregion 🔖️ContentDigest

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

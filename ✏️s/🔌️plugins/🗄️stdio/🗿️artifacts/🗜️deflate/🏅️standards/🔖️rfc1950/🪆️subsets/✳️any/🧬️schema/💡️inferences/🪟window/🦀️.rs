//! 🪟 `window` — one named inference: real RFC1950 zlib header semantics read straight off the
//! snapshot's own typed CMF/FLG fields — `windowSize = 2^(windowBits+8)` (RFC1950 §2.2's own
//! CINFO-to-window-size formula, valid for `windowBits` `0..=7`; anything outside that range is
//! spec-reserved and honestly reported as `0`, never a fabricated size), `compressionLevelHint`
//! echoes the real FLG.FLEVEL-decoded hint, `hasPresetDictionary` reads FDICT via `dictId`'s
//! presence, plus a real byte-size + content digest over `payload`. A pure whole-snapshot scalar
//! read/fold — no `InferredField` needed.

use crate::standards::v_rfc1950::subsets::any::schema::snapshot::DeflateSnapshot;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//#region 🔖️Window
/// 🪟️ deflate's real RFC1950 zlib-header-derived facet.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DeflateWindow {
    pub window_size: u32,
    pub compression_level_hint: String,
    pub has_preset_dictionary: bool,
    pub payload_size: u64,
    pub content_digest: String,
}

/// 🌱 Hand-rolled (not derived) — `DeflateSnapshot::default()`'s `window_bits: 7` is RFC1950's own
/// real maximum-window normal form (a derived all-zero `Default` would disagree with the honest
/// compute below and break `inference_default_law`, the same class of trap the family-root
/// `DeflateInference::default()` already hand-rolls one level up).
impl Default for DeflateWindow {
    fn default() -> Self {
        compute_deflate_window(&DeflateSnapshot::default())
    }
}

/// 🪟️ Computes [`DeflateWindow`] per RFC1950 §2.2: CINFO (here `window_bits`) `0..=7` maps to a
/// real `2^(CINFO+8)` byte window (up to the format's 32KB ceiling at `7`); `8..=15` is spec-
/// reserved and honestly reported as `0` (matching the codec's own tolerant-but-honest treatment
/// of reserved header bits elsewhere in this snapshot). `contentDigest` folds `payload` through
/// `std`'s own `DefaultHasher` (same std-only reasoning `🎒️zip/🗃entries` already established).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_deflate_window(snapshot: &DeflateSnapshot) -> DeflateWindow {
    let window_size = if snapshot.window_bits <= 7 { 1u32 << (snapshot.window_bits as u32 + 8) } else { 0 };
    let mut hasher = DefaultHasher::new();
    snapshot.payload.hash(&mut hasher);
    DeflateWindow {
        window_size,
        compression_level_hint: format!("{:?}", snapshot.compression_level_hint),
        has_preset_dictionary: snapshot.dict_id.is_some(),
        payload_size: snapshot.payload.len() as u64,
        content_digest: format!("{:016x}", hasher.finish()),
    }
}
//#endregion 🔖️Window

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

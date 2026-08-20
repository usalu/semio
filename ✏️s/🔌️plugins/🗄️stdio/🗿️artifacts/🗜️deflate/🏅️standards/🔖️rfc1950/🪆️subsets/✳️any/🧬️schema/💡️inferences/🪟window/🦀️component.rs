//! 🪟 `window` — one named inference: real RFC1950 zlib header semantics read straight off the
//! snapshot's own typed CMF/FLG fields — `windowSize = 2^(windowBits+8)` (RFC1950 §2.2's own
//! CINFO-to-window-size formula, valid for `windowBits` `0..=7`; anything outside that range is
//! spec-reserved and honestly reported as `0`, never a fabricated size), `compressionLevelHint`
//! echoes the real FLG.FLEVEL-decoded hint, `hasPresetDictionary` reads FDICT via `dictId`'s
//! presence, plus a real byte-size + content digest over `payload`. A pure whole-snapshot scalar
//! read/fold — no `InferredField` needed.

use crate::artifacts::deflate::standards::v_rfc1950::subsets::any::schema::snapshot::DeflateSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//#region 🔖️Window
/// 🪟️ deflate's real RFC1950 zlib-header-derived facet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
mod tests {
    use super::*;
    use crate::artifacts::deflate::standards::v_rfc1950::subsets::any::schema::snapshot::DeflateLevelHint;

    #[semio_framework_async_macros::async_test]
    async fn window_bits_7_yields_the_real_32kb_rfc1950_ceiling() {
        let snapshot = DeflateSnapshot { window_bits: 7, ..DeflateSnapshot::default() };
        assert_eq!(compute_deflate_window(&snapshot).window_size, 32_768);
    }

    #[semio_framework_async_macros::async_test]
    async fn a_reserved_cinfo_value_is_honestly_reported_as_zero_not_fabricated() {
        let snapshot = DeflateSnapshot { window_bits: 15, ..DeflateSnapshot::default() };
        assert_eq!(compute_deflate_window(&snapshot).window_size, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn preset_dictionary_and_level_hint_are_read_verbatim() {
        let snapshot = DeflateSnapshot { compression_level_hint: DeflateLevelHint::Maximum, dict_id: Some(0xdead_beef), ..DeflateSnapshot::default() };
        let window = compute_deflate_window(&snapshot);
        assert!(window.has_preset_dictionary);
        assert_eq!(window.compression_level_hint, "Maximum");
    }

    #[semio_framework_async_macros::async_test]
    async fn different_payloads_yield_different_digests() {
        let a = DeflateSnapshot { payload: vec![1, 2, 3], ..DeflateSnapshot::default() };
        let b = DeflateSnapshot { payload: vec![4, 5, 6], ..DeflateSnapshot::default() };
        assert_ne!(compute_deflate_window(&a).content_digest, compute_deflate_window(&b).content_digest);
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = DeflateSnapshot { payload: vec![9, 9, 9], ..DeflateSnapshot::default() };
        assert_eq!(compute_deflate_window(&snapshot), compute_deflate_window(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(compute_deflate_window(&DeflateSnapshot::default()), DeflateWindow::default());
    }
}
//#endregion 🧪️Tests

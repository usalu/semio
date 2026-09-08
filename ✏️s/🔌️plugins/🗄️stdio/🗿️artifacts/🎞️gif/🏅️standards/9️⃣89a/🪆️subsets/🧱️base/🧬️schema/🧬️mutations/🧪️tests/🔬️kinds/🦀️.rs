
use super::*;

/// 🧪️ Keeps the declaration honest: `KINDS` must equal every variant's `kind()` (via
/// `demo_mutation_cases()`, which already covers all 21 variants) with none missing or stray,
/// and the oracle catalog manifest must declare every one of them — the framework never parses
/// Rust, so this is the only thing that can catch the two drifting apart.
#[semio_framework_async_macros::async_test]
async fn kinds_matches_every_variant_and_manifest() {
    let from_variants: std::collections::BTreeSet<&str> = demo_mutation_cases().iter().map(GifMutation::kind).collect();
    let from_kinds: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
    assert_eq!(from_variants, from_kinds, "KINDS must equal every GifMutation variant's kind()");
    assert_eq!(KINDS.len(), 20, "KINDS must list exactly the declared 20 kinds");
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "oracle catalog manifest must declare kind {kind:?}");
    }
}

use super::*;
use protocol::SemanticMutation;

/// 🏷️ `KINDS` must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed
/// catalog — the framework reads the manifest and never parses Rust, so this test is the only
/// thing that keeps the two in step. A plain `#[test]`: it suspends on nothing.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = RemodelingMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared RemodelingMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed remodeling-1-any catalog");
    }
}

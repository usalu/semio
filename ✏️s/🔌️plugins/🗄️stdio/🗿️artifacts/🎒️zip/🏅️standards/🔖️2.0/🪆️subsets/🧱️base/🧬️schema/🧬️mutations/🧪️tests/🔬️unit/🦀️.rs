use super::*;
use protocol::{Mutation as _, MutationDiff as _, OpBinary as _, OpText as _};

#[semio_framework_async_macros::async_test]
async fn logical_mutations_diff_and_codecs_round_trip() {
    let base = base_snapshot();
    for mutation in demo_mutation_cases() {
        let text = mutation.print_op();
        assert_eq!(ZipMutation::parse_op(&text).expect("text operation"), mutation);
        let bytes = mutation.encode_op().expect("binary operation");
        assert_eq!(ZipMutation::decode_op(&bytes).expect("binary operation"), mutation);
        assert_eq!(mutation.diff(&base).diff().apply(&base).unwrap(), {
            let mut next = base.clone();
            apply_zip_mutation(&mut next, &mutation);
            next
        });
    }
}

#[semio_framework_async_macros::async_test]
async fn kinds_matches_enum_variants_and_manifest() {
    let observed: std::collections::BTreeSet<&str> = demo_mutation_cases().iter().map(kind_of).collect();
    let declared: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
    assert_eq!(observed, declared, "KINDS must list exactly the kebab-case spelling of every ZipMutation variant");
    assert_eq!(KINDS.len(), demo_mutation_cases().len(), "KINDS must cover every variant exactly once, with no duplicates");

    let manifest: serde_json::Value = serde_json::from_str(include_str!("../../../../🔮️oracles/🔣️.json")).expect("valid oracle manifest JSON");
    let catalog_kinds: std::collections::BTreeSet<String> = manifest["mutationCatalogs"][0]["kinds"].as_array().expect("mutationCatalogs[0].kinds array").iter().map(|value| value.as_str().expect("kind is a string").to_string()).collect();
    let declared_owned: std::collections::BTreeSet<String> = KINDS.iter().map(|kind| kind.to_string()).collect();
    assert_eq!(catalog_kinds, declared_owned, "the oracle manifest's mutationCatalogs[0].kinds must match KINDS exactly");
}

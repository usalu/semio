use super::*;

#[semio_framework_async_macros::async_test]
async fn example_ids_are_unique_and_resolvable() {
    let ids: std::collections::BTreeSet<&str> = REMODELING_EXAMPLES.iter().map(|example| example.id).collect();
    assert_eq!(ids.len(), REMODELING_EXAMPLES.len(), "example ids must be unique");
    assert!(ids.contains(REMODELING_EXAMPLE_BOOT_ID), "the boot example must be registered");
}

#[semio_framework_async_macros::async_test]
async fn every_source_carries_its_committed_text() {
    let sources = example_sources();
    assert_eq!(sources.len(), REMODELING_EXAMPLES.len());
    for example in REMODELING_EXAMPLES {
        assert_eq!(example_text(example.id), Some(example.text));
    }
}

#[semio_framework_async_macros::async_test]
async fn the_boot_document_parses_the_demo_example() {
    assert_eq!(boot_snapshot(), crate::snapshot::text::parse_dsl(example_text(REMODELING_EXAMPLE_BOOT_ID).expect("boot example text")).expect("boot example parses"));
}

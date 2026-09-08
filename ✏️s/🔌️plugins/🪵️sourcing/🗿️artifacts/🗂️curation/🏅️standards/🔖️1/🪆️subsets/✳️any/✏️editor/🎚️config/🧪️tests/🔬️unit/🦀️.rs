
use super::*;
use crate::SortDirection;

#[semio_framework_async_macros::async_test]
async fn sourcing_curation_config_default_matches_the_prior_document_defaults() {
    let config = SourcingCurationConfig::default();
    assert_eq!(config.filters, Filters::default());
    assert_eq!(config.locale, "en-US");
}

fn sample_config() -> SourcingCurationConfig {
    SourcingCurationConfig {
        filters: Filters {
            query: "glulam".into(),
            module_ids: vec!["beams".into()],
            typology_path: vec!["beams".into(), "steel".into()],
            min_availability: 5,
            sort: Some(TableSort { column_id: "availability".into(), direction: SortDirection::Desc }),
        },
        locale: "de-DE".into(),
        contributions_json: "[]".into(),
    }
}

/// 🎞️ Every variant's `backwards()` must exactly restore the pre-operation config.
fn round_trip(config: &SourcingCurationConfig, operation: &SourcingCurationConfigMutation) -> SourcingCurationConfig {
    let forward = operation.diff(config).into_parts().0;
    let backwards = operation.inverse(config);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(&restored).into_parts().0;
    }
    assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
    forward
}

#[semio_framework_async_macros::async_test]
async fn config_mutations_round_trip_every_variant() {
    let config = sample_config();
    round_trip(&config, &SourcingCurationConfigMutation::SetFilterQuery { value: "kvh".into() });
    round_trip(&config, &SourcingCurationConfigMutation::SetFilterModules { module_ids: vec!["windows".into(), "slabs".into()] });
    round_trip(&config, &SourcingCurationConfigMutation::SetFilterTypology { path: vec!["slabs".into()] });
    round_trip(&config, &SourcingCurationConfigMutation::SetFilterMinAvailability { value: 12 });
    round_trip(&config, &SourcingCurationConfigMutation::SetSort { sort: None });
    round_trip(&config, &SourcingCurationConfigMutation::SetLocale { value: "en-US".into() });
    round_trip(&config, &SourcingCurationConfigMutation::SetContributions { json: "[]".into() });
    let snapshot = round_trip(&config, &SourcingCurationConfigMutation::Snapshot { config: SourcingCurationConfig::default() });
    assert_eq!(snapshot, SourcingCurationConfig::default());
}

#[semio_framework_async_macros::async_test]
async fn config_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::Snapshot { config: sample_config() });
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterQuery { value: "kvh".into() });
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterModules { module_ids: vec!["beams".into(), "slabs".into()] });
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterTypology { path: vec!["beams".into(), "steel".into()] });
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterMinAvailability { value: 7 });
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetSort { sort: Some(TableSort { column_id: "name".into(), direction: SortDirection::Asc }) });
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetSort { sort: None });
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetLocale { value: "de-DE".into() });
}


use super::*;
use crate::SortDirection;

#[semio_framework_async_macros::async_test]
async fn sourcing_curation_config_default_matches_the_prior_document_defaults() {
    let config = SourcingCurationConfig::default();
    assert_eq!(config.filters, Filters::default());
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
        contributions_json: "[]".into(),
    }
}

/// 🎞️ Every variant's `backwards()` must exactly restore the pre-operation config.
async fn round_trip(config: &SourcingCurationConfig, operation: &SourcingCurationConfigMutation) -> SourcingCurationConfig {
    let forward = protocol::apply_diff(operation.diff(config).diff(), config).expect("the forward diff applies");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(operation, config).await;
    let backwards = operation.inverse(config).expect("valid retained mutation inverse fixture");
    let mut restored = forward.clone();
    for back in &backwards {
        restored = protocol::apply_diff(back.diff(&restored).diff(), &restored).expect("the inverse diff applies");
    }
    assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
    forward
}

#[semio_framework_async_macros::async_test]
async fn config_mutations_round_trip_every_variant() {
    let config = sample_config();
    round_trip(&config, &SourcingCurationConfigMutation::SetFilterQuery(SetFilterQueryEdit { value: "kvh".into() })).await;
    round_trip(&config, &SourcingCurationConfigMutation::SetFilterModules(SetFilterModulesEdit { module_ids: vec!["windows".into(), "slabs".into()] })).await;
    round_trip(&config, &SourcingCurationConfigMutation::SetFilterTypology(SetFilterTypologyEdit { path: vec!["slabs".into()] })).await;
    round_trip(&config, &SourcingCurationConfigMutation::SetFilterMinAvailability(SetFilterMinAvailabilityEdit { value: 12 })).await;
    round_trip(&config, &SourcingCurationConfigMutation::SetSort(SetSortEdit { sort: None })).await;
    round_trip(&config, &SourcingCurationConfigMutation::SetContributions(SetContributionsEdit { json: "[]".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn config_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterQuery(SetFilterQueryEdit { value: "kvh".into() }));
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterModules(SetFilterModulesEdit { module_ids: vec!["beams".into(), "slabs".into()] }));
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterTypology(SetFilterTypologyEdit { path: vec!["beams".into(), "steel".into()] }));
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterMinAvailability(SetFilterMinAvailabilityEdit { value: 7 }));
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetSort(SetSortEdit { sort: Some(TableSort { column_id: "name".into(), direction: SortDirection::Asc }) }));
    store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetSort(SetSortEdit { sort: None }));
}

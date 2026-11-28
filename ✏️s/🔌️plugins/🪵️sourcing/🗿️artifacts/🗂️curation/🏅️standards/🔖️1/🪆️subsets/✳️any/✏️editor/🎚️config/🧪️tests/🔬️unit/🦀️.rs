use crate::{Filters, TableSort};
#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};

use crate::editor::sourcing::config::*;
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
    let forward = protocol::apply_diff_cold(operation.diff(config).diff(), config).expect("the forward diff applies");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(operation, config).await;
    let backwards = operation.inverse(config).expect("valid retained mutation inverse fixture");
    let mut restored = forward.clone();
    for back in &backwards {
        restored = protocol::apply_diff_cold(back.diff(&restored).diff(), &restored).expect("the inverse diff applies");
    }
    assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
    forward
}

#[semio_framework_async_macros::async_test]
async fn config_mutations_round_trip_every_variant() {
    let config = sample_config();
    round_trip(&config, &SourcingCurationConfigMutation::SetFilterQuery(SetFilterQueryEdit { value: "kvh".into() })).await;
    round_trip(&config, &SourcingCurationConfigMutation::ToggleFilterModule(ToggleFilterModuleEdit { module_id: "slabs".into(), enabled: true })).await;
    round_trip(&config, &SourcingCurationConfigMutation::SetFilterTypology(SetFilterTypologyEdit { path: vec!["slabs".into()] })).await;
    round_trip(&config, &SourcingCurationConfigMutation::SetFilterMinAvailability(SetFilterMinAvailabilityEdit { value: 12 })).await;
    round_trip(&config, &SourcingCurationConfigMutation::SetSort(SetSortEdit { sort: None })).await;
    round_trip(&config, &SourcingCurationConfigMutation::SetContributions(SetContributionsEdit { json: "[]".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn config_op_text_round_trips_every_variant() {
    semio_os_store_test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterQuery(SetFilterQueryEdit { value: "kvh".into() }));
    semio_os_store_test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::ToggleFilterModule(ToggleFilterModuleEdit { module_id: "beams".into(), enabled: true }));
    semio_os_store_test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterTypology(SetFilterTypologyEdit { path: vec!["beams".into(), "steel".into()] }));
    semio_os_store_test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterMinAvailability(SetFilterMinAvailabilityEdit { value: 7 }));
    semio_os_store_test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetSort(SetSortEdit { sort: Some(TableSort { column_id: "name".into(), direction: SortDirection::Asc }) }));
    semio_os_store_test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetSort(SetSortEdit { sort: None }));
}

/// 🧩️ Two module toggles issued from the same stale view both land: each is applied against the stored list, never a list the issuer read earlier.
#[semio_framework_async_macros::async_test]
async fn back_to_back_module_toggles_both_apply_and_undo_in_order() {
    let toggle = |module: &str, enabled: bool| SourcingCurationConfigMutation::ToggleFilterModule(ToggleFilterModuleEdit { module_id: module.into(), enabled });
    let apply = |config: &SourcingCurationConfig, mutation: &SourcingCurationConfigMutation| protocol::apply_diff_cold(mutation.diff(config).diff(), config).expect("the toggle diff applies");
    let mut config = sample_config();
    config.filters.module_ids.clear();
    let (first, second) = (toggle("beams", true), toggle("windows", true));
    let after_first = apply(&config, &first);
    let after_second = apply(&after_first, &second);
    assert_eq!(after_second.filters.module_ids, vec!["beams".to_string(), "windows".to_string()]);
    let off_first = apply(&after_second, &toggle("beams", false));
    let off_second = apply(&off_first, &toggle("windows", false));
    assert!(off_second.filters.module_ids.is_empty(), "both toggles off leave no module filter");
    assert_eq!(apply(&after_second, &toggle("beams", true)), after_second, "enabling an enabled module is a no-op");
    assert!(toggle("beams", true).diff(&after_second).diff().filters_module_ids.is_none(), "a no-op toggle yields an empty diff");
    let inverse = second.inverse(&after_first).expect("inverse");
    assert_eq!(inverse, vec![toggle("windows", false)]);
}

use super::*;

//#region 🔖️Priority
#[test]
fn priority_rank_matches_declaration_order() {
    for (index, priority) in Priority::ALL.iter().enumerate() {
        assert_eq!(priority.rank(), index);
    }
    assert!(Priority::System < Priority::Command);
    assert!(Priority::Command < Priority::Preview);
}

#[test]
fn only_preview_is_sheddable() {
    for priority in Priority::ALL {
        assert_eq!(priority.sheddable(), priority == Priority::Preview);
    }
}

#[test]
fn default_weights_are_strictly_decreasing_by_priority_order() {
    let weights: Vec<u32> = Priority::ALL.iter().map(|priority| priority.default_weight()).collect();
    for window in weights.windows(2) {
        assert!(window[0] > window[1], "weights must strictly decrease: {weights:?}");
    }
}
//#endregion 🔖️Priority

//#region 🔖️Config
#[test]
fn mailbox_capacities_get_set_round_trip_per_lane() {
    let mut capacities = MailboxCapacities::uniform(10);
    assert_eq!(capacities.get(Priority::Command), 10);
    capacities.set(Priority::Preview, 2);
    assert_eq!(capacities.get(Priority::Preview), 2);
    assert_eq!(capacities.get(Priority::System), 10);
}

#[test]
fn profile_defaults_order_durability_test_below_dev_below_prod() {
    let test_config = DbConfig::for_profile(Profile::Test);
    let dev_config = DbConfig::for_profile(Profile::Dev);
    let prod_config = DbConfig::for_profile(Profile::Prod);
    assert!(test_config.default_durability < dev_config.default_durability);
    assert!(dev_config.default_durability < prod_config.default_durability);
    assert!(!test_config.capabilities.cluster);
    assert!(prod_config.capabilities.cluster);
    assert_eq!(test_config.capabilities.max_durability, test_config.default_durability);
}

#[test]
fn test_profile_has_tighter_limits_than_prod() {
    let test_config = DbConfig::for_profile(Profile::Test);
    let prod_config = DbConfig::for_profile(Profile::Prod);
    assert!(test_config.limits.max_command_bytes < prod_config.limits.max_command_bytes);
    assert!(test_config.limits.max_batch_commands < prod_config.limits.max_batch_commands);
}
//#endregion 🔖️Config

//#region 🔖️WorkerSubmitRetry
/// 🔁️ The language-agnostic decision table: a lock race never spends an attempt (so no run of
/// races, however long, refuses an owner's work), saturation spends exactly one of `limit`, and
/// shutdown/poisoning are terminal.
#[test]
fn worker_submit_retry_follows_the_declared_decision_table() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️worker-submit-retry/🔣️.json")).unwrap();
    let limit = u8::try_from(fixture["limit"].as_u64().unwrap()).unwrap();
    let rows = fixture["rows"].as_array().unwrap();
    for row in rows {
        let kind = match row["kind"].as_str().unwrap() {
            "Contended" => semio_framework_async::WorkerSubmitErrorKind::Contended,
            "Saturated" => semio_framework_async::WorkerSubmitErrorKind::Saturated,
            "Shutdown" => semio_framework_async::WorkerSubmitErrorKind::Shutdown,
            "Poisoned" => semio_framework_async::WorkerSubmitErrorKind::Poisoned,
            other => panic!("undeclared kind {other}"),
        };
        let attempt = u8::try_from(row["attempt"].as_u64().unwrap()).unwrap();
        let next = row["next"].as_u64().map(|next| u8::try_from(next).unwrap());
        assert_eq!(worker_submit_retry_attempt(kind, attempt, limit), next, "{row}");
    }
    assert_eq!(rows.len(), 12);
    let mut attempt = 0;
    for _ in 0..10_000 {
        attempt = worker_submit_retry_attempt(semio_framework_async::WorkerSubmitErrorKind::Contended, attempt, limit).expect("a lock race is never terminal");
    }
    assert_eq!(attempt, 0);
}
//#endregion 🔖️WorkerSubmitRetry
